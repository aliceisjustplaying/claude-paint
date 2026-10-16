import { createHash } from "node:crypto";
import { mkdirSync } from "node:fs";
import { dirname } from "node:path";
import { DatabaseSync } from "node:sqlite";
import type { Config, IngestEvent, Intent, Job, JobRow } from "./types.ts";

export class Store {
  db: DatabaseSync;
  constructor(path: string, identity = "prototype", maxBytes = 52428800) {
    mkdirSync(dirname(path), { recursive: true });
    this.db = new DatabaseSync(path);
    const size = (this.db.prepare("PRAGMA page_size").get() as { page_size: number }).page_size;
    const limit = Math.floor(maxBytes / size);
    const applied = this.db.prepare(`PRAGMA max_page_count=${limit}`).get() as { max_page_count: number };
    if (applied.max_page_count > limit) {
      this.db.close();
      throw new Error("existing_database_exceeds_cap");
    }
    this.db.exec(`PRAGMA journal_mode=WAL; PRAGMA busy_timeout=5000;
      CREATE TABLE IF NOT EXISTS meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS events (id TEXT PRIMARY KEY, cursor INTEGER NOT NULL, event TEXT NOT NULL, handled INTEGER NOT NULL DEFAULT 0);
      CREATE TABLE IF NOT EXISTS seen (uri TEXT PRIMARY KEY, reason TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS invalidated (uri TEXT PRIMARY KEY);
      CREATE TABLE IF NOT EXISTS optouts (did TEXT PRIMARY KEY);
      CREATE TABLE IF NOT EXISTS observed (uri TEXT PRIMARY KEY, root TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS jobs (id TEXT PRIMARY KEY, did TEXT NOT NULL, root TEXT NOT NULL, admitted INTEGER NOT NULL, started INTEGER, status TEXT NOT NULL, job TEXT NOT NULL);
      CREATE TABLE IF NOT EXISTS outbox (id TEXT PRIMARY KEY, did TEXT NOT NULL, root TEXT NOT NULL, rkey TEXT NOT NULL UNIQUE, record TEXT NOT NULL, created INTEGER NOT NULL, sent INTEGER, status TEXT NOT NULL);
      CREATE INDEX IF NOT EXISTS jobs_admitted ON jobs(admitted);
      CREATE INDEX IF NOT EXISTS outbox_created ON outbox(created);`);
    const old = this.db.prepare("SELECT value FROM meta WHERE key='identity'").get() as { value: string } | undefined;
    if (old && old.value !== identity) throw new Error("Database identity/mode mismatch");
    this.db.prepare("INSERT OR IGNORE INTO meta VALUES ('identity', ?)").run(identity);
  }
  close(): void { this.db.close(); }
  transaction<T>(fn: () => T): T {
    this.db.exec("BEGIN IMMEDIATE");
    try { const value = fn(); this.db.exec("COMMIT"); return value; }
    catch (error) { this.db.exec("ROLLBACK"); throw error; }
  }
  count(sql: string, ...params: (string | number)[]): number {
    return (this.db.prepare(sql).get(...params) as { n: number }).n;
  }
  optedOut(did: string, config: Config): boolean {
    return config.optedOutDids.includes(did) || this.count("SELECT count(*) n FROM optouts WHERE did=?", did) > 0;
  }
  cancel(did: string): void {
    this.db.prepare("INSERT OR IGNORE INTO optouts VALUES (?)").run(did);
    this.db.prepare("UPDATE jobs SET status='canceled' WHERE did=? AND status IN ('queued','running')").run(did);
    this.db.prepare("UPDATE outbox SET status='canceled' WHERE did=? AND status='pending'").run(did);
  }
  cancelRequest(uri: string): void {
    this.db.prepare("INSERT OR IGNORE INTO invalidated VALUES (?)").run(uri);
    this.db.prepare("UPDATE jobs SET status='canceled' WHERE id=? AND status IN ('queued','running')").run(uri);
    this.db.prepare("UPDATE outbox SET status='canceled' WHERE id LIKE ? AND status='pending'").run(`${uri}#%`);
  }
  stage(event: IngestEvent, config: Config): void {
    const id = `${event.cursor}:${event.uri}:${event.operation}`;
    if (this.count("SELECT count(*) n FROM events WHERE id=?", id)) return;
    if (this.count("SELECT count(*) n FROM events WHERE handled=0") >= config.caps.pendingEvents) throw new Error("ingest_queue_full");
    this.db.prepare("INSERT INTO events VALUES (?, ?, ?, 0)").run(id, event.cursor, JSON.stringify(event));
  }
  nextEvent(): { id: string; event: IngestEvent } | null {
    const row = this.db.prepare("SELECT id, event FROM events WHERE handled=0 ORDER BY cursor, rowid LIMIT 1").get() as { id: string; event: string } | undefined;
    return row ? { id: row.id, event: JSON.parse(row.event) as IngestEvent } : null;
  }
  ack(id: string, cursor: number): void {
    this.db.prepare("UPDATE events SET handled=1 WHERE id=?").run(id);
    this.db.prepare("INSERT INTO meta VALUES ('cursor', ?) ON CONFLICT(key) DO UPDATE SET value=CAST(max(CAST(value AS INTEGER),CAST(excluded.value AS INTEGER)) AS TEXT)").run(String(cursor));
    this.db.prepare("DELETE FROM events WHERE handled=1 AND cursor<?").run(cursor);
  }
  cursor(): number | null {
    const row = this.db.prepare("SELECT value FROM meta WHERE key='cursor'").get() as { value: string } | undefined;
    return row ? Number(row.value) : null;
  }
  jobs(): JobRow[] { return this.db.prepare("SELECT * FROM jobs ORDER BY admitted,rowid").all() as unknown as JobRow[]; }
  intents(): Intent[] { return this.db.prepare("SELECT * FROM outbox ORDER BY created,rowid").all() as unknown as Intent[]; }
  addJob(job: Job, now: number, status: "queued" | "running"): void {
    this.db.prepare("INSERT INTO jobs VALUES (?, ?, ?, ?, ?, ?, ?)").run(job.id, job.post.did, job.post.rootUri, now, status === "running" ? now : null, status, JSON.stringify(job));
  }
  addReply(job: Job, kind: string, text: string, now: number, embed?: unknown, link?: string): void {
    const id = `${job.id}#${kind}`;
    const start = link ? text.indexOf(link) : -1;
    const facets = link && start >= 0 ? [{
      index: { byteStart: Buffer.byteLength(text.slice(0, start)), byteEnd: Buffer.byteLength(text.slice(0, start + link.length)) },
      features: [{ $type: "town.delve.richtext.facet#link", uri: link }]
    }] : undefined;
    const record = {
      $type: "town.delve.feed.post", text, createdAt: new Date(now).toISOString(),
      reply: { root: job.root, parent: { uri: job.post.uri, cid: job.post.cid } },
      ...(embed ? { embed } : {}),
      ...(facets ? { facets } : {})
    };
    this.db.prepare("INSERT OR IGNORE INTO outbox VALUES (?, ?, ?, ?, ?, ?, NULL, 'pending')").run(id, job.post.did, job.post.rootUri, createHash("sha256").update(id).digest("hex").slice(0, 32), JSON.stringify(record), now);
  }
}
