import { readFileSync } from "node:fs";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { isLoopback } from "./config.ts";
import { stopped } from "./gate.ts";
import { jobDir, paintingBlob } from "./painter.ts";
import type { Store } from "./store.ts";
import type { Config, Intent } from "./types.ts";

export class Posting {
  private token: string | null = null;
  private config: Config;
  private store: Store;
  private fetcher: typeof fetch;
  private env: NodeJS.ProcessEnv;
  constructor(config: Config, store: Store, fetcher: typeof fetch = fetch, env: NodeJS.ProcessEnv = process.env) {
    this.config = config;
    this.store = store;
    this.fetcher = fetcher;
    this.env = env;
  }

  private guard(intent: Intent): void {
    const current = this.store.intents().find(item => item.id === intent.id);
    const stopConfirmation = intent.id.endsWith("#stopped");
    if (stopped(this.config, this.env) || this.config.blockedDids.includes(intent.did) ||
        (!stopConfirmation && this.store.optedOut(intent.did, this.config)) || current?.status === "canceled") throw new Error("posting_blocked");
  }

  private async request(method: string, payload: Record<string, unknown>, intent: Intent, bytes?: Buffer): Promise<{ status: number; value: unknown }> {
    this.guard(intent);
    if (this.config.dryRun || !this.writable()) throw new Error("live_writes_disabled_without_livePosting");
    const url = new URL(`/xrpc/${method}`, this.config.pdsUrl);
    const read = method === "com.atproto.repo.getRecord";
    if (read) url.search = new URLSearchParams(Object.entries(payload).map(([key, value]) => [key, String(value)])).toString();
    try {
      const response = await this.fetcher(url, {
        method: read ? "GET" : "POST", redirect: "error", signal: AbortSignal.timeout(10000),
        headers: { "Content-Type": bytes ? "image/jpeg" : "application/json", ...(this.token ? { Authorization: `Bearer ${this.token}` } : {}) },
        ...(read ? {} : { body: bytes ? new Uint8Array(bytes) : JSON.stringify(payload) })
      });
      const value = await response.json() as unknown;
      if (response.status === 401 || ["ExpiredToken", "InvalidToken"].includes((value as { error?: string }).error ?? "")) this.token = null;
      return { status: response.status, value };
    } catch { throw new Error("pds_request_uncertain"); }
  }

  private writable(): boolean { return isLoopback(this.config.pdsUrl) || this.config.livePosting; }

  async send(intent: Intent, reserve: () => boolean): Promise<void> {
    this.guard(intent);
    if (this.config.dryRun) {
      if (!reserve()) return;
      this.store.db.prepare("UPDATE outbox SET status='simulated' WHERE id=? AND status='pending'").run(intent.id);
      return;
    }
    if (!this.writable()) throw new Error("live_writes_disabled_without_livePosting");
    if (this.token === null) {
      const identifier = this.env["DELVE_HANDLE"], password = this.env["DELVE_APP_PASSWORD"];
      if (!identifier || !password) throw new Error("credentials_missing");
      const session = await this.request("com.atproto.server.createSession", { identifier, password }, intent);
      const value = session.value as { did?: string; accessJwt?: string };
      if (session.status !== 200 || value.did !== this.config.did || !value.accessJwt) throw new Error("session_identity_mismatch");
      this.token = value.accessJwt;
    }
    const params = { repo: this.config.did, collection: "town.delve.feed.post", rkey: intent.rkey };
    const expected = `at://${this.config.did}/town.delve.feed.post/${intent.rkey}`;
    const record = JSON.parse(intent.record) as Record<string, unknown>;
    const verify = (value: unknown): void => {
      const result = value as { uri?: string; cid?: string; value?: unknown };
      if (result.uri !== expected || !result.cid || !isDeepStrictEqual(result.value, record)) throw new Error("readback_mismatch");
      this.store.db.prepare("UPDATE outbox SET status='verified' WHERE id=? AND status='pending'").run(intent.id);
    };
    const before = await this.request("com.atproto.repo.getRecord", params, intent);
    if (before.status === 200) { verify(before.value); return; }
    if (![400, 404].includes(before.status) || (before.value as { error?: string }).error !== "RecordNotFound") throw new Error("getRecord_failed");
    if (!reserve()) return;
    const embed = record["embed"] as { images: { image: unknown }[] } | undefined;
    if (embed) {
      const bytes = readFileSync(join(jobDir(this.config, intent.id.slice(0, intent.id.lastIndexOf("#"))), "painting.jpg"));
      const blob = paintingBlob(bytes, "image/jpeg");
      if (bytes.length > this.config.caps.imageBytes || !isDeepStrictEqual(embed.images[0]?.image, blob)) throw new Error("painting_mismatch");
      const uploaded = await this.request("com.atproto.repo.uploadBlob", {}, intent, bytes);
      if (uploaded.status !== 200 || !isDeepStrictEqual((uploaded.value as { blob?: unknown }).blob, blob)) throw new Error("blob_mismatch");
    }
    if (!reserve()) return;
    const created = await this.request("com.atproto.repo.createRecord", { ...params, record, validate: true }, intent);
    if (created.status !== 200 && (created.value as { error?: string }).error !== "RecordAlreadyExists") throw new Error("createRecord_failed");
    const after = await this.request("com.atproto.repo.getRecord", params, intent);
    if (after.status !== 200) throw new Error("readback_missing");
    verify(after.value);
    if (created.status === 200) {
      const written = created.value as { uri?: string; cid?: string };
      const readback = after.value as { uri?: string; cid?: string };
      if (written.uri !== readback.uri || written.cid !== readback.cid) {
        this.store.db.prepare("UPDATE outbox SET status='pending' WHERE id=?").run(intent.id);
        throw new Error("create_readback_mismatch");
      }
    }
  }
}
