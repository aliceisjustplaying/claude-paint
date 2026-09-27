// node --test harness/painter/test/context-images.test.ts
import assert from "node:assert/strict";
import { test } from "node:test";
import { imagesToDrop, pruneImages } from "../context-images.ts";

const L = { maxImages: 20, maxImageChars: 12_000_000, step: 5 };

test("under both limits nothing is dropped", () => {
	assert.equal(imagesToDrop([], L), 0);
	assert.equal(imagesToDrop(Array(20).fill(500_000), L), 0);
});

test("the count limit drops in steps, so the cut moves once every step images", () => {
	const drops = [21, 22, 25, 26, 30, 31].map((n) => imagesToDrop(Array(n).fill(1000), L));
	assert.deepEqual(drops, [5, 5, 5, 10, 10, 15]);
});

test("the size limit drops enough old images (in steps) to fit", () => {
	// 74 looks of ~456 KB (the session that hit the 413): the count limit binds, 19 kept.
	assert.equal(imagesToDrop(Array(74).fill(456_000), L), 55);
	// 30 looks of 750 KB: the size limit binds; 12 MB holds 16, the step rounds that down to 15.
	const sizes = Array(30).fill(750_000);
	const drop = imagesToDrop(sizes, L);
	assert.equal(drop, 15);
	assert.ok(sizes.slice(drop).reduce((a, b) => a + b, 0) <= 12_000_000);
	// 18 big images: under the count, over the size
	assert.equal(imagesToDrop(Array(18).fill(750_000), L), 5);
});

test("the newest image is always kept, even alone over the size limit", () => {
	assert.equal(imagesToDrop([20_000_000], L), 0);
	assert.equal(imagesToDrop([1, 20_000_000], L), 1);
});

function session(n: number, chars = 10) {
	const msgs: any[] = [{ role: "user", content: [{ type: "text", text: "brief" }] }];
	for (let i = 1; i <= n; i++) {
		const path = `out/easel/painting/look-${String(i).padStart(4, "0")}.png`;
		msgs.push({ role: "assistant", content: [{ type: "toolCall", id: `t${i}`, name: "read", arguments: { path } }] });
		msgs.push({
			role: "toolResult", toolCallId: `t${i}`, toolName: "read",
			content: [{ type: "text", text: "Read image file [image/png]" }, { type: "image", data: "x".repeat(chars), mimeType: "image/png" }],
		});
	}
	return msgs;
}

test("old looks become a line naming the file; the newest stay; the input is untouched", () => {
	const msgs = session(23);
	const before = JSON.stringify(msgs);
	const r = pruneImages(msgs, L);
	assert.equal(JSON.stringify(msgs), before);
	assert.equal(r.images, 23);
	assert.equal(r.dropped, 5);
	const results = r.messages.filter((m) => m.role === "toolResult");
	assert.deepEqual(results[0].content, [
		{ type: "text", text: "Read image file [image/png]" },
		{ type: "text", text: "[an earlier look: out/easel/painting/look-0001.png]" },
	]);
	assert.equal(results[4].content[1].text, "[an earlier look: out/easel/painting/look-0005.png]");
	assert.equal(results[5].content[1].type, "image");
	assert.equal(results[22].content[1].type, "image");
	assert.equal(r.messages[results.length], msgs[results.length], "unchanged messages are the same objects");
});

test("an image without a matching read call gets a generic line", () => {
	const msgs = session(21).map((m) => (m.role === "assistant" ? { ...m, content: [] } : m));
	const r = pruneImages(msgs, L);
	assert.equal(r.messages[2].content[1].text, "[an earlier image]");
});

test("an image from the look tool gets a line saying it was a look", () => {
	const msgs = session(21).map((m) => (m.role === "assistant"
		? { ...m, content: m.content.map((c: any) => ({ ...c, name: "look", arguments: { crop: "0,0,100,100" } })) }
		: m));
	const r = pruneImages(msgs, L);
	assert.equal(r.messages[2].content[1].text, "[an earlier look]");
});

test("the same messages give the same request", () => {
	const a = pruneImages(session(40), L);
	const b = pruneImages(session(40), L);
	assert.equal(JSON.stringify(a.messages), JSON.stringify(b.messages));
});

test("the limits come from PAINTER_MAX_IMAGES and PAINTER_MAX_IMAGE_MB, else the defaults", async () => {
	const { limitsFromEnv, LIMITS } = await import("../context-images.ts");
	assert.deepEqual(limitsFromEnv({}), LIMITS);
	assert.deepEqual(limitsFromEnv({ PAINTER_MAX_IMAGES: "8" }), { ...LIMITS, maxImages: 8 });
	assert.deepEqual(limitsFromEnv({ PAINTER_MAX_IMAGE_MB: "4" }), { ...LIMITS, maxImageChars: 4_000_000 });
	assert.throws(() => limitsFromEnv({ PAINTER_MAX_IMAGES: "0" }));
	assert.throws(() => limitsFromEnv({ PAINTER_MAX_IMAGES: "lots" }));
	// 13 looks with 8 kept at most: 5 dropped, then 10 once there are 14
	const L8 = limitsFromEnv({ PAINTER_MAX_IMAGES: "8" });
	assert.equal(imagesToDrop(Array(13).fill(1000), L8), 5);
	assert.equal(imagesToDrop(Array(14).fill(1000), L8), 10);
});
