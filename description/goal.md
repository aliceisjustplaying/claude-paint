# Drafting contract

## Source and reading order

Source: the repository, commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. Read README, glossary, the looking pilot and foundations before drafting. Then read `notes/easel_guide.md`, the relevant command in `crates/easel/src/main.rs`, `session.rs`, the relevant operation implementation and its behavior tests. For viewing, read `studio/index.html`, `studio/studio.py` and `studio/export_static.py`. For the harness, read its current TypeScript rather than assuming its README is current.

The source repository is read-only. Documentation belongs here. Existing untracked `notes/rag/dark-review/` belongs to the user. Runtime probes use an isolated EASEL_ROOT outside the source. No publishing or production service changes are part of this task.

## Writing rules

Describe the experience, not implementation mechanics. Technical detail belongs only in a `> Technical note:` when it changes the reader's expectations. Use sentence case, direct concrete prose, US spelling and no Oxford comma. Define terms in the glossary. Link to the owner of shared facts rather than restating thresholds. Every behavior needs a source receipt, with a source path and line number in a technical note, or recorded runtime evidence. Mark uncertainty explicitly: source-supported is not manually verified. Every feature footer names the source and commit, with the verification limit.

Use all eight sections and all five phases from README. Interrupt rows, exactly: Explicit abort; Another action; Environment failure; Target changed externally; Input channel changed. Cross-cutting paragraphs, exactly in order: Access; History; Containers; Restricted state; Offline; Collaboration; Notifications; Preferences.

Modifiers describe options at submission versus during computation; there is no interactive mid-chunk option editing. Never claim Ctrl-C rolls back a running server command. A disconnected client and a stopped server are different events. Unknown behavior is an open question, never a guess.

Commit as `docs: add <path>` or `docs: revise <path>`, using the source's alice attribution. Do not change source or tests.

## Order of work

Scaffold; looking pilot; commands, canvas and sessions foundations; passages as the hardest painting area; remaining feature documents; consistency and checklists; isolated runtime checks; triage and review.

## Established facts

- A command is not a mouse gesture. The terminal or harness submits a whole Lua chunk and waits for a result.
- Successful chunks remain and are logged; failed chunks restore state, with a replay rebuild when exact restoration requires it. Persistence errors after execution are distinct from Lua errors. Owner: commands.
- No undo exists. Recovery by painting or wiping is a new chunk. Owner: commands.
- Logical width is 1000 units; live raster width is 2400 pixels. The canvas owns coordinates and setup bounds.
- Painter build has one painting; default build includes named sessions, replay and finishing. Owner: sessions.
- Viewing does not advance painting time or alter canvas paint. Owner: looking.
- Real time between chunks does not dry paint. Owner: time.

## State ownership

Commands owns transaction, client disconnect and persistence failure. Canvas owns setup and coordinates. Sessions owns process lifecycle and selection. Passages owns mask overrun, clipping and area handling. Brushes owns held brush state. Rags owns cloth state. Shapes owns mask construction. Space owns depth restrictions. Time owns aging. Looking owns rendered views. Delivery owns exported files and finishing. Harness owns model-request context and tool presentation. Studio owns browser state; it never changes a painting.

- Live look size defaults to 1000 and clamps to 1–1600 pixels on the long side; it never enlarges the source. Crop limit is 1200 pixels per side (500 logical units live). Looking owns these numbers.
- Painting time starts at day 1, 09:00. Time owns the wait limit, slice cadence and palette-trip timing. Ground preparation is outside the painting clock.
- Runtime probe at aspect 5 printed H as 199.99998474121094: logical dimensions use floating-point arithmetic. The 1000/aspect model is mathematical, not a guarantee of exact decimal output.
