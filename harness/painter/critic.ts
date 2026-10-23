/**
 * The critic's session (visit.ts starts it): a second painter visiting the studio. Load with
 * `pi --no-extensions -e harness/painter/critic.ts --tools look,read ...`.
 *
 * Its tools are the painter's `look` (easel-tools.ts) and a `read` limited to the painter's journal,
 * the painter's note to it and the easel's looks (visit.ts's criticReadable). It has no paint tools and
 * reads nothing else in the studio. Its system prompt is only critic_system_prompt.md and old images
 * leave each request, as painter.ts does for the painter.
 */
import { existsSync } from "node:fs";
import { createReadToolDefinition, type ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { registerLook } from "./easel-tools.ts";
import { limitsFromEnv, pruneImages } from "./context-images.ts";
import { onlyOurSystemPrompt } from "./painter.ts";
import vision from "./vision.ts";
import { JOURNAL } from "./journal.ts";
import { criticReadable, TO_CRITIC } from "./visit.ts";

export default function critic(pi: ExtensionAPI) {
	onlyOurSystemPrompt(pi);
	const studio = process.cwd();
	const limits = limitsFromEnv();
	pi.on("context", (event) => {
		const { messages, dropped } = pruneImages(event.messages, limits);
		return dropped > 0 ? { messages } : undefined;
	});
	const readable = (path: string): string => {
		const real = criticReadable(studio, path);
		if (!real) throw new Error(`${path}: you can read ${JOURNAL}, ${TO_CRITIC} and the easel's looks`);
		return real;
	};
	registerLook(pi, studio, limits, {
		resolve: readable,
		description:
			"Look at the painting as it is now. Without options: the whole canvas, scaled down. " +
			"crop: \"x0,y0,x1,y1\" in canvas units (two opposite corners), at full detail up to 1200 pixels a side; larger crops are scaled down. The reply gives the scale in pixels per canvas unit. " +
			"mode: \"value\" (the picture in grays, its lightness alone, in the gallery light), \"squint\", \"edges\" (the picture's edges drawn over a dim gray of it: orange where an edge is hard, blue where it is soft, brighter the more the lightness changes across it; hard and soft as they read at the size of the image the look returns), \"mirror\", \"relief\" (a raking light on the paint's ridges and furrows), \"gallery\" (the light the picture hangs in) or several, comma-separated. " +
			"light: \"azimuth,elevation\" in degrees for the relief light (default \"135,25\", from the upper left). size: the long side in pixels. " +
			"grid: true, or a spacing in canvas units. " +
			"survey: true surveys the whole canvas at full detail, as several tiles (with mode, not crop or size); a partial reply lists remaining tiles to read in separate turns. " +
			"compare: the path of an earlier look, as its reply names it, resized to the current view's height and shown left of it; to compare an earlier look, supply matching crop, mode and light options explicitly.",
	});
	const read = createReadToolDefinition(studio);
	pi.registerTool({
		...read,
		async execute(id, p, signal, onUpdate, ctx) {
			const real = readable(p.path);
			if (!existsSync(real)) throw new Error(`${p.path}: there's no such file yet`);
			return read.execute(id, { ...p, path: real }, signal, onUpdate, ctx);
		},
		executionMode: "sequential",
	});
	vision(pi);
}
