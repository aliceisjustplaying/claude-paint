-- Round 6 lab 2, subject "rock": a weathered granite erratic in a summer meadow, afternoon light
-- from the left and a little in front, its shadow falling right. Shared setup for rock_B (round 1's
-- rock B recipe) and rock_H (hierarchical detail): canvas, palette, sky and meadow colors, the
-- rock drawn and inferred.
canvas{style="friedrich", aspect=3/2, size=300, seed=81}
HZ = 205
SUN = {from={-1, -0.7}, front=0.35, ambient=0.25, bounce=0.3, bounce_from={0.2, 1, 0.3}}
sky = function(x, y) return gradient({{0, "#7f95b3"}, {0.65, "#b4bfc6"}, {1, "#d8d2bc"}}, y / HZ) end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the meadow: pale and blue-gray at the horizon, greener in the middle, darker and cooler near,
-- with a slow patchiness
gn = noise{seed=82, octaves=4, period=150}
groundcol = function(x, y)
  local t = smoothstep(HZ, H, y)
  local c = mix(mix("#98a088", "#76804f", smoothstep(0, 0.35, t)), "#56613a", smoothstep(0.35, 1, t))
  return shift(c, 0.03 * gn(x, y), -0.004 * gn(x + 90, y), 0.01 * gn(x, y + 70))
end
land = below(function(x) return HZ + 1.5 * math.sin(x / 70) end)
-- the erratic: a rounded crown over a broad weathered face turned to the sun, the foot tucked under
-- on the left, and on the right a fracture face (in shadow) whose upper edge falls in a straight
-- run to a blunt corner, with one crack down it
ERRATIC = {{300,552,"c"},{284,522},{280,482,"c"},{294,428},{316,380},{352,334,"c"},{404,302,"c"},{468,282},
           {528,288},{590,302,"c"},{648,336},{704,370,"c"},{726,408},{736,450,"c"},{726,478,"c"},{738,512},
           {742,550,"c"},{640,558},{500,561},{380,558}}
-- the arris between the lit face and the fracture face, and the brow between the crown and the face
ARRIS = {{590,302},{572,370},{566,440},{570,510},{576,556}}
BROW = {{352,336},{420,322},{500,318},{560,314},{590,304}}
CRACK = {{648,340},{658,400},{650,462},{660,522}}
erratic = outline{pts=ERRATIC, char="broken", seed=83}
rk = rock{outline=erratic, planes={ARRIS, BROW}, cracks={CRACK}, kind="granite", sun=SUN, seed=84}
print(rk)
