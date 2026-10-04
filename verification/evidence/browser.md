# Studio browser observations

2026-10-04, T3 collaborative browser, 1280 × 800. Source commit `4e525e50897807e9b5f734071dfeb330f1a393d3`. The server runs the current source against synthetic pi records containing two PNGs from the isolated CLI painting. This is not a live model session or a human verification pass.

URL: [private preview](http://<tailscale-host>:18765/?p=paint-studio-decafe). HTTP GET returned 200 using the serving machine's Tailscale IP. Browser navigation loaded the studio. Access from the other Mac was not tested. The server remains in exec session 72568; no existing service or Serve route was changed.

| Action | Observation | Result |
|---|---|---|
| Load fixture | Untitled, in progress; documentation-fixture; LIVE; 9/9 events | pass |
| Open painter picker | Dialog showed Painters, all sessions, close and the fixture card | pass |
| Escape | Picker dismissed | pass |
| Previous look | REWOUND, 6/9 events, earlier look | pass |
| Toggle code | CODE and painting.lua displayed reconstructed first chunk | pass |
| LIVE | Returned to latest event | pass |
| Replay | REPLAY, Pause control and earlier look displayed | pass |
| Pause then LIVE | Returned to latest view | pass |

[Replay screenshot](studio-replay.png) records the replay/code state. The fixture's code text is abbreviated for display and is not a runnable painting log; [probe.lua](probe.lua) is the actual successful log.

Not covered: real provider streaming, multiple painters or sittings, palette/crop image classification, zoom, mobile layout, static export, public mode, stream layout, network outage or late-arriving records.
