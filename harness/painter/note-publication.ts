import { accessSync, chmodSync, closeSync, constants, existsSync, fsyncSync, linkSync, lstatSync, mkdirSync, mkdtempSync, openSync, readdirSync, readFileSync, readlinkSync, realpathSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";

function inside(root: string, path: string): boolean {
	const rel = relative(root, path);
	return rel !== ".." && !rel.startsWith(`..${sep}`) && !isAbsolute(rel);
}

function destination(root: string, path: string, label: string, published?: string, studio = root): { real: string; anchor: string } {
	const logicalRoot = resolve(root);
	root = realpathSync(root);
	let pending = relative(inside(logicalRoot, resolve(path)) ? logicalRoot : root, resolve(path)).split(sep);
	let real = root;
	let anchor: string | undefined;
	let links = 0;
	while (pending.length) {
		real = resolve(real, pending.shift()!);
		if (!inside(root, real)) throw new Error(label);
		const stat = lstatSync(real, { throwIfNoEntry: false });
		if (!stat?.isSymbolicLink()) continue;
		if (++links > 40) return { real: realpathSync.native(path), anchor: real };
		const target = resolve(dirname(real), readlinkSync(real));
		if (target === published && !pending.length) return { real: destination(studio, target, label).real, anchor: real };
		if (!inside(root, target)) throw new Error(label);
		if (target === published) anchor = real;
		pending = [...relative(root, target).split(sep), ...pending];
		real = root;
	}
	return { real, anchor: anchor ?? real };
}

export function notePath(studio: string, path: string, tool: string, key = tool): string {
	return destination(studio, resolve(studio, path), outside(tool, key)).real;
}

function outside(tool: string, key: string): string {
	if (tool === "ask_critic") return key === "to-critic"
		? "ask_critic: your note to the critic is outside the studio, so nothing was written"
		: "ask_critic: the critic visited, but its notes are outside the studio, so they weren't saved";
	return `${tool}: your ${tool === "note" ? "journal" : "notebook"} is outside the studio, so nothing was written`;
}

function sync(path: string): void {
	const fd = openSync(path, "r");
	try { fsyncSync(fd); } finally { closeSync(fd); }
}

export interface NoteFile {
	path: string;
	slot: string;
	text: string;
	root?: string;
	independent?: boolean;
}

export function publishNotes(studio: string, key: string, tool: string, files: NoteFile[], signal?: AbortSignal): void {
	const base = notePath(studio, `out/easel/.note-publications/${key}`, tool, key);
	const pointer = join(base, "current");
	notePath(studio, pointer, tool, key);
	const targets = files.map((file) => {
		const label = outside(tool, key);
		const target = destination(file.root ?? studio, file.path, label, join(pointer, file.slot), studio);
		const before = existsSync(target.real) ? readFileSync(target.real) : undefined;
		if (before !== undefined) accessSync(target.real, constants.W_OK);
		const mode = before === undefined ? undefined : lstatSync(target.real).mode & 0o777;
		return { ...file, ...target, before, mode };
	});
	mkdirSync(base, { recursive: true });
	let stage: string | undefined;
	const temporary: string[] = [];
	const prepared: string[] = [];
	const copies: { path: string; anchor: string }[] = [];
	const copyDirs: string[] = [];
	try {
		if (!existsSync(pointer)) {
			stage = mkdtempSync(join(base, "generation-"));
			for (const file of targets) if (file.before !== undefined) {
				writeFileSync(join(stage, file.slot), file.before, { mode: file.mode });
				chmodSync(join(stage, file.slot), file.mode!);
				sync(join(stage, file.slot));
			}
			sync(stage);
			symlinkSync(relative(base, stage), pointer);
			stage = undefined;
			sync(base);
		}
		const previous = realpathSync(pointer);
		const predecessor = join(previous, ".previous");
		const obsolete = lstatSync(predecessor, { throwIfNoEntry: false })?.isSymbolicLink()
			? resolve(previous, readlinkSync(predecessor)) : undefined;
		stage = mkdtempSync(join(base, "generation-"));
		symlinkSync(relative(stage, previous), join(stage, ".previous"));
		for (const slot of readdirSync(previous)) {
			if (slot === ".previous") continue;
			notePath(studio, join(previous, slot), tool, key);
			if (!targets.some((file) => file.slot === slot)) linkSync(join(previous, slot), join(stage, slot));
		}
		for (const file of targets) {
			writeFileSync(join(stage, file.slot), file.text, { mode: file.mode ?? 0o666 });
			if (file.mode !== undefined) chmodSync(join(stage, file.slot), file.mode);
			sync(join(stage, file.slot));
			if (file.independent) {
				const dir = mkdtempSync(join(dirname(file.anchor), ".note-copy-"));
				copyDirs.push(dir);
				const path = join(dir, "archive");
				writeFileSync(path, file.text, { mode: file.mode ?? 0o666 });
				if (file.mode !== undefined) chmodSync(path, file.mode);
				sync(path);
				sync(dir);
				copies.push({ path, anchor: file.anchor });
			}
		}
		sync(stage);
		for (const file of targets) {
			const published = join(pointer, file.slot);
			if (lstatSync(file.anchor, { throwIfNoEntry: false })?.isSymbolicLink() && resolve(dirname(file.anchor), readlinkSync(file.anchor)) === published) continue;
			const prior = join(previous, file.slot);
			if (file.real !== prior) {
				if (file.before === undefined) rmSync(prior, { force: true });
				else {
					const snapshot = join(stage, `.before-${file.slot}`);
					writeFileSync(snapshot, file.before, { mode: file.mode });
					chmodSync(snapshot, file.mode!);
					sync(snapshot);
					renameSync(snapshot, prior);
				}
				sync(previous);
			}
			mkdirSync(dirname(file.anchor), { recursive: true });
			const link = join(dirname(file.anchor), `.note-link-${key}-${stage.split(sep).at(-1)}-${file.slot}`);
			temporary.push(link);
			symlinkSync(relative(dirname(file.anchor), published), link);
			renameSync(link, file.anchor);
			if (file.before === undefined) prepared.push(file.anchor);
			sync(dirname(file.anchor));
		}
		const next = join(base, "next");
		temporary.push(next);
		rmSync(next, { force: true });
		symlinkSync(relative(base, stage), next);
		if (signal?.aborted) throw new Error("Operation aborted");
		renameSync(next, pointer);
		stage = undefined;
		prepared.length = 0;
		sync(base);
		for (const copy of copies) {
			renameSync(copy.path, copy.anchor);
			sync(dirname(copy.anchor));
		}
		if (obsolete && dirname(obsolete) === base && relative(base, obsolete).startsWith("generation-") && obsolete !== previous && obsolete !== realpathSync(pointer)) {
			rmSync(obsolete, { recursive: true, force: true });
			sync(base);
		}
	} finally {
		for (const path of [...temporary, ...prepared]) rmSync(path, { force: true });
		for (const dir of copyDirs) rmSync(dir, { recursive: true, force: true });
		if (stage) rmSync(stage, { recursive: true, force: true });
	}
}
