import assert from "node:assert/strict";
import { test } from "node:test";
import { GEMINI_LEVEL, sharpen } from "../vision-payload.ts";

const img = { inlineData: { mimeType: "image/png", data: "AAAA" } };

test("a Gemini tool result's image moves to a user turn after it, at the ultra-high resolution", () => {
	const payload = {
		contents: [
			{ role: "model", parts: [{ functionCall: { name: "look", args: {} } }] },
			{ role: "user", parts: [{ functionResponse: { name: "look", response: { output: "look-0001.png" }, parts: [img] } }] },
			{ role: "user", parts: [{ text: "hi" }, img] },
		],
	};
	const out = sharpen(payload, "high");
	assert.equal(out.contents.length, 4);
	assert.equal(out.contents[1].parts[0].functionResponse.parts, undefined); // the field Gemini refuses there is gone
	assert.deepEqual(out.contents[2], {
		role: "user",
		parts: [{ text: "Tool result image:" }, { ...img, mediaResolution: { level: GEMINI_LEVEL } }],
	});
	assert.deepEqual(out.contents[3].parts[1].mediaResolution, { level: GEMINI_LEVEL }); // a plain image part: in place
});

test("every OpenAI input_image gets the detail, wherever it sits", () => {
	const payload = { input: [{ type: "function_call_output", output: [{ type: "input_image", detail: "auto", image_url: "data:" }] },
		{ role: "user", content: [{ type: "input_image", detail: "auto", image_url: "data:" }] }] };
	const out = sharpen(payload, "high") as any;
	assert.equal(out.input[0].output[0].detail, "high");
	assert.equal(out.input[1].content[0].detail, "high");
});

test("an Anthropic payload is left as it is", () => {
	const payload = { messages: [{ role: "user", content: [{ type: "image", source: { type: "base64", media_type: "image/png", data: "AAAA" } }] }] };
	const before = JSON.stringify(payload);
	assert.equal(JSON.stringify(sharpen(payload, "high")), before);
});
