import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { join } from "node:path";
import type { ExtensionAPI, ToolResultEvent } from "@earendil-works/pi-coding-agent";

const FRAMING = "A message from the person who runs this studio:";
type Message = { id: string; text: string };
type Attachment = { ids: string[]; failed?: boolean; opening?: { text: string; original: string };
	result?: Pick<ToolResultEvent, "toolCallId" | "content" | "details"> };

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
	let attachments: Attachment[] = [];
	const settle = () => {
		const released = new Set(call("settle", { pid: process.pid }).map((m) => m.id));
		attachments = attachments.filter((a) => a.failed ||= a.ids.some((id) => released.has(id)));
	};
	let opening = true;
	pi.on("input", (event, ctx) => {
		if (!opening) return;
		opening = false;
		const messages = receive(ctx.sessionManager.getSessionFile(), null, null);
		if (!messages.length) return;
		const text = `${frame(messages)}\n\n${event.text}`;
		attachments.push({ ids: messages.map((m) => m.id), opening: { text, original: event.text } });
		return { action: "transform", text, images: event.images };
	});
	pi.on("tool_result", (event, ctx) => {
		if (event.parentToolCallId) return;
		const messages = receive(ctx.sessionManager.getSessionFile(), event.toolCallId, event.toolName);
		if (!messages.length) return;
		attachments.push({ ids: messages.map((m) => m.id), result: { toolCallId: event.toolCallId, content: event.content, details: event.details } });
		return { content: [...event.content, { type: "text", text: frame(messages) }],
			details: { ...event.details, painterInbox: messages.map((m) => m.id) } };
	});
	pi.on("context", (event) => {
		settle();
		if (!attachments.length) return;
		return { messages: event.messages.map((message) => {
			for (const a of attachments) {
				if (a.opening && message.role === "user") {
					const { text, original } = a.opening;
					message = { ...message, content: typeof message.content === "string"
						? (message.content === text ? original : message.content)
						: message.content.map((b) => b.type === "text" && b.text === text ? { ...b, text: original } : b) };
				} else if (a.result && message.role === "toolResult" && message.toolCallId === a.result.toolCallId) {
					message = { ...message, content: a.result.content, details: a.result.details };
				}
			}
			return message;
		}) };
	});
	pi.on("agent_end", settle);
}
