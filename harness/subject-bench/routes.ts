import { readFileSync } from "node:fs";
import { join } from "node:path";
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";

export default function routes(pi: ExtensionAPI) {
	const { model } = JSON.parse(readFileSync(join(process.env.SUBJECT_BENCH_RUN!, "control.json"), "utf8"));
	if (model.route === "codex") return;
	pi.registerProvider(model.provider, {
		baseUrl: model.base_url, apiKey: "$" + model.key_env, api: model.api,
		headers: model.route === "go" ? { "x-opencode-session": process.env.SUBJECT_BENCH_SESSION! } : undefined,
		models: [{ id: model.model, name: model.model, reasoning: model.reasoning, input: ["text", "image"],
			contextWindow: model.contextWindow, maxTokens: model.maxTokens, cost: model.cost }]
	});
	if (model.route === "openrouter") pi.on("before_provider_request", event => ({
		...event.payload as object, provider: { max_price: { prompt: model.cost.input, completion: model.cost.output } }
	}));
}
