import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { test } from "node:test";

const HERE = resolve(import.meta.dirname, "..");
const TELL = resolve(HERE, "../../scripts/tell-painter");

function studio() {
	assert.ok(process.env.TMPDIR, "set TMPDIR to a scratch directory");
	const dir = mkdtempSync(join(process.env.TMPDIR, "inbox-test-"));
	const run = join(dir, "run");
	mkdirSync(run);
	writeFileSync(join(dir, "BRIEF.md"), "Paint a picture.\n");
	writeFileSync(join(dir, ".painter-run"), run + "\n");
	return { dir, run };
}

function launch(at: ReturnType<typeof studio>, enabled = true, send = false, sitting = "1") {
	execFileSync("pi", ["--print", "--no-extensions", "-e", join(HERE, "painter.ts"),
		"-e", join(HERE, "test/fixtures/faux-inbox.ts"), "--tools", "read", "--no-context-files", "--no-skills",
		"--no-prompt-templates", "--no-approve", "--session-dir", join(at.dir, "sessions"),
		"--provider", "faux", "--model", "m", "Opening message."], { cwd: at.dir,
		env: { ...process.env, PAINTER_INBOX_DIR: enabled ? at.run : "", PAINTER_INBOX_SITTING: sitting,
			INBOX_TEST_SEND: send ? TELL : "" }, stdio: ["ignore", "pipe", "pipe"] });
	const sessions = readdirSync(join(at.dir, "sessions")).sort();
	return sessions.map((f) => readFileSync(join(at.dir, "sessions", f), "utf8")).join("\n").split("\n")
		.filter(Boolean).map((l) => JSON.parse(l)).filter((e) => e.type === "message").map((e) => e.message);
}

test("mid-sitting delivery is in the tool result and stays delivered across a process restart", () => {
	const at = studio();
	const transcript = launch(at, true, true);
	const result = transcript.find((m) => m.role === "toolResult");
	assert.match(JSON.stringify(result.content), /A message from the person who runs this studio:/);
	assert.match(JSON.stringify(result.content), /The mid-sitting message\./);
	assert.match(JSON.stringify(result.content), /End of message\./);
	let log = JSON.parse(readFileSync(join(at.run, "painter-inbox.json"), "utf8"));
	assert.equal(log[0].delivery.state, "delivered");
	assert.equal(log[0].delivery.tool_call, "inbox-read");
	assert.equal(log[0].delivery.sitting, "1");
	execFileSync(TELL, [at.run, "The between-sittings message."]);
	const after = launch(at, true, false, "2");
	const users = after.filter((m) => m.role === "user");
	assert.match(JSON.stringify(users.at(-1).content), /A message from the person who runs this studio.*The between-sittings message.*End of message.*Opening message/);
	assert.equal(JSON.stringify(after).split("The mid-sitting message.").length - 1, 1);
	log = JSON.parse(readFileSync(join(at.run, "painter-inbox.json"), "utf8"));
	assert.equal(log[1].delivery.state, "delivered");
	assert.equal(log[1].delivery.sitting, "2");
	assert.equal(log[1].delivery.tool_call, null);
});

test("an enabled empty inbox leaves user and tool content identical to disabled", () => {
	const enabled = launch(studio());
	const disabled = launch(studio(), false);
	const content = (messages: typeof enabled) => messages.map((m) => [m.role, m.content, m.details]);
	assert.deepEqual(content(enabled), content(disabled));
});
