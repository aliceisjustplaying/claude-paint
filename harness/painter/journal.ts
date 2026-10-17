/**
 * Revising the journal (round 21: painters may edit their notes; whether they use it to get
 * around the canvas's no-undo is something to watch). `note` with `replaces` swaps one passage
 * of notes/journal.md for new text, like an edit tool: the passage must appear exactly once.
 * Every revision is kept, whole, in out/easel/journal-revisions.jsonl (time, the passage, its
 * replacement and the journal before), so the record of what the painter first wrote survives.
 */
import { appendFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

export const JOURNAL = "notes/journal.md";
export const REVISIONS = "out/easel/journal-revisions.jsonl";

/** Replace the one occurrence of `old` in the studio's journal with `text`; the tool's reply. Throws a reply for the painter on a bad request. */
export function reviseJournal(studio: string, old: string, text: string, now: Date = new Date()): string {
	const path = join(studio, JOURNAL);
	if (!old.trim()) throw new Error("note: `replaces` is empty; give the exact text of the passage to replace");
	if (!existsSync(path)) throw new Error(`note: ${JOURNAL} has no entries yet`);
	const before = readFileSync(path, "utf8");
	const at = before.indexOf(old);
	if (at < 0) throw new Error(`note: the passage given in \`replaces\` isn't in ${JOURNAL} (it has to match exactly)`);
	if (before.indexOf(old, at + 1) >= 0) throw new Error(`note: the passage given in \`replaces\` is in ${JOURNAL} more than once; give more of it`);
	const after = before.slice(0, at) + text + before.slice(at + old.length);
	const log = join(studio, REVISIONS);
	mkdirSync(dirname(log), { recursive: true });
	appendFileSync(log, JSON.stringify({ at: now.toISOString(), replaced: old, with: text, before }) + "\n");
	writeFileSync(path, after);
	return `revised in ${path}\n`;
}
/** The line a handoff's list of open passages starts with (the runner's first-sitting message names it). */
export const OPEN_PASSAGES = "Open passages:";

const STAMP = /^- day \d+, (?:[01]\d|2[0-3]):[0-5]\d: ?/;
const HEADING = /^[#*_\s]*open passages\s*[*_]*:/i;
const ITEM = /^(\s*)(?:[-*•]|\d+[.)])\s+/;

/**
 * The journal's latest list of open passages: the last line reading `Open passages:` (in a stamped
 * entry or on its own, with or without Markdown emphasis) and the list items under it, dedented and
 * otherwise as the painter wrote them; undefined when the journal has no such line.
 */
export function openPassages(journal: string): string | undefined {
	const lines = journal.split(/\r\n|\r|\n/);
	let at = -1;
	for (let i = 0; i < lines.length; i++) if (HEADING.test(lines[i].replace(STAMP, ""))) at = i;
	if (at < 0) return undefined;
	const block = [lines[at].replace(STAMP, "").trim()];
	let indent: number | undefined;
	let blanks = 0;
	for (const line of lines.slice(at + 1)) {
		if (STAMP.test(line)) break;
		if (!line.trim()) {
			blanks++;
			continue;
		}
		const item = ITEM.exec(line);
		const depth = line.length - line.trimStart().length;
		if (item && (indent === undefined || depth >= indent)) indent ??= depth;
		else if (indent === undefined || depth <= indent) break;
		for (; blanks > 0; blanks--) block.push("");
		block.push(line.slice(Math.min(depth, indent!)));
	}
	return block.join("\n");
}
