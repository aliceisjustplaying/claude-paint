import { execFile, spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { closeSync, cpSync, existsSync, mkdirSync, openSync, readdirSync, readFileSync, readSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { promisify } from "node:util";
import type { Config, Job, Painting, Post } from "./types.ts";

const asset = (name: string) => new URL(`../assets/${name}`, import.meta.url);
const exec = promisify(execFile);

// the house runner's (r40 r21_chains.py): its tools, its first message and its message for an interrupted sitting
const TOOLS = "paint,look,note,status,log,read";
export const PAINTER_MSG = "Your brief is in BRIEF.md in this folder. Your last message is your reply: the " +
  "painting's title if you give it one and a few sentences about the picture. " +
  "At the end of each sitting, leave a handoff in your journal and last reply: passages still unresolved, " +
  "practical lessons to carry forward from what you observed and recipes that failed. Before retrying " +
  "a failed recipe, identify what has changed and why that could address the observed failure. " +
  "Carry forward the conditions of material trials and repairs: substrate and wetness, recipe ratios, " +
  "medium or thinner, load, coverage and viewed result; label broader rules as hypotheses. " +
  "Record what you observed and what remains uncertain.";
export const CONTINUE_MESSAGE = "Carry on where you left off.";

export function painterJob(post: Post, root: { uri: string; cid: string }, instruction: string, config: Config): Job {
  const folder = `paint-studio-${createHash("sha256").update(post.uri).digest("hex").slice(0, 6)}`;
  const dir = resolve(config.studio.root, folder), home = homedir();
  const brief = readFileSync(asset("brief_template.md"), "utf8")
    .replace("{OPENING}", instruction)
    .replace("{STUDIO}", dir.startsWith(home + "/") ? "~" + dir.slice(home.length) : dir)
    .replace("{READING}", "- notes/easel_guide.md");
  return { id: post.uri, post, root, instruction, brief, studio: config.studio, folder };
}

export function jobDir(config: Config, id: string): string {
  return join(dirname(config.database), "jobs", createHash("sha256").update(id).digest("hex"));
}

export function paintingBlob(bytes: Buffer, mimeType: string): { $type: "blob"; ref: { $link: string }; mimeType: string; size: number } {
  const digest = Buffer.concat([Buffer.from([1, 0x55, 0x12, 0x20]), createHash("sha256").update(bytes).digest()]);
  const alphabet = "abcdefghijklmnopqrstuvwxyz234567";
  let bits = 0, value = 0, encoded = "b";
  for (const byte of digest) {
    value = (value << 8) | byte;
    bits += 8;
    while (bits >= 5) { bits -= 5; encoded += alphabet[(value >>> bits) & 31]; }
  }
  if (bits) encoded += alphabet[(value << (5 - bits)) & 31];
  return { $type: "blob", ref: { $link: encoded }, mimeType, size: bytes.length };
}

export interface Usage { input: number; output: number; cost: number }

/** Token and dollar totals of a painter's pi sessions, read as the files grow. */
export class UsageMeter {
  readonly dir: string;
  private read = new Map<string, { offset: number; rest: string }>();
  private total: Usage = { input: 0, output: 0, cost: 0 };
  constructor(dir: string) { this.dir = dir; }
  usage(): Usage {
    const files = existsSync(this.dir) ? readdirSync(this.dir).filter(name => name.endsWith(".jsonl")).sort() : [];
    for (const name of files) {
      const path = join(this.dir, name), state = this.read.get(path) ?? { offset: 0, rest: "" };
      const size = statSync(path).size;
      if (size <= state.offset) continue;
      const bytes = Buffer.alloc(size - state.offset), fd = openSync(path, "r");
      try { readSync(fd, bytes, 0, bytes.length, state.offset); } finally { closeSync(fd); }
      const lines = (state.rest + bytes.toString("utf8")).split("\n");
      state.rest = lines.pop() ?? "";
      state.offset = size;
      this.read.set(path, state);
      for (const line of lines) {
        let entry: { type?: string; message?: { role?: string; usage?: { input?: number; output?: number; cacheRead?: number; cacheWrite?: number; cost?: { total?: number } } } };
        try { entry = JSON.parse(line) as typeof entry; } catch { continue; }
        const usage = entry.type === "message" && entry.message?.role === "assistant" ? entry.message.usage : undefined;
        if (!usage) continue;
        this.total.input += (usage.input ?? 0) + (usage.cacheRead ?? 0) + (usage.cacheWrite ?? 0);
        this.total.output += usage.output ?? 0;
        this.total.cost += usage.cost?.total ?? 0;
      }
    }
    return { ...this.total };
  }
}

/** Stops the studio's easel server (named in its session lock) and waits for it to exit; a server still replaying has no socket for `close`. */
export async function stopServer(dir: string): Promise<void> {
  let pid: number;
  try {
    pid = Number(readFileSync(join(dir, "out/easel/painting/lock"), "utf8").trim());
    if (!Number.isSafeInteger(pid) || pid <= 1 || pid > 2147483647) return;
    if (!readFileSync(`/proc/${pid}/cmdline`, "utf8").startsWith(join(dir, "bin/easel"))) return;
    process.kill(pid, "SIGTERM");
  } catch { return; }
  for (let waited = 0; existsSync(`/proc/${pid}`); waited += 100) {
    if (waited === 10000) try { process.kill(pid, "SIGKILL"); } catch { /* gone */ }
    await new Promise(resolve => setTimeout(resolve, 100));
  }
}

export function overLimit(usage: Usage, studio: Config["studio"]): string | null {
  if (studio.maxInputTokens != null && usage.input >= studio.maxInputTokens) return "input_tokens";
  if (studio.maxOutputTokens != null && usage.output >= studio.maxOutputTokens) return "output_tokens";
  if (studio.maxCostUsd != null && usage.cost >= studio.maxCostUsd) return "cost";
  return null;
}

export interface Run { stopped: string | null }

export interface Painter {
  run(job: Job, signal: AbortSignal, started: number): Promise<Run | void>;
  finish(job: Job, run: Run | void): Promise<Painting | "dry">;
}

/** Test seams for the launch: another provider and model, extra pi extensions and environment. */
export interface Launch { model?: string[]; extensions?: string[]; env?: Record<string, string> }

/**
 * The house painter, launched as the house runner launches an Opus sitting: a studio exported from
 * the house exporter (copied from studio.template), pi with harness/painter's extensions, system
 * prompt and tools, cwd the studio. It stops the painter at the studio's time, token and cost limits.
 */
export class HousePainter implements Painter {
  readonly config: Config;
  readonly env: NodeJS.ProcessEnv;
  readonly log: (value: object) => void;
  readonly launch: Launch;
  constructor(config: Config, env: NodeJS.ProcessEnv = process.env, log: (value: object) => void = value => console.log(JSON.stringify(value)), launch: Launch = {}) {
    this.config = config; this.env = env; this.log = log; this.launch = launch;
  }
  studio(job: Job): string { return resolve(this.config.studio.root, job.folder); }
  sessions(job: Job): string {
    return resolve(this.config.studio.sessions, `--${this.studio(job).replace(/^\/+|\/+$/gu, "").replaceAll("/", "-")}--`);
  }
  args(job: Job, session?: string): string[] {
    const h = resolve(this.config.studio.harness), s = this.config.studio;
    return ["--print", "--no-extensions", "-e", join(h, "painter.ts"), "-e", join(h, "compaction.ts"),
      ...(this.launch.extensions ?? []).flatMap(path => ["-e", path]),
      "--system-prompt", join(h, "system_prompt.md"), "--tools", TOOLS,
      "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve",
      ...(this.launch.model ?? ["--provider", "anthropic", "--model", s.painterModel, "--thinking", s.thinking]),
      "--session-dir", this.sessions(job), ...(session ? ["--session", session, CONTINUE_MESSAGE] : [PAINTER_MSG])];
  }
  async run(job: Job, signal: AbortSignal, started: number): Promise<Run> {
    const dir = this.studio(job), sessions = this.sessions(job);
    if (this.config.dryRun) {
      this.log({ event: "painter_dry_run", studio: dir, command: ["pi", ...this.args(job)] });
      return { stopped: null };
    }
    if (!existsSync(dir)) {
      cpSync(realpathSync(this.config.studio.template), dir, { recursive: true, verbatimSymlinks: true });
      await exec("chmod", ["-R", "u+w", dir]);
    }
    writeFileSync(join(dir, "BRIEF.md"), job.brief);
    mkdirSync(sessions, { recursive: true });
    const previous = readdirSync(sessions).filter(name => name.endsWith(".jsonl")).sort().at(-1);
    const args = this.args(job, previous && join(sessions, previous));
    const key = this.env["ANTHROPIC_API_KEY"];
    const env: Record<string, string> = { PATH: process.env["PATH"] ?? "/usr/bin:/bin", HOME: process.env["HOME"] ?? homedir(),
      ...(process.env["TMPDIR"] ? { TMPDIR: process.env["TMPDIR"] } : {}), ...(key ? { ANTHROPIC_API_KEY: key } : {}), ...this.launch.env };
    const reason = () => signal.reason === "time" ? "time" : "canceled";
    const wall = this.config.studio.maxWallSeconds, left = () => wall == null ? Infinity : wall * 1000 - (Date.now() - started);
    if (left() <= 0) return { stopped: "time" };
    try { await exec(join(dir, "bin/easel"), ["open"], { cwd: dir, env, timeout: Math.min(1800000, left()), maxBuffer: 64 * 1024 * 1024, signal }); }
    catch (error) {
      await stopServer(dir);
      if (signal.aborted) return { stopped: reason() };
      if (left() <= 0) return { stopped: "time" };
      throw error;
    }
    if (signal.aborted) {
      await exec(join(dir, "bin/easel"), ["close"], { cwd: dir, env, timeout: 60000 }).catch(() => {});
      return { stopped: reason() };
    }
    const out = openSync(join(sessions, "reply.txt"), "w"), err = openSync(join(sessions, "stderr.txt"), "a");
    const child = spawn("pi", args, { cwd: dir, env, detached: true, stdio: ["ignore", out, err] });
    closeSync(out); closeSync(err);
    const meter = new UsageMeter(sessions);
    let stopped: string | null = null;
    const stop = (reason: string) => {
      if (stopped || child.exitCode !== null || child.signalCode !== null) return;
      stopped = reason;
      this.log({ event: "painter_stopped", studio: dir, reason, usage: meter.usage() });
      try { process.kill(-child.pid!, "SIGTERM"); } catch { /* already gone */ }
      setTimeout(() => { try { process.kill(-child.pid!, "SIGKILL"); } catch { /* already gone */ } }, 5000).unref();
    };
    const check = () => {
      if (signal.aborted) stop(reason());
      else if (this.config.studio.maxWallSeconds != null && Date.now() - started >= this.config.studio.maxWallSeconds * 1000) stop("time");
      else { const reason = overLimit(meter.usage(), this.config.studio); if (reason) stop(reason); }
    };
    const poll = setInterval(check, 1000);
    signal.addEventListener("abort", check, { once: true });
    const code = await new Promise<number | null>(done => {
      child.once("error", () => done(-1));
      child.once("close", exit => done(exit));
    });
    clearInterval(poll);
    signal.removeEventListener("abort", check);
    if (stopped === "canceled") await exec(join(dir, "bin/easel"), ["close"], { cwd: dir, env, timeout: 60000 }).catch(() => {});
    return { stopped: stopped ?? (code === 0 ? null : "crashed") };
  }
  async finish(job: Job, result: Run | void): Promise<Painting | "dry"> {
    if (this.config.dryRun) return "dry";
    const dir = this.studio(job), sessions = this.sessions(job), easel = join(dir, "bin/easel");
    const png = join(dir, "out/easel/painting/painting.png"), jpeg = join(sessions, "painting.jpg");
    rmSync(png, { force: true });
    try {
      await exec(easel, ["open"], { cwd: dir, timeout: 1800000, maxBuffer: 64 * 1024 * 1024 }).catch(async (error: unknown) => { await stopServer(dir); throw error; });
      await exec(easel, ["save"], { cwd: dir, timeout: 600000 });
    }
    finally { await exec(easel, ["close"], { cwd: dir, timeout: 60000 }).catch(() => {}); }
    const source = readFileSync(png);
    const s = this.config.studio, w = source.readUInt32BE(16), h = source.readUInt32BE(20);
    const scale = Math.min(1, s.width / w, s.height / h);
    const width = Math.max(1, Math.round(w * scale)), height = Math.max(1, Math.round(h * scale));
    let bytes: Buffer | undefined;
    for (const quality of [2, 3, 4, 6, 8, 11, 15, 20, 31]) {
      await exec("ffmpeg", ["-v", "error", "-y", "-i", png, "-vf", `scale=${width}:${height}:flags=lanczos`, "-q:v", String(quality), jpeg], { timeout: 120000 });
      bytes = readFileSync(jpeg);
      if (bytes.length <= this.config.caps.imageBytes) break;
    }
    if (!bytes || bytes.length > this.config.caps.imageBytes) throw new Error("painting_too_large");
    const reply = existsSync(join(sessions, "reply.txt")) ? readFileSync(join(sessions, "reply.txt"), "utf8").replaceAll("**", "").replace(/\s+/gu, " ").trim() : "";
    const fallback = (JSON.parse(readFileSync(asset("replies.json"), "utf8")) as { imageAlt: string }).imageAlt;
    return { bytes, width, height, alt: [...reply].slice(0, 1000).join("") || fallback, stopped: result?.stopped ?? null };
  }
}
