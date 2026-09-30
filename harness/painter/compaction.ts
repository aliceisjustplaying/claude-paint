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
 *   - the painting's Lua globals as the easel holds them (`easel globals`), so the painter's
 *     names keep working: name, the chunk that last assigned it, what it holds; bounded.
 *     If the easel can't answer, the summary says so.
 *   - the canvas clock: chunks in the log, the latest valid painting time a successful paint call printed
 * summary.ts builds it. It deliberately has no next steps, remaining tasks or progress checklist.
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
import { type ExtensionAPI, SettingsManager } from "@earendil-works/pi-coding-agent";
import { buildSummary, HEADER } from "./summary.ts";

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

export default function painterCompaction(pi: ExtensionAPI) {
	if (!applyCompactionSettings()) {
		console.error("painter compaction: pi's SettingsManager has no compaction getters to override; pi's own settings apply");
	}
	pi.on("session_before_compact", async (event, ctx) => {
		const { preparation, branchEntries, reason, signal } = event;
		let summary: string;
		let facts: Record<string, unknown> = {};
		try {
			({ summary, facts } = await buildSummary(ctx.cwd, branchEntries, signal));
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
