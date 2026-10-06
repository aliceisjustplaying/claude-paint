# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow"]
# ///
"""Round 24: round 23's lane INNS on engine 3 (one Claude Opus 5.5 painter, thinking high, pi-black), on the code
of one tag on the final engine-3 commit (TAG, pending until freeze), checked out detached at claude-paint-r24run
(BASE and H); main() starts only from a clean checkout of TAG. See CHANGES.md.

Round 23: round 22.1's lane INNS again (one Claude Opus 5.5 painter, thinking xhigh, pi-black, in Inness's
studio, MAX_SITTINGS 4, the same brief), on the code of the round-23 tag: the painter's look tool has
palette: true (each pile a global holds, thick, thin and very thin over the ground, thin over a black and white
card; notes/easel_guide.md "Looking"), its only painter-facing change. The session writes a save file at close
(out/easel/painting/live.ckpt), so finishing (scripts/finish_painting) takes seconds instead of a replay, and
the check (scripts/check_painting) replays once and compares the save. BASE and H are the round-23 checkout.

Round 22.1: one Claude Opus 5.5 painter (thinking xhigh, pi-black) in George Inness's studio (lane
INNS), the first painter in it: his box, his materials note (as edited on main on 2026-10-02, which the
round-22 tag doesn't have; hence BRANCH round-22.1) and the plain inness brief (OPENING, READING, as in
round 21.5). No reader, no reference pictures (they are Tonn's: inness isn't in ARTIST, so the lane
gets no reference/ folder and doesn't need REFERENCE). Round 22's lanes (TONN, BUNT) run from round
22's runner; their code paths are kept here unchanged, and LANES is INNS alone.

Round 22: one Claude Fable 5.1 painter (thinking xhigh) in Kendric Tonn's studio (lane TONN), the
first studio with pictures of the artist's paintings. They are there with his permission: the owner
puts them (jpg, jpeg, png or webp, and if she likes a README.md describing them) in REFERENCE
(~/src/a/tonn-reference, outside the repo), and the studio gets a copy in reference/ (a picture
larger than pi's read tool shows is copied resized to fit; her files stay as they are). Without a
picture there the runner stops before anything starts. The painter looks at them with its read tool,
as it reads its notes. The brief's opening says one picture or several (OPENING, OPENING_ONE). No
reader. Everything below is as in round 21.5's runner unless it says otherwise.

A warmup before it (lane BUNT): one Space Bunny painter (thinking max, opencode-go, as round 21's FRDC
chain) in Tonn's studio without the pictures (lane(..., reference=False)): no reference/ folder, and
the brief is round 21.5's for this studio, from knowledge and the notes. Only a lane that gets pictures
needs REFERENCE. The two lanes can run as two processes from this folder, started at different times
(--only BUNT, later --only TONN): each process watches only its own lanes' studios, and each needs its
own output file (run/runner.BUNT.out, run/runner.TONN.out; `>` empties the file it names).

Round 19: the Friedrich chain (lane F: three painters, one after another) and, if switched
on in LANES, blank-studio single painters, all run in parallel.

Lanes (LANES): F, a chain of PAINTERS=3 Opus 5.5 (thinking high) painters in Friedrich
studios, each after the one before, with the reader's records of the earlier painters merged
into its studio notes (nothing in them says there were earlier painters). The blank lanes
(one painter each, blank studio, "a picture of your choosing") are prefilled in LANES and
commented out: uncomment the ones to run. Lane names never reach a painter: studios get
neutral names (paint-studio-<hash>), and the mapping stays in run/studios.json.

Round 19's changes to what painters see (see CHANGES.md): no `check` in the painter's
easel, brief, guide or notes; no "in one session", no `timeout` rule and no water in the
brief; a plain launch and sitting message; subject-neutral guide examples; research notes
without the developer framing, varnish, craquelure or snow; a neutral compaction header; no
settings file in the studio.

Sittings: the painter works in sittings, each a new pi session (same harness and studio),
until it decides it's done. Sitting 1 gets PAINTER_MSG, later ones SITTING_MESSAGE. After each
sitting the chunks of the painting's log that put marks on the canvas are counted
(painting_chunks.py). The painter finishes after a sitting it ended itself that added no such
chunks and delivered whole-canvas and detail looks. MAX_SITTINGS completed sittings is only
a safety cap (logged as NOT FINISHED).
A sitting that crashes (pi exits non-zero, or its session ends on a provider error) is no
judgment and counts for nothing: the runner waits (CRASH_WAITS, or longer if the error says
"retry in Ns") and starts another sitting; after MAX_CRASHES crashes the painter stops (NOT
FINISHED). A usage limit (OpenCode Go's 5-hour or weekly window) is no crash. The painter's pi
waits it out itself (harness/painter/limits.ts) and goes on with nothing added to the
conversation; only a limit that holds for a day ends the session. Then the sitting is
marked 'limited', the runner probes the provider until it answers (LIMIT_PROBE_S, up to
LIMIT_GIVE_UP_H hours; a probe error waiting won't fix, like a refused key or no credit, stops
the painter at once), and then the painter carries on in the same session (CONTINUE_MESSAGE,
same sitting number) if it had worked in it, or starts a fresh sitting if not. Each attempt is
recorded in run/<lane>/p<n>_sittings.json (a continued sitting's parts share its number).

After a painter's last sitting, in the background (the next painter doesn't wait):
  - check: scripts/check_painting (replay build) reopens a copy of the log, runs `check`
    and compares the replay with the painter's last save; the result goes to the log
    (run/<lane>/p<n>_check.log, workdir run/<lane>/p<n>_check/);
  - finishing: varnish and cracks on a replay of the log (run/<lane><n>_finished.png; the
    painter's own save is untouched).
Then, in a chain, a reader writes the record for the next painter, launched as isolated as the
painter (reader.ts, reader_system_prompt.md; read and write only, checked by reader.ts against
READER_SCOPE: the logs, the journal and its brief to read, the record to write). Lines that look
like what to do or where things go in the picture (record_flags) only warn: they are logged and
listed in p<n>_record.flags.md, and the record still goes on.

That is a lane's default record (record_kind="free-text", as imported). A lane with
record_kind="structured" gets structured records instead: the reader writes observations as JSON
(reader_brief_structured.md, p<n>_observations.json), record_schema.py checks each against the
session logs it cites, the box and the words and names a record may not hold (an observation that
fails is dropped and logged; a record with none, or none left, goes on as an empty record; one
not of the schema, or a reader that failed, stops the lane), the
runner keeps it with its metadata (round 21, the commit BRANCH names) as p<n>_record.json, and
record_render.py writes the next studio's notes from the records through fixed templates, leaving
out what doesn't fit that studio (p<n>_inherited.json). Such a lane is labeled chain-inherited,
non-neutral in run/<lane>/condition.json; the painter isn't told the notes are inherited.
record_kind="none" runs no reader. A lane keeps one kind of record: a run folder holding the other
kind is refused. See CHANGES.md.

Painters run in the clean harness (claude-paint-r19-base/harness/painter): no global
extensions, our system prompt, bash and read only, our compaction (its thresholds set by
compaction.ts), --no-approve and no .pi/ in the studio. pi-black for Anthropic lanes only.
Per-lane environment (model(..., env=)) reaches the painter's pi: PAINTER_MAX_IMAGES,
PAINTER_MAX_IMAGE_MB, PAINTER_INPUT_TPM (harness/painter/README.md).

Resumable: every finished step leaves a marker in the run folder; a rerun skips it (and
starts any check or finishing that didn't finish). A watchdog stops any process working in
one of this round's studios (its cwd there, or the studio in its command line) that has run
longer than WATCHDOG_MIN minutes, except the painter's pi, the easel server and the runner's
own check and finishing; it logs each stop. It also watches that each painting's log only
grows.

    uv run r21_chains.py                  # the lanes in LANES
    uv run r21_chains.py --only TONN      # some of them
    uv run r21_chains.py --dry            # print the exact commands, run nothing
    uv run r21_chains.py --briefs DIR     # render every profile's brief into DIR, run nothing
"""
import argparse
import hashlib
import json
import os
import re
import shlex
import shutil
import subprocess
import tempfile
import threading
import time
from pathlib import Path

from painting_chunks import count_painting_chunks
from defaults_used import defaults_used
from log_copy import copy_logs
from copied_shapes import copied_shapes

# a studio's box comes from its bin/box and a painting's log; an inherited EASEL_BOX would conflict with both
# (review 2026-09-29): the runner and everything it starts run without it
os.environ.pop("EASEL_BOX", None)

HOME = Path.home()
A = HOME / "src/a"
# round 21: the code of the round-21 tag, checked out (detached, no branch) at claude-paint-r21
# round 21.1: round 21 with the painter's prompts tweaked. The export, check and finishing scripts still run
# from claude-paint-r21 (its target/studio-build holds round 21's easel build; round-21.1's code trees are
# round-21's, so the export reuses it: nothing recompiles); they git-archive BRANCH from the shared repo.
# Only the painter's harness (H, with its system prompt) comes from the round-21.1 checkout.
# round 22: the runner (Tonn's studio, his pictures in reference/) and the notes change; the harness, the
# Rust code and the export scripts are round 21.5's, so H stays the round-21.5 checkout and BASE round 21's.
# The export builds the tonn box's easel in BASE's target/studio-build.
BASE = A / "claude-paint-r21"
# round 23: the code, harness, export and finishing scripts of the round-23 tag (the palette look, the save
# file), checked out at claude-paint-r23; the export builds the inness easel in its target/studio-build
BASE = A / "claude-paint-r23"
# round 22.1: the notes at round-22.1 (inness_materials.md as edited on 2026-10-02); code, harness and export
# scripts still round 21.5's (the inness easel is built in BASE's target/studio-build)
BRANCH = "round-22.1"
BRANCH = "round-23"
# round 24 (engine 3): the code, harness, guide, export, check and finishing scripts of one tag on the final
# engine-3 commit, checked out detached at claude-paint-r24run; the export builds the inness easel in its
# target/studio-build, check and finishing build the replay easel in its target.
# Round 24.1 keeps the engine and adds completion review and guide corrections. Before launch:
# `git tag -a <TAG> <commit>` in the shared repo and `git worktree add --detach ~/src/a/claude-paint-r24run <TAG>`.
# main() refuses to start while BASE isn't a clean checkout of TAG (checkout_problems).
TAG = "round-24.2"
BRANCH = TAG
BASE = A / "claude-paint-r24-2run"
EXPORT = BASE / "scripts/export_r16_studio"      # honors R16_BRANCH
FINISH = BASE / "scripts/finish_painting"
CHECK = BASE / "scripts/check_painting"
H = A / "claude-paint-r21.5" / "harness/painter"
H = A / "claude-paint-r23" / "harness/painter"     # round 23: the look tool's palette
H = BASE / "harness/painter"                      # round 24: the tag's harness
BLACK = HOME / ".pi/agent/git/github.com/aliceisjustplaying/pi-black/extensions/pi-black.ts"
HERE = Path(__file__).resolve().parent
RUN = HERE / "run"
SESS = HOME / ".pi/agent/sessions"
WATCHDOG_MIN = 30

# the painter's tools (harness/painter/easel-tools.ts): the easel and reading the studio; no shell
TOOLS = "paint,look,note,status,log,read"
HARNESS = ["--no-extensions", "-e", str(H / "painter.ts"), "-e", str(H / "compaction.ts"),
           "--system-prompt", str(H / "system_prompt.md"), "--tools", TOOLS,
           "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]
PAINTER_MSG = ("Your brief is in BRIEF.md in this folder. Your last message is your reply: the "
               "painting's title if you give it one and a few sentences about the picture.")
SITTING_MESSAGE = ("You're back at the easel. The painting is as you left it. "
                   "Your brief is in BRIEF.md and your journal in notes/journal.md. "
                   "Take your time with a fresh look at the whole canvas and detail crops in normal color. "
                   "Is your heart happy with this? Is there something you would enjoy taking further? "
                   "You can keep working for hours if you like, and the painting's simulated time is "
                   "yours to use, whether that means days or years. Enjoy it. When you are happy with "
                   "the painting, record your reflections in your journal and finish.")
# A painter works in up to MAX_SITTINGS sittings (completed ones, and crashed ones that painted); it stops
# earlier after a sitting that reviews whole and detail views without adding paint
MAX_SITTINGS = 4                                 # default; run/max_sittings.txt is read between sittings
MAX_CRASHES = 6                                   # crashed sittings (in all) before a painter is stopped
# only after a usage limit that held for a day (painter.ts waits out shorter ones with nothing added)
CONTINUE_MESSAGE = "Carry on where you left off."
CRASH_WAITS = [90, 180, 300, 600, 900, 1200]      # s before the sitting after the 1st, 2nd, ... crash
# A provider's usage limit (OpenCode Go: 5-hour, weekly and monthly windows) is no crash: the
# painter waits it out. The runner asks the provider every LIMIT_PROBE_S (or at the reset time
# the error names) with a one-line prompt outside the studio, and starts the next sitting once
# it answers; after LIMIT_GIVE_UP_H hours it stops the painter (rerun to go on).
LIMIT_PROBE_S = 30 * 60
LIMIT_GIVE_UP_H = 24

READING = {
    p: (f"notes/easel_guide.md; notes/studio_notes.md;\n"
        f"notes/research/{f}_materials.md (his materials and method)"
        + " and\nnotes/research/oil_paint_physics.md as needed.")
    for p, f in [("sargent", "sargent"), ("inness", "inness"), ("alma-tadema", "alma_tadema"), ("tonn", "tonn"), ("hopper", "hopper")]
}
# round 22: pictures of his paintings in reference/
READING["tonn"] = ("notes/easel_guide.md; notes/studio_notes.md;\n"
                   "notes/research/tonn_materials.md (his materials and method);\n"
                   "reference/ (his paintings; reference/README.md lists them) and\n"
                   "notes/research/oil_paint_physics.md as needed.")
# round 21's studio (round 19's reading list for it)
READING["friedrich"] = ("notes/easel_guide.md; notes/studio_notes.md;\n"
                        "notes/research/friedrich_materials.md (his materials and method, sourced);\n"
                        "notes/research/trees.md (how trees are built) and\n"
                        "notes/research/oil_paint_physics.md as needed.")
EXPORT_AS = {}
OPENING = {
    "sargent": (
        "Compose and paint one original portrait in the manner of John Singer\n"
        "Sargent, at the easel, a simulator of oil paint on linen. The sitter,\n"
        "setting and composition are yours to invent. Work from knowledge and the\n"
        "notes in your studio; don't use reference images, image models or pictures\n"
        "of his work."),
    "inness": (
        "Compose and paint one original landscape in the manner of George Inness,\n"
        "at the easel, a simulator of oil paint on linen. The place, subject and\n"
        "composition are yours to invent. Work from knowledge and the notes in your\n"
        "studio; don't use reference images, image models or pictures of his work."),
    "alma-tadema": (
        "Compose and paint one original scene from Roman antiquity in the manner of\n"
        "Lawrence Alma-Tadema, at the easel, a simulator of oil paint on linen. The\n"
        "subject, setting and composition are yours to invent. Work from knowledge\n"
        "and the notes in your studio; don't use reference images, image models or\n"
        "pictures of his work."),
    "hopper": (
        "Compose and paint one original scene of American city life in the manner\n"
        "of Edward Hopper, at the easel, a simulator of oil paint on linen. The\n"
        "subject, setting and composition are yours to invent. Work from knowledge\n"
        "and the notes in your studio; don't use reference images, image models or\n"
        "pictures of his work."),
    "friedrich": (
        "Compose and paint one original landscape in the manner of Caspar David\n"
        "Friedrich, at the easel, a simulator of oil paint on linen. The place,\n"
        "subject and composition are yours to invent. Work from knowledge and the\n"
        "notes in your studio; don't use reference images, image models or pictures\n"
        "of his work."),
    # round 22: with pictures of his paintings (reference/); OPENING_ONE when there is one
    "tonn": (
        "Compose and paint one original picture in the manner of Kendric Tonn, at\n"
        "the easel, a simulator of oil paint on linen. Pictures of his paintings are\n"
        "in reference/, there with his permission: study them for his manner. The\n"
        "subject and composition are yours to invent; your picture isn't a copy or a\n"
        "version of any of his. Work from those pictures, from knowledge and from the\n"
        "notes in your studio; don't use other reference images or image models."),
}
# a studio whose reference/ holds exactly one picture
OPENING_ONE = {
    "tonn": (
        "Compose and paint one original picture in the manner of Kendric Tonn, at\n"
        "the easel, a simulator of oil paint on linen. A picture of one of his\n"
        "paintings is in reference/, there with his permission: study it for his\n"
        "manner. The subject and composition are yours to invent; your picture isn't\n"
        "a copy or a version of his. Work from that picture, from knowledge and from\n"
        "the notes in your studio; don't use other reference images or image models."),
}
# a studio of the same profile without the pictures (lane(..., reference=False)): round 21.5's opening and
# reading list for it, word for word
OPENING_PLAIN = {
    "tonn": (
        "Compose and paint one original picture in the manner of Kendric Tonn, at\n"
        "the easel, a simulator of oil paint on linen. The subject and composition\n"
        "are yours to invent. Work from knowledge and the notes in your studio;\n"
        "don't use reference images, image models or pictures of his work."),
}
READING_PLAIN = {
    "tonn": ("notes/easel_guide.md; notes/studio_notes.md;\n"
             "notes/research/tonn_materials.md (his materials and method) and\n"
             "notes/research/oil_paint_physics.md as needed."),
}

# Round 22: pictures of the artist's paintings, there with his permission. The owner puts them in REFERENCE
# (outside the repo); a studio of a profile in ARTIST gets a copy in <studio>/reference/ (copy_reference).
REFERENCE = A / "tonn-reference"
ARTIST = {"tonn": "Kendric Tonn"}
IMAGE_SUFFIXES = (".jpg", ".jpeg", ".png", ".webp")
# pi's read tool shows an image as it is if it fits 2000x2000 px and 4.5 MB of base64; a larger one it
# resizes at every read and says so to the model ("[Image: original WxH, displayed at ...]"). The limits are
# claude-fable-5-1's inputLimits.images.resize in pi-ai's providers/data/anthropic.json (pi's defaults too,
# utils/image-resize-core.js). A picture larger than that is copied resized to fit, in its own format.
READ_MAX_SIDE = 2000
READ_MAX_BASE64 = 4718592


def api_key(entry):
    """A provider key stored in pi's auth.json (read at run time; never printed or written)."""
    a = json.loads((HOME / ".pi/agent/auth.json").read_text())
    e = a.get(entry) or {}
    return e.get("key") or e.get("apiKey") or ""


def model(provider, name, thinking, black=False, env=None, key_from=None):
    """black: load pi-black (Anthropic OAuth through the Claude subscription). env: extra environment
    for the painter's pi (harness/painter's PAINTER_MAX_IMAGES, PAINTER_MAX_IMAGE_MB, PAINTER_INPUT_TPM)."""
    return dict(args=["--provider", provider, "--model", name, "--thinking", thinking], name=name,
                black=black, env=env or {}, key_from=key_from)


RECORD_KINDS = ("free-text", "structured", "none")


def lane(profile, m, painters=1, records=(), record_kind="free-text", read_last=False, reference=True):
    """profile: the studio (friedrich or blank). painters > 1: a chain, with a reader between painters.
    records: reader records from another lane's painters (paths), merged into this lane's first painter's
    studio notes as a chain's are: this painter continues that chain.
    record_kind: what the reader writes for the next painter: free-text (p<n>_record.md, the default,
    round 21's record as it was), structured (p<n>_observations.json, validated by record_schema into
    p<n>_record.json and rendered into the next studio's notes by record_render) or none (no reader;
    the next studio gets the plain notes and the records of another lane, if any). A structured lane
    can't continue another lane's free-text records.
    read_last: the reader also reads the lane's last painter (its record is kept; no painter follows),
    so a single painter gets a record too. A rerun reads a last painter that is done but unread.
    reference: False for a studio of a profile in ARTIST without the artist's pictures: no reference/
    folder, the profile's plain brief (OPENING_PLAIN, READING_PLAIN), and REFERENCE isn't needed."""
    if record_kind not in RECORD_KINDS:
        raise ValueError(f"record_kind={record_kind!r}: want one of {', '.join(RECORD_KINDS)}")
    if record_kind == "structured" and records:
        raise ValueError("a structured lane can't continue another lane's free-text records (records=)")
    return dict(profile=profile, model=m, painters=painters, records=[Path(r) for r in records],
                record_kind=record_kind, read_last=read_last, reference=reference)


def referenced(t):
    """Whether lane t's studios get the artist's pictures in reference/."""
    return t["profile"] in ARTIST and t.get("reference", True)


def reads(t, n):
    """Whether the reader reads painter n of lane t: between painters, and after the last with read_last."""
    return t["record_kind"] != "none" and (n < t["painters"] or (t.get("read_last", False) and n == t["painters"]))


def unread(t, rd, n):
    """A painter the reader should have read whose record isn't there (a run that ended before read_last)."""
    rec = rd / (f"p{n}_record.json" if t["record_kind"] == "structured" else f"p{n}_record.md")
    return reads(t, n) and not rec.exists()


OPUS = model("anthropic", "claude-opus-5-5", "high", black=True)
# the reader: GPT-6.1 Sol through the ChatGPT subscription. Round 19's Anthropic reader was refused
# ("reverse engineering or duplicating model outputs"); Sol and GPT-6 Astra both read round 19's
# SONF logs without a refusal (2026-10-01), and Sol's record was the better of the two.
READER = ["--provider", "openai-codex", "--model", "gpt-6.1-sol", "--thinking", "medium"]
READER_BLACK = READER[READER.index("--provider") + 1] == "anthropic"     # pi-black only for an Anthropic reader
# the reader's launch, as isolated as the painter's (HARNESS)
READER_HARNESS = ["--no-extensions", "-e", str(HERE / "reader.ts"), *(["-e", str(BLACK)] if READER_BLACK else []),
                  "--system-prompt", str(HERE / "reader_system_prompt.md"), "--tools", "read,write",
                  "--no-context-files", "--no-skills", "--no-prompt-templates", "--no-approve"]

# GPT-6 Luna through the ChatGPT subscription (dev runs), thinking max
LUNA = model("openai-codex", "gpt-6-luna", "max")
GEM2 = model("google", "gemini-3.8-flash", "high", env={"PAINTER_MAX_IMAGES": "8", "PAINTER_INPUT_TPM": "900000"})

# Round 20: four single painters in the manner of an artist, each studio with that artist's tube box
# (r20-base: bin/box, the guide's table from the studio's easel) and materials note. No chains (the
# reader was blocked on 2026-09-29). DeepSeek V4.1 Flash, thinking high, through OpenCode Zen (billed per
# token with the opencode-go key; the Go allowance was used up), as MUSEF ran in round 19.
DSF = model("opencode", "deepseek-v4.1-flash", "high", key_from="opencode-go")
# Round 21: one chain. Three painters in Friedrich's studio (the default box, his materials note and
# trees.md), each studio after
# the first inheriting the structured records of the painters before it (record_render.py). The round's
# other lanes (SARG, INNS, ALMA, TONN on DSF and the NEW slot) were dropped before launch (2026-10-01).
BUNNY = model("opencode-go", "space-bunny-free", "max")
# Round 21.5: one Sonnet 5.5 painter in Friedrich's studio (round 19's SONF), as round 21.4, with trees.md
# without its old-oak lead (no "In short" point 5, no stag-headed, species in alphabetical order)
# A single painter: no records, so round_number() (which wants round-<n>) isn't reached.
# Round 22: one Claude Fable 5.1 painter (thinking xhigh, through the Claude subscription: pi-black) in Tonn's
# studio, with pictures of his paintings in reference/. No reader.
# The warmup before it: one Space Bunny painter (thinking max, through OpenCode Go with its own key in pi's
# auth.json, as round 21's FRDC chain) in Tonn's studio without the pictures. No reader.
LANES = {
    # round 22.1 and round 23: one Opus 5.5 painter (thinking xhigh, through the Claude subscription: pi-black) in Inness's studio
    # round 24: thinking high (the owner's choice for the engine-3 painting)
    "INNS": lane("inness", OPUS, record_kind="none"),
}
DRY = False

lock = threading.Lock()
background = []


def log(msg):
    line = f"{time.strftime('%F %T')} {msg}"
    with lock:
        print(line, flush=True)
        if not DRY:
            with open(RUN / "chains.log", "a") as f:
                f.write(line + "\n")


def studio(key):
    """A neutral folder name (nothing about rounds, lanes, models or order); the mapping stays in the run folder."""
    with lock:
        m = RUN / "studios.json"
        names = json.loads(m.read_text()) if m.exists() else {}
        if key not in names:
            names[key] = "paint-studio-" + hashlib.sha1(f"r19{key}{time.time()}".encode()).hexdigest()[:6]
            if not DRY:
                # whole or not at all: another process of this runner (another lane) may be reading it
                tmp = m.with_name(f"studios.json.{os.getpid()}.tmp")
                tmp.write_text(json.dumps(names, indent=1))
                tmp.replace(m)
        return A / names[key]


def session_dir(d):
    return SESS / ("--" + str(d).strip("/").replace("/", "-") + "--")


def brief(profile, d, pictures=None, reference=True):
    """pictures: how many pictures the studio's reference/ holds (profiles in ARTIST; counted there if None):
    the opening says one picture or several. reference=False: a studio without the pictures gets the
    profile's plain opening and reading list."""
    opening, reading = OPENING[profile], READING[profile]
    if profile in ARTIST and not reference:
        opening, reading = OPENING_PLAIN[profile], READING_PLAIN[profile]
    elif profile in ARTIST:
        n = len(reference_pictures(d / "reference")) if pictures is None else pictures
        if n < 1:
            raise RuntimeError(f"{d / 'reference'} holds no picture, and the {profile} brief sends the painter there")
        opening = OPENING_ONE[profile] if n == 1 else opening
    tpl = (HERE / "brief_template.md").read_text()
    return (tpl.replace("{OPENING}", opening).replace("{STUDIO}", f"~/src/a/{d.name}")
               .replace("{READING}", reading))


def reference_pictures(src):
    """The picture files in src (by suffix, not hidden), by name."""
    if not src.is_dir():
        return []
    return sorted((f for f in src.iterdir() if f.is_file() and not f.name.startswith(".")
                   and f.suffix.lower() in IMAGE_SUFFIXES), key=lambda f: f.name)


def reference_readme(src):
    """The owner's README.md in src (any case), or None."""
    return next((f for f in sorted(src.iterdir()) if f.is_file() and f.name.lower() == "readme.md"), None) if src.is_dir() else None


PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"


def image_kind(f):
    """What pi's read tool would show f as (it goes by the content: utils/mime.js): "jpeg", "png" (not an
    animated one) or "webp"; None if it wouldn't show it as an image (it would read it as text)."""
    with open(f, "rb") as fh:
        b = fh.read(4100)
    if b[:3] == b"\xff\xd8\xff":
        return "jpeg" if b[3:4] != b"\xf7" else None
    if b[:8] == PNG_SIGNATURE and len(b) >= 16 and b[12:16] == b"IHDR":
        at = 8
        while at + 8 <= len(b):
            kind = b[at + 4:at + 8]
            if kind == b"acTL":
                return None
            if kind == b"IDAT":
                break
            at += 12 + int.from_bytes(b[at:at + 4], "big")
        return "png"
    if b[:4] == b"RIFF" and b[8:12] == b"WEBP":
        return "webp"
    return None


def listed_readme(profile, pictures):
    """reference/README.md when the owner gave none: the files, plainly."""
    return (f"# reference/\n\nPictures of paintings by {ARTIST[profile]}:\n\n"
            + "".join(f"- {p.name}\n" for p in pictures))


def names_in(text, own):
    """Painters' names in text a studio may not hold (the export's names check, less own): the hits, or []."""
    with tempfile.TemporaryDirectory() as t:
        (Path(t) / "notes").mkdir()
        (Path(t) / "notes" / "README.md").write_text(text)
        r = subprocess.run([str(NAMES), t] + own, stdin=subprocess.DEVNULL, capture_output=True, text=True)
    return ([h for h in r.stderr.splitlines()[1:] if h] or [gist(r.stderr)]) if r.returncode else []


def reference_problems(profile, src):
    """Why src can't give a studio of profile its pictures (empty if it can)."""
    put = (f"put pictures of {ARTIST[profile]}'s paintings (jpg, jpeg, png or webp files) in {src}, and if you like "
           f"a README.md there describing them")
    if not src.is_dir():
        return [f"there is no folder {src}: {put}"]
    pictures = reference_pictures(src)
    if not pictures:
        return [f"there is no picture in {src}: {put}"]
    problems = [f"{src / p.name} isn't a picture pi's read tool shows (a JPEG, a PNG that isn't animated or a WebP)"
                for p in pictures if not image_kind(p)]
    if not NAMES.exists():
        return problems + [f"can't check {src}'s README and file names for painters' names: no {NAMES}"]
    readme = reference_readme(src)
    # what the painter can read of them: the README (hers, or the list of files) and the file names
    text = (readme.read_text(errors="replace") if readme else listed_readme(profile, pictures)) + "".join(
        f"\n{p.name}" for p in pictures)
    hits = names_in(text, OWN_NAMES[profile])
    if hits:
        problems.append(f"{src}'s README or file names name other painters, which no studio holds: {'; '.join(hits)}")
    return problems


def copy_reference(profile, src, dest):
    """Copy the pictures in src into dest (<studio>/reference), and src's README.md or, if there is none,
    one listing them. A picture larger than pi's read tool shows (READ_MAX_SIDE) is copied resized to
    fit, in its own format, its colors kept; one its EXIF turns is copied upright (pi sends a picture that
    fits as its bytes are, EXIF and all). src's files stay as they are. Returns lines for the log."""
    from PIL import Image, ImageOps
    Image.MAX_IMAGE_PIXELS = None                 # the owner's own files: no decompression-bomb guard
    pictures = reference_pictures(src)
    dest.mkdir(parents=True, exist_ok=True)
    said = []
    for p in pictures:
        out = dest / p.name
        with Image.open(p) as im:
            size = im.size
            turned = im.getexif().get(0x0112, 1) not in (0, 1)       # EXIF Orientation
            if max(size) <= READ_MAX_SIDE and not turned:
                shutil.copyfile(p, out)
            else:
                small = ImageOps.exif_transpose(im)
                small.thumbnail((READ_MAX_SIDE, READ_MAX_SIDE), Image.LANCZOS)
                kind = image_kind(p)
                keep = {"icc_profile": im.info["icc_profile"]} if im.info.get("icc_profile") else {}
                if kind == "jpeg":
                    small.save(out, "JPEG", quality=92, **keep)
                elif kind == "webp":
                    small.save(out, "WEBP", quality=92, **keep)
                else:
                    small.save(out, "PNG", optimize=True, **keep)
                said.append(f"{p.name}: {size[0]}x{size[1]} copied as {small.size[0]}x{small.size[1]}"
                            + (", upright (its EXIF turned it)" if turned else "")
                            + (f" (pi's read tool shows at most {READ_MAX_SIDE} px a side)" if max(size) > READ_MAX_SIDE else ""))
        b64 = (out.stat().st_size + 2) // 3 * 4
        if b64 >= READ_MAX_BASE64:
            said.append(f"{p.name}: {b64 / 1e6:.1f} MB of base64, over pi's {READ_MAX_BASE64 / 1e6:.1f} MB: "
                        f"its read tool re-encodes it as JPEG at each read")
    readme = reference_readme(src)
    if readme:
        shutil.copyfile(readme, dest / "README.md")
    else:
        (dest / "README.md").write_text(listed_readme(profile, pictures))
    total = sum((f.stat().st_size + 2) // 3 * 4 for f in dest.iterdir() if f.suffix.lower() in IMAGE_SUFFIXES)
    said.append(f"{len(pictures)} picture(s) in reference/ ({total / 1e6:.1f} MB of base64; a request keeps at most 12 MB "
                f"of images, context-images.ts), README.md {'from ' + str(src) if readme else 'listing them'}")
    left = sorted(f.name for f in src.iterdir() if not f.name.startswith(".") and f not in pictures and f != readme)
    if left:
        said.append(f"left out of reference/ (not jpg, jpeg, png or webp): {', '.join(left)}")
    return said


def key_args(m):
    # key_from: a provider whose stored key is a placeholder (opencode Zen) gets another entry's key
    return (["--api-key", f"<{m['key_from']} key from auth.json>" if DRY else api_key(m["key_from"])]
            if m.get("key_from") else [])


def painter_cmd(m, message=PAINTER_MSG, session=None):
    """session: a session file to continue (the painter carries on in it) instead of a new one."""
    cont = ["--session", str(session)] if session else []
    return (["pi", "--print"] + HARNESS + (["-e", str(BLACK)] if m["black"] else []) + m["args"] + key_args(m)
            + cont + [message])


def painter_env(m, message):
    """The painter's environment: a later sitting (SITTING_MESSAGE) gets PAINTER_SITTING_RECOVERY=1,
    so painter.ts opens it with what a compaction summary holds (brief, journal, globals, clock)
    and a fresh look at the canvas."""
    return {**(m["env"] or {}), **({"PAINTER_SITTING_RECOVERY": "1"} if message == SITTING_MESSAGE else {})}


def probe_cmd(m):
    """A one-line request to the painter's provider and model: no tools, no session, no studio."""
    return (["pi", "--print", "--no-session", "--no-extensions", "--no-tools", "--no-context-files", "--no-skills",
             "--no-prompt-templates"] + (["-e", str(BLACK)] if m["black"] else []) + m["args"] + key_args(m)
            + ["Reply with the word ok."])


def count_chunks(d):
    f = d / "paintings/lua/painting.lua"
    if not f.exists():
        return 0
    return sum(1 for line in f.read_text(errors="replace").splitlines() if line.startswith("--@ chunk"))


def count_painting(d):
    """Chunks of the log that put marks on the canvas (see painting_chunks.py)."""
    f = d / "paintings/lua/painting.lua"
    return count_painting_chunks(f.read_text(errors="replace")) if f.exists() else 0


def added_painting(s):
    """Whether a sitting added painting chunks (chunk counts for records without painting counts)."""
    before, after = (("painting_before", "painting_after") if s.get("painting_after") is not None
                     else ("chunks_before", "chunks_after"))
    return s.get(after) is not None and s.get(before) is not None and s[after] > s[before]


def next_step(sittings, max_sittings=MAX_SITTINGS, max_crashes=MAX_CRASHES):
    """What the painter does next: ("new", k) a fresh sitting numbered k, ("continue", k) the
    session of sitting k, cut off by a usage limit, carries on, or (None, why) the painter is done.

    sittings: the records so far, in order ({'sitting', 'painting_before', 'painting_after',
    'chunks_before', 'chunks_after', 'status', 'worked'}); a continued sitting's later parts
    repeat its number and carry its painting_before, so a record's counts cover the whole sitting.
    The painter finishes after a sitting it ended itself (status 'completed') that added no painting
    chunks and reviewed whole and detail views, or stops unfinished after max_sittings sittings:
    completed ones, and crashed ones that added painting (cut off after it painted, not judged).
    Sittings cut off by a usage limit ('limited') or the runner stopping ('interrupted') count for
    nothing; if the painter had worked in one (a reply that wasn't an error: 'worked'), its session
    carries on (a limited one once the limit lifts); otherwise a fresh sitting follows. After
    max_crashes crashes the painter stops (not finished).
    A record without painting counts (from an older runner) is judged by its chunk counts.
    """
    if not sittings:
        return ("new", 1)
    done = [s for s in sittings if s.get("status") == "completed"]
    if done and not added_painting(done[-1]) and done[-1].get("reviewed"):
        return (None, f"the painter is done: sitting {done[-1]['sitting']} reviewed whole and detail views and added no painting")
    counted = {s["sitting"] for s in sittings if s.get("status") == "completed"
               or (s.get("status") == "crashed" and added_painting(s))}
    if max_sittings and len(counted) >= max_sittings:
        return (None, f"NOT FINISHED: stopped after {max_sittings} sittings (MAX_SITTINGS)")
    if sum(1 for s in sittings if s.get("status") == "crashed") >= max_crashes:
        return (None, f"NOT FINISHED: {max_crashes} crashes (MAX_CRASHES)")
    last = sittings[-1]
    if last.get("status") in ("limited", "interrupted") and last.get("worked") and last.get("sessions"):
        return ("continue", last["sitting"])
    return ("new", max(s["sitting"] for s in sittings) + 1)


def session_worked(files):
    """Whether any of these pi session files holds an assistant reply that wasn't an error."""
    for f in files:
        try:
            for line in open(f, errors="replace"):
                if "assistant" not in line:
                    continue
                m = json.loads(line).get("message") or {}
                if m.get("role") == "assistant" and m.get("stopReason") not in ("error", "aborted"):
                    return True
        except (OSError, ValueError):
            continue
    return False


def session_reviewed(files):
    """A normal whole-canvas look and detail crop delivered after the latest successful paint call.

    Count returned images, not requested views, palette charts or the recovery snapshot.
    Missing or unreadable evidence cannot establish completion.
    """
    whole = detail = False
    for f in files:
        calls = {}
        try:
            with open(f, errors="replace") as stream:
                for line in stream:
                    m = json.loads(line).get("message") or {}
                    content = m.get("content") or []
                    if not isinstance(content, list):
                        continue
                    if m.get("role") == "assistant":
                        for c in content:
                            if c.get("type") == "toolCall":
                                calls[c.get("id")] = c
                    if m.get("role") != "toolResult" or m.get("isError") is not False:
                        continue
                    if m.get("toolName") == "paint":
                        whole = detail = False
                    call = calls.pop(m.get("toolCallId"), {})
                    if call.get("name") != "look" or not any(c.get("type") == "image" for c in content):
                        continue
                    args = call.get("arguments") or {}
                    if args.get("palette") or args.get("mode"):
                        continue
                    if args.get("crop"):
                        detail = True
                    else:
                        whole = True
        except (OSError, ValueError):
            return False
    return whole and detail


def session_reply(files):
    """The painter's reply: the last assistant message in the last of these session files that is
    text with no pending tool call (a completed turn). pi --print emits the same text on stdout, but
    not always (the Anthropic/pi-black path left stdout empty once, while the session kept the text);
    the session file is the authoritative record, so prefer it and fall back to stdout."""
    for f in reversed(files):
        try:
            with open(f, "rb") as fh:
                fh.seek(0, 2)
                fh.seek(max(0, fh.tell() - 4_000_000))
                lines = fh.read().decode(errors="replace").splitlines()
        except OSError:
            continue
        for line in reversed(lines):
            try:
                m = json.loads(line).get("message") or {}
            except ValueError:
                continue
            if m.get("role") != "assistant" or m.get("stopReason") in ("error", "aborted"):
                continue
            content = m.get("content")
            if not isinstance(content, list) or any(p.get("type") == "toolCall" for p in content):
                continue
            text = "\n".join(p.get("text", "") for p in content if p.get("type") == "text").strip()
            if text:
                return text
    return None


def session_error(files):
    """The provider error the last of these pi session files ended on (its last assistant message
    has stopReason 'error'), or None."""
    for f in reversed(files):
        try:
            with open(f, "rb") as fh:
                fh.seek(0, 2)
                fh.seek(max(0, fh.tell() - 4_000_000))
                lines = fh.read().decode(errors="replace").splitlines()
        except OSError:
            continue
        for line in reversed(lines):
            try:
                m = json.loads(line).get("message") or {}
            except ValueError:
                continue
            if m.get("role") == "assistant":
                return (m.get("errorMessage") or "unknown provider error") if m.get("stopReason") == "error" else None
    return None


def retry_hint(text):
    """The longest "retry in Ns" (or "retryDelay": "Ns") a provider error asks for, in s (0 if none)."""
    hints = re.findall(r"retry in ([\d.]+)\s*s", text, re.I) + re.findall(r'retryDelay\W+([\d.]+)s', text)
    return max((float(h) for h in hints), default=0.0)


def usage_limit(text):
    """Whether a provider error is a usage limit that resets with time (OpenCode Go's 5-hour,
    weekly and monthly windows), not a crash. An empty balance (Google's 402 "credits are
    depleted") is not: waiting doesn't refill it."""
    return bool(re.search(r"GoUsageLimitError|FreeUsageLimitError|usage limit (exceeded|reached)", text, re.I))


def limit_reset_s(text):
    """Seconds until a usage limit resets, from "Resets in 2hr 15min" (or "Resets in 2 days"), or None."""
    m = re.search(r"resets? in ((?:\s*[\d.]+\s*(?:days?|d|hours?|hrs?|h|minutes?|mins?|m|seconds?|secs?|s)\b)+)", text, re.I)
    if not m:
        return None
    unit = {"d": 86400, "h": 3600, "m": 60, "s": 1}
    return sum(float(v) * unit[u[0].lower()] for v, u in re.findall(r"([\d.]+)\s*([a-z]+)", m.group(1), re.I))


def first_probe_wait(text, ended, now):
    """Seconds before the first probe after a sitting that ended on a usage limit at `ended`
    (epoch s): the reset time the error named (or LIMIT_PROBE_S), less the time since then (a
    resumed runner asks at once when that has passed)."""
    reset = limit_reset_s(text)
    return max(0.0, (reset + 60 if reset else LIMIT_PROBE_S) - (now - ended))


# A probe error that waiting won't fix: how the provider is used (key, credit, model, request), not
# an outage. Matched only in an error that isn't a usage limit or a transient one (TRANSIENT); when
# unsure, the probe is asked again. HTTP statuses as providers and pi print them: "code": 402,
# HTTP 401, status 403, or a leading "400: {...}".
STATUS = r'(?:"code"\W{0,3}|\bHTTP\W{0,2}|\bstatus(?: code)?\W{0,3}|\berror \((?=\d{3}\))|(?:^|\s)(?=\d{3}:\s))'
PROBE_FATAL = [
    ("the API key or its access was refused", re.compile(
        STATUS + r"40[13]\b|no API key|(?:invalid|incorrect|missing)\W+(?:x-)?api\W?key|authentication_error"
        r"|permission_error|\bunauthorized\b|\bforbidden\b|token is expired|expired (?:token|credentials)", re.I)),
    ("no credit or billing on the account", re.compile(
        STATUS + r"402\b|payment required|credit balance|credits are depleted|\bbilling\b", re.I)),
    ("the model isn't known to the provider", re.compile(
        r"\bmodel\b[^\n.]{0,80}\b(?:not found|does not exist)(?![^\n]{0,40}custom model id)|unknown model"
        r"|model_not_found|not_found_error|\bmodel\b[^\n.]{0,80}\bis not supported\b", re.I)),
    ("the request was malformed", re.compile(STATUS + r"400\b|invalid_request_error", re.I)),
]
# an outage, overload or rate limit: asked again even if a fatal-looking word is in it (Google's
# 429 "check your plan and billing details")
TRANSIENT = re.compile(STATUS + r"(?:429|5\d\d)\b|rate.?limit|too many requests|exceeded your current quota"
                       r"|overloaded|timed? ?out|ECONNRESET|ECONNREFUSED|ENOTFOUND|EAI_AGAIN|network"
                       r"|server_error|\bis unavailable\b", re.I)
# no credit, whatever status it comes with (Anthropic's 429 credits_required): fatal before TRANSIENT
OUT_OF_CREDIT = re.compile(r"credits_required|out_of_credits", re.I)


def probe_fatal(text):
    """Why a probe error means the painter should stop now (see PROBE_FATAL), or None."""
    if usage_limit(text):
        return None
    if OUT_OF_CREDIT.search(text):
        return "no credit or billing on the account"
    if TRANSIENT.search(text):
        return None
    return next((why for why, pat in PROBE_FATAL if pat.search(text)), None)


def probe_outcome(r):
    """What a probe's result (a CompletedProcess, or the TimeoutExpired it raised) says: ("available",
    reply) only when the probe exited 0 and the reply has the word ok or okay (any case); ("limited",
    text) for a usage limit; otherwise ("unavailable", why): a timeout, an error or an odd reply is no
    recovery; ("fatal", why) for an error waiting won't fix (probe_fatal)."""
    if isinstance(r, subprocess.TimeoutExpired):
        return "unavailable", f"the probe timed out after {r.timeout:.0f} s"
    out = (r.stdout or "") + (r.stderr or "")
    if usage_limit(out):
        return "limited", out
    if not (r.returncode == 0 and re.search(r"\bok(ay)?\b", r.stdout or "", re.I)):
        why = probe_fatal(out)
        if why:
            return "fatal", f"{why} (exit {r.returncode}): {gist(out, 160)}"
    if r.returncode:
        return "unavailable", f"the probe exited {r.returncode}: {gist(out, 160)}"
    if not (r.stdout or "").strip():
        return "unavailable", f"no reply to the probe: {gist(out, 160)}"
    if not re.search(r"\bok(ay)?\b", r.stdout, re.I):
        return "unavailable", f"an unexpected reply to the probe: {gist(out, 160)}"
    return "available", r.stdout


def wait_out_limit(m, tag, text, ended):
    """Wait until the provider answers again (probing it), or give up after LIMIT_GIVE_UP_H hours.
    True when the painter can go on: only a probe that got its reply (probe_outcome) counts; a
    probe that timed out or failed is logged as such and asked again, like a continuing limit. A
    probe error waiting won't fix (a refused key, no credit, an unknown model, a malformed request)
    stops the painter at once."""
    t0 = time.time()
    wait = first_probe_wait(text, ended, t0)
    attempt = 0
    while True:
        if time.time() + wait - t0 > LIMIT_GIVE_UP_H * 3600:
            log(f"{tag}: the usage limit hasn't lifted in {LIMIT_GIVE_UP_H} h ({attempt} probes); "
                f"painter stops (rerun to go on)")
            return False
        log(f"{tag}: usage limit; asking the provider again in {wait / 60:.0f} min (not a crash)")
        time.sleep(wait)
        (RUN / "probe").mkdir(exist_ok=True)
        attempt += 1
        try:
            r = subprocess.run(probe_cmd(m), cwd=RUN / "probe", stdin=subprocess.DEVNULL,
                               capture_output=True, text=True, timeout=600)
        except subprocess.TimeoutExpired as e:
            r = e
        state, what = probe_outcome(r)
        if state == "available":
            log(f"{tag}: the provider answers again after {(time.time() - t0) / 3600:.1f} h "
                f"(attempt {attempt}): {gist(what, 80)}")
            return True
        if state == "fatal":
            log(f"{tag}: PROBE ERROR, painter stops now (attempt {attempt}): {what}; waiting won't fix it, "
                f"rerun after fixing")
            return False
        if state == "limited":
            log(f"{tag}: still limited (attempt {attempt}): {gist(what, 80)}")
            reset = limit_reset_s(what)
            wait = reset + 60 if reset else LIMIT_PROBE_S
        else:
            log(f"{tag}: PROBE FAILED (attempt {attempt}), not a recovery: {what}")
            wait = LIMIT_PROBE_S


def crash_wait(n_crashes, text):
    """Seconds to wait after the n-th crash: CRASH_WAITS, at least the provider's retry hint plus 15 s."""
    return max(CRASH_WAITS[min(n_crashes, len(CRASH_WAITS)) - 1], retry_hint(text) + 15)


def gist(text, n=240):
    """One line of an error for the log."""
    t = " ".join(text.split())
    code = re.search(r'"code"\W+(\d{3})', text)
    quota = re.search(r'quotaId\W+(\w+)', text)
    head = " ".join(x for x in (code and f"HTTP {code.group(1)}", quota and quota.group(1)) if x)
    return (head + ": " if head else "") + (t[:n] + ("..." if len(t) > n else ""))


def load_sittings(rd, n):
    f = rd / f"p{n}_sittings.json"
    return json.loads(f.read_text()) if f.exists() else []


def save_sittings(rd, n, sittings):
    f = rd / f"p{n}_sittings.json"
    tmp = f.with_suffix(".json.tmp")
    tmp.write_text(json.dumps(sittings, indent=1))
    tmp.replace(f)


def close_easel(d):
    """End the painter's easel session cleanly if one is still up (the log stays), then stop leftovers."""
    if (d / "bin/easel").exists():
        subprocess.run(["timeout", "60", str(d / "bin/easel"), "close"], cwd=d,
                       stdin=subprocess.DEVNULL, capture_output=True)
    stop_leftovers(d)


def open_easel(d, tag):
    """Replay the painting before Pi starts, with progress in the runner log, so the sitting message
    ("the painting is as you left it") is true when the painter reads it."""
    p = subprocess.Popen([str(d / "bin/easel"), "open"], cwd=d, stdin=subprocess.DEVNULL,
                         stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    for line in p.stdout:
        if not line.startswith("resuming "):
            log(f"{tag}: {line.rstrip()}")
    rc = p.wait()
    if rc:
        log(f"{tag}: EASEL OPEN FAILED (exit {rc})")
    return rc == 0


def paint(name, n, d, rd):
    """Run the painter's sittings until next_step says stop; resumable from p<n>_sittings.json."""
    m = LANES[name]["model"]
    tag = f"{name}{n}"
    sittings = load_sittings(rd, n)
    for s in sittings:
        if s.get("status") == "running":      # the runner stopped during this sitting
            t0 = time.mktime(time.strptime(s["start"], "%Y-%m-%d %H:%M:%S")) - 5
            files = s.get("sessions") or sorted((f for f in session_dir(d).glob("*.jsonl") if f.stat().st_mtime >= t0),
                                                key=lambda f: f.stat().st_mtime)[-1:]
            s.update(status="interrupted", chunks_after=count_chunks(d), painting_after=count_painting(d),
                     end=time.strftime("%F %T"), sessions=[str(f) for f in files], worked=session_worked(files))
            log(f"{tag}: sitting {s['sitting']} was interrupted (the runner stopped); "
                + ("it carries on in its session" if s["worked"] else "it will be taken again"))
    for s in sittings:
        if s.get("status") == "completed" and s.get("exit") not in (0, None):   # an older runner judged a crash
            s.update(status="crashed", reclassified=time.strftime("%F %T"))
            log(f"{tag}: sitting {s['sitting']} exited {s['exit']}: counted as a crash, not a judgment")
    for s in sittings:
        if s.get("status") == "crashed" and usage_limit(s.get("error") or ""):   # an older runner counted it
            s.update(status="limited", reclassified=time.strftime("%F %T"))
            log(f"{tag}: sitting {s['sitting']} ended on a usage limit: not a crash")
    save_sittings(rd, n, sittings)
    for s in sittings:
        if s.get("status") == "limited" and "worked" not in s:      # an older runner didn't record it
            s["worked"] = session_worked(s.get("sessions") or [])
    save_sittings(rd, n, sittings)
    sitting_cap = MAX_SITTINGS
    cap_file = RUN / "max_sittings.txt"
    while True:
        try:
            cap = int(cap_file.read_text().strip())
            if cap < 1:
                raise ValueError("expected a positive integer")
            sitting_cap = cap
        except FileNotFoundError:
            pass
        except (OSError, ValueError) as e:
            log(f"{tag}: keeping sitting cap {sitting_cap}; {cap_file.name}: {e}")
        step, k = next_step(sittings, max_sittings=sitting_cap)
        if step is None:
            why_done = k
            log(f"{tag}: {why_done}")
            break
        if sittings and sittings[-1].get("status") == "limited":
            last = sittings[-1]
            ended = time.mktime(time.strptime(last["end"], "%Y-%m-%d %H:%M:%S")) if last.get("end") else time.time()
            if not wait_out_limit(m, tag, last.get("error") or "", ended):
                return None
        # one session across sittings: reattaches at once; replays the log only if the server is gone
        stop_leftovers(d, keep_server=True)
        if not open_easel(d, tag):
            log(f"{tag}: painter stops (the easel didn't open); rerun to retry")
            return None
        before = count_chunks(d)
        painting_before = count_painting(d)
        cont = sittings[-1] if step == "continue" else None
        if cont:
            # the same sitting carries on in its session: its counts start where the sitting started
            part = cont.get("part", 1) + 1
            msg, session = CONTINUE_MESSAGE, cont["sessions"][-1]
            first = dict(chunks_before=cont["chunks_before"], painting_before=cont["painting_before"])
        else:
            part, session = 1, None
            msg = PAINTER_MSG if before == 0 else SITTING_MESSAGE
            first = dict(chunks_before=before, painting_before=painting_before)
        stem = f"p{n}_s{k}" + (f"_part{part}" if part > 1 else "")
        known = set(session_dir(d).glob("*.jsonl"))
        rec = dict(sitting=k, part=part, status="running", start=time.strftime("%F %T"), end=None,
                   chunks_after=None, painting_after=None, **first,
                   message=msg, sessions=[], exit=None, final_file=str(rd / f"{stem}_final.txt"), final=None)
        sittings.append(rec)
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k}{f' continues (part {part}, in its session)' if cont else ''} ({m['name']}, painter harness, "
            f"{before} chunks on the easel, {painting_before} painting)")
        t0 = time.time()
        rc = run(painter_cmd(m, msg, session), d, rd / f"{stem}_final.txt", rd / f"{stem}_err.txt", env=painter_env(m, msg))
        stop_leftovers(d, keep_server=True)
        new = sorted(set(session_dir(d).glob("*.jsonl")) - known, key=lambda f: f.stat().st_mtime)
        if session:
            new = [Path(session)] + new
        api_error = session_error(new)
        err_text = (rd / f"{stem}_err.txt").read_text(errors="replace")
        crashed = rc != 0 or api_error is not None
        why = api_error or err_text.strip() or f"exit {rc}"
        limited = crashed and usage_limit(why + "\n" + err_text)
        stdout_reply = (rd / f"{stem}_final.txt").read_text(errors="replace")
        # the painter's reply: from the session (authoritative) when the sitting ended cleanly, else stdout
        final_reply = (session_reply(new) if not (crashed or limited) else None) or stdout_reply
        rec.update(status="limited" if limited else "crashed" if crashed else "completed", end=time.strftime("%F %T"),
                   chunks_after=count_chunks(d), painting_after=count_painting(d), exit=rc,
                   sessions=[str(f) for f in new], error=why[-4000:] if crashed else None,
                   worked=session_worked(new), reviewed=session_reviewed(new), final=final_reply)
        save_sittings(rd, n, sittings)
        log(f"{tag}: sitting {k} ended (exit {rc}{', USAGE LIMIT' if limited else ', CRASHED' if crashed else ''}) after {(time.time() - t0) / 60:.0f} min: "
            f"chunks {rec['chunks_before']} -> {rec['chunks_after']} "
            f"(painting {rec['painting_before']} -> {rec['painting_after']}), {len(new)} session file(s)")
        if not crashed and not added_painting(rec) and not rec["reviewed"]:
            log(f"{tag}: completion not established: missing successful whole-canvas or detail review")
        if limited:
            log(f"{tag}: sitting {k} ended on a usage limit ({gist(why)}): not a crash; the painter waits it out")
        elif crashed:
            crashes = sum(1 for s in sittings if s.get("status") == "crashed")
            if crashes >= MAX_CRASHES:
                log(f"{tag}: last error: {gist(why)}")
            wait = crash_wait(crashes, why + "\n" + err_text)
            if crashes < MAX_CRASHES:
                log(f"{tag}: sitting {k} crashed ({gist(why)}); crash {crashes} of at most {MAX_CRASHES}, "
                    f"not a judgment: another sitting in {wait:.0f} s")
                time.sleep(wait)
    close_easel(d)
    shutil.copy(sittings[-1]["final_file"], rd / f"p{n}_final.txt")
    (rd / f"p{n}.painted").write_text(f"{time.strftime('%F %T')} {len(sittings)} record(s): {why_done}\n")
    return sittings


# What reader_brief.md says a record isn't, in plain English words a pattern can catch (not a
# semantic review: a paraphrase, another language or a hint gets past it). Flagged, not rejected:
# the same words turn up in plain observations. What to do: a sentence that starts with a command
# (after a bullet, a "Label:" or a full stop).
PRESCRIPTION = re.compile(r"(?:^|[.;!?]\s)\s*(?:[-*]\s+|\d+[.)]\s+)?(?:[\w ]{1,30}:\s+)?"
                          r"(?:always|never|don't|do not|keep|put|place|make|use|avoid|try|start|begin|"
                          r"remember|leave|let|save|you should|you must|one should)\b", re.I)
# where things go in the picture: a part of the picture, left or right, or composition words
PLACEMENT = re.compile(r"\b(?:center|centre|middle|top|bottom|upper|lower|corner|edge|third|quarter)"
                       r" of the (?:picture|canvas|painting|composition|image|frame)\b|\bon the (?:left|right)\b"
                       r"|\b(?:horizon|foreground|background|middle ground|composition|focal|motif)\b", re.I)


def record_flags(path):
    """Lines of the record at path that look like what to do or where things go in the picture:
    [(what, line number, line)]. A warning, not a rejection: the patterns also catch plain
    observations, so a flagged record still reaches the next studio."""
    if not path.exists():
        return []
    lines = path.read_text(errors="replace").strip().splitlines()
    # a line that carries on the sentence before it (a wrapped line) doesn't start a sentence
    cont = ["\u2026" + line if i and lines[i - 1].strip() and not re.search(r"[.!?:]\s*$|^\s*#", lines[i - 1])
            and not re.match(r"\s*(?:[-*]|\d+[.)])\s", line) else line for i, line in enumerate(lines)]
    return [(what, i, lines[i - 1]) for what, pat, ls in (("what to do", PRESCRIPTION, cont),
                                                           ("where things go in the picture", PLACEMENT, lines))
            for i, line in enumerate(ls, 1) if pat.search(line)]


# Painters' names a record may not hold: the round's names check (the one the export runs), less
# the studio's own artist (export_r16_studio's own=)
NAMES = BASE / "scripts/check_studio_names"
OWN_NAMES = {"friedrich": ["Friedrich"], "blank": [], "sargent": ["Sargent"], "inness": ["Inness"],
             "alma-tadema": ["Alma-Tadema", "Tadema"], "tonn": ["Tonn"], "hopper": ["Hopper"]}

# The free-text record's hard checks, round 19's (notes/round19/runner/r19_chains.py record_problems): a
# record that fails one isn't passed on; it is set aside as p<n>_record.rejected.md and the lane stops
RECORD_MAX_LINES = 60
# another painter: the word, or a name a lane's painter goes by (its model or maker)
PAINTER_WORDS = re.compile(r"\b(?:painters?|claude|opus|sonnet|gpt|gemini|kimi|mimo|codex|anthropic|openai|deepseek)\b", re.I)
# a line of code: a Lua statement, not an operation named in prose (`b:stroke`, `m:grow(14)`)
CODE_LINE = re.compile(r"^\s*(?:[-*]\s+)?(?:local\s|function\b|(?:for|while)\s.*\bdo\b|if\s.*\bthen\b|end\s*$"
                       r"|[A-Za-z_][\w.]*(?:\[[^\]]*\])?\s*=[^=])")
# a color recipe: a pile's makeup ({"<tube>", <parts>})
RECIPE = re.compile(r'\bpile\s*\{|\{\s*"[^"]+"\s*,\s*[\d.]+\s*[,}]')


def record_problems(path, own):
    """Why the free-text record at path can't go into the next studio's notes (empty if it can)."""
    if not path.exists():
        return ["no record was written"]
    text = path.read_text(errors="replace")
    if not text.strip():
        return ["the record is empty"]
    lines = text.strip().splitlines()
    problems = []
    if len(lines) > RECORD_MAX_LINES:
        problems.append(f"{len(lines)} lines (at most {RECORD_MAX_LINES})")
    if "```" in text:
        problems.append("a code block")
    for what, pat in (("code", CODE_LINE), ("a color recipe", RECIPE)):
        hits = [str(i) for i, line in enumerate(lines, 1) if pat.search(line)]
        if hits:
            problems.append(f"{what} on line {', '.join(hits)}")
    words = sorted({w.lower() for w in PAINTER_WORDS.findall(text)})
    if words:
        problems.append(f"another painter ({', '.join(words)})")
    with tempfile.TemporaryDirectory(dir=path.parent) as t:
        (Path(t) / "notes").mkdir()
        shutil.copy(path, Path(t) / "notes" / path.name)
        r = subprocess.run([str(NAMES), t] + own, capture_output=True, text=True)
    if r.returncode:
        hits = [h for h in r.stderr.splitlines()[1:] if h] or [gist(r.stderr)]
        problems.append(f"painters' names ({'; '.join(hits)})")
    return problems


def reader_cmd(prompt):
    """The reader's launch (READER_HARNESS), cwd the run folder."""
    return ["pi", "--print"] + READER_HARNESS + READER + [prompt]


def export_cmd(profile, d):
    return [str(EXPORT), EXPORT_AS.get(profile, profile), str(d)]


def finish_cmd(d, name, n):
    return [str(FINISH), str(d / "paintings/lua/painting.lua"), str(RUN / f"{name}{n}_finished.png")]


def check_cmd(d, name, n):
    return ["nice", str(CHECK), str(d), str(RUN / name / f"p{n}_check")]


def run(cmd, cwd, out, err, env=None):
    with open(out, "w") as o, open(err, "w") as e:
        return subprocess.run(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=o, stderr=e,
                              env={**os.environ, **env} if env else None).returncode


def show(tag, what, cmd, cwd, env=None):
    pre = " ".join(f"{k}={shlex.quote(v)}" for k, v in (env or {}).items())
    log(f"{tag}: {what}:\n    cd {shlex.quote(str(cwd))} && {pre + ' ' if pre else ''}{shlex.join(cmd)}")


def clear_settings(d):
    """No settings file in the studio (compaction.ts sets the thresholds); remove one if present."""
    f = d / ".pi" / "settings.json"
    if f.exists():
        f.unlink()
    if (d / ".pi").is_dir() and not any((d / ".pi").iterdir()):
        (d / ".pi").rmdir()


def in_background(fn):
    th = threading.Thread(target=fn)
    th.start()
    background.append(th)


def finish(name, n, d):
    """Varnish and crack a replay of the painter's log, in the background; the painter's save stays as is."""
    rd = RUN / name
    tag = f"{name}{n}"
    if (rd / f"p{n}.finished").exists():
        return
    lua = d / "paintings/lua/painting.lua"
    if not lua.exists() or not lua.stat().st_size:
        log(f"{tag}: no log to finish at {lua}")
        return
    # which of the easel's ready-made handlings the painting called on (defaults_used.py)
    (rd / f"p{n}_defaults.json").write_text(json.dumps(defaults_used(lua.read_text(errors="replace")), indent=1) + "\n")
    # mirrored or rotated coordinate reads (the brief's rule; copied_shapes.py): flags for a person to read
    copied = copied_shapes(lua.read_text(errors="replace"))
    (rd / f"p{n}_copied_shapes.json").write_text(json.dumps(copied, indent=1) + "\n")
    if copied:
        log(f"{tag}: COPIED SHAPES? {len(copied)} mirrored or rotated coordinate reads (see {rd}/p{n}_copied_shapes.json)")

    def go():
        log(f"{tag}: finishing in the background -> {RUN / f'{tag}_finished.png'}")
        rc = run(finish_cmd(d, name, n), RUN, rd / f"p{n}_finish.log", rd / f"p{n}_finish_err.txt")
        if rc == 0:
            (rd / f"p{n}.finished").write_text(time.strftime("%F %T"))
            log(f"{tag}: finished painting written")
        else:
            log(f"{tag}: FINISHING FAILED (exit {rc}, see {rd}/p{n}_finish_err.txt)")

    in_background(go)


def check_result(text):
    """The "check:" line scripts/check_painting ends its report with, or a note that there is none."""
    lines = [l for l in text.splitlines() if l.startswith("check:")]
    return lines[-1] if lines else "check: no result line (see the log)"


def check(name, n, d):
    """Replay the painter's log with the replay build and compare (scripts/check_painting), in the
    background after the painter's last sitting; the result goes to the runner's log."""
    rd = RUN / name
    tag = f"{name}{n}"
    if (rd / f"p{n}.checked").exists():
        return
    lua = d / "paintings/lua/painting.lua"
    if not lua.exists() or not lua.stat().st_size:
        log(f"{tag}: no log to check at {lua}")
        return

    def go():
        work = rd / f"p{n}_check"
        if work.exists():              # an earlier check was cut off
            shutil.rmtree(work)
        log(f"{tag}: checking the log in the background (replay build) -> {rd}/p{n}_check.log")
        t0 = time.time()
        rc = run(check_cmd(d, name, n), RUN, rd / f"p{n}_check.log", rd / f"p{n}_check_err.txt")
        result = check_result((rd / f"p{n}_check.log").read_text(errors="replace"))
        (rd / f"p{n}.checked").write_text(f"{time.strftime('%F %T')} exit {rc}\n{result}\n")
        log(f"{tag}: {'' if rc == 0 else 'CHECK NOT OK: '}{result} (exit {rc}, {(time.time() - t0) / 60:.0f} min)")

    in_background(go)


def stop_leftovers(d, keep_server=False):
    """Stop easel processes of this studio (not the runner's check and finishing; not the
    server with keep_server: the session stays open across sittings)."""
    r = subprocess.run(["/bin/ps", "-Ao", "pid=,command="], capture_output=True, text=True).stdout
    for line in r.splitlines():
        pid, _, cmd = line.strip().partition(" ")
        if keep_server and " serve " in f" {cmd} ":
            continue
        if str(d) in cmd and "easel" in cmd and "finish_painting" not in cmd and "check_painting" not in cmd:
            subprocess.run(["kill", pid])
            log(f"stopped leftover {pid}: {cmd[:100]}")


# The lanes this process runs (main() sets it; None: every lane). run/studios.json is shared by every process
# started from this folder, and a process watches only its own lanes' studios (our_studios).
MY_LANES = None


def our_studios():
    """This process's studios (key, folder name): those in run/studios.json of the lanes it runs. Two
    processes watching one painting's log would each compare it with what the other last recorded, and a
    log read a moment apart would look as if it had got shorter."""
    m = RUN / "studios.json"
    mine = None if MY_LANES is None else {f"{l}{n}" for l in MY_LANES for n in range(1, LANES[l]["painters"] + 1)}
    return [(k, v) for k, v in (json.loads(m.read_text()).items() if m.exists() else [])
            if mine is None or k in mine]


def monitor_histories():
    """Each painting's log may only grow: record it, and alert if a history got shorter or changed."""
    mon = RUN / "monitor"
    mon.mkdir(exist_ok=True)
    for key, name in our_studios():
        f = A / name / "paintings" / "lua" / "painting.lua"
        if not f.exists():
            continue
        now = f.read_bytes()
        last = mon / f"{key}.last"
        if last.exists():
            before = last.read_bytes()
            if not now.startswith(before):
                (mon / f"{key}.before-{int(time.time())}").write_bytes(before)
                log(f"ALERT {key} ({name}): the painting's history got shorter or changed ({len(before)} -> {len(now)} bytes)")
        last.write_bytes(now)


def etime_seconds(etime):
    """Seconds from ps's etime ([[dd-]hh:]mm:ss), or None. (macOS ps has no `etimes`: rounds 17
    and 18 asked for it, got a PID and COMMAND listing, and their watchdog skipped every line.)"""
    m = re.fullmatch(r"(?:(\d+)-)?(?:(\d+):)?(\d+):(\d+)", etime.strip())
    if not m:
        return None
    d, h, mi, s = (int(x) if x else 0 for x in m.groups())
    return ((d * 24 + h) * 60 + mi) * 60 + s


def spared(cmd):
    """Processes in a studio the watchdog leaves alone: the painter's pi (it titles itself "pi"),
    the easel server (it lives for the whole session), `easel open` (it replays the log, tens of
    minutes for a big painting, and gives up by itself when the replay stalls) and the runner's own
    check and finishing."""
    exe = os.path.basename(cmd.split()[0]) if cmd.split() else ""
    return (exe == "pi" or " serve " in f" {cmd} " or "finish_painting" in cmd or "check_painting" in cmd
            or cmd.rstrip().endswith("easel open"))    # a replay; `open` stops itself after 30 min without progress


def overdue(ps_lines, cwds, studios, limit_s, me=None):
    """The (pid, seconds, cwd, command) of processes to stop: running longer than limit_s, working
    in one of `studios` (cwd in it, or its path in the command line) and not spared.
    ps_lines: `ps -Ao pid=,etime=,command=` lines; cwds: pid -> cwd."""
    out = []
    for line in ps_lines:
        parts = line.split(None, 2)
        if len(parts) < 3 or not parts[0].isdigit():
            continue
        pid, etime, cmd = parts
        secs = etime_seconds(etime)
        if secs is None or secs < limit_s or pid == me or spared(cmd):
            continue
        where = cwds.get(pid, "")
        if any(where == s or where.startswith(s + "/") or s + "/" in cmd or cmd.endswith(s) for s in studios):
            out.append((pid, secs, where, cmd))
    return out


def cwds_of_all():
    """pid -> cwd of every process, from one lsof call."""
    r = subprocess.run(["lsof", "-a", "-d", "cwd", "-Fpn"], capture_output=True, text=True).stdout
    out, pid = {}, None
    for l in r.splitlines():
        if l.startswith("p"):
            pid = l[1:]
        elif l.startswith("n") and pid:
            out[pid] = l[1:]
    return out


def watchdog(stop):
    """Stop processes working in one of this round's studios that run too long; watch the logs grow."""
    while not stop.is_set():
        try:
            monitor_histories()
        except Exception as e:
            log(f"monitor error: {e}")
        try:
            mine = [str(A / name) for _, name in our_studios()]
            ps = subprocess.run(["/bin/ps", "-Ao", "pid=,etime=,command="], capture_output=True, text=True).stdout.splitlines()
            for pid, secs, where, cmd in overdue(ps, cwds_of_all() if mine else {}, mine, WATCHDOG_MIN * 60, str(os.getpid())):
                subprocess.run(["kill", pid])
                log(f"WATCHDOG stopped {pid} after {secs // 60} min in {where}: {cmd[:140]}")
        except Exception as e:
            log(f"watchdog error: {e}")
        stop.wait(60)


def chain(name):
    t = LANES[name]
    rd = RUN / name
    if not DRY:
        rd.mkdir(parents=True, exist_ok=True)
        if t["painters"] > 1 and t["record_kind"] == "structured":
            (rd / "condition.json").write_text(json.dumps(condition(name, t), indent=1))
    env = {"R16_BRANCH": BRANCH}
    for n in range(1, t["painters"] + 1):
        d = studio(f"{name}{n}")
        tag = f"{name}{n}"
        if (rd / f"p{n}.done").exists():
            log(f"{tag}: done already")
            if not DRY:
                check(name, n, d)
                finish(name, n, d)
            if DRY or not unread(t, rd, n):
                continue
            log(f"{tag}: not read yet")
        if DRY:
            extra = " + trees.md" if t["profile"] == "friedrich" else ""
            if referenced(t):
                extra += f" + reference/ from {REFERENCE} ({len(reference_pictures(REFERENCE))} pictures)"
            merged = {"free-text": " + records", "structured": " + the records rendered for this studio (p<k>_record.json, "
                      f"see p{n}_inherited.json)", "none": ""}[t["record_kind"]]
            as_brief = (f"{t['profile']}-no-pictures" if t["profile"] in ARTIST and not referenced(t)
                        else f"{t['profile']}-one-picture" if referenced(t) and len(reference_pictures(REFERENCE)) == 1
                        else t["profile"])
            show(tag, f"export {t['profile']} studio (then studio_notes.md{merged if n > 1 else ''}{extra}, "
                      f"BRIEF.md as briefs/{as_brief}.md with this studio's path)", export_cmd(t["profile"], d), BASE, env)
            show(tag, f"open the easel ({d / 'bin/easel'} open); it stays open across sittings", [str(d / "bin/easel"), "open"], d)
            show(tag, "sitting 1 (painter)", painter_cmd(t["model"]), d, t["model"]["env"])
            show(tag, f"after each sitting: count painting chunks;\n"
                      f"    more sittings until one reviews whole and detail views and adds no painting chunks (safety cap {MAX_SITTINGS}; a usage limit continues its session),\n"
                      f"    at the same open easel (reopened, replaying the log, only if its server is gone); closed after the last",
                 painter_cmd(t["model"], SITTING_MESSAGE), d, painter_env(t["model"], SITTING_MESSAGE))
            show(tag, "check (background, after the last sitting; result in the log)", check_cmd(d, name, n), RUN)
            show(tag, "finishing (background, after the last sitting)", finish_cmd(d, name, n), RUN)
            if reads(t, n) and t["record_kind"] == "free-text":
                show(tag, "reader", reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd)
            elif reads(t, n) and t["record_kind"] == "structured":
                show(tag, f"reader (structured: writes p{n}_observations.json from reader_brief_structured.md; "
                          f"record_schema validates it into p{n}_record.json)",
                     reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd)
            continue
        if not (rd / f"p{n}.exported").exists():
            if d.exists():
                shutil.rmtree(d)
            log(f"{tag}: exporting {t['profile']} studio from {BRANCH} to {d}")
            r = subprocess.run(export_cmd(t["profile"], d), capture_output=True, text=True, env={**os.environ, **env})
            (rd / f"p{n}_export.log").write_text(r.stdout + r.stderr)
            if r.returncode:
                log(f"{tag}: EXPORT FAILED, lane stops (see {rd}/p{n}_export.log)")
                return
            if t["record_kind"] == "structured" and n > 1:
                inherit_structured(tag, rd, n, d)
            else:
                notes = [(HERE / "studio_notes.md").read_text()]
                for rec in t["records"]:
                    if not rec.exists():
                        log(f"{tag}: RECORD MISSING {rec}, lane stops")
                        return
                    notes.append("\n## More notes from the studio\n\n" + rec.read_text())
                for k in range(1, n if t["record_kind"] == "free-text" else 1):
                    rec = rd / f"p{k}_record.md"
                    if rec.exists():
                        notes.append("\n## More notes from the studio\n\n" + rec.read_text())
                (d / "notes" / "studio_notes.md").write_text("\n".join(notes))
            if t["profile"] == "friedrich":
                shutil.copy(HERE / "trees.md", d / "notes" / "research" / "trees.md")
                # without its source keys and list, as the export leaves the other research notes
                subprocess.run([str(BASE / "scripts/strip_sources"), str(d / "notes" / "research" / "trees.md")],
                               stdin=subprocess.DEVNULL, check=True)
            if referenced(t):
                # pictures of the artist's paintings (round 22); checked again here as main() checks them
                problems = reference_problems(t["profile"], REFERENCE)
                if problems:
                    log(f"{tag}: NO REFERENCE PICTURES, lane stops: {'; '.join(problems)}")
                    return
                for line in copy_reference(t["profile"], REFERENCE, d / "reference"):
                    log(f"{tag}: reference: {line}")
            (rd / f"p{n}.exported").write_text(time.strftime("%F %T"))
        if referenced(t) and not reference_pictures(d / "reference"):
            log(f"{tag}: NO REFERENCE PICTURES in {d / 'reference'}, lane stops (remove {rd}/p{n}.exported and rerun "
                f"to export the studio again)")
            return
        (rd / f"p{n}_brief.md").write_text(brief(t["profile"], d, reference=referenced(t)))
        (d / "BRIEF.md").write_text(brief(t["profile"], d, reference=referenced(t)))
        clear_settings(d)
        if not (rd / f"p{n}.painted").exists():
            if paint(name, n, d, rd) is None:
                log(f"{name}: chain stops")
                return
        check(name, n, d)
        finish(name, n, d)
        logs = [p for s in load_sittings(rd, n) for p in s.get("sessions", []) if Path(p).exists()]
        if reads(t, n) and t["record_kind"] == "structured" and not read_structured(
                tag, rd, n, d, logs, OWN_NAMES[t["profile"]], name, t["profile"]):
            log(f"{name}: chain stops")
            return
        if reads(t, n) and t["record_kind"] == "free-text":
            out = rd / f"p{n}_record.md"
            if not logs:
                log(f"{tag}: RECORD MISSING: no session logs to read; rerun to try again")
                log(f"{name}: chain stops")
                return
            out.unlink(missing_ok=True)                   # an earlier attempt's record isn't this one's
            copies, images = reader_logs(rd, n, d, logs)
            rb = ((HERE / "reader_brief.md").read_text().replace("{LOG}", ", ".join(copies))
                  .replace("{JOURNAL}", str(d / "notes/journal.md")).replace("{OUT}", str(out)))
            (rd / f"p{n}_reader_brief.md").write_text(rb)
            flagged = rd / f"p{n}_record.flags.md"
            flagged.unlink(missing_ok=True)               # an earlier attempt's flags aren't this record's
            log(f"{tag}: reader")
            # reader.ts lets it read these files only (a journal the painter never wrote isn't one) and write only out
            journal = d / "notes/journal.md"
            scope = {"read": [*copies, *images, *([str(journal)] if journal.exists() else []), str(rd / f"p{n}_reader_brief.md")],
                     "write": str(out)}
            rc = run(reader_cmd(f"Read {rd}/p{n}_reader_brief.md and do what it says."), rd,
                     rd / f"p{n}_reader_final.txt", rd / f"p{n}_reader_err.txt", {"READER_SCOPE": json.dumps(scope)})
            problems = ([f"the reader exited {rc}"] if rc else []) + record_problems(out, OWN_NAMES[t["profile"]])
            if problems:
                if out.exists():
                    out.replace(rd / f"p{n}_record.rejected.md")
                log(f"{tag}: RECORD REJECTED: {'; '.join(problems)} (see {rd}/p{n}_reader_err.txt and "
                    f"p{n}_record.rejected.md); the painter isn't done, rerun to read it again")
                log(f"{name}: chain stops")
                return
            flags = record_flags(out)
            if flags:
                flagged.write_text(f"# p{n}_record.md: lines the pattern gate flagged (a warning, not a rejection)\n"
                                   + "".join(f"- line {i} ({what}): {line.strip()}\n" for what, i, line in flags))
                log(f"{tag}: RECORD FLAGGED ({len(flags)} lines, see {flagged}): "
                    + "; ".join(f"line {i} ({what}): {line.strip()[:120]}" for what, i, line in flags))
            log(f"{tag}: record written"
                + (f" ({len(flags)} flagged lines went on with it, not a review of what it says)" if flags else ""))
        (rd / f"p{n}.done").write_text(time.strftime("%F %T"))
    log(f"lane {name} finished")


def round_number():
    """The round, from BRANCH (round-21, or round-21.1 for a prompt revision of it, stamped as round 21:
    the record's commit tells them apart): what a structured record is stamped with."""
    return int(re.fullmatch(r"round-(\d+)(?:\.\d+)?", BRANCH).group(1))


def code_commit():
    """The commit BRANCH names in BASE (the export builds the studio's easel from it); it stands
    in for the easel's version."""
    r = subprocess.run(["git", "-C", str(BASE), "rev-parse", "--verify", f"{BRANCH}^{{commit}}"],
                       capture_output=True, text=True)
    if r.returncode:
        raise RuntimeError(f"no commit for {BRANCH} in {BASE}: {r.stderr.strip()[:200]}")
    return r.stdout.strip()


def checkout_problems():
    """Why BASE isn't a clean checkout of BRANCH (empty if it is): the export archives BRANCH, but the
    harness (H), the check and the finishing run from BASE's files, so they must be the same commit."""
    try:
        want = code_commit()
    except RuntimeError as e:
        return [str(e)]
    git = lambda *a: subprocess.run(["git", "-C", str(BASE), *a], capture_output=True, text=True).stdout.strip()
    head = git("rev-parse", "HEAD")
    if head != want:
        return [f"{BASE} is at {head[:12] or 'no commit'}, not {BRANCH} ({want[:12]})"]
    if git("status", "--porcelain", "--untracked-files=no"):
        return [f"{BASE} has uncommitted changes to tracked files"]
    return []


TUBE_TABLE = "| tube | pigment | hiding | stiffness | tinting strength | drying |"


def box_of(d):
    """A studio's box: its name (bin/box, else default), the hash of the tube table its guide shows
    (the easel's `tubes --markdown`, put there by the export) and the tube names in it."""
    guide = (d / "notes/easel_guide.md").read_text().splitlines()
    start = guide.index(TUBE_TABLE)
    table = [guide[start]]
    for line in guide[start + 1:]:
        if not line.startswith("|"):
            break
        table.append(line)
    tubes = [line.split("|")[1].strip() for line in table[2:]]
    name = (d / "bin/box").read_text().strip() if (d / "bin/box").exists() else "default"
    return {"name": name, "tubes_sha256": hashlib.sha256("\n".join(table).encode()).hexdigest()}, tubes


LUA_TOKEN = re.compile(r'--\[(=*)\[.*?\]\1\]|--[^\n]*|"(?:\\.|[^"\\\n])*"|\'(?:\\.|[^\'\\\n])*\'|\[(=*)\[.*?\]\2\]', re.S)


def lua_code(text):
    """Lua text with its comments blanked out (strings kept)."""
    return LUA_TOKEN.sub(lambda m: " " if m.group(0).startswith("--") else m.group(0), text)


LUA_STRING = re.compile(r'"(?:\\.|[^"\\\n])*"|\'(?:\\.|[^\'\\\n])*\'|\[(=*)\[.*?\]\1\]', re.S)


def canvas_call(code):
    """The table of the first canvas{...} call in Lua code (comments already out), or "": the call
    and its braces are found outside strings."""
    masked = LUA_STRING.sub(lambda m: m.group(0)[0] + "x" * (len(m.group(0)) - 2) + m.group(0)[-1], code)
    m = re.search(r"\bcanvas\s*\{", masked)
    if not m:
        return ""
    depth = 0
    for i in range(m.end() - 1, len(masked)):
        depth += {"{": 1, "}": -1}.get(masked[i], 0)
        if depth == 0:
            return code[m.end() - 1:i + 1]
    return ""


def support_of(d):
    """The painting's support, read from the code of its canvas{} call (comments left out): the
    linen's threads per cm ({warp, weft}, or one number for both) and how each ground layer was
    applied, the easel's own words only (record_render.GROUND_APPLY); the ground piles are recipes,
    left out."""
    from record_render import GROUND_APPLY
    f = d / "paintings/lua/painting.lua"
    text = f.read_text(errors="replace") if f.exists() else ""
    chunks = [lua_code(c) for c in re.split(r"^--@ chunk \d+\n", text, flags=re.M)[1:]]
    call = next((canvas_call(c) for c in chunks if canvas_call(c)), "")
    number = r"(\d+(?:\.\d+)?)"
    pair = re.search(r"\blinen\s*=\s*\{\s*" + number + r"\s*,\s*" + number + r"\s*\}", call)
    one = re.search(r"\blinen\s*=\s*" + number + r"\b", call)
    linen = pair.groups() if pair else (one.group(1),) * 2 if one else None
    return {"kind": "linen", "linen": [float(x) if "." in x else int(x) for x in linen] if linen else None,
            "ground_layers": [a for a in re.findall(r'\bapply\s*=\s*"(\w+)"', call) if a in GROUND_APPLY]}


def recipient_of(d):
    box, tubes = box_of(d)
    return {"medium": "oil", "support_kind": "linen", "commit": code_commit(), "box": box, "tubes": tubes}


def sha256_file(f):
    return hashlib.sha256(Path(f).read_bytes()).hexdigest()


def reader_logs(rd, n, d, logs):
    """The reader's copies of the painter's logs (log_copy.py: images out of the lines, every line
    readable) in run/<lane>/p<n>_reader_logs, and the image files they name. Evidence ids are the
    originals', and record_schema checks evidence against the original logs."""
    return copy_logs(logs, d, rd / f"p{n}_reader_logs")


def reader_model():
    return f"{READER[READER.index('--provider') + 1]}/{READER[READER.index('--model') + 1]}"


def read_structured(tag, rd, n, d, logs, own, name, profile):
    """The reader writes p<n>_observations.json (reader_brief_structured.md); record_schema checks
    it against the session logs, the studio's box and the names check, and the runner keeps it
    with its metadata as p<n>_record.json. True if the record passed (some observations may be
    dropped, each logged with why); if not, the observations are set aside as
    p<n>_observations.rejected.json and the lane stops."""
    import record_render                          # only structured lanes need them (a free-text runner
    import record_schema                          # copy runs without record_*.py beside it)
    out = rd / f"p{n}_observations.json"
    rec = rd / f"p{n}_record.json"
    if not logs:
        log(f"{tag}: RECORD MISSING: no session logs to read; rerun to try again")
        return False
    out.unlink(missing_ok=True)                   # an earlier attempt's record isn't this one's
    rec.unlink(missing_ok=True)
    brief_file = rd / f"p{n}_reader_brief.md"
    copies, images = reader_logs(rd, n, d, logs)
    brief_file.write_text((HERE / "reader_brief_structured.md").read_text()
                          .replace("{LOGS}", "\n".join(f"- log {i}: {p}" for i, p in enumerate(copies, 1)))
                          .replace("{JOURNAL}", str(d / "notes/journal.md")).replace("{OUT}", str(out)))
    log(f"{tag}: reader (structured; {len(images)} images)")
    journal = d / "notes/journal.md"
    scope = {"read": [*copies, *images, *([str(journal)] if journal.exists() else []), str(brief_file)], "write": str(out)}
    rc = run(reader_cmd(f"Read {brief_file} and do what it says."), rd,
             rd / f"p{n}_reader_final.txt", rd / f"p{n}_reader_err.txt", {"READER_SCOPE": json.dumps(scope)})

    def reject(why):
        if out.exists():
            out.replace(rd / f"p{n}_observations.rejected.json")
        log(f"{tag}: RECORD REJECTED: {why} (see {rd}/p{n}_reader_err.txt and p{n}_observations.rejected.json); "
            f"the painter isn't done, rerun to read it again")
        return False

    if rc:
        return reject(f"the reader exited {rc}")
    if not out.exists():
        return reject("no record was written")
    try:
        obs = json.loads(out.read_text())
    except ValueError as e:
        return reject(f"not JSON ({e})")
    box, tubes = box_of(d)
    try:
        v = record_schema.validate(obs, tubes, [record_schema.LogIndex(p) for p in logs],
                                   record_schema.studio_names(NAMES, own, rd))
    except record_schema.Rejected as e:
        return reject(str(e))
    record = {"schema": record_render.RECORD_SCHEMA, "condition": record_render.CONDITION,
              "round": round_number(), "lane": name, "slot": n, "profile": profile,
              "code": {"tag": BRANCH, "commit": code_commit()}, "medium": "oil", "box": box, "support": support_of(d),
              "reader": {"model": reader_model(), "brief_sha256": sha256_file(brief_file),
                         "system_sha256": sha256_file(HERE / "reader_system_prompt.md")},
              "logs": list(logs), **v}
    rec.write_text(json.dumps(record, indent=1, ensure_ascii=False))
    for x in v["dropped"]:
        log(f"{tag}: RECORD DROPPED observation {x['index']}: {'; '.join(x['why'])}")
    for w in v["warnings"]:
        log(f"{tag}: RECORD FLAGGED observation {w['index']} {w['field']} ({w['pattern']}: {w['words']})")
    if not v["observations"]:
        log(f"{tag}: RECORD EMPTY: {len(obs['observations'])} observations written, none passed; the chain goes on "
            f"and the next studio inherits nothing new from this painter")
    log(f"{tag}: record written ({len(v['observations'])} of {len(obs['observations'])} observations passed the "
        f"hard checks, schema, evidence, words and names, not a review of what they say"
        + (f"; {len(v['warnings'])} flagged went on with them)" if v["warnings"] else ")"))
    return True


def inherit_structured(tag, rd, n, d):
    """The notes the n-th studio inherits: p1..p<n-1>_record.json rendered for this studio
    (record_render), with what was left out and why in p<n>_inherited.json and the log."""
    import record_render
    recs = []
    for k in range(1, n):
        f = rd / f"p{k}_record.json"
        recs.append((k, json.loads(f.read_text()) if f.exists() else None))
    rcp = recipient_of(d)
    text, report = record_render.render(recs, rcp, record_render.load_compat(HERE / "record_compat.json"))
    notes = (HERE / "studio_notes.md").read_text() + ("\n" + text if text else "")
    (d / "notes" / "studio_notes.md").write_text(notes)
    (rd / f"p{n}_inherited.json").write_text(json.dumps(record_render.inherited(report, rcp, notes.encode()), indent=1))
    log(record_render.summary(tag, report))


def condition(name, t):
    """run/<lane>/condition.json for a lane that inherits structured records: its label (a later
    painter's results aren't comparable with a blank studio's without it), the reader, and the
    hashes of the reader's briefs and of the code that checks and renders its records."""
    return {"condition": "chain-inherited, non-neutral", "lane": name, "painters": t["painters"],
            "record_kind": t["record_kind"], "profile": t["profile"],
            "reader": {"model": reader_model(), "system_sha256": sha256_file(HERE / "reader_system_prompt.md")},
            "reader_briefs_sha256": {f.name: sha256_file(f) for f in sorted(HERE.glob("reader_brief*.md"))},
            "renderer_sha256": {f: sha256_file(HERE / f) for f in ("record_schema.py", "record_render.py")}}


def mixed_records(name, kind):
    """Why lane name's run folder can't go on with this kind of record (a lane keeps one kind), or None."""
    rd = RUN / name
    other = {"structured": ["p*_record.md", "p*_record.rejected.md"],
             "free-text": ["p*_record.json", "p*_observations*.json"], "none": []}[kind]
    found = sorted(str(f.name) for pat in other for f in rd.glob(pat)) if rd.exists() else []
    return (f"lane {name} is record_kind={kind!r} but {rd} holds {', '.join(found)} (one kind of record per lane)"
            if found else None)


def render_briefs(out):
    """Every profile's brief, with a placeholder studio (paint-studio-000000) for the path and scratch name."""
    out.mkdir(parents=True, exist_ok=True)
    d = A / "paint-studio-000000"
    for profile in OPENING:
        (out / f"{profile}.md").write_text(brief(profile, d, pictures=2))
        print(out / f"{profile}.md")
        if profile in OPENING_ONE:                 # a studio with one picture in reference/
            (out / f"{profile}-one-picture.md").write_text(brief(profile, d, pictures=1))
            print(out / f"{profile}-one-picture.md")
        if profile in OPENING_PLAIN:               # a studio without the pictures (lane(..., reference=False))
            (out / f"{profile}-no-pictures.md").write_text(brief(profile, d, reference=False))
            print(out / f"{profile}-no-pictures.md")


def preflight(lanes):
    """Why these lanes can't start (empty if they can): a chain lane's reader needs its files next to
    this runner (a runner copied without them fails every reader at launch), a structured lane
    stamps each record with BRANCH's commit in BASE, and a lane whose studios get the artist's
    pictures (referenced) needs them in REFERENCE (reference_problems); a lane that isn't among
    `lanes`, or gets no pictures, isn't held to that."""
    problems = []
    # a studio with pictures of the artist's paintings (round 22): they must be there before anything starts
    for profile in sorted({LANES[l]["profile"] for l in lanes if referenced(LANES[l])}):
        problems += reference_problems(profile, REFERENCE)
    chains = [l for l in lanes if any(reads(LANES[l], n) for n in range(1, LANES[l]["painters"] + 1))]
    if chains:
        need = ["reader.ts", "reader-scope.ts", "reader_system_prompt.md", "reader_brief.md"]
        if any(LANES[l].get("record_kind") == "structured" for l in chains):
            need += ["reader_brief_structured.md", "record_schema.py", "record_render.py"]
        missing = [f for f in need if not (HERE / f).exists()]
        if missing:
            problems.append(f"chain lanes {', '.join(chains)} need {', '.join(missing)} next to {Path(__file__).name} in {HERE}")
        if READER_BLACK and not BLACK.exists():
            problems.append(f"the reader loads pi-black, which isn't at {BLACK}")
    if not DRY and any(LANES[l].get("record_kind") == "structured" and any(reads(LANES[l], n) for n in range(1, LANES[l]["painters"] + 1))
                       for l in lanes):
        try:
            code_commit()
        except RuntimeError as e:
            problems.append(str(e))
    return problems


def main():
    global DRY, MY_LANES
    ap = argparse.ArgumentParser()
    ap.add_argument("--only")
    ap.add_argument("--dry", action="store_true")
    ap.add_argument("--briefs", type=Path)
    a = ap.parse_args()
    if a.briefs:
        render_briefs(a.briefs)
        return
    DRY = a.dry
    lanes = a.only.split(",") if a.only else list(LANES)
    for l in lanes:
        if l not in LANES:
            raise SystemExit(f"no lane {l}; lanes: {', '.join(LANES)}")
        mixed = mixed_records(l, LANES[l].get("record_kind", "free-text"))
        if mixed:
            raise SystemExit(mixed)
    problems = preflight(lanes) + ([] if DRY else checkout_problems())
    if problems:
        raise SystemExit("can't start: " + "; ".join(problems))
    MY_LANES = lanes
    if not DRY:
        RUN.mkdir(parents=True, exist_ok=True)
    stop = threading.Event()
    if not DRY:
        threading.Thread(target=watchdog, args=(stop,), daemon=True).start()
    th = [threading.Thread(target=chain, args=(l,)) for l in lanes]
    for x in th:
        x.start()
        if DRY:
            x.join()          # one lane after the other, so the printout reads in order
        else:
            time.sleep(5)
    for x in th:
        x.join()
    if background:
        log("waiting for checks and finishing")
    for x in background:
        x.join()
    stop.set()
    log("all lanes finished")


if __name__ == "__main__":
    main()
