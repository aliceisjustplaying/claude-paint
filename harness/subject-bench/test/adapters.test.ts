import { test } from "node:test";
import assert from "node:assert/strict";
import { streamSimple } from "@earendil-works/pi-ai/api/openai-completions";

test("built-in OpenRouter output cap and price ceiling work without network", async () => {
	let calls = 0;
	const model: any = { id: "google/gemini-3.8-flash", name: "Offline probe", provider: "subject-openrouter",
		api: "openai-completions", baseUrl: "https://openrouter.ai/api/v1", reasoning: true, input: ["text", "image"],
		contextWindow: 131072, maxTokens: 2048, cost: { input: 0.75, output: 3.75, cacheRead: 0.075, cacheWrite: 0.75 } };
	const result = await streamSimple(model, { messages: [{ role: "user", content: "offline probe", timestamp: 0 }] }, {
		apiKey: "offline", maxRetries: 0,
		onPayload(payload: any) {
			assert.equal(payload.max_tokens ?? payload.max_completion_tokens, 2048);
			return { ...payload, provider: { max_price: { prompt: 0.75, completion: 3.75 } } };
		},
		fetch: async (_url: any, init: any) => {
			calls++;
			assert.deepEqual(JSON.parse(init.body).provider.max_price, { prompt: 0.75, completion: 3.75 });
			const chunks = [
				{ id: "offline", choices: [{ index: 0, delta: { role: "assistant", content: "Plan: a blue bowl." }, finish_reason: null }] },
				{ id: "offline", choices: [{ index: 0, delta: {}, finish_reason: "stop" }], usage: { prompt_tokens: 100, completion_tokens: 10, total_tokens: 110 } }
			];
			return new Response(chunks.map(c => `data: ${JSON.stringify(c)}\n\n`).join("") + "data: [DONE]\n\n",
				{ headers: { "content-type": "text/event-stream" } });
		}
	}).result();
	assert.equal(result.stopReason, "stop", result.errorMessage);
	assert.equal(calls, 1);
	assert.equal(result.usage.input, 100);
});
