import assert from "node:assert/strict";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { atEasel, easel, inStudio, lookArgs, tail } from "../easel-client.ts";

test("read stays inside the studio, links included", () => {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(studio, "notes"));
	writeFileSync(join(studio, "notes", "a.md"), "a");
	symlinkSync("/etc/hosts", join(studio, "notes", "out"));
	assert.equal(inStudio(studio, "notes/a.md"), true);
	assert.equal(inStudio(studio, join(studio, "BRIEF.md")), true); // not there yet: still the studio
	assert.equal(inStudio(studio, "../other/BRIEF.md"), false);
	assert.equal(inStudio(studio, "/etc/hosts"), false);
	assert.equal(inStudio(studio, "notes/out"), false);
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
