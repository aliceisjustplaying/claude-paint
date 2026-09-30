# Painter harness

A pi setup for the claude-paint painters that gives them only what we give them:
our system prompt, the `bash` and `read` tools, and our own compaction. Nothing
from the machine's global pi setup (packages, `~/.pi/agent/extensions`,
`APPEND_SYSTEM.md`, `AGENTS.md`, skills, prompt templates) reaches the model.

| file | what it is |
|---|---|
| `painter.ts` | extension: drops pi's `APPEND_SYSTEM.md` addendum, context files, skills and guidelines from the system prompt, strips the leading `<!-- -->` comment of `system_prompt.md`, makes `bash`/`read` run one at a time, and keeps old images out of each request (`context-images.ts`) |
| `context-images.ts` | the image pruning `painter.ts` runs before each request (see "Images in the request"); `test/context-images.test.ts` tests it (`node --test harness/painter/test/context-images.test.ts`) |
| `pace.ts` | input tokens per minute: with `PAINTER_INPUT_TPM`, `painter.ts` holds each request until the last minute's input tokens plus its own fit that budget; and it makes a per-minute quota 429 (Google's `...PerMinute` quota ids) retryable for pi's retry, which skips errors that mention "quota exceeded" or "billing". `PAINTER_MAX_IMAGES` / `PAINTER_MAX_IMAGE_MB` set `context-images.ts`'s limits for a lane (round 18g's Gemini lane: 8 images, 1.5M tokens a minute under Google's 2M). `test/pace.test.ts` tests it |
| `compaction.ts` | extension: `session_before_compact` with a deterministic summary, no model call; also sets the compaction thresholds (see "Settings") |
| `summary.ts` | the summary `compaction.ts` returns; `test/summary.test.ts` tests it |
| `system_prompt.md` | the painter's system prompt (approved by Alice, 2026-09-27) |

## Launch

Run from the studio folder (pi's cwd is the studio). `H` is this folder,
`BLACK` is Alice's installed pi-black extension, `TEMP_GUARD` her persistent-temp
extension (sets TMPDIR under ~/tmp and blocks writes to OS temp folders, with the
reason shown to the painter; it adds no prompt text):

```sh
H=~/src/a/claude-paint/harness/painter
BLACK=~/.pi/agent/git/github.com/aliceisjustplaying/pi-black/extensions/pi-black.ts
TEMP_GUARD=~/.pi/agent/extensions/persistent-temp.ts

# Opus lane (anthropic, Claude subscription login)
pi --print --no-extensions -e "$H/painter.ts" -e "$H/compaction.ts" -e "$TEMP_GUARD" -e "$BLACK" \
   --system-prompt "$H/system_prompt.md" --tools bash,read \
   --no-context-files --no-skills --no-prompt-templates --no-approve \
   --provider anthropic --model claude-opus-5-5 --thinking high \
   "Your brief is in BRIEF.md in this folder. Your last message is your reply: the paths of the saved painting and its log, its title if you give it one and, if you like, a few sentences about the picture."

# Gemini lane (openrouter, API key): the same without -e "$BLACK"
pi --print --no-extensions -e "$H/painter.ts" -e "$H/compaction.ts" -e "$TEMP_GUARD" \
   --system-prompt "$H/system_prompt.md" --tools bash,read \
   --no-context-files --no-skills --no-prompt-templates --no-approve \
   --provider openrouter --model google/gemini-3.8-flash --thinking high \
   "<same message>"
```

The message is round 19's (`r19_chains.py`; rounds 16 to 18 sent "Complete
your task autonomously. Read BRIEF.md in this folder and follow it exactly.
That file is your whole brief. Your FINAL message is the reply it asks
for."). From a Python launcher, pass the same list with `stdin=DEVNULL`, as
round 16 did.

- `--no-extensions` disables every configured and discovered extension,
  including pi-anthropic-compat's native compaction and pi-codex-compaction;
  the explicit `-e` paths still load.
- `--no-approve` ignores the studio's `.pi/`, so nothing a painter writes
  there takes effect in a later sitting. The studio has no `.pi/` from us:
  `compaction.ts` sets the compaction thresholds. (Round 17 launched with
  `--approve` and a `.pi/settings.json`, which its painters read.)
- pi-black is loaded only for the Anthropic lane: see below.

## Why pi-black, and not pi-anthropic-compat

- **pi-anthropic-compat is compaction only.** It registers
  `session_before_compact` (native Anthropic compaction, the round 16
  summaries), `before_provider_request` (writes system/tools templates and
  replays signed summaries) and `context_with_system` (hides pi's readable
  summary once a signed one exists), plus the `/anthropic-settings` command.
  Its README: "It never replaces the `anthropic` provider." Opus works
  without it (checked below).
- **pi-black is what routes Anthropic OAuth requests through the Claude
  subscription.** `auth.json` holds an `oauth` credential for `anthropic`.
  Without pi-black, pi itself warns: "Anthropic subscription auth is active.
  Third-party harness usage draws from extra usage and is billed per token,
  not your Claude plan limits." pi-black wraps only the built-in Anthropic
  provider and transforms only OAuth requests. It adds no tools, commands or
  compaction. On the wire it prepends Claude Code's billing block and the
  system line "You are a Claude agent, built on Anthropic's Claude Agent
  SDK." (and a model-identity line for Fable 5.1, Opus 5 and Sonnet 5, not
  Opus 5.5). Round 16 painters had the same blocks.
- **OpenRouter needs nothing.** It's pi's built-in provider with an API key.
  None of the global extensions touch it.
- **pi-batch-order is replaced.** Round 16 painters had it globally: it
  serializes conflicting tool calls in one assistant message, for example two
  `bin/easel` commands in one message. `painter.ts` gets the same effect by
  registering pi's own `bash` and `read` with `executionMode: "sequential"`.

## Images in the request

Every look the painter reads is a PNG tool result of 50 to 750 KB of base64, and
pi sends all of them again with each request. A round 17 painter got
`413 request_too_large` after 74 looks (33.7 MB of base64) with its token count
far below the compaction point; pi recovered only by overflow compaction.

`painter.ts` has a `context` handler, which pi runs before each provider request
on a copy of the messages (`structuredClone` in pi's `emitContext`), so the
session file keeps every image for the studio viewer. It replaces the image of
older tool results with a line naming the file, `[an earlier look:
out/easel/painting/look-0031.png]`, and keeps the newest images: at most 20,
and at most 12 MB of base64. Old images go 5 at a time, so the request prefix
(and the prompt cache) changes once every 5 looks, not with every look.

Checked 2026-09-27 on a fake session holding the 74 looks of that session,
resumed with Haiku 4.5 and a probe extension logging `before_provider_request`:
without the handler the request was 33,797,791 characters with 74 images (413,
then overflow compaction); with it, one request of 9,158,275 characters with 19
images and 55 placeholders (look-0001 to look-0055), and the model named
exactly those ranges. The stored session entries were byte-identical afterward.

## Compaction

`compaction.ts` answers every `session_before_compact` (threshold, overflow
and manual) with this summary, built from the studio's files and the session
entries:

1. one neutral line: "Earlier parts of this session were condensed." (to
   round 18: "Earlier parts of this conversation were condensed. Look at
   the canvas to see where the painting stands.", an instruction)
2. `BRIEF.md`, verbatim
3. `notes/journal.md`, verbatim but quoted (every line starts with `> `,
   after a line saying these are the painter's own notes), so a heading in
   the journal can't pass for one of the summary's; only the last 120,000
   characters if it's longer, with a pointer to the file
4. the painting's globals as the easel's Lua state holds them (`easel
   globals`, through the same client and time limit as the tools): name, the
   chunk that last assigned it and what it holds; the 150 most recently
   assigned. If the easel can't answer, the summary says so
5. the canvas clock: chunks in the log, the last valid `day N, HH:MM` line
   a successful `paint` call printed (`wait()` returns it; other tools'
   results and failed chunks don't count), the last journal stamp

It has no next steps, remaining tasks or progress checklist. It keeps
`firstKeptEntryId` and `tokensBefore` from pi's preparation, so the recent
`keepRecentTokens` of conversation stay verbatim after the summary. It never
returns nothing, even when a file can't be read, so pi's built-in LLM
summarizer (with its "Next Steps" section) never runs. The entry's
`details.type` is `claude-paint-painter-compaction`, with the resolved
settings, chunk count, clock and number of globals.

### Settings

`compaction.ts` sets them, for every model, in `COMPACTION`:

```ts
export const COMPACTION = { enabled: true, reserveTokens: 100_000, keepRecentTokens: 20_000 } as const;
```

Compaction triggers when context exceeds window minus `reserveTokens`: about
900K tokens for both lanes (both have a 1M window). Round 16 had the same
point for Opus (Alice's global override) and about 1,021K for Gemini (global
`reserveTokens` 27200). `keepRecentTokens` 20000 is pi's default, which round
16 also used. `PAINTER_COMPACT_RESERVE` (tokens) sets `reserveTokens` for one
run: for a smaller window, or a test that makes compaction happen early (no
round 19 sitting came near 900K; the largest was Gemini's 328K).

Round 17's painters read the settings file in their studio (`cat
.pi/settings.json` was the first thing both did), so the values moved out of
the studio. Pi has no extension API for settings and no flag or environment
variable for a settings file. `PI_CODING_AGENT_DIR` moves the whole agent
directory, `auth.json` included, and pi locks `auth.json` for an OAuth refresh
at `<path>.lock` without resolving links, so a linked `auth.json` would get a
second lock and two pi processes could refresh the same token at once. So
`compaction.ts` overrides the four compaction getters of pi's `SettingsManager`
(the class `@earendil-works/pi-coding-agent` exports, the same module pi runs)
in the painter's process. Pi's own threshold check, cut point and overflow
recovery then use `COMPACTION`, whatever the global settings or a studio
`.pi/settings.json` say. If pi's `SettingsManager` ever lacks those getters,
it prints a line to stderr and pi's settings apply. Each compaction entry
records `details.settings` (what pi used) and `details.harnessSettings`
(whether that was `COMPACTION`). Everything else in the global settings still
applies (for example `retry`: 120 retries, 60 s apart).

Checked 2026-09-27 with Haiku 4.5 (200K window) on a fake session of about 31K
tokens whose last reply reported 130,010 input tokens, launched as above with
no `.pi/` in the studio: with the committed `compaction.ts` of before this
change (global settings: `reserveTokens` 27200) nothing compacted; with this
one pi compacted before the prompt, `"reason": "threshold"`, `"settings":
{"enabled": true, "reserveTokens": 100000, "keepRecentTokens": 20000}`,
`"harnessSettings": true`, and the reply's input fell from 31,190 to 21,296
tokens. Opus 5.5 with the Opus launch above (pi-black, `--no-approve`)
answered a one-line prompt (`stopReason: "stop"`, $0.0013).

## Verification (2026-09-27)

A fake studio in scratch (BRIEF.md, a three-entry journal, a three-chunk
`painting.lua`, a stub `bin/easel`). Haiku 4.5 ran over RPC with the command
above plus `--session-dir`, and a haiku-only `keepRecentTokens: 1` override
so a short session could compact. Then `{"type":"compact"}`. From the
session `.jsonl`:

- system message: `sections` = `preamble` (the text of `system_prompt.md`,
  then still carrying a DRAFT comment that painter.ts strips) and `cwd`; `toolsAdded` = `bash`, `read`
- compaction entry: `"fromHook": true`,
  `"details": {"type": "claude-paint-painter-compaction", "version": 1, "reason": "manual", "settings": {"enabled": true, "reserveTokens": 100000, "keepRecentTokens": 1}, "chunks": 3, "lastChunk": 3, "globals": 6, "clock": "day 2, 16:05", "journalStamp": "day 2, 16:05"}`
  (`reserveTokens` 100000 is the studio file's; the global value is 27200)
- after compaction, the model quoted the header line back

One-line prompts with the launch commands above: `anthropic/claude-opus-5-5`
answered (`stopReason: "stop"`, $0.0045) and `openrouter/google/gemini-3.8-flash`
answered ($0.0004).
