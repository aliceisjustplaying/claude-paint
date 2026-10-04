import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { getEventListeners } from "node:events";
import { join } from "node:path";
import { test } from "node:test";
import { atEasel, easel, lookArgs, WAIT_MS, studioPath, tail, toolWords, paintReply } from "../easel-client.ts";

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

test("a name that only begins with two dots is a studio file; a '..' folder step out is not", () => {
	const top = realpathSync(mkdtempSync(join(tmpdir(), "studio-")));
	const real = join(top, "studio");
	mkdirSync(join(real, "notes"), { recursive: true });
	mkdirSync(join(top, "..outside"));
	const cases: [string, string | undefined][] = [
		["..draft.md", join(real, "..draft.md")],
		["notes/..old/a.md", join(real, "notes", "..old", "a.md")],
		["...", join(real, "...")],
		[".", real],
		[real, real],
		["notes/..", real],
		["..", undefined],
		["../", undefined],
		["../..outside/a.md", undefined],
		[join(top, "..outside", "a.md"), undefined],
		["notes/../../studio/x", join(real, "x")],
		["notes/../..", undefined],
	];
	for (const [path, want] of cases) assert.equal(studioPath(real, path), want, path);
});

test("look's options become the easel's arguments", () => {
	assert.deepEqual(lookArgs({}), []);
	assert.deepEqual(lookArgs({ crop: "300,200,500,350", mode: "value,squint", size: 600, grid: 10 }),
		["--crop", "300,200,500,350", "--mode", "value,squint", "--size", "600", "--grid", "10"]);
	assert.deepEqual(lookArgs({ grid: true }), ["--grid"]);
	assert.deepEqual(lookArgs({ grid: false }), []);
	assert.deepEqual(lookArgs({ mode: "relief", light: "45,15" }), ["--mode", "relief", "--light", "45,15"]);
	assert.deepEqual(lookArgs({ palette: true }), ["--palette"]);
	assert.deepEqual(lookArgs({ palette: false }), []);
	assert.equal(toolWords("look: --palette takes no other option"), "look: palette takes no other option");
	assert.deepEqual(lookArgs({ survey: true, mode: "gallery" }), ["--survey", "--mode", "gallery"]);
	assert.deepEqual(lookArgs({ palette: true, mode: "gallery" }), ["--palette"]);
	assert.deepEqual(lookArgs({ compare: "out/easel/painting/a.png" }), ["--compare", "out/easel/painting/a.png"]);
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
		assert.equal(paintReply(ok), "hi\nday 1, 09:00\nok");
		// No print(wait(...)): the successful tool reply supplies the current clock itself.
		assert.equal(paintReply(await atEasel(studio, ["do", "-"], "wait(60)")), "day 1, 10:00\nok");
		await assert.rejects(atEasel(studio, ["do", "-"], 'error("boom")'), /boom[\s\S]*changed nothing/);
		const look = await atEasel(studio, ["look", ...lookArgs({ crop: "0,0,200,200" })], undefined);
		assert.match(look, /look-0001\.png \(480x480/);
		assert.match(await atEasel(studio, ["note", "-"], "first note"), /noted/);
		assert.match(readFileSync(join(studio, "notes", "journal.md"), "utf8"), /first note/);
		assert.match(await atEasel(studio, ["status"], undefined), /^2 chunks/);
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
	assert.equal(toolWords("--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)"), "a crop may be at most 500 units on either side; choose a smaller crop");
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

test("an easel that exits before reading a large chunk is the tool's error, with its words", async () => {
	const studio = stubStudio('[ "$1" = status ] && exit 0\necho "the easel stopped"; exit 1');
	await assert.rejects(atEasel(studio, ["do", "-"], "x = 1\n".repeat(1_500_000)), /^Error: the easel stopped$/);
});

test("the easel gets the operator's RAYON_NUM_THREADS and nothing else of the painter's environment", async () => {
	const studio = stubStudio(`printf 'threads=%s set=%s other=%s\\n' "$RAYON_NUM_THREADS" "\${RAYON_NUM_THREADS+yes}" "$PAINTER_SECRET"`);
	const saved = { threads: process.env.RAYON_NUM_THREADS, secret: process.env.PAINTER_SECRET };
	try {
		process.env.PAINTER_SECRET = "s";
		process.env.RAYON_NUM_THREADS = "6";
		assert.equal((await easel(studio, ["x"], undefined, undefined, 5000)).out, "threads=6 set=yes other=\n");
		delete process.env.RAYON_NUM_THREADS;
		assert.equal((await easel(studio, ["x"], undefined, undefined, 5000)).out, "threads= set= other=\n");
	} finally {
		for (const [k, v] of [["RAYON_NUM_THREADS", saved.threads], ["PAINTER_SECRET", saved.secret]] as const) {
			if (v === undefined) delete process.env[k];
			else process.env[k] = v;
		}
	}
});

test("no reply counts the painter's work or times the machine", async () => {
	const { paintReply, statusReply, logReply, hideCounters } = await import("../easel-client.ts");
	const cases: [(t: string) => string, string, string][] = [
		[paintReply, "drawn and laid in\nok · chunk 1 (76.50 s to compute)\n", "drawn and laid in\nok\n"],
		[paintReply, "ok · chunk 12 (0.03 s to compute)", "ok"],
		[paintReply, "ok · chunk 12", "ok"],
		// a chunk's own print that happens to look like the easel's line is left alone; only the last line is the easel's
		[paintReply, "we took 3 s to compute this\nok · chunk 2 (1.00 s to compute)", "we took 3 s to compute this\nok"],
		[paintReply, "ok · chunk 9 (1.00 s to compute)\nok · chunk 3 (2.00 s to compute)\n", "ok · chunk 9 (1.00 s to compute)\nok\n"],
		[hideCounters, 'runtime error: [string "chunk 12"]:3: boom\nstack traceback:\n\t[string "chunk 4"]:9: in function \'tree\'', 'runtime error: [string "chunk"]:3: boom\nstack traceback:\n\t[string "chunk"]:9: in function \'tree\''],
		[statusReply, "12 chunks · 2400px · canvas{size=900}", "canvas{size=900}"],
		[statusReply, "0 chunks · 2400px · no canvas yet", "no canvas yet"],
		[hideCounters, "the chunk ran longer than 10 minutes and was stopped; nothing it did was kept", "the chunk didn't finish and was stopped; nothing it did was kept"],
		[logReply, "-- easel session\n--@ engine 2\n\n--@ chunk 1\nx = 1\n\n--@ chunk 12\ny = 2\n", "-- easel session\n--@ engine 2\n\n--@ chunk\nx = 1\n\n--@ chunk\ny = 2\n"],
	];
	for (const [f, input, want] of cases) assert.equal(f(input), want, input);
});

test("a look is renamed at random and shown without the machine's seconds", async () => {
	const { renameLook } = await import("../easel-client.ts");
	const s = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(s, "out"));
	writeFileSync(join(s, "out", "look-0012.png"), "png");
	const { said, path } = renameLook(s, "out/look-0012.png (1000x714, 0.04s)");
	assert.match(path, /^out\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.png$/);
	assert.equal(said, `${path} (1000x714)`);
	assert.equal(readFileSync(join(s, path), "utf8"), "png");
	assert.ok(!existsSync(join(s, "out", "look-0012.png")));
});

test("every look a survey names is renamed, in order", async () => {
	const { renameLooks } = await import("../easel-client.ts");
	const s = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(s, "out"));
	for (const n of ["0004", "0005"]) writeFileSync(join(s, "out", `look-${n}.png`), n);
	const { said, paths } = renameLooks(s, "survey: 1 rows x 2 columns of 500 x 333 units, at full detail\nout/look-0004.png (1200x800): row 1 column 1 (0,0,500,333)\nout/look-0005.png (1200x800): row 1 column 2 (500,0,1000,333)\n");
	assert.equal(paths.length, 2);
	assert.equal(readFileSync(join(s, paths[0]), "utf8"), "0004");
	assert.equal(readFileSync(join(s, paths[1]), "utf8"), "0005");
	assert.match(said, /^survey: /);
	assert.ok(said.includes(`${paths[1]} (1200x800): row 1 column 2`));
	writeFileSync(join(s, "out", "look-0006.png"), "0006");
	const one = renameLooks(s, "out/look-0006.png (1000x714, 0.04s)");
	assert.equal(one.said, `${one.paths[0]} (1000x714)`, "without the machine's seconds");
	// a studio whose folders have spaces in their names; a line that only ends like a look
	mkdirSync(join(s, "my out"));
	writeFileSync(join(s, "my out", "look-0007.png"), "0007");
	const spaced = renameLooks(s, "my out/look-0007.png (1000x714, 0.04s)\nno such look.png");
	assert.equal(spaced.paths.length, 1);
	assert.equal(readFileSync(join(s, spaced.paths[0]), "utf8"), "0007");
	assert.ok(spaced.said.endsWith("\nno such look.png"));
	// a folder named like a picture: the look inside it is renamed, not the folder
	mkdirSync(join(s, "archive.png (old)"));
	writeFileSync(join(s, "archive.png (old)", "look-0008.png"), "0008");
	const nested = renameLooks(s, "archive.png (old)/look-0008.png (1000x714, 0.04s)");
	assert.equal(nested.paths.length, 1);
	assert.equal(readFileSync(join(s, nested.paths[0]), "utf8"), "0008");
	assert.ok(nested.paths[0].startsWith(join("archive.png (old)", "")));
});
