/**
 * The atelier painter's notebook (atelier/): notebook.md in the studio, which the atelier carries from one
 * painting to the next. `notebook` appends an entry after a blank line, with no stamp, or with `replaces` swaps
 * one passage that appears exactly once, as `note` does for the journal. Every call is kept, whole, in
 * out/easel/notebook-revisions.jsonl (time, what was added or replaced and with what, and the notebook before).
 */
import { appendFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";

export const NOTEBOOK = "notebook.md";
export const NOTEBOOK_REVISIONS = "out/easel/notebook-revisions.jsonl";

/** Add `text` to the studio's notebook, or put it in place of the one occurrence of `replaces`; the tool's reply. Throws a reply for the painter on a bad request. */
export function writeNotebook(studio: string, text: string, replaces?: string, now: Date = new Date()): string {
	const path = join(studio, NOTEBOOK);
	const before = existsSync(path) ? readFileSync(path, "utf8") : "";
	let after: string;
	let record: Record<string, string>;
	if (replaces === undefined) {
		const entry = text.endsWith("\n") ? text : `${text}\n`;
		after = before === "" ? entry : before + (before.endsWith("\n") ? "\n" : "\n\n") + entry;
		record = { at: now.toISOString(), added: text, before };
	} else {
		if (!replaces.trim()) throw new Error("notebook: `replaces` is empty; give the exact text of the passage to replace");
		if (!before) throw new Error(`notebook: ${NOTEBOOK} has no entries yet`);
		const at = before.indexOf(replaces);
		if (at < 0) throw new Error(`notebook: the passage given in \`replaces\` isn't in ${NOTEBOOK} (it has to match exactly)`);
		if (before.indexOf(replaces, at + 1) >= 0) throw new Error(`notebook: the passage given in \`replaces\` is in ${NOTEBOOK} more than once; give more of it`);
		after = before.slice(0, at) + text + before.slice(at + replaces.length);
		record = { at: now.toISOString(), replaced: replaces, with: text, before };
	}
	const log = join(studio, NOTEBOOK_REVISIONS);
	mkdirSync(dirname(log), { recursive: true });
	appendFileSync(log, JSON.stringify(record) + "\n");
	writeFileSync(path, after);
	return replaces === undefined ? `noted in ${path}\n` : `revised in ${path}\n`;
}
