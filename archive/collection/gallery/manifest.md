# Gallery inventory: claude-paint paintings

52 paintings from 52 painter sessions. The machine-readable version is `manifest.json`; the images are in `finals/<id>.png`.
Times are UTC from the session logs. "Delivered" means the painter's own final render, or a render of its unchanged program on its own engine; "fallback" means the painter's last whole-canvas look in its session log.

**Gap:** the painters' own words for rounds 1–4 (12 paintings) were not collected. The helper assigned to extract them was blocked by the model provider's Terms of Service filter, and it wasn't retried. Their notes are listed per painting.

**Reception:** quotes marked **Alice (own words)** are her words as quoted in notes/round6/alice_review.md. Quotes marked *paraphrase* are the integrator's summary of what she said.

## Round 1
Three AI painters that had never seen a painting from this project each composed an original picture in Friedrich's manner (winter, coast, mountains) as one Rust program. (notes/fresh_painters.md:3-7)

### r01-winter: (untitled)
- Clip: r02-fresh-1.mp4 · date 2026-09-22 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 57 min (log 2026-09-22T21:59:39Z to 2026-09-22T22:56:28Z, 57 min); full log time counted; the log runs to a power loss (resume messages at 22:53 and 22:55 UTC); the session stopped for good at 22:56 before a final render or report (notes/fresh_painters.md:5-7)
- Title source: None
- Subject: A winter cross with a dead oak, spruces, a figure seen from behind and a church in fog (notes/fresh_painters.md:11-12).
- Final: **fallback**, 1000×690 px, from `session log ~/.pi/agent/sessions/--Users-alice-src-a-claude-paint-fresh--/2026-09-22T21-59-38-550Z_8a4016b9-95c0303e-0b84b628-45e2.jsonl, last whole-canvas look at 2026-09-22T22:32:38.832Z` (the painter's last whole-canvas look (1000 px preview it viewed); a power loss cut the session off before its final report)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r02-fresh-1/018.jpg`: mean difference 0.41/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: none (power loss before its report); notes/fresh_painters.md summarizes
- Reception, project notes (user review plus the painters' reasoning): "Winter best." (of the three round 1 pictures), notes/fresh_painters.md:12

### r01-coast: (untitled)
- Clip: r02-fresh-2.mp4 · date 2026-09-22 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 57 min (log 2026-09-22T21:59:39Z to 2026-09-22T22:56:27Z, 57 min); full log time counted; the log runs to a power loss (resume messages at 22:53 and 22:55 UTC); the session stopped for good at 22:56 before a final render or report (notes/fresh_painters.md:5-7)
- Title source: None
- Subject: A dusk shore with a brig, a figure and an anchor (notes/fresh_painters.md:11).
- Final: **fallback**, 1000×714 px, from `session log ~/.pi/agent/sessions/--Users-alice-src-a-claude-paint-fresh--/2026-09-22T21-59-38-576Z_ad7233c6-6e12f10d-4ec9b11b-beb0.jsonl, last whole-canvas look at 2026-09-22T22:55:32.262Z` (the painter's last whole-canvas look (1000 px preview it viewed); a power loss cut the session off before its final report)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r02-fresh-2/015.jpg`: mean difference 0.4/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: none (power loss before its report); notes/fresh_painters.md summarizes

### r01-mountains: (untitled)
- Clip: r02-fresh-3.mp4 · date 2026-09-22 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 57 min (log 2026-09-22T21:59:39Z to 2026-09-22T22:56:27Z, 57 min); full log time counted; the log runs to a power loss (resume messages at 22:53 and 22:55 UTC); the session stopped for good at 22:56 before a final render or report (notes/fresh_painters.md:5-7)
- Title source: None
- Subject: A cross on a rock with firs and a wanderer (notes/fresh_painters.md:10-11).
- Final: **fallback**, 1000×625 px, from `session log ~/.pi/agent/sessions/--Users-alice-src-a-claude-paint-fresh--/2026-09-22T21-59-38-610Z_97b9c6cf-717ee6b0-9275b4e9-4499.jsonl, last whole-canvas look at 2026-09-22T22:54:16.674Z` (the painter's last whole-canvas look (1000 px preview it viewed); a power loss cut the session off before its final report)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r02-fresh-3/018.jpg`: mean difference 0.44/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: none (power loss before its report); notes/fresh_painters.md summarizes

## Round 2
The same three themes again with a short brief, a better engine and no access to round 1; each painter wrote one Rust program. (notes/amnesia2.md:3-11)

### r02-winter: Winter Evening with a Ruined Choir
- Clip: r03-amnesia-winter.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 54 min (log 2026-09-23T01:26:08Z to 2026-09-23T02:20:27Z, 54 min)
- Title source: notes/amnesia2/fresh2_winter.md:1; notes/amnesia2.md:17
- Subject: Dusk: a Gothic ruin in mist, a stag-headed oak with snow on its limbs, spruces, a broken fence, a frozen brook and a walker seen from behind.
- Final: **delivered**, 3200×2310 px, from `~/tmp/paint-r6-b943b1ca/r2winter/out/fresh2_winter_full.png (copy: ~/src/a/claude-paint/notes/round7/winter_port/original_3200.png)` (3200 px render built on 2026-09-24 from the painter's own branch (amnesia-winter) and engine, i.e. the painting as it was; the same program re-rendered at 1000 px there is byte-identical to the original (notes/round7/winter_port.md:68, 145-146))
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r03-amnesia-winter/022.jpg`: mean difference 1.04/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia2/fresh2_winter.md
- Reception, **Alice (own words)**: "Still flawed" (round 7 comparison, where round 2's winter was judged still the best), notes/round6/alice_review.md:98
- Reception, **Alice (own words)**: "a better tree, better composition and so on", notes/round6/alice_review.md:98
- Reception, **Alice (own words)**: "good entropy, perceived", notes/round6/alice_review.md:99
- Reception, **Alice (own words)**: "The thing about round two is that it almost moved something in me, especially the winter one.", notes/round6/alice_review.md:157
- Reception, blind AI critics: "All three put round 2 first" (round 11 cross-critique: Gemini, Astra and Fable each ranked it first among six winters), notes/round11/blind/key.md:15

### r02-coast: Morning on the Shore at Arkona
- Clip: r03-amnesia-coast.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 57 min (log 2026-09-23T01:26:08Z to 2026-09-23T02:23:04Z, 57 min)
- Title source: notes/amnesia2/fresh2_coast.md:1; notes/amnesia2.md:18
- Subject: Before sunrise: fishermen's poles with a drying net, an erratic boulder, a woman at the water's edge, a brig on the horizon.
- Final: **delivered**, 1000×714 px, from `~/src/a/claude-paint/notes/amnesia2/fresh2_coast.jpg` (the painter's 1000 px preview as archived in the notes (JPEG); its 3200 render is not on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r03-amnesia-coast/023.jpg`: mean difference 0.47/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia2/fresh2_coast.md

### r02-mountains: Daybreak in the Riesengebirge
- Clip: r03-amnesia-mountains.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 55 min (log 2026-09-23T01:26:08Z to 2026-09-23T02:21:08Z, 55 min)
- Title source: notes/amnesia2/fresh2_mountains.md:1; notes/amnesia2.md:19
- Subject: A wanderer by a granite tor above a sea of fog, the Schneekoppe with its chapel, wind-thinned spruces.
- Final: **delivered**, 1000×690 px, from `~/src/a/claude-paint/notes/amnesia2/fresh2_mountains.jpg` (the painter's 1000 px preview as archived in the notes (JPEG); its 3200 render is not on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r03-amnesia-mountains/011.jpg`: mean difference 1.93/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia2/fresh2_mountains.md

## Round 3
The first paintings made live at an interactive easel, mark by mark, looking and undoing as a painter would; one free subject and two steered ones. (notes/amnesia3.md:3-19)

### r03-free: Dolmen on the Baltic Shore at Evening
- Clip: easel3-free.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Lua easel
- Time at the easel: 45 min (log 2026-09-23T12:12:53Z to 2026-09-23T12:58:04Z, 45 min)
- Title source: notes/amnesia3/easel3_free.md:1; notes/amnesia3.md:14
- Subject: A megalithic grave on a dune after sunset, two bare oaks, a small figure seen from behind, two sails, a crescent moon and the evening star.
- Final: **delivered**, 1000×714 px, from `~/src/a/claude-paint/notes/amnesia3/easel3_free.jpg` (the painter's 1000 px preview archived in the notes (JPEG); the only render from that day on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/easel3-free/029.jpg`: mean difference 2.53/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia3/easel3_free.md

### r03-green: The Oak on the Common, Summer Morning
- Clip: easel3-green.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Lua easel
- Time at the easel: 52 min (log 2026-09-23T12:12:53Z to 2026-09-23T13:05:19Z, 52 min)
- Title source: notes/amnesia3/easel3_green.md:1; notes/amnesia3.md:15
- Subject: A lone oak in leaf with a dead snag, a shepherd and five sheep, a track, a far church, a pale range, cumulus.
- Final: **delivered**, 1000×714 px, from `~/src/a/claude-paint/notes/amnesia3/easel3_green.jpg` (the painter's 1000 px preview archived in the notes (JPEG); the only render from that day on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/easel3-green/021.jpg`: mean difference 0.53/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia3/easel3_green.md

### r03-near: (untitled)
- Clip: easel3-near.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Lua easel
- Time at the easel: 61 min (log 2026-09-23T12:12:53Z to 2026-09-23T13:13:51Z, 61 min)
- Title source: notes/amnesia3/easel3_near.md:1 has a description, not a title
- Subject: A bedded sandstone block with a young birch growing from a crack, moss and heather on its ledges, rust bracken, dark spruces, crows.
- Final: **delivered**, 1000×769 px, from `~/src/a/claude-paint/notes/amnesia3/easel3_near.jpg` (the painter's 1000 px preview archived in the notes (JPEG); the only render from that day on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/easel3-near/032.jpg`: mean difference 0.61/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia3/easel3_near.md

## Round 4
The easel again, now with drawing tools (pencil underdrawing, hand-drawn outlines, depth masks), same three assignments as round 3. (notes/round4_plan.md:3-6, 29)

### r04-free: Evening on the Heath: Dead Oak and Hünengrab
- Clip: easel4-free.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Lua easel
- Time at the easel: 52 min (log 2026-09-23T15:50:25Z to 2026-09-23T17:43:11Z, 113 min); 113 min in the log minus a 61-minute pause at a usage limit (session log, resume message at 17:32 UTC)
- Title source: notes/amnesia4/easel4_free.md:1
- Subject: A dead oak and a Hünengrab (megalithic grave) on a heath at evening (from the title).
- Final: **delivered**, 3200×2207 px, from `git origin/easel4-free:out/easel4_free_full.png` (3200 px render committed on the painter's branch easel4-free; it is later than the clip's last frame (the last look in the session): the committed render has the finished foreground)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/easel4-free/019.jpg`: mean difference 4.4/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia4/easel4_free.md

### r04-green: Summer Morning above the River Valley
- Clip: easel4-green.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Lua easel
- Time at the easel: 20 min (log 2026-09-23T15:50:25Z to 2026-09-23T16:10:45Z, 20 min)
- Title source: notes/amnesia4/easel4_green.md:1
- Subject: A summer morning above a river valley (from the title; the painter's notes describe it further).
- Final: **delivered**, 1000×714 px, from `~/src/a/claude-paint/notes/amnesia4/easel4_green.jpg` (the painter's 1000 px preview archived in the notes (JPEG); the only render from that day on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/easel4-green/021.jpg`: mean difference 0.98/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia4/easel4_green.md

### r04-near: (untitled)
- Clip: easel4-near.mp4 · date 2026-09-23 · claude-opus-5-5 (anthropic), thinking high · Lua easel
- Time at the easel: 24 min (log 2026-09-23T15:50:25Z to 2026-09-23T16:14:23Z, 24 min)
- Title source: notes/amnesia4/easel4_near.md:1 has a description ("an erratic boulder at the edge of a spruce wood, first snow"), not a title
- Subject: An erratic boulder at the edge of a spruce wood in first snow.
- Final: **delivered**, 1000×769 px, from `~/src/a/claude-paint/notes/amnesia4/easel4_near.jpg` (the painter's 1000 px preview archived in the notes (JPEG); the only render from that day on this machine)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/easel4-near/010.jpg`: mean difference 0.84/255 at 400 px, same picture by eye
- Painter's words: not collected. Notes: notes/amnesia4/easel4_near.md

## Round 7
Three ways of working compared on a free subject: the easel without ready-made motifs, one Rust program, the easel with everything; plus a control painted in plain Python with no paint simulation. (notes/round7/arms/key.md; notes/round7/compare/key.md)

### r07-1: The Old Willow at Evening
- Clip: r07-arm1.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Lua easel
- Time at the easel: 46 min (log 2026-09-24T22:18:23Z to 2026-09-24T23:04:29Z, 46 min)
- Title source: r7-arm1:notes/round7/arm1/notes.md:1
- Subject: A flooded meadow in late autumn after sunset, with one old pollard willow on a low dyke on the axis, still water mirroring a banded sky and a thin far shore of woods and small pollards; no figure and no moon.
- Final: **delivered**, 3200×2286 px, from `git r7-arm1:notes/round7/arm1/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r07-arm1/026.jpg`: mean difference 0.67/255 at 400 px, same picture by eye
- Painter's words: "I'll plan the picture: a dusk in late autumn over flooded marshland, one old pollard willow on the axis on a dyke, the glow behind it. … The sky at 3200 is stippled, with faint warm patches where the ground shows through; that reads as dusk warmth." (session log 2026-09-24T22-18-22-766Z_a03452ff-34fbd933-0c408931-4cd4.jsonl @ 2026-09-24T22:20:45.189Z; session log 2026-09-24T22-18-22-766Z_a03452ff-34fbd933-0c408931-4cd4.jsonl @ 2026-09-24T22:58:32.901Z)
- Reception, **Alice (own words)**: "Overall fits with a good painting." (blind, as painting A), notes/round6/alice_review.md:144
- Reception, **Alice (own words)**: "like the brush you use for shaving" (on the spiky willow crown), notes/round6/alice_review.md:138

### r07-2: Two Poplars at a Pond, Evening
- Clip: r07-arm2.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Rust program
- Time at the easel: 34 min (log 2026-09-24T22:18:23Z to 2026-09-24T22:51:59Z, 34 min)
- Title source: r7-arm2:notes/round7/arm2/notes.md:1 (also paintings/src/bin/pond_poplars.rs:1)
- Subject: Dusk after sunset over a still pond: two Lombardy poplars on a low far bank near the axis (one whole, one with its top broken off), a young moon high on the left, thin evening clouds and a dark reedy near shore; no figure.
- Final: **delivered**, 3200×2286 px, from `git r7-arm2:notes/round7/arm2/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r07-arm2/014.jpg`: mean difference 2.64/255 at 400 px, same picture by eye
- Painter's words: "The order: a level horizon, most of the canvas given to the air; on the axis two Lombardy poplars on a low far bank, one whole, one with its top broken off; the still pond repeats them and the sky; a near shore of reeds that rises a little to both corners (a curve opening upward) and stops the eye. The sun has gone down behind the trees; a young moon, veiled." (r7-arm2:paintings/src/bin/pond_poplars.rs:4-8)
- Reception, **Alice (own words)**: "In many ways better than anything before." (blind, as painting B), notes/round6/alice_review.md:146

### r07-3: Evening on the Bodden
- Clip: r07-arm3.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Lua easel
- Time at the easel: 29 min (log 2026-09-24T22:18:23Z to 2026-09-24T22:47:46Z, 29 min)
- Title source: r7-arm3:notes/round7/arm3/notes.md:1 (also final reply, session log 2026-09-24T22-18-22-809Z_e4603d21-4a6d6fa5-a946960e-fccb.jsonl @ 2026-09-24T22:47:46.581Z)
- Subject: A calm lagoon after sunset, with a small town and Gothic tower on the axis of the far shore, windmills on either side, a glowing sky mirrored in the water and a dark grassy bank with reeds in front; no figure.
- Final: **delivered**, 3200×2207 px, from `git r7-arm3:notes/round7/arm3/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r07-arm3/022.jpg`: mean difference 2.43/255 at 400 px, same picture by eye
- Painter's words: "the far shore and the town against the glow: dark and cool at the top, lost into the mist on the water … the town mirrored in the calm water: same width, a little shorter, softer and lighter than itself" (r7-arm3:paintings/lua/bodden.lua:168; r7-arm3:paintings/lua/bodden.lua:288)
- Reception, **Alice (own words)**: "Weirdly, C has the best vibe somehow... there's something about C that feels different." (blind, as painting C), notes/round6/alice_review.md:154
- Reception, blind AI critics (Gemini, Astra): "C strongest", notes/HANDOFF.md:94

### r07-python: The Chapel Gable at Evening
- Clip: r07-python.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Python program (no paint engine)
- Time at the easel: 24 min (log 2026-09-24T19:44:54Z to 2026-09-24T20:09:15Z, 24 min)
- Title source: r7-python:experiments/python_friedrich/NOTES.md:1 (also paint.py docstring line 1)
- Subject: A ruined chapel gable with a pointed arch and a round window stands on the axis on a bare ledge above a calm sea at dusk, a man in a dark coat standing inside the arch with his back to us, a veiled crescent moon on the right.
- Final: **delivered**, 3200×2304 px, from `git r7-python:experiments/python_friedrich/out_3200.png` (3200 px render committed by the painter)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r07-python/014.jpg`: mean difference 0.59/255 at 400 px, same picture by eye
- Painter's words: "A man in a dark blue-green coat and a beret stands still inside the arch with his back to us. His head sits on the ruled horizon, against the brightest band of light. The sun has set." (r7-python:experiments/python_friedrich/NOTES.md:17-20)
- Reception, **Alice (own words)**: "very simplistic, very digital tells, the sky is very gradient-like.", notes/round6/alice_review.md:100

## Round 8
A patron-style commission, 'a winter landscape', with round 2's style of brief, in the same three ways of working. (notes/round8/blind/README.md:1-4)

### r08-1: Winter Evening by a Frozen Pond, with a Wayside Cross
- Clip: r08-arm1.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Lua easel
- Time at the easel: 25 min (log 2026-09-24T23:28:52Z to 2026-09-24T23:54:12Z, 25 min)
- Title source: r8-arm1:notes/round8/arm1/notes.md:7
- Subject: A tall clear evening sky with a thin new moon over a frozen pond, a far bank with pollard willows, a bare oak and a church spire in the haze, and in front a woman in a dark cloak with her back to us beside a leaning wooden cross in the snow.
- Final: **delivered**, 3200×2286 px, from `git r8-arm1:notes/round8/arm1/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r08-arm1/014.jpg`: mean difference 0.64/255 at 400 px, same picture by eye
- Painter's words: "The distance: a low, flat line of hills and a village church spire half lost in the evening haze, so that the eye ends at a building of faith (Friedrich's habit of the church seen far off: a far goal). … Symmetry and stillness, a picture of evening, winter and waiting." (r8-arm1:notes/round8/arm1/notes.md:14-16; r8-arm1:notes/round8/arm1/notes.md:25)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "again the "computational light simulation" effect" (blind, as X), notes/round6/alice_review.md:176
- Reception, blind AI critic (Astra): "X moves me most" (ranked first of four, above round 2's winter), notes/round8/blind/critics/astra.md:1

### r08-2: Winter Evening, the Way to the Ruined Choir
- Clip: r08-arm2.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Rust program
- Time at the easel: 45 min (log 2026-09-24T23:28:52Z to 2026-09-25T00:13:54Z, 45 min)
- Title source: r8-arm2:notes/round8/arm2/notes.md:3 (also winter_ruin.rs:1)
- Subject: A snowed-over field at the end of a winter afternoon, with the ruined gable of a Gothic choir in the mist on the left, spruces on the right, a bare oak and a snow-capped boulder in front, and one traveler walking away from us toward the ruin.
- Final: **delivered**, 3200×2286 px, from `git r8-arm2:notes/round8/arm2/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r08-arm2/018.jpg`: mean difference 0.67/255 at 400 px, same picture by eye
- Painter's words: "No road: one small traveler in a dark coat with a stick walks away from us toward the ruin through fresh snow, breaking the trail; his footprints and a shallow trough run back toward us. … the ruined Gothic church (Eldena, the Oybin choir) as the image of the old faith; spruces as the evergreen hope rising out of snow and mist; the dead or bare oak as death/the pagan past" (r8-arm2:notes/round8/arm2/notes.md:20-22; r8-arm2:notes/round8/arm2/notes.md:25-27)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "The tree is good, the rock is good." (blind, as Z), notes/round6/alice_review.md:174

### r08-3: Winter Morning by a Frozen Pond
- Clip: r08-arm3.mp4 · date 2026-09-24 · claude-opus-5-5 (anthropic), thinking medium · Lua easel
- Time at the easel: 28 min (log 2026-09-24T23:28:52Z to 2026-09-24T23:56:40Z, 28 min)
- Title source: r8-arm3:notes/round8/arm3/notes.md:5
- Subject: A winter dawn under a tall violet-to-straw sky: a snow bank with a dead oak and young firs on the left, a frozen pond, a misty fir wood and a village spire on the far shore, and one small figure walking a trodden path toward the church.
- Final: **delivered**, 3200×2286 px, from `git r8-arm3:notes/round8/arm3/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r08-arm3/019.jpg`: mean difference 2.51/255 at 400 px, same picture by eye
- Painter's words: "Across the middle distance lies a frozen pond, its ice a pale mirror of the sky; beyond it a dark band of firs and, faint in the morning mist, the spire of a village church (the church as the far goal is a Friedrich constant). … One small dark figure walks away from us along a trodden path toward the church: a Rückenfigur, small enough that the landscape stays the subject." (r8-arm3:notes/round8/arm3/notes.md:13-16; r8-arm3:notes/round8/arm3/notes.md:20-22)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "Doesn't move me, but the right direction visually." (blind, as W), notes/round6/alice_review.md:168

## Round 9
Round 8 repeated with two added lines: paint skies in broad strokes blended wet (no stippled veil), and let neighboring paint bury the feet of things. (notes/round9/README.md:16-22)

### r09-1: Winter Evening with Pollard Willows
- Clip: r09-arm1.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking medium · Lua easel
- Time at the easel: 58 min (log 2026-09-25T00:18:17Z to 2026-09-25T01:16:16Z, 58 min)
- Title source: r9-arm1:notes/round9/arm1/notes.md:7 (also final reply, session log 2026-09-25T00-18-17-266Z_494f5502-c6622cbb-38a84d6f-3ff7.jsonl @ 2026-09-25T01:16:16.138Z)
- Subject: A flat snowy lowland just after sunset: a frozen brook winds past a row of pollard willows shrinking toward a far village and its church spire, a man walks toward the church and a thin crescent moon hangs over the afterglow.
- Final: **delivered**, 3200×2286 px, from `git r9-arm1:notes/round9/arm1/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r09-arm1/023.jpg`: mean difference 0.67/255 at 400 px, same picture by eye
- Painter's words: "A frozen brook winds out of the lower left foreground and back into the plain; a row of pollard willows stands along its bank, diminishing toward a far village whose church spire is the only vertical on the horizon. A lone walker in a dark coat goes along a trodden track toward the village." (r9-arm1:notes/round9/arm1/notes.md:8-12)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "the same bad spiky trees." (blind, as K), notes/round6/alice_review.md:180

### r09-2: Winter Morning on the Ryck
- Clip: r09-arm2.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking medium · Rust program
- Time at the easel: 33 min (log 2026-09-25T00:18:17Z to 2026-09-25T00:51:30Z, 33 min)
- Title source: r9-arm2:notes/round9/arm2/notes.md:1 (also r9_ryck_winter.rs:1 and final reply)
- Subject: A still, veiled winter morning below Greifswald: the frozen Ryck winds toward us from under a low hazy sun, pollard willows line a path to the town's three towers, and a man in a greatcoat walks along it, with a big split willow framing the left edge.
- Final: **delivered**, 3200×2207 px, from `git r9-arm2:notes/round9/arm2/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r09-arm2/014.jpg`: mean difference 2.61/255 at 400 px, same picture by eye
- Painter's words: "A still, veiled winter morning on the flat coast of Pomerania below Greifswald, Friedrich's home town. … one man seen from behind (Rückenfigur) on the path, walking toward the town, dark against the snow: the figure the viewer stands in for." (r9-arm2:notes/round9/arm2/notes.md:8-9; r9-arm2:notes/round9/arm2/notes.md:25-26)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "surprisingly nice." (blind, as M), notes/round6/alice_review.md:182
- Reception, **Alice (own words)**: "there's something different about it" (blind, as M), notes/round6/alice_review.md:185
- Reception, blind AI critic (Gemini): "ranks L, M, K, N" (second only to round 2's winter (L)), notes/round9/blind/key.md:7

### r09-3: Winter Evening with a Wayside Cross
- Clip: r09-arm3.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking medium · Lua easel
- Time at the easel: 26 min (log 2026-09-25T00:18:18Z to 2026-09-25T00:44:29Z, 26 min)
- Title source: r9-arm3:notes/round9/arm3/notes.md:66
- Subject: A winter evening just after sunset: an old dead oak on a snow mound against a lemon-rose glow, a leaning wooden wayside cross half buried in snow, and a lone walker seen from behind going toward it, with a far fir wood and a young moon.
- Final: **delivered**, 3200×2286 px, from `git r9-arm3:notes/round9/arm3/painting_3200.png` (3200 px render committed by the painter at the end of its session)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r09-arm3/017.jpg`: mean difference 0.59/255 at 400 px, same picture by eye
- Painter's words: "On a snow mound left of center stands an old dead oak, the picture's main vertical, its crooked limbs against the glow (Friedrich's Oak Tree in the Snow, the dead oaks of Abbey in the Oakwood: the tree as a figure). … an old oak on the mound: a short massive bole, limbs breaking out low and wide, torn open on the left, one long limb reaching right toward the cross" (r9-arm3:notes/round9/arm3/notes.md:10-13; r9-arm3:paintings/lua/r9a3_winter.lua:114-115)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "feels like a regression." (blind, as N), notes/round6/alice_review.md:186
- Reception, blind AI critic (Astra): "N: strongest emotional picture" (ranked first of four), notes/round9/blind/key.md:7

## Round 10
Round 2's setup replicated exactly on today's engine: its brief word for word, one Rust program, the strongest model at high thinking; two winters and one summer. (notes/round10/README.md)

### r10-1: Dolmen in the Snow at Dusk
- Clip: r10-arm1.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 47 min (log 2026-09-25T09:06:56Z to 2026-09-25T09:54:09Z, 47 min)
- Title source: r10-arm1:notes/r10_winter_a.md:1 (also r10_winter_a.rs:1 and final reply)
- Subject: A winter evening after sunset on flat land: a passage grave on a snowy rise with a great bare oak beside it and a broken oak behind, a crescent moon and evening star above, and a lone man in a dark coat standing in the snow with his back to us.
- Final: **delivered**, 3200×2254 px, from `~/src/a/claude-paint-r10-arm1/out/r10_winter_a_full.png` (the painter's --full render; pixel-identical to notes/round10/look/A_3200.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r10-arm1/012.jpg`: mean difference 0.63/255 at 400 px, same picture by eye
- Painter's words: "I placed him low and left of center, facing the grave: the living man before the ancient dead. … Evening light through the dolmen, under the capstone and between the uprights. This is the one moment of drama in the picture, and it reads." (r10-arm1:notes/r10_winter_a.md:27-28; r10-arm1:notes/r10_winter_a.md:237-238)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "trees nice, but zoomed in, twigs hang in the air." (blind, as A), notes/round6/alice_review.md:202
- Reception, blind AI critic (Astra): "P moves me most" (ranked first, above round 2's winter: "The first new painting a critic has ranked above round 2's winter on feeling" (notes/round10/blind/key.md)), notes/round10/blind/critics/astra.md:1

### r10-2: Winter evening by a frozen pond
- Clip: r10-arm2.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 24 min (log 2026-09-25T09:06:56Z to 2026-09-25T09:31:23Z, 24 min)
- Title source: r10-arm2:notes/r10_winter_b.md:1 (also r10_winter_b.rs:1 and final reply, session log 2026-09-25T09-06-56-280Z_5e033060-4640a57e-a27b6374-6e22.jsonl @ 2026-09-25T09:31:23.025Z)
- Subject: A flat snowfield under a winter afterglow: a stag-headed oak black against the glow on the left, three snow-laden young spruces on the right, a frozen pond between them and a small man walking the path toward a far church spire.
- Final: **delivered**, 3200×2240 px, from `~/src/a/claude-paint-r10-arm2/out/r10_winter_b_full.png` (3200 render; pixel-identical to notes/round10/look/C_3200.png, the one Alice saw)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r10-arm2/010.jpg`: mean difference 0.69/255 at 400 px, same picture by eye
- Painter's words: "The young firs are the hopeful counterpart of the dead oak, the pairing in his 1811 winter pair. … It reads as a Friedrich idea at 1000px: the dead tree and the young firs, the lone walker, the church in the haze." (r10-arm2:notes/r10_winter_b.md:28-30; r10-arm2:notes/r10_winter_b.md:194-196)
- Reception, **Alice (own words)**: "probably the best one ... technically pretty good" (blind, as C), notes/round6/alice_review.md:209
- Reception, blind AI critics (Gemini, Astra, Fable): "all three put round 10's pond third" (round 11 cross-critique of six winters), notes/round11/blind/key.md:15

### r10-3: Summer Afternoon: the Lime Tree on the Rise
- Clip: r10-arm3.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 61 min (log 2026-09-25T09:06:56Z to 2026-09-25T10:08:05Z, 61 min)
- Title source: r10-arm3:notes/r10_summer.md:1 (also r10_summer.rs:1 and final reply)
- Subject: Flat summer meadows under a high sky with a single lime tree in full leaf on a low rise, two small figures seen from behind looking across the plain to a far town with church spires, with willows, haycocks and grazing horses in between.
- Final: **delivered**, 3200×2254 px, from `~/src/a/claude-paint-r10-arm3/out/r10_summer_full.png` (the painter's --full render; pixel-identical to notes/round10/look/B_3200.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r10-arm3/018.jpg`: mean difference 0.79/255 at 400 px, same picture by eye
- Painter's words: "Friedrich paints single trees as presences (the lone oaks, the solitary tree in the flat land); the lime is the village tree, home and summer, where the oak in his winters is death. … Under the lime the afternoon stands still, two backs turned toward the steeples' blue; the hay lies cocked along the mown strip, and swallows stitch the sky the whole day through." (r10-arm3:notes/r10_summer.md:16-19; session log 2026-09-25T09-06-56-300Z_cf050e06-71a42ba1-3d5d3b07-cfee.jsonl @ 2026-09-25T10:08:05.570Z (poem break))
- Reception, **Alice (own words)**: "Finally ... the right-ish direction." (blind, as B), notes/round6/alice_review.md:208
- Reception, the person whose post started the project (outside reaction): ""wow!!", "The trees are amazing", "The second one especially"" (shown round 10's three paintings; the second is this one), notes/round6/alice_review.md:267
- Reception, blind AI critics: "the summer painting last ("identical leaf sprites"" (Gemini ranked it last; Astra too (P, S, Q, T)), notes/round10/blind/key.md:13

## Round 11
Other AI models paint the same winter brief: GPT-6 Astra, Gemini 3.8 Flash and Claude Fable 5.1. (notes/round11/README.md)

### r11-astra: The Ford Before Daybreak
- Clip: r11-astra.mp4 · date 2026-09-25 · gpt-6-astra (openai-codex), thinking low · Rust program
- Time at the easel: 30 min (log 2026-09-25T10:40:34Z to 2026-09-25T11:10:30Z, 30 min)
- Title source: r11-astra:notes/r11_winter_astra.md:1 (also r11_winter_astra.rs:1 and session log 2026-09-25T10-40-34-233Z_da1be2eb-b80d12c2-1a646af0-c49f.jsonl @ 2026-09-25T10:42:27.756Z)
- Subject: A cold valley before sunrise: a frozen stream bends toward a pale eastern sky, an old leafless oak stands on the left bank, a small traveler has stopped on the right, and a tiny church sits on the distant ridge.
- Final: **delivered**, 3200×2238 px, from `~/src/a/claude-paint/notes/round11/look/E_3200.png` (3200 render filed in notes/round11/look at 12:19 local, right after the session; it matches the painter's own 1000 px preview (mean diff 1.9/255); the worktree _full.png was re-rendered at 12:38 and differs)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r11-astra/007.jpg`: mean difference 1.71/255 at 400 px, same picture by eye
- Painter's words: "The stream is a threshold, not a picturesque road: its near ice is broken and its farther course disappears in mist. A very small church on the distant ridge offers an uncertain destination rather than a dominant Gothic emblem. … Dark roots and reeds arrest the eye below; the oak's living twigs reach into the large still sky." (r11-astra:notes/r11_winter_astra.md:6; r11-astra:notes/r11_winter_astra.md:8)
- Reception, **Alice (own words)**: "looks so digital" (blind, as E), notes/round6/alice_review.md:231
- Reception, blind AI critics: "N | r11 E | Astra, low (no cracks: `cracks: None`) | 2 | 5 (own) | 2" (ranked 2nd of six by Gemini and Fable, 5th by Astra itself), notes/round11/blind/key.md:10

### r11-fable: Winter dusk on the marsh
- Clip: r11-fable.mp4 · date 2026-09-25 · claude-fable-5-1 (anthropic), thinking xhigh · Rust program
- Time at the easel: 30 min (log 2026-09-25T10:42:01Z to 2026-09-25T11:12:26Z, 30 min)
- Title source: r11-fable:notes/r11_winter_fable.md:1 (also r11_winter_fable.rs:1 and final reply heading, session log 2026-09-25T10-42-00-804Z_b589e383-d4ac0a94-054d1cc8-c85b.jsonl @ 2026-09-25T11:12:26.135Z)
- Subject: A frozen marsh half an hour after a winter sunset: a dead stag-headed oak with snow on its limbs leans in from the left, spruces and a broken fence stand on the right, and a wanderer seen from behind has stopped before a frozen pool to look toward a village spire in the mist.
- Final: **delivered**, 3200×2286 px, from `~/src/a/claude-paint/notes/round11/look/D_3200.png` (3200 render filed in notes/round11/look at 12:19 local, right after the session; it matches the painter's own 1000 px preview (mean diff 2.9/255); the worktree _full.png was re-rendered at 12:41 and differs (mean 19))
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r11-fable/006.jpg`: mean difference 0.65/255 at 400 px, same picture by eye
- Painter's words: "One wanderer, from behind, has stopped on the snow before the pool and looks toward the spire; his tracks come up from the bottom edge. … The fence (added last) recedes to the right of the spruces with snow caps and a sagging rail; it is the most Friedrich-like particular thing in the picture, and it took ten minutes, which says where the next hour should go: more such things (a stone, a gate, a second figure far off), not more passes on the sky." (r11-fable:notes/r11_winter_fable.md:19-21; r11-fable:notes/r11_winter_fable.md:228-231)
- Reception, **Alice (own words)**: "The figure is really nice" (blind, as D), notes/round6/alice_review.md:226
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "An emptiness that doesn't do anything for her." (blind, as D), notes/round6/alice_review.md:227

### r11-flash: Winter Twilight with Megalith and Spruces
- Clip: r11-flash.mp4 · date 2026-09-25 · gemini-3.8-flash (google), thinking high · Rust program
- Time at the easel: 32 min (log 2026-09-25T10:40:39Z to 2026-09-25T11:13:00Z, 32 min)
- Title source: r11-flash:paintings/src/bin/r11_winter_flash.rs:1 (also notes/r11_winter_flash.md:1 and :7)
- Subject: A snowy twilight with a megalithic tomb and a gnarled bare oak in front, slender spruces, distant ridges and mist, and a solitary wanderer seen from behind contemplating the stones under a faint crescent moon.
- Final: **delivered**, 3200×2286 px, from `~/src/a/claude-paint-r11-flash/out/r11_winter_flash_full.png` (the painter's --full render; pixel-identical to notes/round11/look/F_3200.png (unfinished: the painter ran out of API credits))
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r11-flash/011.jpg`: mean difference 3.98/255 at 400 px, same picture by eye
- Painter's words: "Solitary wanderer (Rückenfigur) contemplating the ancient stones in the fading light. … In Friedrich's iconography, these ancient stones represent pagan antiquity, memory, historical time, and mortality, standing silent and enduring through centuries of weather." (r11-flash:paintings/src/bin/r11_winter_flash.rs:14; r11-flash:notes/r11_winter_flash.md:9)
- Reception, **Alice (own words)**: "Paradoxically, F might be the best one ... in sort of a bad painter way." (blind, as F), notes/round6/alice_review.md:238

## Round 12
A study of one bare tree in winter, grown with the engine's tree-growth model. (notes/round12/README.md)

### r12-tree1: (untitled)
- Clip: none (still only) · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 42 min (log 2026-09-25T16:23:00Z to 2026-09-25T17:04:41Z, 42 min)
- Title source: None
- Subject: An old, crooked, stag-headed oak with a broken dead limb stands alone in a snow field under a pale veiled winter sky, with crows on its dead ends and a thin band of distant woods on a low horizon.
- Final: **delivered**, 2400×3077 px, from `~/src/a/claude-paint-r12-tree1/out/r12_tree1_full.png` (the painter's 2400 px render; pixel-identical to the copy in notes/round12/look)
- Still: no clip; a finished whole study (sky, ground and a complete tree), shown as a still
- Painter's words: "The composition is Friedrich's: a single motif on the center line, low horizon, a thin band of distance, much sky. It is Friedrich-like in its emptiness but lacks the small particulars (tracks, tussock heads, the blue in the shadow of each drift) that would make it a place." (r12-tree1:notes/r12_tree1.md:106-108, 121-123)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "R and P "not half bad"." (blind, as P), notes/round6/alice_review.md:275
- Reception, **Alice (own words)**: "Looks very fractal-ey, which is the opposite of what we are going for." (on all three round 12 trees), notes/round6/alice_review.md:273

### r12-tree2: (untitled)
- Clip: none (still only) · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 23 min (log 2026-09-25T16:23:00Z to 2026-09-25T16:45:39Z, 23 min)
- Title source: None
- Subject: An old stag-headed oak stands alone on a snowfield under a low gray winter sky in late afternoon, a storm-broken limb lying half sunk in the snow beside it.
- Final: **delivered**, 2400×3000 px, from `~/src/a/claude-paint-r12-tree2/out/r12_tree2_full.png` (the painter's 2400 px render; pixel-identical to the copy in notes/round12/look)
- Still: no clip; a finished whole study (sky, ground and a complete tree), shown as a still
- Painter's words: "The fallen limb and the drift at the foot tell the story. Composition: the tree sits a little left and low, with a lot of sky above; I kept it (the sky is part of the subject) but a Friedrich would place it with more intent." (r12-tree2:notes/r12_tree2.md:177-179, 185)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "R and P "not half bad"." (blind, as R), notes/round6/alice_review.md:275

### r12-tree3: Old Oak in Snow (a study)
- Clip: none (still only) · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 43 min (log 2026-09-25T16:23:00Z to 2026-09-25T17:05:46Z, 43 min)
- Title source: r12-tree3:notes/r12_tree3.md:1 (program header r12-tree3:paintings/src/bin/r12_tree3.rs:1 has 'Old Oak in Snow, a study')
- Subject: One ancient oak, its top dead and broken into silver-gray wood, stands alone on a low rise of snow under a still, clouded late-afternoon sky with a pale glow low on the left.
- Final: **delivered**, 2400×3053 px, from `~/src/a/claude-paint-r12-tree3/out/r12_tree3_full.png` (the painter's 2400 px render; pixel-identical to the copy in notes/round12/look)
- Still: no clip; a finished whole study (sky, ground and a complete tree), shown as a still
- Painter's words: "The dead grey against the live near-black gives the tree a story. The sky's warm low band behind the bole puts the darkest dark against the lightest light, where the eye should go. In short, a believable old oak, painted with a hand, standing in a placeholder landscape." (r12-tree3:notes/r12_tree3.md:162-165, 187-188)
- Reception, Alice, as summarized in the integrator's notes (paraphrase, not her exact words): "Q (the middle one) is too symmetrical" (blind, as Q), notes/round6/alice_review.md:275

## Round 13
The same tree study with the growth model taken away: each painter drew its tree limb by limb. (notes/round13/README.md)

### r13-tree1: (untitled)
- Clip: none (still only) · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 29 min (log 2026-09-25T17:14:04Z to 2026-09-25T17:42:43Z, 29 min)
- Title source: None
- Subject: An old stag-headed oak, two bleached dead limbs standing above its living crown and one limb sawn to a stub, stands alone on a low swell of snow under a cold gray-blue sky, a fallen limb half buried at the right.
- Final: **delivered**, 2400×3000 px, from `~/src/a/claude-paint-r13-tree1/out/r13_tree1_full.png` (the painter's 2400 px render; pixel-identical to the copy in notes/round13/look)
- Still: no clip; a finished whole study (sky, ground and a complete tree), shown as a still
- Painter's words: "The twigs are the best passage: pointed-rigger strokes lifted off to the tip, warm gray-brown against the cold sky, overlapping into a filigree that looks painted, not rendered. Snow on the limbs is thin and timid. Only a few bands show at viewing distance, so the tree barely says "snow has fallen"." (r13-tree1:notes/r13_tree1.md:183-186, 200-201)
- Reception, **Alice (own words)**: "Ouch. Wow. Ouch. U is the least bad." (on all three round 13 trees; this one is S), notes/round6/alice_review.md:280

### r13-tree2: (untitled)
- Clip: none (still only) · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 32 min (log 2026-09-25T17:14:04Z to 2026-09-25T17:45:42Z, 32 min)
- Title source: None
- Subject: An old stag-headed oak with a sawn stump and an old wound stands alone on a low rise of snow under a pale overcast sky, seen from low down so the horizon crosses its lower trunk.
- Final: **delivered**, 2400×3000 px, from `~/src/a/claude-paint-r13-tree2/out/r13_tree2_full.png` (the painter's 2400 px render; pixel-identical to the copy in notes/round13/look)
- Still: no clip; a finished whole study (sky, ground and a complete tree), shown as a still
- Painter's words: "The snow along the tops of the level limbs is the most "winter" thing in it. The low horizon crossing the bole puts us on the ground, looking up. Friedrich's oaks are wilder: limbs that go out, turn back, break off." (r13-tree2:notes/r13_tree2.md:180-183, 187)
- Reception, **Alice (own words)**: "Ouch. Wow. Ouch. U is the least bad." (on all three round 13 trees; this one is T), notes/round6/alice_review.md:280

### r13-tree3: (untitled)
- Clip: none (still only) · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 32 min (log 2026-09-25T17:14:04Z to 2026-09-25T17:45:39Z, 32 min)
- Title source: None
- Subject: An old stag-headed oak, its dead silver-gray leader standing above a round live crown, stands alone in a snowfield under a low overcast sky, seen from low down with a fallen limb half buried at its foot.
- Final: **delivered**, 2400×3000 px, from `~/src/a/claude-paint-r13-tree3/out/r13_tree3_full.png` (the painter's 2400 px render; pixel-identical to the copy in notes/round13/look)
- Still: no clip; a finished whole study (sky, ground and a complete tree), shown as a still
- Painter's words: "The crown's fine net against the pale sky is the best passage. The picture's quiet, a single tree under a low overcast with the horizon across its trunk, is the right mood." (r13-tree3:notes/r13_tree3.md:169, 171-173)
- Reception, **Alice (own words)**: "Ouch. Wow. Ouch. U is the least bad." (this one is U), notes/round6/alice_review.md:280

## Round 14
A chain of painters: each paints a landscape of its choosing and leaves craft notes for the next one. (notes/round14/README.md)

### r14-p1a: Ploughed field before Greifswald, October morning
- Clip: r14-p1a-ploughed-field.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 63 min (log 2026-09-25T20:46:30Z to 2026-09-25T21:49:29Z, 63 min)
- Title source: r14a-p1:notes/r14_p1.md:1
- Subject: A ploughed field with crows and a heap of field stones rises to a low crest, and beyond it the towers of a small Baltic town stand in morning mist, with a cart track winding up toward them.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-r14a-p1/out/r14_p1_full.png` (the painter's 2400 px render; pixel-identical to notes/round14/look/p1a_ploughed_field_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r14-p1a-ploughed-field/016.jpg`: mean difference 0.55/255 at 400 px, same picture by eye
- Painter's words: "The eye runs up the furrows and the track to the town. The town silhouette (spire, bulbed tower, squat tower, naves) is the one firm, found shape in the picture. Its foot dissolves into mist laid wet over it, which is where I wanted the picture's tension." (r14a-p1:notes/r14_p1.md:38-39, 187-190)
- Reception, **Alice (own words)**: "I kind of like it", notes/round6/alice_review.md:287
- Reception, **Alice (own words)**: "it's not a Friedrich", notes/round6/alice_review.md:287
- Reception, **Alice (own words)**: "very funny ... adorable", notes/round6/alice_review.md:288

### r14-p1: Morning in the Mountains
- Clip: r14-p1.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 66 min (log 2026-09-25T22:29:46Z to 2026-09-25T23:35:33Z, 66 min); the final session; a 3-minute false start (22:25 UTC) preceded it
- Title source: r14-p1:notes/r14_p1.md:1 (also r14-p1:paintings/src/bin/r14_p1.rs:1)
- Subject: From a granite summit before sunrise, a small man seen from behind stands on the highest rock facing the glow over mountain ranges that recede through a sea of mist, with spruces on the right and a thin old moon above.
- Final: **delivered**, 2400×1690 px, from `~/src/a/paint-r14-p1/out/r14_p1_full.png` (the painter's 2400 px render; pixel-identical to notes/round14/look/p1_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r14-p1/025.jpg`: mean difference 0.55/255 at 400 px, same picture by eye
- Painter's words: "The small dark figure on the highest rock against the brightest mist is where the eye lands and then goes on to where he looks. Friedrich would have put something there (a path, a plant drawn from a study) that rewards looking close. My attempt at stones failed; I didn't find the thing." (r14-p1:notes/r14_p1.md:201-202, 210-212)

### r14-p2: Evening over the flat country
- Clip: r14-p2.mp4 · date 2026-09-25 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 54 min (log 2026-09-25T23:36:41Z to 2026-09-26T00:30:27Z, 54 min)
- Title source: r14-p2:paintings/src/bin/r14_p2.rs:1 (notes heading r14-p2:notes/r14_p2.md:1 adds '(a distant town after sunset)')
- Subject: Just after sunset, a small town stands as a silhouette on the far edge of a flat, marshy coastal plain by a lagoon, while two small figures seen from behind watch from a dark rise under a thin crescent moon.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-r14-p2/out/r14_p2_full.png` (the painter's 2400 px render; pixel-identical to notes/round14/look/p2_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r14-p2/020.jpg`: mean difference 0.55/255 at 400 px, same picture by eye
- Painter's words: "The young moon with its ashen light, placed off the glow's axis with its lit limb toward the sunken sun, is right and quiet. That emptiness is Friedrich's, but his empty passages are finer and more deliberate than mine." (r14-p2:notes/r14_p2.md:174-175, 190-191)

### r14-p3: Evening on the Baltic shore
- Clip: r14-p3.mp4 · date 2026-09-26 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 87 min (log 2026-09-26T00:31:05Z to 2026-09-26T01:58:23Z, 87 min)
- Title source: r14-p3:notes/r14_p3.md:1 (also r14-p3:paintings/src/bin/r14_p3.rs:1)
- Subject: After sunset on a stony Rügen beach, a woman in a dark gown stands at the water's edge between a line of fishermen's stakes and a granite erratic, looking toward the afterglow under a thin young moon.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-r14-p3/out/r14_p3_full.png` (the painter's 2400 px render; pixel-identical to notes/round14/look/p3_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r14-p3/034.jpg`: mean difference 0.54/255 at 400 px, same picture by eye
- Painter's words: "The woman is the one hard shape, her head just over the sea's rim, and the eye goes to her and then to the glow. The upper sky is soft and a little heavy; a painter with another day would lay it once more, cleaner, and let the blue sing." (r14-p3:notes/r14_p3.md:182-184, 197-198)

## Round 15
The chain again from a stripped-down studio with only the tools and notes a painter needs. (notes/round15/README.md)

### r15-p1: Evening over a Misty Valley in the Mountains
- Clip: r15-p1.mp4 · date 2026-09-26 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 49 min (log 2026-09-26T01:45:26Z to 2026-09-26T02:34:16Z, 49 min)
- Title source: r15-p1:notes/r15_p1.md:1 (also main:notes/round15/p1_final_reply.md:1)
- Subject: A few minutes after sunset in late October, a small figure with a staff stands on a dark heath knoll with two tall spruces, looking over a mist-filled valley where four ranges recede toward the afterglow under a thin crescent moon.
- Final: **delivered**, 2400×1655 px, from `~/src/a/paint-r15-p1/out/r15_p1_full.png` (the painter's 2400 px render; pixel-identical to notes/round15/look/p1_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r15-p1/019.jpg`: mean difference 0.56/255 at 400 px, same picture by eye
- Painter's words: "He is small, so the space stays large. He stands where the mist is palest, so a small dark shape carries the most contrast in the picture. That vertical link from man to moon replaces any gesture." (r15-p1:notes/r15_p1.md:31-32, 35-36)

### r15-p2: Oak in the snow at dusk
- Clip: r15-p2.mp4 · date 2026-09-26 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 49 min (log 2026-09-26T02:34:38Z to 2026-09-26T06:29:33Z, 235 min); 235 min in the log minus two hung ImageMagick/Inkscape conversions of an SVG sketch (125 and 60 min tool calls, killed by the integrator; notes/round15/README.md)
- Title source: r15-p2:notes/r15_p2.md:1 (also main:notes/round15/p2_final_reply.md:1)
- Subject: On a winter evening after sunset, an old stag-headed oak with crows in it stands on a snowy rise at the left, while a lone walker follows a trodden path across the snow plain toward a Gothic church on the horizon under the evening star.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-r15-p2/out/r15_p2_full.png` (the painter's 2400 px render; pixel-identical to notes/round15/look/p2_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r15-p2/014.jpg`: mean difference 0.62/255 at 400 px, same picture by eye
- Painter's words: "An old dead tree and a living walker are joined by the path to the church under the evening star. I kept it quiet: small figure, no dramatic light. I meant that emptiness; it is right for him, but the picture's weight sits left." (r15-p2:notes/r15_p2.md:43-45, 241-242)

### r15-p3: Morning on the Baltic shore
- Clip: r15-p3.mp4 · date 2026-09-26 · claude-opus-5-5 (anthropic), thinking high · Rust program
- Time at the easel: 41 min (log 2026-09-26T06:29:52Z to 2026-09-26T07:11:02Z, 41 min)
- Title source: r15-p3:notes/r15_p3.md:1 (also r15-p3:paintings/src/bin/r15_p3.rs:1)
- Subject: Before sunrise on a calm Baltic shore, a woman in a dark dress and red shawl stands at the water's edge facing the glow, with net poles on a low dune to the left, a granite boulder on the right and a waning crescent high in the sky.
- Final: **delivered**, 2400×1846 px, from `~/src/a/paint-r15-p3/out/r15_p3_full.png` (the painter's 2400 px render; pixel-identical to notes/round15/look/p3_2400.png)
- Checked against the clip's last frame `~/tmp/catchup-r15-04ce1082/clip/frames/r15-p3/022.jpg`: mean difference 0.52/255 at 400 px, same picture by eye
- Painter's words: "It is empty in the way his *Monk* and his shore pictures are: the subject is the light and the waiting. They are the verticals that cross the horizon: human work, idle at this hour." (r15-p3:notes/r15_p3.md:17-19, 29-30)

## Round 16
Four chains of three painters at the reduced Lua easel (no undo); lanes A winter and B summer (Friedrich), C and D free subject with no Friedrich (C Claude Opus, D Gemini). (notes/round16/look/README.md; notes/HANDOFF.md)

### r16-a1: Hünengrab im Schnee am Abend (Dolmen in Snow at Evening)
- Clip: r16-a1.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane winter, Friedrich (key A1)
- Time at the easel: 53 min (log 2026-09-27T00:06:30Z to 2026-09-27T00:59:13Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/A/p1_final.txt (also notes/round16/look/README.md)
- Subject: A snowy rise at dusk under a thin crescent moon: a huge dead-topped oak with a crow stands over a small figure seen from behind who has come up a trail to a snow-capped stone grave, with a broken oak on the right and a Gothic ruin and fir woods in the far haze.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-studio-9aae10/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/A1_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-a1/024.jpg`: mean difference 0.65/255 at 400 px (the last frame is the 1000 px A1.png)
- Painter's words: "The grave is a snow-capped capstone on leaning uprights, with a fallen stone and the misty plain showing between them. … The bottom-right boulder is still a bit schematic, with its snow, shade and rock in stacked bands." (~/tmp/gallery-fcf9c110/r16/run/A/p1_final.txt)

### r16-a2: Hünengrab im Schnee am Abend (Dolmen in Snow at Evening)
- Clip: r16-a2.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane winter, Friedrich (key A2)
- Time at the easel: 41 min (log 2026-09-27T01:01:36Z to 2026-09-27T01:42:35Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/A/p2_final.txt (also notes/round16/look/README.md)
- Subject: A stag-headed oak with dead silver limbs stands over a snow-covered dolmen on a low mound; a small figure in a long coat, seen from behind, has walked out across the field, under a thin crescent moon and a few crows, with a village church and snowy spruces low on the right.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-studio-9cf69b/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/A2_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-a2/022.jpg`: mean difference 0.53/255 at 400 px (the last frame is the 1000 px A2.png)
- Painter's words: "I left the band of glowing sky above the far woods empty on purpose, since that open space carries most of the Friedrich mood." (~/tmp/gallery-fcf9c110/r16/run/A/p2_final.txt)

### r16-a3: Evening at the Frozen Mere
- Clip: r16-a3.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane winter, Friedrich (key A3)
- Time at the easel: 49 min (log 2026-09-27T01:44:18Z to 2026-09-27T02:32:57Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/A/p3_final.txt (also notes/round16/look/README.md)
- Subject: A winter dusk just after sunset: a stag-headed oak with crows on a snowy knoll at the left, a small walker with a staff at the edge of a frozen mere looking toward a faint Gothic church in low mist and a thin young moon over spruces on the right.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-studio-7ff345/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/A3_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-a3/027.jpg`: mean difference 0.69/255 at 400 px (the last frame is the 1000 px A3.png)
- Painter's words: "The reeds: two glazes on them went wrong, one an opaque brown and one a bright blue. I lifted both off with a clean brush while they were still wet." (~/tmp/gallery-fcf9c110/r16/run/A/p3_final.txt)

### r16-b1: Summer Evening on the Elbe Meadows
- Clip: r16-b1.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane summer, Friedrich (key B1)
- Time at the easel: 53 min (log 2026-09-27T00:05:18Z to 2026-09-27T00:58:18Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/B/p1_final.txt (also notes/round16/look/README.md)
- Subject: A low-horizon summer evening after sunset: an old stag-headed oak on the left and two figures seen from behind on the near bank, looking over hay cocks, pollard willows and a winding river with a sailing barge toward a misty town with a spire, under a thin new moon.
- Final: **delivered**, 2400×1690 px, from `~/src/a/paint-studio-cd267e/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/B1_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-b1/017.jpg`: mean difference 0.61/255 at 400 px (the last frame is the 1000 px B1.png)
- Painter's words: "Near the middle, two figures seen from behind (a woman in a red dress and pale shawl, a man in a dark coat with a stick) stand on the dark near bank. … I didn't varnish it because parts were still wet." (~/tmp/gallery-fcf9c110/r16/run/B/p1_final.txt)

### r16-b2: Summer Evening on the Meadows before Greifswald
- Clip: r16-b2.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane summer, Friedrich (key B2)
- Time at the easel: 44 min (log 2026-09-27T01:00:54Z to 2026-09-27T01:45:19Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/B/p2_final.txt (also notes/round16/look/README.md)
- Subject: Just after sunset, a town with three church towers and two post mills stands against the afterglow; a man in a top hat and a woman in a wine-red dress stand on the footpath with their backs to us, among meadows with haycocks, two horses behind a fence and a dark grove on the left.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-studio-7ace8e/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/B2_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-b2/024.jpg`: mean difference 0.59/255 at 400 px (the last frame is the 1000 px B2.png)
- Painter's words: "A man in a greatcoat and top hat and a woman in a wine-red dress and shawl stand on the footpath with their backs to us, looking at the town. … The Bodden water on the far plain still looks a little like ice." (~/tmp/gallery-fcf9c110/r16/run/B/p2_final.txt)

### r16-b3: Summer Evening on the Meadows before the Town
- Clip: r16-b3.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane summer, Friedrich (key B3)
- Time at the easel: 47 min (log 2026-09-27T01:45:46Z to 2026-09-27T02:32:50Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/B/p3_final.txt (also notes/round16/look/README.md)
- Subject: A flat Baltic plain late on a summer day: a man and a woman seen from behind on a footpath look toward a small town with three brick church towers between two post mills, across meadows with pollard willows, horses, cows, haycocks and a hay wagon.
- Final: **delivered**, 2400×1548 px, from `~/src/a/paint-studio-774267/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/B3_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-b3/026.jpg`: mean difference 0.62/255 at 400 px (the last frame is the 1000 px B3.png)
- Painter's words: "The ditch was first a stark white diagonal line across the whole plain. I painted most of it out and kept only the stretch by the willows." (~/tmp/gallery-fcf9c110/r16/run/B/p3_final.txt)

### r16-c1: Stoneware Jug with Two Lemons and a Knife
- Clip: r16-c1.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane free, no Friedrich (key C1)
- Time at the easel: 69 min (log 2026-09-27T00:05:23Z to 2026-09-27T01:14:01Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/C/p1_final.txt (also notes/round16/look/README.md)
- Subject: A still life: a cream stoneware jug on a stone ledge against a smoky olive wall, lit from the upper left, with two lemons beside it and a knife with a dark wooden handle over the front edge.
- Final: **delivered**, 2400×1920 px, from `~/src/a/paint-studio-3be9e8/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/C1_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-c1/018.jpg`: mean difference 0.49/255 at 400 px (the last frame is the 1000 px C1.png)
- Painter's words: "It's a quiet still life in the manner of Chardin. … The jug's shadow side is mottled like salt glaze; it's made of thin umber glazes over dry light paint, blended only within the shadow half." (~/tmp/gallery-fcf9c110/r16/run/C/p1_final.txt)

### r16-c2: Low Water, After Sunset
- Clip: r16-c2.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane free, no Friedrich (key C2)
- Time at the easel: 39 min (log 2026-09-27T01:16:14Z to 2026-09-27T01:55:23Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/C/p2_final.txt (also notes/round16/look/README.md)
- Subject: An estuary at low tide just after sunset: two dark banks of wet mud frame a slate-blue channel that narrows toward a pale lemon glow, with a spit of land and trees on the right, a line of old stakes and a small boat stranded on the left bank.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-studio-75c237/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/C2_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-c2/033.jpg`: mean difference 0.46/255 at 400 px (the last frame is the 1000 px C2.png)
- Painter's words: "The banks first read as hills, and later as raised slabs with cliff-like sides. I eventually saw the cause: the channel between them got wider toward the horizon instead of narrowing. Once I repainted it so its edges converged, the flats lay down." (~/tmp/gallery-fcf9c110/r16/run/C/p2_final.txt)

### r16-c3: The Punt at Sundown
- Clip: r16-c3.mp4 · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane free, no Friedrich (key C3)
- Time at the easel: 30 min (log 2026-09-27T01:57:14Z to 2026-09-27T02:27:43Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/C/p3_final.txt (also notes/round16/look/README.md)
- Subject: An evening marsh with a low horizon: elms stand dark against a pale lemon glow on the left and a lower copse on the right, and a bright creek winds down to the bottom edge, with a small punt and a standing figure with a pole on its far bend.
- Final: **delivered**, 2400×1714 px, from `~/src/a/paint-studio-96b5e4/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/C3_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-c3/037.jpg`: mean difference 0.58/255 at 400 px (the last frame is the 1000 px C3.png)
- Painter's words: "The focal point is a small punt on the far bend of the creek. … The foreground was the hardest part. It took four glazes to get it dark enough" (~/tmp/gallery-fcf9c110/r16/run/C/p3_final.txt)

### r16-d2: Quince, Stoneware Pitcher, and Bay Leaves on a Wooden Ledge
- Clip: r16-d2.mp4 · date 2026-09-27 · gemini-3.8-flash (openrouter), thinking high · Lua easel · lane free, no Friedrich (key D2)
- Time at the easel: 20 min (log 2026-09-27T00:20:59Z to 2026-09-27T00:41:05Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/D/p2_final.txt (also notes/round16/look/README.md)
- Subject: A still life: a round glazed earthenware pitcher and a ripe quince on an aged wooden shelf, lit from the upper left, with a sprig of dried bay leaves across the front.
- Final: **delivered**, 2400×1920 px, from `~/src/a/paint-studio-7b31aa/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/D2_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-d2/013.jpg`: mean difference 0.54/255 at 400 px (the last frame is the 1000 px D2.png)
- Painter's words: "Illuminated by raking light from the upper left, the composition sets a round, glazed earthenware pitcher and a ripe, pyriform quince in quiet dialogue on an aged wooden shelf." (~/tmp/gallery-fcf9c110/r16/run/D/p2_final.txt)

### r16-d3: Still Life with Terracotta Jug and Quince
- Clip: r16-d3.mp4 · date 2026-09-27 · gemini-3.8-flash (openrouter), thinking high · Lua easel · lane free, no Friedrich (key D3)
- Time at the easel: 19 min (log 2026-09-27T00:43:27Z to 2026-09-27T01:02:11Z)
- Title source: ~/tmp/gallery-fcf9c110/r16/run/D/p3_final.txt (also notes/round16/look/README.md)
- Subject: A still life: a terracotta jug with an amber-glazed neck, a golden quince and a halved lemon whose peel curls over the edge of a weathered limestone ledge.
- Final: **delivered**, 2400×1920 px, from `~/src/a/paint-studio-b5323f/out/easel/painting/painting.png` (the saved 2400 px canvas; pixel-identical to notes/round16/look/D3_2400.png)
- Checked against the clip's last frame `~/tmp/site-r16-41d18220/frames/r16-d3/011.jpg`: mean difference 0.48/255 at 400 px (the last frame is the 1000 px D3.png)
- Painter's words: "To the right sits a golden, textured quince with a puckered dried calyx, and between them in the foreground rests a halved lemon whose delicate ribbon of peel curls forward over the lip of a weathered limestone ledge." (~/tmp/gallery-fcf9c110/r16/run/D/p3_final.txt)

D1 (Gemini, lane D) is left out: aborted after 8 chunks, never saved (notes/round16/look/README.md).

## Round 17

### r17-f1: June Morning on the Meadows before the Town
- No clip · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane summer, Friedrich (key F1)
- Sittings: 4 (74 min, 34 min, 33 min, 42 min), studio ~/src/a/paint-studio-cfa19c
- Title source: ~/tmp/gallery-fcf9c110/r17/run/F/p1_final.txt
- Subject: A lone oak in full June leaf on a grassy rise, with a couple seen from behind under it looking out over river meadows, haymakers, sheep and cattle toward a low Hanseatic town with a needle spire under a large summer cloud.
- Final: finished, 2400×1714 px, from `~/tmp/gallery-fcf9c110/r17/run/F1_finished.png` (the painter's saved log replayed with varnish and cracks applied after the session (run/F1_finished.png))
- Painter's words: "A lone oak in full June leaf stands on a grassy rise at the left. Under it a couple, seen from behind, looks out over the river meadows toward a Hanseatic town. … As in Friedrich, the sky takes most of the picture, and the figures are small and turned away." (~/tmp/gallery-fcf9c110/r17/run/F/p1_final.txt)

### r17-f2: June Morning on the Meadows before the Town
- No clip · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane summer, Friedrich (key F2)
- Sittings: 3 (54 min, 24 min, 14 min), studio ~/src/a/paint-studio-497bcf
- Title source: ~/tmp/gallery-fcf9c110/r17/run/F/p2_final.txt
- Subject: A wide river meadow on a clear June morning under a large soft sky: haymakers and a loaded wagon, pollard willows, cattle and a stork, a pale town on the horizon, and a man and his wife in a red shawl seen from behind on a grassy rise.
- Final: finished, 2400×1714 px, from `~/tmp/gallery-fcf9c110/r17/run/F2_finished.png` (the painter's saved log replayed with varnish and cracks applied after the session (run/F2_finished.png))
- Painter's words: "A sandy cart track curves up the rise to its crest, where a man in a greatcoat and his wife in a red shawl stand arm in arm with their backs to us, looking out over the harvest to the town." (~/tmp/gallery-fcf9c110/r17/run/F/p2_final.txt)

### r17-o1: Evening Tide, Barge off the Marsh
- No clip · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane free subject (key O1)
- Sittings: 4 (21 min, 12 min, 12 min, 11 min), studio ~/src/a/paint-studio-1def1a
- Title source: ~/tmp/gallery-fcf9c110/r17/run/O/p1_final.txt
- Subject: An estuary just after sunset: a sailing barge with tan sails dark against a low gold sky, its reflection broken in the water, a thin far shore with trees and a windmill, and a muddy marsh spit with mooring posts.
- Final: finished, 2400×1600 px, from `~/tmp/gallery-fcf9c110/r17/run/O1_finished.png` (the painter's saved log replayed with varnish and cracks applied after the session (run/O1_finished.png))
- Painter's words: "The sails are painted almost flat, as silhouettes against the light. Only a narrow warm edge along the right side of the mainsail (the leech) suggests the glow coming through the cloth." (~/tmp/gallery-fcf9c110/r17/run/O/p1_final.txt)

### r17-o2: Low Water, Evening
- No clip · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane free subject (key O2)
- Sittings: 4 (32 min, 15 min, 14 min, 15 min), studio ~/src/a/paint-studio-4fb403
- Title source: ~/tmp/gallery-fcf9c110/r17/run/O/p2_final.txt
- Subject: An estuary at low tide just before sunset: a pale channel with old posts leads back to a gold glow under a lilac cloud bank; a tarred boat with a red strake lies on the dark mud at the left.
- Final: finished, 2400×1600 px, from `~/tmp/gallery-fcf9c110/r17/run/O2_finished.png` (the painter's saved log replayed with varnish and cracks applied after the session (run/O2_finished.png))
- Painter's words: "The dark mud wedge and the boat hold the bottom of the picture, and the eye follows the channel back to the light." (~/tmp/gallery-fcf9c110/r17/run/O/p2_final.txt)

### r17-o3: Evening on the Estuary, the Punt Coming Home
- No clip · date 2026-09-27 · claude-opus-5-5 (anthropic), thinking high · Lua easel · lane free subject (key O3)
- Sittings: 2 (49 min, 24 min), studio ~/src/a/paint-studio-c6b5dd
- Title source: ~/tmp/gallery-fcf9c110/r17/run/O/p3_final.txt
- Subject: A quiet estuary at sundown: a big tree dark against the light over a reedy bank, violet clouds with pink undersides, lavender hills, and one punter poling a small boat across the glow's reflection.
- Final: finished, 2400×1714 px, from `~/tmp/gallery-fcf9c110/r17/run/O3_finished.png` (the painter's saved log replayed with varnish and cracks applied after the session (run/O3_finished.png))
- Painter's words: "One punter stands poling a small boat on the still water, a single dark mark against the glow's reflection." (~/tmp/gallery-fcf9c110/r17/run/O/p3_final.txt)
