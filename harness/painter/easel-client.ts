/**
 * The easel client and small helpers for the painter's tools (easel-tools.ts), without pi
 * imports so node --test can load them.
 */
import { spawn } from "node:child_process";
import { setTimeout as pause } from "node:timers/promises";
import { realpathSync } from "node:fs";
import { homedir } from "node:os";
import { basename, dirname, isAbsolute, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export interface Ran {
	code: number | null;
	out: string;
	timedOut?: boolean;
}

/** Tool budgets include opening, except advancing log rebuilds get a separate stall budget.
 * A chunk may run 10 minutes (the easel's own limit).
 */
export const WAIT_MS = { do: 12 * 60_000, other: 3 * 60_000, rebuild: 30 * 60_000 };

/** Run the studio's easel client with `args`, `input` on stdin; stdout and stderr together, in order. */
export function easel(studio: string, args: string[], input?: string, signal?: AbortSignal, waitMs?: number): Promise<Ran> {
	return new Promise((done, fail) => {
		if (signal?.aborted) return fail(new Error("Operation aborted"));
		const env = { PATH: "/usr/bin:/bin", HOME: process.env.HOME ?? "" };
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

/** Open if needed, waiting for replay readiness. Returns whether a rebuild was awaited. */
export async function ensureOpen(studio: string, signal?: AbortSignal, deadline = Date.now() + WAIT_MS.other): Promise<boolean> {
	let r = await step(studio, ["status"], undefined, signal, deadline);
	let opened = false;
	let rebuilding = false;
	let completed = -1;
	let total: number | undefined;
	let progressDeadline = deadline;
	for (;;) {
		if (r.timedOut) throw new Error(rebuilding ? "the easel rebuild stalled without progress" : opened ? "the easel didn't open in time" : "the easel isn't answering");
		const progress = /^rebuilding from the log \((\d+) of (\d+) chunks\)$/.exec(r.out.trim());
		if (progress) {
			const k = Number(progress[1]);
			const n = Number(progress[2]);
			if (r.code !== 0 || !Number.isSafeInteger(k) || !Number.isSafeInteger(n) || k > n || k < completed || (total !== undefined && n !== total)) {
				throw new Error(`the easel returned invalid rebuild progress: ${r.out.trim()}`);
			}
			total = n;
			if (!rebuilding || k > completed) {
				completed = k;
				progressDeadline = Date.now() + WAIT_MS.rebuild;
			}
			rebuilding = true;
			if (Date.now() >= progressDeadline) throw new Error("the easel rebuild stalled without progress");
			await pause(Math.min(100, progressDeadline - Date.now()), undefined, { signal });
			r = await step(studio, ["status"], undefined, signal, progressDeadline);
			continue;
		}
		if (r.code === 0) return rebuilding;
		// Never open a second easel after an open or a rebuild failure.
		if (opened || rebuilding) throw new Error(r.out.trim() || "the easel didn't open");
		opened = true;
		r = await step(studio, ["open"], undefined, signal, deadline);
	}
}

/** Run an easel command at an open easel; a failure becomes the tool's error, with the easel's words. */
export async function atEasel(studio: string, args: string[], input: string | undefined, signal?: AbortSignal): Promise<string> {
	const wait = args[0] === "do" ? WAIT_MS.do : WAIT_MS.other;
	let deadline = Date.now() + wait;
	if (await ensureOpen(studio, signal, deadline)) deadline = Date.now() + wait;
	const r = await step(studio, args, input, signal, deadline);
	if (r.timedOut) {
		throw new Error(`the easel didn't answer within ${wait / 60_000} minutes; \`status\` shows whether the chunk count changed`);
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
	return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel)) ? real : undefined;
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

export function lookArgs(p: { crop?: string; mode?: string; size?: number; grid?: boolean | number }): string[] {
	const a: string[] = [];
	if (p.crop) a.push("--crop", p.crop);
	if (p.mode) a.push("--mode", p.mode);
	if (p.size !== undefined) a.push("--size", String(p.size));
	if (p.grid === true) a.push("--grid");
	else if (typeof p.grid === "number") a.push("--grid", String(p.grid));
	return a;
}

/** An easel `look` message in the tool's words: `--crop` is the look tool's `crop`, and so on. */
export function toolWords(t: string): string {
	return t.replace(/--(crop|mode|size|grid)\b/g, "$1");
}
