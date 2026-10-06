# Round 7 streams: common brief (claude-paint)

claude-paint is a physical oil-paint simulator in Rust; paintings are Lua
programs at an easel (README.md, crates/easel/README.md). Alice (the
project's owner; call her Alice) judges by eye; her yardstick: "less
digital, more bad painter". Her reviews: notes/round6/alice_review.md
(read the last section, on "Evening at a Mountain Lake"). The latest
painting and two blind critiques: notes/paint1/ (README.md, the plain
PNGs, critic_astra.md, critic_gemini.md, evening_lake.md). Its log is on
branch r7-paint-wet: `git show r7-paint-wet:paintings/lua/evening_lake.lua`.

Principles first: notes/principles.md on main (tools give physics and constraints, not answers; painters decide, the way a person would).

Rules (as before; notes/briefs/common.md has the full list): work only in
your worktree, commit often, don't push; scratch via
`~/.local/bin/agent-tmp <name>`, export TMPDIR=TMP=TEMP, never /tmp;
wrap long commands in `timeout`; view renders with `scripts/peek` (JPEG
q95, for you); anything for Alice is lossless PNG built from the renders
and committed (she views on another machine), plus plain images, not only
sheets. Tests: `cargo test --workspace` (~1 min) and, before you finish,
`cargo test --release -p easel --test hand_time`. Benchmarks
(notes/loops/l5_near.lua, l3_green.lua at 1000) byte-identical unless a
change means to alter them (then measure and say where; golden re-recorded
with a stated reason). Bugs: a test that fails first. US English, no
Oxford comma; the project is published under the pseudonym "alice": never
write the user's real name or an absolute home path into committed files.
Four streams run in parallel (edges, piles, cracks, friedrich research);
keep changes to shared files (handling.rs, api.rs, bristle.rs) small and
say so. Final message: a concise report (what changed, API, evidence
paths, tests, open issues).
