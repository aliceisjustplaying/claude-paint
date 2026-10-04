# Studio runtime verification

Agent-run on 2026-10-04 against a clean detached source checkout at `4e525e50897807e9b5f734071dfeb330f1a393d3`. This is runtime evidence, not a human verification signoff.

## Existing server and export tests

Ran from the pinned checkout, using the documentation repository's existing virtual environment:

```text
uv run --python <docs>/.preview-venv/bin/python --with pytest --with pillow pytest -q -p no:cacheprovider studio
23 passed in 6.51s
```

The first run without Pillow reported 18 passed and 5 skipped. The recorded 23-pass run supplies Pillow and has no skips. These existing tests exercise actual HTTP responses and export subprocesses with disposable logs. In particular, `test_file_endpoint_serves_the_painting_source_only` verifies the identified source is accessible and BRIEF.md, notes and the binary are refused. Export ownership tests cover refusal, adoption and pruning without deleting hand-added files. Image export tests cover originals, reduced copies and replacement. These tests do not establish browser appearance.

## Browser fixture and method

T3 collaborative browser tab `tab_4` opened an isolated server at `http://<tailscale-host>:18767/`. It used production `studio.H` and the production HTML unchanged. A temporary subclass intercepted only selected HTTP requests: a control file enabled an HTTP 503 plain-text response for `/api/sessions` or `/api/events`, or an eight-second delay for painter A's event response. Other requests delegated to the production handler. Request timestamps were logged. This introduced real HTTP failures and latency; no browser application variables or functions were replaced.

Three disposable painter folders A (`paint-studio-aaaaaa`), B (`paint-studio-bbbbbb`) and C (`paint-studio-cccccc`) initially copied the committed [two-look fixture](../fixtures/sessions/paint-studio-decafe/probe.jsonl), replacing its painter suffix. Thus each started with nine events and two real probe images. These are synthetic histories, not live model runs. Read-only DOM inspection recorded text, focus, image URL and computed style. The existing review server at port 18765 was preserved.

## Startup failure: BUG-02 reproduced

At 11:35:46 UTC, the first session-list request returned HTTP 503 with `probe outage`. Browser console:

```text
Uncaught (in promise) SyntaxError: Unexpected token 'p', "probe outage" is not valid JSON
```

The page showed `Choose a painter`, `painting for 0:00:00` and empty picture/history. The fault was removed at 11:35:46. At 11:36:09 the page was unchanged and its resource list still contained only `/api/sessions`; the server request log contained no retry or event request between those observations. Reloading resumed startup. This proves no automatic recovery during the observed 23-second window; the missing timer installation is separately source-backed at `studio/index.html:824`.

## Event failure: BUG-03 reproduced

At 11:36:10 the session list succeeded but the first events response returned 503. The page said:

```text
This painter hasn't started yet.
Its canvas will show here when it does.
```

The painter already had nine events. Failed requests continued at approximately two-second intervals (11:36:12.892, 14.886, 16.899 and 18.877). Removing the fault let the viewer recover to `9/9` without a reload.

After successful loading, events were failed again from 11:36:44 through 11:37:00. The existing image and `9/9` remained; the badge stayed `FINISHED`. There was no disconnected indication. Rewind and return controls still worked. This fixture was old enough to be finished, so this run establishes a stale finished badge, not specifically a stale LIVE badge.

## Empty picker: BUG-09 reproduced for every navigation key

The server's sessions directory was temporarily replaced with an empty directory. Opened picker, clicked its all-sessions checkbox to place focus inside the dialog, then pressed Home, End, Left, Right, Up and Down. Each generated a new uncaught browser exception, respectively at 11:37:23.423, 24.484, 25.558, 26.569, 27.141 and 28.144. The dialog remained open and empty. Pressing keys while focus remained on the header outside the dialog did not exercise this handler. The production null-card dereference is at `studio/index.html:371–372`.

## Selection, focus and delayed response

- With three cards, opening the picker focused the currently selected A card. Home focused C; Right focused B; Down focused A; Left focused B; Up focused C; End focused A. The ordering matched the displayed cards.
- Home then Enter selected C and replaced the query with `?p=paint-studio-cccccc`. Opening again, End then Space selected A. Both keyboard activations worked.
- From A at `6/9 REWOUND`, choosing A again closed the dialog and retained `6/9 REWOUND`. Focus returned to `who`.
- Escape and the close button each closed the picker and returned focus to `who`. Outside-click and Tab destinations were not fully checked.
- With A's events delayed eight seconds, the request began at epoch time `1791113849.457`; C's request began at `1791113850.169`. C displayed `9/9` and its own `/img?p=paint-studio-cccccc&i=1`. After A's delay had elapsed, C's query, image and position remained unchanged. The stale A response did not overwrite the chosen target.
- A newly addressed D initially showed the waiting message. Creating its fixture while the page remained open let polling display `9/9`, its D image and a hidden waiting message without reloading.

## Polling, source, zoom and replacement

- Production events polling continued roughly every two seconds; representative request times were `1791113851.898`, `3853.897`, `3855.890`, `3857.889` and `3859.819`. Exact 30-second session refresh and static cadence were not measured.
- Appended a pending paint call `print("late probe")` to C. It appeared as a tenth event. Appended its successful result (`late probe`, `ok · chunk 3`) and a reply. At the live edge with code shown, reconstructed code included `--@ chunk 3` and `print("late probe")`. A subsequent appended reply left the selected point at `9/11 REWOUND`. Adjacent reply handling means appended JSON lines are not necessarily separate event positions.
- Replacing C's history with its original nine-event fixture while following reset the display to `9/9`. Rewound/replaying replacement variants remain unrun.
- At an earlier whole look, clicking the canvas opened zoom with its original `/img?...&i=0` URL. Escape dismissed it (`display: none`). Pressing `c` showed the reconstructed first chunk at `6/9`. Palette/reference zoom and zoom during replay remain unrun.
- Loading `stream=1` loaded `/stream.css`, hid transport (`display: none`) and retained the thumbnail strip (`display: flex`). Tall-canvas layout and switching to a different painter in stream mode remain unrun.
- The original old fixture displayed `FINISHED` in the viewer while its newly copied file appeared as `painting now` in the picker. This reproduces the documented activity disagreement for an old-history/new-file fixture; it does not establish the exact three-minute closing-reply scenario.

## Public HTTP boundary

A separate temporary production HTTP server on port 18768 used public mode. Its scrub identity was explicitly set to the synthetic `/Users/private-fixture` account. An appended assistant message contained `Path /Users/private-fixture/work and PRIVATE-FIXTURE`.

```text
/api/sessions                 200; synthetic private identity absent
/api/events?p=<painter>       200; text contains "Path ~/work and user"
/api/events?s=<session path>  404
/api/events?p=other           404
```

This checks scrubbing and target refusal at the real HTTP boundary. It does not independently inspect public browser presentation or every source-file scrub variant.

## Limits

Unrun viewer rows still require the fixtures named in their Setup cells. At the end of the first pass, mixed image sequences, static browser behavior and independent viewers were still unrun; the expanded pass below covers many of those cases. Exact activity thresholds, the full shortcut exclusion matrix and remaining row-specific variants are still incomplete. Tests of export behavior do not substitute for those browser checks. All results remain agent-driven.

## Expanded static and public browser pass

A second disposable fixture used two painters and actual production `export_static.main()`, with Pillow available. Each history contained successful `canvas{}`, a red whole image (300×600), green crop (200×200, requested crop 100,100,300,300), gray value image, blue palette, yellow `reference/study.png`, purple whole image and orange whole image, followed by a bold closing title. The image colors are fixture identifiers, not simulated paintings. Source files contained `canvas{}` followed by `print("current file")`. The export reported two painters, 18 events and seven images per painter, with 14 web copies. It was served privately by Python's HTTP server on port 18767. Production public-mode studio served the same histories on port 18768. Neither server published anything externally.

### Images and replay

At latest, canvas zoom used `data/paint-studio-aaaaaa/img/6.png`; clicking the overlay closed it. Palette zoom used original `img/3.png`; Escape closed it. At `13/18`, the reference appeared beside the retained first whole image, its Reference label was visible and clicking it opened original `img/4.png`. At `9/18`, the value study appeared beside the retained whole image without the Reference label. At crop event `7/18`, the crop overlay was visible with a nonzero rectangle (`left:286.5px; top:21.25px; width:42.5px; height:42.5px` at the tested desktop layout).

Starting Replay from latest showed `5/18` and Pause. Opening canvas zoom there retained original `img/0.png` while replay continued. After replay completed, position was `18/18`, the Replay button returned and zoom still showed `img/0.png`. Thus zoom did not pause playback or follow later pictures. Pausing a fresh replay at `5/18` retained that position across subsequent tool calls lasting more than its one-second interval. Starting from the middle at `15/18` moved to `17/18`, the next whole look.

Previous-image controls traversed positions `17,15,13,11,9,7,5` out of 18. Next-image controls traversed `7,9,11,13,15,17,18`: crop, value, palette and reference were included, and stepping past the final image returned to the latest event.

### Static title, gallery, old link and refresh

Without `paintings.json`, the header used the closing title `Tall aaaaaa`. Adding a local gallery entry with title `Gallery override`, thumbnail `/data/paint-studio-aaaaaa/t/0.jpg` and URL `/finished-fixture`, then reloading, changed the header and exposed the finished link with that URL. The fixture does not contain a real finished destination page. The introduction stayed dismissed after reload; code returned hidden and speed was the default `1`.

An old `?s=/archive/--Users-fixture-src-a-paint-studio-aaaaaa--/s1.jsonl` address selected the owning painter and displayed `18/18` with its gallery title. This was an old-link mapping check, not access to that nonexistent session path.

While B was rewound at `17/18`, appended a pending paint call to its source history and re-exported. The exporter reported 19 events for B and zero new image copies. The page remained at its selected point. Resource timing showed B's event requests at navigation-relative `15203.9ms` and `76242.7ms`, about 61 seconds apart. After the later request, clicking LIVE displayed `19/19` and the appended paint action. This verifies static replacement becomes visible on the permitted refresh and does not force a rewound viewer to latest. Unchanged-metadata redisplay avoidance was not separately instrumented.

### Stream and narrow viewport

At desktop size, stream mode hid transport and finished link, retained the strip and placed a tall canvas beside its palette: whole rectangle x=12,width=326.09 and palette rectangle x=352.09,width=326.10. Keyboard Home/Enter in the picker switched A to B while retaining `stream=1`. The bracket shortcut then moved to `17/19 REWOUND` despite the hidden transport.

At the iPhone 12 Pro CSS viewport (390 pixels wide; desktop user agent unchanged), ordinary public mode had document scrollWidth 390 and a 236.2-pixel painter button. Stream mode had scrollWidth 459 and an 18-pixel painter button, with clipped/overlapping header text. [The narrow stream screenshot](studio-stream-narrow.png) records it. This is a reproduced layout limitation; whether broadcast mode must support phone-width screens is a product decision. Picker cards still expanded to the viewport width and keyboard selection remained available. This does not establish touch behavior on a real iPhone.

### Public mode, independent viewers and current source

The public browser hid the all-sessions control and listed the two painter cards. Opening an explicit `?s=/invalid/session.jsonl` address returned a plain `not found` HTTP 404 page. Appending a pending paint call while A was rewound retained the earlier point; returning to latest displayed `19/19`. Its result was then appended. At the live edge, showing code at desktop size displayed the actual current file (`canvas{}` and `print("current file")`), rather than asserting that file matched the tool-call history.

Two browser tabs opened the same public A painter. Rewinding and using the picker in one did not change the other tab's A query, `19/19` position or hidden-code state. The static and public fixture servers have independent origins; the ordinary preference observations above do not establish cross-origin preference sharing.
