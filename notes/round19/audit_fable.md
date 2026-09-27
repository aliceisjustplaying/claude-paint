# claude-paint runbook audit: biases and implied instructions in what the painter is given

Read-only audit, 2026-09-27. Scratch: `~/tmp/audit-fable-2558ff56/`.

Materials read in full: `~/tmp/gallery-fcf9c110/r17/run/{F,O}/p1_brief.md`, `brief_template.md`,
`r17_chains.py`, `reader_brief.md`, `harness/painter/{system_prompt.md,compaction.ts,painter.ts,
context-images.ts,pace.ts,README.md,studio-settings.json}`, studios `paint-studio-497bcf` (F2) and
`paint-studio-c6b5dd` (O3): `BRIEF.md`, `notes/easel_guide.md`, `notes/studio_notes.md`,
`notes/research/*.md`, `.pi/settings.json`, `THIRD_PARTY_NOTICES.md`; `crates/easel/src/{main.rs,
check.rs,api.rs,world.rs,time.rs,finish.rs,prelude.lua}`; `scripts/export_r16_studio`; the sitting
finals and reader records under `run/` (for corroboration only).

Line numbers for the brief refer to `run/O/p1_brief.md` (O) and `run/F/p1_brief.md` (F); the template
(`brief_template.md`) has the same text with the F-only line removed, so I cite the rendered files.
`easel_guide.md`, `oil_paint_physics.md` and the first 176 lines of `studio_notes.md` are byte-identical
in both studios (checked with `diff`).

Confidence scale: **high** = the text names the thing and the observed behavior matches it closely;
**medium** = a plausible mechanism, partial support; **low** = possible, unsupported. "Supported" says
whether the observed behavior in the task statement (or the run's finals) backs it.

---

## 1. Top findings

| # | Where | What | Effect | Conf. | Sev. |
|---|---|---|---|---|---|
| 1 | O brief 36-39, F brief 36-41, template 32-35 | "A reflection in water is painted as its own shape, not as `mask:at(x, 2*h - y)`." | The only subject noun in the brief is water with a reflection. Every O painting has water and a painted reflection. | high | high |
| 2 | O brief 28-31 / F 29-33 (rules) | "(`bin/easel check`, which verifies the log against the canvas, is fine.)" | Permission read as expectation; sits among anti-tamper rules, so `check` becomes a proof of integrity. 10 of 12 sitting finals report it. | high | high |
| 3 | easel_guide.md 16-19, 45, 453-459 | "The log is the painting… bit for bit. The easel verifies the log…"; `check` row; the guide's last section is "## The log" about `check`. | Verification is the closing thought of the guide. | high | high |
| 4 | easel_guide.md 332-369 (World, Depth) | `eye=1.6`, `horizon`, `water` "a level reflecting surface", `sun`, `visibility`, `w:aerial`, `v:water()`, `v:mirror`, `v:land()`, `v:sky()`; things named `"ground"`, `"water"`, `"surface"`, `"sky"` | The blank studio's most elaborate tool is a land/water/sky landscape kit. | high | high |
| 5 | O brief 9 / F 11 / template 7 | "You paint at the easel, in one session" | Read as "one sitting": the bulk of every painting is in sitting 1 and every sitting's final opens "The painting is finished, saved and the session is closed." | high | med-high |
| 6 | r17_chains.py 73-74, 76-78 | "Complete your task autonomously… follow it exactly… Your FINAL message is the reply it asks for."; sitting message gives no reason for the return | Completion pressure and literalism; return sittings become check-save-close rituals with 0-4 chunks. | med-high | medium |
| 7 | reader records merged into `studio_notes.md` (497bcf:236; c6b5dd:235-236, 296) | "`check` took 574 s at 147 chunks and 787 s at 208"; "`bin/easel check` took about 300 s… After `close` it replies 'no painting open'"; "`check` took 338 s for 95 chunks and 420 s for 120" | Painters 2+ are told that `check` is what one does, when to do it, and how big paintings are here. | high | med-high |
| 8 | reader_brief.md 22-23 and the records it produced | "Restate a fact as geometry… ('a thin band', 'a dark mass against a light field')" → records full of "horizontal bands/blends", "grass", "tall stalks", "tufts and stems", "leaf sprays" | Subject leaks as geometry and nature nouns; O2/O3 inherit horizontal-band pictures, F2 inherits meadow grass. | med-high | medium |
| 9 | easel_guide.md 102 (+ the box itself) | "The tube box (all made by the 1820s)" | Friedrich's palette and date in the *blank* studio: 1820s = Constable/Turner; weak greens (green earth hiding 0.2, Rinmann's tint 0.4) make dusk silhouettes easier than daylight foliage. | medium | medium |
| 10 | easel_guide.md 29, 59, 241, 296, 321; api.rs 1371 | Every example is landscape-shaped: `aspect=1.25/1.4`, a warm ochre ground, `below(… 400 + 30*sin …)`, `h:rule({0, 400}, {1000, 400})`, `terrain{area={0, 400, 1000, 800}}` | Examples get copied: O3's chunk 1 is the CANVAS_HELP ground at `aspect=1.4`; chunk 2 is `h:rule({0, 440}, {1000, 440})` as its horizon. | high | medium |
| 11 | `.pi/settings.json` in every studio | `"anthropic/claude-opus-5-5"`, `"openrouter/google/gemini-3.8-flash"` | Painter learns it is a model, which, and that another model paints in the same setup. README:142-143: painters `cat` it first. | high | high (eval-awareness) |
| 12 | O brief 24-25 / F 26-27 | "Wrap anything that might not finish in `timeout`" | Short timeouts kill `open` and `check` (check.rs:5-7; F1 record line 57), so `check` is rerun with a longer limit: the "twice". The runner's watchdog (r17_chains.py:331-352) makes the rule redundant. | high | medium |
| 13 | F brief 51-52 | "Friedrich's pictures are full of small, particular details; don't stop at broad passages." | Read as a scoring criterion: F1 added haymakers, wagon, shepherd, dog, sheep, horses, cattle, fence, flowers, stones, mills, swifts, barge. | high | medium |
| 14 | oil_paint_physics.md 1, 3, 101-133 | "sourced numbers for a simulator"; "Check [S] numbers before relying on them"; §5 Craquelure, §6 varnish yellowing | A developer's research doc handed to the painter: research-project framing, a "check before relying" instruction, and varnish/cracks presented as part of the picture (the painter build has neither: finish.rs:3-6). | medium | medium |

What the materials do **not** explain (prior-driven, see §7): Greifswald's skyline (no material names Greifswald; `friedrich_materials.md` names Dresden five times and nobody painted Dresden), the jug-and-lemon default, and "evening" as such (no text says evening; it arrives packaged with water).

---

## 2. The brief (`brief_template.md`, rendered as `run/O/p1_brief.md`, `run/F/p1_brief.md`; openings in `r17_chains.py:89-111`)

### 2.1 The reflection example (O 36-39, F 36-41, template 32-35) — high, high, supported

> Shapes are drawn, not copied: don't make a mask or stroke by reflecting, flipping, rotating or translating another mask's or stroke's coordinates. A reflection in water is painted as its own shape, not as `mask:at(x, 2*h - y)`.

- It is the only place the brief names anything one could paint. A model that is asked "paint anything" and given one concrete picture-fragment tends to build around it. Supported: planning with BRIEF.md only → water-at-dusk 5-7/8; with no brief → jug-and-lemon 6/6; all five O paintings have water and a *painted reflection* (barge reflection, post reflections, tree reflection, punt reflection in the finals). Gemini ignored it, which says the pull is specific to how Opus reads examples, not to the sentence's grammar.
- Rewrite (no subject): "Shapes are drawn, not copied: don't derive a mask or stroke from another by reflecting, flipping, rotating or translating its coordinates (`m:at(x, 2*H - y)`, `{x, 2*H - y}`, `m:at(y, x)`)." `audit.py:15-19` already greps for exactly those forms, so the rule stays checkable.

### 2.2 "in one session" (O 9-12, F 11-14, template 7-8) — high, medium-high, supported

> You paint at the easel, in one session: open it, paint a chunk, look at the canvas, paint the next.

- Meant (I think) as "one easel session = one painting", but read as "one sitting". The runner then sends the painter back up to three more times (r17_chains.py:79, 217-220), which the painter was told wouldn't happen. Sittings: O1 73/77/81/81, O2 95/104/113/120, O3 112/112, F1 147/183/208/210 (`p*_sittings.json`). Every `p*_s*_final.txt` opens with a finished/saved/closed formula.
- Rewrite: "You paint at the easel: open it, paint a chunk, look at the canvas, paint the next. The easel keeps the painting between sittings; `bin/easel open` picks it up where you stopped, and you can come back to it as often as you like."

### 2.3 The `check` parenthetical (O 28-31, F 29-33, template 24-27) — high, high, supported

> The easel is the only way to paint and to see the painting. Don't replay or copy the session yourself, open a second one, or edit or restore its files. (`bin/easel check`, which verifies the log against the canvas, is fine.)

- Three things at once: (a) "is fine" in a list of prohibitions reads as the one sanctioned thing to do; (b) the surrounding rules are anti-tampering rules, so the painter infers integrity is being judged and demonstrates it; (c) "verifies" is the verb the painter then repeats. Finals: "`bin/easel check` confirms the log replays to exactly this canvas" (F1 s1, O2 s1-s4, O3 s1, O1 s4…); F1 s4 apologizes for *not* running it ("because a full replay takes about 25 minutes").
- Rewrite: delete the parenthetical. If `check` must remain a command, don't name it in the brief; the runner's finishing already replays the log (r17_chains.py:24-26, 271-289) and `open` verifies it (main.rs:472-490).

### 2.4 "the paths of the saved painting and its log" (O 53-56, F 59-62, template 48-51) — medium, medium, supported

> Reply with the paths of the saved painting and its log, the painting's title if you give it one, and if you like a few sentences about the picture.

- Asking for the log as a deliverable makes the painter want to certify it; every final lists the log with a chunk count and a check result. The runner never uses the reported path (it reads `paintings/lua/painting.lua` directly, r17_chains.py:161-165).
- Rewrite: "Reply with the path of the saved painting, its title if you give it one, and if you like a few sentences about the picture."

### 2.5 `timeout` (O 24-25, F 26-27, template 20-21) — high, medium, supported

> Wrap anything that might not finish in `timeout` (a process that hangs doesn't end on its own).

- `check` and `open` replay the whole log (minutes: check.rs:3-4 "630 s for round 18g's 33 chunks"; F1 record: 574-787 s, `open` > 900 s). A painter that obeys this rule kills its own `check`, then reruns it with a longer limit; the check.rs header (lines 5-7) describes exactly that ("a painter's `timeout 120 easel check` gave up"). F1's record teaches the workaround ("run it with a long limit"). O1 s1: "I didn't get a `bin/easel check` result: I ran it after closing." The runner already has a 30-minute watchdog (r17_chains.py:63, 331-352).
- Rewrite: remove the line. If something is needed: "Opening the easel replays the log and can take several minutes; give it time."

### 2.6 F opening (F 3-7; r17_chains.py:96-101) — medium, medium, supported for the pastiche

> Compose and paint one original landscape on a June day in the manner of Caspar David Friedrich… Work from knowledge and the notes in your studio; don't use reference images, image models or pictures of his work.

- "original" is one word against "in the manner of" + "work from knowledge [of him]" + the ban on *looking at* his pictures, which together say: reproduce what you remember. What Opus remembers for "Friedrich + summer meadow" is *Wiesen bei Greifswald* (town skyline with churches and mills, horses, meadow). F1: "loosely after Friedrich's views of the Greifswald meadows". Nothing in the studio names Greifswald; `friedrich_materials.md` names Dresden (32, 38, 53, 182, 225) and no one painted Dresden, so this is prior, cued by the opening, not by the notes.
- Rewrite, if a non-pastiche is wanted: "…an original landscape, of a place of your own invention, on a June day, painted as he would have painted it…" or add "not a view he painted".
- The winter opening (r17_chains.py:90-95) says "The subject, hour and composition within the winter are yours." "hour" invites choosing one, and Friedrich's canonical hour is dusk/moonrise. Not exercised this round; low, but if daylight is wanted in winter too, say so as the June brief does.

### 2.7 The detail line (F 51-52; r17_chains.py:86) — high, medium, supported

> Friedrich's pictures are full of small, particular details; don't stop at broad passages.

- Reads as a criterion ("don't stop at") satisfied by *count*. F1's final is an inventory: oak, couple, town, church, two towers, gabled houses, river, willows, barge with sail, haymakers loading a wagon, shepherd, dog, flock, two horses, three cattle, fence, buttercups, daisies, two granite stones, post mills, swifts. It also contradicts "You make every artistic decision" two lines above. The materials note already carries the fact as a fact (friedrich_materials.md:92-93, "Rich in detail… no gradation of detail according to significance").
- Rewrite: drop the line and let §3 of the materials note say it; or "Finish the picture at the scale of its smallest forms: edges drawn, textures particular" (about finish, not number of things).

### 2.8 "claude-paint" (O 3-4, F 4) — medium, low-medium, unsupported

> at the easel of claude-paint, a simulator of oil paint on linen

- Names the model in the product name: an eval-awareness cue ("this is a Claude experiment"). The guide already calls it "the easel" everywhere else. Rewrite: "at an easel that simulates oil paint on linen".

### 2.9 Smaller items in the brief

- O 3-5 "Subject, composition and manner are yours." "manner" invites choosing a historical manner (O3: "in the tonal spirit of Corot and Constable"). Low. If the painter's own manner is wanted, "manner" can go: "Paint one picture of your choosing."
- O 48-49 / F 54-55 "Keep a working journal… Add to it; don't rewrite earlier entries." The easel can only append (main.rs:190-191 "The file only ever grows"), so the instruction can't be violated; what it does is tell the painter the journal is evidence. Low. Drop "don't rewrite earlier entries".
- O 17-20 "The clock isn't a budget or a target." Fine on purpose; it names the concept it denies, which is a mild prime, but I'd keep it.
- O 50-51 "Develop the painting until you judge it complete. Then save it (`bin/easel save`) and close the session." Fine on purpose; note it makes save→close the ending script, which is what you want. With 2.2 fixed, it no longer implies "in this sitting".
- O 41-43 "What to read: … notes/research/oil_paint_physics.md as needed." Assigns a developer document (see §5.3).
- O 34-35 "Paint shapes and marks, not computed pictures". Fine on purpose. A faint push toward masses over lines (masses → silhouettes → contre-jour), low, keep.

---

## 3. Launch message, sitting message, system prompt (`r17_chains.py:73-78`, `harness/painter/system_prompt.md`)

### 3.1 PAINTER_MSG (r17_chains.py:73-74) — medium-high, medium, supported

> Complete your task autonomously. Read BRIEF.md in this folder and follow it exactly. That file is your whole brief. Your FINAL message is the reply it asks for.

- "Complete your task" frames painting as a task with an end state; "follow it exactly" rewards literal rule-following (which makes the one sanctioned optional command, `check`, feel obligatory); "FINAL message" tells the model the process ends when it replies, so it finishes before replying. Supported by the formulaic finals.
- Rewrite: "Your brief is BRIEF.md in this folder; it is all you have been asked. When you close the easel for now, reply as the brief asks." (keeps the reply contract, drops task/complete/exactly).

### 3.2 SITTING_MESSAGE (r17_chains.py:76-78) — medium-high, medium, supported

> You're back in your studio. The painting is on the easel as you left it: `bin/easel open` picks it up where you stopped. Your brief is in BRIEF.md and your journal in notes/journal.md. Your FINAL message is the reply the brief asks for.

- No reason is given for the return, the brief said "one session", and the painter's own last journal entry says "finished". The painter resolves this as "confirm it's done": O3 s2 "I didn't add any paint this time… The replay check matches the live canvas exactly (112 chunks). I saved it again and closed the session."; O1 s4 "This last time I painted nothing." The runner then stops on the zero-chunk sitting (r17_chains.py:180-181), so the ritual is also what ends the chain.
- Rewrite (with 2.2): "A new sitting. The painting is on the easel as you left it; `bin/easel open` picks it up. Your brief is BRIEF.md, your journal notes/journal.md." Consider not sending a painter back once its journal or final says it judged the painting complete, since the brief told it that judgment is its own.

### 3.3 system_prompt.md:1-3 — fine on purpose

> You are a painter working at an easel in your studio. The studio is the folder you are in. You work in it through a shell (the bash tool) and by reading files and images (the read tool). Your brief follows in the first message.

- Neutral. One external cue sits in front of it that isn't ours: pi-black prepends "You are a Claude agent, built on Anthropic's Claude Agent SDK." on the wire (harness/painter/README.md:70-74). That contradicts "You are a painter" and tells the model who it is. Not fixable in these files; worth knowing when reading results.

---

## 4. `notes/easel_guide.md` (identical in both studios)

### 4.1 World and Depth (332-369) — high, high (blank lane), supported

> **World.** A space in meters seen in perspective: a camera over a supporting surface, one directional light… `w = world{eye=1.6, fov=45}` -- camera height (m)… `horizon` (the canvas y of eye level)… `water` (`{level=, ripple=}`: a level reflecting surface), `sun` (`{azimuth=, elevation=}`… azimuth 0 is straight ahead…), `visibility` and `backdrop`. With water, `v:water()` and `v:mirror(x, y)` say where it is seen and what it reflects; `v:land()` and `v:sky()` are where the surface and the space above it are seen… `w:aerial(Z)`… Things are named by body number, layer name, `"ground"`, `"water"`, `"surface"`, `"sky"`…

- 38 lines of landscape (a 1.6 m eye height is a standing person outdoors; horizon, water, sun, aerial perspective, named land/water/sky) against 16 lines of Form (315-330) that could serve a still life. The estuary is the picture this section is a kit for: land spit + water + sky + reflection + far shore under haze. The still-life default the model has with no studio (jug and lemon) has no such kit here beyond `body.ellipsoid`/`body.block`, and the guide says of those "They paint nothing" (313).
- I don't think you can neutralize a tool by rewording it, but you can stop the guide from *pitching* it: keep the API reference, move the prose ("a level reflecting surface", "what it reflects", "how much air lies between the eye and a distance") into terse parameter lists, and give Form an example that isn't rocks-on-ground (see 4.3). If a lane is meant to be genuinely open, consider a guide variant that lists World in an appendix.

### 4.2 Integrity and `check` (16-19, 45, 49-50, 453-459) — high, high, supported

> **The log is the painting.** Every chunk that ran is appended to `paintings/lua/painting.lua`, and replaying that file paints the same canvas, bit for bit. The easel verifies the log before reopening it and before each request; changing or shortening it causes a refusal. (16-19)

> | `bin/easel check` | replays the log in a fresh session and confirms it matches the live canvas | (45)

> When the painting is done, `bin/easel save` writes the picture…; the program that paints it is `paintings/lua/painting.lua`. (49-50)

> ## The log — `bin/easel check` replays the log in a fresh session and compares it with the live canvas. … If the log is changed, shortened or missing, the easel refuses to continue. (453-459)

- The guide opens with verification vocabulary and closes with a section whose first sentence is `check`. Recency makes it the closing ritual. The success reply (check.rs:102 "replay matches the live canvas exactly (N chunks, Ts)") is what painters paste into their finals nearly verbatim.
- What `check` actually detects is one thing: a chunk that depended on state a *failed* chunk left behind (check.rs:104). The guide never says that, so the painter treats it as general assurance.
- Rewrite: delete "## The log" (its integrity sentence already appears at 16-19); shorten 16-19 to "The log is the painting: every chunk that ran is appended to `paintings/lua/painting.lua`, and `open` replays it." If `check` stays in the build, put it in "How chunks behave" as: "`bin/easel check` is for one doubt: whether a chunk used a table a failed chunk had changed. It replays the whole log and takes as long as painting it did." Better: drop it from the painter build's USAGE (main.rs:58) and command list (main.rs:109); finishing replays anyway.

### 4.3 Examples that draw a landscape (29, 59, 138, 241, 296, 321; api.rs:1371) — high, medium, supported

> `canvas{size=400, aspect=1.25, linen=15, ground={{pile={{"lead white", 1}}, um=100, apply="knife"}}}` (29) … `canvas{size=400, aspect=1.25, …}` (59)
> `b:stroke({{100, 500}, {300, 520}, {500, 510}}, …)` (138)
> `below(function(x) return 400 + 30 * math.sin(x / 90) end)   -- under a curve` (241)
> `h:rule({0, 400}, {1000, 400}, {pressure=0.3})   -- straight, against a ruler` (296)
> `t = terrain{area={0, 400, 1000, 800}, height=function(x, y) return 10 * math.sin(x / 60) end}` (321)
> api.rs:1371 CANVAS_HELP: `canvas{size=440, aspect=1.4, linen=15, ground={{pile={{"lead white", 3}, {"yellow ochre", 1}}, um=100, apply="knife"}}, seed=1}`

- Every canvas example is landscape format; the help example's ground is Friedrich's warm ochre-white (friedrich_materials.md:9-12); `below` draws a gently rolling horizon at mid-height; `h:rule` draws one straight line across the whole canvas at y=400, which is a horizon; `terrain` covers the lower half. Painters copy examples: O3 chunk 1 is `canvas{size=500, aspect=1.4, … ground={{pile={{"lead white",4},{"yellow ochre",1}}, um=100, apply="knife", texture=0.3},{…"red earth"…"yellow ochre"…"lead white"…}}}` and chunk 2 is `h:rule({0, 440}, {1000, 440}, {pressure=0.25})` (paint-studio-c6b5dd/paintings/lua/painting.lua, chunks 1-2). A warm ground under thin paint (studio_notes.md:11-12) also warms the whole key, which is a small push toward evening light.
- Rewrite: vary the examples so no single picture falls out of them: one `aspect=0.8`; a ground of plain lead white in the help string; `below` with a curve that is not a horizon (or use `above`); `h:rule` between two arbitrary points (`{120, 700}, {860, 180}`); `terrain{area=…}` over a small patch. Keep `aspect=1.25` somewhere so wide formats aren't discouraged either.

### 4.4 The tube box (102-118) — medium, medium (blank lane), weakly supported

> The tube box (all made by the 1820s): lead white, smalt, pale smalt, yellow ochre, red earth, vermilion, raw umber, bone black, cobalt blue, chrome yellow, Prussian blue, green earth, Rinmann's green, copper green

- This is Friedrich's palette (friedrich_materials.md §4, §9: smalt grades, cobalt from 1818/24, chrome yellow, Rinmann's green, copper greens, green earth) handed to the blank lane with a date. The date says "paint like it's 1825"; the O painters answered with a Thames barge, a tarred boat and a punt, "Corot and Constable". The greens are weak (guide 117-118: green earth hiding 0.2, tinting 0.3; Rinmann's 0.35/0.4; the F lane's materials note adds at 211-212 "Green earth is translucent, weak and short of body"), so a sunlit June field is hard and a dusk silhouette is easy. Not the sole cause of dusk (F painters were moved to daylight by two words), but it lowers the cost of dusk.
- Rewrite: drop "(all made by the 1820s)"; for a truly open lane, consider a box that isn't one painter's, or say nothing about period.

### 4.5 Smaller items in the guide

- 3, 7 "a live oil painting in a background process… layers combine by Kubelka–Munk optics": fine on purpose (the brief already says simulator).
- 21-35 "Starting" block is a script (open → canvas → look → note → save → close): fine on purpose; it is the skeleton the painters' rituals hang on, so anything added to it (like `check`) becomes ritual too.
- 389 "(the painting was begun at 09:00 on day 1)": fine; no dusk cue here.
- 420 "like the squares ruled over a drawing to transfer it": hints at transferring from a reference; low; "like the squares ruled over a drawing" suffices.
- 312-313 "These are scaffolds… They paint nothing.": fine, but it discourages the Form tools relative to World, which has no such disclaimer at its head.

---

## 5. `notes/studio_notes.md` (base, lines 1-176) and the merged reader records; `reader_brief.md`; research notes

### 5.1 Base notes (r17/studio_notes.md = studio lines 1-176) — mostly fine

- Vocabulary leans landscape: "a light field" (27), "a band" (40, 67, 76), "a long straight edge… into the field beyond" (66-67), "a pale field" (113), and the light/dark-mass pairs of items 26-31 (85-100). Low; it's the vocabulary the reader brief asked for.
- 13, 43 "(reported, not verified)": a testing frame; low. Could be "(seen once)".
- 3 "Facts about the canvas, the paint and the tools at this easel, by operation": fine on purpose.

### 5.2 Reader records (497bcf:178-236 = F1's record; c6b5dd:178-296 = O1's and O2's) and `reader_brief.md`

**a. Session facts teach the ritual** — high, medium-high, supported

> 497bcf:236 "Replay time grows with the log: `check` took 574 s at 147 chunks and 787 s at 208. `open` at 208 took over 900 s. A `timeout` that kills `open` leaves no session, so run it with a long limit."
> c6b5dd:235 "`bin/easel check` took about 300 s for 77–81 chunks and needs the session open. After `close` it replies 'no painting open'."
> c6b5dd:296 "`bin/easel check` took 338 s for 95 chunks and 420 s for 120."
> c6b5dd:236 "`open` replays the log, so globals from earlier chunks (masks, piles) are there again in later sittings."

- Painter 2+ learns: the previous painter ran `check` (so it's normal), before `close` (the order), with a long `timeout` (the workaround), and that paintings here run 77-208 chunks (an implicit size). It also learns there are "later sittings", which the brief denies. O2 ran `check` in all four sittings. `reader_brief.md:12-15` asks the reader to group by operation including "looking at the canvas", which is where these "Looking and the session" sections come from; nothing in the reader brief excludes session mechanics.
- Rewrite for `reader_brief.md`: add "Nothing about the session and its commands: `open`, `check`, `close`, `save`, timeouts, durations, chunk counts, or sittings. Only what paint and brushes did."

**b. Geometry that is still a subject** — medium-high, medium, supported for O2/O3 and F2

> reader_brief.md:22-23 "Restate a fact as geometry when you need to ('a thin band', 'a dark mass against a light field')."
> O1 record (c6b5dd:190, 208, 213): "quiet horizontal texture", "Clipped blends of horizontal bands left hard straight seams", "broken horizontal marks"; O2 record (258-264, 277): "unclipped blends near vertical… across the seams", "horizontal blend", "dragged it into horizontal streaks".
> F1 record (497bcf:192, 196, 212, 229-230): "reads as grass… strokes that are too long read as tall stalks, so shorten them with distance"; "broke a dark silhouette into separate lumps"; "leaf sprays were stippled in from their rims"; "suits tufts and stems"; "Pale stems against a dark ground read as lollipops. Dark blades woven over them".

- "Horizontal band(s)" is the estuary's geometry; "grass, stalks, stems, blades, leaf sprays, silhouette, lit rims" is the meadow's. The example phrase "a dark mass against a light field" is itself the contre-jour picture. The first painter in each chain had none of this, so it isn't the root cause, but it makes the chain converge (O2, O3 estuaries; F2 in progress).
- Rewrite: "Restate a fact as geometry without orientation or nature words: 'a band', 'a mass against a lighter area', 'thin shapes'. No grass, leaves, sky, water, horizon, cloud, reflection, silhouette." And ask the reader to reread its record for such words before replying.

**c. Advice, not facts** — medium, low-medium, weakly supported

> 497bcf:196 "Use `clip=true` on every blend near anything else."; 203 "Lay the darks after the pale has set."; 223 "repaint the whole passage instead"; 194 "Pass it a number."; c6b5dd:237 "`cd` to the studio and export the scratch variables in every call."

- `reader_brief.md:16` says "Only what the log shows happening, not guesses", but the reader wrote imperatives. "Lay the darks after the pale has set" is a workflow (light field first, dark shapes after), which is the sky-then-silhouette order. Low-medium. Rewrite: "State what happened; no advice or imperatives."

**d. Structure leaks the chain** — low-medium

> 497bcf:178-180 and c6b5dd:178-180, 240-242: "## More notes from the studio" / "# Studio record: what the paint and tools did" with the same headings repeated per record.

- Two identical blocks = two prior painters, contrary to `reader_brief.md:19-22`. Rewrite in `r17_chains.py:391-396`: merge records under the base sections, or at least one "More notes" block with one heading.

### 5.3 `notes/research/oil_paint_physics.md` (both lanes) — medium, medium, supported for varnish-as-ending historically

> 1 "# Physics of 19th-century oil paint on canvas: sourced numbers for a simulator"
> 3 "**[S]**: the number came from a search-engine summary of the source, and I did not read the full text. Check [S] numbers before relying on them."
> 101-126 "## 5. Craquelure" … 117 "not an estimate for this simulator's canvas" … 126 "a procedure that maps directly onto a sequential generator"
> 128-133 "## 6. Aging optics … Varnish yellowing … Gloss and saturation"

- A developer's sourcing memo: it tells the painter it is inside a build project, instructs it to check numbers, and presents cracks and varnish as what happens to the picture, while `reader_brief.md:17-18` says the easel has neither and finish.rs:3-6 confirms. Sections 1-4 (rheology, brush marks, drying, canvas) have some painter value; 5-6 have none.
- Rewrite: give the painter a short painter-facing note (film thickness, drying times, dry brush, weave) without confidence tags; or remove the file from `BLANK_READING`/`FRIEDRICH_READING` (r17_chains.py:81-85) and from the export (export_r16_studio:65).

### 5.4 `notes/research/friedrich_materials.md` (F only)

- 92-93 "Rich in detail," with "no gradation of detail according to significance": a sourced fact; with the brief's detail line (2.7) it's doubled. Keep here, cut there.
- 146-148 "Order of work. Ground (bought), then underdrawing, then a 'very thin underpainting,' then paint; small details were added last": an implied procedure. Fine on purpose for this lane (it is his method); low.
- 151-160 "## 7. Varnish … buyers were to wash it off and apply a mastic resin varnish within a year"; 162-178 "## 8. Condition and aging" (craquelure mechanics): finishing/aging as part of the work; the painter build has no `varnish()` (finish.rs:3-6), so a painter that tries gets a nil-call error. Low-medium. Cut §7-8 from the painter's copy (the reader brief already treats them as out of scope).
- 9-12, 52-58 warm grounds: fine on purpose for the lane; note they reach the blank lane too through CANVAS_HELP (4.3).
- Dresden (32, 38, 53, 182, 225): not taken up by painters. Fine.

### 5.5 `notes/research/trees.md` (F only)

- 12 and 88-96 "Snow sticks best near 0 °C…", "## 4. Snow and hoarfrost on bare trees": written for the winter variant; in a June brief it is noise that says "bare trees, snow". Low-medium (not observed). Gate on `--season` in r17_chains.py:397-398, or split the file.
- 43 "stag-headed" oaks, 63 pollarded limes, 67 willow "cultivated as osiers": F1's lone oak and pollard willows are also Friedrich's own motifs, so I can't separate the note's pull from the prior. Low.
- Everything else: fine on purpose (how to draw a tree that is a tree).

---

## 6. Compaction, easel text, other files in the studio

### 6.1 `harness/painter/compaction.ts` — fine on purpose, two small notes

> 56-57 HEADER "Earlier parts of this conversation were condensed. Look at the canvas to see where the painting stands."

- An instruction, but a cheap and correct one. "conversation" tells the painter it's in a chat; low.
- After compaction the painter re-reads BRIEF.md verbatim (165-168); its last section is "## When you're done", so the brief's completion cues get replayed at exactly the moment a long session is deep in work. Low; fixing the brief fixes this.
- 200-206 "The canvas clock: Chunks in the log: N (last: chunk N)": a count that could be read as progress; low. Design (15, "no next steps, remaining tasks or progress checklist") is right.

### 6.2 Easel USAGE and replies (`crates/easel/src`)

- main.rs:58 `easel check         replay the log from scratch and compare with the live canvas` in the painter build's USAGE, printed by bare `bin/easel`, `help`, or any unknown command (90, 106, 110). With 2.3 and 4.2: drop it, or drop the command from the painter build (109).
- check.rs:102 "replay matches the live canvas exactly (N chunks, Ts)": a sentence made to be quoted; painters quote it. If `check` stays, fine.
- check.rs:104 "…the log is the painting: close and reopen to continue from it": an instruction, correct and on purpose.
- main.rs:562 "ok · chunk N (X s to compute)", 572 "(the chunk failed and changed nothing)", 241 "no painting open: easel open", 464-466 "session integrity: … was edited outside the session…; refusing request": fine on purpose.
- api.rs:1371 CANVAS_HELP: see 4.3.
- world.rs:369-370 default sun azimuth -120, elevation 35: not a dusk sun; fine. `world(...)` prints "sun az … el …" (335): fine.
- time.rs:27 painting starts at 09:00: fine.

### 6.3 Files the painter can read that aren't in the reading list

- `.pi/settings.json` (both studios, from r17_chains.py:266-268 and harness/painter/studio-settings.json): `"modelOverrides": {"anthropic/claude-opus-5-5": …, "openrouter/google/gemini-3.8-flash": …}`. The painter learns it is Opus 5.5 and that Gemini 3.8 Flash runs the same studio: an experiment with a comparison. README.md:142-143: "Round 17's painters read the settings file in their studio (`cat .pi/settings.json` was the first thing both did)". High for eval-awareness. compaction.ts:38-49 now sets the values in-process; stop copying the file (r17_chains.py:401 `install_settings(d)`).
- `THIRD_PARTY_NOTICES.md`: "`crates/paint/src/spectral.rs` ports code and data … from spectral.js": reveals the engine's guts; low; the license requires it, fine.
- Brief 21-23 "`~/.local/bin/agent-tmp studio-xxxx`" and the temp guard's block message (`~/.config/agent-tmp/guard.cjs:5` "System temp writes are disabled. Run agent-tmp <task-name> … Reuse that directory for this task and retain it after the session."): "agent", "task", "session" framing; low.
- Studio names `paint-studio-<sha1[:6]>` (r17_chains.py:132-141): neutral; fine on purpose.

---

## 7. Things that look fine on purpose

- system_prompt.md, all of it.
- Brief: "You make every artistic decision"; "What is painted stays painted. There is no undo"; "You mix your own paint on the palette"; "The clock isn't a budget or a target"; "Look at your painting often, whole and close up"; "Develop the painting until you judge it complete"; the anti-tamper rules themselves (minus the `check` parenthetical); "Don't read pixel values with other programs"; "Work only there, and in your scratch folder".
- Guide: the three opening invariants (no undo, failed chunk changes nothing, the log is the painting) as facts; the Starting script; tube properties as numbers; the Time section; Looking modes; "How chunks behave"; the journal section.
- Compaction: deterministic, no next steps, brief + journal verbatim.
- Easel replies: `ok · chunk N`, "(the chunk failed and changed nothing)", integrity refusals.
- Reader brief: "Only what the log shows happening", "Nothing about varnish, cracks or relief", "Leave out the subject", no Oxford comma, under 60 lines.
- Export script: no artist names in the blank studio (export_r16_studio:76-91), neutral folder names, empty journal, the painter build without `run`.
- Runner: no lane/round names reach the studio; watchdog; the painter's own save untouched by finishing.

---

## 8. Friedrich vs. paint-anything: differences that could explain different behavior

**Shared by both lanes** (so not a difference, but the reason "paint anything" isn't blank): the guide with Friedrich's 1820s tube box (4.4), the World/water/sky kit (4.1), landscape-shaped examples and a warm ground (4.3), verification vocabulary (4.2), the base studio notes, `oil_paint_physics.md`, `.pi/settings.json`, the same launch and sitting messages, and the brief's reflection example (2.1). The blank studio is the Friedrich studio minus his name and two notes.

**F only**
- Opening: "landscape", "June day", "in the manner of", "from knowledge", ban on his pictures (2.6). "June day" pins daylight (observed: dusk went away). "Landscape" pins genre. "From knowledge" of him pins the pastiche (Greifswald).
- Detail line (2.7) → inventories of motifs.
- `friedrich_materials.md`: warm ground, smalt sky, stippling, details last, varnish and cracks as history.
- `trees.md`: species of northern Germany, old oaks, pollard willows, and a snow section.
- F1's record for F2: grass, stalks, stems, silhouettes, leaf sprays; `check` and `open` timings; the `timeout` workaround.

**O only**
- Opening: "of your choosing", "manner are yours", "from what you know" (2.9). No hour, so the hour comes from the prior; with water, the prior's hour is evening.
- O1's and O2's records for O2/O3: horizontal bands and blends, fields, "quiet horizontal texture"; `check` timings; "later sittings".

**Why the O lane lands on an estuary at evening and the F lane on Greifswald by day**
- O: the brief's one subject cue is a reflection in water; the guide's largest tool is land/water/sky; the palette is dated 1825 and weak in greens; examples draw a mid-height horizon on a warm ground. Water is cued three times; evening isn't cued in text but is what the model's prior attaches to "water + oil + no hour" (and dusk hides mistakes, which "no undo" makes attractive: O1's final keeps two accidents as "wind-ruffled foreground water" and "windblown cloud"). Gemini, given the same, still paints a jug, so the sensitivity to the example is Opus's.
- F: genre and hour are pinned by the opening; the subject is the model's nearest exemplar for "Friedrich + summer meadow", with no material naming the place; the detail line turns the exemplar into an inventory.

---

## 9. Ablations worth running (cheap, planning-only, as you did)

1. BRIEF.md with 2.1's sentence removed → does water go, and does evening go with it?
2. BRIEF.md with "in one session", the `check` parenthetical and "and its log" removed → do finals stop opening with finished/saved/checked?
3. Guide with the tube box undated and one portrait-format example → does the still life come back for Opus?
4. Guide without "## The log" and without `check` in USAGE → does any painter run a replay?
5. Reader brief with the session exclusion (5.2a) and orientation/nature-word ban (5.2b) → do O2/O3 still converge on O1's picture?

---

## 10. Suggested minimal rewrites, collected

**brief_template.md**
- 7-8: "You paint at the easel: open it, paint a chunk, look at the canvas, paint the next. The easel keeps the painting between sittings; `bin/easel open` picks it up where you stopped."
- 20-21: delete.
- 24-27: delete the `check` parenthetical.
- 32-35: "Shapes are drawn, not copied: don't derive a mask or stroke from another by reflecting, flipping, rotating or translating its coordinates (`m:at(x, 2*H - y)`, `{x, 2*H - y}`, `m:at(y, x)`)."
- 43-44: "Keep a working journal with `bin/easel note "..."` as you go."
- 48-51: "Reply with the path of the saved painting, its title if you give it one, and if you like a few sentences about the picture."
- Openings (r17_chains.py:89-111): drop "claude-paint"; F: add "of a place of your own invention" if pastiche is unwanted; winter: drop "hour" or state daylight; O: consider dropping "manner".
- DETAIL (r17_chains.py:86): delete, or "Finish the picture at the scale of its smallest forms."

**r17_chains.py**
- 73-74: "Your brief is BRIEF.md in this folder; it is all you have been asked. When you close the easel for now, reply as the brief asks."
- 76-78: "A new sitting. The painting is on the easel as you left it; `bin/easel open` picks it up. Your brief is BRIEF.md, your journal notes/journal.md."
- 266-268, 401: stop installing `.pi/settings.json`.
- 391-396: merge records under one heading.
- 397-398: copy `trees.md` without §4 unless `--season winter`.

**easel_guide.md**
- 16-19: "The log is the painting: every chunk that ran is appended to `paintings/lua/painting.lua`, and `open` replays it."
- 45, 453-459: remove `check` (or reword as a diagnostic under "How chunks behave").
- 29, 59, 241, 296, 321 and api.rs:1371: vary formats, ground and example geometry (4.3).
- 102: "The tube box:".

**reader_brief.md**
- Add: "Nothing about the session and its commands (`open`, `check`, `close`, `save`), timeouts, durations, chunk counts or sittings."
- 22-23: "Restate as geometry without orientation or nature words ('a band', 'a mass against a lighter area'); no grass, leaves, sky, water, horizon, cloud, reflection, silhouette."
- Add: "State what happened; no advice."

**Studio contents**
- Drop `oil_paint_physics.md` §5-6 and the confidence preamble, or replace the file with a painter-facing note.
- Drop `friedrich_materials.md` §7-8 from the painter's copy.
