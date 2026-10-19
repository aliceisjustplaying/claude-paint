import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

const FRAMING = "A message from the person who runs this studio:";
type Message = { id: string; text: string };

export function registerInbox(pi: ExtensionAPI) {
	const run = process.env.PAINTER_INBOX_DIR;
	const sitting = process.env.PAINTER_INBOX_SITTING;
	delete process.env.PAINTER_INBOX_DIR;
	delete process.env.PAINTER_INBOX_SITTING;
	if (!run) return;
	if (!sitting) throw new Error("PAINTER_INBOX_SITTING is required with PAINTER_INBOX_DIR");
	const helper = join(import.meta.dirname, "inbox.py");
	const call = (operation: string, target?: object): Message[] => {
		if (!existsSync(join(run, "painter-inbox.json"))) return [];
		return JSON.parse(execFileSync("uv", ["run", "--no-project", "python", helper, operation, run,
			...(target ? [JSON.stringify(target)] : [])], { encoding: "utf8", maxBuffer: Number.POSITIVE_INFINITY }));
	};
	const receive = (session: string | undefined, tool_call: string | null, tool: string | null) => {
		if (!existsSync(join(run, "painter-inbox.json"))) return [];
		if (!session) throw new Error("Painter inbox requires a persistent pi session");
		return call("receive", { session, sitting, tool_call, tool, pid: process.pid });
	};
	const frame = (messages: Message[]) => messages.map((m) => `${FRAMING}\n\n${m.text}\n\nEnd of message.`).join("\n\n");
	let opening = true;
	pi.on("input", (event, ctx) => {
		if (!opening) return;
		opening = false;
		const messages = receive(ctx.sessionManager.getSessionFile(), null, null);
		if (!messages.length) return;
		return { action: "transform", text: `${frame(messages)}\n\n${event.text}`, images: event.images };
	});
	pi.on("tool_result", (event, ctx) => {
		if (event.parentToolCallId) return;
		const messages = receive(ctx.sessionManager.getSessionFile(), event.toolCallId, event.toolName);
		if (!messages.length) return;
		return { content: [...event.content, { type: "text", text: frame(messages) }],
			details: { ...event.details, painterInbox: messages.map((m) => m.id) } };
	});
	pi.on("context", () => { call("reconcile"); });
	pi.on("agent_end", () => { call("reconcile"); });
}
