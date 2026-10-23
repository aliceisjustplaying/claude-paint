import { AgentSession, type ExtensionContext } from "@earendil-works/pi-coding-agent";

const KEY = Symbol.for("stillwet.painter.host-cancellation");
type Cancel = () => Promise<unknown> | undefined;
type Registry = WeakMap<ExtensionContext["sessionManager"], Cancel>;

export function bindHostCancellation(manager: ExtensionContext["sessionManager"], cancel: Cancel): () => void {
	const prototype = AgentSession.prototype as typeof AgentSession.prototype & { [KEY]?: Registry };
	const registry = prototype[KEY] ?? new WeakMap();
	if (!prototype[KEY]) {
		const abort = prototype.abort;
		prototype[KEY] = registry;
		prototype.abort = async function () {
			const stopped = registry.get(this.sessionManager)?.();
			await Promise.all([abort.call(this), stopped]);
		};
	}
	registry.set(manager, cancel);
	return () => { if (registry.get(manager) === cancel) registry.delete(manager); };
}
