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
 *    file keeps every image.
 * 4. Input tokens per minute (pace.ts): with PAINTER_INPUT_TPM set, each request waits until
 *    the input tokens of the last minute's requests plus its own fit that budget; and a
 *    per-minute quota 429 (Google's `...PerMinute` quota ids) is made retryable for pi's
 *    retry, which otherwise skips it for mentioning "quota exceeded" and "billing".
 * The PAINTER_* variables are read once and removed from the environment.
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { registerEaselTools } from "./easel-tools.ts";
import { limitsFromEnv, pruneImages } from "./context-images.ts";
import { requestTokens, retryablePerMinuteQuota, TokenPace } from "./pace.ts";

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
	for (const k of ["PAINTER_MAX_IMAGES", "PAINTER_MAX_IMAGE_MB", "PAINTER_INPUT_TPM"]) delete process.env[k];
	const pace = tpm ? new TokenPace(tpm) : undefined;
	let lastRequest = 0;

	pi.on("context", async (event, ctx) => {
		const { messages, dropped } = pruneImages(event.messages, limits);
		if (pace) {
			const wait = pace.waitMs(lastRequest, Date.now());
			if (wait > 0) await new Promise<void>((resolve) => {
				const timer = setTimeout(resolve, wait);
				ctx.signal?.addEventListener("abort", () => (clearTimeout(timer), resolve()), { once: true });
			});
		}
		return dropped > 0 ? { messages } : undefined;
	});

	pi.on("message_end", (event) => {
		const m = event.message as { role?: string; stopReason?: string; errorMessage?: string; usage?: Parameters<typeof requestTokens>[0] };
		if (m.role !== "assistant") return undefined;
		const tokens = requestTokens(m.usage);
		if (tokens > 0) {
			lastRequest = tokens;
			pace?.record(tokens, Date.now());
		}
		const retryable = m.stopReason === "error" ? retryablePerMinuteQuota(m.errorMessage) : undefined;
		return retryable ? { message: { ...event.message, errorMessage: retryable } as typeof event.message } : undefined;
	});

	registerEaselTools(pi, process.cwd());
}
