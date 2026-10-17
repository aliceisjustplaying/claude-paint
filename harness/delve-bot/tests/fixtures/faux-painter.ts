/**
 * A provider for painter.test.ts (load with `-e` and `--provider faux --model m`). It appends the last user
 * message of every request to FAUX_OUT. FAUX_SCRIPT=paint: a canvas, a stroke, a look and a reply.
 * FAUX_SCRIPT=chatty or quiet: an endless sitting that keeps asking for the easel's status, with a long text
 * (2000 tokens at 4000 a second) or a short one (50 tokens at 100 a second) before each call.
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { fauxAssistantMessage, fauxProvider, fauxText, fauxToolCall } from "@earendil-works/pi-ai";
import { appendFileSync } from "node:fs";

type Context = { messages: { role: string; content: unknown }[] };

const CANVAS = 'canvas{size=300, aspect=4/3, linen=14, seed=1, ground={{pile={{"lead white",1}}, um=50, apply="roller"}}}';
const STROKE = 'p = pile{{"vermilion",1}}\nb = brush{kind="round", width=20, point=1, stiffness=0.5}\nb:load(p, 0.8)\nb:stroke({{200, 300}, {500, 400}, {800, 350}})';

export default function (pi: ExtensionAPI) {
	const faux = fauxProvider({ provider: "faux", models: [{ id: "m", input: ["text", "image"], contextWindow: 200_000 }],
		...(process.env.FAUX_SCRIPT === "paint" ? {} : { tokensPerSecond: process.env.FAUX_SCRIPT === "chatty" ? 4000 : 100 }) });
	const asked = (context: Context) => {
		const last = [...context.messages].reverse().find((m) => m.role === "user");
		const content = typeof last?.content === "string" ? last.content
			: ((last?.content ?? []) as { type: string; text?: string }[]).filter((b) => b.type === "text").map((b) => b.text).join("\n");
		appendFileSync(process.env.FAUX_OUT!, JSON.stringify(content) + "\n");
	};
	const say = (context: Context, message: ReturnType<typeof fauxAssistantMessage>) => (asked(context), message);
	if (process.env.FAUX_SCRIPT === "paint") {
		faux.setResponses([
			(c: Context) => say(c, fauxAssistantMessage(fauxToolCall("paint", { lua: CANVAS }))),
			(c: Context) => say(c, fauxAssistantMessage(fauxToolCall("paint", { lua: STROKE }))),
			(c: Context) => say(c, fauxAssistantMessage(fauxToolCall("look", {}))),
			(c: Context) => say(c, fauxAssistantMessage("**Tiny Test**\n\nA red stroke across a white ground.")),
		]);
	} else {
		const words = "x ".repeat(process.env.FAUX_SCRIPT === "chatty" ? 4000 : 100);
		const step = (c: Context) => {
			faux.appendResponses([step]);
			return say(c, fauxAssistantMessage([fauxText(words), fauxToolCall("status", {})]));
		};
		faux.setResponses([step]);
	}
	pi.registerProvider(faux.provider);
}
