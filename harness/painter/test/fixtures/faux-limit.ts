/**
 * A provider for limits.test.ts: two usage-limit errors, then a reply that records the messages
 * it was asked with (in FAUX_OUT). Load with `pi -e painter.ts -e faux-limit.ts --provider faux --model m`.
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { fauxAssistantMessage, fauxProvider } from "@earendil-works/pi-ai";
import { writeFileSync } from "node:fs";

const LIMIT = '429: {"type":"GoUsageLimitError","message":"Go usage limit exceeded"}';

export default function (pi: ExtensionAPI) {
	const faux = fauxProvider({ provider: "faux", models: [{ id: "m" }] });
	faux.setResponses([
		fauxAssistantMessage("", { stopReason: "error", errorMessage: LIMIT }),
		fauxAssistantMessage("", { stopReason: "error", errorMessage: LIMIT }),
		(context) => {
			const asked = context.messages.map((m) => ({ role: m.role, content: m.content }));
			writeFileSync(process.env.FAUX_OUT!, JSON.stringify({ calls: faux.state.callCount, asked }));
			return fauxAssistantMessage("done");
		},
	]);
	pi.registerProvider(faux.provider);
}
