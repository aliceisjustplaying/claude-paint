import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { after } from "node:test";

/** A studio whose bin/easel records command names in `calls`, then runs `body`. */
export function stubStudio(body: string): string {
	const studio = mkdtempSync(join(tmpdir(), "studio-"));
	after(() => rmSync(studio, { recursive: true, force: true }));
	mkdirSync(join(studio, "bin"));
	writeFileSync(join(studio, "bin", "easel"), `#!/bin/sh\necho "$1" >> calls\n${body}\n`, { mode: 0o755 });
	return studio;
}
