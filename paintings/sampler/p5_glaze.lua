-- Glaze and scumble over dry paint
local Lt = pile({{"lead white",6},{"barium yellow",0.4}, blot=0.3})
local Dk = pile{{"ultramarine blue",2},{"viridian",1},{"carmine lake",0.4},{"lead white",0.4}}
work(rect(30, 30, 620, 600), {hand="body", pile=Lt, coverage=2.2, angle=0.3, length={20,50}, load=1.0, tool={kind="filbert", width=11, lay=5}})
work(rect(680, 30, 290, 600), {hand="body", pile=pile({{"ultramarine blue",2},{"viridian",1},{"carmine lake",0.4},{"lead white",0.4}, blot=0.3}), coverage=2.2, angle=0.2, length={25,60}, load=1.0, tool={kind="filbert", width=11, lay=5}})
print(wait(14*24*60))
local G1 = pile({{"carmine lake",1}, medium=0.8})
local G2 = pile({{"viridian",1}, medium=0.8})
local G3 = pile({{"ultramarine blue",1}, medium=0.8})
for i, g in ipairs({G1, G2, G3}) do work(rect(30 + (i-1)*210, 30, 200, 600), {hand="glaze", pile=g, coverage=1.2, angle=math.pi/2, clip=rect(30 + (i-1)*210, 30, 200, 600)}) end
local S = pile{{"lead white",6},{"cobalt blue",0.5},{"carmine lake",0.15}}
-- a dry, half-empty brush of light paint dragged over the dried texture: it catches the ridges
work(rect(680, 200, 290, 260), {hand="body", pile=S, coverage=0.9, angle=0.1, length={60,140}, load=0.25, pressure={0.15, 0.3}, tool={kind="filbert", width=16, stiffness=0.9}, clip=rect(680, 30, 290, 600)})
print(wait(60))
