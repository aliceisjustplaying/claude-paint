import { createHash } from "node:crypto";
import { setTimeout as delay } from "node:timers/promises";
import { parseEvent } from "./jetstream-parser.ts";
import type { Engine } from "./engine.ts";
import type { Config, IngestEvent } from "./types.ts";

export function readMessage(data: unknown, config: Config): IngestEvent | null {
  let envelope: unknown = null;
  try {
    if (typeof data !== "string" || Buffer.byteLength(data) > config.caps.eventBytes) throw new Error("invalid_frame");
    envelope = JSON.parse(data) as unknown;
    return parseEvent(envelope, config.botDids);
  } catch {
    const cursor = typeof envelope === "object" && envelope !== null && "time_us" in envelope ? envelope.time_us : null;
    if (typeof cursor !== "number" || !Number.isSafeInteger(cursor) || cursor < 0) return null;
    return { cursor, operation: "create", uri: `rejected:${createHash("sha256").update(JSON.stringify(envelope)).digest("hex")}`, post: null, rejectionReason: "invalid_jetstream_record" };
  }
}

export async function consume(engine: Engine, signal: AbortSignal): Promise<void> {
  while (!signal.aborted) {
    try { await engine.drain(); }
    catch {}
    const url = new URL(engine.config.jetstreamUrl);
    url.searchParams.set("wantedCollections", "town.delve.feed.post");
    url.searchParams.set("compress", "false");
    const cursor = engine.store.cursor();
    if (cursor !== null) url.searchParams.set("cursor", String(cursor));
    await new Promise<void>((resolve, reject) => {
      const socket = new WebSocket(url);
      let processing = Promise.resolve();
      let draining = false;
      let failed = false;
      const drain = () => {
        if (draining) return;
        draining = true;
        processing = engine.drain().catch(() => {}).finally(() => { draining = false; });
      };
      const retry = setInterval(drain, 1000);
      const abort = () => socket.close();
      const timeout = setTimeout(() => socket.close(), 10000);
      signal.addEventListener("abort", abort, { once: true });
      socket.addEventListener("open", () => clearTimeout(timeout), { once: true });
      socket.addEventListener("message", event => {
        if (failed || signal.aborted) return;
        try {
          const parsed = readMessage(event.data, engine.config);
          if (!parsed) return;
          engine.stage(parsed);
          drain();
        } catch { failed = true; socket.close(); }
      });
      socket.addEventListener("error", abort, { once: true });
      socket.addEventListener("close", () => {
        clearTimeout(timeout);
        clearInterval(retry);
        signal.removeEventListener("abort", abort);
        void processing.then(() => failed ? reject(new Error("jetstream_processing_failed")) : resolve());
      }, { once: true });
      if (signal.aborted) abort();
    }).catch(() => {});
    if (!signal.aborted) await delay(1000, undefined, { signal }).catch(() => {});
  }
}
