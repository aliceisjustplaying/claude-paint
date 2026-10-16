import { closeSync, existsSync, mkdirSync, openSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { loadConfig } from "./config.ts";
import { fetchProfile, fetchThread, fetchTown } from "./delve.ts";
import { Engine } from "./engine.ts";
import { consume } from "./jetstream.ts";
import { Store } from "./store.ts";
import type { Config } from "./types.ts";

async function demo(config: Config): Promise<void> {
  const { mockNetwork, NOW, post } = await import("../tests/helpers.ts");
  const network = await mockNetwork();
  config = structuredClone(config);
  config.appviewUrl = network.url;
  config.anthropicUrl = network.url;
  config.dryRun = true;
  const store = new Store(":memory:");
  const engine = new Engine(config, store, fetch, { ANTHROPIC_API_KEY: "mock-anthropic-key" });
  const first = post("demo1");
  const second = post("demo2", "did:plc:requester2", "Paint a friendly cartoon monster");
  const unsafe = post("demo3", "did:plc:requester3", "Paint a famous real person");
  try {
    console.log("OFFLINE MOCK TRANSCRIPT — synthetic requests, a mocked content check, dryRun=true, no painter and no live writes");
    for (const item of [first, second, unsafe]) {
      network.threads.set(item.uri, [item]);
      console.log(`request: ${item.text}\nresult: ${await engine.intake(item, NOW)}`);
    }
    await engine.flush(NOW);
    await engine.pump(NOW);
    await engine.pump(NOW + 1000);
    await engine.flush(NOW + 3600000);
    for (const intent of store.intents()) console.log(JSON.stringify({ id: intent.id.split("/").at(-1), status: intent.status, record: JSON.parse(intent.record) }));
    console.log(JSON.stringify({ jobs: store.jobs().map(row => ({ id: row.id.split("/").at(-1), status: row.status })), pdsWrites: network.calls.filter(call => call.verb !== "GET").length }));
  } finally { store.close(); await network.close(); }
}

async function liveRead(config: Config): Promise<void> {
  if (!config.dryRun) throw new Error("live_read_requires_dry_run");
  let reads = 0;
  const publicFetch: typeof fetch = async (input, init) => {
    const url = new URL(String(input));
    if ((init?.method ?? "GET") !== "GET" || init?.headers || url.origin !== "https://api.delve.town") throw new Error("nonpublic_request_blocked");
    reads++;
    return fetch(input, init);
  };
  const posts = await fetchTown(config, 20, publicFetch);
  const first = posts[0];
  if (!first) throw new Error("empty_public_feed");
  const thread = await fetchThread(first.uri, config, publicFetch);
  const profile = await fetchProfile(first.did, config, publicFetch) as { did?: string };
  const store = new Store(":memory:");
  const engine = new Engine(config, store, publicFetch, {});
  const decisions: Record<string, number> = {};
  try {
    for (const item of posts) {
      const reason = await engine.intake(item);
      decisions[reason] = (decisions[reason] ?? 0) + 1;
    }
    await engine.flush();
    console.log(JSON.stringify({ observedAt: new Date().toISOString(), dryRun: config.dryRun, account: config.did,
      publicGetRequests: reads, credentialsUsed: false, writes: 0, feedPosts: posts.length, threadPosts: thread.length,
      profileDidMatched: profile.did === first.did, sampleUri: first.uri, decisions,
      intents: store.intents().length, note: "The configured Stillwet DID is a placeholder, not a live account. No addressed requests are expected." }, null, 2));
  } finally { store.close(); }
}

async function run(config: Config): Promise<void> {
  mkdirSync(dirname(config.database), { recursive: true });
  const path = `${config.database}.lock`;
  if (existsSync(path)) {
    const pid = Number(readFileSync(path, "utf8"));
    if (!Number.isSafeInteger(pid) || pid <= 0) throw new Error("invalid_lock_file");
    try { process.kill(pid, 0); throw new Error("daemon_already_running"); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ESRCH") throw error; }
    unlinkSync(path);
  }
  const fd = openSync(path, "wx", 0o600);
  writeFileSync(fd, String(process.pid));
  const store = new Store(config.database, `${config.did}:${config.dryRun}`, config.caps.databaseBytes);
  const engine = new Engine(config, store);
  const controller = new AbortController();
  const stop = () => controller.abort();
  process.once("SIGINT", stop);
  process.once("SIGTERM", stop);
  const pumps = new Set<Promise<void>>();
  let flushed = Promise.resolve();
  let flushing = false;
  const timer = setInterval(() => {
    const pump: Promise<void> = engine.pump().catch(() => console.error("worker_paused_after_error")).finally(() => pumps.delete(pump));
    pumps.add(pump);
    if (flushing) return;
    flushing = true;
    flushed = engine.flush().catch(() => console.error("flush_paused_after_error")).finally(() => { flushing = false; });
  }, 1000);
  try { await consume(engine, controller.signal); }
  finally {
    clearInterval(timer);
    await engine.streaming.stop();
    await Promise.all([...pumps, flushed]);
    store.close(); closeSync(fd); unlinkSync(path);
    process.removeListener("SIGINT", stop); process.removeListener("SIGTERM", stop);
  }
}

const command = process.argv[2] ?? "demo";
const config = loadConfig(process.argv[3] ? resolve(process.argv[3]) : "config.json");
try {
  if (command === "demo") await demo(config);
  else if (command === "live-read") await liveRead(config);
  else if (command === "run") await run(config);
  else throw new Error("unknown_command");
} catch (error) {
  console.error(error instanceof Error ? error.message : "prototype_failed");
  process.exitCode = 1;
}
