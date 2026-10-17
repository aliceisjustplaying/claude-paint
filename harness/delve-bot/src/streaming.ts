import { randomInt } from "node:crypto";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { isLoopback } from "./config.ts";
import { stopped } from "./gate.ts";
import { capturePage, ffmpegArgs } from "./capture.ts";
import type { Config, Job } from "./types.ts";

export interface LivestreamRecord {
  $type: "place.stream.livestream";
  title: string;
  url: string;
  canonicalUrl: string;
  createdAt: string;
  idleTimeoutSeconds: number;
  endedAt?: string;
}

let active = false;
let lastMicros = 0n;
function tid(): string {
  lastMicros = [BigInt(Date.now()) * 1000n, lastMicros + 1n].reduce((a, b) => a > b ? a : b);
  let value = (lastMicros << 10n) | BigInt(randomInt(1024));
  let result = "";
  for (let i = 0; i < 13; i++) { result = "234567abcdefghijklmnopqrstuvwxyz"[Number(value & 31n)] + result; value >>= 5n; }
  return result;
}

export function streamLink(config: Config): string {
  return `${config.streaming.stationUrl.replace(/\/$/u, "")}/${config.did}`;
}

export function studioUrl(config: Config, job: Job): string {
  const url = new URL("/", config.streaming.viewerUrl);
  url.search = new URLSearchParams({ p: job.folder, stream: "1" }).toString();
  return url.href;
}

class LivestreamPds {
  private token: string | undefined;
  readonly config: Config;
  readonly env: NodeJS.ProcessEnv;
  readonly fetcher: typeof fetch;
  constructor(config: Config, env: NodeJS.ProcessEnv, fetcher: typeof fetch) {
    this.config = config; this.env = env; this.fetcher = fetcher;
  }
  async request(method: string, payload: Record<string, unknown>): Promise<Record<string, unknown>> {
    if (this.config.dryRun || (!isLoopback(this.config.pdsUrl) && !this.config.liveStreaming)) throw new Error("live_stream_records_disabled");
    const url = new URL(`/xrpc/${method}`, this.config.pdsUrl);
    const read = method === "com.atproto.repo.getRecord";
    if (read) url.search = new URLSearchParams(Object.entries(payload).map(([k, v]) => [k, String(v)])).toString();
    try {
      const res = await this.fetcher(url, { method: read ? "GET" : "POST", redirect: "error", signal: AbortSignal.timeout(3000),
        headers: { "Content-Type": "application/json", ...(this.token ? { Authorization: `Bearer ${this.token}` } : {}) },
        ...(read ? {} : { body: JSON.stringify(payload) }) });
      if (!res.ok) throw new Error();
      return await res.json() as Record<string, unknown>;
    } catch { throw new Error("stream_pds_request_failed"); }
  }
  async login(): Promise<void> {
    const identifier = this.env["DELVE_HANDLE"], password = this.env["DELVE_APP_PASSWORD"];
    if (!identifier || !password) throw new Error("credentials_missing");
    const session = await this.request("com.atproto.server.createSession", { identifier, password });
    if (session["did"] !== this.config.did || typeof session["accessJwt"] !== "string") throw new Error("stream_session_identity_mismatch");
    this.token = session["accessJwt"];
  }
  async end(rkey: string): Promise<void> {
    const params = { repo: this.config.did, collection: "place.stream.livestream", rkey };
    const current = await this.request("com.atproto.repo.getRecord", params);
    const value = current["value"] as LivestreamRecord;
    if (current["uri"] !== `at://${this.config.did}/place.stream.livestream/${rkey}` ||
        typeof current["cid"] !== "string" || value?.$type !== "place.stream.livestream") throw new Error("stream_record_mismatch");
    if (!value.endedAt) await this.request("com.atproto.repo.putRecord", {
      ...params, record: { ...value, endedAt: new Date().toISOString() }, swapRecord: current["cid"], validate: true
    });
  }
}

export class Streaming {
  readonly config: Config;
  readonly env: NodeJS.ProcessEnv;
  readonly fetcher: typeof fetch;
  readonly log: (value: object) => void;
  private runs = new Set<{ controller: AbortController; settled: Promise<void> }>();
  constructor(config: Config, env: NodeJS.ProcessEnv = process.env, fetcher: typeof fetch = fetch,
    log: (value: object) => void = value => console.log(JSON.stringify(value))) {
    this.config = config; this.env = env; this.fetcher = fetcher; this.log = log;
  }
  async stop(): Promise<void> {
    for (const run of this.runs) run.controller.abort("shutdown");
    await Promise.all([...this.runs].map(run => run.settled));
  }
  private track(): { controller: AbortController; settle: () => void } {
    const controller = new AbortController(), done = Promise.withResolvers<void>(), run = { controller, settled: done.promise };
    this.runs.add(run);
    return { controller, settle: () => { this.runs.delete(run); done.resolve(); } };
  }
  async run<T>(job: Job, work: (signal: AbortSignal) => Promise<T>, canceled: () => boolean = () => false): Promise<T> {
    if (!this.config.streaming.enabled) {
      const { controller, settle } = this.track();
      const poll = setInterval(() => { if (stopped(this.config, this.env) || canceled()) controller.abort(); }, 100);
      try {
        const result = await work(controller.signal);
        if (controller.signal.reason === "shutdown") throw new Error("interrupted_by_shutdown");
        return result;
      } finally { clearInterval(poll); settle(); }
    }
    const title = (JSON.parse(readFileSync(new URL("../assets/replies.json", import.meta.url), "utf8")) as Record<string, string>)["streamTitle"]!;
    if (active) throw new Error("stream_slot_busy");
    if (stopped(this.config, this.env) || canceled()) throw new Error("stream_canceled");
    active = true;
    const { controller, settle } = this.track();
    const limit = this.config.streaming.maxSeconds;
    const timer = limit == null ? undefined : setTimeout(() => controller.abort("time"), limit * 1000);
    const poll = setInterval(() => { if (stopped(this.config, this.env) || canceled()) controller.abort(); }, 100);
    let grace: NodeJS.Timeout | undefined;
    const abort = new Promise<never>((_, reject) => controller.signal.addEventListener("abort", () => {
      const fail = () => reject(new Error("stream_canceled"));
      if (controller.signal.reason === "time") grace = setTimeout(fail, 10000); else fail();
    }, { once: true }));
    void abort.catch(() => {});
    const rkey = tid(), link = streamLink(this.config);
    const record: LivestreamRecord = { $type: "place.stream.livestream", title,
      url: link, canonicalUrl: link, createdAt: new Date().toISOString(), idleTimeoutSeconds: 30 };
    const params = { repo: this.config.did, collection: "place.stream.livestream", rkey, record, validate: true };
    const pds = new LivestreamPds(this.config, this.env, this.fetcher);
    let capture: Awaited<ReturnType<typeof capturePage>> | undefined;
    let attempted = false;
    try {
      if (this.config.dryRun) {
        const destination = this.config.streaming.ingestUrl.startsWith("file:") ? fileURLToPath(this.config.streaming.ingestUrl)
          : `${this.config.streaming.ingestUrl.replace(/\/$/u, "")}/[STREAMPLACE_STREAM_KEY]`;
        this.log({ event: "stream_start_dry_run", viewerUrl: studioUrl(this.config, job),
          command: ["ffmpeg", ...ffmpegArgs(this.config, destination)].map(value => JSON.stringify(value)).join(" "),
          method: "com.atproto.repo.createRecord", payload: params });
      } else {
        const ingest = new URL(this.config.streaming.ingestUrl);
        if (!this.config.liveStreaming && (!isLoopback(this.config.pdsUrl) || !((ingest.protocol === "rtmp:" && ingest.hostname === "127.0.0.1") || ingest.protocol === "file:"))) throw new Error("live_streaming_disabled");
        const key = this.env["STREAMPLACE_STREAM_KEY"];
        if (!key || !/^[A-Za-z0-9_-]+$/u.test(key)) throw new Error("stream_key_missing_or_invalid");
        const destination = ingest.protocol === "file:" ? fileURLToPath(ingest) : `${ingest.href.replace(/\/$/u, "")}/${key}`;
        await pds.login();
        controller.signal.throwIfAborted();
        capture = await capturePage(this.config, studioUrl(this.config, job), destination, controller.signal);
        controller.signal.throwIfAborted();
        attempted = true;
        const created = await pds.request("com.atproto.repo.createRecord", params);
        if (created["uri"] !== `at://${this.config.did}/place.stream.livestream/${rkey}` || typeof created["cid"] !== "string") throw new Error("stream_create_mismatch");
      }
      controller.signal.throwIfAborted();
      const ended = capture && new Promise<never>((_, reject) => void capture!.finished.then(() => { throw new Error("capture_ended"); })
        .catch((error: unknown) => { if (controller.signal.reason !== "time") reject(error); }));
      const result = await Promise.race([work(controller.signal), abort, ...(ended ? [ended] : [])]);
      if (stopped(this.config, this.env) || canceled()) throw new Error("stream_canceled");
      return result;
    } catch { throw new Error("stream_job_failed_or_canceled"); }
    finally {
      clearTimeout(timer); clearTimeout(grace); clearInterval(poll);
      controller.abort();
      try {
        await capture?.stop();
        if (this.config.dryRun) this.log({ event: "stream_end_dry_run", method: "com.atproto.repo.putRecord",
          payload: { ...params, swapRecord: "[CURRENT_RECORD_CID]", record: { ...record, endedAt: new Date().toISOString() } } });
        else if (attempted) {
          try { await pds.login(); await pds.end(rkey); } catch { this.log({ event: "stream_end_failed", rkey }); }
        }
      } finally { active = false; settle(); }
    }
  }
}
