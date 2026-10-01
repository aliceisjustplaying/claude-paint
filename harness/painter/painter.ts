/**
 * Painter session hygiene. Load with `pi --no-extensions -e harness/painter/painter.ts ...`.
 *
 * 1. The system prompt is only what we give the painter: `--system-prompt` supplies the
 *    preamble, and this handler drops everything pi would otherwise add around it from the
 *    machine's global setup (the agent-directory APPEND_SYSTEM.md "addendum", context files,
 *    skills, tool guidelines). Pi's own <cwd> section stays.
 * 2. The painter's tools are the easel's (easel-tools.ts: paint, look, note, status, log) and
 *    `read` inside the studio. No shell: no network, no other studios, no processes to see,
 *    nothing of this machine's setup in the environment. They run one at a time (pi runs the
 *    tool calls of one message in parallel; two chunks must not race).
 * 3. Old images stay out of the request (context-images.ts): before each provider request,
 *    the images of older tool results are replaced by a line naming the file, keeping the
 *    newest 20 and at most 12 MB of base64 (PAINTER_MAX_IMAGES and PAINTER_MAX_IMAGE_MB change
 *    that for a lane). Pi runs `context` handlers on a copy of the messages, so the session
 *    file keeps every image, and a `painter-request-images` entry records what each request
 *    kept and dropped.
 * 4. Input tokens per minute (pace.ts): with PAINTER_INPUT_TPM set, each request waits until
 *    the input tokens of the last minute's requests plus its own fit that budget; and a
 *    per-minute quota 429 (Google's `...PerMinute` quota ids) is made retryable for pi's
 *    retry, which otherwise skips it for mentioning "quota exceeded" and "billing".
 * 5. Every painter sees its looks at its provider's best image resolution (vision.ts): Gemini's
 *    ultra-high media resolution, OpenAI's `detail: "high"`.
 * 6. After a compaction the painter sees the canvas as it was when the summary was written: the
 *    summary names a fresh whole-canvas look (summary.ts), and each request shows that image
 *    right after the summary. A later sitting (PAINTER_SITTING_RECOVERY=1, set by the runner)
 *    opens with the same sections as a compaction summary and the same fresh look.
 * 7. A usage limit that resets with time (limits.ts) doesn't end the sitting: the run waits until
 *    the limit should have lifted and then asks again with the conversation as it was, nothing
 *    added (PAINTER_LIMIT_PROBE_S and PAINTER_LIMIT_GIVE_UP_H change the waits).
 * The PAINTER_* variables are read once and removed from the environment.
 */
import { createReadToolDefinition, type ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { buildSummary, CANVAS_VIEW } from "./summary.ts";
import { registerEaselTools } from "./easel-tools.ts";
import vision from "./vision.ts";
import { limitsFromEnv, pruneImages } from "./context-images.ts";
import { requestTokens, retryablePerMinuteQuota, TokenPace } from "./pace.ts";
import { limitWaitFromEnv, limitWaitMs, usageLimit } from "./limits.ts";

export default function painter(pi: ExtensionAPI) {
	pi.on("before_agent_start", (event) => {
		const options = event.systemPromptOptions;
		// system_prompt.md may open with an HTML comment for us (e.g. its DRAFT marker): not for the painter.
		if (options.customPrompt) options.customPrompt = options.customPrompt.replace(/^\s*<!--[\s\S]*?-->\s*/, "");
		options.appendSystemPrompt = "";
		options.contextFiles = [];
		options.skills = [];
		options.promptGuidelines = [];
		options.toolGuidelines = {};
	});

	const limits = limitsFromEnv();
	const tpm = process.env.PAINTER_INPUT_TPM ? Number(process.env.PAINTER_INPUT_TPM) : undefined;
	if (tpm !== undefined && !(tpm > 0)) throw new Error(`PAINTER_INPUT_TPM=${process.env.PAINTER_INPUT_TPM}: want tokens per minute`);
	const limitWait = limitWaitFromEnv();
	let sittingRecovery = process.env.PAINTER_SITTING_RECOVERY === "1";
	for (const k of ["PAINTER_MAX_IMAGES", "PAINTER_MAX_IMAGE_MB", "PAINTER_INPUT_TPM", "PAINTER_LIMIT_PROBE_S", "PAINTER_LIMIT_GIVE_UP_H", "PAINTER_SITTING_RECOVERY"]) delete process.env[k];
	const studio = process.cwd();
	const read = createReadToolDefinition(studio);
	const views = new Map<string, unknown[]>();
	/** The image blocks of a canvas view in the studio (read once). */
	async function viewImages(path: string, ctx: Parameters<typeof read.execute>[4]): Promise<unknown[]> {
		let imgs = views.get(path);
		if (!imgs) {
			try {
				const r = await read.execute("canvas-view", { path }, undefined, undefined, ctx);
				imgs = r.content.filter((c) => c.type === "image");
			} catch {
				imgs = [];
			}
			views.set(path, imgs);
		}
		return imgs;
	}

	// a later sitting opens with what a compaction summary holds, and the canvas as it is
	pi.on("input", async (event, ctx) => {
		if (!sittingRecovery) return { action: "continue" as const };
		sittingRecovery = false;
		const { summary } = await buildSummary(studio, [], undefined, "");
		const view = CANVAS_VIEW.exec(summary)?.[1];
		const images = view ? await viewImages(view, ctx) : [];
		return { action: "transform" as const, text: `${event.text}\n\n${summary}`, images: [...(event.images ?? []), ...(images as never[])] };
	});
	const pace = tpm ? new TokenPace(tpm) : undefined;
	let lastRequest = 0;
	// the latest usage-limit error of this run, and since when the limit has held
	let limited: { text: string; since: number } | undefined;
	let pendingLimit = false;

	pi.on("context", async (event, ctx) => {
		// after each compaction summary, the canvas view it names (the session file holds the summary only)
		let shown = false;
		const withViews: typeof event.messages = [];
		for (const m of event.messages) {
			withViews.push(m);
			const summary = (m as { role?: string; summary?: string }).role === "compactionSummary" ? (m as { summary?: string }).summary : undefined;
			const view = summary ? CANVAS_VIEW.exec(summary)?.[1] : undefined;
			const images = view ? await viewImages(view, ctx) : [];
			if (images.length) {
				withViews.push({ role: "custom", customType: "painter-canvas-view", display: false, timestamp: (m as { timestamp?: number }).timestamp ?? 0,
					content: [{ type: "text", text: `The canvas as it was when this was written (${view}):` }, ...images] } as never);
				shown = true;
			}
		}
		const { messages, dropped, images, keptChars } = pruneImages(withViews, limits);
		// what this request showed of the images (the session file only, not the model): with each
		// response's token usage, the record for deciding whether more images could be kept
		pi.appendEntry("painter-request-images", { images, kept: images - dropped, dropped, keptChars, maxImages: limits.maxImages, maxImageChars: limits.maxImageChars });
		if (pace) {
			const wait = pace.waitMs(lastRequest, Date.now());
			if (wait > 0) await new Promise<void>((resolve) => {
				const timer = setTimeout(resolve, wait);
				ctx.signal?.addEventListener("abort", () => (clearTimeout(timer), resolve()), { once: true });
			});
		}
		return dropped > 0 || shown ? { messages } : undefined;
	});

	pi.on("message_end", (event) => {
		const m = event.message as { role?: string; stopReason?: string; errorMessage?: string; usage?: Parameters<typeof requestTokens>[0] };
		if (m.role !== "assistant") return undefined;
		const tokens = requestTokens(m.usage);
		if (tokens > 0) {
			lastRequest = tokens;
			pace?.record(tokens, Date.now());
		}
		if (m.stopReason === "error" && usageLimit(m.errorMessage)) {
			limited = { text: m.errorMessage!, since: limited?.since ?? Date.now() };
			pendingLimit = true;
		} else if (m.stopReason !== "error") limited = undefined;
		const retryable = m.stopReason === "error" ? retryablePerMinuteQuota(m.errorMessage) : undefined;
		return retryable ? { message: { ...event.message, errorMessage: retryable } as typeof event.message } : undefined;
	});

	// a run that ended on a usage limit waits, then goes on with nothing added to the conversation:
	// the errored response is left out of the model's context (a context_edit; the session file
	// keeps it), so the next request is the conversation as it was before the limit
	pi.on("agent_before_settle", async (event) => {
		if (!pendingLimit || !limited) return undefined;
		pendingLimit = false;
		const wait = limitWaitMs(limited.text, limited.since, Date.now(), limitWait);
		const last = event.context.contextEntries.at(-1);
		const m = last?.sourceEntry.type === "message" ? (last.sourceEntry as { message: { role?: string; stopReason?: string } }).message : undefined;
		if (wait === undefined || !last || m?.role !== "assistant" || m.stopReason !== "error") {
			console.error(`painter: usage limit since ${new Date(limited.since).toISOString()}; settling on it`);
			return undefined;
		}
		console.error(`painter: usage limit; asking again in ${Math.round(wait / 60_000)} min`);
		await new Promise<void>((resolve) => setTimeout(resolve, wait));
		return { entries: [{ type: "context_edit" as const, targetId: last.sourceEntry.id, replacement: null }], continue: true };
	});

	registerEaselTools(pi, process.cwd());
	vision(pi);
}
