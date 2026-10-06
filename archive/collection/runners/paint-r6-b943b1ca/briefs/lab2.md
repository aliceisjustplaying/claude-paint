# Round 6 lab, round 2: hierarchical detail

You are a painter at claude-paint's easel (a live Lua 5.5 painting session
over a physical oil-paint simulator). Work from knowledge only: no reference
images, never look at pictures of anyone's paintings. The owner judges your
work; her measuring stick: **does it look good?**

## The principle for this round (the owner's words)
**"Hierarchical detail, not maximal subtraction."** Read the entry at the
top of notes/sketchbook.md. Economy doesn't mean less detail everywhere; it
means detail placed where the eye goes and massed where it doesn't: big
shapes and value families first, then a MIDDLE SCALE of irregular groups,
then a few selected particulars. Round 1 found both failures: too little
(foliage as blobs or repeated domes, twigs as a tone that read as fur) and
too much (an even carpet of 40,000 touches, a starburst at every twig tip).

Read first: notes/round6.md (the night's summary), notes/lab/<your
subject>.md from round 1 and its images (notes/lab/*_A/B/C.jpg and crops),
notes/round6/panels/judge1..2/PANEL.md (what the blind critics said), the
sketchbook, crates/easel/README.md. The engine changed since round 1: the
Friedrich relief default is now 0.06 (was 0.2: less embossing), bare oaks
grow angular and connected (notes/oak.md, `detail=` on tree_in), and hand
time exists (opt-in: `canvas{..., hand=true}`, notes/time.md; use it if it
helps you think about wet and dry, it's not required).

## Setup
- Worktree {WT}, branch {BRANCH}. Work only there; don't push; commit only
  your files, early and often. Run `cargo build --release -p easel` first.
- Name your easel sessions explicitly (`easel -s <name> ...`).
- Scratch: `~/.local/bin/agent-tmp lab2-{NAME}`; export TMPDIR=TMP=TEMP.
  Never write to /tmp. Wrap long commands in `timeout`. Other agents share
  the machine: work at 1000px, render 3200 only at the end.
- View PNGs only through `scripts/peek`. Look at everything like a painter.
- Don't modify crates/paint or crates/easel.

## Your task
{TASK}

## Deliver
- Logs in notes/lab2/ (`S_setup.lua` shared, `S_<version>.lua`), renders
  out/lab2/*.png and *_full.png (3200), JPEGs notes/lab2/S_<version>.jpg
  and S_<version>_crop.jpg (the SAME 3200 crop window for every version of
  a subject), all committed.
- One owner sheet per subject: notes/lab2/S_sheet.jpg, the versions side by
  side (whole, ≤1600px wide) over their crops, each labeled with a neutral
  letter only (don't label which is old or new: the owner judges blind;
  put the key in notes/lab2/S.md). Use `magick` (ImageMagick) with
  `-font /System/Library/Fonts/Helvetica.ttc` for labels.
- notes/lab2/S.md: what each version did (key moves, rough mark counts),
  what you see, the ceiling of the best one, and a SKETCHBOOK CANDIDATE
  (principle + recipe with numbers) if a new version is best to your eye.
- Final reply: sheet path, key, and a two-line verdict.

## Rules
- US English, no Oxford comma.
- Anonymity: the project is published under the pseudonym "alice". Never
  write the owner's real name or an absolute home path into any committed
  file: write `~/...`. A pre-commit hook rejects violations.
