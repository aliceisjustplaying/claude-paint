-- Round 6 lab, subject "rock": one granite boulder sitting on a heath in afternoon daylight,
-- with its cast shadow. Shared setup for rock_A (old way) and rock_B (new way): canvas,
-- palette, sky and ground colors, the rock drawn and inferred, its light and shadow.
canvas{style="friedrich", aspect=3/2, size=300, seed=71}
HZ = 150
-- the sun from the upper left, a little in front: an afternoon light, the shadow falling right
SUN = {from={-1, -0.55}, front=0.35, ambient=0.25, bounce=0.3, bounce_from={0.2, 1, 0.3}}
sky = function(x, y) return gradient({{0, "#8395ad"}, {0.7, "#b7bfc2"}, {1, "#cfccbd"}}, y / HZ) end
skypal = pal:only{"lead white", "pale smalt", "cobalt blue", "yellow ochre", "raw umber", "red earth"}
-- the heath: far and pale at the horizon, near and darker, warm, with a slow patchiness
gn = noise{seed=72, octaves=4, period=140}
groundcol = function(x, y)
  local t = smoothstep(HZ, H, y)
  local c = mix(mix("#9c9a82", "#7d7a58", smoothstep(0, 0.35, t)), "#5f5a3c", smoothstep(0.35, 1, t))
  return shift(c, 0.03 * gn(x, y), 0.004 * gn(x + 90, y), 0.01 * gn(x, y + 70))
end
land = below(function(x) return HZ + 2 * math.sin(x / 60) end)
-- the boulder, drawn lopsided: a high shoulder left, a broken back falling to the right, a flat
-- foot sunk in the heath; one crack running off the shoulder at an angle
BOULDER = {{258,520,"c"},{262,470},{286,402,"c"},{334,338},{398,300,"c"},{470,282},{528,296,"c"},
           {586,318},{640,352,"c"},{700,392},{742,440,"c"},{758,488},{752,528,"c"},{640,540},{470,544},{350,536}}
CRACK = {{470,286},{492,340},{506,392},{500,450}}
boulder = outline{pts=BOULDER, char="broken", seed=73}
rk = rock{outline=boulder, cracks={CRACK}, kind="granite", sun=SUN, seed=74}
print(rk)
GRANITE = {core="#34322f", shadow="#57534d", half="#86807a", light="#aea79a", top="#cdc6b6", bounce="#7a6c5c", crevice="#2e2c2f"}
SHADOWCOL = "#3d3a30"
