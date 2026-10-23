// node --test harness/painter/test/recovery.test.ts  (TMPDIR: a scratch directory; runs pi)
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { CARRIES_OVER, HEADER } from "../summary.ts";

const HERE = join(import.meta.dirname, "..");
const PNG = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

/** A studio whose bin/easel answers status, globals and look (a 1-pixel PNG), with a brief and a journal. */
function studio(unreadable = false): { dir: string; studio: string } {
	const tmp = process.env.TMPDIR;
	assert.ok(tmp, "set TMPDIR to a scratch directory");
	const dir = mkdtempSync(join(tmp, "recovery-test-"));
	const s = join(dir, "studio");
	for (const d of ["bin", "notes", "paintings/lua", "out/easel/painting"]) mkdirSync(join(s, d), { recursive: true });
	writeFileSync(join(s, "BRIEF.md"), "# Paint a picture\n");
	writeFileSync(join(s, "notes/journal.md"), "- day 1, 09:00: began\n");
	writeFileSync(join(s, "paintings/lua/painting.lua"), "--@ chunk 1\ncanvas{}\n");
	writeFileSync(join(s, "out/easel/painting/fresh.png"), Buffer.from(PNG, "base64"));
	writeFileSync(join(s, "bin/easel"), `#!${process.execPath}
const { chmodSync, copyFileSync } = require('node:fs');
switch (process.argv[2]) {
  case 'status': console.log('1 chunks · 2400px'); break;
  case 'globals': console.log('1\\tsky\\ttable with 2 entries'); break;
  case 'look':
    copyFileSync('out/easel/painting/fresh.png', 'out/easel/painting/look-0007.png');
    if (${unreadable}) chmodSync('out/easel/painting/look-0007.png', 0o000);
    console.log('out/easel/painting/look-0007.png (1x1, 0.01s)');
    break;
  default: console.error('no ' + process.argv[2]); process.exit(1);
}
`, { mode: 0o755 });
	return { dir, studio: s };
}

type Requests = { role: string; text: string; images: number }[][];

function run(env: Record<string, string>, prompt: string, at = studio(), more: string[] = []): { requests: Requests; reply: string; at: ReturnType<typeof studio> } {
	const { dir, studio: s } = at;
	const out = join(dir, "requests.jsonl");
	writeFileSync(out, "");
	const reply = execFileSync("pi", ["--print", ...more, "--no-extensions", "-e", join(HERE, "painter.ts"), "-e", join(HERE, "compaction.ts"),
		"-e", join(HERE, "test/fixtures/faux-record.ts"), "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve",
		"--session-dir", join(dir, "sessions"), "--provider", "faux", "--model", "m", prompt],
		{ cwd: s, env: { ...process.env, FAUX_OUT: out, ...env }, stdio: ["ignore", "pipe", "pipe"], encoding: "utf8" });
	return { requests: readFileSync(out, "utf8").trim().split("\n").map((l) => JSON.parse(l)), reply: reply.trim(), at };
}

test("a later sitting opens with the brief, journal, globals, clock and a fresh look at the canvas", () => {
	const { requests, reply } = run({ PAINTER_SITTING_RECOVERY: "1" }, "You're back at the easel.");
	assert.equal(reply, "done");
	const [user] = requests[0].filter((m) => m.role === "user");
	assert.match(user.text, /^You're back at the easel\.\n\n## BRIEF\.md/);
	for (const s of ["## Your journal", "> - day 1, 09:00: began", "## Your globals", "`sky`: table with 2 entries", "## The canvas clock", "## The canvas",
		]) assert.ok(user.text.includes(s), s);
	assert.match(user.text, /The whole canvas as it was when this was written: out\/easel\/painting\/[0-9a-f-]{36}\.png/);
	assert.doesNotMatch(user.text, /chunk \d|chunks|look-\d|\d s\)/);
	assert.doesNotMatch(user.text, /condensed/);   // not a compaction
	assert.equal(user.images, 1);
});

test("a first sitting's message is left as it is", () => {
	const { requests } = run({}, "Your brief is in BRIEF.md.");
	assert.deepEqual(requests[0].filter((m) => m.role === "user").map((m) => [m.text, m.images]), [["Your brief is in BRIEF.md.", 0]]);
});

test("failed recovery and compaction attachments are reported and retried until an image is loaded", () => {
	const at = studio(true);
	run({}, "Test recovery hooks.", at, ["-e", join(HERE, "test/fixtures/recovery-hooks.ts")]);
	assert.equal(readFileSync(join(at.studio, "recovery-hooks-ok"), "utf8"), "passed");
});

/** The first request after a compaction, and the index of the summary in it. */
function compacted(extra: Record<string, string> = {}): { after: Requests[number]; i: number } {
	// a long answer puts the session over the (lowered) compaction point; the next prompt compacts first
	// (two long turns: pi keeps the last 20K tokens as they are, so the first is what gets summarized)
	const env = { PAINTER_COMPACT_RESERVE: "170000", ...extra };
	const first = run({ ...env, FAUX_SCRIPT: "long" }, "paint");
	run({ ...env, FAUX_SCRIPT: "long" }, "more", first.at, ["--continue"]);
	const { requests } = run(env, "go on", first.at, ["--continue"]);
	const after = requests[0];
	const i = after.findIndex((m) => m.text.startsWith("The conversation history before this point was compacted") || m.text.includes("Earlier parts of this session were condensed."));
	assert.ok(i >= 0, JSON.stringify(after.map((m) => [m.role, m.text.slice(0, 60)])));
	return { after, i };
}

test("after a compaction the painter sees the canvas as it was when the summary was written", () => {
	const { after, i } = compacted();
	assert.doesNotMatch(after[i].text, /What follows is what carries over/);
	assert.match(after[i].text, /The whole canvas as it was when this was written: out\/easel\/painting\/[0-9a-f-]{36}\.png/);
	assert.match(after[i + 1].text, /^The canvas as it was when this was written \(out\/easel\/painting\/[0-9a-f-]{36}\.png\):/);
	assert.equal(after[i + 1].images, 1);
});

test("with PAINTER_COMPACTION_NOTES=1 the summary says what carries over, right after its header", () => {
	const { after, i } = compacted({ PAINTER_COMPACTION_NOTES: "1" });
	assert.ok(after[i].text.includes(`${HEADER}\n\n${CARRIES_OVER}\n\n## BRIEF.md`), after[i].text.slice(0, 800));
});
