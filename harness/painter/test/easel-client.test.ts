import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { getEventListeners } from "node:events";
import { join } from "node:path";
import { test, type TestContext } from "node:test";
import { painterEasel, WAIT_MS, studioPath, type PainterCommand } from "../easel-client.ts";
import { stubStudio } from "./fixtures/stub-easel.ts";

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

test("look's options become the easel's arguments", async () => {
	const studio = stubStudio('[ "$1" = status ] && exit 0\nprintf "%s\\n" "$@" > args\necho painted');
	const cases: [PainterCommand & { kind: "look" }, string[]][] = [
		[{ kind: "look" }, []],
		[{ kind: "look", crop: "300,200,500,350", mode: "value,squint", size: 600, grid: 10 }, ["--crop", "300,200,500,350", "--mode", "value,squint", "--size", "600", "--grid", "10"]],
		[{ kind: "look", grid: true }, ["--grid"]],
		[{ kind: "look", grid: false }, []],
		[{ kind: "look", mode: "relief", light: "45,15" }, ["--mode", "relief", "--light", "45,15"]],
		[{ kind: "look", palette: true }, ["--palette"]],
		[{ kind: "look", palette: false }, []],
		[{ kind: "look", survey: true, mode: "gallery" }, ["--survey", "--mode", "gallery"]],
		[{ kind: "look", compare: "out/easel/painting/a.png" }, ["--compare", join(realpathSync(studio), "out/easel/painting/a.png")]],
		[{ kind: "look", hold: "skyP", at: "400,320", mode: "value" }, ["--hold", "skyP", "--at", "400,320", "--mode", "value"]],
	];
	for (const [command, args] of cases) {
		await painterEasel(studio, "painting", command);
		assert.deepEqual(readFileSync(join(studio, "args"), "utf8").trim().split("\n"), ["look", ...args]);
	}
	for (const options of [{ mode: "gallery" }, { compare: "outside.png" }, { survey: true }, { size: 0 }]) {
		await assert.rejects(painterEasel(studio, "painting", { kind: "look", palette: true, ...options }), /palette takes no other option/);
	}
});

test("a long log keeps its end and says what was left out", async () => {
	const studio = stubStudio('[ "$1" = status ] && exit 0\ncat reply');
	writeFileSync(join(studio, "reply"), "abc");
	assert.equal((await painterEasel(studio, "painting", { kind: "log" })).said, "abc");
	const kept = "kept beginning\n" + "a".repeat(50000 - "kept beginning\n".length - "\nkept ending".length) + "\nkept ending";
	writeFileSync(join(studio, "reply"), "discarded prefix\n" + kept);
	for (const canvas of ["painting", "scratch"] as const) {
		const want = `(the first 17 characters are left out; read paintings/lua/${canvas}.lua for all of it)\n${kept}`;
		assert.equal((await painterEasel(studio, canvas, { kind: "log" })).said, canvas === "scratch" ? `[scratch canvas]\n${want}` : want);
	}
});

// With EASEL_BIN (a painter build of the easel): the tools' path through a real session.
const bin = process.env.EASEL_BIN;
async function withRealEasel(t: TestContext, run: (studio: string) => Promise<void>): Promise<void> {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(studio, "bin"));
	mkdirSync(join(studio, "notes"));
	copyFileSync(bin!, join(studio, "bin", "easel"));
	let failed = false;
	try {
		await run(studio);
	} catch (error) {
		failed = true;
		throw error;
	} finally {
		// Match the client's restricted environment, including its absence of TMPDIR.
		const env: Record<string, string> = { PATH: "/usr/bin:/bin", HOME: process.env.HOME ?? "" };
		if (process.env.RAYON_NUM_THREADS !== undefined) env.RAYON_NUM_THREADS = process.env.RAYON_NUM_THREADS;
		const closed = spawnSync(join(studio, "bin/easel"), ["close"], { cwd: studio, env, encoding: "utf8", timeout: 10_000 });
		if (closed.status === 0) rmSync(studio, { recursive: true, force: true });
		else {
			const message = `real easel cleanup failed: ` + (closed.error?.message ?? closed.stderr);
			if (failed) t.diagnostic(message); // don't replace the original assertion failure
			else assert.fail(message);
		}
	}
}

test("paint, look, note and status at a real easel", { skip: !bin || !existsSync(bin) }, async (t) => {
	await withRealEasel(t, async (studio) => {
		// no session yet: the first call opens one
		const ok = await painterEasel(studio, "painting", { kind: "paint", lua: 'canvas{size=400, aspect=1.25, linen=15, ground={{pile={{"lead white", 1}}, um=100, apply="knife"}}}\nprint("hi")' });
		assert.equal(ok.said, "hi\nday 1, 09:00\nok");
		// No print(wait(...)): the successful tool reply supplies the current clock itself.
		assert.equal((await painterEasel(studio, "painting", { kind: "paint", lua: "wait(60)" })).said, "day 1, 10:00\nok");
		await assert.rejects(painterEasel(studio, "painting", { kind: "paint", lua: 'error("boom")' }), /boom[\s\S]*changed nothing/);
		const look = await painterEasel(studio, "painting", { kind: "look", crop: "0,0,200,200" });
		assert.equal(look.said, `${look.paths[0]} (480x480)`);
		assert.match((await painterEasel(studio, "painting", { kind: "note", text: "first note" })).said, /noted/);
		assert.match(readFileSync(join(studio, "notes", "journal.md"), "utf8"), /first note/);
		assert.equal((await painterEasel(studio, "painting", { kind: "status" })).said, "size=400, aspect=1.25, linen={15, 15}, seed=1");
	});
});

test("a failing Lua chunk keeps its own error wording", { skip: !bin || !existsSync(bin) }, async (t) => {
	await withRealEasel(t, async (studio) => {
		await assert.rejects(painterEasel(studio, "painting", { kind: "paint", lua: 'error("mix 3 s into the sky --crop")' }), (error: Error) => {
			assert.match(error.message, /mix 3 s into the sky --crop/);
			assert.match(error.message, /changed nothing/);
			assert.doesNotMatch(error.message, /\[string "chunk \d+"\]/);
			return true;
		});
	});
});

test("a client that hangs is stopped and reported, not waited on forever", async () => {
	const studio = stubStudio('[ "$1" = status ] && exit 0\nsleep 30');
	const other = WAIT_MS.other;
	WAIT_MS.other = 300;
	try {
		const t0 = Date.now();
		await assert.rejects(painterEasel(studio, "painting", { kind: "look" }), /didn't answer/);
		assert.ok(Date.now() - t0 < 5000);
	} finally { WAIT_MS.other = other; }
});

const calls = (studio: string) => readFileSync(join(studio, "calls"), "utf8").trim().split("\n");

test("a tool's opening status keeps to the tool's time limit, and a busy easel isn't opened again", async () => {
	const studio = stubStudio('[ "$1" = status ] && sleep 30\necho ok');
	const other = WAIT_MS.other;
	WAIT_MS.other = 300;
	try {
		const t0 = Date.now();
		await assert.rejects(painterEasel(studio, "painting", { kind: "status" }), /isn't answering/);
		assert.ok(Date.now() - t0 < 5000);
		assert.deepEqual(calls(studio), ["status"]);
	} finally {
		WAIT_MS.other = other;
	}
});

test("an abort stops the tool's chain: nothing after it runs, and no listener is left behind", async () => {
	const studio = stubStudio('[ "$1" = status ] && [ -f slow ] && { sleep 2; exit 1; }\necho ok');
	const kept = new AbortController();
	for (let i = 0; i < 3; i++) await painterEasel(studio, "painting", { kind: "status" }, kept.signal);
	assert.equal(getEventListeners(kept.signal, "abort").length, 0);

	writeFileSync(join(studio, "slow"), "");
	writeFileSync(join(studio, "calls"), "");
	const ac = new AbortController();
	setTimeout(() => ac.abort(), 300);
	await assert.rejects(painterEasel(studio, "painting", { kind: "paint", lua: "print(1)" }, ac.signal), /aborted/);
	await assert.rejects(painterEasel(studio, "painting", { kind: "paint", lua: "print(1)" }, ac.signal), /aborted/);
	assert.deepEqual(calls(studio), ["status"]);
});

test("look's errors name the tool's options, not the easel's flags", async () => {
	const studio = stubStudio('[ "$1" = status ] && exit 0\ncat reply; exit 1');
	for (const [reply, want] of [
		["look: --palette takes no other option", "look: palette takes no other option"],
		["--crop exceeds 1200 pixels per side; choose a smaller crop (crops stay 1:1)", "a crop may be at most 500 units on either side; choose a smaller crop"],
		["--mode x: normal, value, squint, mirror", "mode x: normal, value, squint, mirror"],
		["look: --survey and --compare are two looks; --light needs --mode relief", "look: survey and compare are two looks; light needs mode relief"],
	]) {
		writeFileSync(join(studio, "reply"), reply);
		await assert.rejects(painterEasel(studio, "painting", { kind: "look" }), { message: want });
	}
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
		assert.equal((await painterEasel(studio, "painting", { kind: "look" })).said, "painted");
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
		await assert.rejects(painterEasel(studio, "painting", { kind: "look" }), /rebuild.*(stalled|progress)/);
		assert.ok(Date.now() - start < 2000);
		assert.equal(calls(studio).filter(c => c === "open").length, 1);
		assert.ok(!calls(studio).includes("look"));
	} finally { WAIT_MS.rebuild = rebuild; }
});

test("abort during rebuild polling stops admission and removes listeners", async () => {
	const studio = stubStudio("echo 'rebuilding from the log (0 of 9 chunks)'");
	const ac = new AbortController();
	const rejected = assert.rejects(painterEasel(studio, "painting", { kind: "look" }, ac.signal), /aborted/i);
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
			for (const command of [{ kind: "look" }, { kind: "paint", lua: "x = 1" }] as const) {
				const studio = stubStudio(`if [ ! -f started ]; then
 touch started; echo 'rebuilding from the log (1 of 3 chunks)'; exit 0
fi
${reply}`);
				await assert.rejects(painterEasel(studio, "painting", command), (error: Error) => {
					assert.match(error.message, /invalid rebuild progress/);
					assert.doesNotMatch(error.message, /\d+ of \d+ chunks/);
					return true;
				});
				assert.ok(calls(studio).every(c => c === "status"));
			}
		}
	} finally { WAIT_MS.other = other; }
});

test("an easel that exits before reading a large chunk is the tool's error, with its words", async () => {
	const studio = stubStudio('[ "$1" = status ] && exit 0\necho "the easel stopped"; exit 1');
	await assert.rejects(painterEasel(studio, "painting", { kind: "paint", lua: "x = 1\n".repeat(1_500_000) }), /^Error: the easel stopped$/);
});

test("the easel gets the operator's RAYON_NUM_THREADS and nothing else of the painter's environment", async () => {
	const studio = stubStudio(`[ "$1" = status ] && exit 0
printf 'threads=%s set=%s other=%s\\n' "$RAYON_NUM_THREADS" "\${RAYON_NUM_THREADS+yes}" "$PAINTER_SECRET"`);
	const saved = { threads: process.env.RAYON_NUM_THREADS, secret: process.env.PAINTER_SECRET };
	try {
		process.env.PAINTER_SECRET = "s";
		process.env.RAYON_NUM_THREADS = "6";
		assert.equal((await painterEasel(studio, "painting", { kind: "note", text: "a note" })).said, "threads=6 set=yes other=");
		delete process.env.RAYON_NUM_THREADS;
		assert.equal((await painterEasel(studio, "painting", { kind: "note", text: "a note" })).said, "threads= set= other=");
	} finally {
		for (const [k, v] of [["RAYON_NUM_THREADS", saved.threads], ["PAINTER_SECRET", saved.secret]] as const) {
			if (v === undefined) delete process.env[k];
			else process.env[k] = v;
		}
	}
});

test("a look is renamed at random and shown without the machine's seconds", async () => {
	const s = stubStudio('[ "$1" = status ] && exit 0\necho "out/look-0012.png (1000x714, 0.04s)"');
	mkdirSync(join(s, "out"));
	writeFileSync(join(s, "out", "look-0012.png"), "png");
	const { said, paths: [path] } = await painterEasel(s, "painting", { kind: "look" });
	assert.match(path, /^out\/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}\.png$/);
	assert.equal(said, `${path} (1000x714)`);
	assert.equal(readFileSync(join(s, path), "utf8"), "png");
	assert.ok(!existsSync(join(s, "out", "look-0012.png")));
});

test("every look a survey names is renamed, in order", async () => {
	const s = stubStudio('[ "$1" = status ] && exit 0\ncat reply');
	mkdirSync(join(s, "out"));
	for (const n of ["0004", "0005"]) writeFileSync(join(s, "out", `look-${n}.png`), n);
	writeFileSync(join(s, "reply"), "survey: 1 rows x 2 columns of 500 x 333 units, at full detail\nout/look-0004.png (1200x800): row 1 column 1 (0,0,500,333)\nout/look-0005.png (1200x800): row 1 column 2 (500,0,1000,333)\n");
	const { said, paths } = await painterEasel(s, "painting", { kind: "look", survey: true });
	assert.equal(paths.length, 2);
	assert.equal(readFileSync(join(s, paths[0]), "utf8"), "0004");
	assert.equal(readFileSync(join(s, paths[1]), "utf8"), "0005");
	assert.match(said, /^survey: /);
	assert.ok(said.includes(`${paths[1]} (1200x800): row 1 column 2`));
	writeFileSync(join(s, "out", "look-0006.png"), "0006");
	writeFileSync(join(s, "reply"), "out/look-0006.png (1000x714, 0.04s)");
	const one = await painterEasel(s, "painting", { kind: "look" });
	assert.equal(one.said, `${one.paths[0]} (1000x714)`, "without the machine's seconds");
	// a studio whose folders have spaces in their names; a line that only ends like a look
	mkdirSync(join(s, "my out"));
	writeFileSync(join(s, "my out", "look-0007.png"), "0007");
	writeFileSync(join(s, "reply"), "my out/look-0007.png (1000x714, 0.04s)\nno such look.png");
	const spaced = await painterEasel(s, "painting", { kind: "look" });
	assert.equal(spaced.paths.length, 1);
	assert.equal(readFileSync(join(s, spaced.paths[0]), "utf8"), "0007");
	assert.ok(spaced.said.endsWith("\nno such look.png"));
	// a folder named like a picture: the look inside it is renamed, not the folder
	mkdirSync(join(s, "archive.png (old)"));
	writeFileSync(join(s, "archive.png (old)", "look-0008.png"), "0008");
	writeFileSync(join(s, "reply"), "archive.png (old)/look-0008.png (1000x714, 0.04s)");
	const nested = await painterEasel(s, "painting", { kind: "look" });
	assert.equal(nested.paths.length, 1);
	assert.equal(readFileSync(join(s, nested.paths[0]), "utf8"), "0008");
	assert.ok(nested.paths[0].startsWith(join("archive.png (old)", "")));
});

// The subprocess boundary owns this contract: success bytes stay unchanged. Easel-owned
// failures hide bookkeeping; paint failures preserve the chunk's words. Helper-only
// tests could not catch a caller omitting a translation. No new production test seam.
const privateWords = /\bchunk \d+|\b\d+(?: of \d+)? chunks?\b|look-\d+\.png|\d+(?:\.\d+)? ?s(?: to compute)?\b|\d+ minutes|--[a-z]/;
const commands: { command: PainterCommand; raw: string; want: string; args: string[]; input?: string }[] = [
	{ command: { kind: "paint", lua: "wait(60)", new_scratch: true }, raw: "day 1, 10:00\nok · chunk 12 (76.50 s to compute)", want: "day 1, 10:00\nok", args: ["do", "-", "--scratch", "--new"], input: "wait(60)" },
	{ command: { kind: "look", mode: "gallery" }, raw: "out/look-0012.png (1000x714, 0.04s)", want: " (1000x714)", args: ["look", "--scratch", "--mode", "gallery"] },
	{ command: { kind: "canvasLook" }, raw: "out/look-0012.png (1000x714, 0.04s)", want: " (1000x714)", args: ["look"] },
	{ command: { kind: "note", text: "warm ground" }, raw: "noted", want: "noted", args: ["note", "-"], input: "warm ground" },
	{ command: { kind: "status" }, raw: "12 chunks · 2400px · canvas{size=900}", want: "canvas{size=900}", args: ["status", "--scratch"] },
	{ command: { kind: "log" }, raw: "-- easel session\n--@ engine 2\n\n--@ chunk 1\nx = 1\n\n--@ chunk 12\ny = 2\n", want: "-- easel session\n--@ engine 2\n\n--@ chunk\nx = 1\n\n--@ chunk\ny = 2", args: ["log", "--scratch"] },
	{ command: { kind: "globals" }, raw: "1\tOB\ttable with 2 entries\n12\ttree\tfunction (chunk 12, line 9)", want: "- `OB`: table with 2 entries\n- `tree`: function", args: ["globals"] },
];
for (const { command, raw, want, args, input } of commands) {
	for (const outcome of ["ok", "error"] as const) {
		test(`${command.kind} ${outcome}: the painter sees no work count, compute time or flag`, async () => {
			const canvases = command.kind === "note" || command.kind === "globals" || command.kind === "canvasLook" ? ["painting"] as const : ["painting", "scratch"] as const;
			for (const canvas of canvases) {
				const s = stubStudio(`if [ ! -f admitted ]; then touch admitted; echo ready; exit 0; fi
printf '%s\n' "$@" > args
cat > input
cat reply
exit ${outcome === "ok" ? 0 : 1}`);
				mkdirSync(join(s, "out"));
				writeFileSync(join(s, "out/look-0012.png"), "png");
				const failure = command.kind === "paint"
					? 'runtime error: [string "chunk 12"]:3: boom\nstack traceback:\n\t[string "chunk 4"]:9: in function \'tree\'\nthe chunk ran longer than 10 minutes and was stopped; nothing it did was kept'
					: '12 chunks · 2400px · easel failed --scratch --new --crop\n[string "chunk 12"]:3: boom\n--@ chunk 4\nrebuilding from the log (2 of 3 chunks)\nok · chunk 12 (76.50 s to compute)\nout/look-0012.png (1000x714, 0.04s)\nthe chunk ran longer than 10 minutes and was stopped; nothing it did was kept';
				writeFileSync(join(s, "reply"), outcome === "ok" ? raw : failure);
				const request = command.kind === "paint" ? { ...command, new_scratch: canvas === "scratch" } : command;
				if (outcome === "ok") {
					const result = await painterEasel(s, canvas, request);
					const tag = canvas === "scratch" ? "[scratch canvas]\n" : "";
					assert.equal(result.said, tag + ((command.kind === "look" || command.kind === "canvasLook") ? result.paths[0] : "") + want);
					assert.doesNotMatch(result.said, privateWords);
					if (command.kind === "look" || command.kind === "canvasLook") {
						assert.match(result.paths[0], /^out\/[0-9a-f-]{36}\.png$/);
						assert.equal(readFileSync(join(s, result.paths[0]), "utf8"), "png");
					} else assert.deepEqual(result.paths, []);
					if (command.kind === "status") {
						writeFileSync(join(s, "reply"), "0 chunks · 2400px · no canvas yet");
						assert.equal((await painterEasel(s, canvas, request)).said, tag + "no canvas yet");
					}
				} else {
					await assert.rejects(painterEasel(s, canvas, request), (error: Error) => {
						assert.doesNotMatch(error.message, privateWords);
						assert.equal(error.message, command.kind === "paint"
							? 'runtime error: [string "chunk"]:3: boom\nstack traceback:\n\t[string "chunk"]:9: in function \'tree\'\nthe chunk didn\'t finish and was stopped; nothing it did was kept'
							: 'easel failed scratch new crop\n[string "chunk"]:3: boom\n--@ chunk\nrebuilding from the log (chunks)\nok · chunk\nout/look.png (1000x714)\nthe chunk didn\'t finish and was stopped; nothing it did was kept');
						return true;
					});
				}
				assert.deepEqual(readFileSync(join(s, "args"), "utf8").trim().split("\n"), canvas === "scratch" ? args : args.filter(a => a !== "--scratch" && a !== "--new"));
				assert.equal(readFileSync(join(s, "input"), "utf8"), input ?? "");
			}
		});
	}
}

test("paint preserves the chunk's own output while translating the final easel line", async () => {
	const s = stubStudio('[ "$1" = status ] && exit 0\ncat reply');
	for (const [raw, want] of [
		["ok · chunk 12", "ok"],
		["we took 3 s to compute this\nok · chunk 2 (1.00 s to compute)", "we took 3 s to compute this\nok"],
		["ok · chunk 9 (1.00 s to compute)\nok · chunk 3 (2.00 s to compute)\n", "ok · chunk 9 (1.00 s to compute)\nok"],
	]) {
		writeFileSync(join(s, "reply"), raw);
		assert.equal((await painterEasel(s, "painting", { kind: "paint", lua: "print('a line')" })).said, want);
	}
});

test("a look with no readable image translates the error text returned to either caller", async () => {
	const s = stubStudio('[ "$1" = status ] && exit 0\necho "look: --mode failed after chunk 12 (76.50 s to compute)"');
	const reply = await painterEasel(s, "scratch", { kind: "look" });
	assert.deepEqual(reply.paths, []);
	assert.equal(reply.said, "[scratch canvas]\nlook: mode failed after chunk");
	assert.doesNotMatch(reply.said, privateWords);
});
