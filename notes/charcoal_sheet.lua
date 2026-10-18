--@ engine 9
--@ paper
--@ width 2400
--@ chunk 1
paper_preset = "toned laid"
canvas{size=200, aspect=1, seed=219, paper={preset=paper_preset}}
materials = {
    {kind="vine",size="thin"}, {kind="vine",size="thick"},
    {kind="willow",size="thin"}, {kind="willow",size="thick"},
    {kind="compressed"}, {kind="pencil",grade="soft"},
    {kind="pencil",grade="medium"}, {kind="pencil",grade="hard"}
}
--@ chunk 2
for i, options in ipairs(materials) do
    local y = 75 + (i-1)*90
    local p = charcoal(options)
    p:stroke({{170,y-8},{220,y+6},{275,y-4}},{force={0.2,1.5,3},alt=60,roll={0,12,25},speed=60})
    if options.kind ~= "pencil" then p:snap(7) end
    for j=0,2 do
        p:stroke({{315,y-12+j*12},{550,y-12+j*12}},{force={0.15,0.7,1.8,4},alt=options.kind == "pencil" and 18 or 8,azimuth=90,speed=50,roll=j*11})
    end
    for j=0,2 do
        p:stroke({{605,y-12+j*12},{745,y-12+j*12}},{force=2.5,alt=options.kind == "pencil" and 18 or 8,azimuth=90,speed=50,roll=j*11})
        p:stroke({{805,y-12+j*12},{955,y-12+j*12}},{force=2.5,alt=options.kind == "pencil" and 18 or 8,azimuth=90,speed=50,roll=j*11})
    end
    smudge({{600,y},{760,y}},{force=1,pad=16,speed=40})
    erase(rect(861,y-33,37,66),{strength=1})
    erase(rect(861,y-33,37,66),{strength=1})
    erase(rect(861,y-33,37,66),{strength=1})
end
--@ chunk 3
for i=0,3 do
    charcoal_powder(rect(170+i*200,810,150,65),{depth=2,seed=219+i})
end
smudge(rect(370,810,150,65),{pad=4,pattern="strokes",passes=2,angle=0})
smudge(rect(570,810,150,65),{pad=100,pattern="strokes",passes=2,angle=0})
b=brush{kind="flat",width=90}
b:dust({{780,830},{910,830},{780,855},{910,855}},{tip=10})
--@ chunk 4
if paper_preset == "toned laid" then
    white = pastel(pile{{"lead white",1}})
    for j=0,3 do
        white:stroke({{200,930+j*7},{450,930+j*7}},{force={0.2,1,3},alt=18,azimuth=90,speed=60,roll=j*9})
    end
    dark = charcoal{kind="compressed"}
    dark:snap(8)
    dark:stroke({{600,945},{850,945}},{force=3,alt=8,azimuth=90})
    white:stroke({{620,945},{830,945}},{force=2.5,alt=25,azimuth=90})
end
