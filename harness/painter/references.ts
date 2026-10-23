import { existsSync, readFileSync } from "node:fs";
import { studioPath } from "./easel-client.ts";

interface Reference {
	id: string;
	title: string;
	keywords: string[];
	text: string;
}

function library(studio: string): Reference[] {
	const path = studioPath(studio, "notes/reference-library.json");
	if (!path || !existsSync(path)) throw new Error("reference: this studio has no reference library");
	const source = readFileSync(path, "utf8");
	let entries: unknown;
	try {
		entries = JSON.parse(source);
	} catch (error) {
		throw new Error(`reference: notes/reference-library.json isn't valid JSON: ${(error as Error).message}`);
	}
	if (!Array.isArray(entries)) throw new Error("reference: notes/reference-library.json must be a list of entries");
	const seen = new Set<string>();
	for (const [index, entry] of entries.entries()) {
		if (!entry || typeof entry !== "object" || Array.isArray(entry)) {
			throw new Error(`reference: notes/reference-library.json entry ${index + 1} must be an object with id, title, keywords and text`);
		}
		const label = typeof entry.id === "string" && entry.id.trim() ? JSON.stringify(entry.id) : String(index + 1);
		for (const field of ["id", "title", "text"]) {
			if (typeof entry[field] !== "string" || !entry[field].trim()) {
				throw new Error(`reference: notes/reference-library.json entry ${label}: ${field} must be a nonblank string`);
			}
		}
		if (!Array.isArray(entry.keywords) || !entry.keywords.every((word: unknown) => typeof word === "string")) {
			throw new Error(`reference: notes/reference-library.json entry ${label}: keywords must be a list of strings`);
		}
		const { id } = entry;
		if (seen.has(id)) throw new Error(`reference: notes/reference-library.json has duplicate id "${id}"; give each entry its own id`);
		seen.add(id);
	}
	return entries as Reference[];
}

export function referenceList(studio: string, params: Record<string, unknown>) {
	if (Object.keys(params).length) throw new Error("reference_list: takes no arguments");
	return { references: library(studio).map(({ id, title }) => ({ id, title })) };
}

export function reference(studio: string, params: { query?: string; id?: string; page?: number }) {
	if (Object.keys(params).some((key) => !["query", "id", "page"].includes(key))) throw new Error("reference: unknown parameter");
	const hasQuery = params.query !== undefined;
	const hasId = params.id !== undefined;
	if (hasQuery === hasId) throw new Error("reference: give exactly one of query or id");
	if (hasQuery && (typeof params.query !== "string" || !params.query.trim())) throw new Error("reference: query must contain words");
	if (hasQuery && params.page !== undefined) throw new Error("reference: page goes with id");
	if (hasId && (typeof params.id !== "string" || !params.id.trim())) throw new Error("reference: id must be a search result id");
	const entries = library(studio);
	if (hasQuery) {
		const words = params.query!.toLowerCase().trim().split(/\s+/u);
		const matches = entries.filter((entry) => {
			const content = `${entry.title} ${entry.keywords.join(" ")} ${entry.text}`.toLowerCase();
			return words.every((word) => content.includes(word));
		}).map(({ id, title }) => ({ id, title }));
		return matches.length ? { matches } : { matches, message: "No matching references." };
	}
	const entry = entries.find((entry) => entry.id === params.id);
	if (!entry) throw new Error("reference: unknown id; use an id from a search result");
	const pages: string[] = [];
	let page = "", bytes = 0;
	for (const character of entry.text) {
		const size = Buffer.byteLength(character);
		if (bytes + size > 4000) {
			pages.push(page);
			page = "";
			bytes = 0;
		}
		page += character;
		bytes += size;
	}
	if (page || pages.length === 0) pages.push(page);
	const number = params.page === undefined ? 1 : params.page;
	if (!Number.isInteger(number) || number < 1 || number > pages.length) throw new Error(`reference: pages run from 1 to ${pages.length}`);
	return { id: entry.id, title: entry.title, page: number, pages: pages.length, next_page: number < pages.length ? number + 1 : null, text: pages[number - 1] };
}
