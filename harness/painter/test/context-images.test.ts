// node --test harness/painter/test/context-images.test.ts
import assert from "node:assert/strict";
import { test } from "node:test";
import { imagesToDrop, pruneImages } from "../context-images.ts";
import { surveyReply } from "../easel-client.ts";

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

test("the newest image is kept when it fits the size limit, and dropped when it alone is over it", () => {
	assert.equal(imagesToDrop([12_000_000], L), 0);
	assert.equal(imagesToDrop([20_000_000, 1], L), 1);
	assert.equal(imagesToDrop([20_000_000], L), 1);
	assert.equal(imagesToDrop([1, 20_000_000], L), 2);
});

test("an image alone over the size limit leaves the request, and a line says so and why", () => {
	const msgs = [{ role: "toolResult", content: [{ type: "text", text: "look-0001.png" }, { type: "image", data: "A".repeat(13_460_000), mimeType: "image/png" }] }];
	const r = pruneImages(msgs, { maxImages: 1, maxImageChars: 12_000_000, step: 5 });
	assert.equal(r.dropped, 1);
	assert.equal(r.keptChars, 0);
	assert.equal(r.messages[0].content[1].text,
		"[this image was left out of the request: it is 13.5 MB, over the 12 MB limit for the images in one request]");
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
		{ type: "text", text: "[You saw this image earlier (out/easel/painting/look-0001.png); removed from this request to save space. Use read to reopen it within the studio, or look for the current canvas.]" },
	]);
	assert.equal(results[4].content[1].text, "[You saw this image earlier (out/easel/painting/look-0005.png); removed from this request to save space. Use read to reopen it within the studio, or look for the current canvas.]");
	assert.equal(results[5].content[1].type, "image");
	assert.equal(results[22].content[1].type, "image");
	assert.equal(r.messages[results.length], msgs[results.length], "unchanged messages are the same objects");
});

test("an image without a matching read call gets a generic line", () => {
	const msgs = session(21).map((m) => (m.role === "assistant" ? { ...m, content: [] } : m));
	const r = pruneImages(msgs, L);
	assert.equal(r.messages[2].content[1].text, "[You saw this image earlier; removed from this request to save space. Call look for the current canvas.]");
});

test("an image from the look tool gets a line saying it was a look", () => {
	const msgs = session(21).map((m) => (m.role === "assistant"
		? { ...m, content: m.content.map((c: any) => ({ ...c, name: "look", arguments: { crop: "0,0,100,100" } })) }
		: m));
	const r = pruneImages(msgs, L);
	assert.equal(r.messages[2].content[1].text, "[You saw this image when you called look earlier; removed from this request to save space. Call look again for the current canvas.]");
});

test("the same messages give the same request", () => {
	const a = pruneImages(session(40), L);
	const b = pruneImages(session(40), L);
	assert.equal(JSON.stringify(a.messages), JSON.stringify(b.messages));
});

test("a fresh multi-image look that fits is delivered whole despite step rounding", () => {
	const msgs = session(3);
	const tiles = Array.from({ length: 8 }, (_, i) => ({ type: "image", data: String(i).repeat(10), mimeType: "image/png" }));
	msgs.push({ role: "toolResult", toolName: "look", content: tiles });
	for (const limits of [
		{ maxImages: 8, maxImageChars: 1000, step: 5 },
		{ maxImages: 20, maxImageChars: 80, step: 5 },
	]) {
		const r = pruneImages(msgs, limits);
		assert.deepEqual(r.messages.at(-1).content, tiles);
		assert.equal(r.dropped, 3);
		assert.ok(r.images - r.dropped <= limits.maxImages);
		assert.ok(r.keptChars <= limits.maxImageChars);
	}
	// Oversized groups still obey the hard limits.
	const small = pruneImages(msgs, { maxImages: 2, maxImageChars: 15, step: 5 });
	assert.ok(small.images - small.dropped <= 2);
	assert.ok(small.keptChars <= 15);
});

test("a survey larger than the request budget delivers a complete first batch and names every unread tile", () => {
	const paths = Array.from({ length: 20 }, (_, i) => `out/tile-${i + 1}.png`);
	const tiles = paths.map((_, i) => ({ type: "image" as const, data: String(i % 10).repeat(750_000), mimeType: "image/png" }));
	for (const [limits, expectedCount] of [
		[L, 16],
		[{ maxImages: 8, maxImageChars: 12_000_000, step: 5 }, 8],
		[{ maxImages: 8, maxImageChars: 1_500_000, step: 5 }, 2],
	] as const) {
		const reply = surveyReply("survey: 20 tiles\n", paths, tiles, limits);
		const r = pruneImages([...session(3), { role: "toolResult", toolName: "look", ...reply }], limits);
		const delivered = r.messages.at(-1).content;
		assert.deepEqual(delivered.filter((c: any) => c.type === "image"), tiles.slice(0, expectedCount));
		assert.match(delivered[0].text, /Partial survey/);
		assert.match(delivered[0].text, /separate turn/);
		assert.equal(delivered[0].text.split("before assessing the whole canvas:\n")[1], paths.slice(expectedCount).join("\n") + "\n");
		assert.ok(r.images - r.dropped <= limits.maxImages);
		assert.ok(r.keptChars <= limits.maxImageChars);
		// Every remaining original image fits on the next turn through the existing read tool.
		for (const tile of tiles.slice(expectedCount)) {
			const next = pruneImages([...r.messages, { role: "toolResult", toolName: "read", content: [tile] }], limits);
			assert.deepEqual(next.messages.at(-1).content, [tile]);
		}
	}
	assert.throws(() => surveyReply("survey", paths, tiles, { ...L, maxImageChars: 700_000 }), /survey incomplete: out\/tile-1.png alone exceeds/);
	const complete = surveyReply("survey: 2 tiles", paths.slice(0, 2), tiles.slice(0, 2), L);
	assert.deepEqual(complete.content, [{ type: "text", text: "survey: 2 tiles" }, ...tiles.slice(0, 2)]);
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
