/**
 * Input tokens per minute, and per-minute quota errors.
 *
 * Google's AI Studio caps input tokens per model per minute (2,000,000 for gemini-3.8-flash on
 * the paid tier), counting the whole prompt of each request, cached or not. A round 18g
 * painter at ~215K tokens a request and ~10 requests a minute went over it and its sitting
 * ended on the 429. Two things keep a lane under such a cap:
 *
 *  - TokenPace: before each request, painter.ts waits until the input tokens of the requests
 *    of the last minute plus this one (estimated as the last request's) fit the budget
 *    (PAINTER_INPUT_TPM). Only the wall clock changes; the painter's messages don't.
 *  - retryablePerMinuteQuota: pi's retry (settings `retry`) skips errors that mention
 *    "quota exceeded" or "billing" (pi-ai's NON_RETRYABLE_PROVIDER_LIMIT_ERROR_PATTERN), and
 *    Google's 429 for a per-minute quota says both. painter.ts rewrites such a message's
 *    error text (message_end) so pi retries it after its usual delay. Daily quotas and real
 *    billing errors are left as they are.
 */

export class TokenPace {
	private sent: { t: number; tokens: number }[] = [];
	readonly budget: number;
	readonly windowMs: number;

	constructor(budget: number, windowMs = 60_000) {
		this.budget = budget;
		this.windowMs = windowMs;
	}

	/** A request of `tokens` input tokens went out (or was answered) at `t`. */
	record(tokens: number, t: number): void {
		if (tokens > 0) this.sent.push({ t, tokens });
	}

	/** How long to wait from `t` before a request of about `next` tokens fits the budget. */
	waitMs(next: number, t: number): number {
		this.sent = this.sent.filter((s) => s.t > t - this.windowMs);
		let used = this.sent.reduce((a, s) => a + s.tokens, 0);
		if (used + next <= this.budget) return 0;
		for (const s of this.sent) {
			used -= s.tokens;
			if (used + next <= this.budget) return Math.max(0, s.t + this.windowMs - t);
		}
		// more than the whole budget on its own: it goes once the window is empty
		const newest = this.sent[this.sent.length - 1];
		return newest ? Math.max(0, newest.t + this.windowMs - t) : 0;
	}
}

/** The input tokens a response's usage says its request had (new plus cached). */
export function requestTokens(usage: { input?: number; cacheRead?: number; cacheWrite?: number } | undefined): number {
	return (usage?.input ?? 0) + (usage?.cacheRead ?? 0) + (usage?.cacheWrite ?? 0);
}

/**
 * For a provider error that is a per-minute quota (Google's `...PerMinute` quota ids; pi's error
 * text holds Google's JSON, possibly escaped inside another JSON string), an error
 * text pi's retry classifier treats as a rate limit; undefined for anything else.
 */
export function retryablePerMinuteQuota(errorMessage: string | undefined): string | undefined {
	if (!errorMessage || !/429|RESOURCE_EXHAUSTED|Too Many Requests/i.test(errorMessage)) return undefined;
	const ids = [...errorMessage.matchAll(/quotaId\\*"\s*:\s*\\*"([A-Za-z]+)/g)].map((m) => m[1]);
	if (ids.length === 0 || !ids.every((id) => /PerMinute/.test(id))) return undefined;
	const wait = errorMessage.match(/retry in ([\d.]+)s/i)?.[1];
	return `429 rate limit (per-minute input quota: ${[...new Set(ids)].join(", ")})${wait ? `; the provider asks to wait ${wait}s` : ""}`;
}
