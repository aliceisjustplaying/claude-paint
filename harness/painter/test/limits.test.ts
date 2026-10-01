// node --test harness/painter/test/limits.test.ts
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { LIMIT_WAIT, limitResetMs, limitWaitFromEnv, limitWaitMs, usageLimit } from "../limits.ts";

const GO_LIMIT = '429: {"type":"GoUsageLimitError","message":"Go usage limit exceeded"}';

test("a usage limit is the runner's: Go's and Zen's limit errors, not an empty balance", () => {
	assert.ok(usageLimit(GO_LIMIT));
	assert.ok(usageLimit("FreeUsageLimitError: try later"));
	assert.ok(usageLimit("Usage limit reached. Resets in 2hr 15min"));
	assert.ok(!usageLimit('402: {"message":"Your credits are depleted"}'));
	assert.ok(!usageLimit("overloaded_error"));
	assert.ok(!usageLimit(undefined));
});

test("the wait is the reset time the error names plus a minute, else the probe interval", () => {
	assert.equal(limitResetMs("Usage limit reached. Resets in 2hr 15min"), (2 * 60 + 15) * 60_000);
	assert.equal(limitResetMs("resets in 2 days"), 2 * 86_400_000);
	assert.equal(limitResetMs(GO_LIMIT), undefined);
	assert.equal(limitWaitMs("usage limit reached, resets in 10 min", 0, 0, LIMIT_WAIT), 11 * 60_000);
	assert.equal(limitWaitMs(GO_LIMIT, 0, 0, LIMIT_WAIT), LIMIT_WAIT.probeMs);
});

test("after a day of limits in a row the run settles on the error", () => {
	assert.equal(limitWaitMs(GO_LIMIT, 0, LIMIT_WAIT.giveUpMs - 1, LIMIT_WAIT), LIMIT_WAIT.probeMs);
	assert.equal(limitWaitMs(GO_LIMIT, 0, LIMIT_WAIT.giveUpMs, LIMIT_WAIT), undefined);
});

test("PAINTER_LIMIT_PROBE_S and PAINTER_LIMIT_GIVE_UP_H set the waits; nonsense is refused", () => {
	assert.deepEqual(limitWaitFromEnv({ PAINTER_LIMIT_PROBE_S: "5", PAINTER_LIMIT_GIVE_UP_H: "1" }), { probeMs: 5_000, giveUpMs: 3_600_000 });
	assert.deepEqual(limitWaitFromEnv({}), LIMIT_WAIT);
	assert.throws(() => limitWaitFromEnv({ PAINTER_LIMIT_PROBE_S: "soon" }), /want seconds/);
});

test("a run cut off by usage limits goes on when they lift, asking with the conversation as it was", () => {
	const tmp = process.env.TMPDIR;
	assert.ok(tmp, "set TMPDIR to a scratch directory");
	const dir = mkdtempSync(join(tmp, "limits-test-"));
	const studio = join(dir, "studio");
	mkdirSync(studio);
	const out = join(dir, "asked.json");
	const here = join(import.meta.dirname, "..");
	const reply = execFileSync("pi", ["--print", "--no-extensions", "-e", join(here, "painter.ts"), "-e", join(here, "test/fixtures/faux-limit.ts"),
		"--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve", "--session-dir", join(dir, "sessions"),
		"--provider", "faux", "--model", "m", "hello"],
		{ cwd: studio, env: { ...process.env, FAUX_OUT: out, PAINTER_LIMIT_PROBE_S: "0" }, stdio: ["ignore", "pipe", "pipe"], encoding: "utf8" });
	assert.equal(reply.trim(), "done");
	const { calls, asked } = JSON.parse(readFileSync(out, "utf8"));
	assert.equal(calls, 3);
	// nothing about a connection, a wait or an error: the prompt as the painter left it
	assert.deepEqual(asked.filter((m: { role: string }) => m.role !== "system"), [{ role: "user", content: [{ type: "text", text: "hello" }] }]);
	// the session file keeps both errored responses, each left out of the context by an edit
	const [log] = readdirSync(join(dir, "sessions"));
	const entries = readFileSync(join(dir, "sessions", log), "utf8").trim().split("\n").map((l) => JSON.parse(l));
	const errored = entries.filter((e) => e.type === "message" && e.message.stopReason === "error").map((e) => e.id);
	assert.equal(errored.length, 2);
	assert.deepEqual(entries.filter((e) => e.type === "context_edit").map((e) => [e.targetId, e.replacement]), errored.map((id) => [id, null]));
});
