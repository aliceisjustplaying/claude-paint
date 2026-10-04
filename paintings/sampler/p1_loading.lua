-- Loading the brush
B  = pile{{"lead white",2},{"cobalt blue",1}}
Pk = pile{{"lead white",3},{"carmine lake",1}}
G  = pile{{"viridian",1},{"cadmium yellow",1.5},{"lead white",1}}
Wt = pile{{"lead white",8},{"barium yellow",0.3}}
V  = pile{{"ultramarine blue",1},{"carmine lake",0.7},{"lead white",1.5}}
Y  = pile{{"cadmium yellow",1},{"lead white",1}}
local b = brush("filbert", 16)
local function row(y, setup) for i = 0, 2 do setup(); b:stroke({{40 + i*150, y + 6}, {110 + i*150, y - 4}, {170 + i*150, y + 4}}, {pressure={0.95, 0.6}}) end end
row(60,  function() b:reload(B, 0.9) end)
row(150, function() b:reload(B, 0.8); b:load(Pk, 0.6, {side=1, share=0.45}) end)
row(240, function() b:reload(G, 0.7); b:load(Wt, 0.5, {streak=0.9}) end)
row(330, function() b:reload(V, 0.7); b:load(B, 0.5, {side=-1, share=0.4}); b:load(Wt, 0.35, {streak=1}) end)
-- wet into wet: a wet yellow field, blue strokes dragged through it
work(rect(40, 400, 440, 210), {hand="body", pile=Y, coverage=2, angle=0, fill=true})
for i = 0, 4 do b:reload(B, 0.7); b:stroke({{50, 420 + i*40}, {250, 412 + i*40}, {470, 426 + i*40}}, {pressure={0.9, 0.6}}) end
-- the same in passes: plain, streaky, double-loaded
local m = function(y) return rect(540, y, 420, 170) end
work(m(40),  {hand="body", pile=B, coverage=1.6, angle=0, length={25,60}, tool="filbert 9"})
work(m(240), {hand="body", pile=B, coverage=1.6, angle=0, length={25,60}, tool="filbert 9", streak=0.8})
work(m(440), {hand="body", pile=B, coverage=1.6, angle=0, length={25,60}, tool="filbert 9", streak=0.6, second={pile=Pk, load=0.5, side=1, share=0.45}})
print(wait(60))
