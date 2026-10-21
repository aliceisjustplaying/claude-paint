import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { registerHooks } from "node:module";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { setTimeout as delay } from "node:timers/promises";
import { test } from "node:test";
import { visit } from "../visit.ts";

function fixture() {
	const dir = mkdtempSync(join(tmpdir(), "visit-lifecycle-"));
	mkdirSync(join(dir, "notes"));
	const pi = join(dir, "pi");
	const pid = join(dir, "pid");
	writeFileSync(pi, `#!${process.execPath}
const fs = require('node:fs');
process.on('SIGTERM', () => fs.writeFileSync(${JSON.stringify(join(dir, "terminated"))}, 'yes'));
fs.writeFileSync(${JSON.stringify(pid)}, String(process.pid));
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

function alive(pid: number): boolean {
	try {
		process.kill(pid, 0);
		if (process.platform === "linux" && /\) Z /.test(readFileSync(`/proc/${pid}/stat`, "utf8"))) return false;
		return true;
	} catch {
		return false;
	}
}

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

test("aborting a visit escalates past an ignored SIGTERM and preserves the previous notes", async () => {
	const f = fixture();
	const descendantFile = join(f.dir, "descendant");
	writeFileSync(f.pi, readFileSync(f.pi, "utf8") + `
require('node:child_process').spawn(process.execPath, ['-e', ${JSON.stringify(`
process.on('SIGTERM', () => {});
require('node:fs').writeFileSync(${JSON.stringify(descendantFile)}, String(process.pid));
setInterval(() => {}, 1000);
`)}], { stdio: 'inherit' });
`);
	writeFileSync(join(f.dir, "notes/critic.md"), "previous notes");
	const controller = new AbortController();
	const result = visit(f.dir, f.critic, "asked", controller.signal, f.pi);
	const rejected = assert.rejects(result, /Operation aborted/);
	await until(() => existsSync(f.pid) && existsSync(descendantFile));
	const pid = Number(readFileSync(f.pid, "utf8"));
	const descendant = Number(readFileSync(descendantFile, "utf8"));
	try {
		controller.abort();
		await until(() => !alive(pid));
		await until(() => !alive(descendant));
		await rejected;
		assert.ok(existsSync(join(f.dir, "terminated")));
		assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "previous notes");
		assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /asked: failed: Operation aborted/);
	} finally {
		try { process.kill(-pid, "SIGKILL"); } catch {}
	}
});

test("a visit is capped at twenty minutes even without a caller signal", async (t) => {
	const f = fixture();
	t.mock.timers.enable({ apis: ["setTimeout"] });
	const result = visit(f.dir, f.critic, "sitting", undefined, f.pi);
	const rejected = assert.rejects(result, /critic timed out after 20 minutes and was stopped/);
	await until(() => existsSync(f.pid));
	const pid = Number(readFileSync(f.pid, "utf8"));
	try {
		t.mock.timers.tick(20 * 60_000 - 1);
		assert.ok(alive(pid));
		t.mock.timers.tick(1);
		await until(() => !alive(pid));
		await rejected;
		assert.ok(!existsSync(join(f.dir, "notes/critic.md")));
		assert.match(readFileSync(join(f.critic.dir, "visits.log"), "utf8"), /sitting: failed: critic timed out/);
	} finally {
		try { process.kill(-pid, "SIGKILL"); } catch {}
	}
});

for (const signal of ["SIGTERM", "SIGKILL"] as const) test(`a critic is cleaned up after its painter receives ${signal}`, async () => {
	const f = fixture();
	const parent = spawn(process.execPath, ["--input-type=module", "-e", `
import { visit } from ${JSON.stringify(new URL("../visit.ts", import.meta.url).href)};
await visit(${JSON.stringify(f.dir)}, ${JSON.stringify(f.critic)}, "sitting", undefined, ${JSON.stringify(f.pi)});
`], { stdio: "ignore" });
	await until(() => existsSync(f.pid));
	const pid = Number(readFileSync(f.pid, "utf8"));
	try {
		parent.kill(signal);
		await until(() => !alive(pid));
		assert.ok(existsSync(join(f.dir, "terminated")));
	} finally {
		parent.kill("SIGKILL");
		try { process.kill(-pid, "SIGKILL"); } catch {}
	}
});

test("boundary visits use sitting cancellation and aren't suppressed by a remembered limit", async () => {
	const hooks = registerHooks({
		resolve(specifier, context, nextResolve) {
			if (specifier === "@earendil-works/pi-ai") return { url: "data:text/javascript,export const Type = Object.fromEntries(['Object', 'Optional', 'String', 'Boolean', 'Integer', 'Number', 'Union'].map(k => [k, x => x]));", shortCircuit: true };
			if (specifier === "@earendil-works/pi-coding-agent") return { url: "data:text/javascript,export const defineTool = x => x; export const createReadToolDefinition = () => ({});", shortCircuit: true };
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
		await until(() => existsSync(f.pid));
		const pid = Number(readFileSync(f.pid, "utf8"));
		try {
			await emit("session_shutdown");
			await until(() => !alive(pid));
			await ending;
			assert.match(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), /critique/);
		} finally {
			try { process.kill(-pid, "SIGKILL"); } catch {}
		}
	} finally {
		process.chdir(cwd);
		process.env = env;
	}
});
