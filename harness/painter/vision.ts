/**
 * Every painter sees its looks at its provider's best image resolution.
 *
 * Providers shrink images inside the model, each its own way, by default:
 *  - OpenAI (Responses API, as openai-codex): pi sends `detail: "auto"`. We send PAINTER_OPENAI_DETAIL
 *    (default "high": a 2,500-patch budget, about 1600 x 1600 px, so a look of at most 1600 px
 *    arrives whole).
 *  - Google (Gemini 3): pi sends no media resolution, so each image gets the default 1120-token
 *    budget. We set `media_resolution_ultra_high` (2240 tokens) on every image part; that level
 *    exists only per part.
 *  - Anthropic: nothing to set; recent Claude models keep images up to 2576 px on the long edge.
 * Applied to the request payload just before it goes out (pi's before_provider_request).
 */
import type { ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { sharpen } from "./vision-payload.ts";

export default function vision(pi: ExtensionAPI) {
	const detail = process.env.PAINTER_OPENAI_DETAIL || "high";
	delete process.env.PAINTER_OPENAI_DETAIL;
	pi.on("before_provider_request", (event) => sharpen(event.payload, detail));
}
