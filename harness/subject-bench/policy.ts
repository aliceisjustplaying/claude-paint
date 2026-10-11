export function choice(text: string, streaming = false): string | undefined {
	const marker = /(?:^|\n)\s*(?:#{1,6}\s*)?(?:plan|subject|title)\s*:\s*[^\n]+/im.exec(text);
	if (marker && (!streaming || /[.!?]$/.test(marker[0].trim()) || text[marker.index + marker[0].length] === "\n")) return marker[0].trim();
	return /\b(?:I['’]ll|I will|I want to|I['’]m going to)\s+(?:paint|depict|compose)\s+[^.!?\n]+[.!?]/i.exec(text)?.[0];
}

export function visible(message: any): string {
	return (message.content ?? []).flatMap((c: any) => c.type === "text" ? [c.text]
		: c.type === "toolCall" && c.name === "note" ? [c.arguments?.text ?? ""] : []).join("\n");
}

export function reserve(model: any): number {
	return model.route === "codex" ? 0 : (model.contextWindow * Math.max(model.cost.input, model.cost.cacheRead, model.cost.cacheWrite)
		+ model.maxTokens * model.cost.output) / 1e6;
}

export function withinContext(payload: unknown, tokens: number): boolean {
	const serialized = JSON.stringify(payload, function (key, value) {
		if (key === "url" && typeof value === "string" && value.startsWith("data:image/")) return "";
		if (key === "data" && typeof value === "string" && this.type === "base64" && this.media_type?.startsWith("image/")) return "";
		return value;
	});
	const images = (serialized.match(/"(?:type|mime_type|mimeType)"\s*:\s*"(?:image|image_url|input_image|image\/[^" ]+)"/g) ?? []).length;
	return Buffer.byteLength(serialized, "utf8") + images * 65536 <= tokens;
}

export function changedCanvas(before: string | undefined, after: string | undefined, lua: string): boolean {
	const simple = lua.replace(/--[^\n]*/g, "").trim();
	const waitOnly = /^(?:(?:print\(\s*)?wait\([\d\s.]+\)\)?\s*;?\s*)+$/.test(simple);
	return Boolean(before && after && before !== after && !waitOnly);
}
