import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import type { Engine } from "../src/engine.ts";
import { consume } from "../src/jetstream.ts";
import type { Config, IngestEvent } from "../src/types.ts";

test("the consumer receives a later delete while startup or message drains keep failing", async t => {
  for (const timing of ["startup", "message"]) await t.test(timing, async t => {
    const config = JSON.parse(readFileSync(new URL("../config.json", import.meta.url), "utf8")) as Config;
    const stop = new AbortController();
    const sockets: Socket[] = [];
    const staged: IngestEvent[] = [];
    let failures = 0;
    let pending = timing === "startup";
    class Socket extends EventTarget {
      closed = false;
      readonly url: URL;
      constructor(url: URL) {
        super();
        this.url = url;
        sockets.push(this);
        queueMicrotask(() => this.dispatchEvent(new Event("open")));
      }
      close(): void {
        if (this.closed) return;
        this.closed = true;
        this.dispatchEvent(new Event("close"));
      }
      receive(operation: "create" | "delete", cursor: number): void {
        if (this.closed) return;
        this.dispatchEvent(new MessageEvent("message", { data: JSON.stringify({
          kind: "commit", did: "did:plc:requester", time_us: cursor,
          commit: { collection: "town.delve.feed.post", rkey: "request", operation, cid: "bafy-request",
            record: { $type: "town.delve.feed.post", text: "Paint a cat", createdAt: new Date().toISOString() } }
        }) }));
      }
    }
    const original = globalThis.WebSocket;
    globalThis.WebSocket = Socket as unknown as typeof WebSocket;
    const engine = {
      config,
      store: { cursor: () => 42 },
      stage(event: IngestEvent) { staged.push(event); pending = true; },
      async drain() {
        if (!pending) return;
        failures++;
        throw new Error("appview_not_ready");
      }
    } as unknown as Engine;
    const consuming = consume(engine, stop.signal);
    t.after(async () => {
      stop.abort();
      try { await consuming; }
      finally { globalThis.WebSocket = original; }
    });
    await t.waitFor(() => assert.equal(sockets.length, 1));
    const socket = sockets[0]!;
    assert.equal(socket.url.searchParams.get("cursor"), "42");
    socket.receive("create", 43);
    await t.waitFor(() => assert.ok(failures >= (timing === "startup" ? 2 : 1)), { timeout: 2500 });
    const beforeIdle = failures;
    await t.waitFor(() => assert.ok(failures > beforeIdle), { timeout: 2500 });
    assert.equal(socket.closed, false);
    const beforeDelete = failures;
    socket.receive("delete", 44);
    await t.waitFor(() => assert.equal(staged.at(-1)?.operation, "delete"));
    assert.equal(staged.at(-1)?.uri, "at://did:plc:requester/town.delve.feed.post/request");
    await t.waitFor(() => assert.ok(failures > beforeDelete), { timeout: 2500 });
    assert.equal(sockets.length, 1);
    assert.equal(socket.closed, false);
  });
});
