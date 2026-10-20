import assert from "node:assert/strict";
import { getEventListeners } from "node:events";
import { registerHooks } from "node:module";
import { test } from "node:test";
import { setImmediate } from "node:timers/promises";

const hooks = registerHooks({
	resolve(specifier, context, nextResolve) {
		const source = specifier === "@earendil-works/pi-ai" ? "export const Type = {};"
			: specifier === "@earendil-works/pi-coding-agent" ? "export const createReadToolDefinition = () => ({}); export const defineTool = x => x;"
			: specifier === "./easel-tools.ts" ? "export const registerEaselTools = () => {};" : undefined;
		return source ? { url: `data:text/javascript,${encodeURIComponent(source)}`, shortCircuit: true } : nextResolve(specifier, context);
	},
});
const { default: painter } = await import("../painter.ts");
hooks.deregister();

test("the token pacing context wait respects host cancellation", async (t) => {
	for (const mode of ["abort during wait", "already aborted", "timer completes", "no signal"]) {
		await t.test(mode, async (t) => {
			for (const key of Object.keys(process.env).filter((key) => key.startsWith("PAINTER_"))) t.mock.env(key, undefined);
			process.env.PAINTER_INPUT_TPM = "100";
			t.mock.method(Date, "now", () => 1_000);
			const handlers: Record<string, (...args: any[]) => any> = {};
			painter({ on: (name, handler) => { handlers[name] = handler; }, appendEntry: () => {} } as never);
			handlers.message_end({ message: { role: "assistant", usage: { input: 100 } } });
			const controller = new AbortController();
			const signal = mode === "no signal" ? undefined : controller.signal;
			const timer = {} as ReturnType<typeof setTimeout>;
			let fire: (() => void) | undefined;
			const schedule = t.mock.method(globalThis, "setTimeout", (callback: () => void) => { fire = callback; return timer; });
			const clear = t.mock.method(globalThis, "clearTimeout", () => {});
			if (mode === "already aborted") controller.abort();
			let settled = false;
			const pending = handlers.context({ messages: [] }, { signal })
				.then((result: unknown) => { settled = true; return result; });
			if (mode !== "already aborted") {
				assert.ok(fire, "the production hook scheduled its token pacing wait");
				assert.equal(schedule.mock.calls[0]?.arguments[1], 60_000);
				assert.equal(settled, false);
				if (signal) assert.equal(getEventListeners(signal, "abort").length, 1);
				if (mode === "abort during wait") controller.abort();
				else fire();
			}
			await setImmediate();
			assert.equal(settled, true, "the hook must settle without firing its timer after an abort");
			assert.equal(await pending, undefined);
			if (mode === "already aborted") assert.equal(schedule.mock.calls.length, 0);
			if (mode === "abort during wait") assert.equal(clear.mock.calls[0]?.arguments[0], timer);
			if (signal) assert.equal(getEventListeners(signal, "abort").length, 0);
		});
	}
});

test("the usage-limit settle wait respects host cancellation", async (t) => {
	for (const mode of ["abort during wait", "already aborted", "timer completes", "no signal"]) {
		await t.test(mode, async (t) => {
			for (const key of Object.keys(process.env).filter((key) => key.startsWith("PAINTER_"))) t.mock.env(key, undefined);
			const handlers: Record<string, (...args: any[]) => any> = {};
			painter({ on: (name, handler) => { handlers[name] = handler; } } as never);
			const message = { role: "assistant", stopReason: "error", errorMessage: "Usage limit reached. Resets in 2hr 15min" };
			handlers.message_end({ message });
			const controller = new AbortController();
			const signal = mode === "no signal" ? undefined : controller.signal;
			const timer = {} as ReturnType<typeof setTimeout>;
			let fire: (() => void) | undefined;
			t.mock.method(globalThis, "setTimeout", (callback: () => void) => { fire = callback; return timer; });
			const clear = t.mock.method(globalThis, "clearTimeout", () => {});
			if (mode === "already aborted") controller.abort();
			let settled = false;
			const pending = handlers.agent_before_settle({ context: { contextEntries: [{ sourceEntry: { type: "message", id: "limit", message } }] } }, { signal })
				.then((result: unknown) => { settled = true; return result; });
			if (mode === "abort during wait") {
				assert.ok(fire, "the production hook scheduled its limit wait");
				assert.equal(settled, false);
				controller.abort();
			} else if (mode !== "already aborted") {
				assert.ok(fire);
				fire();
			}
			await setImmediate();
			assert.equal(settled, true, "the hook must settle without firing its timer after an abort");
			assert.deepEqual(await pending, mode.includes("abort") ? undefined : {
				entries: [{ type: "context_edit", targetId: "limit", replacement: null }], continue: true,
			});
			if (mode === "already aborted") assert.equal(fire, undefined);
			if (mode === "abort during wait") assert.equal(clear.mock.calls[0]?.arguments[0], timer);
			if (signal) assert.equal(getEventListeners(signal, "abort").length, 0);
		});
	}
});
