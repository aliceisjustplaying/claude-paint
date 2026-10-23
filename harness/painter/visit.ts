/**
 * The critic's visits (painter.ts), without pi imports so node --test can load it.
 *
 * The critic is a second pi session on the painter's model (critic.ts, critic_system_prompt.md),
 * run in the studio at the end of each sitting and whenever the painter calls `ask_critic`. It sees
 * the painting with `look` and reads the painter's journal and the painter's note to it; it has no
 * paint tools. Its last message goes to notes/critic.md: the painter gets it as `ask_critic`'s
 * reply, or at the start of its next sitting.
 *
 * PAINTER_CRITIC turns it on (the runner sets it, run_friedrich.py): JSON
 * `{"args": [the model's pi arguments], "dir": a folder outside the studio}`. The folder keeps the
 * critic's sessions, a copy of every visit's notes and of every note the painter left, and its log.
 */
import { spawn } from "node:child_process";
import { appendFileSync, existsSync, mkdirSync, readdirSync, readFileSync, realpathSync } from "node:fs";
import { join, relative, sep } from "node:path";
import { studioPath } from "./easel-client.ts";
import { JOURNAL, OPEN_PASSAGES, openPassages } from "./journal.ts";
import { publishNotes } from "./note-publication.ts";
import { quote } from "./summary.ts";

export const CRITIQUE = "notes/critic.md";
export const TO_CRITIC = "notes/to_critic.md";

export interface Critic {
	args: string[];
	dir: string;
}

/** "sitting": at the end of a sitting; "asked": the painter called ask_critic. */
export type Occasion = "sitting" | "asked";

const HEADER: Record<Occasion, string> = {
	sitting: "From the critic's visit at the end of a sitting:",
	asked: "From the critic's visit you asked for:",
};

export function criticFromEnv(env: Record<string, string | undefined> = process.env): Critic | undefined {
	if (!env.PAINTER_CRITIC) return undefined;
	const c = JSON.parse(env.PAINTER_CRITIC);
	if (!Array.isArray(c?.args) || !c.args.every((a: unknown) => typeof a === "string") || typeof c.dir !== "string" || !c.dir) {
		throw new Error(`PAINTER_CRITIC=${env.PAINTER_CRITIC}: want {"args": [pi arguments], "dir": "a folder"}`);
	}
	return { args: c.args, dir: c.dir };
}

/** What the critic may read: the journal, the painter's note to it and the easel's looks; the file, or undefined. */
export function criticReadable(studio: string, path: string): string | undefined {
	const real = studioPath(studio, path);
	if (!real) return undefined;
	const rel = relative(realpathSync(studio), real).split(sep).join("/");
	return real === studioPath(studio, JOURNAL) || real === studioPath(studio, TO_CRITIC) || (rel.startsWith("out/easel/") && rel.endsWith(".png")) ? real : undefined;
}

/** The journal's latest list of open passages (journal.ts), or undefined. */
function journalPassages(studio: string): string | undefined {
	const path = join(studio, JOURNAL);
	return existsSync(path) ? openPassages(readFileSync(path, "utf8")) : undefined;
}

/** The message that opens a visit. */
export function visitMessage(studio: string, occasion: Occasion): string {
	const why = occasion === "sitting"
		? "The painter has just ended a sitting at the easel, and you've come to see the painting."
		: "The painter has asked you to visit while it works, and you've come to see the painting.";
	const read = existsSync(join(studio, TO_CRITIC))
		? `Read the painter's journal, ${JOURNAL}, and its note to you, ${TO_CRITIC}.`
		: `Read the painter's journal, ${JOURNAL}. The painter hasn't left you a note.`;
	const passages = journalPassages(studio);
	const list = passages
		? `The painter's journal handoff lists the passages it has left open. Its latest list, quoted as the painter wrote it:\n\n${quote(passages)}`
		: `The painter's journal has no \`${OPEN_PASSAGES}\` list yet.`;
	const gets = occasion === "sitting"
		? `It is saved in the studio as ${CRITIQUE}, and the painter's next sitting opens with it.`
		: `The painter reads it as soon as you finish, and it is saved in the studio as ${CRITIQUE}.`;
	return [
		why,
		"Start with the whole canvas, then take only the few views that answer what you noticed there: a detail crop, the value, squint, " +
			"mirror, relief or gallery view, the palette. Don't look again at a view you've already seen. " + read,
		list,
		"Then tell the painter, as one painter talking to another, whether the picture is finished. Open your notes with one line that reads " +
			"`Verdict: finished` or `Verdict: not finished`. The picture is finished when you see no change that would make it clearly better as a " +
			"whole, rather than different, and that is worth the risk to what already holds. A fault you can name isn't enough on its own. The " +
			"painting ends when the painter ends a sitting without marks, so each point you give keeps it at the easel for at least another sitting. " +
			"If it is finished, say what holds and stop there. If it isn't, give the one to five things that matter most to the picture. For each, " +
			"say what you see, what it does to the picture and what you would do about it. You see the painting and the journal, not the painter's " +
			"code, masks or mixtures, so give your method as one way to try. Go with what you see rather than weighing every alternative, and leave " +
			"the rest for a later visit. Write your notes once, as you would say them. Say which passage, if any, you would take up first, whether " +
			"or not it is on the painter's list. Your last message is what the painter gets. " + gets +
			" The painter decides what to do with it. Once you've written your notes, your visit is over.",
	].join("\n\n");
}

/** pi's arguments for a visit; sessions is the visit's own session folder, here is this folder. */
export function criticArgs(studio: string, critic: Critic, occasion: Occasion, sessions: string, here = import.meta.dirname): string[] {
	return ["--print", "--no-extensions", "-e", join(here, "critic.ts"), "--system-prompt", join(here, "critic_system_prompt.md"),
		"--tools", "look,read", "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve", "--no-mcp",
		"--session-dir", sessions, ...critic.args, visitMessage(studio, occasion)];
}

const stamp = () => new Date().toISOString().replace(/[:.]/g, "-");

/**
 * The visit's reply from its session: the last assistant message that is text with no pending tool
 * call (the house runner's session_reply). On the Anthropic/pi-black path pi --print has left stdout
 * empty while the session kept the text (notes/handoff-2026-10-02.md), so the session comes first.
 */
export function sessionReply(sessions: string): string | undefined {
	if (!existsSync(sessions)) return undefined;
	let lastAssistant = true;
	for (const f of readdirSync(sessions).filter((f) => f.endsWith(".jsonl")).sort().reverse()) {
		for (const line of readFileSync(join(sessions, f), "utf8").split("\n").reverse()) {
			let m: { role?: string; stopReason?: string; errorMessage?: string; content?: { type?: string; text?: string }[] } | undefined;
			try {
				m = JSON.parse(line).message;
			} catch {
				continue;
			}
			if (m?.role !== "assistant") continue;
			if (lastAssistant && (m.stopReason === "error" || m.stopReason === "aborted")) {
				throw new Error(`the critic couldn't visit: ${m.errorMessage || m.stopReason}`);
			}
			lastAssistant = false;
			if (m.stopReason === "error" || m.stopReason === "aborted" || !Array.isArray(m.content)) continue;
			if (m.content.some((p) => p.type === "toolCall")) continue;
			const text = m.content.filter((p) => p.type === "text").map((p) => p.text ?? "").join("\n").trim();
			if (text) return text;
		}
	}
	return undefined;
}

/** Keep the painter's note to the critic (ask_critic's note) for this and later visits. */
export function leaveNote(studio: string, critic: Critic, note: string): void {
	mkdirSync(critic.dir, { recursive: true });
	const history = join(critic.dir, "notes-to-critic.jsonl");
	publishNotes(studio, "to-critic", "ask_critic", [
		{ path: join(studio, TO_CRITIC), slot: "current", text: note.trimEnd() + "\n" },
		{ path: history, root: critic.dir, slot: "history", text: (existsSync(history) ? readFileSync(history, "utf8") : "") + JSON.stringify({ at: new Date().toISOString(), note }) + "\n", independent: true },
	]);
}

/** Run one visit; what the critic wrote, as notes/critic.md now holds it. Throws when the critic wrote nothing. */
export async function visit(studio: string, critic: Critic, occasion: Occasion, signal?: AbortSignal, pi = "pi"): Promise<string> {
	mkdirSync(critic.dir, { recursive: true });
	const started = stamp();
	const sessions = join(critic.dir, "sessions", started);
	let timedOut = false;
	const { code, out, err, reply } = await new Promise<{ code: number | null; out: string; err: string }>((done, fail) => {
		if (signal?.aborted) return fail(new Error("Operation aborted"));
		const p = spawn("node", [join(import.meta.dirname, "visit-process.ts"), pi, ...criticArgs(studio, critic, occasion, sessions)],
			{ cwd: studio, stdio: ["pipe", "pipe", "pipe"], detached: true });
		const kill = () => p.stdin.end();
		const timeout = setTimeout(() => {
			timedOut = true;
			kill();
		}, 20 * 60_000);
		const cleanup = () => {
			clearTimeout(timeout);
			signal?.removeEventListener("abort", kill);
		};
		signal?.addEventListener("abort", kill, { once: true });
		p.stdin.on("error", () => {});
		let out = "";
		let err = "";
		p.stdout.on("data", (d) => (out += d));
		p.stderr.on("data", (d) => (err += d));
		p.on("error", (e) => (cleanup(), fail(e)));
		p.on("close", (code) => (cleanup(), done({ code, out, err })));
	}).then((result) => ({
		...result,
		reply: (result.code === 0 ? sessionReply(sessions) : undefined) ?? result.out.trim(),
	})).catch((error) => {
		if (!signal?.aborted) appendFileSync(join(critic.dir, "visits.log"), `${started} ${occasion}: failed: ${error.message}\n`);
		throw error;
	});
	const failure = signal?.aborted ? "Operation aborted"
		: timedOut ? "critic timed out after 20 minutes and was stopped"
		: code !== 0 || !reply ? `the critic couldn't visit: ${err.trim().split("\n").at(-1) || `exit ${code}`}` : undefined;
	appendFileSync(join(critic.dir, "visits.log"), `${started} ${occasion}: ${failure ? `failed: ${failure}` : `exit ${code}`}, ${out.trim().length} characters\n${err}`);
	if (failure) throw new Error(failure);
	const notes = `${HEADER[occasion]}\n\n${reply}\n`;
	publishNotes(studio, "critic", "ask_critic", [
		{ path: join(studio, CRITIQUE), slot: "current", text: notes },
		{ path: join(critic.dir, `${started}-${occasion}.md`), root: critic.dir, slot: `${started}-${occasion}.md`, text: notes, independent: true },
	], signal);
	return notes;
}

/**
 * What opens a later sitting with PAINTER_OPEN_PASSAGES=1 (the house runner from r41), before the runner's
 * message: the journal's latest list of open passages and, with a critic, its latest notes.
 */
export function passagesToTakeUp(studio: string, critic: boolean): string {
	const passages = journalPassages(studio);
	const parts = ["## Passages to take up"];
	parts.push(passages
		? `Your open passages, from your journal handoff (${JOURNAL}), quoted as you wrote them:\n\n${quote(passages)}`
		: `Your journal has no \`${OPEN_PASSAGES}\` list yet. Any passages your last handoff left open are in your journal, quoted below under "Your journal".`);
	const path = join(studio, CRITIQUE);
	if (critic && existsSync(path)) {
		parts.push("## The critic's last visit", `The critic's notes from its last visit (${CRITIQUE}), quoted as the critic wrote them. They are its view of the ` +
			"picture, not your list. Weigh each point against what you see, take up the ones that would make the picture better, and say in your " +
			"journal which you leave and why. It doesn't see your code, masks or mixtures, so a method it suggests is one way to try, at the " +
			"passage's real scale. When it says the picture is finished and you agree, end this sitting without marks." +
			`\n\n${quote(readFileSync(path, "utf8").trimEnd())}`);
	}
	return parts.join("\n\n");
}

/** What a later sitting's opening adds after the summary: the critic's latest notes, if there are any. */
export function sittingCritique(studio: string): string {
	const path = join(studio, CRITIQUE);
	if (!existsSync(path)) return "";
	return `\n\n## The critic's notes (${CRITIQUE})\n\nQuoted as the critic wrote them:\n\n${quote(readFileSync(path, "utf8").trimEnd())}`;
}
