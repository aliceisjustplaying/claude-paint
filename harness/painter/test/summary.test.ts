import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { buildSummary } from "../summary.ts";

/** A studio with a log whose column-0 lines look like globals, and a bin/easel that runs `body`. */
function studio(body: string): string {
	const s = mkdtempSync(join(tmpdir(), "studio-"));
	mkdirSync(join(s, "bin"));
	mkdirSync(join(s, "paintings", "lua"), { recursive: true });
	writeFileSync(join(s, "paintings", "lua", "painting.lua"), "--@ chunk 1\nlocal dm = nil\ndm = 3\n\n--@ chunk 2\nx = 1\n");
	writeFileSync(join(s, "bin", "easel"), `#!/bin/sh\n[ "$1" = status ] && { echo '2 chunks'; exit 0; }\n${body}\n`, { mode: 0o755 });
	return s;
}

const section = (summary: string) => summary.split("## Your globals\n\n")[1].split("\n\n## ")[0];

test("the summary lists the globals the easel reports, not names read off the log", async () => {
	const s = studio(`[ "$1" = globals ] && printf '1\\tOB\\ttable with 2 entries\\n2\\ttree\\tfunction (chunk 2, line 9)\\n'`);
	const { summary, facts } = await buildSummary(s, []);
	assert.equal(section(summary), "- `OB` (chunk 1): table with 2 entries\n- `tree` (chunk 2): function (chunk 2, line 9)");
	assert.equal(facts.globals, 2);
	assert.match(summary, /Chunks in the log: 2 \(last: chunk 2\)/);
});

test("when the easel can't answer, the summary says so and guesses nothing", async () => {
	const s = studio(`echo 'easel: no command "globals"' >&2; exit 1`);
	const { summary } = await buildSummary(s, []);
	assert.equal(section(summary), `(the easel couldn't list them: easel: no command "globals")`);
});
