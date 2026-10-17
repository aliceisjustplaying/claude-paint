// node --test harness/painter/test/journal.test.ts  (TMPDIR: a scratch directory)
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";
import { JOURNAL, openPassages, REVISIONS, reviseJournal } from "../journal.ts";

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

test("the latest Open passages list comes out of the journal as the painter wrote it", () => {
	const j = "- day 1, 09:00: began\n- day 5, 22:05: ## Handoff\n  The picture: a heath\n\n  **Open passages:**\n" +
		"  - Sky left of the trunk: patches show in raking light\n    (x 100-195)\n\n  - Hill: speckle\n    - along the right slope\n\n" +
		"  Practical lessons:\n  - glaze thin\n- day 6, 10:00: open passages are slow work\n";
	assert.equal(openPassages(j), "**Open passages:**\n- Sky left of the trunk: patches show in raking light\n  (x 100-195)\n\n- Hill: speckle\n  - along the right slope");
	assert.equal(openPassages(j + "- day 7, 10:00: Open passages: none\n"), "Open passages: none");
	assert.equal(openPassages(j + "- day 8, 10:00: Open passages:\n  - the far shore\n- day 9, 11:00: dried\n"), "Open passages:\n- the far shore");
	assert.equal(openPassages("- day 1, 09:00: began\n"), undefined);
});
