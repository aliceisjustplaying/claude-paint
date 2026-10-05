-- the sky: warm near the horizon, cooler above; a few close colors in long strokes, softened
local glowM = sky * rect(0, horizon - 200, 1000, 200):soften(90)
local highM = sky - rect(0, horizon - 150, 1000, 160):soften(90)
for _, p in ipairs({pile{{"lead white",4},{"barium yellow",0.7},{"vermilion",0.08}}, pile{{"lead white",4},{"carmine lake",0.4},{"barium yellow",0.2}}, pile{{"lead white",4},{"deep cadmium",0.25}}}) do
  work(glowM, {hand="body", pile=p, coverage=0.75, angle=0.01, length={50,130}, tool="filbert 14", streak=0.4, clip=sky})
end
for _, p in ipairs({pile{{"lead white",3},{"cobalt blue",0.7},{"carmine lake",0.25}}, pile{{"lead white",3},{"cobalt violet",0.8},{"cobalt blue",0.2}}, pile{{"lead white",3.5},{"cobalt blue",0.5},{"viridian",0.12}}}) do
  work(highM, {hand="body", pile=p, coverage=0.75, angle=0.03, length={60,150}, tool="filbert 16", streak=0.4, clip=sky})
end
blend(sky, {angle=0.02})
-- the poplars and bank, against the light: violet-blue with a green edge on the brush
local treeP = pile{{"ultramarine blue",1},{"carmine lake",0.55},{"viridian",0.35},{"lead white",0.7}}
local treeG = pile{{"viridian",1},{"cadmium yellow",0.35},{"carmine lake",0.15},{"lead white",0.5}}
work(treesM, {hand="body", pile=treeP, coverage=1.5, angle=math.pi/2, angle_jitter=0.12, length={20,60}, tool="filbert 6", second={pile=treeG, load=0.45, side=-1, share=0.45}, streak=0.5, clip=treesM:grow(2)})
work(bankM, {hand="body", pile=treeP, coverage=1.5, angle=0, length={20,50}, tool="filbert 7", second={pile=treeG, load=0.4, side=1, share=0.4}, clip=bankM:grow(2)})
-- the water: the sky mirrored, darker and cooler, in horizontal strokes
local nearM = water * rect(0, horizon + 140, 1000, 400):soften(80)
for _, p in ipairs({pile{{"lead white",2.5},{"cobalt blue",0.8},{"carmine lake",0.3}}, pile{{"lead white",3},{"carmine lake",0.45},{"barium yellow",0.15}}, pile{{"lead white",2.5},{"cobalt violet",0.8},{"cobalt blue",0.2}}}) do
  work(water, {hand="body", pile=p, coverage=0.7, angle=0, length={40,110}, tool="filbert 11", streak=0.4, clip=water})
end
work(nearM, {hand="body", pile=pile{{"lead white",1.5},{"cobalt blue",1},{"carmine lake",0.3},{"viridian",0.1}}, coverage=0.6, angle=0, length={40,120}, tool="filbert 12", broken=0.3, clip=water})
-- reflections of the poplars: short horizontal strokes, columns broken by ripples
work(reflM:map(function(v) return v > 0.12 and v or 0 end), {hand="body", pile=pile{{"ultramarine blue",1},{"carmine lake",0.5},{"viridian",0.3},{"lead white",1.4}}, coverage=1.1, angle=0, length={8,22}, tool="filbert 5", broken=0.4, second={pile=treeG, load=0.3, streak=0.9}, clip=water})
print(wait(4*24*60))
