/**
 * What the reader may touch (reader.ts checks every read and write against it): the files it was
 * given to read (the session logs, the journal, its brief) and the one record it writes. The
 * runner passes them in READER_SCOPE as JSON, {"read": [absolute paths], "write": absolute path}.
 * Paths are compared by their real paths, links followed, so no spelling, `..` or link reaches
 * another file; the record may not be a link, and none of the evidence is writable.
 */
import { lstatSync, realpathSync } from "node:fs";
import { homedir } from "node:os";
import { basename, dirname, isAbsolute, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

export interface ReaderScope {
	read: Set<string>;
	write: string;
}

/** The scope from READER_SCOPE's JSON; throws if it is missing or bad (no scope, no reader). */
export function readerScope(json: string | undefined): ReaderScope {
	if (!json) throw new Error("READER_SCOPE is not set: the reader has nothing it may read or write");
	let s: { read?: unknown; write?: unknown };
	try {
		s = JSON.parse(json);
	} catch {
		throw new Error("READER_SCOPE is not JSON");
	}
	if (!Array.isArray(s.read) || !s.read.length || s.read.some((p) => typeof p !== "string" || !isAbsolute(p)))
		throw new Error("READER_SCOPE.read: want a list of absolute paths");
	if (typeof s.write !== "string" || !isAbsolute(s.write)) throw new Error("READER_SCOPE.write: want an absolute path");
	const read = new Set(s.read.map((p) => realpathSync(p)));   // throws for evidence that isn't there
	const write = join(realpathSync(dirname(s.write)), basename(s.write));
	if (read.has(write) || read.has(real(write) ?? "")) throw new Error("READER_SCOPE.write is one of the files to read");
	return { read, write };
}

/** The real path of a file the reader may read (its record too, once written), or undefined. */
export function readPath(scope: ReaderScope, cwd: string, path: string): string | undefined {
	const p = real(spelled(cwd, path));
	return p && (scope.read.has(p) || p === scope.write) ? p : undefined;
}

/** The record's path if `path` names it, or undefined (anything else, or the record made a link). */
export function writePath(scope: ReaderScope, cwd: string, path: string): string | undefined {
	const full = spelled(cwd, path);
	const dir = real(dirname(full));
	if (!dir || join(dir, basename(full)) !== scope.write) return undefined;
	try {
		if (lstatSync(scope.write).isSymbolicLink()) return undefined;
	} catch {}
	return scope.write;
}

/** The content the reader may write: a structured record (a .json record) must be JSON, so the
 * reader fixes its syntax in the session; what the JSON says is checked by the runner. Throws "not
 * JSON: ..." if it isn't. A free-text record (.md) is written as it is. */
export function checkRecordContent(scope: ReaderScope, content: unknown): void {
	if (!scope.write.endsWith(".json")) return;
	if (typeof content !== "string") throw new Error("not JSON: the content isn't text");
	try {
		JSON.parse(content);
	} catch (e) {
		throw new Error(`not JSON: ${(e as Error).message}; write ${basename(scope.write)} again as JSON`);
	}
}

/** An absolute path as pi's tools would read `path` (`@` prefix, `~`, `file://`, relative to cwd). */
function spelled(cwd: string, path: string): string {
	let p = path.replace(/[\u00A0\u2000-\u200A\u202F\u205F\u3000]/g, " ");
	if (p.startsWith("@")) p = p.slice(1);
	if (p === "~") p = homedir();
	else if (p.startsWith("~/")) p = join(homedir(), p.slice(2));
	else if (/^file:\/\//.test(p)) p = fileURLToPath(p);
	return resolve(cwd, p);
}

function real(p: string): string | undefined {
	try {
		return realpathSync(p);
	} catch {
		return undefined;
	}
}
