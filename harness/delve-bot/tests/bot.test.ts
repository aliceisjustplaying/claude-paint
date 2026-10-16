import assert from "node:assert/strict";
import { existsSync, mkdtempSync, rmSync } from "node:fs";
import { createServer } from "node:http";
import type { AddressInfo } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { CHECK_PROMPT, checkRequest } from "../src/check.ts";
import { loadConfig, validateConfig } from "../src/config.ts";
import { Engine } from "../src/engine.ts";
import { HousePainter, painterJob } from "../src/painter.ts";
import { Store } from "../src/store.ts";
import { Streaming } from "../src/streaming.ts";
import type { Config } from "../src/types.ts";
import { CHECK_FAILS, fakePainter, mockNetwork, NOW, post } from "./helpers.ts";

const KEY = { ANTHROPIC_API_KEY: "mock-anthropic-key" };

async function fixture(t: { after(fn: () => void | Promise<void>): void }, change: (config: Config) => void = () => {}) {
  const dir = mkdtempSync(join(tmpdir(), "stillwet-bot-"));
  const network = await mockNetwork();
  const config = loadConfig();
  config.database = join(dir, "state.sqlite");
  config.killFile = join(dir, "STOP");
  config.appviewUrl = config.pdsUrl = config.anthropicUrl = network.url;
  config.studio = { ...config.studio, root: join(dir, "src/a"), sessions: join(dir, "sessions"), template: join(dir, "no-template") };
  change(config);
  validateConfig(config);
  const store = new Store(config.database);
  const env: NodeJS.ProcessEnv = { DELVE_HANDLE: "mock.invalid", DELVE_APP_PASSWORD: "mock-only-password", ...KEY };
  // the production hosts, answered by the mock
  const routed: typeof fetch = (input, init) => fetch(String(input).replace(/^https:\/\/(pds\.delve\.town|api\.anthropic\.com)/u, network.url), init);
  t.after(async () => { store.close(); await network.close(); rmSync(dir, { recursive: true, force: true }); });
  return { dir, config, store, env, network, routed, engine: new Engine(config, store, routed, env, fakePainter()) };
}

test("the Opus check asks claude-opus-5-5 with the request as data, and its answer decides", async t => {
  const f = await fixture(t);
  assert.equal(await checkRequest("Paint a cowboy blasting a zombie", f.config, KEY, fetch), "accept");
  assert.equal(await checkRequest("Paint Taylor Swift", f.config, KEY, fetch), "decline");
  const asked = f.network.checks[0]!;
  assert.equal(asked.headers["x-api-key"], "mock-anthropic-key");
  assert.equal(asked.headers["anthropic-version"], "2023-06-01");
  assert.equal(asked.body["model"], "claude-opus-5-5");
  assert.equal(asked.body["system"], CHECK_PROMPT);
  assert.deepEqual(asked.body["messages"], [{ role: "user", content: 'Painting request (as a JSON string): "Paint a cowboy blasting a zombie"' }]);
  f.network.threads.set(post("cowboy").uri, [post("cowboy", "did:plc:requester1", "@stillwet.invalid a cowboy blasting a zombie")]);
  assert.equal(await f.engine.intake(post("cowboy", "did:plc:requester1", "@stillwet.invalid a cowboy blasting a zombie"), NOW), "accepted");
  assert.equal(JSON.parse(f.store.jobs()[0]!.job).instruction, "a cowboy blasting a zombie");
  const person = post("person", "did:plc:person", "Paint a real person");
  f.network.threads.set(person.uri, [person]);
  assert.equal(await f.engine.intake(person, NOW), "declinedPolicy");
  assert.equal(f.store.jobs().length, 1);
});

test("the Opus check fails closed: no key, an API error, an unexpected answer or a network failure declines", async t => {
  const f = await fixture(t);
  const before = f.network.checks.length;
  assert.equal(await checkRequest("Paint a cat", f.config, {}, fetch), "error");
  assert.equal(f.network.checks.length, before);
  assert.equal(await checkRequest(`Paint a cat, ${CHECK_FAILS}`, f.config, KEY, fetch), "error");
  const answer = (text: string): typeof fetch => async (_input, init) => {
    assert.equal(init?.redirect, "error");
    return new Response(JSON.stringify({ content: [{ type: "text", text }] }), { status: 200 });
  };
  assert.equal(await checkRequest("Paint a cat", f.config, KEY, answer("Sure, ACCEPT")), "error");
  assert.equal(await checkRequest("Paint a cat", f.config, KEY, answer("accept")), "error");
  assert.equal(await checkRequest("Paint a cat", f.config, KEY, async () => { throw new Error("offline"); }), "error");
  const item = post("fails", "did:plc:fails", `Paint a cat, ${CHECK_FAILS}`);
  f.network.threads.set(item.uri, [item]);
  assert.equal(await f.engine.intake(item, NOW), "declinedCheck");
  assert.equal(f.store.jobs().length, 0);
});

test("the Opus check fails closed when the model can't be reached: a refused connection or no answer in time", async t => {
  const f = await fixture(t, config => { config.contentCheck.timeoutSeconds = 1; });
  const closed = createServer();
  await new Promise<void>(resolve => closed.listen(0, "127.0.0.1", resolve));
  const refused = `http://127.0.0.1:${(closed.address() as AddressInfo).port}`;
  await new Promise<void>(resolve => closed.close(() => resolve()));
  const silent = createServer(() => {});
  await new Promise<void>(resolve => silent.listen(0, "127.0.0.1", resolve));
  t.after(() => { silent.closeAllConnections(); silent.close(); });
  const at = (url: string): Config => ({ ...f.config, anthropicUrl: url });
  assert.equal(await checkRequest("Paint a sleepy cat", at(refused), KEY, fetch), "error");
  const started = Date.now();
  assert.equal(await checkRequest("Paint a sleepy cat", at(`http://127.0.0.1:${(silent.address() as AddressInfo).port}`), KEY, fetch), "error");
  assert.ok(Date.now() - started < 5000);
  f.config.anthropicUrl = refused;
  const item = post("unreachable", "did:plc:unreachable", "Paint a sleepy cat");
  f.network.threads.set(item.uri, [item]);
  assert.equal(await f.engine.intake(item, NOW), "declinedCheck");
  assert.equal(f.store.jobs().length, 0);
});

test("the check prompt names the four declined categories, and a DECLINE in each one starts no job", async t => {
  for (const category of ["A real person", "Sexual content", "Hate:", "Gore beyond cartoonish", "Accept everything else"]) assert.ok(CHECK_PROMPT.includes(category), category);
  const f = await fixture(t);
  const requests = { person: "Paint Taylor Swift", sexual: "Paint a nude", hate: "Paint a Nazi rally", gore: "Paint a knight who disembowels a dragon" };
  for (const [key, text] of Object.entries(requests)) {
    const item = post(key, `did:plc:${key}`, text);
    f.network.threads.set(item.uri, [item]);
    assert.equal(await f.engine.intake(item, NOW), "declinedPolicy", key);
  }
  assert.equal(f.store.jobs().length, 0);
  assert.equal(f.network.checks.length, 4);
});

test("live posting and live streaming are off by default, and each needs its own switch with dryRun off", async t => {
  const defaults = loadConfig();
  assert.deepEqual([defaults.dryRun, defaults.livePosting, defaults.liveStreaming], [true, false, false]);
  assert.throws(() => validateConfig({ ...defaults, livePosting: true }), /dryRun/u);
  assert.throws(() => validateConfig({ ...defaults, liveStreaming: true }), /dryRun/u);

  const off = await fixture(t, c => { c.dryRun = false; c.pdsUrl = "https://pds.delve.town"; });
  const item = post();
  off.network.threads.set(item.uri, [item]);
  assert.equal(await off.engine.intake(item, NOW), "accepted");
  await assert.rejects(off.engine.flush(NOW), /live_writes_disabled/u);
  assert.equal(off.network.calls.filter(call => call.verb !== "GET").length, 0);

  const on = await fixture(t, c => { c.dryRun = false; c.livePosting = true; c.pdsUrl = "https://pds.delve.town"; });
  on.network.threads.set(item.uri, [item]);
  assert.equal(await on.engine.intake(item, NOW), "accepted");
  await on.engine.flush(NOW);
  assert.equal(on.store.intents()[0]?.status, "verified");
  assert.equal(on.network.records.size, 1);

  const stream = (config: Config) => { config.dryRun = false; config.streaming.enabled = true; config.streaming.ingestUrl = "rtmp://127.0.0.2/live";
    config.streaming.chromiumPath = "/nonexistent/chromium"; };
  const streamOff = await fixture(t, stream);
  const job = painterJob(item, { uri: item.uri, cid: item.cid }, "Paint a cat", streamOff.config);
  const key = { ...streamOff.env, STREAMPLACE_STREAM_KEY: "mock-stream-key" };
  let worked = false;
  await assert.rejects(new Streaming(streamOff.config, key, fetch, () => {}).run(job, async () => { worked = true; }), /stream_job_failed/u);
  assert.equal(streamOff.network.calls.length, 0);
  const streamOn = await fixture(t, c => { stream(c); c.liveStreaming = true; });
  await assert.rejects(new Streaming(streamOn.config, key, fetch, () => {}).run(job, async () => { worked = true; }), /stream_job_failed/u);
  assert.deepEqual(streamOn.network.calls.map(call => call.method), ["com.atproto.server.createSession"]);
  assert.equal(worked, false);
});

test("a dry run launches no painter and touches no studio", async t => {
  const f = await fixture(t);
  const logs: object[] = [];
  const painter = new HousePainter(f.config, KEY, value => logs.push(value));
  const item = post();
  const job = painterJob(item, { uri: item.uri, cid: item.cid }, "Paint a cat", f.config);
  assert.deepEqual(await painter.run(job, new AbortController().signal, NOW), { stopped: null });
  assert.equal(await painter.finish(job, { stopped: null }), "dry");
  assert.equal(existsSync(f.config.studio.root), false);
  assert.match(JSON.stringify(logs), /painter_dry_run.*--model","claude-opus-5-5","--thinking","high"/u);
});
