import { createAssistantMessageEventStream } from "@earendil-works/pi-ai";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { readFileSync } from "node:fs";
import { join } from "node:path";

export default function mock(pi: ExtensionAPI) {
	globalThis.fetch = () => { throw new Error("Offline stub forbids network access"); };
	const config = JSON.parse(readFileSync(join(process.env.SUBJECT_BENCH_RUN!, "control.json"), "utf8"));
	let turn = 0;
	pi.registerProvider("subject-mock", {
		baseUrl: "https://invalid.invalid", apiKey: "offline-stub", api: "subject-mock",
		models: [{ id: "stub", name: "Offline stub", reasoning: false, input: ["text", "image"],
			cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0 }, contextWindow: 131072, maxTokens: 2048 }],
		streamSimple(model, context, options) {
			const stream = createAssistantMessageEventStream();
			queueMicrotask(async () => {
				await options?.onPayload?.({ messages: context.messages }, model);
				const call = (lua: string, id: string) => ({ type: "toolCall", name: "paint", arguments: { lua }, id });
				let content: any[];
				if (turn++ === 0) content = [
					...(config.mock_case === "plan" ? [{ type: "text", text: "Plan: a lamp lighting a kitchen table.\nTitle: Supper." }] : []),
					{ type: "toolCall", name: "read", arguments: { path: "BRIEF.md" }, id: "brief" }
				];
				else content = [
					call('canvas{size=120, aspect=1.25, linen=15, ground={{pile={{"yellow ochre", 1}}, um=100, apply="knife"}}}', "one"),
					call('p = pile{{"ultramarine blue", 1}}\nb = brush("filbert", 30)\nb:load(p, 1)', "two"),
					call('b:stroke({{150,600},{450,300},{750,500}}, {pressure=0.8})', "three"),
					call('print("four")', "four"), call('error("fifth must never execute")', "five")
				];
				const message: any = { role: "assistant", content, api: model.api, provider: model.provider, model: model.id,
					usage: { input: 100, output: 50, cacheRead: 0, cacheWrite: 0, totalTokens: 150,
						cost: { input: 0, output: 0, cacheRead: 0, cacheWrite: 0, total: 0 } },
					stopReason: "toolUse", timestamp: Date.now() };
				stream.push({ type: "start", partial: message }); stream.push({ type: "done", reason: "toolUse", message }); stream.end(message);
			});
			return stream;
		}
	});
}
