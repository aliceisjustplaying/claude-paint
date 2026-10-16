import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { checkRequest } from "./check.ts";
import { fetchThread } from "./delve.ts";
import { addressed, instruction, optOut, postAuthor, stopped } from "./gate.ts";
import { HousePainter, jobDir, painterJob, paintingBlob } from "./painter.ts";
import type { Painter, Run } from "./painter.ts";
import { Posting } from "./posting.ts";
import { Store } from "./store.ts";
import { Streaming, streamLink } from "./streaming.ts";
import type { Config, IngestEvent, Intent, Job, JobRow, Post } from "./types.ts";

interface Result { kind: "done" | "doneLimit" | "doneDry" | "failed"; width?: number; height?: number; alt?: string }
const STREAM_KIND: Record<string, string> = { done: "doneStream", doneLimit: "doneLimitStream", doneDry: "doneStreamDry" };

const replies = JSON.parse(readFileSync(new URL("../assets/replies.json", import.meta.url), "utf8")) as Record<string, string>;
export function wording(kind: string, position = 0): string {
  const text = replies[kind];
  if (text === undefined) throw new Error("unknown_reply_template");
  return text.replace("{position}", String(position));
}

export class Engine {
  readonly posting: Posting;
  readonly streaming: Streaming;
  readonly painter: Painter;
  private working = new Set<string>();
  private flushing = false;
  readonly config: Config;
  readonly store: Store;
  readonly fetcher: typeof fetch;
  readonly env: NodeJS.ProcessEnv;
  constructor(config: Config, store: Store, fetcher: typeof fetch = fetch, env: NodeJS.ProcessEnv = process.env, painter?: Painter) {
    this.config = config;
    this.store = store;
    this.fetcher = fetcher;
    this.env = env;
    this.posting = new Posting(config, store, fetcher, env);
    this.streaming = new Streaming(config, env, fetcher);
    this.painter = painter ?? new HousePainter(config, env);
  }
  private replyText(kind: string, position = 0): string {
    const streamKind = kind === "accepted" ? `acceptedStream${this.config.dryRun ? "Dry" : ""}` : STREAM_KIND[kind];
    if (this.config.streaming.enabled && this.config.streaming.includeLink && streamKind) {
      return wording(streamKind).replace("{streamUrl}", streamLink(this.config));
    }
    return wording(kind, position);
  }
  stage(event: IngestEvent): void {
    this.store.transaction(() => {
      if (event.post && postAuthor(event.post.uri) === event.post.did && event.post.did !== this.config.did && optOut(event.post, this.config)) {
        this.store.cancel(event.post.did);
      }
      if (event.operation === "delete" || event.operation === "update") this.store.cancelRequest(event.uri);
      this.store.stage(event, this.config);
    });
  }
  async drain(now = Date.now()): Promise<void> {
    for (let item = this.store.nextEvent(); item; item = this.store.nextEvent()) {
      const event = item.event;
      if (event.post && !event.rejectionReason) {
        const result = await this.intake(event.post, now, event.operation !== "create");
        if (result === "thread_missing") throw new Error("appview_not_ready");
      }
      this.store.transaction(() => this.store.ack(item.id, event.cursor));
    }
  }
  private turnCount(root: string, includePending = true): number {
    return this.store.count(`SELECT count(*) n FROM (
      SELECT uri FROM observed WHERE root=?
      UNION SELECT 'at://' || ? || '/town.delve.feed.post/' || rkey FROM outbox WHERE root=? AND (sent IS NOT NULL OR (?=1 AND status='pending'))
      UNION SELECT 'reserved-completion:' || id FROM jobs WHERE root=? AND status IN ('running','queued') AND ?=1
    )`, root, this.config.did, root, Number(includePending), root, Number(includePending));
  }
  private saveJob(job: Job): void {
    const dir = jobDir(this.config, job.id);
    mkdirSync(dir, { recursive: true });
    writeFileSync(join(dir, "job.json"), JSON.stringify(job, null, 2) + "\n");
    writeFileSync(join(dir, "BRIEF.md"), job.brief);
  }
  async intake(post: Post, now = Date.now(), edited = false): Promise<string> {
    if (post.did === this.config.did || postAuthor(post.uri) !== post.did || !post.cid || !addressed(post, this.config)) return "unaddressed_or_invalid";
    const isStop = optOut(post, this.config);
    if (isStop) this.store.transaction(() => this.store.cancel(post.did));
    if (stopped(this.config, this.env)) return "kill_switch";
    if (this.config.blockedDids.includes(post.did)) return "blocked";
    if (!isStop && this.store.optedOut(post.did, this.config)) return "opted_out";
    if (edited && !isStop) return "edited";
    if (this.store.count("SELECT count(*) n FROM seen WHERE uri=?", post.uri)) return "duplicate";
    const age = now - Date.parse(post.createdAt);
    if (!Number.isFinite(age) || age < 0 || age > this.config.caps.requestAgeSeconds * 1000) return "stale";
    const thread = await fetchThread(post.uri, this.config, this.fetcher);
    const hydrated = thread.find(item => item.uri === post.uri);
    const root = thread.find(item => item.uri === post.rootUri);
    if (!hydrated || !root) return "thread_missing";
    if (!hydrated || hydrated.cid !== post.cid || hydrated.did !== post.did || hydrated.text !== post.text || hydrated.rootUri !== post.rootUri || hydrated.parentUri !== post.parentUri || !addressed(hydrated, this.config) || !root?.cid || postAuthor(root.uri) !== root.did) return "thread_mismatch";
    const request = instruction(post, this.config);
    const job = painterJob(post, { uri: root.uri, cid: root.cid }, request, this.config);
    const oversize = [...post.text].length > this.config.caps.requestChars || Buffer.byteLength(post.text) > this.config.caps.requestBytes;
    const verdict = isStop || oversize ? null : await checkRequest(request, this.config, this.env, this.fetcher);
    let accepted = false;
    const reason = this.store.transaction(() => {
      if (stopped(this.config, this.env)) return "kill_switch";
      if (!isStop && this.store.count("SELECT count(*) n FROM invalidated WHERE uri=?", post.uri)) return "invalidated";
      if (this.store.count("SELECT count(*) n FROM seen WHERE uri=?", post.uri)) return "duplicate";
      for (const item of thread) this.store.db.prepare("INSERT OR IGNORE INTO observed VALUES (?, ?)").run(item.uri, post.rootUri);
      const remember = (reason: string): string => {
        this.store.db.prepare("INSERT OR IGNORE INTO seen VALUES (?, ?)").run(post.uri, reason);
        return reason;
      };
      if (this.turnCount(post.rootUri) >= this.config.caps.botOnlyTurns) return remember("bot_only_backoff");
      const last = this.store.db.prepare("SELECT max(sent) last FROM outbox WHERE root=? AND sent IS NOT NULL").get(post.rootUri) as { last: number | null };
      if (last.last !== null && now - last.last < this.config.caps.botCooldownSeconds * 1000 && !isStop) return remember("bot_cooldown");
      const reply = (kind: string, position = 0): string => {
        this.store.addReply(job, kind, this.replyText(kind, position), now, undefined,
          this.config.streaming.enabled && this.config.streaming.includeLink ? streamLink(this.config) : undefined);
        return remember(kind);
      };
      if (this.store.intents().filter(row => row.status === "pending").length >= this.config.caps.pendingReplies) return remember("reply_queue_full");
      if (isStop) {
        if (this.store.count("SELECT count(*) n FROM outbox WHERE did=? AND id LIKE '%#stopped'", post.did)) return remember("opted_out");
        return reply("stopped");
      }
      if (this.store.optedOut(post.did, this.config)) return remember("opted_out");
      if (oversize) return reply("declinedSize");
      if (verdict === "error") return reply("declinedCheck");
      if (verdict !== "accept") return reply("declinedPolicy");
      if (this.turnCount(post.rootUri) + 2 > this.config.caps.botOnlyTurns) return remember("bot_only_backoff");
      const recent = this.store.jobs().filter(row => now - row.admitted < 86400000);
      const byAuthor = this.store.jobs().filter(row => row.did === post.did);
      if (recent.length >= this.config.caps.paintingsPerDay || recent.filter(row => row.did === post.did).length >= this.config.caps.requesterPerDay ||
          byAuthor.some(row => now - row.admitted < this.config.caps.requesterCooldownSeconds * 1000)) return reply("declinedLimit");
      const active = Math.max(this.working.size, this.store.jobs().filter(row => row.status === "running").length);
      const queued = this.store.jobs().filter(row => row.status === "queued").length;
      const starts = this.store.jobs().filter(row => row.started !== null && now - row.started < 86400000);
      const canStart = starts.length < this.config.caps.paintingsPerDay && starts.filter(row => row.did === post.did).length < this.config.caps.requesterPerDay;
      const status = active < this.config.caps.running && queued === 0 && canStart ? "running" : "queued";
      if (status === "queued" && queued >= this.config.caps.queue) return reply("declinedQueue");
      this.store.addJob(job, now, status);
      accepted = true;
      return reply(status === "running" ? "accepted" : "queued", queued + 1);
    });
    if (accepted) this.saveJob(job);
    return reason;
  }
  async pump(now = Date.now(), work?: (job: Job, signal: AbortSignal) => Promise<Run | void>): Promise<void> {
    if (stopped(this.config, this.env)) return;
    this.store.transaction(() => {
      for (const row of this.store.jobs()) {
        const job = JSON.parse(row.job) as Job;
        if (this.store.optedOut(row.did, this.config) || this.config.blockedDids.includes(row.did)) {
          this.store.db.prepare("UPDATE jobs SET status='canceled' WHERE id=? AND status IN ('queued','running')").run(row.id);
          continue;
        } else if (row.status === "queued" && now - row.admitted >= this.config.caps.queueTtlSeconds * 1000) {
          this.store.db.prepare("UPDATE jobs SET status='expired' WHERE id=?").run(row.id);
        }
        if ((row.status === "expired" || (row.status === "queued" && now - row.admitted >= this.config.caps.queueTtlSeconds * 1000)) &&
            this.store.intents().filter(item => item.status === "pending").length < this.config.caps.pendingReplies) {
          this.store.addReply(job, "expired", wording("expired"), now);
        }
      }
      for (const row of this.store.jobs().filter(row => row.status === "queued")) {
        const rows = this.store.jobs();
        if (rows.filter(row => row.status === "running").length >= this.config.caps.running) break;
        const starts = rows.filter(row => row.started !== null && now - row.started < 86400000);
        if (starts.length >= this.config.caps.paintingsPerDay || starts.filter(item => item.did === row.did).length >= this.config.caps.requesterPerDay) continue;
        this.store.db.prepare("UPDATE jobs SET status='running', started=? WHERE id=? AND status='queued'").run(now, row.id);
      }
    });
    const running = this.store.jobs().filter(row => row.status === "running" && !this.working.has(row.id)).slice(0, Math.max(0, this.config.caps.running - this.working.size));
    await Promise.all(running.map(async (row: JobRow) => {
      this.working.add(row.id);
      try {
        const job = JSON.parse(row.job) as Job;
        this.saveJob(job);
        if (stopped(this.config, this.env)) return;
        const dir = jobDir(this.config, job.id), saved = join(dir, "result.json");
        const canceled = () => this.store.jobs().find(item => item.id === row.id)?.status !== "running" ||
          this.store.optedOut(row.did, this.config) || this.config.blockedDids.includes(row.did);
        if (!existsSync(saved)) {
          const run = await this.streaming.run(job, signal => work ? work(job, signal) : this.painter.run(job, signal, row.started ?? now), canceled);
          if (stopped(this.config, this.env) || canceled()) return;
          let result: Result = { kind: "failed" };
          try {
            const painting = await this.painter.finish(job, run);
            if (painting === "dry") result = { kind: "doneDry" };
            else {
              writeFileSync(join(dir, "painting.jpg"), painting.bytes);
              result = { kind: painting.stopped ? "doneLimit" : "done", width: painting.width, height: painting.height, alt: painting.alt };
            }
          } catch (error) { console.error(JSON.stringify({ event: "painting_failed", job: job.id, error: error instanceof Error ? error.message : "unknown" })); }
          writeFileSync(saved, JSON.stringify(result) + "\n");
        }
        const result = JSON.parse(readFileSync(saved, "utf8")) as Result;
        const link = this.config.streaming.enabled && this.config.streaming.includeLink ? streamLink(this.config) : undefined;
        const embed = result.width && result.height ? { $type: "town.delve.embed.images", images: [{
          image: paintingBlob(readFileSync(join(dir, "painting.jpg")), "image/jpeg"), alt: result.alt ?? wording("imageAlt"),
          aspectRatio: { width: result.width, height: result.height } }] } : undefined;
        this.store.transaction(() => {
          const current = this.store.jobs().find(item => item.id === row.id);
          if (current?.status !== "running" || this.store.optedOut(row.did, this.config) || this.config.blockedDids.includes(row.did)) return;
          if (this.store.intents().filter(item => item.status === "pending").length >= this.config.caps.pendingReplies) return;
          this.store.db.prepare("UPDATE jobs SET status=? WHERE id=?").run(result.kind === "failed" ? "failed" : "done", row.id);
          if (result.kind === "failed") this.store.addReply(job, "failed", wording("failed"), now);
          else this.store.addReply(job, "done", this.replyText(result.kind), now, embed, link);
        });
      } catch (error) {
        if (this.config.streaming.enabled) this.store.db.prepare("UPDATE jobs SET status='canceled' WHERE id=? AND status='running'").run(row.id);
        throw error;
      } finally { this.working.delete(row.id); }
    }));
  }
  async flush(now?: number): Promise<void> {
    if (this.flushing || stopped(this.config, this.env)) return;
    this.flushing = true;
    try {
      for (const intent of this.store.intents().filter(item => item.status === "pending")) {
        if (this.config.blockedDids.includes(intent.did) || (!intent.id.endsWith("#stopped") && this.store.optedOut(intent.did, this.config))) {
          this.store.db.prepare("UPDATE outbox SET status='canceled' WHERE id=?").run(intent.id);
          continue;
        }
        const reserve = (): boolean => this.store.transaction(() => {
          const at = now ?? Date.now();
          if (stopped(this.config, this.env)) return false;
          if (this.config.blockedDids.includes(intent.did) || (!intent.id.endsWith("#stopped") && this.store.optedOut(intent.did, this.config))) {
            this.store.db.prepare("UPDATE outbox SET status='canceled' WHERE id=?").run(intent.id);
            return false;
          }
          const record = JSON.parse(intent.record) as { text: string };
          if ([...record.text].length > this.config.caps.replyChars || Buffer.byteLength(record.text) > this.config.caps.replyBytes) throw new Error("reply_too_large");
          const current = this.store.intents().find(item => item.id === intent.id) as Intent;
          const ownCount = current.sent === null ? 0 : 1;
          if (this.turnCount(intent.root, false) - ownCount >= this.config.caps.botOnlyTurns) {
            this.store.db.prepare("UPDATE outbox SET status='canceled' WHERE id=?").run(intent.id);
            return false;
          }
          const sent = this.store.intents().filter(item => item.id !== intent.id && item.sent !== null);
          if (sent.filter(item => at - item.sent! < 3600000).length >= this.config.caps.globalPerHour ||
              sent.filter(item => item.root === intent.root && at - item.sent! < 21600000).length >= this.config.caps.threadPerSixHours ||
              sent.some(item => item.root === intent.root && at - item.sent! < this.config.caps.botCooldownSeconds * 1000)) return false;
          this.store.db.prepare("UPDATE outbox SET sent=? WHERE id=?").run(at, intent.id);
          return true;
        });
        await this.posting.send(intent, reserve);
      }
    } finally { this.flushing = false; }
  }
}
