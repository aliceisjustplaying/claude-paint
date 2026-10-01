// node --test harness/painter/test/journal.test.ts  (TMPDIR: a scratch directory)
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { JOURNAL, REVISIONS, reviseJournal } from "../journal.ts";

function studio(journal?: string): string {
	assert.ok(process.env.TMPDIR, "set TMPDIR to a scratch directory");
	const s = mkdtempSync(join(process.env.TMPDIR, "journal-test-"));
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
