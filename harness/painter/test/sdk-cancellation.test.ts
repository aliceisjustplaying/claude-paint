import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test, type TestContext } from "node:test";
import { setTimeout as delay } from "node:timers/promises";
import { createAgentSession, DefaultResourceLoader, ModelRuntime, SessionManager, SettingsManager } from "@earendil-works/pi-coding-agent";
import { fauxAssistantMessage, fauxProvider } from "@earendil-works/pi-ai";

async function until(check: () => boolean) {
	for (let i = 0; i < 250; i++) {
		if (check()) return;
		await delay(20);
	}
	assert.fail("SDK cancellation did not settle");
}

function alive(pid: number) {
	try {
		process.kill(pid, 0);
		return !/\) Z /.test(readFileSync(`/proc/${pid}/stat`, "utf8"));
	} catch { return false; }
}

async function fixture(t: TestContext, env: Record<string, string> = {}) {
	const dir = mkdtempSync(join(tmpdir(), "painter-sdk-"));
	for (const d of ["bin", "notes", "agent"]) mkdirSync(join(dir, d));
	const cwd = process.cwd();
	const previous = { ...process.env };
	process.chdir(dir);
	for (const key of Object.keys(process.env).filter((key) => key.startsWith("PAINTER_"))) delete process.env[key];
	Object.assign(process.env, env);
	const settingsManager = SettingsManager.inMemory({ retry: { enabled: false }, compaction: { enabled: false } });
	const modelRuntime = await ModelRuntime.create({ modelsPath: null, refreshOnCreate: false });
	const faux = fauxProvider({ provider: "painter-cancel-test", models: [{ id: "m", contextWindow: 200_000 }] });
	modelRuntime.registerNativeProvider(faux.provider);
	const loader = new DefaultResourceLoader({ cwd: dir, agentDir: join(dir, "agent"), settingsManager,
		noExtensions: true, noSkills: true, noContextFiles: true, noPromptTemplates: true,
		additionalExtensionPaths: [join(import.meta.dirname, "../painter.ts")],
	});
	await loader.reload();
	assert.deepEqual(loader.getExtensions().errors, []);
	const { session } = await createAgentSession({ cwd: dir, agentDir: join(dir, "agent"), modelRuntime,
		model: faux.getModel(), settingsManager, resourceLoader: loader, sessionManager: SessionManager.inMemory(dir), tools: [],
	});
	const errors: string[] = [];
	session.extensionRunner.onError((error) => errors.push(error.error));
	let requests = 0;
	const respond = (limit = false, input?: number) => {
		faux.setResponses([() => {
			requests++;
			const message = fauxAssistantMessage("done", limit ? { stopReason: "error", errorMessage: "Usage limit reached" } : {});
			if (input !== undefined) message.usage.input = input;
			return message;
		}]);
	};
	respond();
	t.after(async () => {
		writeFileSync(join(dir, "release"), "yes");
		await session.extensionRunner.emit({ type: "session_shutdown", reason: "quit" });
		await session.abort();
		session.dispose();
		process.chdir(cwd);
		process.env = previous;
	});
	return { dir, session, errors, respond, requests: () => requests };
}

for (const stop of ["abort", "shutdown"]) test(`SDK ${stop} cleans a held recovery globals child and prevents a later provider request`, async (t) => {
	const f = await fixture(t, { PAINTER_SITTING_RECOVERY: "1" });
	writeFileSync(join(f.dir, "bin/easel"), `#!${process.execPath}
const fs = require('node:fs');
if (process.argv[2] === 'globals') {
 fs.writeFileSync('globals.pid', String(process.pid));
 const timer = setInterval(() => { if (fs.existsSync('release')) { clearInterval(timer); console.log('1\\tsky\\ttable'); } }, 10);
} else if (process.argv[2] === 'status') console.log('ready');
else process.exit(1);
`, { mode: 0o755 });
	const opening = f.session.prompt("Back at the easel.");
	await until(() => existsSync(join(f.dir, "globals.pid")));
	const pid = Number(readFileSync(join(f.dir, "globals.pid"), "utf8"));
	if (stop === "abort") await f.session.abort();
	else await f.session.extensionRunner.emit({ type: "session_shutdown", reason: "quit" });
	const childAliveAfterStop = alive(pid);
	writeFileSync(join(f.dir, "release"), "yes");
	await opening;
	assert.deepEqual({ childAliveAfterStop, requests: f.requests() }, { childAliveAfterStop: false, requests: 0 });
	assert.deepEqual(f.errors, []);
	if (stop === "abort") {
		await f.session.prompt("Try the opening again.");
		assert.equal(f.requests(), 1, "an intentional later opening can retry interrupted recovery");
	}
});

test("SDK abort is isolated to its session manager while another recovery is held", async (t) => {
	const easel = `#!${process.execPath}
const fs = require('node:fs');
if (process.argv[2] === 'globals') {
 fs.writeFileSync('globals.pid', String(process.pid));
 const timer = setInterval(() => { if (fs.existsSync('release')) { clearInterval(timer); console.log('1\\tsky\\ttable'); } }, 10);
} else if (process.argv[2] === 'status') console.log('ready');
else process.exit(1);
`;
	const first = await fixture(t, { PAINTER_SITTING_RECOVERY: "1" });
	writeFileSync(join(first.dir, "bin/easel"), easel, { mode: 0o755 });
	let firstSettled = false;
	const firstOpening = first.session.prompt("Back at the easel.").then(() => { firstSettled = true; });
	await until(() => existsSync(join(first.dir, "globals.pid")));
	const firstPid = Number(readFileSync(join(first.dir, "globals.pid"), "utf8"));
	await t.test("aborting the second painter leaves the first painter running", async (t) => {
		const second = await fixture(t, { PAINTER_SITTING_RECOVERY: "1" });
		writeFileSync(join(second.dir, "bin/easel"), easel, { mode: 0o755 });
		assert.notEqual(first.session.sessionManager, second.session.sessionManager);
		const secondOpening = second.session.prompt("Back at the easel.");
		await until(() => existsSync(join(second.dir, "globals.pid")));
		const secondPid = Number(readFileSync(join(second.dir, "globals.pid"), "utf8"));
		await second.session.abort();
		await secondOpening;
		assert.equal(alive(secondPid), false);
		assert.equal(second.requests(), 0);
		assert.equal(alive(firstPid), true);
		assert.equal(firstSettled, false);
		assert.equal(first.requests(), 0);
		assert.deepEqual(second.errors, []);
	});
	assert.equal(alive(firstPid), true, "the second session's shutdown also leaves the first recovery alone");
	writeFileSync(join(first.dir, "release"), "yes");
	await firstOpening;
	assert.equal(first.requests(), 1, "the first painter completes recovery and reaches its faux provider");
	assert.deepEqual(first.errors, []);
});

test("SDK shutdown alone releases token pacing before its context signal is aborted", async (t) => {
	const f = await fixture(t, { PAINTER_INPUT_TPM: "100" });
	f.respond(false, 100);
	await f.session.prompt("First.");
	f.respond();
	const schedule = globalThis.setTimeout;
	const clear = globalThis.clearTimeout;
	let timer: ReturnType<typeof setTimeout> | undefined;
	let cleared = false;
	t.mock.method(globalThis, "setTimeout", (callback, ms, ...args) => {
		const handle = schedule(callback, ms, ...args);
		if (ms > 50_000 && ms <= 60_000) timer = handle;
		return handle;
	});
	t.mock.method(globalThis, "clearTimeout", (handle) => {
		if (handle === timer) cleared = true;
		return clear(handle);
	});
	const prompt = f.session.prompt("Second.");
	await until(() => timer !== undefined);
	const signal = f.session.extensionRunner.createContext().signal;
	assert.ok(signal);
	assert.equal(signal.aborted, false);
	await f.session.extensionRunner.emit({ type: "session_shutdown", reason: "quit" });
	assert.equal(cleared, true);
	assert.equal(signal.aborted, false, "shutdown alone canceled the harness timer");
	await f.session.abort();
	await prompt;
});

test("SDK abort cancels a usage-limit boundary without a context signal", async (t) => {
	const f = await fixture(t, { PAINTER_LIMIT_PROBE_S: "60" });
	f.respond(true);
	let boundary = false;
	const emit = f.session.extensionRunner.emitBoundary.bind(f.session.extensionRunner);
	t.mock.method(f.session.extensionRunner, "emitBoundary", (...args) => {
		if (args[0].type === "agent_before_settle") {
			assert.equal(f.session.extensionRunner.createContext().signal, undefined);
			boundary = true;
		}
		return emit(...args);
	});
	const prompt = f.session.prompt("Paint.");
	await until(() => boundary);
	let aborted = false;
	const abort = f.session.abort().then(() => { aborted = true; });
	await until(() => aborted);
	await Promise.all([prompt, abort]);
	assert.equal(f.requests(), 1);
	assert.deepEqual(f.errors, []);
});

test("SDK abort cancels a held boundary critic and saves no notes or critique archive", async (t) => {
	const base = mkdtempSync(join(tmpdir(), "painter-critic-sdk-"));
	const critic = { args: [], dir: join(base, "archive") };
	const f = await fixture(t, { PAINTER_CRITIC: JSON.stringify(critic) });
	const pi = join(f.dir, "bin/pi");
	writeFileSync(pi, `#!${process.execPath}
const fs = require('node:fs');
fs.writeFileSync('critic.pid', String(process.pid));
const timer = setInterval(() => { if (fs.existsSync('release')) { clearInterval(timer); console.log('new critique'); } }, 10);
`, { mode: 0o755 });
	process.env.PATH = `${join(f.dir, "bin")}:${process.env.PATH}`;
	writeFileSync(join(f.dir, "notes/critic.md"), "previous notes");
	const prompt = f.session.prompt("Paint.");
	await until(() => existsSync(join(f.dir, "critic.pid")));
	const pid = Number(readFileSync(join(f.dir, "critic.pid"), "utf8"));
	let aborted = false;
	const abort = f.session.abort().then(() => { aborted = true; });
	await until(() => aborted);
	await Promise.all([prompt, abort]);
	assert.equal(alive(pid), false);
	assert.equal(readFileSync(join(f.dir, "notes/critic.md"), "utf8"), "previous notes");
	assert.deepEqual(readdirSync(critic.dir).filter((name) => name.endsWith(".md")), []);
	assert.equal(f.requests(), 1);
	assert.deepEqual(f.errors, []);
});

for (const mode of ["recovery", "limit"]) test(`bundled pi RPC abort cancels ${mode} before returning success`, async (t) => {
	const dir = mkdtempSync(join(tmpdir(), "painter-rpc-"));
	for (const d of ["bin", "notes", "agent"]) mkdirSync(join(dir, d));
	writeFileSync(join(dir, "agent/settings.json"), JSON.stringify({ retry: { enabled: false }, compaction: { enabled: false } }));
	writeFileSync(join(dir, "bin/easel"), `#!${process.execPath}
const fs = require('node:fs');
if (process.argv[2] === 'globals') {
 fs.writeFileSync('globals.pid', String(process.pid));
 const timer = setInterval(() => { if (fs.existsSync('release')) { clearInterval(timer); console.log('1\\tsky\\ttable'); } }, 10);
} else if (process.argv[2] === 'status') console.log('ready');
else process.exit(1);
`, { mode: 0o755 });
	const provider = join(dir, "faux.ts");
	writeFileSync(provider, `
import { fauxProvider, fauxAssistantMessage } from '@earendil-works/pi-ai';
import { appendFileSync, writeFileSync } from 'node:fs';
export default function(pi) {
 const faux = fauxProvider({ provider: 'cancel-faux', models: [{ id: 'm', contextWindow: 200000 }] });
 faux.setResponses([() => { appendFileSync(${JSON.stringify(join(dir, "requests"))}, 'request\\n'); return fauxAssistantMessage('done', ${mode === "limit" ? "{ stopReason: 'error', errorMessage: 'Usage limit reached' }" : "{}"}); }]);
 pi.registerProvider(faux.provider);
 pi.on('agent_end', () => writeFileSync(${JSON.stringify(join(dir, "ended"))}, 'yes'));
}
`);
	const env = { ...process.env };
	for (const key of Object.keys(env).filter((key) => key.startsWith("PAINTER_"))) delete env[key];
	const child = spawn("pi", ["--mode", "rpc", "--no-extensions", "-e", join(import.meta.dirname, "../painter.ts"), "-e", provider,
		"--no-context-files", "--no-skills", "--no-prompt-templates", "--no-mcp", "--no-approve", "--provider", "cancel-faux", "--model", "m"], {
		cwd: dir, env: { ...env, PI_CODING_AGENT_DIR: join(dir, "agent"), PAINTER_SITTING_RECOVERY: mode === "recovery" ? "1" : "0", PAINTER_LIMIT_PROBE_S: "60" },
		stdio: ["pipe", "pipe", "pipe"],
	});
	let output = "";
	let stderr = "";
	child.stdout.on("data", (chunk) => { output += chunk; });
	child.stderr.on("data", (chunk) => { stderr += chunk; });
	t.after(async () => {
		writeFileSync(join(dir, "release"), "yes");
		child.kill("SIGTERM");
		if (child.exitCode === null && child.signalCode === null) await new Promise((resolve) => child.once("exit", resolve));
	});
	child.stdin.write(JSON.stringify({ id: "prompt", type: "prompt", message: "Paint." }) + "\n");
	await until(() => existsSync(join(dir, mode === "recovery" ? "globals.pid" : "ended")));
	child.stdin.write(JSON.stringify({ id: "abort", type: "abort" }) + "\n");
	await until(() => output.includes('"id":"abort"'));
	const response = output.split("\n").filter(Boolean).map((line) => JSON.parse(line)).find((entry) => entry.id === "abort");
	assert.equal(response.success, true, stderr);
	if (mode === "recovery") {
		assert.equal(alive(Number(readFileSync(join(dir, "globals.pid"), "utf8"))), false);
		assert.equal(existsSync(join(dir, "requests")), false);
	} else assert.equal(readFileSync(join(dir, "requests"), "utf8"), "request\n");
});
