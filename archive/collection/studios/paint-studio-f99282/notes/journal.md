- day 1, 09:00: Plan. Friedrich: fine linen, 2-4 thin warm-under/light-over grounds; precise graphite underdrawing; one or two very thin layers; gradations by stippling and thin paint pooling in the ground's texture, not by thick blending. Few pigments: lead white, smalt, yellow ochre, red earth, raw umber, bone black, vermilion (a touch).

  My picture (invented): late evening on a broad upland lake. Low horizon, high sky; sun gone down behind the hills so the sky's warm band sits low and cool grey above it. Three receding ranges, each paler and cooler than the one in front, the farthest taking the last light. A band of mist lying on the water at their feet. Still water with broken horizontal light, the ranges faintly repeated in it. Dark near shore, bare trees at the left edge cropping the top of the picture, a leaning fence, reeds. Small figure from behind at the water's edge. A few birds high up. Mood: waiting.

  Numbers: canvas 72 x 56 cm, aspect 1.28 -> W 1000, H 781. Horizon y=445. Water 445-655. Shore 655-781.
- day 1, 09:48: Drawing done (day 1). Two passes: 2H faint, H/4B bold. Trees built as upright axes with limbs leaving near the tip of each shoot, turning up; only three orders drawn, the finest twigs left to paint. The near trees stand on the bank and are cut by the top and bottom edges, which is what a low viewpoint does to a tree a few paces away.

  Lesson: my first tree attempts were fans of straight sticks — three children per node and a random walk in the angle that walked past vertical. Fixed by giving every limb a start angle AND a tip angle, and cutting children to two per node.
- day 9, 11:15: Learned twice, now costly: a light glaze dragged over dark paint that is still open lifts it and leaves a ghost. Both times it was the trees and the figure that went. Before any light pass over an area with incident in it, wait until the paint there is dry, and lay the incident last.

  Also: the dark masses only darken with bone black and lead white in them. A pile of umber, smalt and green earth is a stain, not a covering — the massif stayed pale through three attempts until I tested three weights side by side and took the bone-black one.
- day 50, 18:23: Done, day 51. The picture: a lake among hills at dusk, the sun gone down behind the far peak; a woman with a bundle of faggots stands at the water's edge with her boat, not going in yet; two bare oaks cropped by the frame, fog on the water, birds gone home.

  What the easel taught me this time, all of it the hard way:
  - `work`'s coverage takes a number, not a function; a graded mask is what varies density. `stipple`'s coverage does take a function.
  - A cover paint only covers if something in the pile hides. Umber, smalt, pale smalt and green earth are stains; lead white and bone black are the body. Everything I wanted dark had to be built on bone black.
  - `fill=true` is what completes a passage. Without it, high coverage still leaves the underlayer showing.
  - The softest masks are the ones that misbehave: with a low threshold and fill, a veil I meant for the feet of a range painted the whole lake white. Mask the veil to its own region.
  - Paint from the tubes, with little or no medium, reads as colour. Piles thinned with oil came out warm, whatever the pigment, because the warm ground comes through.
  - A global glaze with long low-pressure strokes leaves the loaded start of every stroke behind as an opaque lozenge. One of those cost me the whole picture; I rebuilt it region by region.
  - Over a wet film, a light pass lifts and smears what is under it. Wait for dry, and lay the incident last.
  - Oil-rich lead-white glazes stay tacky for five weeks. That is why a blob of one took a month to cover.

  Where it falls short of what I aimed at: the trees are sticks rather than oaks, and the lake is soft where it should be glassy.
- day 99, 13:21: day 99: Cost me a week, but the lake is finally quiet: a dark violet-grey surface, tone only — a pale wedge for the mountain upside down, a dark band for the bank, a cool lift under the bright sky at the left, a handful of long drags. Lesson on the water: bright marks on dark water read as sparks, and even at low contrast they read as scribble; the reflection has to be carried by tone. Light on water belongs in the *horizon band*, not spread over the lake.

  The rect() trap, which cost three floods: rect(x, y, w, h) takes a HEIGHT. I had been passing the second y. Every band I built from chunk 158 ran to the bottom of the canvas and buried the trees, the bank, the figure and the lake. Masks built from functions — mask(function(x, y) ... end) — cannot get this wrong; use those from now on.
- day 177, 20:46: day 180, finished: "Evening on the Upland Lake".

  The picture: an upland lake after sundown. The sun has gone down behind the hill on the right; its last light is a warm band low in the sky and on the flank of the pale peak. Mist lies on the water at the far shore and a low wooded bank shows through it. The lake is dark and quiet — the peak is in it upside down as a pale wedge, and a few long drags are all the movement there is. Two bare trees, cropped by the left edge, stand on the near bank; their fine branchlets are the most delicate thing in the picture. At the water's edge a woman stands with her bundle of faggots on her shoulder and her boat drawn up beside her, not going in yet. Birds are going home. Mood: waiting.

  What this last stretch taught me, all of it expensively:
  - rect(x, y, w, h) takes a HEIGHT. I passed the second y for a week of painting time; every "band" ran to the bottom edge and buried the trees, the bank, the figure. Build bands from mask(function(x,y)) instead — it cannot get this wrong.
  - A light glaze over dark, wet paint lifts it. And a bright pile with fill=true scatters flat pale lozenges that read as leaves or scum. Mist is a low-contrast veil at low pressure, nothing else.
  - Reflections on water are carried by tone, not by marks. Every attempt to draw the light — dashes, drags, stipple — read as sparks or scribble. What works: darken the lake, then lift a soft wedge under the mountain and blend it.
  - stipple's coverage does take a function(x, y), and that is the only honest way to get a gradient: `work`'s coverage is a single number and a soft mask only softens the edge, it does not thin the paint. But stipple leaves specks — do not use it where the surface must stay smooth.
  - A trunk's pale core came back twice: it was not enough coverage, not a wrong colour. Bone black at coverage 4 with fill finally held.
  - The trees are better and still not oaks. The fine branchlets changed them from a diagram into a tree; what they cannot do is untaper a limb that was drawn as a bar.

  Where it falls short: the limbs are still bars at 1:1, the bank is a smooth mound, and the far shore's line is straighter than I would choose.
- day 186, 23:38: day 186: Back at the bank, which was the emptiest passage left. Three things it cost me, all of them the same mistake in different clothes:

  - A shadow is not a shape you lay on top. My first shadows were long soft ribbons of dark paint; the bank was still open, they lifted it and left pale ghosts of the ground showing through. Paint into a wet passage and you erase it, not shade it.
  - A pile's value is not what I intend. Umber and black in a mixture do not darken it; lead white and yellow ochre make it light, and I keep underestimating how much. The bank came out first as bright olive, then, having put white in a "ridge" paint, as a strip of butter. Take the value from what the pile does on the canvas, not from the recipe.
  - A bank is not a shape to be modelled in bands. It is a dark mass with a few light accidents in it — a stone, a stalk, the lit edge of a tussock. The bands were what made it look like cardboard.

  Also: the trunks were flat bars, so I rebuilt their masks from the drawn spines and laid a graded light down each right flank. The first try was a pale zipper; the second, covering it, a chain of beads. A trunk that has been painted wants even, dark, low-contrast handling and nothing else.
- day 212, 03:32: day 212: back to the picture after the bank, day 199. I had left it "finished"; the bank was the one passage I had always known was weak — a smooth olive mound with nothing in it — and the trunks were flat bars.

  What I did, and what each thing cost:

  - The trunks. Three attempts at giving them a lit flank: a pale zipper down each one, then a chain of beads where I covered it, then a rim light set in from the edge that read as strips of tape. All three were the same error: I mixed a "slightly lighter than the trunk" pile and it came out three times too light, because a trunk that reads black on the canvas is a pile with a great deal of bone black in it and any white or ochre at all changes it. They are silhouettes against a sunset sky, and silhouettes want nothing done to them. I gave them a few dark bark knots instead and left them.
  - The bank. Four passes and two reversals. First it came out bright olive, then, when I put white into a "ridge" pile, as a strip of butter; then I made it the darkest mass in the picture, which was right, and it is what it is now — a dark mottled ground with a fringe of grass along its edge and nothing in the middle. Every attempt at light incident on it (yellow tussocks, pale stones, a bleached stick, a dry scumble) came out as sparks, eyes, chalk scratches or a flat haze, and each one had to be scrubbed back. The rule I ended on: a dark mass wants its interest at its edge, not in its middle.
  - The cast shadows I put on the bank lifted the still-open paint and left pale ghosts of the ground; the warm glaze I tried later laid visible stripes and lifted again in the one place the bank was still open. A glaze is not a repair.

  Where the picture now falls short, honestly: the bank is empty, the limbs are still bars, and the far shore's line is straighter than I would choose. I would rather stop here than keep correcting — every further stroke I have put into those three things this month has made them worse, and the easel is not the place to argue with a decision I have now made three times.
- day 223, 16:39: day 224, 16:40: Back to the far shore — the one passage my last note called too straight. What I did and what it cost:

  - GOOD: I laid five bare trees rising above the shore line, with a second small tree and two more at uneven intervals, each stem hand-plotted and stroked with a rigger at falling pressure so it tapers. A far shore needs a few individual trees standing out of the wood; that one addition is what broke the comb. The trees read as trees by picture scale.
  - BAD, twice, the same mistake: `work` with hand="scumble" or a big filbert over blurred ellipse masks lays FLAT LOZENGES. The reason is mechanical and I should write it down: a "body" stroke is 20-60 units long and my ellipses were 90 units wide and 12 tall, so one stroke covered the whole shape and its own outline became the mark. Lozenges are not a mistake of colour or coverage, they are a mistake of stroke size against shape size. Small strokes (hand="detail", 4-14 units) over the same masks gave a fine broken tone instead. Every lozenge I have made in this painting was made this way.
  - The cover works and costs four days each time: a light veil of mistFix at coverage 3, fill, over the blots, after the blots are dry, buries them and matches the surrounding mist almost exactly. I have now done this twice.
  - Smearing wet paint with a clean soft brush works, and it is the cheapest thing I have found for softening a mistake: several passes in both directions merged a row of lozenges into one continuous soft band. But laying light mist paint into a wet dark band to break it up made pale lozenges in a darker band, exactly as documented.
  - `work`'s coverage takes a number only. Confirmed again by the error message. Vary density with a graded mask or with small strokes.

  The shore is now: a low broken line of dark wood along the waterline, clumped, with gaps, seven bare trees standing from it, mist swallowing its base and its right-hand end. It is the best the middle distance has looked.
- day 227, 16:54: day 228, 17:00: Done this time, and I mean it.

  The reflection under the far shore is in, and it is the third way I have tried it. The first two (a dark smear, then a band of grey) both came out as evenly spaced lozenges and had to be covered; what worked was hand="detail" over short, blurred ellipse segments — small strokes, 5-14 units, against a band 6-8 units thick — which broke the reflection into a fine mottled grain, and then ONE set of long horizontal smear passes with a clean soft brush while it was still wet, which turned the row of separate marks into one continuous band with a rippled lower edge. Small marks first, then merge them wet: that is the recipe for a reflection, and it is the same recipe I use for water anywhere.

  Standing back: the middle distance is now a low broken line of wood along the far waterline, seven bare trees standing out of it at uneven intervals, mist over its base, mist swallowing its right-hand end, and the wood's own dark grain reflected in the mist just below. Before this week that passage was a comb. It is the part of the picture I am proudest of.

  Also settled, after looking hard at it for a while and nearly repainting it: the pale broken streak at y 628-640 on the right is not scum floating on the lake, it is the lit crest of the near bank rising to the right, with the scrub on it. I was wrong twice about what it was before I measured it against a grid and found the bank mass immediately below it. Measure before you decide that a passage is wrong.

  Where the picture still falls short, and I am leaving it there: the limbs of the near trees are still bars at 1:1, and the bank is still empty in the middle. Both have been corrected three times this month and both got worse. The sky's cloud grain is regular if you stand close to it, and that I chose to leave rather than glaze over a sky that works.
