import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createServer } from "node:http";
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { pathToFileURL } from "node:url";
import type { AddressInfo } from "node:net";
import { test } from "node:test";
import { loadConfig, validateConfig } from "../src/config.ts";
import { ffmpegArgs } from "../src/capture.ts";
import { Engine } from "../src/engine.ts";
import { painterJob } from "../src/painter.ts";
import { Store } from "../src/store.ts";
import { Streaming, streamLink, studioUrl } from "../src/streaming.ts";
import { mockNetwork, NOW, post } from "./helpers.ts";

async function fixture(t: { after(fn: () => void | Promise<void>): void }, real = false) {
  const dir = mkdtempSync(join(tmpdir(), "stillwet-stream-"));
  const config = loadConfig();
  config.streaming.enabled = true;
  config.streaming.includeLink = true;
  config.streaming.width = 320; config.streaming.height = 180;
  config.streaming.fps = 5; config.streaming.videoKbps = 128;
  config.streaming.maxSeconds = 30;
  config.database = join(dir, "state.sqlite");
  config.killFile = join(dir, "STOP");
  config.dryRun = !real;
  const output = join(dir, "capture.flv");
  if (real) config.streaming.ingestUrl = pathToFileURL(output).href;
  const env: NodeJS.ProcessEnv = { STREAMPLACE_STREAM_KEY: "mock-key-never-log-this", DELVE_HANDLE: "mock.invalid", DELVE_APP_PASSWORD: "mock-password-never-log-this", ANTHROPIC_API_KEY: "mock-anthropic-key" };
  const logs: object[] = [];
  const network = await mockNetwork();
  config.appviewUrl = network.url;
  config.anthropicUrl = network.url;
  const records = new Map<string, { uri: string; cid: string; value: Record<string, unknown> }>();
  const calls: { method: string; body: Record<string, unknown> }[] = [];
  const created = Promise.withResolvers<void>();
  let failCreate = false, redirect = false, failPut = false, wrongDid = false;
  const pds = createServer(async (req, res) => {
    const url = new URL(req.url!, "http://127.0.0.1");
    const chunks: Buffer[] = [];
    for await (const chunk of req) chunks.push(Buffer.from(chunk));
    const body = chunks.length ? JSON.parse(Buffer.concat(chunks).toString()) as Record<string, unknown> : {};
    const method = url.pathname.split("/").at(-1)!;
    calls.push({ method, body });
    const send = (value: object, status = 200) => { res.writeHead(status, { "Content-Type": "application/json" }); res.end(JSON.stringify(value)); };
    if (redirect) { res.writeHead(307, { Location: "https://must-not-follow.invalid" }); res.end(); return; }
    if (method === "com.atproto.server.createSession") {
      send({ did: wrongDid ? "did:plc:wrong" : config.did, accessJwt: "mock-token-never-log-this" }); return;
    }
    assert.equal(req.headers.authorization, "Bearer mock-token-never-log-this");
    const rkey = (body["rkey"] ?? url.searchParams.get("rkey")) as string;
    const record = records.get(rkey);
    if (method === "com.atproto.repo.createRecord") {
      assert.match(rkey, /^[234567abcdefghij][234567abcdefghijklmnopqrstuvwxyz]{12}$/u);
      assert.equal(body["collection"], "place.stream.livestream");
      const entry = { uri: `at://${config.did}/place.stream.livestream/${rkey}`, cid: "created-cid", value: body["record"] as Record<string, unknown> };
      records.set(rkey, entry);
      created.resolve();
      if (failCreate) { req.socket.destroy(); return; }
      send({ uri: entry.uri, cid: entry.cid });
    } else if (method === "com.atproto.repo.getRecord") {
      if (record) { record.cid = "concurrent-update-cid"; record.value["tags"] = ["preserved"]; send(record); }
      else send({ error: "RecordNotFound" }, 400);
    } else if (method === "com.atproto.repo.putRecord") {
      if (failPut) { send({ error: "mock_failure" }, 500); return; }
      assert.equal(body["swapRecord"], "concurrent-update-cid");
      assert.deepEqual((body["record"] as Record<string, unknown>)["tags"], ["preserved"]);
      assert.ok(record);
      record.value = body["record"] as Record<string, unknown>;
      send({ uri: record.uri, cid: "ended-cid" });
    } else send({ error: "UnknownMethod" }, 404);
  });
  await new Promise<void>(resolve => pds.listen(0, "127.0.0.1", resolve));
  config.pdsUrl = `http://127.0.0.1:${(pds.address() as AddressInfo).port}`;
  const requests: string[] = [];
  const viewer = real ? createServer((req, res) => {
    requests.push(req.url!);
    res.setHeader("Content-Type", "text/html");
    res.end('<html><body style="margin:0;background:rgb(10,20,160);width:100vw;height:100vh"></body></html>');
  }) : undefined;
  if (viewer) await new Promise<void>((resolve, reject) => { viewer.once("error", reject); viewer.listen(8766, "127.0.0.1", resolve); });
  const streaming = new Streaming(config, env, fetch, value => logs.push(value));
  const item = post();
  const job = painterJob(item, { uri: item.uri, cid: item.cid }, "Paint a cat", config);
  t.after(async () => {
    await streaming.stop();
    await network.close();
    for (const server of [pds, viewer]) if (server) { server.closeAllConnections(); await new Promise<void>(resolve => server.close(() => resolve())); }
    rmSync(dir, { recursive: true, force: true });
  });
  return { config, env, streaming, job, logs, calls, records, output, requests, network, created: created.promise,
    failCreate() { failCreate = true; }, redirect() { redirect = true; }, failPut() { failPut = true; }, wrongDid() { wrongDid = true; } };
}

test("stream configuration enforces concurrency, duration, resolution, fps and bitrate ceilings", () => {
  const config = loadConfig();
  for (const change of [{ concurrent: 2 }, { maxSeconds: 7201 }, { width: 1282 }, { height: 722 },
    { fps: 16 }, { videoKbps: 1501 }, { audioKbps: 97 }, { width: 321 },
    { ingestUrl: "rtmps://user:secret@rtmp.stream.place/live" }, { ingestUrl: "rtmps://rtmp.stream.place/live/key" },
    { stationUrl: "https://stream.place/user?key=secret" }]) {
    assert.throws(() => validateConfig({ ...config, streaming: { ...config.streaming, ...change } }));
  }
  assert.throws(() => validateConfig({ ...config, streaming: { ...config.streaming, enabled: true }, caps: { ...config.caps, running: 2 } }));
  const args = ffmpegArgs(config, "test.flv");
  assert.equal(args[args.indexOf("-bf") + 1], "0");
  assert.equal(args[args.indexOf("-g") + 1], String(config.streaming.fps));
  assert.equal(args[args.indexOf("-maxrate") + 1], "1200k");
});

test("dry run logs capture and both records without network, browser, encoder or secret access", async t => {
  const f = await fixture(t);
  let used = false;
  const neverFetch: typeof fetch = async () => { used = true; throw new Error("no network"); };
  const env: NodeJS.ProcessEnv = new Proxy({}, { get(_target, key) {
    if (key === "STREAMPLACE_STREAM_KEY") throw new Error("dry_run_read_a_secret");
    return undefined;
  } });
  const streaming = new Streaming(f.config, env, neverFetch, value => f.logs.push(value));
  await streaming.run(f.job, async () => {});
  assert.equal(used, false);
  assert.equal(f.calls.length, 0);
  const text = JSON.stringify(f.logs);
  assert.match(text, /ffmpeg/u); assert.match(text, /createRecord/u); assert.match(text, /putRecord/u); assert.match(text, /endedAt/u);
  assert.match(text, /STREAMPLACE_STREAM_KEY/u); assert.doesNotMatch(text, /never-log-this/u);
  const url = new URL(studioUrl(f.config, f.job));
  assert.equal(url.origin, "http://127.0.0.1:8766"); assert.equal(url.searchParams.get("stream"), "1");
  assert.equal(url.searchParams.get("p"), f.job.folder);
});

test("stream slot is reserved before awaiting work and released after failure", async t => {
  const f = await fixture(t);
  const release = Promise.withResolvers<void>();
  const running = f.streaming.run(f.job, () => release.promise);
  await assert.rejects(new Streaming(f.config, {}, fetch, () => {}).run(f.job, async () => {}), /stream_slot_busy/u);
  release.reject(new Error("work failed"));
  await assert.rejects(running, /stream_job_failed/u);
  await f.streaming.run(f.job, async () => {});
  assert.equal(f.logs.length, 4);
});

test("the stream's wall-clock limit stops the painter as a time limit and keeps its result", async t => {
  const f = await fixture(t);
  f.config.streaming.maxSeconds = 1;
  const start = performance.now();
  const result = await f.streaming.run(f.job, s => new Promise(resolve => s.addEventListener("abort", () => resolve(s.reason), { once: true })));
  assert.equal(result, "time");
  assert.ok(performance.now() - start < 2500);
  assert.match(JSON.stringify(f.logs.at(-1)), /endedAt/u);
});

test("a live stream's wall-clock limit waits for the painter's own time stop", async t => {
  const f = await fixture(t, true);
  f.config.streaming.maxSeconds = 10;
  const result = await f.streaming.run(f.job, s => new Promise(resolve =>
    s.addEventListener("abort", () => setTimeout(() => resolve(s.reason), 1000), { once: true })));
  assert.equal(result, "time");
  assert.ok([...f.records.values()][0]?.value["endedAt"]);
});

test("shutdown interrupts a painting that isn't streamed", async t => {
  const f = await fixture(t);
  f.config.streaming.enabled = false;
  let stoppedWork = false;
  const run = f.streaming.run(f.job, s => new Promise(resolve => s.addEventListener("abort", () => { stoppedWork = true; resolve("stopped"); }, { once: true })));
  await f.streaming.stop();
  await assert.rejects(run, /interrupted_by_shutdown/u);
  assert.equal(stoppedWork, true);
});

test("kill env, STOP file, job cancellation and shutdown stop active streams", async t => {
  const f = await fixture(t);
  for (const trigger of ["env", "file", "job", "shutdown"]) {
    let canceled = false;
    const run = f.streaming.run(f.job, async () => {
      if (trigger === "env") f.env["STILLWET_KILL_SWITCH"] = "";
      if (trigger === "file") writeFileSync(f.config.killFile, "");
      if (trigger === "job") canceled = true;
      if (trigger === "shutdown") queueMicrotask(() => { void f.streaming.stop(); });
      await new Promise(() => {});
    }, () => canceled);
    await assert.rejects(run, /canceled/u);
    delete f.env["STILLWET_KILL_SWITCH"]; rmSync(f.config.killFile, { force: true });
  }
  assert.equal(f.logs.length, 8);
});

test("production ingest or PDS stays hard-disabled and redirects are not followed", async t => {
  const f = await fixture(t);
  f.config.dryRun = false;
  await assert.rejects(f.streaming.run(f.job, async () => {}), /stream_job_failed/u);
  assert.equal(f.calls.length, 0);
  f.config.streaming.ingestUrl = "rtmp://127.0.0.1:1935/live";
  f.config.pdsUrl = "https://pds.delve.town";
  await assert.rejects(f.streaming.run(f.job, async () => {}), /stream_job_failed/u);
  assert.equal(f.calls.length, 0);
  f.config.pdsUrl = f.network.url;
  const redirects: typeof fetch = async (_url, init) => { assert.equal(init?.redirect, "error"); throw new Error(f.env["STREAMPLACE_STREAM_KEY"]); };
  await assert.rejects(new Streaming(f.config, f.env, redirects).run(f.job, async () => {}), error => {
    assert.doesNotMatch(String(error), /never-log-this/u); return true;
  });
});

test("mock session rejects identity mismatch and HTTP redirects before capture", async t => {
  const f = await fixture(t);
  f.config.dryRun = false;
  f.config.streaming.ingestUrl = pathToFileURL(f.output).href;
  f.wrongDid();
  await assert.rejects(f.streaming.run(f.job, async () => {}), /stream_job_failed/u);
  assert.deepEqual(f.calls.map(call => call.method), ["com.atproto.server.createSession"]);
  f.redirect();
  await assert.rejects(f.streaming.run(f.job, async () => {}), /stream_job_failed/u);
  assert.equal(f.calls.length, 2);
});

test("real headless viewer frames reach local FLV sink and PDS record ends with a CID swap", async t => {
  const f = await fixture(t, true);
  await f.streaming.run(f.job, async () => { await delay(1800); });
  assert.equal(f.records.size, 1);
  const record = [...f.records.values()][0]!;
  assert.ok(record.value["endedAt"]);
  assert.equal(record.value["canonicalUrl"], streamLink(f.config));
  assert.ok(f.requests.some(request => new URL(request, "http://127.0.0.1").searchParams.get("stream") === "1"));
  const probe = JSON.parse(execFileSync("ffprobe", ["-v", "error", "-count_frames", "-show_streams", "-of", "json", f.output], { encoding: "utf8" })) as { streams: { codec_type: string; codec_name: string; width: number; height: number; nb_read_frames: string }[] };
  const video = probe.streams.find(stream => stream.codec_type === "video")!;
  assert.equal(video.codec_name, "h264"); assert.equal(video.width, 320); assert.equal(video.height, 180);
  assert.ok(Number(video.nb_read_frames) >= 3);
  assert.equal(probe.streams.find(stream => stream.codec_type === "audio")?.codec_name, "aac");
  const pixel = execFileSync("ffmpeg", ["-v", "error", "-i", f.output, "-frames:v", "1", "-vf", "crop=2:2:100:100", "-f", "rawvideo", "-pix_fmt", "rgb24", "pipe:1"]);
  assert.ok(pixel[2]! > 120 && pixel[0]! < 40 && pixel[1]! < 40, `captured pixel ${pixel.subarray(0, 3).toString("hex")}`);
  assert.equal(f.logs.length, 0);
});

test("kill switch closes actual encoder and ends the mocked record even when job hangs", async t => {
  const f = await fixture(t, true);
  const running = f.streaming.run(f.job, async () => { await delay(1000); f.env["STILLWET_KILL_SWITCH"] = "1"; await new Promise(() => {}); });
  await f.created;
  await assert.rejects(running, /canceled/u);
  assert.ok([...f.records.values()][0]?.value["endedAt"]);
  assert.ok(f.calls.some(call => call.method === "com.atproto.repo.putRecord"));
});

test("lost create response still stops capture and ends the known record key", async t => {
  const f = await fixture(t, true);
  f.failCreate();
  await assert.rejects(f.streaming.run(f.job, async () => {}), /stream_job_failed/u);
  assert.equal(f.records.size, 1);
  assert.ok([...f.records.values()][0]?.value["endedAt"]);
});

test("actual capture also stops on the wall-clock deadline", async t => {
  const f = await fixture(t, true);
  f.config.streaming.maxSeconds = 4;
  const start = performance.now();
  await assert.rejects(f.streaming.run(f.job, async () => { await new Promise(() => {}); }), /canceled/u);
  assert.ok(performance.now() - start < 17000);
  assert.ok([...f.records.values()][0]?.value["endedAt"]);
});

test("failed end write is reported, keeps the painting and doesn't hold the stream slot", async t => {
  const f = await fixture(t, true);
  f.failPut();
  assert.equal(await f.streaming.run(f.job, async () => "painted"), "painted");
  assert.ok(f.logs.some(log => JSON.stringify(log).includes("stream_end_failed")));
  assert.equal([...f.records.values()][0]?.value["endedAt"], undefined);
  f.config.dryRun = true;
  await f.streaming.run(f.job, async () => {});
});

test("viewer failure creates no livestream record and a later stream can start", async t => {
  const f = await fixture(t);
  f.config.dryRun = false;
  f.config.streaming.ingestUrl = pathToFileURL(f.output).href;
  await assert.rejects(f.streaming.run(f.job, async () => {}), /stream_job_failed/u);
  assert.equal(f.records.size, 0);
  f.config.dryRun = true;
  await f.streaming.run(f.job, async () => {});
});

test("encoder output failure prevents announcing or starting work", async t => {
  const f = await fixture(t, true);
  f.config.streaming.ingestUrl = pathToFileURL(join(f.output, "missing", "capture.flv")).href;
  let worked = false;
  await assert.rejects(f.streaming.run(f.job, async () => { worked = true; }), /stream_job_failed/u);
  assert.equal(worked, false);
  assert.equal(f.records.size, 0);
});

test("engine wraps work, uses optional Opus stream wording and never marks canceled work done", async t => {
  const f = await fixture(t);
  t.mock.method(console, "log", (value: string) => f.logs.push(JSON.parse(value) as object));
  const store = new Store(":memory:");
  t.after(() => store.close());
  const engine = new Engine(f.config, store, fetch, f.env);
  const item = post(); f.network.threads.set(item.uri, [item]);
  assert.equal(await engine.intake(item, NOW), "accepted");
  assert.match(store.intents()[0]!.record, /dry run/u);
  assert.match(store.intents()[0]!.record, /https:\/\/stream.place\//u);
  const accepted = JSON.parse(store.intents()[0]!.record) as { text: string; facets: { index: { byteStart: number; byteEnd: number }; features: { uri: string }[] }[] };
  const facet = accepted.facets[0]!;
  assert.equal(Buffer.from(accepted.text).subarray(facet.index.byteStart, facet.index.byteEnd).toString(), streamLink(f.config));
  assert.equal(facet.features[0]?.uri, streamLink(f.config));
  await engine.pump(NOW);
  assert.equal(store.jobs()[0]?.status, "done");
  assert.match(store.intents().find(intent => intent.id.endsWith("#done"))!.record, /nothing streamed/u);
  const second = post("second", "did:plc:second"); f.network.threads.set(second.uri, [second]);
  await engine.intake(second, NOW);
  await assert.rejects(engine.pump(NOW, async () => { f.env["STILLWET_KILL_SWITCH"] = ""; await new Promise(() => {}); }), /canceled/u);
  assert.equal(store.jobs()[1]?.status, "canceled");
  assert.equal(store.intents().filter(intent => intent.id.endsWith("#done")).length, 1);
});
