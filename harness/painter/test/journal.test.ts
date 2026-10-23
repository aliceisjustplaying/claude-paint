// node --test harness/painter/test/journal.test.ts  (TMPDIR: a scratch directory; EASEL_BIN: the easel)
import assert from "node:assert/strict";
import { type ChildProcess, spawn, spawnSync } from "node:child_process";
import { closeSync, existsSync, mkdirSync, mkdtempSync, openSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { setTimeout as sleep } from "node:timers/promises";
import { JOURNAL, JOURNAL_LOCK, openPassages, REVISIONS, reviseJournal } from "../journal.ts";

function studio(journal?: string): string {
	assert.ok(process.env.TMPDIR, "set TMPDIR to a scratch directory");
	assert.ok(process.env.EASEL_BIN, "set EASEL_BIN to the easel");
	const s = mkdtempSync(join(process.env.TMPDIR, "journal-test-"));
	mkdirSync(join(s, "bin"));
	symlinkSync(process.env.EASEL_BIN, join(s, "bin", "easel"));
	if (journal !== undefined) {
		mkdirSync(join(s, "notes"));
		writeFileSync(join(s, JOURNAL), journal);
	}
	return s;
}

const J = "- day 1, 09:00: the sky is finished\n- day 2, 10:00: glaze the shadows\n";

test("a revision replaces one passage and keeps what was there in the revisions log", () => {
	const s = studio(J);
	reviseJournal(s, "the sky is finished", "the sky needs a second pass", new Date("2026-10-01T12:00:00Z"));
	assert.equal(readFileSync(join(s, JOURNAL), "utf8"), "- day 1, 09:00: the sky needs a second pass\n- day 2, 10:00: glaze the shadows\n");
	const [rev] = readFileSync(join(s, REVISIONS), "utf8").trim().split("\n").map((l) => JSON.parse(l));
	assert.deepEqual(rev, { at: "2026-10-01T12:00:00.000Z", replaced: "the sky is finished", with: "the sky needs a second pass", before: J });
});

test("a passage that isn't there, is there twice or is empty changes nothing", () => {
	const s = studio(J + "- day 3, 08:00: glaze the shadows\n");
	assert.throws(() => reviseJournal(s, "the sea", "x"), /isn't in notes\/journal\.md/);
	assert.throws(() => reviseJournal(s, "glaze the shadows", "x"), /more than once/);
	assert.throws(() => reviseJournal(s, "  ", "x"), /empty/);
	assert.throws(() => reviseJournal(studio(), "a", "b"), /no entries yet/);
	assert.equal(readFileSync(join(s, JOURNAL), "utf8"), J + "- day 3, 08:00: glaze the shadows\n");
});

test("an edit made while a revision is written is kept and the revision refused", () => {
	const s = studio(J);
	const revise = (old: string, text: string, edit: string) => spawnSync(process.execPath, ["--experimental-strip-types", "--input-type=module", "-e", `
import fs from 'node:fs';
import {syncBuiltinESMExports} from 'node:module';
const read = fs.readFileSync;
let edited = false;
fs.readFileSync = (p, ...rest) => {
 const text = read(p, ...rest);
 if (!edited && String(p).endsWith('journal.md')) { edited = true; fs.appendFileSync(p, ${JSON.stringify(edit)}); }
 return text;
};
syncBuiltinESMExports();
const {reviseJournal} = await import(${JSON.stringify(new URL("../journal.ts", import.meta.url).href)});
try { reviseJournal(${JSON.stringify(s)}, ${JSON.stringify(old)}, ${JSON.stringify(text)}); } catch (e) { console.log(e.message); }
`], { encoding: "utf8" });
	// before the journal's first revision, and after it (a published journal)
	let child = revise("the sky is finished", "the sky needs a second pass", "- day 3, 11:00: concurrent\n");
	assert.equal(child.status, 0, child.stderr);
	assert.match(child.stdout, /changed while this revision was being written/);
	assert.equal(readFileSync(join(s, JOURNAL), "utf8"), J + "- day 3, 11:00: concurrent\n");
	assert.ok(!existsSync(join(s, REVISIONS)));
	reviseJournal(s, "the sky is finished", "the sky needs a second pass");
	const revised = J.replace("the sky is finished", "the sky needs a second pass") + "- day 3, 11:00: concurrent\n";
	assert.equal(readFileSync(join(s, JOURNAL), "utf8"), revised);
	child = revise("glaze the shadows", "glaze the hill", "- day 4, 09:00: another\n");
	assert.match(child.stdout, /changed while this revision was being written/);
	assert.equal(readFileSync(join(s, JOURNAL), "utf8"), revised + "- day 4, 09:00: another\n");
	assert.equal(readFileSync(join(s, REVISIONS), "utf8").trim().split("\n").length, 1);
});

test("a note and a revision wait for the journal's lock, and both are kept", async (t) => {
	const s = studio(J);
	const env = { EASEL_ROOT: s };
	const easel = join(s, "bin", "easel");
	t.after(() => spawnSync(easel, ["-s", "a", "close"], { cwd: s, env }));
	assert.equal(spawnSync(easel, ["open", "a"], { cwd: s, env, encoding: "utf8" }).status, 0);
	mkdirSync(join(s, "out/easel"), { recursive: true });
	let lock: number | undefined = openSync(join(s, JOURNAL_LOCK), "a");
	try {
		assert.equal(spawnSync(easel, ["lock-journal"], { cwd: s, env, stdio: [lock, "ignore", "inherit"] }).status, 0);
		const exited = (c: ChildProcess) => new Promise<number | null>((done) => c.on("exit", done));
		const note = spawn(easel, ["-s", "a", "note", "the hill is next"], { cwd: s, env, stdio: ["ignore", "ignore", "inherit"] });
		const revision = spawn(process.execPath, ["--experimental-strip-types", "--input-type=module", "-e",
			`const {reviseJournal} = await import(${JSON.stringify(new URL("../journal.ts", import.meta.url).href)}); reviseJournal(${JSON.stringify(s)}, "the sky is finished", "the sky needs a second pass");`], { stdio: "inherit" });
		const [noted, revised] = [exited(note), exited(revision)];
		await sleep(500);
		assert.equal(note.exitCode, null);
		assert.equal(revision.exitCode, null);
		assert.equal(readFileSync(join(s, JOURNAL), "utf8"), J);
		closeSync(lock);
		lock = undefined;
		assert.equal(await noted, 0);
		assert.equal(await revised, 0);
		const journal = readFileSync(join(s, JOURNAL), "utf8");
		assert.ok(journal.startsWith(J.replace("the sky is finished", "the sky needs a second pass")), journal);
		assert.match(journal, /: the hill is next\n$/);
	} finally {
		if (lock !== undefined) closeSync(lock);
	}
});

test("the latest Open passages list comes out of the journal as the painter wrote it", () => {
	const j = "- day 1, 09:00: began\n- day 5, 22:05: ## Handoff\n  The picture: a heath\n\n  **Open passages:**\n" +
		"  - Sky left of the trunk: patches show in raking light\n    (x 100-195)\n\n  - Hill: speckle\n    - along the right slope\n\n" +
		"  Practical lessons:\n  - glaze thin\n- day 6, 10:00: open passages are slow work\n";
	assert.equal(openPassages(j), "**Open passages:**\n- Sky left of the trunk: patches show in raking light\n  (x 100-195)\n\n- Hill: speckle\n  - along the right slope");
	assert.equal(openPassages(j + "- day 7, 10:00: Open passages: none\n"), "Open passages: none");
	assert.equal(openPassages(j + "- day 8, 10:00: Open passages:\n  - the far shore\n- day 9, 11:00: dried\n"), "Open passages:\n- the far shore");
	assert.equal(openPassages("- day 1, 09:00: began\n"), undefined);
});
