/**
 * Usage limits that reset with time (OpenCode Go's 5-hour, weekly and monthly windows).
 *
 * When a request ends on such a limit, painter.ts holds the run open at `agent_before_settle`
 * until the limit should have lifted, then asks pi for one more request (`continue: true`).
 * Pi leaves the errored response out of what the model sees, so the painter's next request is
 * its conversation as it was, with nothing added: no message about a connection or a wait.
 * If the limit is still there, that request fails the same way and the wait repeats. After
 * giveUpMs of limits in a row the run settles on the error, and the runner's own handling of
 * a limited sitting (r21_chains.py) takes over.
 *
 * The patterns are the runner's (usage_limit, limit_reset_s): an empty balance is not a usage
 * limit, since waiting doesn't refill it.
 */

const USAGE_LIMIT = /GoUsageLimitError|FreeUsageLimitError|usage limit (exceeded|reached)/i;
const RESETS = /resets? in ((?:\s*[\d.]+\s*(?:days?|d|hours?|hrs?|h|minutes?|mins?|m|seconds?|secs?|s)\b)+)/i;
const UNIT: Record<string, number> = { d: 86_400_000, h: 3_600_000, m: 60_000, s: 1_000 };

export function usageLimit(text: string | undefined): boolean {
	return !!text && USAGE_LIMIT.test(text);
}

/** Milliseconds until the limit resets, from "Resets in 2hr 15min" (or "Resets in 2 days"), or undefined. */
export function limitResetMs(text: string): number | undefined {
	const m = RESETS.exec(text);
	if (!m) return undefined;
	let ms = 0;
	for (const [, v, u] of m[1].matchAll(/([\d.]+)\s*([a-z]+)/gi)) ms += Number(v) * UNIT[u[0].toLowerCase()];
	return ms;
}

export interface LimitWait {
	probeMs: number;     // the wait when the error names no reset time (the runner's LIMIT_PROBE_S)
	giveUpMs: number;    // limits in a row for this long: settle on the error (the runner's LIMIT_GIVE_UP_H)
}

export const LIMIT_WAIT: LimitWait = { probeMs: 30 * 60_000, giveUpMs: 24 * 3_600_000 };

/** LIMIT_WAIT with PAINTER_LIMIT_PROBE_S and PAINTER_LIMIT_GIVE_UP_H from `env` where set. */
export function limitWaitFromEnv(env: NodeJS.ProcessEnv = process.env): LimitWait {
	const probe = env.PAINTER_LIMIT_PROBE_S, giveUp = env.PAINTER_LIMIT_GIVE_UP_H;
	const w = { ...LIMIT_WAIT };
	if (probe !== undefined) {
		if (!(Number(probe) >= 0)) throw new Error(`PAINTER_LIMIT_PROBE_S=${probe}: want seconds`);
		w.probeMs = Number(probe) * 1000;
	}
	if (giveUp !== undefined) {
		if (!(Number(giveUp) >= 0)) throw new Error(`PAINTER_LIMIT_GIVE_UP_H=${giveUp}: want hours`);
		w.giveUpMs = Number(giveUp) * 3_600_000;
	}
	return w;
}

/**
 * How long to wait before asking again after a limit error with `text`, or undefined to settle
 * on the error: the limit has held since `since` for giveUpMs or more.
 */
export function limitWaitMs(text: string, since: number, now: number, w: LimitWait): number | undefined {
	if (now - since >= w.giveUpMs) return undefined;
	const reset = limitResetMs(text);
	return reset !== undefined ? reset + 60_000 : w.probeMs;
}
