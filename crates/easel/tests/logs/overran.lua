-- easel session "overran": a painting replayed chunk by chunk.
--   easel run paintings/lua/overran.lua [--width 3200]
-- Each "--@ chunk" line starts one chunk as it was run at the easel (clock = painting minutes).

--@ chunk 1 · clock 0
-- a log from before sittings were enforced: hand time on, a sitting planned
-- at 12 minutes that runs far over it (as Evening at a Mountain Lake's did)
canvas{style="friedrich", aspect=1.4, seed=7, hand=true}; sitting{hours=0.2}

--@ chunk 2 · clock 0
work(rect(0, 0, 1000, 460), {hand="broad", color="#8c9cb0", angle=0, coverage=3})
local t = timesheet() print(string.format("sky: sitting %d %.3f of %.3f h", t.sittings, t.sitting / 60, t.hours))

--@ chunk 3 · clock 0
b = brush("round", 3); b:load("#2b2620", 0.9)
for i = 1, 40 do b:stroke({{100 + 20 * i, 520}, {110 + 20 * i, 460}}) end
sitting{hours=0.5}
stipple(rect(0, 380, 1000, 120), {width=3, color="#cfccc2", coverage=1.5})
local t = timesheet() print(string.format("clock %.4f sitting %d %.4f of %.3f h", t.clock, t.sittings, t.sitting, t.hours))

--@ chunk 4 · clock 0
rest(2)
b:load("#6a5040", 0.9); b:stroke({{200, 300}, {600, 320}})
print(string.format("clock %.4f sittings %d", clock(), timesheet().sittings))
