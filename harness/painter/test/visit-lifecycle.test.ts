import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmdirSync, writeFileSync } from "node:fs";
import { registerHooks } from "node:module";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { test } from "node:test";
import { passagesToTakeUp, sittingCritique, visit } from "../visit.ts";

function fixture() {
	const dir = mkdtempSync(join(tmpdir(), "visit-lifecycle-"));
	mkdirSync(join(dir, "notes"));
	const pi = join(dir, "pi");
	const pid = join(dir, "pid");
	writeFileSync(pi, `#!${process.execPath}
const fs = require('node:fs');
process.on('SIGTERM', () => fs.writeFileSync(${JSON.stringify(join(dir, "terminated"))}, 'yes'));
fs.writeFileSync(${JSON.stringify(pid + ".tmp")}, String(process.pid));
fs.renameSync(${JSON.stringify(pid + ".tmp")}, ${JSON.stringify(pid)});
setInterval(() => {}, 1000);
`, { mode: 0o755 });
	return { dir, pi, pid, critic: { args: [], dir: join(dir, "critic") } };
}

async function until(check: () => boolean) {
	for (let i = 0; i < 250; i++) {
		if (check()) return;
		await delay(20);
	}
	assert.fail("process lifecycle did not settle");
}

function validPid(pid: number): boolean {
	return Number.isSafeInteger(pid) && pid > 1 && pid <= 2147483647;
}

function alive(pid: number): boolean {
	if (!validPid(pid)) return false;
	try {
		process.kill(pid, 0);
		if (process.platform === "linux" && /\) Z /.test(readFileSync(`/proc/${pid}/stat`, "utf8"))) return false;
		return true;
	} catch {
		return false;
	}
}

async function readyPid(path: string): Promise<number> {
	let pid = 0;
	await until(() => {
		try {
			const text = readFileSync(path, "utf8").trim();
			pid = /^\d+$/.test(text) ? Number(text) : 0;
			return validPid(pid);
		} catch { return false; }
	});
	return pid;
}

function stopGroup(pid: number) {
	if (!validPid(pid)) return;
	try { process.kill(-pid, "SIGKILL"); } catch {}
}

test("PID readiness waits through an empty file before allowing cleanup", async () => {
	const f = fixture();
	writeFileSync(f.pid, "");
	let settled = false;
	const pending = readyPid(f.pid).then(pid => { settled = true; return pid; });
	await delay(60);
	assert.equal(settled, false);
	writeFileSync(f.pid, String(process.pid));
	assert.equal(await pending, process.pid);
});

test("PID probes and group cleanup never signal invalid targets", (t) => {
	const kill = t.mock.method(process, "kill", () => true);
	for (const pid of [0, -0, -42, 1, NaN, Infinity, 2.5, 2147483648]) {
		assert.equal(alive(pid), false);
		stopGroup(pid);
	}
	assert.equal(kill.mock.callCount(), 0);
	stopGroup(42);
	assert.deepEqual(kill.mock.calls[0].arguments, [-42, "SIGKILL"]);
});

for (const previous of [false, true]) test(`a failed latest visit is explicit in both openings ${previous ? "with older notes" : "without prior notes"}`, async () => {
	const f = fixture();
	if (previous) writeFileSync(join(f.dir, "notes/critic.md"), "Old advice\n");
	writeFileSync(f.pi, "#!/bin/sh\necho unavailable >&2\nexit 7\n", { mode: 0o755 });
	await assert.rejects(visit(f.dir, f.critic, "sitting", undefined, f.pi), /unavailable/);
	const passages = passagesToTakeUp(f.dir, true);
	const critique = sittingCritique(f.dir);
	const warning = previous ? "The critic's last visit failed, so it left no new notes." : "The critic's last visit failed, so it left no notes.";
	assert.ok(passages.includes(warning));
	assert.ok(critique.includes(warning));
	assert.ok(!passagesToTakeUp(f.dir, false).includes(warning));
	if (previous) {
		assert.match(passages, /Its notes from an earlier visit \(notes\/critic\.md\) are below, quoted as the critic wrote them\. They don't cover anything you've painted since that visit\./);
		assert.match(critique, /These are from an earlier visit and don't cover anything you've painted since\. Quoted as the critic wrote them:/);
		assert.ok(passages.indexOf(warning) < passages.indexOf("> Old advice"));
		assert.ok(critique.indexOf(warning) < critique.indexOf("> Old advice"));
		assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "Old advice\n");
	} else {
		assert.equal(critique, `\n\n## The critic's notes\n\n${warning}`);
		assert.ok(!existsSync(join(f.dir, "notes/critic.md")));
	}
	writeFileSync(f.pi, "#!/bin/sh\necho 'New advice'\n", { mode: 0o755 });
	await visit(f.dir, f.critic, "sitting", undefined, f.pi);
	for (const opening of [passagesToTakeUp(f.dir, true), sittingCritique(f.dir)]) {
		assert.ok(!opening.includes("last visit failed"));
		assert.ok(opening.includes("> New advice"));
	}
});

test("an archive collision preserves old advice and reports failure until a complete visit publishes", async () => {
	const f = fixture();
	writeFileSync(join(f.dir, "notes/critic.md"), "Old advice\n");
	writeFileSync(f.pi, `#!${process.execPath}
const fs = require('node:fs'), path = require('node:path');
const sessions = process.argv[process.argv.indexOf('--session-dir') + 1];
fs.mkdirSync(path.join(path.dirname(path.dirname(sessions)), path.basename(sessions) + '-asked.md'));
console.log('New advice');
`, { mode: 0o755 });
	await assert.rejects(visit(f.dir, f.critic, "asked", undefined, f.pi), { code: "EISDIR" });
	assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "Old advice\n");
	const log = readFileSync(join(f.critic.dir, "visits.log"), "utf8");
	assert.match(log, /asked: failed:.*EISDIR/);
	assert.doesNotMatch(log, /asked: exit 0/);
	assert.match(sittingCritique(f.dir), /last visit failed[\s\S]*> Old advice/);
	for (const entry of readdirSync(f.critic.dir).filter((name) => name.endsWith("-asked.md"))) rmdirSync(join(f.critic.dir, entry));
	writeFileSync(f.pi, "#!/bin/sh\necho 'New advice'\n", { mode: 0o755 });
	await visit(f.dir, f.critic, "asked", undefined, f.pi);
	assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /asked: exit 0/);
	assert.doesNotMatch(sittingCritique(f.dir), /last visit failed/);
	const archive = readdirSync(f.critic.dir).find((name) => name.endsWith("-asked.md"))!;
	assert.equal(readFileSync(join(f.critic.dir, archive), "utf8"), readFileSync(join(f.dir, "notes/critic.md"), "utf8"));
});

test("a freshness-marker directory collision preserves notes and archives until retry", async () => {
	const f = fixture();
	writeFileSync(join(f.dir, "notes/critic.md"), "Old advice\n");
	const marker = join(f.dir, "out/easel/critic-visit.failed");
	mkdirSync(marker, { recursive: true });
	writeFileSync(f.pi, "#!/bin/sh\necho 'New advice'\n", { mode: 0o755 });
	await assert.rejects(visit(f.dir, f.critic, "asked", undefined, f.pi));
	assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "Old advice\n");
	assert.deepEqual(readdirSync(f.critic.dir).filter((name) => name.endsWith("-asked.md")), []);
	assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /asked: failed:/);
	assert.match(sittingCritique(f.dir), /last visit failed[\s\S]*> Old advice/);
	rmdirSync(marker);
	await visit(f.dir, f.critic, "asked", undefined, f.pi);
	assert.doesNotMatch(sittingCritique(f.dir), /last visit failed/);
	assert.match(sittingCritique(f.dir), /> New advice/);
	const archive = readdirSync(f.critic.dir).find((name) => name.endsWith("-asked.md"))!;
	assert.equal(readFileSync(join(f.critic.dir, archive), "utf8"), readFileSync(join(f.dir, "notes/critic.md"), "utf8"));
});

test("a visit runs its supervisor with Node when the extension host is pi", async () => {
	const f = fixture();
	const host = join(f.dir, "extension-host");
	writeFileSync(host, "#!/bin/sh\necho 'pi is not a JavaScript runtime' >&2\nexit 64\n", { mode: 0o755 });
	writeFileSync(f.pi, "#!/bin/sh\necho critique\n", { mode: 0o755 });
	const execPath = process.execPath;
	try {
		process.execPath = host;
		const notes = await visit(f.dir, f.critic, "sitting", undefined, f.pi);
		assert.match(notes, /critique/);
		assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), notes);
	} finally {
		process.execPath = execPath;
	}
});

for (const stopReason of ["error", "aborted"]) {
	for (const stdout of ["", "Printed critique"]) test(`a terminal critic ${stopReason} rejects a zero exit with ${stdout ? "printed text" : "empty stdout"}`, async () => {
		const f = fixture();
		writeFileSync(join(f.dir, "notes/critic.md"), "previous notes");
		const entries = [
			{ message: { role: "assistant", stopReason: "stop", content: [{ type: "text", text: "Lift the sky." }] } },
			{ message: { role: "assistant", stopReason, errorMessage: "provider unavailable", content: [] } },
		];
		writeFileSync(f.pi, `#!${process.execPath}
const fs = require('node:fs');
const path = require('node:path');
const sessions = process.argv[process.argv.indexOf('--session-dir') + 1];
fs.mkdirSync(sessions, { recursive: true });
fs.writeFileSync(path.join(sessions, 'a.jsonl'), ${JSON.stringify(entries.map((entry) => JSON.stringify(entry)).join("\n") + "\n")});
process.stdout.write(${JSON.stringify(stdout)});
`, { mode: 0o755 });
		await assert.rejects(visit(f.dir, f.critic, "sitting", undefined, f.pi), /the critic couldn't visit: provider unavailable/);
		assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "previous notes");
		const log = readFileSync(join(f.critic.dir, "visits.log"), "utf8");
		assert.match(log, /sitting: failed: the critic couldn't visit: provider unavailable/);
		assert.doesNotMatch(log, /sitting: exit 0/);
	});
}

test("a healthy critic completion after transient errors accepts the session reply with empty stdout", async () => {
	const f = fixture();
	const entries = [
		{ message: { role: "assistant", stopReason: "error", errorMessage: "temporary provider error", content: [] } },
		{ message: { role: "assistant", stopReason: "aborted", content: [] } },
		{ message: { role: "assistant", stopReason: "stop", content: [{ type: "text", text: "Lift the sky." }] } },
	];
	writeFileSync(f.pi, `#!${process.execPath}
const fs = require('node:fs');
const path = require('node:path');
const sessions = process.argv[process.argv.indexOf('--session-dir') + 1];
fs.mkdirSync(sessions, { recursive: true });
fs.writeFileSync(path.join(sessions, 'a.jsonl'), ${JSON.stringify(entries.map((entry) => JSON.stringify(entry)).join("\n") + "\n")});
`, { mode: 0o755 });
	const notes = await visit(f.dir, f.critic, "sitting", undefined, f.pi);
	assert.equal(notes, "From the critic's visit at the end of a sitting:\n\nLift the sky.\n");
	assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), notes);
	assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /sitting: exit 0, 0 characters/);
});

test("aborting a visit escalates past an ignored SIGTERM and preserves the previous notes", async () => {
	const f = fixture();
	const descendantFile = join(f.dir, "descendant");
	writeFileSync(f.pi, readFileSync(f.pi, "utf8") + `
require('node:child_process').spawn(process.execPath, ['-e', ${JSON.stringify(`
process.on('SIGTERM', () => {});
require('node:fs').writeFileSync(${JSON.stringify(descendantFile + ".tmp")}, String(process.pid));
require('node:fs').renameSync(${JSON.stringify(descendantFile + ".tmp")}, ${JSON.stringify(descendantFile)});
setInterval(() => {}, 1000);
`)}], { stdio: 'inherit' });
`);
	writeFileSync(join(f.dir, "notes/critic.md"), "previous notes");
	const controller = new AbortController();
	const result = visit(f.dir, f.critic, "asked", controller.signal, f.pi);
	const rejected = assert.rejects(result, /Operation aborted/);
	const pid = await readyPid(f.pid);
	const descendant = await readyPid(descendantFile);
	try {
		controller.abort();
		await until(() => !alive(pid));
		await until(() => !alive(descendant));
		await rejected;
		assert.ok(existsSync(join(f.dir, "terminated")));
		assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "previous notes");
		assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /asked: failed: Operation aborted/);
	} finally {
		stopGroup(pid);
	}
});

test("a visit is capped at twenty minutes even without a caller signal", async (t) => {
	const f = fixture();
	t.mock.timers.enable({ apis: ["setTimeout"] });
	const result = visit(f.dir, f.critic, "sitting", undefined, f.pi);
	const rejected = assert.rejects(result, /critic timed out after 20 minutes and was stopped/);
	const pid = await readyPid(f.pid);
	try {
		t.mock.timers.tick(20 * 60_000 - 1);
		assert.ok(alive(pid));
		t.mock.timers.tick(1);
		await until(() => !alive(pid));
		await rejected;
		assert.ok(!existsSync(join(f.dir, "notes/critic.md")));
		assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /sitting: failed: critic timed out/);
	} finally {
		stopGroup(pid);
	}
});

for (const signal of ["SIGTERM", "SIGKILL"] as const) test(`a critic is cleaned up after its painter receives ${signal}`, async () => {
	const f = fixture();
	const parent = spawn(process.execPath, ["--input-type=module", "-e", `
import { visit } from ${JSON.stringify(new URL("../visit.ts", import.meta.url).href)};
await visit(${JSON.stringify(f.dir)}, ${JSON.stringify(f.critic)}, "sitting", undefined, ${JSON.stringify(f.pi)});
`], { stdio: "ignore" });
	const pid = await readyPid(f.pid);
	try {
		parent.kill(signal);
		await until(() => !alive(pid));
		assert.ok(existsSync(join(f.dir, "terminated")));
	} finally {
		parent.kill("SIGKILL");
		stopGroup(pid);
	}
});

test("boundary visits use sitting cancellation and aren't suppressed by a remembered limit", async () => {
	const hooks = registerHooks({
		resolve(specifier, context, nextResolve) {
			if (specifier === "@earendil-works/pi-ai") return { url: "data:text/javascript,export const Type = Object.fromEntries(['Object', 'Optional', 'String', 'Boolean', 'Integer', 'Number', 'Union'].map(k => [k, x => x]));", shortCircuit: true };
			if (specifier === "@earendil-works/pi-coding-agent") return { url: "data:text/javascript,export class AgentSession {} export const defineTool = x => x; export const createReadToolDefinition = () => ({});", shortCircuit: true };
			return nextResolve(specifier, context);
		},
	});
	const { default: painter } = await import("../painter.ts");
	hooks.deregister();
	const f = fixture();
	const hanging = readFileSync(f.pi, "utf8");
	const cwd = process.cwd();
	const path = process.env.PATH;
	const env = { ...process.env };
	const handlers: Record<string, ((...args: any[]) => any)[]> = {};
	try {
		process.chdir(f.dir);
		process.env.PATH = `${f.dir}:${path}`;
		process.env.PAINTER_CRITIC = JSON.stringify(f.critic);
		process.env.PAINTER_LIMIT_PROBE_S = "0";
		painter({ on: (name, handler) => { (handlers[name] ??= []).push(handler); }, registerTool() {} } as never);
		const emit = async (name: string, ...args: any[]) => {
			let result;
			for (const handler of handlers[name] ?? []) result = await handler(...args) ?? result;
			return result;
		};
		const controller = new AbortController();
		const ctx = { signal: controller.signal };
		const event = { context: { contextEntries: [{ sourceEntry: { type: "message", id: "limit", message: { role: "assistant", stopReason: "error" } } }] } };
		await emit("message_end", { message: { role: "assistant", stopReason: "error", errorMessage: "Go usage limit exceeded" } });
		assert.equal((await emit("agent_before_settle", event, ctx)).continue, true);
		writeFileSync(f.pi, "#!/bin/sh\necho critique\n", { mode: 0o755 });
		await emit("message_end", { message: { role: "assistant", stopReason: "error", errorMessage: "network error" } });
		await emit("agent_before_settle", event, ctx);
		assert.match(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), /critique/);
		controller.abort();
		const before = readFileSync(join(f.critic.dir, "visits.log"), "utf8");
		await emit("agent_before_settle", event, ctx);
		assert.equal(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), before);
		await emit("before_agent_start", { systemPromptOptions: {} });
		writeFileSync(f.pi, hanging, { mode: 0o755 });
		const ending = emit("agent_before_settle", event, {});
		const pid = await readyPid(f.pid);
		try {
			await emit("session_shutdown");
			await until(() => !alive(pid));
			await ending;
			assert.match(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), /critique/);
		} finally {
			stopGroup(pid);
		}
	} finally {
		process.chdir(cwd);
		process.env = env;
	}
});
