import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { reference, referenceList } from "../references.ts";

function studio() {
	const root = mkdtempSync(join(process.env.TMPDIR || tmpdir(), "references-"));
	mkdirSync(join(root, "notes"));
	return root;
}

test("search retrieves chosen topics, never a default reference", () => {
	const root = studio();
	writeFileSync(join(root, "notes/reference-library.json"), JSON.stringify([
		{ id: "a", title: "Tree structure", keywords: ["branches"], text: "The trunk supports limbs." },
		{ id: "b", title: "Paint glazing", keywords: ["transparent"], text: "A glaze goes over dry paint." },
	]));
	assert.deepEqual(reference(root, { query: " branches  TRUNK " }), { matches: [{ id: "a", title: "Tree structure" }] });
	assert.deepEqual(reference(root, { query: "transparent glaze" }), { matches: [{ id: "b", title: "Paint glazing" }] });
	assert.deepEqual(reference(root, { query: "unavailable" }), { matches: [], message: "No matching references." });
	assert.deepEqual(referenceList(root, {}), { references: [{ id: "a", title: "Tree structure" }, { id: "b", title: "Paint glazing" }] });
	assert.throws(() => referenceList(root, { query: "tree" }));
	for (const args of [{}, { query: "" }, { query: "a", id: "b" }, { query: "a", page: 1 }, { id: "../private" }, { id: "a", page: 0 }, { id: "a", page: 1.5 }]) {
		assert.throws(() => reference(root, args));
	}
});

test("every page reconstructs the full text, including multibyte boundaries", () => {
	const root = studio();
	const original = "x".repeat(3999) + "雪🌿\n".repeat(2200) + "END\n";
	writeFileSync(join(root, "notes/reference-library.json"), JSON.stringify([{ id: "a", title: "Reference", keywords: [], text: original }]));
	let received = "", page = 1;
	while (true) {
		const result = reference(root, { id: "a", page });
		assert.ok("text" in result);
		assert.ok(Buffer.byteLength(result.text) <= 4000);
		assert.ok(Buffer.byteLength(JSON.stringify(result)) < 8000);
		received += result.text;
		if (result.next_page === null) break;
		page = result.next_page;
	}
	assert.equal(received, original);
	assert.throws(() => reference(root, { id: "a", page: page + 1 }));
});

test("duplicate ids are rejected, so a read never returns another entry's text", () => {
	const root = studio();
	writeFileSync(join(root, "notes/reference-library.json"), JSON.stringify([
		{ id: "same", title: "First", keywords: [], text: "First entry text." },
		{ id: "same", title: "Second", keywords: [], text: "Second entry text." },
	]));
	assert.throws(() => referenceList(root, {}), /duplicate id "same"/);
	assert.throws(() => reference(root, { query: "second" }), /duplicate id "same"/);
	assert.throws(() => reference(root, { id: "same" }), /duplicate id "same"/);
});

test("a missing library or escaping symlink is not read", () => {
	const root = studio();
	assert.throws(() => reference(root, { query: "anything" }), /no reference library/);
	assert.throws(() => referenceList(root, {}), /no reference library/);
	const outside = join(studio(), "private.json");
	writeFileSync(outside, "[]");
	symlinkSync(outside, join(root, "notes/reference-library.json"));
	assert.throws(() => reference(root, { query: "anything" }), /no reference library/);
	assert.throws(() => referenceList(root, {}), /no reference library/);
});
