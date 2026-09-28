/**
 * Painter compaction: a deterministic summary, no model call.
 *
 * Load with `pi --no-extensions -e harness/painter/compaction.ts ...`. `--no-extensions`
 * keeps every other compaction handler (pi-anthropic-compat's native Anthropic compaction,
 * pi-codex-compaction) from loading. This handler always returns a summary, even if a file
 * can't be read, so pi's built-in LLM summarizer (Goal / Progress / Next Steps) never runs.
 *
 * The summary holds only material from the studio, not a retelling of the conversation:
 *   - BRIEF.md, verbatim
 *   - notes/journal.md, verbatim (the painter's own words)
 *   - the top-level Lua globals the log defines (paintings/lua/painting.lua), so the
 *     painter's names keep working: name, chunk and defining line, bounded
 *   - the canvas clock: chunks in the log, the latest painting time seen in the easel's replies
 * It deliberately has no next steps, remaining tasks or progress checklist.
 *
 * It also sets when compaction happens (COMPACTION below), so no settings file has to sit in
 * the studio where the painter would read it. Pi has no extension API for settings, and no
 * CLI flag or environment variable for a settings file apart from moving the whole agent
 * directory (which moves auth.json and its OAuth refresh lock). So this module overrides the
 * compaction getters of pi's SettingsManager in this process: pi's own threshold check, cut
 * point and overflow recovery then run with these values, whatever the global
 * ~/.pi/agent/settings.json or a studio .pi/settings.json says. Each compaction entry's
 * `details.settings` records the values pi used, and `details.harnessSettings` whether they
 * were these.
 */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { type ExtensionAPI, SettingsManager } from "@earendil-works/pi-coding-agent";

export const DETAILS_TYPE = "claude-paint-painter-compaction";

/**
 * Compact when the context passes the window minus reserveTokens (about 900K of both lanes'
 * 1M windows), keeping the last keepRecentTokens verbatim (pi's default). Round 16's point
 * for Opus; see README "Compaction".
 */
export const COMPACTION = { enabled: true, reserveTokens: reserveFromEnv(), keepRecentTokens: 20_000 } as const;

/**
 * PAINTER_COMPACT_RESERVE (tokens) moves the point: compaction then comes at the window minus
 * this. For a painter whose window is smaller than 1M, or to make compaction happen early in a
 * test run (none of round 19's sittings came near 900K: the largest was 328K). Read once and
 * removed from the environment, like painter.ts's PAINTER_* variables.
 */
function reserveFromEnv(): number {
	const v = process.env.PAINTER_COMPACT_RESERVE;
	delete process.env.PAINTER_COMPACT_RESERVE;
	if (v === undefined) return 100_000;
	const n = Number(v);
	if (!(Number.isInteger(n) && n > 0)) throw new Error(`PAINTER_COMPACT_RESERVE=${v}: want a number of tokens`);
	return n;
}

/** Make pi's settings answer COMPACTION for every model. Returns false if pi's SettingsManager has changed shape. */
export function applyCompactionSettings(proto: Record<string, unknown> = SettingsManager.prototype as never): boolean {
	const names = ["getCompactionSettings", "getCompactionEnabled", "getCompactionReserveTokens", "getCompactionKeepRecentTokens"];
	if (!names.every((n) => typeof proto[n] === "function")) return false;
	proto.getCompactionSettings = () => ({ ...COMPACTION });
	proto.getCompactionEnabled = () => COMPACTION.enabled;
	proto.getCompactionReserveTokens = () => COMPACTION.reserveTokens;
	proto.getCompactionKeepRecentTokens = () => COMPACTION.keepRecentTokens;
	return true;
}

function sameSettings(s: unknown): boolean {
	const t = s as Partial<typeof COMPACTION> | undefined;
	return t?.enabled === COMPACTION.enabled && t?.reserveTokens === COMPACTION.reserveTokens && t?.keepRecentTokens === COMPACTION.keepRecentTokens;
}

/** A statement, not an instruction: the sections after it say what they hold. */
const HEADER = "Earlier parts of this session were condensed.";

const MAX_BRIEF_CHARS = 60_000;
const MAX_JOURNAL_CHARS = 120_000;
const MAX_GLOBALS = 150;
const MAX_LINE_CHARS = 160;

const LUA_KEYWORDS = new Set([
	"and", "break", "do", "else", "elseif", "end", "false", "for", "function", "global", "goto", "if",
	"in", "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
]);

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

function oneLine(line: string): string {
	const t = line.trim();
	return t.length > MAX_LINE_CHARS ? `${t.slice(0, MAX_LINE_CHARS)} …` : t;
}

export interface LogFacts {
	chunks: number;
	lastChunk?: number;
	globals: { name: string; chunk: number; line: string }[];
	totalGlobals: number;
}

/**
 * Top-level globals of the log. A chunk is its own Lua chunk, so a line at column 0 that
 * assigns a bare name (not `local`, not a field) or declares `function name(...)` defines
 * a global that later chunks can use. Indented lines (function bodies, loops) are skipped.
 */
export function scanLog(source: string): LogFacts {
	const defs = new Map<string, { name: string; chunk: number; line: string }>();
	let chunk = 0;
	let chunks = 0;
	let lastChunk: number | undefined;
	for (const line of source.split("\n")) {
		const marker = /^--@ chunk (\d+)/.exec(line);
		if (marker) {
			chunk = Number(marker[1]);
			chunks += 1;
			lastChunk = chunk;
			continue;
		}
		let names: string[] = [];
		const fn = /^function\s+([A-Za-z_]\w*)\s*\(/.exec(line);
		const assign = /^([A-Za-z_]\w*(?:\s*,\s*[A-Za-z_]\w*)*)\s*=(?!=)/.exec(line);
		if (fn) names = [fn[1]];
		else if (assign) names = assign[1].split(",").map((n) => n.trim());
		for (const name of names) {
			if (LUA_KEYWORDS.has(name)) continue;
			defs.delete(name); // re-insert so the map stays ordered by latest definition
			defs.set(name, { name, chunk, line: oneLine(line) });
		}
	}
	const all = [...defs.values()];
	const globals = all.slice(Math.max(0, all.length - MAX_GLOBALS));
	return { chunks, lastChunk, globals, totalGlobals: all.length };
}

const CLOCK_LINE = /^day \d+, \d\d:\d\d$/;
const JOURNAL_STAMP = /^- (day \d+, \d\d:\d\d):/;

/** The latest painting time printed on a line of its own in a tool result (what `wait()` returns). */
export function latestClockInMessages(entries: readonly unknown[]): string | undefined {
	for (let i = entries.length - 1; i >= 0; i--) {
		const entry = entries[i] as { type?: string; message?: { role?: string; content?: unknown } };
		if (entry?.type !== "message" || entry.message?.role !== "toolResult") continue;
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

export function buildSummary(cwd: string, entries: readonly unknown[]) {
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

	const log = readText(join(cwd, "paintings", "lua", "painting.lua"));
	parts.push("## Globals defined in paintings/lua/painting.lua");
	let logFacts: LogFacts | undefined;
	if (!log) parts.push("(no log yet)");
	else if (log.note) parts.push(`(${log.note})`);
	else {
		logFacts = scanLog(log.text);
		if (logFacts.globals.length === 0) parts.push("(none at the top level of any chunk)");
		else {
			const lines: string[] = [];
			if (logFacts.totalGlobals > logFacts.globals.length) {
				lines.push(`(the ${logFacts.globals.length} most recently defined of ${logFacts.totalGlobals}; \`bin/easel log\` prints the whole log)`);
			}
			for (const g of logFacts.globals) lines.push(`- \`${g.name}\` (chunk ${g.chunk}): \`${g.line}\``);
			parts.push(lines.join("\n"));
		}
		facts.chunks = logFacts.chunks;
		facts.lastChunk = logFacts.lastChunk;
		facts.globals = logFacts.totalGlobals;
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

export default function painterCompaction(pi: ExtensionAPI) {
	if (!applyCompactionSettings()) {
		console.error("painter compaction: pi's SettingsManager has no compaction getters to override; pi's own settings apply");
	}
	pi.on("session_before_compact", async (event, ctx) => {
		const { preparation, branchEntries, reason } = event;
		let summary: string;
		let facts: Record<string, unknown> = {};
		try {
			({ summary, facts } = buildSummary(ctx.cwd, branchEntries));
		} catch (error) {
			// Never fall through to pi's default (LLM, "Next Steps") summarizer.
			summary = `${HEADER}\n\nBRIEF.md and notes/journal.md are in the studio.`;
			facts = { error: error instanceof Error ? error.message : String(error) };
		}
		return {
			compaction: {
				summary,
				firstKeptEntryId: preparation.firstKeptEntryId,
				tokensBefore: preparation.tokensBefore,
				details: {
					type: DETAILS_TYPE, version: 1, reason, settings: preparation.settings,
					harnessSettings: sameSettings(preparation.settings), ...facts,
				},
			},
		};
	});
}
