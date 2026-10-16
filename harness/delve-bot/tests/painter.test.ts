import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdtempSync, readdirSync, readFileSync, readlinkSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { loadConfig, validateConfig } from "../src/config.ts";
import { CONTINUE_MESSAGE, HousePainter, overLimit, PAINTER_MSG, painterJob, stopServer, UsageMeter } from "../src/painter.ts";
import type { Config } from "../src/types.ts";
import { post } from "./helpers.ts";

const repo = resolve(import.meta.dirname, "../../..");
let template = process.env["DELVE_TEST_TEMPLATE"];

/** The house studio this bot paints in, exported once from the checkout's HEAD by the house exporter. */
function studioTemplate(): string {
  if (template) return template;
  const dir = mkdtempSync(join(tmpdir(), "delve-template-"));
  template = join(dir, "studio");
  execFileSync(join(repo, "scripts/export_r16_studio"), ["every", template], { env: { ...process.env, R16_BRANCH: "HEAD" }, stdio: ["ignore", "ignore", "inherit"] });
  return template;
}

/** Processes whose working directory is in the studio (the painter's pi, its easel server). */
function inStudio(studio: string): string[] {
  return readdirSync("/proc").filter(pid => /^\d+$/u.test(pid)).filter(pid => {
    try { return readlinkSync(`/proc/${pid}/cwd`).startsWith(studio); } catch { return false; }
  }).map(pid => readFileSync(`/proc/${pid}/cmdline`, "utf8").split("\0")[0]!);
}

function setup(t: { after(fn: () => void): void }, script: string, change: (config: Config) => void = () => {}) {
  const dir = mkdtempSync(join(tmpdir(), "delve-painter-"));
  const config = loadConfig();
  config.dryRun = false;
  config.database = join(dir, "state.sqlite");
  config.studio = { ...config.studio, template: studioTemplate(), root: join(dir, "src/a"), sessions: join(dir, "sessions"), harness: join(repo, "harness/painter") };
  change(config);
  validateConfig(config);
  const asked = join(dir, "asked.jsonl"), logs: object[] = [];
  const painter = new HousePainter(config, {}, value => logs.push(value), { model: ["--provider", "faux", "--model", "m"],
    extensions: [resolve(import.meta.dirname, "fixtures/faux-painter.ts")], env: { FAUX_OUT: asked, FAUX_SCRIPT: script } });
  const item = post();
  const job = painterJob(item, { uri: item.uri, cid: item.cid }, "Paint a red stroke", config);
  t.after(() => {
    try { execFileSync(join(painter.studio(job), "bin/easel"), ["close"], { cwd: painter.studio(job), stdio: "ignore", timeout: 60000 }); } catch { /* no server */ }
    execFileSync("chmod", ["-R", "u+w", dir]);
    rmSync(dir, { recursive: true, force: true });
  });
  const messages = () => readFileSync(asked, "utf8").trim().split("\n").map(line => JSON.parse(line) as string);
  return { config, painter, job, logs, messages, studio: painter.studio(job), sessions: painter.sessions(job) };
}

test("the house painter paints a short test painting in a copy of a read-only house studio and hands back a fitted JPEG", async t => {
  const f = setup(t, "paint", c => {
    const readOnly = join(dirname(c.database), "template");
    cpSync(realpathSync(c.studio.template), readOnly, { recursive: true });
    execFileSync("chmod", ["-R", "a-w", readOnly]);
    c.studio.template = readOnly;
  });
  const run = await f.painter.run(f.job, new AbortController().signal, Date.now());
  assert.deepEqual(run, { stopped: null });
  assert.equal(readFileSync(join(f.studio, "BRIEF.md"), "utf8"), f.job.brief);
  assert.deepEqual(readFileSync(join(f.studio, "bin/easel")), readFileSync(join(f.config.studio.template, "bin/easel")));
  assert.equal(readFileSync(join(f.studio, "bin/box"), "utf8").trim(), "every");
  assert.equal(execFileSync("find", [f.studio, "!", "-perm", "-u+w"], { encoding: "utf8" }), "");
  assert.equal(f.messages().length, 4);
  assert.equal(f.messages()[0], PAINTER_MSG);
  assert.equal(readdirSync(f.sessions).filter(name => name.endsWith(".jsonl")).length, 1);
  assert.equal(readFileSync(join(f.studio, "paintings/lua/painting.lua"), "utf8").match(/^--@ chunk/gmu)?.length, 2);
  const painting = await f.painter.finish(f.job, run);
  assert.notEqual(painting, "dry");
  if (painting === "dry") return;
  assert.deepEqual([painting.width, painting.height], [1200, 900]);
  assert.deepEqual([...painting.bytes.subarray(0, 2)], [0xff, 0xd8]);
  assert.ok(painting.bytes.length <= f.config.caps.imageBytes);
  assert.equal(painting.alt, "Tiny Test A red stroke across a white ground.");
  assert.equal(painting.stopped, null);
  assert.deepEqual(inStudio(f.studio), []);
});

test("the runner stops the painter at its token and time limits, a stop or cancel kills it, and the next run carries on in its session", async t => {
  const f = setup(t, "chatty", c => { c.studio.maxOutputTokens = 3000; });
  assert.deepEqual(await f.painter.run(f.job, new AbortController().signal, Date.now()), { stopped: "output_tokens" });
  assert.deepEqual(inStudio(f.studio).filter(cmd => !cmd.endsWith("/bin/easel")), []);
  const quiet = setup(t, "quiet", c => { c.studio.maxWallSeconds = 3; });
  const start = Date.now();
  assert.deepEqual(await quiet.painter.run(quiet.job, new AbortController().signal, start), { stopped: "time" });
  assert.ok(Date.now() - start < 12000);
  const sessions = readdirSync(quiet.sessions).filter(name => name.endsWith(".jsonl"));
  const before = quiet.messages().length;
  const controller = new AbortController();
  setTimeout(() => controller.abort(), 1500);
  assert.deepEqual(await quiet.painter.run(quiet.job, controller.signal, Date.now()), { stopped: "canceled" });
  assert.equal(quiet.messages()[0], PAINTER_MSG);
  assert.equal(quiet.messages()[before], CONTINUE_MESSAGE);
  assert.deepEqual(readdirSync(quiet.sessions).filter(name => name.endsWith(".jsonl")), sessions);
  assert.deepEqual(inStudio(quiet.studio), []);
  await assert.rejects(quiet.painter.finish(quiet.job, { stopped: "canceled" }));
  assert.ok(quiet.logs.some(log => JSON.stringify(log).includes('"reason":"time"')));
});

test("a job already past its wall time never opens the easel or starts the painter", async t => {
  const f = setup(t, "paint", c => { c.studio.maxWallSeconds = 3; });
  assert.deepEqual(await f.painter.run(f.job, new AbortController().signal, Date.now() - 4000), { stopped: "time" });
  assert.equal(existsSync(join(dirname(f.config.database), "asked.jsonl")), false);
  assert.deepEqual(inStudio(f.studio), []);
});

test("a studio's easel server can be stopped without its socket", async t => {
  const f = setup(t, "paint");
  cpSync(realpathSync(f.config.studio.template), f.studio, { recursive: true });
  execFileSync("chmod", ["-R", "u+w", f.studio]);
  execFileSync(join(f.studio, "bin/easel"), ["open"], { cwd: f.studio, stdio: "ignore" });
  assert.deepEqual(inStudio(f.studio), [join(f.studio, "bin/easel")]);
  await stopServer(f.studio);
  assert.deepEqual(inStudio(f.studio), []);
});

test("a resumed job past its wall time still posts the canvas it already has", async t => {
  const f = setup(t, "paint", c => { c.studio.maxWallSeconds = 600; });
  assert.deepEqual(await f.painter.run(f.job, new AbortController().signal, Date.now()), { stopped: null });
  await f.painter.finish(f.job, { stopped: null });
  const asked = f.messages().length;
  const run = await f.painter.run(f.job, new AbortController().signal, Date.now() - 601000);
  assert.deepEqual(run, { stopped: "time" });
  assert.equal(f.messages().length, asked);
  const painting = await f.painter.finish(f.job, run);
  assert.notEqual(painting, "dry");
  if (painting === "dry") return;
  assert.deepEqual([painting.width, painting.height], [1200, 900]);
  assert.equal(painting.stopped, "time");
  assert.deepEqual(inStudio(f.studio), []);
});

test("a job canceled while the easel opens never starts the painter", async t => {
  const f = setup(t, "paint");
  const controller = new AbortController();
  controller.abort();
  assert.deepEqual(await f.painter.run(f.job, controller.signal, Date.now()), { stopped: "canceled" });
  assert.equal(existsSync(join(dirname(f.config.database), "asked.jsonl")), false);
  assert.deepEqual(inStudio(f.studio), []);
});

test("usage counts every prompt token, output tokens and dollars across a painter's session files", t => {
  const dir = mkdtempSync(join(tmpdir(), "delve-usage-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const line = (input: number, cacheRead: number, cacheWrite: number, output: number, total: number) => JSON.stringify({ type: "message",
    message: { role: "assistant", usage: { input, output, cacheRead, cacheWrite, cost: { total } } } }) + "\n";
  writeFileSync(join(dir, "a.jsonl"), line(100, 2000, 300, 50, 0.25) + JSON.stringify({ type: "message", message: { role: "user" } }) + "\n");
  const meter = new UsageMeter(dir);
  assert.deepEqual(meter.usage(), { input: 2400, output: 50, cost: 0.25 });
  writeFileSync(join(dir, "b.jsonl"), line(0, 400000, 0, 10, 9.75));
  const usage = meter.usage();
  assert.deepEqual(usage, { input: 402400, output: 60, cost: 10 });
  assert.equal(overLimit({ input: 1e9, output: 1e9, cost: 1e6 }, loadConfig().studio), null);
  const studio = { ...loadConfig().studio, maxInputTokens: 500000, maxOutputTokens: 50000, maxCostUsd: 10 };
  assert.equal(overLimit(usage, studio), "cost");
  assert.equal(overLimit({ ...usage, input: 500000 }, studio), "input_tokens");
  assert.equal(overLimit({ input: 1, output: 50000, cost: 0 }, studio), "output_tokens");
  assert.equal(overLimit({ input: 1, output: 1, cost: 0 }, studio), null);
  assert.deepEqual(new UsageMeter(join(dir, "missing")).usage(), { input: 0, output: 0, cost: 0 });
});
