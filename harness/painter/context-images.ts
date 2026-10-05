/**
 * Old images out of the request, not out of the session.
 *
 * Every `look` the painter reads is a PNG of 50–750 KB of base64 in a tool result, and pi sends
 * every one of them again with each request. A round 17 painter hit Anthropic's
 * `413 request_too_large` after 74 looks (33.7 MB of base64) while its token count was far
 * below the compaction point. `pruneImages` is run from painter.ts's `context` handler, which
 * pi calls before each provider request on a copy of the messages (the session file, which
 * the studio viewer reads the images from, is not touched). It replaces the image blocks of
 * older messages with a line naming the file, and keeps the newest ones:
 *
 *   - at most MAX_IMAGES images, and
 *   - at most MAX_IMAGE_CHARS characters of base64 among them.
 *
 * The newest tool result's images stay together when that group fits both limits.
 * The newest image is kept unless it alone is over MAX_IMAGE_CHARS; then it is dropped too,
 * and its line says it was left out and why.
 *
 * Images are dropped STEP at a time, oldest first, so the request's prefix (and the
 * provider's prompt cache) changes once every STEP new images rather than with every look.
 * The number dropped depends only on the images in the messages, so the same messages
 * always give the same request.
 *
 * A launcher can lower (or raise) the limits for a lane with PAINTER_MAX_IMAGES and
 * PAINTER_MAX_IMAGE_MB (see limitsFromEnv): round 18g's Gemini lane keeps 8, since every
 * request counts against Google's input tokens per minute.
 */

export const MAX_IMAGES = 20;
export const MAX_IMAGE_CHARS = 12_000_000;
export const STEP = 5;

export interface PruneLimits {
	maxImages: number;
	maxImageChars: number;
	step: number;
}

export const LIMITS: PruneLimits = { maxImages: MAX_IMAGES, maxImageChars: MAX_IMAGE_CHARS, step: STEP };

/** LIMITS, with PAINTER_MAX_IMAGES (a count, at least 1) and PAINTER_MAX_IMAGE_MB (MB of base64) from `env` where set. */
export function limitsFromEnv(env: Record<string, string | undefined> = process.env): PruneLimits {
	const num = (name: string, min: number): number | undefined => {
		const raw = env[name];
		if (raw === undefined || raw.trim() === "") return undefined;
		const v = Number(raw);
		if (!Number.isFinite(v) || v < min) throw new Error(`${name}=${raw}: want a number >= ${min}`);
		return v;
	};
	const images = num("PAINTER_MAX_IMAGES", 1);
	const mb = num("PAINTER_MAX_IMAGE_MB", 0.001);
	return {
		maxImages: images === undefined ? MAX_IMAGES : Math.floor(images),
		maxImageChars: mb === undefined ? MAX_IMAGE_CHARS : Math.round(mb * 1_000_000),
		step: STEP,
	};
}

type Block = { type?: string; data?: string; text?: string; [k: string]: unknown };
type Message = { role?: string; content?: unknown; toolCallId?: string; [k: string]: unknown };

/**
 * How many of `sizes` (oldest first) to drop so the rest fit the limits; a multiple of `step`,
 * but never the newest unless it alone is over the size limit.
 */
export function imagesToDrop(sizes: readonly number[], limits: PruneLimits = LIMITS): number {
	const total = sizes.length;
	if (total === 0) return 0;
	const roundUp = (n: number) => (n <= 0 ? 0 : Math.ceil(n / limits.step) * limits.step);
	const byCount = roundUp(total - limits.maxImages);
	let chars = sizes.reduce((a, b) => a + b, 0);
	let first = 0;
	while (first < total && chars > limits.maxImageChars) chars -= sizes[first++];
	const byChars = roundUp(first);
	const most = sizes[total - 1] > limits.maxImageChars ? total : total - 1;
	return Math.min(Math.max(byCount, byChars), most);
}

/** The file each tool call read, by tool call id (from the assistant messages' `read` calls). */
function readPaths(messages: readonly Message[]): Map<string, string> {
	const paths = new Map<string, string>();
	for (const m of messages) {
		if (m?.role !== "assistant" || !Array.isArray(m.content)) continue;
		for (const c of m.content as { type?: string; id?: string; arguments?: { path?: unknown } }[]) {
			if (c?.type === "toolCall" && c.id && typeof c.arguments?.path === "string") paths.set(c.id, c.arguments.path);
			else if (c?.type === "toolCall" && c.id && (c as { name?: string }).name === "look") paths.set(c.id, "");
		}
	}
	return paths;
}

export function placeholder(path: string | undefined): string {
	return path ? `[an earlier look: ${path}]` : path === "" ? "[an earlier look]" : "[an earlier image]";
}

/** What stands for an image that alone is over the size limit. */
export function oversize(chars: number, limit: number): string {
	const mb = (n: number) => `${Number((n / 1_000_000).toFixed(1))} MB`;
	return `[this image was left out of the request: it is ${mb(chars)}, over the ${mb(limit)} limit for the images in one request]`;
}

export interface PruneResult<M> {
	messages: M[];
	images: number;
	dropped: number;
	keptChars: number;
}

/**
 * The messages with their oldest images replaced by a placeholder line. Messages that change
 * are new objects; the others (and the input array) are left as they are.
 */
export function pruneImages<M extends Message>(messages: readonly M[], limits: PruneLimits = LIMITS): PruneResult<M> {
	const found: { m: number; c: number; chars: number }[] = [];
	messages.forEach((msg, m) => {
		if (msg?.role === "assistant" || !Array.isArray(msg?.content)) return;
		(msg.content as Block[]).forEach((block, c) => {
			if (block?.type === "image") found.push({ m, c, chars: typeof block.data === "string" ? block.data.length : 0 });
		});
	});
	const sizes = found.map((f) => f.chars);
	let dropped = imagesToDrop(sizes, limits);
	// Cache-friendly step rounding must not cut into a fresh survey batch that fits.
	const newestMessage = found.at(-1)?.m;
	const firstNewest = found.findIndex((f) => f.m === newestMessage);
	if (firstNewest >= 0 && messages[newestMessage!].role === "toolResult") {
		const newest = sizes.slice(firstNewest);
		if (newest.length <= limits.maxImages && newest.reduce((a, b) => a + b, 0) <= limits.maxImageChars) {
			dropped = Math.min(dropped, firstNewest);
		}
	}
	const keptChars = sizes.slice(dropped).reduce((a, b) => a + b, 0);
	if (dropped === 0) return { messages: messages.slice(), images: found.length, dropped, keptChars };

	const paths = readPaths(messages);
	const out = messages.slice();
	const byMessage = new Map<number, Map<number, number>>();
	found.slice(0, dropped).forEach((f, i) => {
		if (!byMessage.has(f.m)) byMessage.set(f.m, new Map());
		byMessage.get(f.m)!.set(f.c, i);
	});
	for (const [m, blocks] of byMessage) {
		const msg = messages[m];
		const path = msg.role === "toolResult" && msg.toolCallId ? paths.get(msg.toolCallId) : undefined;
		const content = (msg.content as Block[]).map((block, c) => {
			const i = blocks.get(c);
			if (i === undefined) return block;
			// the newest image goes only when it alone is over the limit: say so, not "an earlier look"
			const text = i === found.length - 1 ? oversize(found[i].chars, limits.maxImageChars) : placeholder(path);
			return { type: "text", text };
		});
		out[m] = { ...msg, content } as M;
	}
	return { messages: out, images: found.length, dropped, keptChars };
}
