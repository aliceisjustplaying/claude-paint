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
	registerLook(pi, studio, limits);
	const read = createReadToolDefinition(studio);
	pi.registerTool({
		...read,
		async execute(id, p, signal, onUpdate, ctx) {
			const real = criticReadable(studio, p.path);
			if (!real) throw new Error(`${p.path}: you can read ${JOURNAL}, ${TO_CRITIC} and the easel's looks`);
			if (!existsSync(real)) throw new Error(`${p.path}: there's no such file yet`);
			return read.execute(id, { ...p, path: real }, signal, onUpdate, ctx);
		},
		executionMode: "sequential",
	});
	vision(pi);
}
