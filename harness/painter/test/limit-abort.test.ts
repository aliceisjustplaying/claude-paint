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

test("monthly usage-limit waits schedule bounded timers without retrying immediately", async (t) => {
	for (const mode of ["timer completes", "abort during wait", "remaining give-up duration"]) {
		await t.test(mode, async (t) => {
			const env = { ...process.env };
			t.after(() => { process.env = env; });
			for (const key of Object.keys(process.env).filter((key) => key.startsWith("PAINTER_"))) delete process.env[key];
			process.env.PAINTER_LIMIT_GIVE_UP_H = "744";
			t.mock.timers.enable({ apis: ["Date", "setTimeout"], now: 0 });
			const schedule = t.mock.method(globalThis, "setTimeout");
			const handlers: Record<string, (...args: any[]) => any> = {};
			painter({ on: (name, handler) => { handlers[name] = handler; } } as never);
			const message = { role: "assistant", stopReason: "error", errorMessage: "Usage limit reached. Resets in 30 days" };
			handlers.message_end({ message });
			if (mode === "remaining give-up duration") t.mock.timers.tick(30 * 86_400_000);
			const wait = mode === "remaining give-up duration" ? 86_400_000 : 2_147_483_647;
			const controller = new AbortController();
			const event = { context: { contextEntries: [{ sourceEntry: { type: "message", id: "limit", message } }] } };
			let settled = false;
			const pending = handlers.agent_before_settle(event, { signal: controller.signal })
				.then((result: unknown) => { settled = true; return result; });
			assert.equal(schedule.mock.callCount(), 1);
			assert.equal(schedule.mock.calls[0].arguments[1], wait);
			assert.ok(schedule.mock.calls.every(({ arguments: args }) => Number.isFinite(args[1]) && args[1] > 0 && args[1] <= 2_147_483_647));
			t.mock.timers.tick(1);
			await setImmediate();
			assert.equal(settled, false);
			if (mode === "abort during wait") controller.abort();
			else {
				t.mock.timers.tick(wait - 2);
				await setImmediate();
				assert.equal(settled, false);
				t.mock.timers.tick(1);
			}
			assert.deepEqual(await pending, mode === "abort during wait" ? undefined : {
				entries: [{ type: "context_edit", targetId: "limit", replacement: null }], continue: true,
			});
			assert.equal(getEventListeners(controller.signal, "abort").length, 0);
			if (mode === "remaining give-up duration") {
				handlers.message_end({ message });
				assert.equal(await handlers.agent_before_settle(event, { signal: controller.signal }), undefined);
				assert.equal(schedule.mock.callCount(), 1);
			}
		});
	}
});
