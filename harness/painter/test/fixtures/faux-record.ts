/**
 * A provider for recovery.test.ts that records what each request holds (FAUX_OUT, JSON lines:
 * per message its role, its text and how many images). FAUX_SCRIPT=long answers at length (so
 * the next prompt in the session comes after a compaction); otherwise it answers "done". Load with `pi -e painter.ts -e faux-record.ts --provider faux --model m`.
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { fauxAssistantMessage, fauxProvider } from "@earendil-works/pi-ai";
import { appendFileSync } from "node:fs";

export default function (pi: ExtensionAPI) {
	const faux = fauxProvider({ provider: "faux", models: [{ id: "m", input: ["text", "image"], contextWindow: 200_000 }] });
	const record = (context: { messages: { role: string; content: unknown }[] }) => {
		const msgs = context.messages.map((m) => {
			const blocks = typeof m.content === "string" ? [{ type: "text", text: m.content }] : (m.content as { type: string; text?: string }[]);
			return { role: m.role, text: blocks.filter((b) => b.type === "text").map((b) => b.text).join("\n"), images: blocks.filter((b) => b.type === "image").length };
		});
		appendFileSync(process.env.FAUX_OUT!, JSON.stringify(msgs) + "\n");
	};
	const steps = process.env.FAUX_SCRIPT === "long"
		? [(c: never) => (record(c), fauxAssistantMessage("x ".repeat(60_000)))]
		: [(c: never) => (record(c), fauxAssistantMessage("done"))];
	faux.setResponses(steps);
	pi.registerProvider(faux.provider);
}
