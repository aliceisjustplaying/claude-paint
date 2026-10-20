import assert from "node:assert/strict";
import { chmodSync, writeFileSync } from "node:fs";
import painter from "../../painter.ts";

export default async function () {
	const handlers: Record<string, (...args: any[]) => any> = {};
	process.env.PAINTER_SITTING_RECOVERY = "1";
	painter({ on: (name, handler) => { handlers[name] = handler; }, registerTool() {}, appendEntry() {} } as never);
	const original = { type: "image", data: "original", mimeType: "image/png" };
	const opening = await handlers.input({ text: "Back at the easel.", images: [original] }, {});
	assert.match(opening.text, /Canvas image unavailable:/);
	assert.deepEqual(opening.images, [original]);
	const path = /The whole canvas as it was when this was written: (\S+\.png)/.exec(opening.text)![1];
	const summary = { role: "compactionSummary", summary: opening.text, timestamp: 1 };
	const event = { messages: [summary] };
	const failed = await handlers.context(event, {});
	assert.match(failed.messages[1].content[0].text, /Canvas image unavailable:/);
	assert.equal(failed.messages[1].content.length, 1);
	assert.deepEqual(event.messages, [summary]);
	chmodSync(path, 0o600);
	const retried = await handlers.context(event, {});
	assert.equal(retried.messages[1].content[1].type, "image");
	assert.doesNotMatch(retried.messages[1].content[0].text, /unavailable/);
	chmodSync(path, 0o000);
	const cached = await handlers.context(event, {});
	assert.deepEqual(cached.messages[1].content, retried.messages[1].content);
	writeFileSync("recovery-hooks-ok", "passed");
}
