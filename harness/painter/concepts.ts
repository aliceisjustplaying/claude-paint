import { renameSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { Type } from "@earendil-works/pi-ai";
import { defineTool, type ExtensionAPI } from "@earendil-works/pi-coding-agent";
import painter from "./painter.ts";

export default function concepts(pi: ExtensionAPI) {
	painter(pi);
	pi.registerTool({
		...defineTool({
			name: "write_concepts",
			label: "write_concepts",
			description: "Replace notes/concepts.md with your sixteen numbered pictures and their estimates.",
			parameters: Type.Object({ text: Type.String() }),
			async execute(_id, p) {
				const dir = join(process.cwd(), "notes");
				const staged = join(dir, `.concepts.md.${process.pid}.tmp`);
				try {
					writeFileSync(staged, p.text);
					renameSync(staged, join(dir, "concepts.md"));
				} catch (error) {
					rmSync(staged, { force: true });
					throw error;
				}
				return { content: [{ type: "text" as const, text: "Saved notes/concepts.md." }], details: {} };
			},
		}),
		executionMode: "sequential",
	});
}
