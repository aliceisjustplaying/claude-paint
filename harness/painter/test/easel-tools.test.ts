import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { registerHooks } from "node:module";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";

const hooks = registerHooks({
	resolve(specifier, context, nextResolve) {
		const source = specifier === "@earendil-works/pi-ai" ? "export const Type = new Proxy({}, { get: () => () => ({}) });"
			: specifier === "@earendil-works/pi-coding-agent" ? "export const createReadToolDefinition = () => ({}); export const defineTool = x => x;" : undefined;
		return source ? { url: `data:text/javascript,${encodeURIComponent(source)}`, shortCircuit: true } : nextResolve(specifier, context);
	},
});
const { registerEaselTools } = await import("../easel-tools.ts");
hooks.deregister();

test("every reply from the painting or the scratch canvas says which canvas it came from", async () => {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(studio, "bin"));
	const path = "'" + (process.env.PATH ?? "/usr/bin:/bin").replaceAll("'", "'\\''") + "'";
	writeFileSync(join(studio, "bin", "easel"), `#!/bin/sh
PATH=${path}
case "$1" in
  status) echo "test canvas" ;;
  do) cat >/dev/null; printf 'day 1, 09:00\\nok \\302\\267 chunk 2\\n' ;;
  probe) echo "at 1,2" ;;
  log) printf -- '--@ chunk 1\\ncanvas{}\\n' ;;
esac
`, { mode: 0o755 });
	const tools: Record<string, { execute: (...args: unknown[]) => Promise<{ content: { text: string }[] }> }> = {};
	registerEaselTools({ registerTool: (t: { name: string }) => { tools[t.name] = t as never; } } as never, studio, {} as never);
	try {
		for (const [scratch, label] of [[false, "[painting]"], [true, "[scratch canvas]"]] as const) {
			const said = async (name: string, p: object) => (await tools[name].execute("id", { ...p, scratch })).content[0].text;
			assert.equal(await said("paint", { lua: "print(1)" }), `${label}\nday 1, 09:00\nok`);
			assert.equal(await said("probe", { at: "1,2" }), `${label}\nat 1,2`);
			assert.equal(await said("status", {}), `${label}\ntest canvas`);
			assert.equal(await said("log", {}), `${label}\n--@ chunk\ncanvas{}`);
		}
	} finally {
		rmSync(studio, { recursive: true, force: true });
	}
});
