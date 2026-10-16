import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { writeNotebook } from "../notebook.ts";

const studio = () => mkdtempSync(join(tmpdir(), "notebook-"));
const revisions = (s: string) => readFileSync(join(s, "out/easel/notebook-revisions.jsonl"), "utf8").trim().split("\n").map((l) => JSON.parse(l));

test("entries are appended after a blank line, with no stamp, and every call is kept", () => {
	const s = studio();
	const at = new Date("2026-10-20T21:00:00Z");
	assert.equal(writeNotebook(s, "Glazes want a set underlayer.", undefined, at), `noted in ${join(s, "notebook.md")}\n`);
	writeNotebook(s, "Rags lift wet paint.\n", undefined, at);
	assert.equal(readFileSync(join(s, "notebook.md"), "utf8"), "Glazes want a set underlayer.\n\nRags lift wet paint.\n");
	assert.deepEqual(revisions(s), [
		{ at: "2026-10-20T21:00:00.000Z", added: "Glazes want a set underlayer.", before: "" },
		{ at: "2026-10-20T21:00:00.000Z", added: "Rags lift wet paint.\n", before: "Glazes want a set underlayer.\n" },
	]);
});

test("a notebook carried over without a final newline still gets a blank line before the next entry", () => {
	const s = studio();
	writeFileSync(join(s, "notebook.md"), "from the last painting");
	writeNotebook(s, "new");
	assert.equal(readFileSync(join(s, "notebook.md"), "utf8"), "from the last painting\n\nnew\n");
});

test("replaces swaps the one passage it names, and refuses what note refuses", () => {
	const s = studio();
	writeNotebook(s, "warm ground\ncool ground");
	assert.equal(writeNotebook(s, "ochre ground", "warm ground"), `revised in ${join(s, "notebook.md")}\n`);
	assert.equal(readFileSync(join(s, "notebook.md"), "utf8"), "ochre ground\ncool ground\n");
	assert.deepEqual(revisions(s)[1].replaced, "warm ground");
	assert.throws(() => writeNotebook(s, "x", " "), /notebook: `replaces` is empty/);
	assert.throws(() => writeNotebook(s, "x", "missing"), /isn't in notebook.md/);
	assert.throws(() => writeNotebook(s, "x", "ground"), /more than once/);
	assert.throws(() => writeNotebook(studio(), "x", "anything"), /notebook.md has no entries yet/);
	assert.equal(revisions(s).length, 2);
});
