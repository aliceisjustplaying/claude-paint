/**
 * The reader's session hygiene, as harness/painter/painter.ts does it for the painter. Load with
 * `pi --no-extensions -e reader.ts --system-prompt reader_system_prompt.md ...`: the system
 * prompt is only reader_system_prompt.md, with nothing from the machine's global setup (the
 * agent-directory APPEND_SYSTEM.md, context files, skills, tool guidelines).
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

export default function reader(pi: ExtensionAPI) {
	pi.on("before_agent_start", (event) => {
		const options = event.systemPromptOptions;
		options.appendSystemPrompt = "";
		options.contextFiles = [];
		options.skills = [];
		options.promptGuidelines = [];
		options.toolGuidelines = {};
	});
}
