/**
 * The painter's tools: the easel, and reading the studio's notes. No shell.
 *
 * Each tool runs the studio's `bin/easel` client (no shell, a bare environment) and returns
 * what it printed. `look` hands the PNG the easel wrote to pi's own read tool, so the image
 * reaches the model exactly as a read image does (resized to the model's limits). `read` is
 * pi's read tool, refused outside the studio folder.
 *
 * The easel is opened by the runner before the painter starts and stays open across sittings;
 * if it isn't (a first run, a crash), the first tool call opens it, which replays the log.
 */
import { Type } from "@earendil-works/pi-ai";
import { createReadToolDefinition, defineTool, type ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { atEasel, inStudio, lookArgs, tail, text } from "./easel-client.ts";

export function registerEaselTools(pi: ExtensionAPI, studio: string): void {
	const read = createReadToolDefinition(studio);

	pi.registerTool({
		...defineTool({
			name: "paint",
			label: "paint",
			description:
				"Run a chunk of Lua at the easel (notes/easel_guide.md). The reply is what the chunk printed, then `ok · chunk N`. " +
				"A chunk that stops with an error changes nothing.",
			parameters: Type.Object({ lua: Type.String({ description: "the chunk" }) }),
			async execute(_id, p, signal) {
				return text(await atEasel(studio, ["do", "-"], p.lua, signal));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "look",
			label: "look",
			description:
				"Look at the canvas as it is now. Without options: the whole canvas, scaled down. " +
				"crop: \"x0,y0,x1,y1\" in canvas units (two opposite corners), shown at 1:1 pixels. " +
				"mode: \"value\", \"squint\", \"mirror\" or several, comma-separated. size: the long side in pixels. " +
				"grid: true, or a spacing in canvas units.",
			parameters: Type.Object({
				crop: Type.Optional(Type.String()),
				mode: Type.Optional(Type.String()),
				size: Type.Optional(Type.Number()),
				grid: Type.Optional(Type.Union([Type.Boolean(), Type.Number()])),
			}),
			async execute(id, p, signal, onUpdate, ctx) {
				const said = await atEasel(studio, ["look", ...lookArgs(p)], undefined, signal);
				const path = said.split("\n").pop()!.replace(/ \(.*\)$/, "");
				const img = await read.execute(id, { path }, signal, onUpdate, ctx);
				return { ...img, content: [{ type: "text" as const, text: said }, ...img.content.filter((c) => c.type === "image")] };
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "note",
			label: "note",
			description: "Add an entry to your journal, notes/journal.md, stamped with the painting's time.",
			parameters: Type.Object({ text: Type.String() }),
			async execute(_id, p, signal) {
				return text(await atEasel(studio, ["note", "-"], p.text, signal));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "status",
			label: "status",
			description: "The number of chunks, the canvas width in pixels and the canvas's setup.",
			parameters: Type.Object({}),
			async execute(_id, _p, signal) {
				return text(await atEasel(studio, ["status"], undefined, signal));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "log",
			label: "log",
			description: "The painting so far: every chunk that ran, in order (paintings/lua/painting.lua).",
			parameters: Type.Object({}),
			async execute(_id, _p, signal) {
				return text(tail(await atEasel(studio, ["log"], undefined, signal)));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({ ...read, executionMode: "sequential" });
	pi.on("tool_call", (event) => {
		if (event.toolName !== "read") return undefined;
		const path = String((event.input as { path?: unknown }).path ?? "");
		return inStudio(studio, path) ? undefined : { block: true, reason: `${path} is outside the studio` };
	});
}
