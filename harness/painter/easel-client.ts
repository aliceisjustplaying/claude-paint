/**
 * The easel client and small helpers for the painter's tools (easel-tools.ts), without pi
 * imports so node --test can load them.
 */
import { spawn } from "node:child_process";
import { setTimeout as pause } from "node:timers/promises";
import { readdirSync, readFileSync, realpathSync, renameSync, statSync } from "node:fs";
import { randomUUID } from "node:crypto";
import { homedir } from "node:os";
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { fileURLToPath } from "node:url";
import type { PruneLimits } from "./context-images.ts";

export interface Ran {
	code: number | null;
	out: string;
	timedOut?: boolean;
}

/** Tool budgets include opening, except advancing log rebuilds get a separate stall budget.
 * A chunk may run 10 minutes (the easel's own limit).
 */
export const WAIT_MS = { do: 12 * 60_000, other: 3 * 60_000, rebuild: 30 * 60_000 };
export const WIDE_WAIT_MS = { do: 48 * 60_000, rebuild: 50 * 60_000 };

export function wideStudio(studio: string): boolean {
	try {
		const lines = readFileSync(join(studio, "paintings", "lua", "painting.lua"), "utf8").split("\n");
		let wide = false;
		for (const line of lines) {
			const head = line.trim();
			if (head.startsWith("--@ chunk")) break;
			if (head === "--@ sketch") return false;
			if (/^--@ width\s+4800$/.test(head)) wide = true;
		}
		return wide;
	} catch (error) {
		if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error;
	}
	try {
		return readFileSync(join(studio, "bin", "width"), "utf8").trim() === "4800";
	} catch {
		return false;
	}
}

/** Run the studio's easel client with `args`, `input` on stdin; stdout and stderr together, in order. */
export function easel(studio: string, args: string[], input?: string, signal?: AbortSignal, waitMs?: number): Promise<Ran> {
	return new Promise((done, fail) => {
		if (signal?.aborted) return fail(new Error("Operation aborted"));
		const env: Record<string, string> = { PATH: "/usr/bin:/bin", HOME: process.env.HOME ?? "" };
		// the operator's thread budget, as an easel the runner opened gets it
		if (process.env.RAYON_NUM_THREADS !== undefined) env.RAYON_NUM_THREADS = process.env.RAYON_NUM_THREADS;
		for (const key of ["EASEL_ASYNC_DISPLAY", "EASEL_DISPLAY_THREADS"]) {
			if (process.env[key] !== undefined) env[key] = process.env[key];
		}
		// its own process group: a timeout or an abort stops it and anything it started
		const p = spawn(join(studio, "bin", "easel"), args, { cwd: studio, env, stdio: ["pipe", "pipe", "pipe"], detached: true });
		const kill = () => {
			try {
				process.kill(-p.pid!, "SIGTERM");
			} catch {
				p.kill("SIGTERM");
			}
		};
		let timedOut = false;
		const timer = waitMs ? setTimeout(() => ((timedOut = true), kill()), waitMs) : undefined;
		signal?.addEventListener("abort", kill, { once: true });
		const end = () => (clearTimeout(timer), signal?.removeEventListener("abort", kill));
		let out = "";
		p.stdout.on("data", (d) => (out += d));
		p.stderr.on("data", (d) => (out += d));
		p.on("error", (e) => (end(), fail(e)));
		p.on("close", (code) => (end(), done({ code: timedOut ? null : code, out: timedOut ? "" : out, timedOut } as Ran)));
		// an easel that exits before reading all of the input (EPIPE): its output and exit
		// status, on close, are the answer
		p.stdin.on("error", () => {});
		p.stdin.end(input ?? "");
	});
}

/** One easel call of a tool's chain: within the tool's time left, and not after an abort. */
async function step(studio: string, args: string[], input: string | undefined, signal: AbortSignal | undefined, deadline: number): Promise<Ran> {
	const r = await easel(studio, args, input, signal, Math.max(1, deadline - Date.now()));
	if (signal?.aborted) throw new Error("Operation aborted");
	return r;
}

/** The easel's answer to `status` while it rebuilds the painting from its log: chunks done, of how many. */
const REPLAY = /^rebuilding from the log \((\d+) of (\d+) chunks\)$/;
/** `status` from an easel that is starting, before it has read the log (main.rs `STARTING`). */
const STARTING = "the easel is starting; retry in a moment";

/**
 * Open if needed, waiting for replay readiness. Returns whether a rebuild was awaited. With
 * `answerReplay` (the status tool), a replay's progress is the answer: it returns at once.
 */
export async function ensureOpen(studio: string, signal?: AbortSignal, deadline = Date.now() + WAIT_MS.other, answerReplay = false): Promise<boolean> {
	let r = await step(studio, ["status"], undefined, signal, deadline);
	let opened = false;
	let rebuilding = false;
	let completed = -1;
	let total: number | undefined;
	let progressDeadline = deadline;
	for (;;) {
		if (r.timedOut && opened && !rebuilding) {
			// an open that outlasts the tool: the easel answers how far its replay of the log has got
			const s = await easel(studio, ["status"], undefined, signal, 10_000);
			if (signal?.aborted) throw new Error("Operation aborted");
			if (!s.timedOut && s.code === 0 && REPLAY.test(s.out.trim())) {
				throw new Error(`the easel is reopening the painting from its log and hasn't finished: ${s.out.trim()}. \`status\` shows how far it has got.`);
			}
		}
		if (r.timedOut) throw new Error(rebuilding ? "the easel rebuild stalled without progress" : opened ? "the easel didn't open in time" : "the easel isn't answering");
		const progress = REPLAY.exec(r.out.trim());
		if (progress) {
			const k = Number(progress[1]);
			const n = Number(progress[2]);
			if (r.code !== 0 || !Number.isSafeInteger(k) || !Number.isSafeInteger(n) || k > n || k < completed || (total !== undefined && n !== total)) {
				throw new Error(`the easel returned invalid rebuild progress: ${r.out.trim()}`);
			}
			total = n;
			if (answerReplay) return true;
			if (!rebuilding || k > completed) {
				completed = k;
				progressDeadline = Date.now() + (wideStudio(studio) ? WIDE_WAIT_MS.rebuild : WAIT_MS.rebuild);
			}
			rebuilding = true;
			if (Date.now() >= progressDeadline) throw new Error("the easel rebuild stalled without progress");
			await pause(Math.min(100, progressDeadline - Date.now()), undefined, { signal });
			r = await step(studio, ["status"], undefined, signal, progressDeadline);
			continue;
		}
		if (r.code !== 0 && r.out.trim() === STARTING) {
			// the easel has its socket but hasn't read the log yet: its progress is a moment away
			if (Date.now() >= deadline) throw new Error("the easel isn't answering");
			await pause(Math.min(100, deadline - Date.now()), undefined, { signal });
			r = await step(studio, ["status"], undefined, signal, deadline);
			continue;
		}
		if (r.code === 0) return rebuilding;
		// Never open a second easel after an open or a rebuild failure.
		if (opened || rebuilding) throw new Error(r.out.trim() || "the easel didn't open");
		opened = true;
		r = await step(studio, ["open"], undefined, signal, deadline);
	}
}

/** A log's text ("" if there is no log yet). */
function logText(log: string): string {
	try {
		return readFileSync(log, "utf8");
	} catch {
		return "";
	}
}

/**
 * Whether `lua` was kept as a new chunk: the log changed since `before` and its last chunk is
 * `lua` as the easel keeps it. A fresh scratch canvas replaces its log, so chunks aren't counted.
 */
function kept(log: string, before: string, lua: string | undefined): boolean {
	const now = logText(log);
	return now !== before && now.split(/^--@ chunk \d+\n/m).at(-1) === `${(lua ?? "").trimEnd()}\n`;
}

/** Run an easel command at an open easel; a failure becomes the tool's error, with the easel's words. */
export async function atEasel(studio: string, args: string[], input: string | undefined, signal?: AbortSignal): Promise<string> {
	const wait = args[0] === "do" ? (wideStudio(studio) ? WIDE_WAIT_MS.do : WAIT_MS.do) : WAIT_MS.other;
	let deadline = Date.now() + wait;
	if (await ensureOpen(studio, signal, deadline, args[0] === "status")) deadline = Date.now() + wait;
	const log = join(studio, "paintings", "lua", args.includes("--scratch") ? "scratch.lua" : "painting.lua");
	const before = args[0] === "do" ? logText(log) : "";
	const r = await step(studio, args, input, signal, deadline);
	if (r.timedOut) {
		if (args[0] === "note") throw new Error("the easel didn't answer; the entry may have been added all the same, so read notes/journal.md before adding it again");
		if (args[0] !== "do") throw new Error(`the easel didn't answer; \`${args[0]}\` changes nothing, so it can be asked again`);
		// what landed: the chunk still running (its progress), in the log, or neither
		const s = await easel(studio, ["status"], undefined, signal, 10_000);
		if (signal?.aborted) throw new Error("Operation aborted");
		const stopped = "`paint` stopped waiting for the easel's answer";
		if (s.timedOut || s.code !== 0) throw new Error(`${stopped}; \`log\` shows whether the chunk ran`);
		const now = progressReply(s.out.trim());
		if (now.startsWith("painting a chunk")) {
			throw new Error(`${stopped}. The chunk is still running and isn't in the log yet: ${now}. \`status\` shows how far it has got while it runs; once it ends, \`log\` shows whether it was kept.`);
		}
		if (kept(log, before, input)) throw new Error(`${stopped}, but the chunk ran: it is in the log.`);
		throw new Error(`${stopped}, and the chunk isn't in the log: it changed nothing.`);
	}
	const text = r.out.trimEnd();
	if (r.code !== 0) throw new Error(text || "the easel gave no answer");
	return text;
}

export const text = (t: string) => ({ content: [{ type: "text" as const, text: t }], details: {} });

/** At most `max` characters: the end of `t`, as bash's output was cut. */
export function tail(t: string, max = 50_000, whole = "paintings/lua/painting.lua"): string {
	return t.length <= max ? t : `(the first ${t.length - max} characters are left out; read ${whole} for all of it)\n` + t.slice(-max);
}

/**
 * The file `path` names, as pi's read tool resolves it (its `resolveToCwd` in
 * dist/core/tools/path-utils.js: Unicode spaces to spaces, a leading `@` dropped, `~` for the
 * home folder, `file://` URLs), with links followed; undefined if that is outside the studio.
 * A missing file is placed by its nearest existing folder.
 */
export function studioPath(studio: string, path: string): string | undefined {
	let p = path.replace(/[\u00A0\u2000-\u200A\u202F\u205F\u3000]/g, " ");
	if (p.startsWith("@")) p = p.slice(1);
	if (p === "~") p = homedir();
	else if (p.startsWith("~/")) p = join(homedir(), p.slice(2));
	else if (/^file:\/\//.test(p)) p = fileURLToPath(p);
	const root = realpathSync(studio);
	const real = realOf(resolve(root, p));
	const rel = relative(root, real);
	// outside: a first step that is exactly ".." (not a name like "..draft.md"), or another drive
	if (rel !== "" && (rel === ".." || rel.startsWith(`..${sep}`) || isAbsolute(rel))) return undefined;
	const executable = statSync(join(root, "bin", "easel"), { throwIfNoEntry: false });
	const requested = statSync(real, { throwIfNoEntry: false });
	if (executable && requested && executable.dev === requested.dev && executable.ino === requested.ino) return undefined;
	return real;
}

export function directoryListing(path: string): string | undefined {
	if (!statSync(path).isDirectory()) return undefined;
	return readdirSync(path, { withFileTypes: true })
		.map((entry) => entry.name + (entry.isDirectory() ? "/" : ""))
		.sort()
		.join("\n") || "empty folder";
}

/** `full` with links followed; for a missing file, its nearest existing folder's. */
function realOf(full: string): string {
	try {
		return realpathSync(full);
	} catch {
		const up = dirname(full);
		return up === full ? full : join(realOf(up), basename(full));
	}
}

export function lookArgs(p: { crop?: string; mask?: string; mode?: string; size?: number; grid?: boolean | number; light?: string; palette?: boolean; survey?: boolean; compare?: string; ref?: string; hold?: string; at?: string; pose?: string; side?: string }): string[] {
	const a: string[] = [];
	if (p.palette) {
		if (Object.entries(p).some(([key, value]) => key !== "palette" && value !== undefined && value !== false && value !== "")) {
			throw new Error("look: palette takes no other option");
		}
		return ["--palette"];
	}
	if (p.survey) a.push("--survey");
	if (p.compare) a.push("--compare", p.compare);
	if (p.ref) a.push("--ref", p.ref);
	if (p.hold) a.push("--hold", p.hold);
	if (p.at) a.push("--at", p.at);
	if (p.pose) a.push("--pose", p.pose);
	if (p.side) a.push("--side", p.side);
	if (p.crop) a.push("--crop", p.crop);
	if (p.mask) a.push("--mask", p.mask);
	if (p.mode) a.push("--mode", p.mode);
	if (p.light) a.push("--light", p.light);
	if (p.size !== undefined) a.push("--size", String(p.size));
	if (p.grid === true) a.push("--grid");
	else if (typeof p.grid === "number") a.push("--grid", String(p.grid));
	return a;
}

/** The easel's `probe` arguments for the tool's `at` or `crop`: one of the two. */
export function probeArgs(p: { at?: string; crop?: string }): string[] {
	if (!p.at === !p.crop) throw new Error('probe: give at: "x,y" or crop: "x0,y0,x1,y1", one of the two');
	return p.at ? ["--at", p.at] : ["--crop", p.crop!];
}

/**
 * An easel `look` message in the tool's words: `--crop` is the look tool's `crop`, and so on.
 */
export function toolWords(t: string): string {
	return t
		.replace(/--crop exceeds 1200 pixels per side; choose a smaller crop \(crops stay 1:1\)/g, "a crop may be at most 500 units on either side; choose a smaller crop")
		.replace(/--(crop|mask|mode|size|grid|palette|light|survey|compare|ref|hold|at|pose|side)\b/g, "$1");
}

/** Deliver a survey in order within the request budget, keeping every original tile on disk. */
export function surveyReply<I extends { type: "image"; data: string }>(said: string, paths: string[], images: I[], limits: PruneLimits) {
	if (images.length !== paths.length) throw new Error("look: a survey tile could not be read; the survey is incomplete");
	const oversized = images.findIndex((image) => image.data.length > limits.maxImageChars);
	if (oversized >= 0) throw new Error(`look: survey incomplete: ${paths[oversized]} alone exceeds the configured image budget; full-detail tiles remain on disk, but this budget cannot deliver them`);
	let count = 0;
	let chars = 0;
	while (count < images.length && count < limits.maxImages && chars + images[count].data.length <= limits.maxImageChars) {
		chars += images[count++].data.length;
	}
	if (count < images.length) {
		said += `\nPartial survey: showing tiles 1–${count} of ${images.length} within the image budget. The remaining full-detail tiles have not been shown. Read each remaining file in a separate turn before assessing the whole canvas:\n${paths.slice(count).join("\n")}\n`;
	}
	return { content: [{ type: "text" as const, text: said }, ...images.slice(0, count)] };
}

/**
 * Nothing that counts the painter's work or times the machine reaches the painter: no chunk
 * numbers, no look numbers, no compute seconds. The painting's own clock (`wait()`'s `day 3,
 * 14:20`, the journal's stamps) is the only time it sees. A Lua error names its chunk
 * `[string "chunk 12"]` (the easel loads each chunk as "chunk N"); it reads `[string "chunk"]`.
 */
export function hideCounters(t: string): string {
	return t
		.replace(/\[string "chunk \d+"\]/g, '[string "chunk"]')
		// the easel's limit on a chunk's machine time (it has never been reached: the longest chunk took under 4 minutes)
		.replace(/the chunk ran longer than \d+ minutes and was stopped/g, "the chunk didn't finish and was stopped");
}

/**
 * A paint reply without the chunk's number and the easel's compute time: `ok · chunk 12 (76.50 s
 * to compute)` reads `ok`. A chunk that runs out of the easel's time limit still says so in its error.
 */
export function paintReply(reply: string): string {
	// the easel's line is the reply's last; a line the chunk printed above it is left as it is
	return hideCounters(reply.replace(/(^|\n)ok · chunk \d+(?: \(\d+(?:\.\d+)? s to compute\))?(\n?)$/, "$1ok$2"));
}

/** A status reply without the chunk count or the pixel width: `12 chunks · 2400px · <setup>` reads `<setup>`. */
export function statusReply(reply: string): string {
	return progressReply(reply.replace(/^\d+ chunks? · /, "").replace(/^\d+px · /, ""));
}

/**
 * A running chunk's progress built only from what the painter may hear: that the chunk is painting
 * or in the log, and the painting's own time. The easel's line numbers, mark counts, chunk number and
 * running time are left out whatever their wording: `painting a chunk (at line 8 of 20): 100 strokes
 * and touches so far, 5 s on the painting's clock (now day 1, 09:00), running for 12 min 3 s` reads
 * `painting a chunk: it has taken 5 s of the painting's time so far, and the painting's clock reads day 1, 09:00`.
 */
export function progressReply(reply: string): string {
	return reply
		.replace(/^painting a chunk\b.*$/m, (line) => {
			const spent = line.match(/(\d+(?: min \d+)? s) on the painting's clock \(now (day \d+, \d\d:\d\d)\)/);
			if (spent) return `painting a chunk: it has taken ${spent[1]} of the painting's time so far, and the painting's clock reads ${spent[2]}`;
			return line.startsWith("painting a chunk: starting") ? "painting a chunk: starting" : "painting a chunk";
		})
		.replace(/^the chunk is in the log\b.*$/m, "the chunk is in the log; the easel is finishing its reply");
}

/**
 * The lines of `log` that hold `find` (in any case), numbered as in the file `whole`: `12: ...`,
 * with the two lines before and after each, `11- ...`, and `--` between passages apart.
 */
export function logFind(log: string, find: string, whole: string): string {
	const lines = log.split("\n");
	const want = find.toLowerCase();
	const hits = lines.flatMap((line, i) => (line.toLowerCase().includes(want) ? [i] : []));
	if (hits.length === 0) return `no line of ${whole} holds "${find}"`;
	const hit = new Set(hits);
	const out = [`${hits.length} ${hits.length === 1 ? "line" : "lines"} of ${whole} ${hits.length === 1 ? "holds" : "hold"} "${find}"${hits.length > 100 ? " (the first 100 are shown)" : ""}`];
	let last = -1;
	for (const i of hits.slice(0, 100)) {
		const from = Math.max(i - 2, last + 1);
		if (last >= 0 && from > last + 1) out.push("--");
		for (let k = from; k <= Math.min(i + 2, lines.length - 1); k++) out.push(`${k + 1}${hit.has(k) ? ":" : "-"} ${lines[k]}`);
		last = Math.max(last, i + 2);
	}
	return out.join("\n");
}

/** The log with its chunk lines unnumbered: `--@ chunk 12` reads `--@ chunk`. */
export function logReply(log: string): string {
	return log.replace(/^--@ chunk \d+[ \t]*$/gm, "--@ chunk");
}

/** Whether `path` is a file that is there (false too where it can't be read). */
function isFile(path: string): boolean {
	try {
		return statSync(path).isFile();
	} catch {
		return false;
	}
}

function renameCompanions(studio: string, before: string, after: string): void {
	for (const folder of [".gallery", ".relief"]) {
		const sibling = resolve(studio, dirname(before), folder, basename(before));
		if (!isFile(sibling)) continue;
		try {
			renameSync(sibling, resolve(studio, dirname(after), folder, basename(after)));
		} catch (e) {
			console.error(`warning: couldn't rename ${folder.slice(1)} companion: ${e}`);
		}
	}
}

/**
 * `renameLook` for every look an answer names (a survey names several): each
 * line that begins with a look's png path (a file that is there; its folders may
 * have spaces, or ".png", in their names) gets a fresh name; the paths in order.
 */
export function renameLooks(studio: string, said: string): { said: string; paths: string[] } {
	const paths: string[] = [];
	const lines = said.split("\n").map((line) => {
		// (the whole path, to its last ".png": a folder's name may hold one too)
		const m = /^(.+\.png)( \(\d+x\d+.*)?$/.exec(line);
		if (!m || !isFile(resolve(studio, m[1]))) return line;
		const stable = isFile(resolve(studio, dirname(m[1]), ".display", basename(m[1])));
		const path = stable ? m[1] : join(dirname(m[1]), `${randomUUID()}.png`);
		if (!stable) {
			renameSync(resolve(studio, m[1]), resolve(studio, path));
			renameCompanions(studio, m[1], path);
		}
		paths.push(path);
		// (without the machine's seconds, as `renameLook`)
		return path + (m[2] ?? "").replace(/^ \((\d+x\d+), \d+(?:\.\d+)?s\)/, " ($1)");
	});
	return { said: lines.join("\n"), paths };
}

/**
 * A look under a random name and without the machine's seconds: the easel's last line
 * `<dir>/look-0012.png (1000x714, 0.04s)` becomes `<dir>/<uuid>.png (1000x714)`, and the file is
 * renamed to match. `studio` resolves a relative path. Anything else is returned as it was.
 */
export function renameLook(studio: string, said: string): { said: string; path: string } {
	const lines = said.split("\n");
	const m = /^(.*\.png)(?: \((\d+x\d+)(?:, \d+(?:\.\d+)?s)?\))?$/.exec(lines[lines.length - 1]);
	if (!m) return { said, path: lines[lines.length - 1].replace(/ \(.*\)$/, "") };
	const stable = isFile(resolve(studio, dirname(m[1]), ".display", basename(m[1])));
	const path = stable ? m[1] : join(dirname(m[1]), `${randomUUID()}.png`);
	if (!stable) {
		renameSync(resolve(studio, m[1]), resolve(studio, path));
		renameCompanions(studio, m[1], path);
	}
	lines[lines.length - 1] = m[2] ? `${path} (${m[2]})` : path;
	return { said: lines.join("\n"), path };
}
