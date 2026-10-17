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
import { writeNotebook } from "./notebook.ts";
import { reference, referenceList } from "./references.ts";
import { atEasel, directoryListing, hideCounters, logFind, logReply, paintReply, probeArgs, renameLook, renameLooks, statusReply, studioPath, lookArgs, tail, text, toolWords, surveyReply } from "./easel-client.ts";
import type { PruneLimits } from "./context-images.ts";

const FIND =
	"Text to look for, in any case. Only the lines that hold it come back, each after its line number in the log's file and a colon, " +
	"with the two lines before and after it, numbered with a dash.";

const scratch = Type.Optional(
	Type.Boolean({
		description:
			"true: the scratch canvas beside the painting instead of the painting. It is set up as the painting's canvas was, with its own palette, brushes and variables, and its own log; nothing done there reaches the painting. It keeps its own clock: time spent or waited there doesn't pass for the painting.",
	}),
);
const on = (p: { scratch?: boolean }) => (p.scratch ? ["--scratch"] : []);
const tag = (p: { scratch?: boolean }, t: string) => (p.scratch ? `[scratch canvas]\n${t}` : t);

export function registerEaselTools(pi: ExtensionAPI, studio: string, limits: PruneLimits): void {
	const read = createReadToolDefinition(studio);
	pi.registerTool({
		...defineTool({
			name: "reference_list",
			label: "reference_list",
			description: "List the studio's available references after choosing a subject or technical question. Read an entry with reference using its id.",
			parameters: Type.Object({}, { additionalProperties: false, required: [] }),
			async execute(_id, p) {
				return text(JSON.stringify(referenceList(studio, p)));
			},
		}),
		executionMode: "sequential",
	});
	pi.registerTool({
		...defineTool({
			name: "reference",
			label: "reference",
			description: "Search the studio's reference library after choosing a subject or technical question. Give query to find notes, then id to read a result. Text comes in pages; follow next_page until it is null.",
			parameters: Type.Object({
				query: Type.Optional(Type.String({ description: "Words to find in the reference text." })),
				id: Type.Optional(Type.String({ description: "The id of a search result to read." })),
				page: Type.Optional(Type.Integer({ minimum: 1, description: "The page to read, starting at 1." })),
			}, { additionalProperties: false, required: [] }),
			async execute(_id, p) {
				return text(JSON.stringify(reference(studio, p)));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "paint",
			label: "paint",
			description:
				"Run a chunk of Lua at the easel (notes/easel_guide.md). The reply is what the chunk printed, the painting's current clock, then `ok`. " +
				"A chunk that stops with an error changes nothing.",
			parameters: Type.Object({
				lua: Type.String({ description: "the chunk" }),
				scratch,
				new_scratch: Type.Optional(Type.Boolean({ description: "with scratch: put the scratch canvas aside and start a fresh one before this chunk" })),
			}),
			async execute(_id, p, signal) {
				if (p.new_scratch && !p.scratch) throw new Error("paint: new_scratch goes with scratch: true");
				try {
					return text(tag(p, paintReply(await atEasel(studio, ["do", "-", ...on(p), ...(p.new_scratch ? ["--new"] : [])], p.lua, signal))));
				} catch (e) {
					throw new Error(hideCounters((e as Error).message));
				}
			},
		}),
		executionMode: "sequential",
	});

	registerLook(pi, studio, limits);

	pi.registerTool({
		...defineTool({
			name: "probe",
			label: "probe",
			description:
				"The paint on the canvas as it is now, in numbers. at: \"x,y\" in canvas units, a point; or crop: \"x0,y0,x1,y1\" (two opposite corners), the average over that area. " +
				"The reply gives its color as red, green and blue from 0 to 255, as a look shows it, and its value from 0 (black) to 255 (white), the gray mode \"value\" shows; " +
				"the same two in the gallery's light, as mode \"gallery\" shows it; and whether the paint there is open, setting, tacky or dry (over an area, the share of each). " +
				"It doesn't say which piles the paint came from: the canvas keeps the color, not the piles.",
			parameters: Type.Object({ scratch, at: Type.Optional(Type.String()), crop: Type.Optional(Type.String()) }),
			async execute(_id, p, signal) {
				const where = probeArgs(p);
				try {
					return text(tag(p, await atEasel(studio, ["probe", ...on(p), ...where], undefined, signal)));
				} catch (e) {
					throw new Error(toolWords((e as Error).message));
				}
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

	// the atelier's painters only (atelier/run.py lists it in --tools): notebook.md carries over between paintings
	pi.registerTool({
		...defineTool({
			name: "notebook",
			label: "notebook",
			description: "Add to your notebook, notebook.md. It carries over from one painting to the next. " +
				"To revise what is already there, give the exact passage to change as `replaces`: `text` takes its place.",
			parameters: Type.Object({ text: Type.String(), replaces: Type.Optional(Type.String()) }),
			async execute(_id, p) {
				return text(writeNotebook(studio, p.text, p.replaces));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "status",
			label: "status",
			description: "The canvas's setup. While a chunk is still running, how far it has got; while the easel rebuilds the painting from its log, how many of its chunks are done.",
			parameters: Type.Object({ scratch }),
			async execute(_id, p, signal) {
				return text(tag(p, statusReply(await atEasel(studio, ["status", ...on(p)], undefined, signal))));
			},
		}),
		executionMode: "sequential",
	});

	pi.registerTool({
		...defineTool({
			name: "log",
			label: "log",
			description: "The painting so far: every chunk that ran, in order (paintings/lua/painting.lua).",
			parameters: Type.Object({ scratch, find: Type.Optional(Type.String({ description: FIND })) }),
			async execute(_id, p, signal) {
				const whole = p.scratch ? "paintings/lua/scratch.lua" : "paintings/lua/painting.lua";
				const log = logReply(await atEasel(studio, ["log", ...on(p)], undefined, signal));
				return text(tag(p, p.find ? logFind(log, p.find, whole) : tail(log, 50_000, whole)));
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
			const listing = directoryListing(real);
			if (listing !== undefined) return text(listing);
			return read.execute(id, { ...p, path: real }, signal, onUpdate, ctx);
		},
		executionMode: "sequential",
	});
}

/** `look`, as the painter has it; the critic's session (critic.ts) has the same tool. */
export function registerLook(pi: ExtensionAPI, studio: string, limits: PruneLimits): void {
	const read = createReadToolDefinition(studio);
	pi.registerTool({
		...defineTool({
			name: "look",
			label: "look",
			description:
				"Look at the canvas as it is now. Without options: the whole canvas, scaled down. " +
				"crop: \"x0,y0,x1,y1\" in canvas units (two opposite corners), shown at full detail. " +
				"mode: \"value\" (the picture in grays, its lightness alone, in the gallery light), \"squint\", \"edges\" (the picture's edges drawn over a dim gray of it: orange where an edge is hard, blue where it is soft, brighter the more the lightness changes across it; hard and soft as they read at the size of this image), \"mirror\", \"relief\" (a raking light on the paint's ridges and furrows), \"gallery\" (the light the picture hangs in) or several, comma-separated. " +
				"light: \"azimuth,elevation\" in degrees for the relief light (default \"135,25\", from the upper left). size: the long side in pixels. " +
				"grid: true, or a spacing in canvas units. " +
				"survey: true surveys the whole canvas at full detail, as several tiles (with mode, not crop or size); a partial reply lists remaining tiles to read in separate turns. " +
				"compare: the path of an earlier look, shown left of the current view; supply matching crop, mode and light options explicitly. " +
				"ref: the path of a picture of the motif in this studio (a photograph, a study), fitted to the canvas's shape (scaled to cover it and centered, so a picture of another shape loses its edges) and shown left of the same view of the canvas: the same crop, size, mode and grid, as a picture pinned beside the easel. " +
				"hold: the name of a knife (what is on it) or a pile (a fresh load), with at: \"x,y\" (canvas units): the loaded knife held up to the canvas there, its paint thick on the blade (crop sets the passage). Supports mode value, squint, relief or gallery and light; size, grid and mirror are unavailable with hold. It shows the paint on the knife, not how it would look laid. " +
				"palette: true shows the palette board instead: each pile knifed out thick and smeared thin across a black stripe.",
			parameters: Type.Object({
				scratch,
				crop: Type.Optional(Type.String()),
				mode: Type.Optional(Type.String()),
				light: Type.Optional(Type.String()),
				survey: Type.Optional(Type.Boolean()),
				compare: Type.Optional(Type.String()),
				ref: Type.Optional(Type.String()),
				hold: Type.Optional(Type.String()),
				at: Type.Optional(Type.String()),
				palette: Type.Optional(Type.Boolean()),
				size: Type.Optional(Type.Number()),
				grid: Type.Optional(Type.Union([Type.Boolean(), Type.Number()])),
			}),
			async execute(id, p, signal, onUpdate, ctx) {
				const { scratch: onScratch, ...view } = p;
				let said: string;
				lookArgs(view); // validate combinations before resolving compare paths
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
					said = await atEasel(studio, ["look", ...on(p), ...lookArgs({ ...view, compare, ref })], undefined, signal);
				} catch (e) {
					throw new Error(toolWords((e as Error).message)); // the easel's messages name its command-line flags
				}
				let paths: string[];
				({ said, paths } = renameLooks(studio, said));
				if (onScratch) said = `[scratch canvas]\n${said}`;
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
}
