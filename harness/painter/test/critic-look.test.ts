import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import critic from "../critic.ts";
import { criticReadable } from "../visit.ts";

const PNG = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

function setup() {
	const studio = mkdtempSync(join(tmpdir(), "critic-look-"));
	for (const dir of ["bin", "notes", "out/easel/painting"]) mkdirSync(join(studio, dir), { recursive: true });
	writeFileSync(join(studio, "private.png"), Buffer.from(PNG, "base64"));
	writeFileSync(join(studio, "out/easel/painting/prior.png"), Buffer.from(PNG, "base64"));
	writeFileSync(join(studio, "bin/easel"), `#!${process.execPath}
const fs = require('node:fs');
fs.appendFileSync('calls', JSON.stringify(process.argv.slice(2)) + '\\n');
if (process.argv[2] === 'status') console.log('1 chunks');
else if (process.argv[2] === 'look') {
  fs.copyFileSync('private.png', 'out/easel/painting/fresh.png');
  console.log('out/easel/painting/fresh.png (1x1)');
}
`, { mode: 0o755 });
	const tools: Record<string, any> = {};
	const cwd = process.cwd();
	try {
		process.chdir(studio);
		critic({ on() {}, registerTool(tool: any) { tools[tool.name] = tool; } } as never);
	} finally {
		process.chdir(cwd);
	}
	return { studio, look: tools.look };
}

test("the critic's look refuses images outside its read allowlist before calling the easel", async () => {
	const { studio, look } = setup();
	assert.equal(criticReadable(studio, "private.png"), undefined);
	await assert.rejects(look.execute("id", { ref: "private.png" }), /look:/);
	await assert.rejects(look.execute("id", { compare: "private.png" }), /private.png/);
	const outside = mkdtempSync(join(tmpdir(), "critic-private-"));
	writeFileSync(join(outside, "secret.png"), Buffer.from(PNG, "base64"));
	symlinkSync(join(outside, "secret.png"), join(studio, "out/easel/painting/alias.png"));
	await assert.rejects(look.execute("id", { compare: "out/easel/painting/alias.png" }), /alias.png/);
	assert.throws(() => readFileSync(join(studio, "calls")), /ENOENT/);
	for (const params of [{}, { crop: "0,0,1,1", mode: "value" }, { compare: "out/easel/painting/prior.png" }]) {
		const result = await look.execute("id", params);
		assert.equal(result.content.filter((block: any) => block.type === "image").length, 1);
	}
});

test("the critic is not offered scratch, masks, held paint or the palette", async () => {
	const { studio, look } = setup();
	assert.deepEqual(Object.keys(look.parameters.properties).sort(), ["compare", "crop", "grid", "light", "mode", "size", "survey"]);
	for (const params of [{ scratch: true }, { mask: "a" }, { hold: "a", at: "1,1" }, { palette: true }, { pose: "flat" }, { side: "front" }]) {
		await assert.rejects(look.execute("id", params));
	}
	assert.throws(() => readFileSync(join(studio, "calls")), /ENOENT/);
});
