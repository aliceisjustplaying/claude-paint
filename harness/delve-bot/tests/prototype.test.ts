import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { loadConfig, validateConfig } from "../src/config.ts";
import { fetchProfile, fetchThread, fetchTown } from "../src/delve.ts";
import { Engine, wording } from "../src/engine.ts";
import { addressed, postAuthor } from "../src/gate.ts";
import { parseEvent } from "../src/jetstream-parser.ts";
import { readMessage } from "../src/jetstream.ts";
import { Store } from "../src/store.ts";
import type { Config, Job, Post } from "../src/types.ts";
import { fakePainter, mockNetwork, NOW, PAINTING, post, view } from "./helpers.ts";

async function fixture(t: { after(fn: () => void | Promise<void>): void }, change: (config: Config) => void = () => {}) {
  const dir = mkdtempSync(join(tmpdir(), "stillwet-test-"));
  const network = await mockNetwork();
  const config = loadConfig();
  config.database = join(dir, "state.sqlite");
  config.killFile = join(dir, "STOP");
  config.appviewUrl = network.url;
  config.pdsUrl = network.url;
  config.anthropicUrl = network.url;
  change(config);
  validateConfig(config);
  const store = new Store(config.database);
  const env: NodeJS.ProcessEnv = { DELVE_HANDLE: "mock.invalid", DELVE_APP_PASSWORD: "mock-only-password", ANTHROPIC_API_KEY: "mock-anthropic-key" };
  const engine = new Engine(config, store, fetch, env, fakePainter());
  t.after(async () => { store.close(); await network.close(); rmSync(dir, { recursive: true, force: true }); });
  const add = (item: Post, thread: Post[] = [item]) => { network.threads.set(item.uri, thread); return item; };
  return { dir, config, store, env, engine, network, add };
}

test("config validates hard ceilings and keeps dryRun explicit/default true", () => {
  const config = loadConfig();
  assert.equal(config.dryRun, true);
  const invalid: unknown[] = [ { ...config, dryRun: undefined }, { ...config, caps: { ...config.caps, botCooldownSeconds: 3599 } },
    { ...config, caps: { ...config.caps, botOnlyTurns: 5 } }, { ...config, caps: { ...config.caps, running: 0 } },
    { ...config, appviewUrl: "http://evil.invalid" }, { ...config, pdsUrl: "https://user:password@pds.delve.town" },
    { ...config, surprise: true }, { ...config, caps: { ...config.caps, queue: -1 } } ];
  for (const item of invalid) assert.throws(() => validateConfig(item));
});

test("intake uses mentions or parent DID, not mere handle text or unrelated posts", async t => {
  const f = await fixture(t);
  const item = f.add(post());
  assert.equal(addressed(item, f.config), true);
  assert.equal(await f.engine.intake(item, NOW), "accepted");
  assert.equal(await f.engine.intake(item, NOW), "duplicate");
  const unrelated = { ...post("other"), mentions: [], text: "@stillwet.invalid paint a cat" };
  assert.equal(await f.engine.intake(unrelated, NOW), "unaddressed_or_invalid");
  const own = post("own", f.config.did);
  const reply = f.add({ ...post("reply", "did:plc:reply", "Paint a cat", own), mentions: [] }, [own, { ...post("reply", "did:plc:reply", "Paint a cat", own), mentions: [] }]);
  assert.equal(await f.engine.intake(reply, NOW), "queued");
  assert.equal(await f.engine.intake({ ...post("forged"), did: "did:plc:impostor" }, NOW), "unaddressed_or_invalid");
  assert.equal(postAuthor("at://did:plc:x/town.delve.feed.post/.."), null);
});

test("policy and size declines are short replies with no painter job", async t => {
  const f = await fixture(t);
  const unsafe = f.add(post("unsafe", "did:plc:unsafe", "Paint a real person"));
  assert.equal(await f.engine.intake(unsafe, NOW), "declinedPolicy");
  const large = f.add(post("large", "did:plc:large", "cat ".repeat(200)));
  assert.equal(await f.engine.intake(large, NOW), "declinedSize");
  assert.equal(f.store.jobs().length, 0);
  assert.equal(f.store.intents().length, 2);
  assert.ok(wording("declinedPolicy").length < 300);
});

test("global and requester reservations are atomic across competing admissions", async t => {
  const f = await fixture(t, c => { c.caps.paintingsPerDay = 2; c.caps.requesterPerDay = 2; });
  const one = f.add(post("one"));
  const same = f.add(post("same"));
  const two = f.add(post("two", "did:plc:two"));
  const three = f.add(post("three", "did:plc:three"));
  assert.equal(await f.engine.intake(one, NOW), "accepted");
  assert.equal(await f.engine.intake(same, NOW), "declinedLimit");
  const results = await Promise.all([f.engine.intake(two, NOW), f.engine.intake(three, NOW)]);
  assert.equal(results.filter(item => item === "queued").length, 1);
  assert.equal(results.filter(item => item === "declinedLimit").length, 1);
  assert.equal(f.store.jobs().length, 2);
});

test("per-requester daily cap persists after cooldown and across roots", async t => {
  const f = await fixture(t, c => { c.caps.requesterCooldownSeconds = 3600; });
  const one = f.add(post("one")), two = f.add(post("two"));
  assert.equal(await f.engine.intake(one, NOW), "accepted");
  two.createdAt = new Date(NOW + 3600001).toISOString();
  assert.equal(await f.engine.intake(two, NOW + 3600001), "declinedLimit");
});

test("queue is FIFO, capped and writes the unmodified house brief with request opening", async t => {
  const f = await fixture(t, c => { c.caps.queue = 1; });
  const one = f.add(post("one")), two = f.add(post("two", "did:plc:two")), three = f.add(post("three", "did:plc:three"));
  assert.equal(await f.engine.intake(one, NOW), "accepted");
  assert.equal(await f.engine.intake(two, NOW), "queued");
  assert.equal(await f.engine.intake(three, NOW), "declinedQueue");
  const job = JSON.parse(f.store.jobs()[0]!.job) as Job;
  assert.ok(job.brief.startsWith("# Paint a picture\n\nPaint a sleepy cat\n"));
  assert.ok(job.brief.includes("You make every artistic decision."));
  assert.equal(job.studio.engine, 7);
  assert.match(job.folder, /^paint-studio-[0-9a-f]{6}$/u);
  assert.ok(job.brief.includes(`/src/a/${job.folder}: this brief`));
  const order: string[] = [];
  await f.engine.pump(NOW, async job => { order.push(job.id); });
  await f.engine.pump(NOW + 1000, async job => { order.push(job.id); });
  assert.deepEqual(order, [one.uri, two.uri]);
  assert.equal(f.store.jobs().filter(row => row.status === "done").length, 2);
  assert.ok(f.store.intents().some(intent => JSON.parse(intent.record).text.includes("position 1")));
});

test("concurrent workers cannot exceed the running cap", async t => {
  const f = await fixture(t, c => { c.caps.running = 2; });
  for (let i = 0; i < 4; i++) await f.engine.intake(f.add(post(`job${i}`, `did:plc:author${i}`)), NOW);
  let current = 0, max = 0;
  const release = Promise.withResolvers<void>();
  const stub = async () => { current++; max = Math.max(max, current); await release.promise; current--; };
  const first = f.engine.pump(NOW, stub);
  await f.engine.pump(NOW, stub);
  assert.equal(current, 2);
  release.resolve();
  await first;
  await f.engine.pump(NOW + 1, stub);
  assert.equal(max, 2);
});

test("queued jobs expire and never consume a start", async t => {
  const f = await fixture(t, c => { c.caps.queueTtlSeconds = 1; });
  await f.engine.intake(f.add(post("one")), NOW);
  await f.engine.intake(f.add(post("two", "did:plc:two")), NOW);
  await f.engine.pump(NOW + 1001);
  const row = f.store.jobs()[1]!;
  assert.equal(row.status, "expired");
  assert.equal(row.started, null);
  assert.ok(f.store.intents().some(item => item.id.endsWith("#expired")));
});

test("bot-only turn limit is at least as strict as Trinity, including unknown actors", async t => {
  const f = await fixture(t);
  const root = post("root", "did:plc:root");
  const thread = [root, post("turn2", "did:plc:t2", "cat", root), post("turn3", "did:plc:t3", "cat", root), post("turn4", "did:plc:t4", "Paint a cat", root)];
  const item = f.add(thread[3]!, thread.map(item => ({ ...item, isBot: false })));
  assert.equal(await f.engine.intake(item, NOW), "bot_only_backoff");
  assert.equal(f.store.intents().length, 0);
});

test("bot cooldown delays done for an hour, with both replies counted", async t => {
  const f = await fixture(t);
  await f.engine.intake(f.add(post()), NOW);
  await f.engine.flush(NOW);
  await f.engine.pump(NOW);
  await f.engine.flush(NOW + 3599999);
  assert.deepEqual(f.store.intents().map(item => item.status), ["simulated", "pending"]);
  await f.engine.flush(NOW + 3600000);
  assert.deepEqual(f.store.intents().map(item => item.status), ["simulated", "simulated"]);
  const followup = f.add(post("followup", "did:plc:another", "cat", post()), [post(), post("followup", "did:plc:another", "cat", post())]);
  assert.equal(await f.engine.intake(followup, NOW), "bot_only_backoff");
});

test("stop cancels running/queued work and pending replies before hydration, and persists", async t => {
  const f = await fixture(t);
  await f.engine.intake(f.add(post()), NOW);
  const command = f.add(post("stop", "did:plc:requester1", "@stillwet.invalid stop"));
  f.engine.stage({ cursor: 10, operation: "create", uri: command.uri, post: command });
  assert.equal(f.store.jobs()[0]?.status, "canceled");
  assert.equal(f.store.intents()[0]?.status, "canceled");
  await f.engine.drain(NOW);
  await f.engine.flush(NOW);
  assert.equal(f.store.intents().filter(item => item.status === "simulated").length, 1);
  assert.equal(await f.engine.intake(f.add(post("new")), NOW), "opted_out");
  const reopened = new Store(f.config.database);
  assert.equal(reopened.optedOut(command.did, f.config), true);
  reopened.close();
});

test("kill env (even empty) and STOP file block admission, dispatch and completion", async t => {
  const f = await fixture(t);
  f.env["STILLWET_KILL_SWITCH"] = "";
  assert.equal(await f.engine.intake(f.add(post()), NOW), "kill_switch");
  delete f.env["STILLWET_KILL_SWITCH"];
  await f.engine.intake(f.add(post()), NOW);
  writeFileSync(f.config.killFile, "");
  await f.engine.flush(NOW);
  await f.engine.pump(NOW);
  assert.equal(f.store.intents()[0]?.status, "pending");
  assert.equal(f.store.jobs()[0]?.status, "running");
  rmSync(f.config.killFile);
  let complete!: () => void;
  const wait = new Promise<void>(resolve => { complete = resolve; });
  const pump = f.engine.pump(NOW, async () => wait);
  f.env["STILLWET_KILL_SWITCH"] = "1";
  complete(); await pump;
  assert.equal(f.store.jobs()[0]?.status, "running");
});

test("mock PDS exercises createSession, uploadBlob, createRecord and exact getRecord verification", async t => {
  const f = await fixture(t, c => { c.dryRun = false; });
  await f.engine.intake(f.add(post()), NOW);
  await f.engine.flush(NOW);
  await f.engine.pump(NOW);
  await f.engine.flush(NOW + 3600000);
  assert.deepEqual(f.store.intents().map(item => item.status), ["verified", "verified"]);
  assert.equal(f.network.records.size, 2);
  assert.equal(f.network.calls.filter(call => call.method === "com.atproto.repo.createRecord").length, 2);
  assert.equal(f.network.calls.filter(call => call.method === "com.atproto.repo.getRecord").length, 4);
  assert.equal(f.network.calls.filter(call => call.method === "com.atproto.repo.uploadBlob").length, 1);
  const done = JSON.parse(f.store.intents()[1]!.record);
  assert.equal(done.embed.$type, "town.delve.embed.images");
  assert.equal(done.embed.images[0].image.size, PAINTING.length);
  assert.equal(done.embed.images[0].image.mimeType, "image/jpeg");
  assert.deepEqual(done.reply.root, done.reply.parent);
});

test("an expired session logs in again on the next flush", async t => {
  const f = await fixture(t, c => { c.dryRun = false; });
  await f.engine.intake(f.add(post()), NOW);
  await f.engine.flush(NOW);
  await f.engine.pump(NOW);
  f.network.expireToken();
  await assert.rejects(f.engine.flush(NOW + 3600000), /getRecord_failed/u);
  await f.engine.flush(NOW + 3600001);
  assert.deepEqual(f.store.intents().map(item => item.status), ["verified", "verified"]);
  assert.equal(f.network.calls.filter(call => call.method === "com.atproto.server.createSession").length, 2);
});

test("uncertain create recovers after restart using the same frozen record key", async t => {
  const f = await fixture(t, c => { c.dryRun = false; });
  await f.engine.intake(f.add(post()), NOW);
  f.network.loseNextCreate();
  await assert.rejects(f.engine.flush(NOW), /uncertain/u);
  const frozen = f.store.intents()[0]!;
  const second = new Store(f.config.database);
  const restarted = new Engine(f.config, second, fetch, f.env, fakePainter());
  await restarted.flush(NOW + 1);
  assert.equal(second.intents()[0]?.rkey, frozen.rkey);
  assert.equal(second.intents()[0]?.record, frozen.record);
  assert.equal(second.intents()[0]?.status, "verified");
  assert.equal(f.network.calls.filter(call => call.method === "com.atproto.repo.createRecord").length, 1);
  second.close();
});

test("mismatched readback and session DID fail closed", async t => {
  const f = await fixture(t, c => { c.dryRun = false; });
  await f.engine.intake(f.add(post()), NOW);
  f.network.wrongDid(true);
  await assert.rejects(f.engine.flush(NOW), /identity_mismatch/u);
  assert.equal(f.network.records.size, 0);
  f.network.wrongDid(false);
  f.network.mismatch(true);
  await assert.rejects(f.engine.flush(NOW), /readback_mismatch/u);
  assert.equal(f.store.intents()[0]?.status, "pending");
});

test("guard is rechecked after asynchronous login before any write", async t => {
  const f = await fixture(t, c => { c.dryRun = false; });
  await f.engine.intake(f.add(post()), NOW);
  f.network.onLogin(() => { f.env["STILLWET_KILL_SWITCH"] = "1"; });
  await assert.rejects(f.engine.flush(NOW), /posting_blocked/u);
  assert.equal(f.network.records.size, 0);
});

test("production PDS cannot write even with dryRun=false and credentials present", async t => {
  const f = await fixture(t, c => { c.dryRun = false; c.pdsUrl = "https://pds.delve.town"; });
  await f.engine.intake(f.add(post()), NOW);
  await assert.rejects(f.engine.flush(NOW), /live_writes_disabled/u);
  assert.equal(f.network.calls.filter(call => call.verb !== "GET").length, 0);
});

test("dryRun never logs in or uploads an image", async t => {
  const f = await fixture(t);
  await f.engine.intake(f.add(post()), NOW);
  await f.engine.flush(NOW); await f.engine.pump(NOW); await f.engine.flush(NOW + 3600000);
  assert.equal(f.network.calls.some(call => call.verb !== "GET"), false);
  assert.equal(f.store.intents()[1]?.status, "simulated");
  const raw = readFileSync(f.config.database).toString();
  assert.equal(raw.includes("mock-only-password"), false);
});

test("appview feed, thread and profile use public GET requests", async t => {
  const f = await fixture(t);
  f.add(post());
  assert.equal((await fetchTown(f.config, 20)).length, 1);
  assert.equal((await fetchThread(post().uri, f.config)).length, 1);
  assert.equal((await fetchProfile(post().did, f.config) as { did: string }).did, post().did);
  assert.ok(f.network.calls.every(call => call.verb === "GET"));
});

test("Jetstream parser preserves valid mentions and durable rejected envelopes advance cursor", async t => {
  const f = await fixture(t);
  const source = f.add(post());
  const record = (view(source) as { record: unknown }).record;
  const envelope = { did: source.did, time_us: 100, kind: "commit", commit: { operation: "create", collection: "town.delve.feed.post", rkey: "request1", cid: source.cid, record } };
  const valid = parseEvent(envelope);
  assert.equal(valid?.post?.mentions[0], f.config.did);
  f.engine.stage(valid!);
  const rejected = readMessage(JSON.stringify({ ...envelope, time_us: 101, commit: { ...envelope.commit, cid: null } }), f.config);
  assert.equal(rejected?.rejectionReason, "invalid_jetstream_record");
  f.engine.stage(rejected!);
  assert.equal(f.store.cursor(), null);
  await f.engine.drain(NOW);
  assert.equal(f.store.cursor(), 101);
  assert.equal(f.store.jobs().length, 1);
  assert.equal(readMessage("no JSON", f.config), null);
});

test("ingest queue backpressure never advances the cursor over unhandled work", async t => {
  const f = await fixture(t, c => { c.caps.pendingEvents = 1; });
  f.engine.stage({ cursor: 1, operation: "create", uri: "rejected:1", post: null });
  assert.throws(() => f.engine.stage({ cursor: 2, operation: "create", uri: "rejected:2", post: null }), /queue_full/u);
  assert.equal(f.store.cursor(), null);
  await f.engine.drain(NOW);
  assert.equal(f.store.cursor(), 1);
});

test("updates and deletions cancel a queued request without admitting edited text", async t => {
  const f = await fixture(t);
  const source = f.add(post());
  await f.engine.intake(source, NOW);
  f.engine.stage({ cursor: 1, operation: "update", uri: source.uri, post: { ...source, text: "Paint a dragon" } });
  await f.engine.drain(NOW);
  assert.equal(f.store.jobs()[0]?.status, "canceled");
  assert.equal(f.store.intents()[0]?.status, "canceled");
});

test("global and thread reply caps include every reply kind", async t => {
  const f = await fixture(t, c => { c.caps.globalPerHour = 1; });
  await f.engine.intake(f.add(post("one")), NOW);
  await f.engine.intake(f.add(post("two", "did:plc:two", "Paint Alice")), NOW);
  await f.engine.flush(NOW);
  assert.equal(f.store.intents().filter(item => item.status === "simulated").length, 1);
  await f.engine.flush(NOW + 3600000);
  assert.equal(f.store.intents().filter(item => item.status === "simulated").length, 2);
  f.config.caps.threadPerSixHours = 1;
  await f.engine.pump(NOW + 3600000);
  await f.engine.flush(NOW + 7200000);
  assert.equal(f.store.intents().filter(item => item.status === "simulated").length, 2);
});

test("replay does not double-charge admission or lose a running job on restart", async t => {
  const f = await fixture(t);
  const source = f.add(post());
  await f.engine.intake(source, NOW);
  const second = new Store(f.config.database);
  const restarted = new Engine(f.config, second, fetch, f.env, fakePainter());
  assert.equal(await restarted.intake(source, NOW), "duplicate");
  await restarted.pump(NOW);
  assert.equal(second.jobs().length, 1);
  assert.equal(second.jobs()[0]?.status, "done");
  assert.equal(second.intents().length, 2);
  second.close();
});

test("daily start caps are rechecked across admission-day boundaries", async t => {
  const f = await fixture(t, c => { c.caps.paintingsPerDay = 1; c.caps.requesterPerDay = 4; });
  const old = f.add(post("old"));
  await f.engine.intake(old, NOW);
  f.store.db.prepare("UPDATE jobs SET admitted=?, started=?, status='done'").run(NOW - 86400001, NOW - 1000);
  const next = f.add(post("next", "did:plc:next"));
  assert.equal(await f.engine.intake(next, NOW), "queued");
  await f.engine.pump(NOW);
  assert.equal(f.store.jobs()[1]?.status, "queued");
  f.config.caps.queueTtlSeconds = 86400;
  await f.engine.pump(NOW + 86399000);
  assert.equal(f.store.jobs()[1]?.status, "done");
});

test("request size counts UTF-8 bytes as well as characters", async t => {
  const f = await fixture(t, c => { c.caps.requestChars = 2000; c.caps.requestBytes = 10; });
  assert.equal(await f.engine.intake(f.add(post("unicode", "did:plc:unicode", "🐱🐱🐱")), NOW), "declinedSize");
});

test("pending reply queue is bounded and delays completion instead of dropping it", async t => {
  const f = await fixture(t, c => { c.caps.pendingReplies = 1; });
  await f.engine.intake(f.add(post("one")), NOW);
  assert.equal(await f.engine.intake(f.add(post("two", "did:plc:two")), NOW), "reply_queue_full");
  await f.engine.pump(NOW);
  assert.equal(f.store.jobs()[0]?.status, "running");
  assert.equal(f.store.intents().length, 1);
  await f.engine.flush(NOW);
  await f.engine.pump(NOW);
  assert.equal(f.store.jobs()[0]?.status, "done");
  assert.equal(f.store.intents().length, 2);
});

test("database main-file page cap is applied", async t => {
  const f = await fixture(t);
  const limited = new Store(join(f.dir, "limited.sqlite"), "prototype", 1048576);
  const max = limited.db.prepare("PRAGMA max_page_count").get() as { max_page_count: number };
  assert.equal(max.max_page_count, 256);
  limited.close();
});

test("appview lag retains pending event and cursor until the thread is readable", async t => {
  const f = await fixture(t);
  const item = post();
  f.engine.stage({ cursor: 1, operation: "create", uri: item.uri, post: item });
  await assert.rejects(f.engine.drain(NOW), /appview_not_ready/u);
  assert.equal(f.store.cursor(), null);
  f.add(item);
  await f.engine.drain(NOW);
  assert.equal(f.store.cursor(), 1);
  assert.equal(f.store.jobs().length, 1);
});

test("a failed send reservation cannot authorize a later burst past cooldown", async t => {
  const f = await fixture(t, c => { c.dryRun = false; });
  await f.engine.intake(f.add(post()), NOW);
  f.network.wrongDid(true);
  await assert.rejects(f.engine.flush(NOW));
  await f.engine.pump(NOW);
  f.network.wrongDid(false);
  await f.engine.flush(NOW + 3600000);
  assert.equal(f.network.records.size, 1);
  assert.equal(f.store.intents()[1]?.status, "pending");
});

test("delete during in-flight hydration leaves a durable tombstone", async t => {
  const f = await fixture(t);
  const item = f.add(post());
  const response = Promise.withResolvers<void>();
  const read = Promise.withResolvers<void>();
  const fetcher: typeof fetch = async (input, init) => {
    const result = await fetch(input, init);
    read.resolve(); await response.promise; return result;
  };
  const engine = new Engine(f.config, f.store, fetcher, f.env, fakePainter());
  const admission = engine.intake(item, NOW);
  await read.promise;
  engine.stage({ cursor: 2, operation: "delete", uri: item.uri, post: null });
  response.resolve();
  assert.equal(await admission, "invalidated");
  assert.equal(f.store.jobs().length, 0);
});

test("same-thread burst reserves the completion turn and cannot strand all replies", async t => {
  const f = await fixture(t);
  const root = f.add(post());
  const child = f.add(post("child", "did:plc:child", "Paint a cat", root), [root, post("child", "did:plc:child", "Paint a cat", root)]);
  assert.equal(await f.engine.intake(root, NOW), "accepted");
  assert.equal(await f.engine.intake(child, NOW), "bot_only_backoff");
  await f.engine.pump(NOW);
  await f.engine.flush(NOW);
  await f.engine.flush(NOW + 3600000);
  assert.ok(f.store.intents().every(intent => intent.status !== "pending"));
  assert.equal(f.store.jobs().length, 1);
});

test("opt-out confirmations respect the pending reply bound", async t => {
  const f = await fixture(t, c => { c.caps.pendingReplies = 1; });
  for (let i = 0; i < 3; i++) await f.engine.intake(f.add(post(`stop${i}`, `did:plc:stopper${i}`, "stop")), NOW);
  assert.equal(f.store.intents().filter(intent => intent.status === "pending").length, 1);
  for (let i = 0; i < 3; i++) assert.equal(f.store.optedOut(`did:plc:stopper${i}`, f.config), true);
});

test("an expired job cannot start even if there is no room for its expiration reply", async t => {
  const f = await fixture(t, c => { c.caps.pendingReplies = 2; c.caps.queueTtlSeconds = 1; });
  await f.engine.intake(f.add(post("one")), NOW);
  await f.engine.intake(f.add(post("two", "did:plc:two")), NOW);
  await f.engine.intake(f.add(post("stop", "did:plc:requester1", "stop")), NOW + 2000);
  await f.engine.pump(NOW + 2000);
  await f.engine.flush(NOW + 2000);
  await f.engine.pump(NOW + 2001);
  assert.equal(f.store.jobs()[1]?.status, "expired");
  assert.equal(f.store.jobs()[1]?.started, null);
});

test("new opt-out or block cannot leave an unsendable head-of-line reply", async t => {
  const f = await fixture(t, c => { c.caps.queueTtlSeconds = 1; });
  await f.engine.intake(f.add(post("one")), NOW);
  await f.engine.intake(f.add(post("two", "did:plc:two")), NOW);
  f.config.optedOutDids.push("did:plc:two");
  f.config.blockedDids.push("did:plc:requester1");
  await f.engine.pump(NOW + 2000);
  await f.engine.flush(NOW + 2000);
  assert.equal(f.store.intents().filter(intent => intent.status === "pending").length, 0);
  assert.equal(f.store.intents().some(intent => intent.id.endsWith("#expired")), false);
});
