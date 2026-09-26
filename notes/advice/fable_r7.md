(¬_¬)ﾉ **Short version: the engine isn't what's stopping you. The brief is.** Tonight's three painters converged because all three were handed the same composition recipe, and round 2's winter moves you because its painter was told the opposite. Receipts below.

## 1. The arc

In 52 hours (first commit 2026-09-22 20:03, arms finished 2026-09-25 00:04, 489 commits) the engine went from 10,840 lines of Rust at round 2's commit `503be2f` to 39,622 today (`crates/paint` 29,309 + `crates/easel` 10,313); notes went from 18 files to 145 (19,146 lines). The best picture still runs on the 1x engine. That is the headline.

What made pictures better:
- Physical marks over pixel math. The Python control (`notes/round7/compare/A.png`) is the worst picture in the repo ("very gradient-like"). The bet holds at the floor.
- Bug fixes: varnish ("definitely after"), `clip=<mask>` (2c1a8c0), pinholes. Your principles file already says round 6's visible gains were bug fixes.
- The bisect (`notes/round7/winter_ab.md`): two defaults silently cost a fixed program mean |diff| 6.18 → 3.01 (variant d) against the original. Defaults matter and nothing was guarding them.

What didn't help: structure tools (arm C's painter "did not use the world, sky, clouds, form, rock or tree generators," `arm3/notes.md:59`, and C had the best vibe; arm A had them removed and got the shaving brush; so they are neither the disease nor the cure), critic panels (panels picked main 4–0 on rock B, `notes/round6.md:108`; you picked wet), craquelure as age ("still too neat"), and hand time (§4).

What made pictures worse: the pointed-tip default, the glaze floor, enforced sittings.

## 2. Why round 2's winter moves you

Not the paint: engine d renders that program "remarkably close." Three things it has that nothing since has.

1. **People, and a route to them.** Oak, ruin, a walker, footprints "back to the lower right edge of the picture, where the viewer stands," a brook that is "a road into the picture," a fence, spruces "as the evergreen hope set against the dead oak," crows, stones (`notes/amnesia2/fresh2_winter.md`). Tonight: "No figure and no moon" (arm 1), "There is no figure" (arm 2), "There is no figure and no sun or moon" (arm 3). Friedrich is longing, and longing needs someone in the picture doing it.
2. **The brief told it to.** Round 2: "Friedrich's pictures are full of tiny particular details; don't stop at broad passages" (`notes/amnesia_brief.md:49`). Round 7's required read: "Leave out, then leave out more. Keep the foreground bare," "You can have no figure at all," "no path into depth," "more than five [kinds] and it drifts" (`notes/briefs/friedrich_painter.md` §2, §4, §9, checklist). That document bans the brook, the footprints, the fence, the crows and the stones. It bans the winter picture.
3. **Snow.** A light ground with dark drawn shapes hides what this engine does badly (greens, foliage, glow gradients, mottled stipple) and shows what it does well (a drawn silhouette on a pale field). The winter is dusk too, but the glow is a band, not the subject. Every picture since (Evening Lake, A, B, C) chose glow over a mirror: the Turner subject, the one you flagged as the most digital in the original post.

"Good entropy, perceived" is content entropy. Nine kinds of things, no two alike, is more variation than any brush model adds to a pond.

## 3. The convergence

It isn't Claude's prior, it's `friedrich_painter.md`. Axis + level horizon + cut the middle + calm water echoing the sky + bare foreground + optional no figure = a silhouette on the axis over a mirror. Arm 2 literally counted: "Kinds of things: sky (with moon), trees, bank, water, reeds." All three notes place the motif "on the vertical axis." Round 2's three converged too (each has a crescent moon and a lone figure), but assigned themes forced three different worlds.

This is prescribing pictures already, in prose. Principle 4 says a brief never says what to paint; this one says how the picture must be organized, which for Friedrich is most of it. The fix that isn't reward hacking: retire the composition guide as required reading (keep it for critics), restore the round 2 brief nearly verbatim and assign a theme the way a patron does: season, place, hour. A subject is not a picture. And drop "no story" from the guide: your own source quote includes "a funeral procession" (`notes/friedrich.md:338`), and the London *Winter Landscape* is a man who threw away his crutches to pray (https://www.nationalgallery.org.uk/paintings/caspar-david-friedrich-winter-landscape).

## 4. Engine vs painter

The bottleneck is the painter's decisions and the painter's attention.

Attention: required reading before one mark tonight was about 2,640 lines (principles 131, painter guide 88, hand guide 80, sketchbook 746, README 157, easel guide 1,438). Round 2: README (108) plus materials (131) plus a few feature notes. A painter agent's scarce resource is context, not simulated hours. Every line read is a look not taken, and a painter who has read 52 lines of ceilings, pitfalls and don'ts (grep of `sketchbook.md`) paints defensively: no figure ("stiff, symmetric silhouettes, cut-out looking," §9), no foliage, no rock. "Too precise" is fear. Reward hacking by subtraction, which the anti-hacking rules don't catch.

Hand time made the passage you complain about most worse. Arm 3: "the sketchbook's sky stipple (width 2.6, coverage 2.6) would take about 23 h for this sky ... So I stippled coarser (width 4 to 4.5, coverage about 1.4)." Arm 1: 3.2 over five sittings, a runaway loop "refused as a whole." Arm 2 (Rust, no clock) used 2.4 then 1.5 (`pond_poplars.rs:178,184`), like round 2's 2.6 then 1.5 (`fresh2_winter.rs:110-115`), and you said B's sky was "less JPEG-artifacty than A's." The mottle in `arms/C_crop_upperright.png` is 4-unit touches. The clock enforced labor and got a coarser hand.

Keep: bristles, KM, wet pickup, drying, look/undo/replay, checkpoints. Cut: enforced sittings (keep the timesheet as information). Change: freeze harder. Make `fresh2_winter` the golden picture: any engine change renders it at 1000 and must stay under mean |diff| 3.5 against `winter_port/original_1000.png` (d is 3.01), or you see a sheet before it merges. The metric exists already.

## 5. The principles

Right: physics not answers; the creativity is the painters'; first principles. Incomplete in three places.

- "Constraints" drifted from perceptual constraints (look first, few piles, few passes, no exact geometry) to labor accounting. Only the first kind shows in the picture.
- "Entropy" is scoped to marks. Add content: subject, cast, incident. Three identical pictures is zero entropy before a brush is picked up.
- The checklist exempts prose. A brief, a guide and a sketchbook are tools and hand out answers as surely as `tree_in{}`. Apply "does it default to the painter deciding?" to every required read.

Add: the patron gives the subject.

## 6. Two weeks

Week 1
1. Day 1: fix the whitish line in arm 1 (a real bug). Add the golden-picture gate; done when it fails with the pointed-tip default put back.
2. Day 1: rewrite the painter brief to round 2's length from `amnesia_brief.md`. Required: README, materials research, the API guide for the medium. Sketchbook and guides linked, not assigned. Hand time off for painters. Themes assigned: winter, coast, mountains again, so round 8 compares straight against round 2.
3. Days 2–3: round 8, six painters, each theme once in Rust and once at the easel without procedural tools (C's vibe was Greifswald, n=1; test the medium with the subject fixed). Same 2-hour budget.
4. Day 4: blind deck for you with round 2's three as unmarked anchors. Worked if you rank any new picture above `fresh2_winter.jpg` or say "moved." If all six converge again under the old brief, the convergence is Claude's, and themes plus a cast list is the next lever.

Week 2
5. Only what you name goes into the sketchbook. Second batch, three new themes you pick.
6. If nothing moved you: the gap is drawing (trees, figures), not paint. Single-subject studies in round 2's style, the painter writing its own tree and figure functions, judged by you alone.

Stop: new engine capabilities, including merging r6-wet and piles (park both for the two weeks); critic panels as deciders; enforced sittings; growing the notes as required reading; a composition guide that describes a picture.

## 7. Else

You got a good picture at hour 31 with a tenth of the code, and it was good because a painter took risks a research document later forbade. The project isn't failing. The pictures got emptier as the notes got fuller, and that is a cheap fix. The round 2 winter is already your benchmark in everything but name; use it as the yardstick and stop worrying about copying it. Nobody copies it by painting a coast.
