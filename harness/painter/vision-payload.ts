/**
 * The payload side of vision.ts, without pi imports (node --test loads it). Changes the payload
 * in place and returns it.
 *
 *  - OpenAI: every `input_image` gets `detail`.
 *  - Gemini 3: the ultra-high media resolution exists only per image part, and an image nested in
 *    a function response (pi's Gemini 3 layout: functionResponse.parts) refuses the field
 *    ("Unknown name mediaResolution at ...function_response.parts[0]"). So a tool result's images
 *    move into a user turn right after it, as pi lays them out for Gemini before 3
 *    ("Tool result image:" and the images), each with the ultra-high resolution. Other image
 *    parts get it in place.
 */
export const GEMINI_LEVEL = "media_resolution_ultra_high";

type Part = Record<string, any>;
type Content = { role?: string; parts?: Part[] };

const isImage = (p: Part) => typeof p?.inlineData?.mimeType === "string" && p.inlineData.mimeType.startsWith("image/");
const sharp = (p: Part): Part => ({ ...p, mediaResolution: { level: GEMINI_LEVEL } });

export function sharpenGemini(contents: Content[]): Content[] {
	const out: Content[] = [];
	for (const c of contents) {
		const moved: Part[] = [];
		for (const p of c.parts ?? []) {
			const fr = p.functionResponse;
			if (fr && Array.isArray(fr.parts) && fr.parts.some(isImage)) {
				moved.push(...fr.parts.filter(isImage).map(sharp));
				fr.parts = fr.parts.filter((q: Part) => !isImage(q));
				if (fr.parts.length === 0) delete fr.parts;
			}
		}
		c.parts = (c.parts ?? []).map((p) => (isImage(p) ? sharp(p) : p));
		out.push(c);
		if (moved.length) out.push({ role: "user", parts: [{ text: "Tool result image:" }, ...moved] });
	}
	return out;
}

export function sharpen<T>(payload: T, detail: string): T {
	const p = payload as Record<string, any>;
	if (p && Array.isArray(p.contents)) p.contents = sharpenGemini(p.contents);
	const walk = (x: unknown): void => {
		if (Array.isArray(x)) return x.forEach(walk);
		if (!x || typeof x !== "object") return;
		const o = x as Record<string, unknown>;
		if (o.type === "input_image") o.detail = detail;
		for (const v of Object.values(o)) walk(v);
	};
	walk(payload);
	return payload;
}
