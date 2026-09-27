/**
 * Painter session hygiene. Load with `pi --no-extensions -e harness/painter/painter.ts ...`.
 *
 * 1. The system prompt is only what we give the painter: `--system-prompt` supplies the
 *    preamble, and this handler drops everything pi would otherwise add around it from the
 *    machine's global setup (the agent-directory APPEND_SYSTEM.md "addendum", context files,
 *    skills, tool guidelines). Pi's own <cwd> section stays.
 * 2. `bash` and `read` run one at a time. Pi runs the tool calls of one assistant message in
 *    parallel; round 16 painters had Alice's global pi-batch-order extension serializing
 *    them. Without that, two easel commands batched in one message would race. Same tool
 *    definitions (name, description, parameters) as pi's built-ins; only the execution
 *    mode differs.
 * 3. Old images stay out of the request (context-images.ts): before each provider request,
 *    the images of older tool results are replaced by a line naming the file, keeping the
 *    newest 20 and at most 12 MB of base64. Pi runs `context` handlers on a copy of the
 *    messages, so the session file keeps every image.
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { createBashToolDefinition, createReadToolDefinition } from "@earendil-works/pi-coding-agent";
import { pruneImages } from "./context-images.ts";

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

	pi.on("context", (event) => {
		const { messages, dropped } = pruneImages(event.messages);
		return dropped > 0 ? { messages } : undefined;
	});

	const cwd = process.cwd();
	pi.registerTool({ ...createBashToolDefinition(cwd), executionMode: "sequential" });
	pi.registerTool({ ...createReadToolDefinition(cwd), executionMode: "sequential" });
}
