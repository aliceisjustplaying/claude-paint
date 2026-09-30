import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { getEventListeners } from "node:events";
import { join } from "node:path";
import { test } from "node:test";
import { atEasel, easel, lookArgs, WAIT_MS, studioPath, tail, toolWords } from "../easel-client.ts";

test("read opens only studio files, however the path is spelled", () => {
	const top = realpathSync(mkdtempSync(join(tmpdir(), "studio-")));
	const real = join(top, "studio");
	mkdirSync(join(real, "notes"), { recursive: true });
	writeFileSync(join(real, "notes", "a.md"), "a");
	mkdirSync(join(top, "outside"));
	const secret = join(top, "outside", "secret.txt");
	writeFileSync(secret, "secret");
	symlinkSync(secret, join(real, "notes", "out"));
	const studio = join(top, "link"); // pi's cwd may be a link to the studio
	symlinkSync(real, studio);
	const home = process.env.HOME;
	process.env.HOME = join(top, "outside"); // pi's read expands ~ with os.homedir()
	try {
		const cases: [string, string | undefined][] = [
			["notes/a.md", join(real, "notes", "a.md")],
			["BRIEF.md", join(real, "BRIEF.md")], // not there yet: still the studio
			[join(studio, "notes", "new.md"), join(real, "notes", "new.md")],
			["@notes/a.md", join(real, "notes", "a.md")],
			[`file://${real}/notes/a.md`, join(real, "notes", "a.md")],
			["../outside/secret.txt", undefined],
			["notes/../../outside/secret.txt", undefined],
			[secret, undefined],
			["notes/out", undefined],
			["~/secret.txt", undefined],
			[`@${secret}`, undefined],
			[`file://${secret}`, undefined],
		];
		for (const [path, want] of cases) assert.equal(studioPath(studio, path), want, path);
	} finally {
		process.env.HOME = home;
	}
});

test("look's options become the easel's arguments", () => {
	assert.deepEqual(lookArgs({}), []);
	assert.deepEqual(lookArgs({ crop: "300,200,500,350", mode: "value,squint", size: 600, grid: 10 }),
		["--crop", "300,200,500,350", "--mode", "value,squint", "--size", "600", "--grid", "10"]);
	assert.deepEqual(lookArgs({ grid: true }), ["--grid"]);
	assert.deepEqual(lookArgs({ grid: false }), []);
});

test("a long log keeps its end and says what was left out", () => {
	assert.equal(tail("abc", 5), "abc");
	const t = tail("0123456789", 4);
	assert.match(t, /^\(the first 6 characters are left out; read paintings\/lua\/painting\.lua for all of it\)\n6789$/);
});

// With EASEL_BIN (a painter build of the easel): the tools' path through a real session.
const bin = process.env.EASEL_BIN;
test("paint, look, note and status at a real easel", { skip: !bin || !existsSync(bin) }, async () => {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(studio, "bin"));
	mkdirSync(join(studio, "notes"));
	copyFileSync(bin!, join(studio, "bin", "easel"));
	try {
		// no session yet: the first call opens one
		const ok = await atEasel(studio, ["do", "-"], 'canvas{size=400, aspect=1.25, linen=15, ground={{pile={{"lead white", 1}}, um=100, apply="knife"}}}\nprint("hi")');
		assert.match(ok, /^hi\nok · chunk 1/);
		await assert.rejects(atEasel(studio, ["do", "-"], 'error("boom")'), /boom[\s\S]*changed nothing/);
		const look = await atEasel(studio, ["look", ...lookArgs({ crop: "0,0,200,200" })], undefined);
		assert.match(look, /look-0001\.png \(480x480/);
		assert.match(await atEasel(studio, ["note", "-"], "first note"), /noted/);
		assert.match(readFileSync(join(studio, "notes", "journal.md"), "utf8"), /first note/);
		assert.match(await atEasel(studio, ["status"], undefined), /^1 chunks/);
	} finally {
		await easel(studio, ["close"]);
	}
});

test("a client that hangs is stopped and reported, not waited on forever", async () => {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(studio, "bin"));
	writeFileSync(join(studio, "bin", "easel"), "#!/bin/sh\nsleep 30\n", { mode: 0o755 });
	const t0 = Date.now();
	const r = await easel(studio, ["do", "-"], "print(1)", undefined, 300);
	assert.equal(r.timedOut, true);
	assert.ok(Date.now() - t0 < 5000);
});

/** A studio whose bin/easel logs each command to `calls` and then runs `body`. */
function stubStudio(body: string): string {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(studio, "bin"));
	writeFileSync(join(studio, "bin", "easel"), `#!/bin/sh\necho "$1" >> calls\n${body}\n`, { mode: 0o755 });
	return studio;
}
const calls = (studio: string) => readFileSync(join(studio, "calls"), "utf8").trim().split("\n");

test("a tool's opening status keeps to the tool's time limit, and a busy easel isn't opened again", async () => {
	const studio = stubStudio('[ "$1" = status ] && sleep 30\necho ok');
	const other = WAIT_MS.other;
	WAIT_MS.other = 300;
	try {
		const t0 = Date.now();
		await assert.rejects(atEasel(studio, ["status"], undefined), /isn't answering/);
		assert.ok(Date.now() - t0 < 5000);
		assert.deepEqual(calls(studio), ["status"]);
	} finally {
		WAIT_MS.other = other;
	}
});

test("an abort stops the tool's chain: nothing after it runs, and no listener is left behind", async () => {
	const studio = stubStudio('[ "$1" = status ] && [ -f slow ] && { sleep 2; exit 1; }\necho ok');
	const kept = new AbortController();
	for (let i = 0; i < 3; i++) await atEasel(studio, ["status"], undefined, kept.signal);
	assert.equal(getEventListeners(kept.signal, "abort").length, 0);

	writeFileSync(join(studio, "slow"), "");
	writeFileSync(join(studio, "calls"), "");
	const ac = new AbortController();
	setTimeout(() => ac.abort(), 300);
	await assert.rejects(atEasel(studio, ["do", "-"], "print(1)", ac.signal), /aborted/);
	await assert.rejects(atEasel(studio, ["do", "-"], "print(1)", ac.signal), /aborted/);
	assert.deepEqual(calls(studio), ["status"]);
});

test("look's errors name the tool's options, not the easel's flags", () => {
	assert.equal(toolWords("--crop exceeds 1200 pixels per side; choose a smaller crop"), "crop exceeds 1200 pixels per side; choose a smaller crop");
	assert.equal(toolWords("--mode x: normal, value, squint, mirror"), "mode x: normal, value, squint, mirror");
});

// These client-boundary regressions protect command admission, progress liveness and
// cancellation. Existing hang tests cover only an unresponsive process, not rebuild replies.
// The executable fixture uses the real subprocess boundary; no production test seam.
test("advancing rebuilds outlive the tool budget and leave a fresh budget for the command", async () => {
	const studio = stubStudio(`if [ "$1" = status ]; then
 n=0; [ ! -f progress ] || n=$(cat progress)
 n=$((n + 1)); echo "$n" > progress
 if [ "$n" = 1 ]; then sleep 0.12; else sleep 0.4; fi
 if [ "$n" -le 4 ]; then echo "rebuilding from the log ($n of 4 chunks)"; exit 0; fi
 echo ready; exit 0
fi
sleep 0.2
echo painted`);
	const other = WAIT_MS.other;
	const rebuild = WAIT_MS.rebuild;
	WAIT_MS.other = 300;
	WAIT_MS.rebuild = 800;
	try {
		assert.equal(await atEasel(studio, ["look"], undefined), "painted");
		assert.deepEqual(calls(studio), [...Array(5).fill("status"), "look"]);
	} finally { WAIT_MS.other = other; WAIT_MS.rebuild = rebuild; }
});

test("a rebuild returned by open is polled and a stalled count is bounded", async () => {
	const studio = stubStudio(`if [ "$1" = status ] && [ ! -f opened ]; then exit 1; fi
 touch opened
 echo 'rebuilding from the log (1 of 3 chunks)'`);
	const rebuild = WAIT_MS.rebuild;
	WAIT_MS.rebuild = 250;
	try {
		const start = Date.now();
		await assert.rejects(atEasel(studio, ["look"], undefined), /rebuild.*(stalled|progress)/);
		assert.ok(Date.now() - start < 2000);
		assert.equal(calls(studio).filter(c => c === "open").length, 1);
		assert.ok(!calls(studio).includes("look"));
	} finally { WAIT_MS.rebuild = rebuild; }
});

test("abort during rebuild polling stops admission and removes listeners", async () => {
	const studio = stubStudio("echo 'rebuilding from the log (0 of 9 chunks)'");
	const ac = new AbortController();
	const timer = setTimeout(() => ac.abort(), 250);
	try {
		await assert.rejects(atEasel(studio, ["look"], undefined, ac.signal), /aborted/i);
		assert.ok(calls(studio).every(c => c === "status"));
		assert.equal(getEventListeners(ac.signal, "abort").length, 0);
	} finally { clearTimeout(timer); }
});

test("invalid rebuild replies never admit a command or open another session", async () => {
	const other = WAIT_MS.other;
	WAIT_MS.other = 2000;
	try {
		for (const reply of [
			"echo 'rebuilding from the log (1 of 3 chunks)'; exit 1",
			"echo 'rebuilding from the log (0 of 3 chunks)'",
			"echo 'rebuilding from the log (2 of 4 chunks)'",
			"echo 'rebuilding from the log (4 of 3 chunks)'",
		]) {
			const studio = stubStudio(`if [ ! -f started ]; then
 touch started; echo 'rebuilding from the log (1 of 3 chunks)'; exit 0
fi
${reply}`);
			await assert.rejects(atEasel(studio, ["look"], undefined), /invalid rebuild progress/);
			assert.ok(calls(studio).every(c => c === "status"));
		}
	} finally { WAIT_MS.other = other; }
});
