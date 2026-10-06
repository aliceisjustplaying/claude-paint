-- easel session "scratch": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ box every
--@ engine 5

--@ chunk 1
canvas{aspect=1.3, ground={{apply="knife", pile={{"lead white", 6}, {"yellow ochre", 1}, {"red earth", 0.4}}, texture=0.4, um=120}}, linen={18, 16}, seed=417, size=650}

--@ chunk 2
sk_gold = pile{{"lead white",2},{"cadmium yellow",1},{"Naples yellow",1},{"orange chrome",0.3}, name="sk_gold"}
tr_dark = pile{{"ultramarine blue",1},{"cobalt violet",0.8},{"permanent alizarin",0.25},{"lead white",0.8},{"burnt sienna",0.2}, name="tr_dark"}
tr_light = pile{{"lead white",2.5},{"cobalt violet",1},{"rose madder",0.3},{"cobalt blue",0.3}, name="tr_light"}
pop_c = pile{{"cobalt violet",1},{"ultramarine blue",0.5},{"lead white",1.2},{"rose madder",0.25}, name="pop_c"}
pop_far = pile{{"lead white",2.5},{"cobalt violet",1},{"rose madder",0.3},{"Naples yellow",0.4}, name="pop_far"}
work(rect(50,250,700,200), {hand="broad", pile=sk_gold, angle=0, coverage=2.0, fill=true})
print(wait(5*24*60))
local function poplar(cx, top, base, w, seed)
  local pts = {{cx-w*0.15, base},{cx-w*0.5, base-(base-top)*0.35},{cx-w*0.42, base-(base-top)*0.7},{cx-w*0.12, top+4},{cx, top},{cx+w*0.14, top+6},{cx+w*0.45, base-(base-top)*0.65},{cx+w*0.5, base-(base-top)*0.3},{cx+w*0.2, base}}
  return poly(pts, true):roughen(2.5, 9, seed)
end
tp = nil
for i,cx in ipairs({150,300,450,600}) do local m = poplar(cx, 280, 420, 70, 300+i); tp = tp and (tp+m) or m end
work(tp, {hand="body", piles={{tr_dark,function() return 0.3 end},{pop_c,function() return 0.8 end},{pop_far,function() return 0.6 end}}, angle=1.57, angle_jitter=0.3, coverage=2.5, fill=true, length={6,14}, tool={kind="filbert", width=4}, edge={found=0.5, soft=0.5, period=15}, seed=211})
print(wait(60*24*60))
gl_sky = pile{{"Indian yellow",1},{"rose madder",0.25},{"Mars orange",0.15}, medium=0.8, name="gl_sky"}
local m = rect(50,250,700,200)
work(m, {hand="glaze", pile=gl_sky, coverage=1.0, threshold=0.2, load_at=function() return 0.3 end, angle=0.0, tool={kind="filbert", width=18, stiffness=0.3}, length={100,240}, seed=421})
blend(m, {angle=0.0, threshold=0.2})
print(wait(60*24*60))

--@ chunk 3
pop_flow = pile{{"cobalt violet",1},{"ultramarine blue",0.3},{"lead white",1.6},{"rose madder",0.25},{"Naples yellow",0.15}, medium=0.65, name="pop_flow"}
gl_vio = pile{{"cobalt violet",1},{"ultramarine blue",0.3},{"rose madder",0.2}, medium=0.8, name="gl_vio"}
local A = tp * rect(100,250,100,180)
local B = tp * rect(250,250,100,180)
local C = tp * rect(400,250,100,180)
work(A, {hand="detail", pile=pop_flow, coverage=3, fill=true, angle=1.57, tool={kind="filbert", width=6, lay=2}, clip=A, seed=1})
work(B, {hand="body", piles={{tr_dark,function() return 0.3 end},{pop_c,function() return 0.8 end},{pop_far,function() return 0.6 end}}, angle=1.57, angle_jitter=0.3, coverage=1.2, length={6,14}, tool={kind="filbert", width=4}, load=0.5, pressure={0.3,0.6}, clip=B, seed=2})
work(C, {hand="glaze", pile=gl_vio, coverage=1.0, load_at=function() return 0.35 end, angle=1.57, tool={kind="filbert", width=10, stiffness=0.3}, length={30,80}, clip=C, seed=3})
print(wait(40*24*60))
