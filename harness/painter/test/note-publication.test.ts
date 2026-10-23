import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { appendFileSync, chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, readlinkSync, realpathSync, renameSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { JOURNAL, REVISIONS, reviseJournal } from "../journal.ts";
import { NOTEBOOK, NOTEBOOK_REVISIONS, writeNotebook } from "../notebook.ts";
import { criticReadable, leaveNote, visit } from "../visit.ts";

function fixture() {
	const dir = mkdtempSync(join(tmpdir(), "note-publication-"));
	const studio = join(dir, "studio");
	const critic = { args: [], dir: join(dir, "critic") };
	mkdirSync(join(studio, "notes"), { recursive: true });
	mkdirSync(join(studio, "out/easel"), { recursive: true });
	assert.ok(process.env.EASEL_BIN, "set EASEL_BIN to the easel");
	mkdirSync(join(studio, "bin"));
	symlinkSync(process.env.EASEL_BIN, join(studio, "bin", "easel"));
	mkdirSync(critic.dir);
	const pi = join(dir, "pi");
	writeFileSync(pi, `#!${process.execPath}\nconsole.log('new critique');\n`, { mode: 0o755 });
	return { dir, studio, critic, pi };
}

const writers = [
	{ name: "journal", current: JOURNAL, history: REVISIONS, save: (s: string) => reviseJournal(s, "old passage", "new passage") },
	{ name: "notebook", current: NOTEBOOK, history: NOTEBOOK_REVISIONS, save: (s: string) => writeNotebook(s, "new passage", "old passage") },
];

for (const writer of writers) {
	test(`${writer.name}: a read-only current file or invalid history leaves both unchanged`, () => {
		const f = fixture();
		const current = join(f.studio, writer.current);
		const history = join(f.studio, writer.history);
		writeFileSync(current, "old passage\n");
		chmodSync(current, 0o444);
		assert.throws(() => writer.save(f.studio), { code: "EACCES" });
		assert.equal(readFileSync(current, "utf8"), "old passage\n");
		assert.ok(!existsSync(history));
		chmodSync(current, 0o644);
		mkdirSync(history);
		assert.throws(() => writer.save(f.studio), { code: "EISDIR" });
		assert.equal(readFileSync(current, "utf8"), "old passage\n");
		assert.ok(lstatSync(history).isDirectory());
	});

	for (const alias of ["current", "parent", "dangling", "history", "history-parent", "backing", "publication-pointer"]) {
		test(`${writer.name}: rejects an escaping ${alias} alias without touching either side`, () => {
			const f = fixture();
			const outside = join(f.dir, "outside");
			mkdirSync(outside);
			writeFileSync(join(outside, "target"), "old passage\n");
			const current = join(f.studio, writer.current);
			if (alias === "current" || alias === "dangling") {
				symlinkSync(join(outside, alias === "dangling" ? "missing" : "target"), current);
			} else if (alias === "parent") {
				const parent = writer.name === "journal" ? "notes" : "alias";
				if (writer.name === "journal") {
					const nested = fixture();
					const nestedStudio = join(nested.dir, "alias-studio");
					mkdirSync(nestedStudio);
					symlinkSync(outside, join(nestedStudio, parent));
					assert.throws(() => writer.save(nestedStudio), /your journal is outside the studio, so nothing was written/);
					return;
				}
				symlinkSync(outside, join(f.studio, parent));
				symlinkSync(join(f.studio, parent, "target"), current);
			} else {
				writeFileSync(current, "old passage\n");
				if (alias === "history") symlinkSync(join(outside, "target"), join(f.studio, writer.history));
				else if (alias === "history-parent") {
					const other = join(f.dir, "history-studio");
					mkdirSync(join(other, "notes"), { recursive: true });
					writeFileSync(join(other, writer.current), "old passage\n");
					symlinkSync(outside, join(other, "out"));
					assert.throws(() => writer.save(other), /outside the studio, so nothing was written/);
					assert.equal(readFileSync(join(outside, "target"), "utf8"), "old passage\n");
					return;
				} else if (alias === "backing") symlinkSync(outside, join(f.studio, "out/easel/.note-publications"));
				else {
					const base = join(f.studio, "out/easel/.note-publications", writer.name);
					mkdirSync(base, { recursive: true });
					symlinkSync(outside, join(base, "current"));
				}
			}
			assert.throws(() => writer.save(f.studio), /outside the studio, so nothing was written/);
			assert.equal(readFileSync(join(outside, "target"), "utf8"), "old passage\n");
			assert.ok(!existsSync(join(outside, "missing")));
			assert.deepEqual(readdirSync(outside), ["target"]);
			if (alias !== "current" && alias !== "dangling" && alias !== "parent") assert.equal(readFileSync(current, "utf8"), "old passage\n");
		});
	}

	test(`${writer.name}: internal aliases and subsequent revisions retain the public paths and history`, () => {
		const f = fixture();
		const target = join(f.studio, "internal.md");
		writeFileSync(target, "old passage\n");
		symlinkSync(target, join(f.studio, writer.current));
		writer.save(f.studio);
		if (writer.name === "journal") reviseJournal(f.studio, "new passage", "third passage");
		else writeNotebook(f.studio, "third passage", "new passage");
		assert.equal(readFileSync(target, "utf8"), "third passage\n");
		assert.equal(readFileSync(join(f.studio, writer.current), "utf8"), "third passage\n");
		const history = readFileSync(join(f.studio, writer.history), "utf8").trim().split("\n").map((line) => JSON.parse(line));
		assert.deepEqual(history.map((entry) => entry.before), ["old passage\n", "new passage\n"]);
		if (writer.name === "journal") assert.equal(criticReadable(f.studio, JOURNAL), realpathSync(target));
	});

	test(`${writer.name}: excessive aliases fail closed before an outside destination`, () => {
		const f = fixture();
		const outside = join(f.dir, "outside.md");
		writeFileSync(outside, "old passage\n");
		for (let i = 0; i < 45; i++) symlinkSync(i === 44 ? outside : `link-${i + 1}`, join(f.studio, `link-${i}`));
		symlinkSync(join(f.studio, "link-0"), join(f.studio, writer.current));
		assert.throws(() => writer.save(f.studio), { code: "ELOOP" });
		assert.equal(readFileSync(outside, "utf8"), "old passage\n");
		assert.ok(!existsSync(join(f.studio, writer.history)));
	});

	test(`${writer.name}: the studio itself may be reached through an alias`, () => {
		const f = fixture();
		const alias = join(f.dir, "studio-alias");
		symlinkSync(f.studio, alias);
		writeFileSync(join(f.studio, writer.current), "old passage\n");
		writer.save(alias);
		assert.equal(readFileSync(join(f.studio, writer.current), "utf8"), "new passage\n");
	});

	test(`${writer.name}: relative publication links survive moving the studio and another revision`, () => {
		const f = fixture();
		writeFileSync(join(f.studio, writer.current), "old passage\n");
		writer.save(f.studio);
		for (const path of [writer.current, writer.history, `out/easel/.note-publications/${writer.name}/current`]) assert.ok(!readlinkSync(join(f.studio, path)).startsWith("/"));
		const moved = join(f.dir, "moved-studio");
		renameSync(f.studio, moved);
		assert.equal(readFileSync(join(moved, writer.current), "utf8"), "new passage\n");
		if (writer.name === "journal") {
			appendFileSync(join(moved, JOURNAL), "- day 2, 10:00: native journal append\n");
			reviseJournal(moved, "new passage", "third passage");
			assert.equal(readFileSync(join(moved, JOURNAL), "utf8"), "third passage\n- day 2, 10:00: native journal append\n");
		} else writeNotebook(moved, "third passage", "new passage");
		const history = readFileSync(join(moved, writer.history), "utf8").trim().split("\n").map((line) => JSON.parse(line));
		assert.equal(history.length, 2);
		assert.equal(history[1].before, writer.name === "journal" ? "new passage\n- day 2, 10:00: native journal append\n" : "new passage\n");
	});
}

test("critic request and completed-visit archive failures preserve the previous current notes", async () => {
	const f = fixture();
	writeFileSync(join(f.studio, "notes/to_critic.md"), "old note\n");
	mkdirSync(join(f.critic.dir, "notes-to-critic.jsonl"));
	assert.throws(() => leaveNote(f.studio, f.critic, "new note"), { code: "EISDIR" });
	assert.equal(readFileSync(join(f.studio, "notes/to_critic.md"), "utf8"), "old note\n");
	writeFileSync(join(f.studio, "notes/critic.md"), "old critique\n");
	writeFileSync(f.pi, `#!${process.execPath}
const fs = require('node:fs'), path = require('node:path');
const sessions = process.argv[process.argv.indexOf('--session-dir') + 1];
fs.mkdirSync(path.join(path.dirname(path.dirname(sessions)), path.basename(sessions) + '-asked.md'));
console.log('new critique');
`, { mode: 0o755 });
	await assert.rejects(visit(f.studio, f.critic, "asked", undefined, f.pi), { code: "EISDIR" });
	assert.equal(readFileSync(join(f.studio, "notes/critic.md"), "utf8"), "old critique\n");
});

test("critic notes remain readable through their public paths across multiple publications", async () => {
	const f = fixture();
	leaveNote(f.studio, f.critic, "first");
	leaveNote(f.studio, f.critic, "second");
	assert.ok(lstatSync(join(f.critic.dir, "notes-to-critic.jsonl")).isFile());
	assert.equal(criticReadable(f.studio, "notes/to_critic.md"), realpathSync(join(f.studio, "notes/to_critic.md")));
	assert.deepEqual(readFileSync(join(f.critic.dir, "notes-to-critic.jsonl"), "utf8").trim().split("\n").map((line) => JSON.parse(line).note), ["first", "second"]);
	const first = await visit(f.studio, f.critic, "asked", undefined, f.pi);
	const archive = readdirSync(f.critic.dir).find((name) => name.endsWith("-asked.md"))!;
	const second = await visit(f.studio, f.critic, "sitting", undefined, f.pi);
	await visit(f.studio, f.critic, "asked", undefined, f.pi);
	assert.ok(lstatSync(join(f.critic.dir, archive)).isFile());
	assert.equal(readFileSync(join(f.critic.dir, archive), "utf8"), first);
	assert.equal(readFileSync(join(f.studio, "notes/critic.md"), "utf8"), first);
	assert.equal(readFileSync(join(f.critic.dir, readdirSync(f.critic.dir).find((name) => name.endsWith("-sitting.md"))!), "utf8"), second);
	renameSync(f.studio, join(f.dir, "moved-studio"));
	assert.equal(readFileSync(join(f.critic.dir, archive), "utf8"), first);
	assert.equal(readFileSync(join(f.critic.dir, "notes-to-critic.jsonl"), "utf8").trim().split("\n").length, 2);
});

test("successful publications retain only two committed generations and leave another writer's staging alone", () => {
	const f = fixture();
	writeNotebook(f.studio, "entry 0");
	const base = join(f.studio, "out/easel/.note-publications/notebook");
	const active = mkdtempSync(join(base, "generation-"));
	writeFileSync(join(active, "other-writer"), "active staging");
	for (let i = 1; i <= 12; i++) writeNotebook(f.studio, `entry ${i}`);
	const current = realpathSync(join(base, "current"));
	const previous = realpathSync(join(current, ".previous"));
	assert.notEqual(previous, current);
	assert.equal(readdirSync(base).filter((name) => name.startsWith("generation-")).length, 3);
	assert.equal(readFileSync(join(active, "other-writer"), "utf8"), "active staging");
	assert.equal(readFileSync(join(f.studio, NOTEBOOK_REVISIONS), "utf8").trim().split("\n").length, 13);
	assert.ok(!readFileSync(join(previous, "current"), "utf8").includes("entry 12"));
	assert.ok(readFileSync(join(f.studio, NOTEBOOK), "utf8").includes("entry 12"));
});

for (const boundary of ["fail", "kill"]) {
	test(`journal: native appends survive a retry that ${boundary === "fail" ? "fails" : "is killed"} after an interrupted bootstrap`, () => {
		const f = fixture();
		const journal = join(f.studio, JOURNAL);
		const history = join(f.studio, REVISIONS);
		writeFileSync(journal, "old passage\n");
		const interrupt = (bootstrap: boolean) => spawnSync(process.execPath, ["--input-type=module", "-e", `
import fs from 'node:fs';
import {syncBuiltinESMExports} from 'node:module';
const stop = () => {
 if (${JSON.stringify(boundary)} === 'kill') process.kill(process.pid, 'SIGKILL');
 throw Object.assign(new Error('injected storage failure'), {code:'ENOSPC'});
};
const mkdtemp = fs.mkdtempSync;
fs.mkdtempSync = (...args) => {
 if (${bootstrap} && fs.existsSync(${JSON.stringify(join(f.studio, "out/easel/.note-publications/journal/current"))})) stop();
 return mkdtemp(...args);
};
const rename = fs.renameSync;
fs.renameSync = (from, to) => {
 if (!${bootstrap} && String(from).endsWith('/next') && String(to).endsWith('/journal/current')) stop();
 return rename(from, to);
};
syncBuiltinESMExports();
const {reviseJournal} = await import(${JSON.stringify(new URL("../journal.ts", import.meta.url).href)});
try {
 reviseJournal(${JSON.stringify(f.studio)}, 'old passage', 'new passage');
 process.exitCode = 99;
} catch (error) {
 console.log(error.code);
}
`], { encoding: "utf8" });
		for (const bootstrap of [true, false]) {
			const child = interrupt(bootstrap);
			if (boundary === "kill") assert.equal(child.signal, "SIGKILL", child.stderr);
			else {
				assert.equal(child.status, 0, child.stderr);
				assert.match(child.stdout, /ENOSPC/);
			}
			if (bootstrap) {
				assert.ok(lstatSync(journal).isFile());
				assert.ok(existsSync(join(f.studio, "out/easel/.note-publications/journal/current")));
				appendFileSync(journal, "- day 2, 10:00: native journal append\n");
			}
			assert.equal(readFileSync(journal, "utf8"), "old passage\n- day 2, 10:00: native journal append\n");
			assert.ok(!existsSync(history));
		}
		reviseJournal(f.studio, "old passage", "new passage");
		assert.equal(readFileSync(journal, "utf8"), "new passage\n- day 2, 10:00: native journal append\n");
		const records = readFileSync(history, "utf8").trim().split("\n").map((line) => JSON.parse(line));
		assert.equal(records.length, 1);
		assert.equal(records[0].before, "old passage\n- day 2, 10:00: native journal append\n");
	});
}

test("independent critic archive copies preserve the destination's permissions", () => {
	for (const [mask, mode] of [[0o022, 0o600], [0o077, 0o640]]) {
		const f = fixture();
		const history = join(f.critic.dir, "notes-to-critic.jsonl");
		writeFileSync(history, JSON.stringify({ note: "original" }) + "\n");
		chmodSync(history, mode);
		const previousMask = process.umask(mask);
		try {
			for (const note of ["first", "second"]) {
				leaveNote(f.studio, f.critic, note);
				assert.ok(lstatSync(history).isFile());
				assert.equal(lstatSync(history).mode & 0o777, mode);
			}
		} finally {
			process.umask(previousMask);
		}
		assert.deepEqual(readFileSync(history, "utf8").trim().split("\n").map((line) => JSON.parse(line).note), ["original", "first", "second"]);
	}
});

for (const kind of ["journal", "notebook", "to-critic", "critic"]) {
	for (const boundary of ["before", "after", "fail", "abort"]) {
		if (boundary === "abort" && kind !== "critic") continue;
		test(`${kind}: ${boundary === "fail" ? "a failed" : boundary === "abort" ? "an aborted" : `SIGKILL ${boundary}`} publication never exposes contradictory readable notes`, () => {
			const f = fixture();
			const current = kind === "journal" ? JOURNAL : kind === "notebook" ? NOTEBOOK : kind === "to-critic" ? "notes/to_critic.md" : "notes/critic.md";
			const history = kind === "journal" ? join(f.studio, REVISIONS) : kind === "notebook" ? join(f.studio, NOTEBOOK_REVISIONS) : join(f.critic.dir, "notes-to-critic.jsonl");
			writeFileSync(join(f.studio, current), "old passage\n");
			if (kind !== "critic") writeFileSync(history, "{\"before\":\"original\",\"with\":\"old passage\"}\n");
			const child = spawnSync(process.execPath, ["--input-type=module", "-e", `
import fs from 'node:fs';
import {syncBuiltinESMExports} from 'node:module';
const controller = new AbortController();
const rename = fs.renameSync;
const symlink = fs.symlinkSync;
fs.renameSync = (from, to) => {
 if (String(from).endsWith('/next') && String(to).endsWith('/${kind}/current')) {
  if (${JSON.stringify(boundary)} === 'before') process.kill(process.pid, 'SIGKILL');
  if (${JSON.stringify(boundary)} === 'fail') throw Object.assign(new Error('injected storage failure'), {code:'ENOSPC'});
  rename(from, to);
  if (${JSON.stringify(boundary)} === 'after') process.kill(process.pid, 'SIGKILL');
  return;
 }
 return rename(from, to);
};
fs.symlinkSync = (...args) => {
 const result = symlink(...args);
 if (${JSON.stringify(boundary)} === 'abort' && String(args[1]).endsWith('/next')) controller.abort();
 return result;
};
syncBuiltinESMExports();
const {reviseJournal} = await import(${JSON.stringify(new URL("../journal.ts", import.meta.url).href)});
const {writeNotebook} = await import(${JSON.stringify(new URL("../notebook.ts", import.meta.url).href)});
const {leaveNote, visit} = await import(${JSON.stringify(new URL("../visit.ts", import.meta.url).href)});
try {
 if (${JSON.stringify(kind)} === 'journal') reviseJournal(${JSON.stringify(f.studio)}, 'old passage', 'new passage');
 else if (${JSON.stringify(kind)} === 'notebook') writeNotebook(${JSON.stringify(f.studio)}, 'new passage', 'old passage');
 else if (${JSON.stringify(kind)} === 'to-critic') leaveNote(${JSON.stringify(f.studio)}, ${JSON.stringify(f.critic)}, 'new passage');
 else await visit(${JSON.stringify(f.studio)}, ${JSON.stringify(f.critic)}, 'asked', controller.signal, ${JSON.stringify(f.pi)});
 process.exitCode = 99;
} catch (error) {
 console.log(error.code ?? error.message);
}
`], { encoding: "utf8" });
			if (boundary === "before" || boundary === "after") assert.equal(child.signal, "SIGKILL", child.stderr);
			else {
				assert.equal(child.status, 0, child.stderr);
				assert.match(child.stdout, boundary === "fail" ? /ENOSPC/ : /Operation aborted/);
			}
			const published = boundary === "after";
			const text = readFileSync(join(f.studio, current), "utf8");
			assert.equal(text, published ? kind === "critic" ? "From the critic's visit you asked for:\n\nnew critique\n" : "new passage\n" : "old passage\n");
			if (kind === "critic") {
				const archives = readdirSync(f.critic.dir).filter((name) => name.endsWith("-asked.md") && existsSync(join(f.critic.dir, name)));
				assert.equal(archives.length, published ? 1 : 0);
				for (const name of archives) assert.equal(readFileSync(join(f.critic.dir, name), "utf8"), text);
				if (boundary === "fail" || boundary === "abort") assert.ok(!readdirSync(f.critic.dir).some((name) => name.endsWith("-asked.md")));
			} else {
				const records = readFileSync(history, "utf8").trim().split("\n").map((line) => JSON.parse(line));
				assert.equal(records.length, published ? 2 : 1);
				if (published) assert.equal(kind === "to-critic" ? records[1].note : records[1].with, "new passage");
			}
			if (boundary === "fail" || boundary === "abort") {
				assert.ok(!readdirSync(f.critic.dir).some((name) => name.startsWith(".note-copy-")));
				const base = join(f.studio, "out/easel/.note-publications", kind);
				assert.ok(!existsSync(join(base, "next")));
				assert.equal(readdirSync(base).filter((name) => name.startsWith("generation-")).length, 1);
			}
		});
	}
}
