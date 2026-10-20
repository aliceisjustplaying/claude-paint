import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { test } from "node:test";

test("sitting 0 has only read and write_concepts, which replaces the concepts file", () => {
	assert.ok(process.env.TMPDIR);
	const dir = mkdtempSync(join(process.env.TMPDIR, "concepts-test-"));
	const studio = join(dir, "studio");
	mkdirSync(join(studio, "notes"), { recursive: true });
	writeFileSync(join(studio, "notes/concepts.md"), "old text");
	const fixture = join(dir, "faux.ts");
	writeFileSync(fixture, `import { fauxAssistantMessage, fauxProvider } from "@earendil-works/pi-ai";
import { writeFileSync } from "node:fs";
export default function(pi) {
  pi.on("before_agent_start", () => writeFileSync(${JSON.stringify(join(dir, "tools.json"))}, JSON.stringify(pi.getActiveTools())));
  const faux = fauxProvider({ provider: "faux", models: [{ id: "m", input: ["text"], contextWindow: 200000 }] });
  faux.setResponses([
    () => ({ ...fauxAssistantMessage(""), content: [{ type: "toolCall", id: "save", name: "write_concepts", arguments: { text: "eight concepts" } }], stopReason: "toolUse" }),
    () => fauxAssistantMessage("done")
  ]);
  pi.registerProvider(faux.provider);
}`);
	const reply = execFileSync("pi", ["--print", "--no-extensions", "-e", join(import.meta.dirname, "../concepts.ts"),
		"-e", fixture, "--system-prompt", join(import.meta.dirname, "../system_prompt.md"), "--tools", "read,write_concepts",
		"--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve", "--session-dir", join(dir, "sessions"),
		"--provider", "faux", "--model", "m", "write concepts"],
		{ cwd: studio, env: { ...process.env, PAINTER_CRITIC: "", PAINTER_SITTING_RECOVERY: "" }, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
	assert.equal(reply.trim(), "done");
	assert.deepEqual(JSON.parse(readFileSync(join(dir, "tools.json"), "utf8")).sort(), ["read", "write_concepts"]);
	assert.equal(readFileSync(join(studio, "notes/concepts.md"), "utf8"), "eight concepts");
});

const CUT_SHORT = `
const { registerHooks } = await import("node:module");
registerHooks({
	resolve(specifier, context, nextResolve) {
		const source = specifier === "./painter.ts" ? "export default () => {};"
			: specifier === "@earendil-works/pi-ai" ? "export const Type = { Object: (x) => x, String: () => ({}) };"
			: specifier === "@earendil-works/pi-coding-agent" ? "export const defineTool = (x) => x;" : undefined;
		return source ? { url: \`data:text/javascript,\${encodeURIComponent(source)}\`, shortCircuit: true } : nextResolve(specifier, context);
	},
});
const { default: concepts } = await import(process.argv[1]);
const tools = [];
concepts({ registerTool: (tool) => tools.push(tool) });
try {
	await tools[0].execute("write", { text: "x".repeat(4096) });
} catch (error) {
	console.log(error.code);
}
`;

test("a write cut short by the file size limit leaves the previous concept draft in place", () => {
	assert.ok(process.env.TMPDIR);
	const dir = mkdtempSync(join(process.env.TMPDIR, "concepts-test-"));
	const studio = join(dir, "studio");
	mkdirSync(join(studio, "notes"), { recursive: true });
	writeFileSync(join(studio, "notes/concepts.md"), "old text");
	const reply = execFileSync("sh", ["-c", 'ulimit -f 1 && exec "$0" --input-type=module -e "$1" "$2"', process.execPath, CUT_SHORT, join(import.meta.dirname, "../concepts.ts")],
		{ cwd: studio, encoding: "utf8" });
	assert.equal(reply.trim(), "EFBIG");
	assert.equal(readFileSync(join(studio, "notes/concepts.md"), "utf8"), "old text");
	assert.deepEqual(readdirSync(join(studio, "notes")), ["concepts.md"]);
});
