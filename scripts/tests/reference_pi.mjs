import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { registerEaselTools } from "../../harness/painter/easel-tools.ts";
import { LIMITS } from "../../harness/painter/context-images.ts";

const studio = process.argv[2];
assert.ok(studio, "pass an exported studio path");
const definitions = [];
registerEaselTools({ registerTool: (tool) => definitions.push(tool) }, studio, LIMITS);
const tool = definitions.find((tool) => tool.name === "reference");
const mcp = JSON.parse(readFileSync(new URL("../../harness/easel_mcp/tools.json", import.meta.url))).find((tool) => tool.name === "reference");
assert.equal(tool.description, mcp.description);
assert.deepEqual(JSON.parse(JSON.stringify(tool.parameters)), mcp.inputSchema);
assert.doesNotMatch(tool.description, /tree|oak|friedrich/i);
const call = async (params) => {
	const reply = await tool.execute("proof", params);
	assert.ok(Buffer.byteLength(reply.content[0].text) < 8000);
	return JSON.parse(reply.content[0].text);
};
for (const [query, expected] of [["branch taper", "tree-structure"], ["wet sand", "water"], ["fruit cloth", "fruit-cloth"], ["fluorescent", "artificial-light"], ["glazing dry", "oil-glazes"]]) {
	assert.ok((await call({ query })).matches.some((match) => match.id === expected));
}
assert.deepEqual(await call({ query: "unfindable-qzjx" }), { matches: [], message: "No matching references." });
const library = JSON.parse(readFileSync(join(studio, "notes/reference-library.json")));
const listTool = definitions.find((tool) => tool.name === "reference_list");
const listSchema = JSON.parse(readFileSync(new URL("../../harness/easel_mcp/tools.json", import.meta.url))).find((tool) => tool.name === "reference_list");
assert.equal(listTool.description, listSchema.description);
assert.deepEqual(JSON.parse(JSON.stringify(listTool.parameters)), listSchema.inputSchema);
assert.doesNotMatch(listTool.description, /tree|oak|friedrich/i);
const listed = (await listTool.execute("proof", {})).content[0].text;
assert.ok(Buffer.byteLength(listed) < 8000);
assert.deepEqual(JSON.parse(listed), { references: library.map(({ id, title }) => ({ id, title })) });
for (const entry of library) {
	let text = "", page = 1;
	while (page !== null) {
		const result = await call({ id: entry.id, page });
		text += result.text;
		page = result.next_page;
	}
	assert.equal(text, entry.text);
}
console.log(`pi reference: ${library.length} titles listed and full texts retrieved; tree, non-tree, no-match and schema checks passed`);
