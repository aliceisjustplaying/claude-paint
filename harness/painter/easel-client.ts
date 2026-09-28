/**
 * The easel client and small helpers for the painter's tools (easel-tools.ts), without pi
 * imports so node --test can load them.
 */
import { spawn } from "node:child_process";
import { realpathSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";

export interface Ran {
	code: number | null;
	out: string;
	timedOut?: boolean;
}

/** Run the studio's easel client with `args`, `input` on stdin; stdout and stderr together, in order. */
/** How long a tool waits for the easel: a chunk may run 10 minutes (the easel's own limit). */
export const WAIT_MS = { do: 12 * 60_000, other: 3 * 60_000 };

export function easel(studio: string, args: string[], input?: string, signal?: AbortSignal, waitMs?: number): Promise<Ran> {
	return new Promise((done, fail) => {
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
		let out = "";
		p.stdout.on("data", (d) => (out += d));
		p.stderr.on("data", (d) => (out += d));
		p.on("error", fail);
		let timedOut = false;
		const timer = waitMs ? setTimeout(() => ((timedOut = true), kill()), waitMs) : undefined;
		p.on("close", (code) => (clearTimeout(timer), done({ code: timedOut ? null : code, out: timedOut ? "" : out, timedOut } as Ran)));
		const stop = kill;
		signal?.addEventListener("abort", stop, { once: true });
		p.stdin.end(input ?? "");
	});
}

/** Open the easel if no session is running (the runner normally has). */
export async function ensureOpen(studio: string, signal?: AbortSignal): Promise<void> {
	const st = await easel(studio, ["status"], undefined, signal);
	if (st.code === 0) return;
	const op = await easel(studio, ["open"], undefined, signal);
	if (op.code !== 0) throw new Error(op.out.trim() || "the easel didn't open");
}

/** Run an easel command at an open easel; a failure becomes the tool's error, with the easel's words. */
export async function atEasel(studio: string, args: string[], input: string | undefined, signal?: AbortSignal): Promise<string> {
	await ensureOpen(studio, signal);
	const wait = args[0] === "do" ? WAIT_MS.do : WAIT_MS.other;
	const r = await easel(studio, args, input, signal, wait);
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

/** Inside the studio folder (after following links)? */
export function inStudio(studio: string, path: string): boolean {
	const full = isAbsolute(path) ? path : resolve(studio, path);
	let real: string;
	try {
		real = realpathSync(full);
	} catch {
		real = full; // doesn't exist: read says so
	}
	const rel = relative(realpathSync(studio), real);
	return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
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
