# Remaining viewer matrix

Source `4e525e50897807e9b5f734071dfeb330f1a393d3`, 2026-10-04. T3 browser tab loaded the unchanged studio HTML/CSS from a disposable server on private Tailscale port 18767. This server supplied protocol-level fixture events and real PNGs. These checks exercise the browser, not the real studio parser, unless separately stated. Existing preview port 18765 was preserved.

## Controls and image sequence

A 15-event history included three whole looks, crop, palette, user-turn and sitting boundaries. Actual DOM observations:

- ArrowLeft moved 15/15→14/15 REWOUND; ArrowRight returned 15/15 LIVE. Brackets moved between images and returned to LIVE beyond the last image. Space started replay, a second Space paused and `l` returned to latest.
- Focused speed/select and scrub/input suppressed global ArrowLeft. Ctrl, Alt and Meta also suppressed it.
- Code toggle and image zoom left replay running. Selecting a different painter stopped replay, hid zoom and replaced the query string.
- Crop at event 7 retained whole image 0 and displayed study. Palette at event 8 appeared separately. User event 9 and sitting event 12 cleared the previous study while retaining main canvas and palette.
- Replay visited only event positions 4, 11 and 14, then latest 15. Observed tick intervals in milliseconds: slow `2012,2017,2007`; 1/s `1005,1008,1007`; 2/s `513,507,511`; 4/s `256,260,259`. Changing slow to 4/s 300 ms after starting retained the scheduled first interval `2016`, then used `262,265` ms intervals.
- A crop-plus-palette-only history kept the crop as main canvas, showed palette separately and Replay remained stopped because there was no eligible whole look.
- Text-only history showed the no-picture message. Reference-only history had no main image and showed the separately labeled Reference study.
- A 261-event history displayed exactly 251 reply blocks, EVENT_9 through EVENT_259.
- A recorded thought grew from 3 to 42 displayed characters in 400 ms. Clicking its earlier image chip entered 2/3 REWOUND.
- Code reconstruction kept successful `GOOD=1` paint while excluding errored `BAD=1` and pending `PENDING=1`; a separate file-write history retained `GOOD=2` and excluded pending overwrite plus failed edit.
- A closing reply aged approximately 116 seconds initially displayed LIVE and changed to FINISHED after 4.5 seconds of actual wall time.

## Picker focus boundary

Home/ArrowLeft clamped to the first card; End/ArrowRight clamped to the last card. Outside pointer-down dismissed the picker. Native Tab from the final card moved focus out of the document (body, no relatedTarget); later Tab reached the header and then previous-look control while the picker remained open. This is a narrow failure of the description's broad Tab-away dismissal claim. The focusout handler only closes when relatedTarget is non-null and outside the picker/header. It does not close when focus leaves the document itself. No inference is made about every browser's toolbar traversal.

## Polls and resting state

A static fixture with stable ETag was checked at browser resource times 93 and 61000 ms. After 75 seconds the journal mutation count remained zero and its original DOM node was retained. Thus the unchanged-metadata check avoided redisplay. The server was a protocol fixture supplying the ETag, not a production static host.

The live picker remained open while another painter was added server-side. Session requests occurred at 46, 30449 and 60073 ms. The focused card node and original three-card list remained intact; closing and reopening then displayed the new fourth card. This exercises both the approximately 30-second refresh and focus-preserving list update.

A closing reply at age 116 seconds with crop study plus palette showed LIVE/study/palette. After 4.5 seconds it showed FINISHED with study hidden and palette retained. A separate three-minute-old reply with fresh file mtime showed FINISHED in the viewer while its picker card said painting now, confirming the existing activity inconsistency with the exact requested age fixture.

## Image failures

The static view retained image 0 while image 3's reduced JPEG was delayed two seconds, then displayed image 3. With the reduced JPEG returning 503, two requests were observed before fallback loaded the original PNG. With all three URLs for a previously uncached image 8 returning 503, the previous image 0 remained displayed after 3.5 seconds and 13 recorded attempts across initial/replacement work; no empty main image replaced it. These are controlled HTTP failures at the browser's real image-loading boundary.

## Actual export and PNG decoding

The unchanged production static exporter output supplied by the delivery lane was served under `/export/`. The browser displayed the actual exported painter at 9/9, with main `data/paint-studio-decafe/v/1.jpg`, natural width 1000. This was a real export artifact, separately from the protocol fixtures above.

The browser decoded the delivery lane's actual `wet.png` into a 2400×480 canvas with explicit `colorSpace: srgb`. `getImageData(...,{colorSpace:'srgb'})` reported `srgb`. Independent Pillow and browser RGBA samples matched exactly:

| Pixel | RGBA |
|---|---|
| 0,0 | 233,229,219,255 |
| 300,192 | 156,74,48,255 |
| 500,192 | 175,74,48,255 |
| 1200,240 | 234,230,219,255 |
| 2399,479 | 234,230,219,255 |

This establishes browser interpretation of those 8-bit sRGB samples, not physical pigment-color calibration. Browser SHA-256 was unavailable because Web Crypto was not exposed on the private HTTP origin; no full-image hash comparison is claimed.

## Late result and rebuilt history

With a pending paint at event 2/15, the code pane read `no program yet`. After a real HTTP poll returned that action's successful result and a new event, the pane showed `--@ chunk 1` and `canvas{}` while the viewer stayed REWOUND at event 2. The scrub maximum became 15 (16 total events), but the position label initially remained **2/15**. This is a stale denominator when appending while rewound; `poll()` does not update that label after adding events unless it shows a position again.

**New defect:** while rewound at 14/16, replacing the server history with two events and a new epoch produced `Uncaught TypeError: Cannot read properties of undefined (reading 'ts')`. The main image was cleared, scrub maximum became 1 and the label remained 14/16. `reset()` retains the old position; the rebuilt-history branch restores `live=false`, so the appended replacement is not shown/clamped before `status()` reads `events[pos].ts`. Source: `studio/index.html:396`, `:415`, `:432`, `:521`. This was a protocol-level rebuilt stream through the actual browser, not a synthetic call to status().

## Routing, preferences and public server

The actual production studio server parsed two JSONL sittings. The painter URL showed 19/19 (including a sitting divider); a local `s=` URL showed 9/9 and labeled sitting 1 of 2 only. At the second sitting's first image, the clock read 16 seconds: ten seconds from sitting one plus six from sitting two, excluding the nearly one-hour gap. The easel clock was absent before printed clock evidence and, on a fresh server after adding `day 1, 09:30` to a successful paint reply, displayed 30 min.

The local picker included a `paint-judge` nonpainter after enabling all sessions. That setting survived reload and did not change the selected painter. A nondefault replay speed of 4 and visible code pane reset to speed 1 and hidden code on reload. Existing intro persistence evidence is in [the earlier browser run](runtime-viewer.md).

A separate actual studio `--public` server, still bound privately to the Tailscale interface, listed only painters, returned 404 for an explicit session request and displayed source containing synthetic home/account fixture text as `-- ~ user`. No login surface appeared.

## Additional browser cases

Initial selection chose the active painter with largest image index; with all inactive it chose the first with a picture; with no pictures it chose the first entry. Cards showed model, subject, date and activity marker. Static listing included active, recently signed-off, titled and gallery-listed painters; an old untitled painter was omitted until explicitly selected in the URL.

For a history whose older source could not be reconstructed, the code pane showed the current source both live and rewound. Hiding code left the file-request count unchanged at two over 2.3 seconds. With event responses delayed two seconds, the thumbnail URL was already displayed while the event-position field was still empty.

During replay, clicking a strip thumbnail stopped at 4/15 REWOUND. The `l` shortcut returned to 15/15 and stopped replay. Repeated Right and `]` reached the latest event and stopped replay. Picker ArrowRight changed the focused painter card while timeline position remained 4/15.

## In-place history rewrite limitation

The actual running production parser continued serving an earlier tool result after its JSONL file was rewritten to a same-size-or-longer version. The raw file contained `day 1, 09:30\nok`; the already-running server still returned `ok · chunk 1`. A fresh server reading the same file returned the new text. [Exact comparison](matrix-viewer-parser.json).

The parser resets only when file size falls below its saved offset (`studio/studio.py:261`). This is distinct from the frontend's failure when an actual new epoch arrives. Whether arbitrary in-place edits are supported is a product call: append-only logs are the normal case, but the checklist's broad external-history-change promise is not established for these edits.

## Final state and timing checks

A production-server history used images copied from the actual idle painting's current look. After zooming, dismissing zoom, replaying, scrubbing, switching painters and navigating away, a fresh look and source-log hash were unchanged. Journal stamps before and after both read day 1, 09:03. [Exact hashes and clock stamps](matrix-viewer-state.json). Navigation away destroys the watching page; literal tab-close behavior is not separately claimed by this observation.

Scrubbing across the nearly one-hour sitting gap moved from 9/19 to 10/19 in one unit. At the far-right live edge, appending a new message through the actual JSONL server advanced the view to 20/20 LIVE and displayed the appended text.

A fresh production-server fixture's last nonreply event was approximately 1796 seconds old. It displayed LIVE, then FINISHED after 4.3 seconds. Together with the reply-boundary check above, both activity thresholds were observed using actual elapsed time.

The wide-canvas stream view had `tall=false`, hidden transport and two thumbnails. The earlier narrow stream overflow remains a failure; this wider variant does not erase it. Existing input/select/picker/button focus and modifier checks cover the controls the viewer actually provides; there is no textarea control in this UI.

A final literal tab-close check then completed the remaining teardown case: while the existing easel executed a bounded chunk assigning close_watch_marker=42, the browser called window.close(). T3 reported the tab unavailable with no URL. The already-running easel finished, and a fresh CLI request printed 42. Closing the viewer did not cancel the painter.
