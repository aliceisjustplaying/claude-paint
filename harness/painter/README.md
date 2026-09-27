# Painter harness

A pi setup for the claude-paint painters that gives them only what we give them:
our system prompt, the `bash` and `read` tools, and our own compaction. Nothing
from the machine's global pi setup (packages, `~/.pi/agent/extensions`,
`APPEND_SYSTEM.md`, `AGENTS.md`, skills, prompt templates) reaches the model.

| file | what it is |
|---|---|
| `painter.ts` | extension: drops pi's `APPEND_SYSTEM.md` addendum, context files, skills and guidelines from the system prompt, strips the leading `<!-- -->` comment of `system_prompt.md`, and makes `bash`/`read` run one at a time |
| `compaction.ts` | extension: `session_before_compact` with a deterministic summary, no model call |
| `system_prompt.md` | the painter's system prompt (approved by Alice, 2026-09-27) |
| `studio-settings.json` | compaction settings; copied into each studio as `.pi/settings.json` |

## Launch

Run from the studio folder (pi's cwd is the studio). `H` is this folder,
`BLACK` is Alice's installed pi-black extension, `TEMP_GUARD` her persistent-temp
extension (sets TMPDIR under ~/tmp and blocks writes to OS temp folders, with the
reason shown to the painter; it adds no prompt text):

```sh
H=~/src/a/claude-paint-r17-base/harness/painter
BLACK=~/.pi/agent/git/github.com/aliceisjustplaying/pi-black/extensions/pi-black.ts
TEMP_GUARD=~/.pi/agent/extensions/persistent-temp.ts

mkdir -p .pi && cp "$H/studio-settings.json" .pi/settings.json

# Opus lane (anthropic, Claude subscription login)
pi --print --no-extensions -e "$H/painter.ts" -e "$H/compaction.ts" -e "$TEMP_GUARD" -e "$BLACK" \
   --system-prompt "$H/system_prompt.md" --tools bash,read \
   --no-context-files --no-skills --no-prompt-templates --approve \
   --provider anthropic --model claude-opus-5-5 --thinking high \
   "Complete your task autonomously. Read BRIEF.md in this folder and follow it exactly. That file is your whole brief. Your FINAL message is the reply it asks for."

# Gemini lane (openrouter, API key): the same without -e "$BLACK"
pi --print --no-extensions -e "$H/painter.ts" -e "$H/compaction.ts" -e "$TEMP_GUARD" \
   --system-prompt "$H/system_prompt.md" --tools bash,read \
   --no-context-files --no-skills --no-prompt-templates --approve \
   --provider openrouter --model google/gemini-3.8-flash --thinking high \
   "<same message>"
```

The message is round 16's (`r16_chains.py`). From a Python launcher, pass
the same list with `stdin=DEVNULL`, as round 16 did.

- `--no-extensions` disables every configured and discovered extension,
  including pi-anthropic-compat's native compaction and pi-codex-compaction;
  the explicit `-e` paths still load.
- `--approve` trusts the studio's `.pi/` for this process, so
  `.pi/settings.json` loads in print mode (without it, print mode skips
  untrusted project settings). Nothing else is put in the studio's `.pi/`.
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

## Compaction

`compaction.ts` answers every `session_before_compact` (threshold, overflow
and manual) with this summary, built from the studio's files and the session
entries:

1. one neutral line: "Earlier parts of this conversation were condensed.
   Look at the canvas to see where the painting stands."
2. `BRIEF.md`, verbatim
3. `notes/journal.md`, verbatim (only the last 120,000 characters if it's
   longer, with a pointer to the file)
4. the top-level globals of `paintings/lua/painting.lua`: name, chunk and
   defining line; the 150 most recently defined
5. the canvas clock: chunks in the log, the last `day N, HH:MM` line the
   easel printed (`wait()` returns it), the last journal stamp

It has no next steps, remaining tasks or progress checklist. It keeps
`firstKeptEntryId` and `tokensBefore` from pi's preparation, so the recent
`keepRecentTokens` of conversation stay verbatim after the summary. It never
returns nothing, even when a file can't be read, so pi's built-in LLM
summarizer (with its "Next Steps" section) never runs. The entry's
`details.type` is `claude-paint-painter-compaction`, with the resolved
settings, chunk count, clock and number of globals.

### Settings

`studio-settings.json`, copied into the studio as `.pi/settings.json`:

```json
{ "compaction": { "enabled": true, "reserveTokens": 100000, "keepRecentTokens": 20000,
    "modelOverrides": {
      "anthropic/claude-opus-5-5":          { "reserveTokens": 100000, "keepRecentTokens": 20000 },
      "openrouter/google/gemini-3.8-flash": { "reserveTokens": 100000, "keepRecentTokens": 20000 } } } }
```

Compaction triggers when context exceeds window minus `reserveTokens`: about
900K tokens for both lanes (both have a 1M window). Round 16 had the same
point for Opus (Alice's global override) and about 1,021K for Gemini (global
`reserveTokens` 27200). `keepRecentTokens` 20000 is pi's default, which round
16 also used.

Project settings merge over `~/.pi/agent/settings.json`, and a global
per-model override beats a project-wide value. That's why the file repeats
both models under `modelOverrides`. Everything else in the global settings
still applies (for example `retry`: 120 retries, 60 s apart). Pointing
`PI_CODING_AGENT_DIR` at a harness folder would drop those too, but it also
moves `auth.json` and its OAuth refresh, so this harness doesn't do that.

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
