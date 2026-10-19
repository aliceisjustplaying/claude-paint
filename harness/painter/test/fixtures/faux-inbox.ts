import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { fauxAssistantMessage, fauxProvider } from "@earendil-works/pi-ai";
import { execFileSync } from "node:child_process";

export default function (pi: ExtensionAPI) {
	const faux = fauxProvider({ provider: "faux", models: [{ id: "m", input: ["text"], contextWindow: 200_000 }] });
	const call = fauxAssistantMessage("");
	call.content = [{ type: "toolCall", id: "inbox-read", name: "read", arguments: { path: "BRIEF.md" } }];
	call.stopReason = "toolUse";
	faux.setResponses([call, fauxAssistantMessage("done")]);
	pi.registerProvider(faux.provider);
	pi.on("tool_call", () => {
		if (process.env.INBOX_TEST_SEND) execFileSync(process.env.INBOX_TEST_SEND, [process.cwd(), "The mid-sitting message."]);
	});
}
