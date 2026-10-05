/**
 * The painter's tools: the easel, and reading the studio's notes. No shell.
 *
 * Each tool runs the studio's `bin/easel` client (no shell, a bare environment but for RAYON_NUM_THREADS) and returns
 * what it printed. `look` hands the PNG the easel wrote to pi's own read tool, so the image
 * reaches the model exactly as a read image does (resized to the model's limits). `read` is
 * pi's read tool, refused outside the studio folder and given the checked path.
 *
 * The easel is opened by the runner before the painter starts and stays open across sittings;
 * if it isn't (a first run, a crash), the first tool call opens it, which replays the log.
 */
import { existsSync } from "node:fs";
import { Type } from "@earendil-works/pi-ai";
import { createReadToolDefinition, defineTool, type ExtensionAPI } from "@earendil-works/pi-coding-agent";
import { reviseJournal } from "./journal.ts";
import { atEasel, hideCounters, logReply, paintReply, renameLook, renameLooks, statusReply, studioPath, lookArgs, tail, text, toolWords, surveyReply } from "./easel-client.ts";
import type { PruneLimits } from "./context-images.ts";

export function registerEaselTools(pi: ExtensionAPI, studio: string, limits: PruneLimits): void {
	const read = createReadToolDefinition(studio);

	pi.registerTool({
		...defineTool({
			name: "paint",
			label: "paint",
			description:
				"Run a chunk of Lua at the easel (notes/easel_guide.md). The reply is what the chunk printed, the painting's current clock, then `ok`. " +
				"A chunk that stops with an error changes nothing.",
			parameters: Type.Object({ lua: Type.String({ description: "the chunk" }) }),
			async execute(_id, p, signal) {
				try {
					return text(paintReply(await atEasel(studio, ["do", "-"], p.lua, signal)));
				} catch (e) {
					throw new Error(hideCounters((e as Error).message));
				}
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
				"mode: \"value\", \"squint\", \"mirror\", \"relief\" (a raking light on the paint's ridges and furrows), \"gallery\" (the light the picture hangs in) or several, comma-separated. " +
				"light: \"azimuth,elevation\" in degrees for the relief light (default \"135,25\", from the upper left). size: the long side in pixels. " +
				"grid: true, or a spacing in canvas units. " +
				"survey: true surveys the whole canvas at full detail, as several tiles (with mode, not crop or size); a partial reply lists remaining tiles to read in separate turns. " +
				"compare: the path of an earlier look, shown left of the current view; supply matching crop, mode and light options explicitly. " +
				"ref: the path of a picture of the motif in this studio (a photograph, a study), fitted to the canvas's shape and shown left of the same view of the canvas: the same crop, size, mode and grid, as a picture pinned beside the easel. " +
				"hold: the name of a knife (what is on it) or a pile (a fresh load), with at: \"x,y\" (canvas units): the loaded knife held up to the canvas there, its paint thick on the blade (crop sets the passage). Supports mode value, squint, relief or gallery and light; size, grid and mirror are unavailable with hold. It shows the paint on the knife, not how it would look laid. " +
				"hold can also name a pastel stick (a field of a table too: \"P.glow\"), with at, and pose: \"force,alt,azimuth\" (N, degrees) or side: \"<direction in degrees>\" (laid flat across a stroke going that way): the stick held there seen from above, where it rests on the tooth at that force in its colour and where its crumbs would settle tinted. " +
				"palette: true shows the palette board instead: each pile knifed out thick and smeared thin across a black stripe.",
			parameters: Type.Object({
				crop: Type.Optional(Type.String()),
				mode: Type.Optional(Type.String()),
				light: Type.Optional(Type.String()),
				survey: Type.Optional(Type.Boolean()),
				compare: Type.Optional(Type.String()),
				ref: Type.Optional(Type.String()),
				hold: Type.Optional(Type.String()),
				at: Type.Optional(Type.String()),
				pose: Type.Optional(Type.String()),
				side: Type.Optional(Type.String()),
				palette: Type.Optional(Type.Boolean()),
				size: Type.Optional(Type.Number()),
				grid: Type.Optional(Type.Union([Type.Boolean(), Type.Number()])),
			}),
			async execute(id, p, signal, onUpdate, ctx) {
				let said: string;
				lookArgs(p); // validate combinations before resolving compare paths
				if (p.survey && p.compare) throw new Error("look: survey and compare are two looks; ask for one");
				if (p.ref && (p.survey || p.compare || p.hold)) throw new Error("look: ref is a look of its own: no survey, compare or hold");
				// ref: a picture of the motif in this studio, nothing outside it (as `read`)
				let ref = p.ref || undefined;
				if (ref !== undefined) {
					ref = studioPath(studio, ref);
					if (ref === undefined) throw new Error("look: ref is the path of a picture of the motif in this studio");
				}
				// compare: an earlier look of this studio, nothing outside it (as `read`)
				let compare = p.compare || undefined; // (an empty path is none)
				if (compare !== undefined) {
					compare = studioPath(studio, compare);
					if (compare === undefined) throw new Error("look: compare is the path of an earlier look in this studio");
				}
				try {
					said = await atEasel(studio, ["look", ...lookArgs({ ...p, compare, ref })], undefined, signal);
				} catch (e) {
					throw new Error(toolWords((e as Error).message)); // the easel's messages name its command-line flags
				}
				let paths: string[];
				({ said, paths } = renameLooks(studio, said));
				if (paths.length === 0) throw new Error(said);
				// (a survey names several looks: each is read, in order)
				const reads = [];
				for (const path of paths) reads.push(await read.execute(id, { path }, signal, onUpdate, ctx));
				const images = reads.flatMap((r) => r.content.filter((c) => c.type === "image"));
				if (p.survey) return { ...reads[0], ...surveyReply(said, paths, images, limits) };
				return { ...reads[0], content: [{ type: "text" as const, text: said }, ...images] };
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "note",
			label: "note",
			description: "Add an entry to your journal, notes/journal.md, stamped with the painting's time. " +
				"To revise what is already there, give the exact passage to change as `replaces`: `text` takes its place.",
			parameters: Type.Object({ text: Type.String(), replaces: Type.Optional(Type.String()) }),
			async execute(_id, p, signal) {
				if (p.replaces !== undefined) return text(reviseJournal(studio, p.replaces, p.text));
				return text(await atEasel(studio, ["note", "-"], p.text, signal));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "status",
			label: "status",
			description: "The canvas's setup.",
			parameters: Type.Object({}),
			async execute(_id, _p, signal) {
				return text(statusReply(await atEasel(studio, ["status"], undefined, signal)));
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
				return text(tail(logReply(await atEasel(studio, ["log"], undefined, signal))));
			},
		}),
		executionMode: "sequential",
	});

	// read gets the path that was checked, so it can't resolve the painter's spelling to another file
	pi.registerTool({
		...read,
		async execute(id, p, signal, onUpdate, ctx) {
			const real = studioPath(studio, p.path);
			if (!real) throw new Error(`${p.path} is outside the studio`);
			if (!existsSync(real)) throw new Error(`${p.path}: no such file in the studio`);
			return read.execute(id, { ...p, path: real }, signal, onUpdate, ctx);
		},
		executionMode: "sequential",
	});
}
