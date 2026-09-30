/**
 * The painter's compaction summary (compaction.ts), without pi imports so node --test can load it.
 * See compaction.ts for what it holds.
 */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { atEasel } from "./easel-client.ts";

/** A statement, not an instruction: the sections after it say what they hold. */
export const HEADER = "Earlier parts of this session were condensed.";

const MAX_BRIEF_CHARS = 60_000;
const MAX_JOURNAL_CHARS = 120_000;
const MAX_GLOBALS = 150;

type ReadResult = { text: string; note?: string };

function readText(path: string): ReadResult | undefined {
	if (!existsSync(path)) return undefined;
	try {
		return { text: readFileSync(path, "utf8") };
	} catch (error) {
		return { text: "", note: `could not read: ${error instanceof Error ? error.message : String(error)}` };
	}
}

/** Keep the end of a long text; say where the whole of it is. */
function tail(text: string, max: number, file: string): string {
	if (text.length <= max) return text;
	const cut = text.slice(text.length - max);
	const start = cut.indexOf("\n") + 1;
	return `(the beginning is in ${file})\n…\n${cut.slice(start)}`;
}

/** Chunks in the log, by their `--@ chunk N` lines. */
function countChunks(source: string): { chunks: number; lastChunk?: number } {
	let chunks = 0;
	let lastChunk: number | undefined;
	for (const line of source.split("\n")) {
		const marker = /^--@ chunk (\d+)/.exec(line);
		if (marker) {
			chunks += 1;
			lastChunk = Number(marker[1]);
		}
	}
	return { chunks, lastChunk };
}

/**
 * The painting's globals as the easel's live Lua state holds them (`easel globals`: one line
 * each, `<chunk>\t<name>\t<what it holds>`, the most recently assigned last).
 */
async function globals(studio: string, signal?: AbortSignal): Promise<{ lines: string[]; total: number } | { error: string }> {
	let out: string;
	try {
		out = await atEasel(studio, ["globals"], undefined, signal);
	} catch (error) {
		return { error: error instanceof Error ? error.message : String(error) };
	}
	const rows = out === "" ? [] : out.split("\n").map((l) => /^(\d+)\t([A-Za-z_]\w*)\t(.*)$/.exec(l));
	if (rows.some((r) => !r)) return { error: `the easel's answer isn't a list of globals: ${out.slice(0, 200)}` };
	const lines = rows.map((r) => `- \`${r![2]}\` (chunk ${r![1]}): ${r![3]}`);
	return { lines: lines.slice(Math.max(0, lines.length - MAX_GLOBALS)), total: lines.length };
}

const CLOCK_LINE = /^day \d+, ([01]\d|2[0-3]):[0-5]\d$/;
const JOURNAL_STAMP = /^- (day \d+, (?:[01]\d|2[0-3]):[0-5]\d):/;

/**
 * The latest painting time printed on a line of its own in a paint result (what `wait()`
 * returns). Only paint calls that succeeded count: a file the painter read, a note or a failed
 * chunk can hold any text.
 */
export function latestClockInMessages(entries: readonly unknown[]): string | undefined {
	for (let i = entries.length - 1; i >= 0; i--) {
		const entry = entries[i] as { type?: string; message?: { role?: string; toolName?: string; isError?: boolean; content?: unknown } };
		if (entry?.type !== "message" || entry.message?.role !== "toolResult") continue;
		if (entry.message.toolName !== "paint" || entry.message.isError !== false) continue;
		const content = Array.isArray(entry.message.content) ? entry.message.content : [];
		const text = content
			.map((c: { type?: string; text?: string }) => (c?.type === "text" ? (c.text ?? "") : ""))
			.join("\n");
		const lines = text.split("\n").map((l) => l.trim());
		for (let j = lines.length - 1; j >= 0; j--) if (CLOCK_LINE.test(lines[j])) return lines[j];
	}
	return undefined;
}

export function latestJournalStamp(journal: string): string | undefined {
	let stamp: string | undefined;
	for (const line of journal.split("\n")) {
		const m = JOURNAL_STAMP.exec(line);
		if (m) stamp = m[1];
	}
	return stamp;
}

/** `cwd` is the studio. */
export async function buildSummary(cwd: string, entries: readonly unknown[], signal?: AbortSignal) {
	const parts: string[] = [HEADER];
	const facts: Record<string, unknown> = {};

	const brief = readText(join(cwd, "BRIEF.md"));
	parts.push("## BRIEF.md");
	if (!brief) parts.push("(no BRIEF.md in the studio)");
	else parts.push(brief.note ? `(${brief.note})` : tail(brief.text.trimEnd(), MAX_BRIEF_CHARS, "BRIEF.md"));

	const journal = readText(join(cwd, "notes", "journal.md"));
	parts.push("## Your journal (notes/journal.md)");
	if (!journal || (!journal.note && !journal.text.trim())) parts.push("(no entries yet)");
	else parts.push(journal.note ? `(${journal.note})` : tail(journal.text.trimEnd(), MAX_JOURNAL_CHARS, "notes/journal.md"));

	parts.push("## Your globals");
	const g = await globals(cwd, signal);
	if ("error" in g) {
		parts.push(`(the easel couldn't list them: ${g.error})`);
		facts.globalsError = g.error;
	}
	else if (g.total === 0) parts.push("(none yet)");
	else {
		if (g.total > g.lines.length) g.lines.unshift(`(the ${g.lines.length} most recently assigned of ${g.total})`);
		parts.push(g.lines.join("\n"));
		facts.globals = g.total;
	}

	const log = readText(join(cwd, "paintings", "lua", "painting.lua"));
	const logFacts = log && !log.note ? countChunks(log.text) : undefined;
	if (logFacts) {
		facts.chunks = logFacts.chunks;
		facts.lastChunk = logFacts.lastChunk;
	}

	const clock = latestClockInMessages(entries);
	const stamp = journal && !journal.note ? latestJournalStamp(journal.text) : undefined;
	facts.clock = clock;
	facts.journalStamp = stamp;
	const clockLines = [
		`- Chunks in the log: ${logFacts ? `${logFacts.chunks}${logFacts.lastChunk !== undefined ? ` (last: chunk ${logFacts.lastChunk})` : ""}` : "0"}`,
	];
	if (clock) clockLines.push(`- Latest painting time the easel printed: ${clock}`);
	if (stamp) clockLines.push(`- Latest journal entry stamped: ${stamp}`);
	parts.push("## The canvas clock");
	parts.push(clockLines.join("\n"));

	return { summary: parts.join("\n\n"), facts };
}
