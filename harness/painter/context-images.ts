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

/** How many of `sizes` (oldest first) to drop so the rest fit the limits; a multiple of `step`, never all of them. */
export function imagesToDrop(sizes: readonly number[], limits: PruneLimits = LIMITS): number {
	const total = sizes.length;
	if (total === 0) return 0;
	const roundUp = (n: number) => (n <= 0 ? 0 : Math.ceil(n / limits.step) * limits.step);
	const byCount = roundUp(total - limits.maxImages);
	let chars = sizes.reduce((a, b) => a + b, 0);
	let first = 0;
	while (first < total && chars > limits.maxImageChars) chars -= sizes[first++];
	const byChars = roundUp(first);
	return Math.min(Math.max(byCount, byChars), total - 1);
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
	const dropped = imagesToDrop(sizes, limits);
	const keptChars = sizes.slice(dropped).reduce((a, b) => a + b, 0);
	if (dropped === 0) return { messages: messages.slice(), images: found.length, dropped, keptChars };

	const paths = readPaths(messages);
	const out = messages.slice();
	const byMessage = new Map<number, Set<number>>();
	for (const f of found.slice(0, dropped)) {
		if (!byMessage.has(f.m)) byMessage.set(f.m, new Set());
		byMessage.get(f.m)!.add(f.c);
	}
	for (const [m, blocks] of byMessage) {
		const msg = messages[m];
		const path = msg.role === "toolResult" && msg.toolCallId ? paths.get(msg.toolCallId) : undefined;
		const content = (msg.content as Block[]).map((block, c) =>
			blocks.has(c) ? { type: "text", text: placeholder(path) } : block,
		);
		out[m] = { ...msg, content } as M;
	}
	return { messages: out, images: found.length, dropped, keptChars };
}
