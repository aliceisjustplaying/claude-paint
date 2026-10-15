import assert from "node:assert/strict";
import { chmodSync, copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { getEventListeners } from "node:events";
import { basename, dirname, join, resolve } from "node:path";
import { test } from "node:test";
import { atEasel, directoryListing, easel, logFind, lookArgs, probeArgs, WAIT_MS, WIDE_WAIT_MS, wideStudio, studioPath, tail, toolWords, paintReply, renameLooks } from "../easel-client.ts";

test("painting headers set the waits before the new-painting width", async () => {
	const studio = stubStudio(`if [ "$1" = status ] && [ ! -f polled ]; then
 touch polled
 echo 'rebuilding from the log (0 of 1 chunks)'
 exit 0
fi
sleep 0.1
echo ok`);
	const logs = join(studio, "paintings", "lua");
	mkdirSync(logs, { recursive: true });
	const log = join(logs, "painting.lua");
	const width = join(studio, "bin", "width");
	const normal = { ...WAIT_MS };
	const wide = { ...WIDE_WAIT_MS };
	try {
		for (const configured of [undefined, "2400", "4800"]) {
			if (configured === undefined) rmSync(width, { force: true });
			else writeFileSync(width, configured);
			for (const [header, expected] of [
				[undefined, configured === "4800"],
				["--@ width 4800\n--@ chunk 1\n", true],
				["--@ chunk 1\n--@ width 4800\n", false],
				["--@ width 2400\n--@ chunk 1\n", false],
				["--@ sketch\n--@ width 4800\n--@ chunk 1\n", false],
			] as const) {
				if (header === undefined) rmSync(log, { force: true });
				else writeFileSync(log, header);
				assert.equal(wideStudio(studio), expected, `${configured}: ${header}`);
			}
			writeFileSync(log, "--@ engine 7\n--@ width 4800\n--@ chunk 1\n");
			rmSync(join(studio, "polled"), { force: true });
			WAIT_MS.do = WAIT_MS.rebuild = 10;
			WIDE_WAIT_MS.do = WIDE_WAIT_MS.rebuild = 2000;
			assert.equal(await atEasel(studio, ["do", "-"], "print(1)"), "ok");
		}
	} finally {
		Object.assign(WAIT_MS, normal);
		Object.assign(WIDE_WAIT_MS, wide);
		rmSync(studio, { recursive: true, force: true });
	}
});

test("reading a studio directory lists its files and subdirectories in order", () => {
	const studio = realpathSync(mkdtempSync(join(tmpdir(), "studio-")));
	try {
		const reference = join(studio, "reference");
		mkdirSync(reference);
		writeFileSync(join(reference, "reference.png"), "image");
		writeFileSync(join(reference, "details.txt"), "details");
		mkdirSync(join(reference, "crops"));
		assert.equal(directoryListing(studioPath(studio, "reference")!), "crops/\ndetails.txt\nreference.png");
		assert.equal(directoryListing(join(reference, "crops")), "empty folder");
		assert.equal(directoryListing(join(reference, "reference.png")), undefined);
	} finally {
		rmSync(studio, { recursive: true, force: true });
	}
});

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
	for (const options of [{ mode: "gallery" }, { compare: "outside.png" }, { survey: true }, { size: 0 }]) {
		assert.throws(() => lookArgs({ palette: true, ...options }), /palette takes no other option/);
	}
	assert.equal(toolWords("--survey --compare --light"), "survey compare light");
	assert.equal(toolWords("look: --ref is a look of its own: no --survey, --compare or --hold"), "look: ref is a look of its own: no survey, compare or hold");
	assert.deepEqual(lookArgs({ ref: "reference/motif.jpg", mode: "value" }), ["--ref", "reference/motif.jpg", "--mode", "value"]);
	assert.deepEqual(lookArgs({ compare: "out/easel/painting/a.png" }), ["--compare", "out/easel/painting/a.png"]);
	assert.deepEqual(lookArgs({ hold: "skyP", at: "400,320", mode: "value" }), ["--hold", "skyP", "--at", "400,320", "--mode", "value"]);
	assert.equal(toolWords("look: --pose and --side hold a pastel stick"), "look: pose and side hold a pastel stick");
	assert.deepEqual(lookArgs({ hold: "P.glow", at: "400,320", pose: "2,55,30" }), ["--hold", "P.glow", "--at", "400,320", "--pose", "2,55,30"]);
	assert.deepEqual(lookArgs({ hold: "P.glow", at: "400,320", side: "0" }), ["--hold", "P.glow", "--at", "400,320", "--side", "0"]);
});

test("mask overlays pass named masks through the look client", () => {
	assert.deepEqual(lookArgs({ crop: "0,100,1000,200", mask: "shape,soft", size: 300 }),
		["--crop", "0,100,1000,200", "--mask", "shape,soft", "--size", "300"]);
	assert.equal(toolWords("look --mask shape: no global mask with that name"), "look mask shape: no global mask with that name");
	assert.throws(() => lookArgs({ palette: true, mask: "shape" }), /palette takes no other option/);
});

test("a long log keeps its end and says what was left out", () => {
	assert.equal(tail("abc", 5), "abc");
	const t = tail("0123456789", 4);
	assert.match(t, /^\(the first 6 characters are left out; read paintings\/lua\/painting\.lua for all of it\)\n6789$/);
});

test("log's find gives the lines that hold the text, numbered as in the file, with two lines around each", () => {
	const log = ["--@ chunk", "canvas{}", "--@ chunk", "sky = pile{{\"lead white\", 4}}", "b = brush(\"flat\", 20)", "b:load(sky)", "x = 1", "y = 2", "w = 0", "z = 3", "q = 4", "Sky2 = mix{{sky, 1}}"].join("\n");
	assert.equal(
		logFind(log, "SKY", "paintings/lua/painting.lua"),
		[
			'3 lines of paintings/lua/painting.lua hold "SKY"',
			"2- canvas{}",
			"3- --@ chunk",
			'4: sky = pile{{"lead white", 4}}',
			'5- b = brush("flat", 20)',
			"6: b:load(sky)",
			"7- x = 1",
			"8- y = 2",
			"--",
			"10- z = 3",
			"11- q = 4",
			"12: Sky2 = mix{{sky, 1}}",
		].join("\n"),
	);
	assert.equal(logFind("a\nb", "a", "paintings/lua/scratch.lua"), '1 line of paintings/lua/scratch.lua holds "a"\n1: a\n2- b');
	assert.equal(logFind(log, "mask", "paintings/lua/painting.lua"), 'no line of paintings/lua/painting.lua holds "mask"');
	const many = logFind(Array.from({ length: 150 }, (_, i) => `m${i}`).join("\n"), "m", "paintings/lua/painting.lua");
	assert.match(many, /^150 lines of paintings\/lua\/painting\.lua hold "m" \(the first 100 are shown\)\n1: m0\n/);
	assert.match(many, /\n100: m99\n101: m100\n102: m101$/);
});

test("probe takes a point or an area, one of the two", () => {
	assert.deepEqual(probeArgs({ at: "10,20" }), ["--at", "10,20"]);
	assert.deepEqual(probeArgs({ crop: "0,0,50,40" }), ["--crop", "0,0,50,40"]);
	for (const p of [{}, { at: "1,2", crop: "0,0,5,5" }, { at: "" }]) assert.throws(() => probeArgs(p), /^Error: probe: give at: "x,y" or crop: "x0,y0,x1,y1", one of the two$/);
	assert.equal(toolWords("probe: --at 1,2,3: want x,y in canvas units"), "probe: at 1,2,3: want x,y in canvas units");
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
		assert.match(await atEasel(studio, ["probe", ...probeArgs({ at: "100,100" })], undefined), /^at 100,100\ncolor \d+,\d+,\d+ · value \d+\nin gallery light \d+,\d+,\d+ · value \d+\ndrying: dry$/);
		assert.match(await atEasel(studio, ["probe", ...probeArgs({ crop: "0,0,10,10" })], undefined), /^over 0,0,10,10, averaged over 24 x 24 pixels\n[\s\S]*\ndrying: dry 100%$/);
		for (const blocked of [".gallery", ".relief"]) {
			const ordinary = await atEasel(studio, ["look"], undefined);
			const primary = resolve(studio, /^(.+\.png)(?: \(.*)?$/m.exec(ordinary)![1]);
			const companion = join(dirname(primary), blocked, basename(primary));
			const old = readFileSync(companion);
			chmodSync(dirname(companion), 0o555);
			try {
				const renamed = renameLooks(studio, ordinary);
				assert.ok(!existsSync(join(dirname(companion), basename(renamed.paths[0]))));
			} finally {
				chmodSync(dirname(companion), 0o755);
			}
			assert.deepEqual(readFileSync(companion), old);
			const analytical = await atEasel(studio, ["look", "--mode", "value"], undefined);
			const fresh = /^(.+\.png)(?: \(.*)?$/m.exec(analytical)![1];
			assert.notEqual(basename(fresh), basename(primary));
			const renamed = renameLooks(studio, analytical).paths[0];
			for (const folder of [".gallery", ".relief"]) assert.ok(!existsSync(resolve(studio, dirname(renamed), folder, basename(renamed))));
			assert.deepEqual(readFileSync(companion), old);
		}
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

test("status answers a replay of the log with its progress at once", async () => {
	const studio = stubStudio("[ \"$1\" = status ] && echo 'rebuilding from the log (2 of 5 chunks)'");
	try {
		assert.equal(await atEasel(studio, ["status"], undefined), "rebuilding from the log (2 of 5 chunks)");
		assert.deepEqual(calls(studio), ["status", "status"]);
	} finally {
		rmSync(studio, { recursive: true, force: true });
	}
});

test("an easel that is starting is asked for its status again, not opened", async () => {
	const studio = stubStudio(`if [ "$1" = status ]; then
 [ -f asked ] || { touch asked; echo 'the easel is starting; retry in a moment'; exit 1; }
 echo 'rebuilding from the log (2 of 5 chunks)'
fi`);
	try {
		assert.equal(await atEasel(studio, ["status"], undefined), "rebuilding from the log (2 of 5 chunks)");
		assert.deepEqual(calls(studio), ["status", "status", "status"]);
	} finally {
		rmSync(studio, { recursive: true, force: true });
	}
});

test("an open that outlasts the tool's wait says how far the replay has got", async () => {
	const studio = stubStudio(`if [ "$1" = status ]; then
 [ -f opened ] || exit 1
 echo 'rebuilding from the log (1 of 4 chunks)'; exit 0
fi
touch opened; sleep 30`);
	const other = WAIT_MS.other;
	WAIT_MS.other = 300;
	try {
		await assert.rejects(atEasel(studio, ["look"], undefined), /^Error: the easel is reopening the painting from its log and hasn't finished: rebuilding from the log \(1 of 4 chunks\)\. `status` shows how far it has got\.$/);
		assert.deepEqual(calls(studio), ["status", "open", "status"]);
	} finally {
		WAIT_MS.other = other;
		rmSync(studio, { recursive: true, force: true });
	}
});

// what the painter must not hear about a chunk: the machine's time, the tool's wait, the easel's line and mark counts, chunk numbers
const MACHINE_WORDS = /running for|within|\d+ minutes?\b|budget|limit|\bline \d|\d+ lines|of \d|strokes|touches|chunk \d|\(chunk|\d+ chunks?/;

test("a chunk that outlasts the tool's wait is reported as running, in the log, or neither", async () => {
	const doWait = WAIT_MS.do;
	WAIT_MS.do = 300;
	const ran = /^Error: `paint` stopped waiting for the easel's answer, but the chunk ran: it is in the log\.$/;
	const nothing = /^Error: `paint` stopped waiting for the easel's answer, and the chunk isn't in the log: it changed nothing\.$/;
	const cases: [string, RegExp][] = [
		["echo \"painting a chunk (at line 3 of 9): 40 strokes and touches so far, 2 min 30 s on the painting's clock (now day 2, 14:05), running for 12 min 3 s\"", /^Error: `paint` stopped waiting for the easel's answer\. The chunk is still running and isn't in the log yet: painting a chunk: it has taken 2 min 30 s of the painting's time so far, and the painting's clock reads day 2, 14:05\. `status` shows how far it has got while it runs; once it ends, `log` shows whether it was kept\.$/],
		["echo 'painting a chunk: starting, running for 12 min 0 s'", /^Error: `paint` stopped waiting for the easel's answer\. The chunk is still running and isn't in the log yet: painting a chunk: starting\. /],
		["printf '\\n--@ chunk 4\\nx = 1\\n' >> paintings/lua/painting.lua; echo '4 chunks · 2400px · no canvas yet'", ran],
		["printf '\\n--@ chunk 4\\nx = 1\\n' >> paintings/lua/painting.lua; echo 'the chunk is in the log (chunk 4); the easel is finishing its reply, running for 1 s'", ran],
		["echo '3 chunks · 2400px · no canvas yet'", nothing],
		// a fresh scratch canvas: a new log, its setup and then the chunk, or the setup alone
		["printf -- '--@ chunk 1\\ncanvas{}\\n\\n--@ chunk 2\\nx = 1\\n' > paintings/lua/painting.lua; echo '2 chunks · 2400px · no canvas yet'", ran],
		["printf -- '--@ chunk 1\\ncanvas{}\\n' > paintings/lua/painting.lua; echo '1 chunks · 2400px · no canvas yet'", nothing],
	];
	try {
		for (const [status, error] of cases) {
			const studio = stubStudio(`if [ "$1" = status ]; then
 [ -f asked ] || { touch asked; echo '1 chunks · 2400px · no canvas yet'; exit 0; }
 ${status}; exit 0
fi
sleep 30`);
			mkdirSync(join(studio, "paintings", "lua"), { recursive: true });
			writeFileSync(join(studio, "paintings", "lua", "painting.lua"), "--@ chunk 1\nx = 0\n\n--@ chunk 2\nx = 0\n\n--@ chunk 3\nx = 0\n");
			try {
				await assert.rejects(atEasel(studio, ["do", "-"], "x = 1"), (e: Error) => {
					assert.match(String(e), error);
					assert.doesNotMatch(e.message, MACHINE_WORDS);
					return true;
				});
			} finally {
				rmSync(studio, { recursive: true, force: true });
			}
		}
	} finally {
		WAIT_MS.do = doWait;
	}
});

test("look's errors name the tool's options, not the easel's flags", () => {
	assert.equal(toolWords("--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)"), "a crop may be at most 500 units on either side; choose a smaller crop");
	assert.equal(toolWords("--mode x: normal, value, squint, mirror"), "mode x: normal, value, squint, mirror");
	assert.equal(toolWords("look: --survey and --compare are two looks; --light needs --mode relief"), "look: survey and compare are two looks; light needs mode relief");
	assert.equal(toolWords("look: --hold is a look of its own: no --survey or --compare"), "look: hold is a look of its own: no survey or compare");
	assert.equal(toolWords("look --hold: --at 1,2 lies outside the --crop"), "look hold: at 1,2 lies outside the crop");
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
	WAIT_MS.other = 1000;
	WAIT_MS.rebuild = 2000;
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
	const rejected = assert.rejects(atEasel(studio, ["look"], undefined, ac.signal), /aborted/i);
	try {
		const deadline = Date.now() + 5000;
		while (!existsSync(join(studio, "calls")) || calls(studio).length < 2) {
			assert.ok(Date.now() < deadline, "the client must enter rebuild polling before abort");
			await new Promise(resolve => setTimeout(resolve, 10));
		}
		ac.abort();
		await rejected;
		assert.ok(calls(studio).every(c => c === "status"));
		assert.equal(getEventListeners(ac.signal, "abort").length, 0);
	} finally { ac.abort(); await rejected; }
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

test("explicit display budgets reach the easel without forwarding unrelated variables", async () => {
	const studio = stubStudio(`printf '%s %s %s\\n' "$EASEL_ASYNC_DISPLAY" "$EASEL_DISPLAY_THREADS" "$PAINTER_SECRET"`);
	const saved = { ...process.env };
	try {
		process.env.EASEL_ASYNC_DISPLAY = "1";
		process.env.EASEL_DISPLAY_THREADS = "2";
		process.env.PAINTER_SECRET = "s";
		assert.equal((await easel(studio, ["x"], undefined, undefined, 5000)).out, "1 2 \n");
	} finally {
		for (const key of ["EASEL_ASYNC_DISPLAY", "EASEL_DISPLAY_THREADS", "PAINTER_SECRET"]) {
			if (saved[key] === undefined) delete process.env[key];
			else process.env[key] = saved[key];
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
		// a running chunk: what is happening and the painting's own time, nothing of the machine's
		[statusReply, "painting a chunk (at line 8 of 20): 100 strokes and touches so far, 5 s on the painting's clock (now day 1, 09:00), running for 12 min 3 s", "painting a chunk: it has taken 5 s of the painting's time so far, and the painting's clock reads day 1, 09:00"],
		[statusReply, "painting a chunk (20 lines): 0 strokes and touches so far, 14 min 20 s on the painting's clock (now day 3, 18:40), running for 40 s", "painting a chunk: it has taken 14 min 20 s of the painting's time so far, and the painting's clock reads day 3, 18:40"],
		[statusReply, "painting a chunk: starting, running for 2 s", "painting a chunk: starting"],
		[statusReply, "the chunk is in the log (chunk 2); the easel is finishing its reply, running for 1 min 15 s", "the chunk is in the log; the easel is finishing its reply"],
		// an easel line worded otherwise still tells only that a chunk is painting
		[statusReply, "painting a chunk (line 3/9): 40 marks, 12 min elapsed", "painting a chunk"],
		[logReply, "-- easel session\n--@ engine 2\n\n--@ chunk 1\nx = 1\n\n--@ chunk 12\ny = 2\n", "-- easel session\n--@ engine 2\n\n--@ chunk\nx = 1\n\n--@ chunk\ny = 2\n"],
	];
	for (const [f, input, want] of cases) assert.equal(f(input), want, input);
	for (const [f, input] of cases) if (f === statusReply) assert.doesNotMatch(f(input), MACHINE_WORDS, input);
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

test("a renamed crop keeps its measurement scale", async () => {
	const s = mkdtempSync(join(tmpdir(), "studio-"));
	try {
		mkdirSync(join(s, "out"));
		writeFileSync(join(s, "out", "look-0012.png"), "png");
		const { said, paths } = renameLooks(s, "out/look-0012.png (1200x216, 0.04s); scale 1.2000 x 1.2000 px/unit");
		assert.equal(paths.length, 1);
		assert.equal(said, `${paths[0]} (1200x216); scale 1.2000 x 1.2000 px/unit`);
	} finally {
		rmSync(s, { recursive: true, force: true });
	}
});

test("background display revisions keep their stable paths while pending or complete", async () => {
	const { renameLook } = await import("../easel-client.ts");
	const s = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(s, "out", ".display"), { recursive: true });
	const path = "out/65c0681641604c1b93c51bdcfd66302e.png";
	writeFileSync(join(s, path), "color");
	for (const state of ["pending", "ready", "failed"]) {
		writeFileSync(join(s, "out", ".display", basename(path)), state + "\n");
		assert.deepEqual(renameLook(s, `${path} (1000x714, 0.04s)`), { path, said: `${path} (1000x714)` });
		assert.deepEqual(renameLooks(s, `${path} (1000x714, 0.04s)`), { paths: [path], said: `${path} (1000x714)` });
		assert.equal(readFileSync(join(s, path), "utf8"), "color");
	}
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

test("look renames keep both hidden display siblings without naming them in replies", async () => {
	const { renameLook, renameLooks } = await import("../easel-client.ts");
	const s = mkdtempSync(join(tmpdir(), "studio-gallery-"));
	const folder = "my archive.png (old)";
	for (const companion of [".gallery", ".relief"]) mkdirSync(join(s, folder, companion), { recursive: true });
	for (const name of ["whole", "tile-a", "tile-b"]) {
		writeFileSync(join(s, folder, `${name}.png`), name);
		for (const companion of [".gallery", ".relief"]) writeFileSync(join(s, folder, companion, `${name}.png`), `${companion} ${name}`);
	}
	const one = renameLook(s, `${folder}/whole.png (1000x800, 0.04s)`);
	assert.equal(one.said, `${one.path} (1000x800)`);
	assert.equal(readFileSync(join(s, one.path), "utf8"), "whole");
	const many = renameLooks(s, `${folder}/tile-a.png (1200x960): row 1 column 1\n${folder}/tile-b.png (1200x960): row 1 column 2\n`);
	assert.equal(many.paths.length, 2);
	for (const [path, name] of [[one.path, "whole"], [many.paths[0], "tile-a"], [many.paths[1], "tile-b"]]) {
		assert.ok(!existsSync(join(s, folder, `${name}.png`)));
		for (const companion of [".gallery", ".relief"]) {
			assert.equal(readFileSync(join(s, dirname(path), companion, basename(path)), "utf8"), `${companion} ${name}`);
			assert.ok(!existsSync(join(s, folder, companion, `${name}.png`)));
		}
	}
	for (const companion of [".gallery", ".relief"]) {
		assert.ok(!one.said.includes(companion));
		assert.ok(!many.said.includes(companion));
	}
	assert.deepEqual(Object.keys(one).sort(), ["path", "said"]);
	assert.deepEqual(Object.keys(many).sort(), ["paths", "said"]);
});

test("each failed companion rename leaves the other companion and primary reply intact", async (t) => {
	const { renameLook, renameLooks } = await import("../easel-client.ts");
	const warnings: string[] = [];
	t.mock.method(console, "error", (warning: string) => warnings.push(warning));
	for (const blocked of [".gallery", ".relief"]) {
		for (const rename of [renameLook, renameLooks]) {
			const s = mkdtempSync(join(tmpdir(), "studio-companion-failure-"));
			mkdirSync(join(s, "out"));
			writeFileSync(join(s, "out", "whole.png"), "original");
			for (const companion of [".gallery", ".relief"]) {
				mkdirSync(join(s, "out", companion));
				writeFileSync(join(s, "out", companion, "whole.png"), companion);
			}
			chmodSync(join(s, "out", blocked), 0o555);
			try {
				const result = rename(s, "out/whole.png (1000x800, 0.04s)");
				const path = "path" in result ? result.path : result.paths[0];
				assert.equal(result.said, `${path} (1000x800)`);
				assert.equal(readFileSync(join(s, path), "utf8"), "original");
				assert.equal(readFileSync(join(s, "out", blocked, "whole.png"), "utf8"), blocked);
				const other = blocked === ".gallery" ? ".relief" : ".gallery";
				assert.equal(readFileSync(join(s, "out", other, basename(path)), "utf8"), other);
				assert.ok(!existsSync(join(s, "out", other, "whole.png")));
				assert.match(warnings.at(-1)!, new RegExp(`couldn't rename ${blocked.slice(1)} companion`));
			} finally {
				chmodSync(join(s, "out", blocked), 0o755);
			}
		}
	}
	assert.equal(warnings.length, 4);
});
