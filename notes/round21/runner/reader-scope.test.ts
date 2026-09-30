// node --test notes/round21/runner/reader-scope.test.ts
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, realpathSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { readerScope, readPath, writePath } from "./reader-scope.ts";

function fixture() {
	const top = realpathSync(mkdtempSync(join(tmpdir(), "reader-")));
	const run = join(top, "run");
	const studio = join(top, "paint-studio-1");
	const other = join(top, "paint-studio-2");
	for (const d of [run, join(studio, "notes"), other, join(top, "sessions")]) mkdirSync(d, { recursive: true });
	const log = join(top, "sessions", "s1.jsonl");
	const journal = join(studio, "notes", "journal.md");
	const brief = join(run, "p1_reader_brief.md");
	for (const f of [log, journal, brief, join(other, "BRIEF.md"), join(studio, "BRIEF.md"), join(top, "secret")])
		writeFileSync(f, "x");
	symlinkSync(join(top, "secret"), join(run, "escape"));          // a link out of the run folder
	symlinkSync(log, join(run, "log-link"));                        // a link to a file it may read
	const out = join(run, "p1_record.md");
	const scope = readerScope(JSON.stringify({ read: [log, journal, brief], write: out }));
	return { top, run, studio, other, log, journal, brief, out, scope };
}

test("the reader reads only the files it was given, however the path is spelled", () => {
	const f = fixture();
	for (const p of [f.log, f.journal, "p1_reader_brief.md", `@${f.brief}`, join(f.run, "log-link"),
		join(f.run, "..", "sessions", "s1.jsonl")])
		assert.ok(readPath(f.scope, f.run, p), p);
	assert.equal(readPath(f.scope, f.run, "log-link"), f.log);          // the checked real path
	for (const p of [join(f.other, "BRIEF.md"), join(f.studio, "BRIEF.md"), join(f.top, "secret"), "escape",
		"~/.ssh/id_ed25519", "/etc/passwd", f.run, join(f.top, "sessions"), "p1_record.md"])
		assert.equal(readPath(f.scope, f.run, p), undefined, p);
});

test("the reader writes only its record, and can read it back once written", () => {
	const f = fixture();
	assert.equal(writePath(f.scope, f.run, "p1_record.md"), f.out);
	assert.equal(writePath(f.scope, f.run, f.out), f.out);
	for (const p of [f.log, f.journal, f.brief, "escape", join(f.run, "p2_record.md"), join(f.studio, "notes", "studio_notes.md"),
		"~/p1_record.md"])
		assert.equal(writePath(f.scope, f.run, p), undefined, p);
	writeFileSync(f.out, "record");
	assert.equal(readPath(f.scope, f.run, f.out), f.out);
	// the record swapped for a link to a file it may not touch: neither written nor read
	const g = fixture();
	symlinkSync(g.journal, g.out);
	assert.equal(writePath(g.scope, g.run, g.out), undefined);
	assert.equal(readPath(g.scope, g.run, g.out), g.journal);   // (it is the journal, which it may read)
});

test("no scope, no reader: a missing or bad scope refuses to load", () => {
	const f = fixture();
	for (const s of [undefined, "", "{}", "not json", JSON.stringify({ read: [f.log] }),
		JSON.stringify({ read: [join(f.top, "missing")], write: f.out }),       // evidence that isn't there
		JSON.stringify({ read: [f.log], write: "p1_record.md" }),               // a write path must be absolute
		JSON.stringify({ read: [f.log], write: join(f.top, "missing", "r.md") }), // its folder must exist
		JSON.stringify({ read: [f.log], write: f.log })])                       // evidence isn't writable
		assert.throws(() => readerScope(s), undefined, String(s));
});

test("a run folder reached through a link (as macOS's temp folder is) reads its files and writes its record", () => {
	const f = fixture();
	const via = join(realpathSync(mkdtempSync(join(tmpdir(), "reader-via-"))), "top");
	symlinkSync(f.top, via);                                        // every path below is spelled through the link
	const run = join(via, "run");
	const log = join(via, "sessions", "s1.jsonl");
	const journal = join(via, "paint-studio-1", "notes", "journal.md");
	const brief = join(run, "p1_reader_brief.md");
	const out = join(run, "p1_record.md");
	const scope = readerScope(JSON.stringify({ read: [log, journal, brief], write: out }));
	for (const cwd of [run, f.run]) {
		for (const p of [log, journal, brief, f.log, "p1_reader_brief.md"])
			assert.ok(readPath(scope, cwd, p), `${cwd}: ${p}`);
		for (const p of [out, f.out, "p1_record.md"])
			assert.equal(writePath(scope, cwd, p), f.out, `${cwd}: ${p}`);
		for (const p of [join(via, "secret"), "escape", join(via, "paint-studio-2", "BRIEF.md")])
			assert.equal(readPath(scope, cwd, p), undefined, `${cwd}: ${p}`);
	}
	writeFileSync(out, "record");
	assert.equal(readPath(scope, run, "p1_record.md"), f.out);
});
