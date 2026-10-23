// node --test harness/painter/test/critic.test.ts  (TMPDIR: a scratch directory; the last test runs pi)
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { criticArgs, criticFromEnv, criticReadable, passagesToTakeUp, sessionReply, sittingCritique, visit, visitMessage } from "../visit.ts";

const HERE = join(import.meta.dirname, "..");

function scratch(name: string): string {
	return mkdtempSync(join(process.env.TMPDIR || tmpdir(), name));
}

function studio(): string {
	const s = join(scratch("critic-test-"), "studio");
	for (const d of ["notes", "out/easel/painting", "paintings/lua"]) mkdirSync(join(s, d), { recursive: true });
	writeFileSync(join(s, "BRIEF.md"), "# Paint a picture\n");
	writeFileSync(join(s, "notes/journal.md"), "- day 1, 09:00: began\n");
	return s;
}

test("PAINTER_CRITIC names the model's arguments and a folder", () => {
	assert.equal(criticFromEnv({}), undefined);
	assert.deepEqual(criticFromEnv({ PAINTER_CRITIC: '{"args": ["--model", "m"], "dir": "/x"}' }), { args: ["--model", "m"], dir: "/x" });
	for (const bad of ['{"args": "m", "dir": "/x"}', '{"args": []}', "[]"]) assert.throws(() => criticFromEnv({ PAINTER_CRITIC: bad }));
});

test("the critic reads the journal, the note to it and the easel's looks, nothing else", () => {
	const s = studio();
	for (const ok of ["notes/journal.md", "notes/to_critic.md", "out/easel/painting/a.png", "./notes/../notes/journal.md"]) assert.ok(criticReadable(s, ok), ok);
	for (const no of ["BRIEF.md", "notes/critic.md", "notes/easel_guide.md", "paintings/lua/painting.lua", "out/easel/painting/server.log", "../journal.md", "/etc/passwd"]) {
		assert.equal(criticReadable(s, no), undefined, no);
	}
});

test("a visit's message says what there is to read and where the notes go", () => {
	const s = studio();
	assert.match(visitMessage(s, "sitting"), /just ended a sitting[\s\S]*The painter hasn't left you a note\.[\s\S]*the painter gets it at the start of its next sitting/);
	writeFileSync(join(s, "notes/to_critic.md"), "the sky?\n");
	assert.match(visitMessage(s, "asked"), /asked you to visit[\s\S]*its note to you, notes\/to_critic\.md\.[\s\S]*as soon as you finish/);
	const args = criticArgs(s, { args: ["--model", "m"], dir: "/c" }, "asked", "/c/sessions/v", "/h");
	assert.deepEqual(args.slice(0, 12), ["--print", "--no-extensions", "-e", "/h/critic.ts", "--system-prompt", "/h/critic_system_prompt.md", "--tools", "look,read", "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]);
	assert.deepEqual(args.slice(-4, -1), ["/c/sessions/v", "--model", "m"]);
});

test("a visit's message quotes the painter's open passages and asks which the critic would take up first", () => {
	const s = studio();
	assert.match(visitMessage(s, "sitting"), /The painter's journal has no `Open passages:` list yet\.\n\nThen tell the painter, as one painter talking to another, whether the picture is finished\. Open your notes with one line that reads `Verdict: finished` or `Verdict: not finished`\.[\s\S]*Say what you would take up first: a passage on the painter's list, one off it or something across the whole picture\./);
	writeFileSync(join(s, "notes/journal.md"), "- day 1, 09:00: began\n- day 2, 18:00: handoff\n  Open passages:\n  - the far shore: flat\n");
	assert.match(visitMessage(s, "asked"), /Its latest list, quoted as the painter wrote it:\n\n> Open passages:\n> - the far shore: flat\n\nThen tell the painter/);
});

test("a later sitting's passages to take up: the open passages, then the critic's last visit under its own heading", () => {
	const s = studio();
	assert.equal(passagesToTakeUp(s, true), "## Passages you left open\n\nYour journal has no `Open passages:` list yet. Any passages your last handoff left open are in your journal, quoted below under \"Your journal\".");
	writeFileSync(join(s, "notes/journal.md"), "- day 2, 18:00: Open passages:\n  - the far shore: flat\n");
	writeFileSync(join(s, "notes/critic.md"), "From the critic's visit at the end of a sitting:\n\nLift the sky.\n");
	assert.equal(passagesToTakeUp(s, false), "## Passages you left open\n\nYour open passages, from your journal handoff (notes/journal.md), quoted as you wrote them. They are what you noted last time, not the order of work: weigh each against the picture as a whole.\n\n> Open passages:\n> - the far shore: flat");
	assert.equal(passagesToTakeUp(s, true), passagesToTakeUp(s, false) +
		"\n\n## The critic's last visit\n\nThe critic's notes from its last visit (notes/critic.md), quoted as the critic wrote them. They are its view of the picture, not your list. " +
		"Weigh each point against what you see, take up the ones that would make the picture better, and say in your journal which you leave and why. " +
		"A point isn't first just because it is named: it competes with whatever would do most for the picture now. " +
		"It doesn't see your code, masks or mixtures, so a method it suggests is one way to try, at the passage's real scale. " +
		"When it says the picture is finished and you agree, end this sitting without marks.\n\n> From the critic's visit at the end of a sitting:\n>\n> Lift the sky.");
});

test("a visit writes the critic's notes to the studio and keeps a copy; a failed one changes nothing", async () => {
	const s = studio();
	const dir = scratch("critic-dir-");
	const good = join(dir, "good-pi");
	writeFileSync(good, "#!/bin/sh\necho 'The horizon sits too low.'\n", { mode: 0o755 });
	const notes = await visit(s, { args: [], dir }, "asked", undefined, good);
	assert.equal(notes, "From the critic's visit you asked for:\n\nThe horizon sits too low.\n");
	assert.equal(readFileSync(join(s, "notes/critic.md"), "utf8"), notes);
	assert.equal(readdirSync(dir).filter((f) => f.endsWith("-asked.md")).length, 1);
	assert.match(sittingCritique(s), /^\n\n## The critic's notes \(notes\/critic\.md\)\n\nQuoted as the critic wrote them:\n\n> From the critic's visit you asked for:\n>\n> The horizon sits too low\.$/);
	const bad = join(dir, "bad-pi");
	writeFileSync(bad, "#!/bin/sh\necho 'no model' >&2\nexit 3\n", { mode: 0o755 });
	await assert.rejects(visit(s, { args: [], dir }, "sitting", undefined, bad), /the critic couldn't visit: no model/);
	assert.equal(readFileSync(join(s, "notes/critic.md"), "utf8"), notes);
	assert.equal(sittingCritique(studio()), "");
});

test("a visit's reply comes from its session when pi printed nothing", async () => {
	const s = studio();
	const dir = scratch("critic-session-");
	const quiet = join(dir, "quiet-pi");
	// a pi that saves the reply in its session (--session-dir is the argument after it) and prints nothing
	writeFileSync(quiet, `#!/bin/sh
while [ "$1" != "--session-dir" ]; do shift; done
mkdir -p "$2"
printf '%s\\n' '{"message":{"role":"assistant","content":[{"type":"toolCall","name":"look"}]}}' '{"message":{"role":"assistant","stopReason":"error","content":[{"type":"text","text":"overloaded"}]}}' '{"message":{"role":"assistant","stopReason":"stop","content":[{"type":"text","text":"Lift the sky."}]}}' > "$2/a.jsonl"
`, { mode: 0o755 });
	assert.equal(await visit(s, { args: [], dir }, "sitting", undefined, quiet), "From the critic's visit at the end of a sitting:\n\nLift the sky.\n");
	assert.equal(sessionReply(join(dir, "missing")), undefined);
});

test("the critic visits at the end of a sitting, and the next sitting opens with its notes", () => {
	const tmp = process.env.TMPDIR;
	assert.ok(tmp, "set TMPDIR to a scratch directory");
	const s = studio();
	const dir = scratch("critic-pi-");
	const out = join(dir, "requests.jsonl");
	const faux = ["-e", join(HERE, "test/fixtures/faux-record.ts"), "--provider", "faux", "--model", "m"];
	const env = { ...process.env, FAUX_OUT: out, PAINTER_CRITIC: JSON.stringify({ args: faux, dir: join(dir, "critic") }) };
	const sitting = (message: string, more: Record<string, string> = {}) => execFileSync("pi", ["--print", "--no-extensions", "-e", join(HERE, "painter.ts"),
		"--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve", "--session-dir", join(dir, "sessions"), ...faux, message],
		{ cwd: s, env: { ...env, ...more }, stdio: ["ignore", "pipe", "pipe"], encoding: "utf8" });
	writeFileSync(out, "");
	assert.equal(sitting("Your brief is in BRIEF.md.").trim(), "done");
	const requests = readFileSync(out, "utf8").trim().split("\n").map((l) => JSON.parse(l) as { role: string; text: string }[]);
	assert.deepEqual(requests.map((r) => r.find((m) => m.role === "user")!.text.slice(0, 40)), ["Your brief is in BRIEF.md.", "The painter has just ended a sitting at "]);
	assert.equal(readFileSync(join(s, "notes/critic.md"), "utf8"), "From the critic's visit at the end of a sitting:\n\ndone\n");
	assert.equal(readdirSync(join(dir, "critic/sessions")).length, 1);   // one visit, its own session folder
	assert.equal(readdirSync(join(dir, "sessions")).length, 1);   // the critic's session isn't among the painter's
	writeFileSync(out, "");
	sitting("You're back at the easel.", { PAINTER_SITTING_RECOVERY: "1" });
	const [next] = readFileSync(out, "utf8").trim().split("\n").map((l) => JSON.parse(l) as { role: string; text: string }[]);
	assert.match(next.find((m) => m.role === "user")!.text, /^You're back at the easel\.\n\n## BRIEF\.md[\s\S]*## The critic's notes \(notes\/critic\.md\)\n\nQuoted as the critic wrote them:\n\n> From the critic's visit at the end of a sitting:\n>\n> done$/);
	writeFileSync(out, "");
	writeFileSync(join(s, "notes/journal.md"), "- day 2, 18:00: Open passages:\n  - the far shore: flat\n");
	sitting("You're back at the easel.", { PAINTER_SITTING_RECOVERY: "1", PAINTER_OPEN_PASSAGES: "1" });
	const [opened] = readFileSync(out, "utf8").trim().split("\n").map((l) => JSON.parse(l) as { role: string; text: string }[]);
	const text = opened.find((m) => m.role === "user")!.text;
	assert.match(text, /^You're back at the easel\.\n\n## Passages you left open\n\nYour open passages[^\n]*\n\n> Open passages:\n> - the far shore: flat\n\n## The critic's last visit\n\nThe critic's notes from its last visit[^\n]*\n\n> From the critic's visit at the end of a sitting:\n>\n> done\n\n## BRIEF\.md/);
	assert.doesNotMatch(text, /## The critic's notes/);
});
