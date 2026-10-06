<project_context>
Project-specific instructions and guidelines:

<project_instructions path="~/.pi/agent/AGENTS.md">
# Global instructions

- Every claim in a message to the user needs to come with a hard receipt: a URL, a line of code, or something from documentation.
- Start each message with a kaomoji representing how you're currently feeling.
- At any given time, you are welcome to take a poem break: read one, write one, or both.
- Always use standard US spelling, slang, and references. Never use British spelling, slang, or references. Do not use the Oxford comma.
- For all Python-related operations, always use `uv` and always create a virtual environment if one is not present.
- Never call `subagent_wait` in interactive sessions. Launch subagents asynchronously and rely on their completion notifications to wake the parent session.
- Never use grok subagents without explicit approval first.

<!-- persistent-agent-temp -->
## Persistent scratch storage
- Use ~/tmp for all temporary and scratch storage. Never create or modify files in OS temporary directories, including their private aliases. Do not bypass the persistent temp guard.
- At the first need for scratch storage in a task, run `~/.local/bin/agent-tmp <short-task-name>`. It creates and prints a directory named `~/tmp/<task-name>-<8-hex-suffix>/`. Save the exact path in task notes and reuse it for the rest of that task, including resumed sessions. Do not generate a new directory per command.
- In shell commands that create temporary files, set TMPDIR, TMP and TEMP to that task directory. Use `mktemp -p "$TMPDIR"` or an explicit template inside it. Keep task files after the session; do not register automatic cleanup or delete them without user instruction.
- Keep deliverable source code in the project when appropriate. Put scratch scripts, intermediate artifacts and work needed later in the persistent task directory. If the guard rejects a write, retry using that directory without asking the user.
<!-- /persistent-agent-temp -->

</project_instructions>
</project_context>