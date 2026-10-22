import { readFileSync } from "node:fs";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import type { Post } from "../src/types.ts";
import { loadConfig } from "../src/config.ts";
import { paintingBlob } from "../src/painter.ts";
import type { Painter } from "../src/painter.ts";

// Original procedural JPEG from scripts/generate_demo_fixtures.py, standing in for a finished painting.
export const PAINTING = readFileSync(new URL("./fixtures/painting.jpg", import.meta.url));
export const DECLINED = ["real person", "Alice", "Taylor Swift", "nude", "Nazi", "disembowel"];
export const CHECK_FAILS = "check fails";

export function fakePainter(): Painter {
  return {
    async run() {},
    async finish(_job, run) { return { bytes: PAINTING, width: 1000, height: 714, alt: "A calm sea at dusk.", stopped: run?.stopped ?? null }; }
  };
}

export const NOW = Date.parse("2026-10-16T08:00:00Z");
export function post(key = "request1", did = "did:plc:requester1", text = "Paint a sleepy cat", root?: Post): Post {
  const config = loadConfig();
  const uri = `at://${did}/town.delve.feed.post/${key}`;
  return { uri, did, text, cid: `bafy-${key}`, handle: "requester.invalid", createdAt: new Date(NOW).toISOString(), rootUri: root?.rootUri ?? uri, parentUri: root?.uri ?? null, mentions: [config.did], isBot: true };
}
export function view(item: Post): object {
  return { uri: item.uri, cid: item.cid, author: { did: item.did, handle: item.handle, labels: [{ val: "bot" }] }, record: {
    $type: "town.delve.feed.post", text: item.text, createdAt: item.createdAt,
    facets: item.mentions.map(did => ({ index: { byteStart: 0, byteEnd: 1 }, features: [{ $type: "town.delve.richtext.facet#mention", did }] })),
    ...(item.parentUri ? { reply: { parent: { uri: item.parentUri, cid: "parent-cid" }, root: { uri: item.rootUri, cid: "root-cid" } } } : {})
  } };
}
export async function mockNetwork() {
  const threads = new Map<string, Post[]>();
  const records = new Map<string, { uri: string; cid: string; value: unknown }>();
  const calls: { method: string; verb: string }[] = [];
  const checks: { headers: Record<string, string | string[] | undefined>; body: Record<string, unknown> }[] = [];
  let loseCreateResponse = false, mismatch = false, wrongDid = false, expire = false, stopOnLogin: (() => void) | null = null;
  const server = createServer(async (req, res) => {
    const url = new URL(req.url ?? "/", "http://127.0.0.1");
    const method = url.pathname.split("/").at(-1) ?? "";
    if (url.pathname !== "/v1/messages") calls.push({ method, verb: req.method ?? "" });
    const chunks: Buffer[] = [];
    for await (const chunk of req) chunks.push(Buffer.from(chunk as Buffer));
    const bytes = Buffer.concat(chunks);
    const body = bytes.length && method !== "com.atproto.repo.uploadBlob" ? JSON.parse(bytes.toString()) as Record<string, unknown> : {};
    const json = (value: unknown, status = 200): void => { res.writeHead(status, { "Content-Type": "application/json" }); res.end(JSON.stringify(value)); };
    if (url.pathname === "/v1/messages") {
      checks.push({ headers: req.headers, body });
      const asked = JSON.stringify(body["messages"]);
      if (asked.includes(CHECK_FAILS)) { json({ type: "error", error: { type: "overloaded_error" } }, 529); return; }
      json({ content: [{ type: "text", text: DECLINED.some(word => asked.includes(word)) ? "DECLINE" : "ACCEPT" }] });
    } else if (method === "town.delve.feed.getPostThread") {
      const items = threads.get(url.searchParams.get("uri") ?? "") ?? [];
      const nodes = items.map(item => ({ post: view(item) }));
      json({ thread: nodes.length ? { ...nodes[0], replies: nodes.slice(1) } : null });
    } else if (method === "town.delve.actor.getProfile") json({ did: url.searchParams.get("actor"), handle: "requester.invalid" });
    else if (method === "town.delve.feed.getFeed") json({ feed: [...threads.values()].flat().map(item => ({ post: view(item) })) });
    else if (method === "com.atproto.server.createSession") {
      stopOnLogin?.();
      json({ did: wrongDid ? "did:plc:wrong" : loadConfig().did, accessJwt: "mock-only-access", refreshJwt: "mock-only-refresh" });
    } else if (expire && req.headers.authorization) { expire = false; json({ error: "ExpiredToken" }, 400); }
    else if (method === "com.atproto.repo.getRecord") {
      const record = records.get(url.searchParams.get("rkey") ?? "");
      json(record ? { ...record, ...(mismatch ? { value: { text: "wrong" } } : {}) } : { error: "RecordNotFound" }, record ? 200 : 400);
    } else if (method === "com.atproto.repo.uploadBlob") json({ blob: paintingBlob(bytes, req.headers["content-type"] ?? "") });
    else if (method === "com.atproto.repo.createRecord") {
      const rkey = body["rkey"] as string;
      if (records.has(rkey)) { json({ error: "RecordAlreadyExists" }, 400); return; }
      const record = { uri: `at://${body["repo"]}/town.delve.feed.post/${rkey}`, cid: `mock-cid-${rkey}`, value: body["record"] };
      records.set(rkey, record);
      if (loseCreateResponse) { loseCreateResponse = false; req.socket.destroy(); return; }
      json({ uri: record.uri, cid: record.cid });
    } else json({ error: "UnknownMethod" }, 404);
  });
  await new Promise<void>(resolve => server.listen(0, "127.0.0.1", resolve));
  return {
    url: `http://127.0.0.1:${(server.address() as AddressInfo).port}`,
    threads, records, calls, checks,
    loseNextCreate() { loseCreateResponse = true; },
    mismatch(value: boolean) { mismatch = value; },
    wrongDid(value: boolean) { wrongDid = value; },
    expireToken() { expire = true; },
    onLogin(fn: () => void) { stopOnLogin = fn; },
    async close() { server.closeAllConnections(); await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())); }
  };
}
