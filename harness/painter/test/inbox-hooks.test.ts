import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { test } from "node:test";
import { registerInbox } from "../inbox.ts";

const TELL = resolve(import.meta.dirname, "../../../scripts/tell-painter");

function harness(enabled = true) {
	assert.ok(process.env.TMPDIR, "set TMPDIR to a scratch directory");
	const run = mkdtempSync(join(process.env.TMPDIR, "inbox-hooks-"));
	process.env.PAINTER_INBOX_DIR = enabled ? run : "";
	process.env.PAINTER_INBOX_SITTING = "2";
	const handlers: Record<string, (...args: any[]) => any> = {};
	registerInbox({ on: (name, handler) => { handlers[name] = handler; } } as never);
	const session = join(run, "session.jsonl");
	return { run, session, handlers, ctx: { sessionManager: { getSessionFile: () => session } },
		send: (text: string) => execFileSync(TELL, [run, text]),
		log: () => JSON.parse(readFileSync(join(run, "painter-inbox.json"), "utf8")) };
}

test("disabled inbox registers nothing; empty inbox transforms nothing and writes nothing", () => {
	assert.deepEqual(harness(false).handlers, {});
	const { handlers, ctx, run } = harness();
	assert.equal(handlers.input({ text: "Opening." }, ctx), undefined);
	assert.equal(handlers.tool_result({ content: [{ type: "text", text: "ok" }] }, ctx), undefined);
	handlers.context();
	handlers.agent_end();
	assert.deepEqual(readdirSync(run), []);
});

test("tool delivery preserves the result and commits its transcript receipt once", () => {
	const { handlers, ctx, session, send, log } = harness();
	handlers.input({ text: "Opening." }, ctx);
	send("Mid-sitting.");
	const event = { toolCallId: "call-1", toolName: "status", content: [{ type: "text", text: "ok" }], details: { kept: true } };
	assert.equal(handlers.tool_result({ ...event, parentToolCallId: "parent" }, ctx), undefined);
	const reply = handlers.tool_result(event, ctx);
	assert.deepEqual(reply.content[0], event.content[0]);
	assert.equal(reply.content[1].text, "A message from the person who runs this studio:\n\nMid-sitting.\n\nEnd of message.");
	assert.equal(reply.details.kept, true);
	assert.equal(log()[0].delivery.state, "attaching");
	writeFileSync(session, JSON.stringify({ message: { role: "toolResult", toolCallId: "call-1", timestamp: 1234567890000, ...reply } }) + "\n");
	handlers.context();
	assert.equal(log()[0].delivery.state, "delivered");
	assert.equal(log()[0].delivery.time, "2009-02-13T23:31:30+00:00");
	assert.equal(log()[0].delivery.sitting, "2");
	assert.equal(handlers.tool_result({ ...event, toolCallId: "call-2" }, ctx), undefined);
});

test("between-sitting messages precede the complete opening and retain images", () => {
	const { handlers, ctx, send, session, log } = harness();
	send("First.");
	send("Second.");
	const images = [{ type: "image", data: "unchanged" }];
	const reply = handlers.input({ text: "Recovery and opening.", images }, ctx);
	assert.equal(reply.action, "transform");
	assert.equal(reply.images, images);
	assert.match(reply.text, /^A message from the person who runs this studio:\n\nFirst\.\n\nEnd of message\.\n\nA message/);
	assert.ok(reply.text.endsWith("Second.\n\nEnd of message.\n\nRecovery and opening."));
	writeFileSync(session, JSON.stringify({ message: { role: "user", content: reply.text } }) + "\n");
	handlers.agent_end();
	assert.deepEqual(log().map((m: any) => [m.delivery.state, m.delivery.tool_call]), [["delivered", null], ["delivered", null]]);
});
