-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.
--@ engine 3

--@ chunk 1
-- Sitting 1. Ground and canvas.
canvas{size=400, aspect=1.25, linen={16, 16}, seed=7,
  ground={{pile={{"lead white", 3}, {"raw umber", 1.1}, {"yellow ochre", 0.6}}, um=38, apply="knife", texture=0.3}}}

-- brushes kept as globals for the rest of the painting
bBroad   = brush("filbert", 22)
bBody    = brush("filbert", 9)
bSoft    = brush("filbert", 26, 0.4)
bRound   = brush{kind="round", width=6, point=0.9, stiffness=0.55}
bRigger  = brush{kind="rigger", width=2.4, point=1, stiffness=0.5}
bBlender = brush("badger", 40)

print(W, H, "brushes ready")

--@ chunk 2
-- layout: jug left of centre, orange and folded cloth to the right
hPen = pencil("HB")
hp2   = pencil("H")

-- table's back edge, a touch below the middle
hPen:line({{-20, 468}, {300, 462}, {640, 470}, {1020, 464}}, {pressure=0.3})
hp2:hatch(below(function(x) return 462 + 6*math.sin(x/180) end), {angle=-0.06, pressure=0.12, spacing=9, length=22})

-- jug: belly, shoulder, neck, rim
hPen:line({{262, 572}, {258, 520}, {268, 460}, {292, 408}, {316, 378}, {322, 344},
           {318, 320}, {312, 296}, {316, 276}, {340, 268}, {378, 268}, {400, 278},
           {400, 300}, {394, 326}, {396, 352}, {408, 384}, {432, 416}, {446, 462},
           {448, 522}, {440, 572}}, {pressure={0.35, 0.3, 0.4}})

-- handle off the right shoulder
hPen:line({{392, 306}, {432, 318}, {452, 352}, {448, 392}, {420, 412}}, {pressure=0.32})

-- base line of the jug
hPen:line({{248, 574}, {340, 578}, {444, 574}}, {pressure=0.35})

-- orange
hPen:line({{588, 592}, {590, 566}, {608, 548}, {638, 542}, {668, 550}, {682, 572},
           {682, 598}, {666, 620}, {638, 628}, {608, 620}, {590, 600}},
          {pressure=0.35})

-- folded cream cloth: a low mound with a few fold ridges
hPen:line({{470, 618}, {492, 566}, {546, 526}, {624, 508}, {712, 512}, {778, 534},
           {818, 574}, {826, 620}, {812, 646}, {700, 660}, {566, 656}, {484, 644}},
          {pressure=0.3})
hPen:line({{512, 610}, {566, 552}, {648, 528}}, {pressure=0.25})
hPen:line({{628, 646}, {676, 578}, {748, 556}}, {pressure=0.25})
hPen:line({{744, 524}, {768, 578}, {800, 610}}, {pressure=0.25})

-- the group's shadow, thrown to the right
hPen:hatch(poly{{440, 570}, {560, 578}, {840, 640}, {700, 690}, {470, 640}},
           {angle=0.16, pressure=0.1, spacing=14, length=40})
print("layout drawn")

--@ chunk 3
-- palette, mixed once and kept in globals
pWall   = pile{{"lead white", 4}, {"smalt", 1.6}, {"raw umber", 1.0}, {"yellow ochre", 0.4}}
pWallD  = pile{{"smalt", 1.4}, {"raw umber", 1.8}, {"lead white", 1.2}, medium=0.15}
pWood   = pile{{"raw umber", 3}, {"vermilion", 1.2}, {"bone black", 0.8}, {"yellow ochre", 0.6}}
pWoodD  = pile{{"raw umber", 2}, {"bone black", 1.5}, {"vermilion", 0.5}}
pWoodL  = pile{{"raw umber", 1.2}, {"vermilion", 1.2}, {"yellow ochre", 0.8}, {"lead white", 0.5}}
pCast   = pile{{"raw umber", 1.6}, {"bone black", 1.3}, {"cobalt blue", 0.4}}
pJug    = pile{{"cobalt blue", 2}, {"smalt", 1.2}, {"lead white", 1.0}}
pJugL   = pile{{"cobalt blue", 1}, {"smalt", 0.8}, {"lead white", 3}}
pJugD   = pile{{"Prussian blue", 1.2}, {"cobalt blue", 1.5}, {"raw umber", 0.8}, medium=0.1}
pCloth  = pile{{"lead white", 4}, {"yellow ochre", 0.8}, {"raw umber", 0.25}}
pClothS = pile{{"lead white", 1.6}, {"raw umber", 0.9}, {"smalt", 0.3}, {"yellow ochre", 0.4}}
pOrange = pile{{"vermilion", 3}, {"chrome yellow", 1.6}, {"lead white", 0.6}, {"red earth", 0.4}}

tableEdge = function(x) return 464 + 5*math.sin(x/200) end
mWall = above(tableEdge)
dark = mask(function(x, y)
  return clamp((x - 180)/620 * 0.8 + (y - 60)/420 * 0.45, 0, 1) end)
mWallDark = ((mWall * dark):blur(26))

work(mWall, {hand="broad", tool="filbert 22", pile=pWall, coverage=1, angle=1.5, edge="soft"})
work(mWallDark, {hand="broad", tool="filbert 22", pile=pWallD, coverage=1, angle=1.5,
                 edge="lost", clip=true, pressure={0.5, 0.9}})
print("wall laid in")

--@ chunk 4
-- the wall wants to be a quiet MID grey; the ground proved lighter than expected
pWall2  = pile{{"smalt", 1.6}, {"raw umber", 2.2}, {"lead white", 0.8}}
pWall2L = pile{{"smalt", 1.2}, {"raw umber", 1.3}, {"lead white", 1.5}}

mWallLit = ((mWall * mask(function(x, y)
  return clamp(1.15 - (x - 60)/620 - (y - 20)/430, 0, 1) end)):blur(30))

work(mWall, {hand="broad", tool="filbert 22", pile=pWall2, coverage=1.3, angle=1.5,
             edge="soft", pressure={0.6, 1.0}})
work(mWall, {hand="broad", tool="filbert 22", pile=pWall2, coverage=0.5, angle=-0.4,
             edge="soft", pressure={0.5, 0.9}})
work(mWallLit, {hand="broad", tool="filbert 22", pile=pWall2L, coverage=1, angle=1.5,
                edge="lost", clip=true, pressure={0.5, 0.9}})
print("wall darkened, light side upper left")

--@ chunk 5
pWallG  = pile{{"smalt", 1.5}, {"raw umber", 2.0}, {"lead white", 0.9}, medium=0.35}
mTable  = below(tableEdge)

-- 1. close the wall up so no ground shows between strokes
work(mWall, {hand="broad", tool="filbert 22", pile=pWall2, coverage=2.8, angle=1.4,
             edge="soft", pressure={0.8, 1.0}, fill=true})
work(mWall, {hand="glaze", tool="filbert 26", pile=pWallG, coverage=1.2, edge="soft"})

-- 2. the dark wooden table
work(mTable, {hand="broad", tool="filbert 22", pile=pWood, coverage=2.4, angle=-0.05,
              edge="found", pressure={0.7, 1.0}, fill=true, clip=true})

-- table light at the back left, falling to dark at the front right
mTableLit = ((mTable * mask(function(x, y)
  return clamp(1.25 - (x - 40)/700 - (y - 430)/420, 0, 1) end)):blur(24))
mTableDk  = ((mTable * mask(function(x, y)
  return clamp((x - 380)/560 * 0.9 + (y - 520)/300 * 0.5 - 0.25, 0, 1) end)):blur(26))
work(mTableLit, {hand="broad", tool="filbert 22", pile=pWoodL, coverage=1, angle=-0.05,
                 edge="lost", clip=true, pressure={0.5, 0.9}})
work(mTableDk, {hand="broad", tool="filbert 22", pile=pWoodD, coverage=1.2, angle=-0.05,
                edge="lost", clip=true, pressure={0.6, 1.0}})
print("wall closed, table laid in")

--@ chunk 6
-- the table read as terracotta; a darker, browner wood, and the shadows it throws
pWood2 = pile{{"raw umber", 4}, {"bone black", 1.3}, {"vermilion", 0.5}, {"yellow ochre", 0.3}}
pWood3 = pile{{"raw umber", 2}, {"yellow ochre", 0.6}, {"bone black", 0.5}, {"lead white", 0.25}}

work(mTable, {hand="broad", tool="filbert 22", pile=pWood2, coverage=2.6, angle=-0.05,
              edge="found", pressure={0.75, 1.0}, fill=true, clip=true})
work(mTableLit, {hand="broad", tool="filbert 22", pile=pWood3, coverage=0.8, angle=-0.05,
                 edge="lost", clip=true, pressure={0.4, 0.8}})
work(mTableDk, {hand="glaze", tool="filbert 26", pile=pWoodD, coverage=1, edge="lost", clip=true})

-- cast shadows: light from upper left, so everything throws to the lower right
mCast = (ellipse(430, 604, 190, 34) + ellipse(676, 634, 210, 40)):blur(20)
work(mCast * mTable, {hand="broad", tool="filbert 22", pile=pCast, coverage=1,
                      edge="lost", clip=true, pressure={0.5, 0.95}})
mContact = ellipse(352, 580, 108, 13):blur(6)
work(mContact, {hand="body", pile=pCast, coverage=1.4, edge="soft"})
print("wood corrected, cast shadows in")

--@ chunk 7
clothPts = {{470,618},{492,566},{546,526},{624,508},{712,512},{778,534},
            {818,574},{826,620},{812,646},{700,660},{566,656},{484,644}}
mCloth = poly(clothPts, true)

-- the mass of the cloth
work(mCloth, {hand="broad", tool="filbert 22", pile=pCloth, coverage=2.2, angle=-0.5,
              edge="soft", pressure={0.75, 1.0}, fill=true})

-- fold planes: each ridge throws a soft band to its right
f1 = ribbon({{518, 606}, {578, 552}, {652, 532}}, 30)
f2 = ribbon({{638, 644}, {686, 580}, {754, 558}}, 26)
f3 = ribbon({{750, 526}, {776, 580}, {806, 612}}, 20)
fFront = ribbon({{500, 642}, {660, 658}, {802, 646}}, 24)
mClothDk = (((f1 + f2 + f3 + fFront):blur(7)) * mCloth)
work(mClothDk, {hand="body", pile=pClothS, coverage=1.8, angle=-0.5,
                edge="lost", clip=true, pressure={0.5, 0.9}})

-- the near end sits in the jug's shadow
mClothLeft = ((mCloth * ellipse(492, 590, 120, 90)):blur(14))
work(mClothLeft, {hand="glaze", pile=pClothS, coverage=1, edge="lost", clip=true})
print("cloth in")

--@ chunk 8
pCloth2 = pile{{"lead white", 3}, {"yellow ochre", 1.4}, {"raw umber", 0.35}}
pClothD = pile{{"lead white", 0.8}, {"raw umber", 1.8}, {"smalt", 0.5}, {"yellow ochre", 0.3}}

-- warmer, more solid cloth
work(mCloth, {hand="broad", tool="filbert 22", pile=pCloth2, coverage=1.8, angle=-0.5,
              edge="soft", pressure={0.7, 1.0}, fill=true})
-- deeper fold shadows
work(mClothDk, {hand="body", pile=pClothD, coverage=2.2, angle=-0.5,
                edge="lost", clip=true, pressure={0.6, 1.0}})
-- the light catching the near side of each ridge
l1 = ribbon({{504, 600}, {564, 546}, {638, 526}}, 22)
l2 = ribbon({{624, 638}, {672, 574}, {740, 552}}, 20)
l3 = ribbon({{738, 522}, {764, 576}, {794, 608}}, 14)
mClothLit = (((l1 + l2 + l3) * mCloth):blur(6))
work(mClothLit, {hand="body", pile=pCloth2, coverage=1.4, angle=-0.5,
                 edge="lost", clip=true, pressure={0.3, 0.6}})
print("cloth warmed, folds cut")

--@ chunk 9
pClothD2 = pile{{"raw umber", 2.5}, {"smalt", 0.8}, {"bone black", 0.4}, {"lead white", 0.4}}
pOrangeL = pile{{"vermilion", 1}, {"chrome yellow", 1.2}, {"lead white", 2.5}}
pOrangeD = pile{{"red earth", 2}, {"vermilion", 1}, {"raw umber", 1}}

-- cloth redrawn with corners instead of an oval; lift the fuzzy fringe first
mCloth2 = poly{{468,620},{486,570},{520,540},{566,514},{630,506},{700,512},
               {760,528},{806,556},{826,596},{822,632},{790,652},{700,662},
               {590,660},{500,648}}
mFringe = ((mCloth2:grow(26) - mCloth2:shrink(4)):blur(6))
rCloth = rag()
print(rCloth:wipe(mFringe, {pressure=0.55, passes=2, angle=0.4}))

work(mCloth2, {hand="broad", tool="filbert 22", pile=pCloth2, coverage=2, angle=-0.5,
               edge="found", pressure={0.75, 1.0}, fill=true})
mClothDk2 = ((f1 + f2 + f3 + fFront):blur(4) * mCloth2)
work(mClothDk2, {hand="body", pile=pClothD2, coverage=2.2, angle=-0.5,
                 edge="soft", clip=true, pressure={0.6, 1.0}})
mClothLit2 = ((l1 + l2 + l3) * mCloth2):blur(5)
work(mClothLit2, {hand="body", pile=pCloth2, coverage=1.3, angle=-0.5,
                  edge="lost", clip=true, pressure={0.3, 0.6}})

-- the orange, sitting on the cloth
mOr = ellipse(635, 585, 47, 43)
mOrSh = ((ellipse(708, 606, 64, 20) * mCloth2):blur(8))
work(mOrSh, {hand="glaze", pile=pClothD2, coverage=1, edge="lost", clip=true})
work(mOr, {hand="body", tool="filbert 9", pile=pOrange, coverage=2.4,
           edge="found", fill=true, pressure={0.7, 1.0}})
mOrCore = ((mOr * mask(function(x, y) return clamp((x - 592)/58, 0, 1)^1.4 end)):blur(8))
work(mOrCore, {hand="body", pile=pOrangeD, coverage=1.6, edge="soft"})
mOrLit = ((mOr * ellipse(612, 561, 26, 20)):blur(7))
work(mOrLit, {hand="body", pile=pOrangeL, coverage=1.2, edge="lost", pressure={0.4, 0.8}})
mOrBounce = ((mOr * ellipse(638, 622, 32, 13)):blur(5))
work(mOrBounce, {hand="glaze", pile=pOrangeL, coverage=0.8, edge="lost"})
print("cloth reshaped, orange in")

--@ chunk 10
-- jug colours re-mixed under fresh names (the palette had filled up)
pJugM  = pile{{"cobalt blue", 2.2}, {"smalt", 1.4}, {"lead white", 0.9}}
pJugLt = pile{{"cobalt blue", 0.8}, {"pale smalt", 1.2}, {"lead white", 3}}
pJugDk = pile{{"Prussian blue", 1.4}, {"cobalt blue", 1.6}, {"raw umber", 0.8}, medium=0.1}
pJugW  = pile{{"cobalt blue", 1}, {"vermilion", 0.35}, {"lead white", 1.3}, medium=0.25}

jugPts = {{262,572},{258,520},{268,460},{292,408},{316,378},{322,344},{318,320},
          {312,296},{316,276},{340,268},{378,268},{400,278},{400,300},{394,326},
          {396,352},{408,384},{432,416},{446,462},{448,522},{440,572}}
mJugBody = poly(jugPts, true)
hPts = {{398,308},{444,320},{460,356},{448,398},{424,412}}
mJug = mJugBody + ribbon(hPts, 13)

-- mass
work(mJug, {hand="body", tool=bBody, pile=pJugM, coverage=2.6, angle=1.2,
            edge="found", fill=true, clip=true, pressure={0.7, 1.0}})
-- core shadow down the right
mJugDkM = ((mJug * mask(function(x, y) return clamp((x - 356)/96, 0, 1)^1.3 end)):blur(9))
work(mJugDkM, {hand="body", pile=pJugDk, coverage=1.9, angle=1.2,
               edge="soft", clip=true, pressure={0.5, 1.0}})
-- broad light on the upper left
mJugLtM = ((mJug * (ellipse(322, 388, 62, 128) + ellipse(330, 520, 54, 58))):blur(14))
work(mJugLtM, {hand="body", pile=pJugLt, coverage=1.3, angle=1.2,
               edge="lost", clip=true, pressure={0.35, 0.8}})
-- warm bounce up the lower right off the table and the orange
mJugW = ((mJug * mask(function(x, y)
  return clamp((x - 330)/70, 0, 1) * clamp((y - 430)/140, 0, 1) end)):blur(11))
work(mJugW, {hand="glaze", pile=pJugW, coverage=1, edge="lost", clip=true})
print("jug massed and modelled")

--@ chunk 11
-- close the jug up solid first, then model it again on an even film
work(mJug, {hand="body", tool=bBody, pile=pJugM, coverage=2.8, angle=1.2,
            edge="found", fill=true, clip=true, pressure={0.75, 1.0}})
work(mJugDkM, {hand="body", pile=pJugDk, coverage=2.4, angle=1.2,
               edge="soft", fill=true, clip=true, pressure={0.6, 1.0}})
work(mJugLtM, {hand="body", pile=pJugLt, coverage=1.6, angle=1.2,
               edge="lost", fill=true, clip=true, pressure={0.4, 0.9}})
work(mJugW, {hand="glaze", pile=pJugW, coverage=1.2, edge="lost", clip=true})

-- the bright edge of the belly, and the reflected warmth back into it
mJugHi = ((mJug * ellipse(272, 462, 15, 82)):blur(5))
work(mJugHi, {hand="detail", tool=bRound, pile=pJugLt, coverage=2,
              edge="lost", clip=true, pressure={0.5, 1.0}})

-- handle: light on top, dark on the far side
mJugHandle = ribbon(hPts, 13)
mHTop = ((mJugHandle * ribbon({{404,306},{444,318},{458,352}}, 7)):blur(4))
mHDk  = ((mJugHandle * mask(function(x, y) return clamp((x - 428)/32, 0, 1) end)):blur(4))
work(mHTop, {hand="body", pile=pJugLt, coverage=1.6, edge="soft", clip=true})
work(mHDk,  {hand="body", pile=pJugDk, coverage=1.8, edge="soft", clip=true})

-- the rim: lit top plane, dark line under the lip
mRim  = ellipse(358, 277, 42, 10)
mRimU = ellipse(358, 288, 39, 6):blur(3)
work(mRim,  {hand="body", pile=pJugLt, coverage=2, edge="soft", clip=mJug, pressure={0.5, 1.0}})
work(mRimU, {hand="body", pile=pJugDk, coverage=2, edge="soft", clip=mJug})

-- the jug sits on the table: the darkest darks at the foot
mFoot = ((mJug * ellipse(352, 574, 112, 16)):blur(6))
work(mFoot, {hand="glaze", pile=pJugDk, coverage=1.4, edge="lost", clip=true})
mContact2 = (ellipse(354, 580, 108, 10)):blur(4)
work(mContact2, {hand="body", pile=pWoodD, coverage=1.6, edge="soft", fill=true})
print("jug rebuilt solid")

--@ chunk 12
-- sinking the jug's value and putting the warmth back into the cloth's folds
pJugDg  = pile{{"Prussian blue", 1.5}, {"cobalt blue", 1.0}, {"raw umber", 0.6}, medium=0.35}
pClothW = pile{{"raw umber", 2.2}, {"yellow ochre", 0.5}, {"bone black", 0.3},
               {"lead white", 0.5}, medium=0.15}

-- jug: glaze the whole form down, then bring the light back up on the left
work(mJug, {hand="glaze", pile=pJugDg, coverage=1.3, edge="found", clip=true})
work(mJugDkM, {hand="glaze", pile=pJugDk, coverage=1.5, edge="soft", clip=true})
work(mJugLtM, {hand="body", pile=pJugLt, coverage=1.2, edge="lost",
               fill=true, clip=true, pressure={0.45, 0.95}})
work(mJugHi, {hand="detail", tool=bRound, pile=pJugLt, coverage=1.6,
              edge="lost", clip=true, pressure={0.6, 1.0}})

-- cloth: warm, solid fold shadows instead of cold scribble
work(mClothDk2, {hand="body", pile=pClothW, coverage=1.8, angle=-0.5,
                 edge="lost", fill=true, clip=true, pressure={0.5, 0.95}})

-- the orange's own contact on the cloth, and the cloth's on the wood
mOrFoot = (ellipse(642, 624, 46, 10)):blur(4)
work(((mOrFoot * mCloth2)), {hand="body", pile=pClothW, coverage=1.6,
                             edge="soft", fill=true})
mClothEdge = ((mCloth2:grow(6) - mCloth2:shrink(4)) * mTable):blur(5)
work(mClothEdge, {hand="glaze", pile=pWoodD, coverage=1, edge="lost", clip=true})

-- kill the orange streak along the back of the table on the left
mBack = ((mTable * rect(0, 436, 380, 74)):blur(10))
work(mBack, {hand="body", pile=pWood2, coverage=1.6, angle=-0.05,
             edge="soft", fill=true, clip=true})
print("values corrected, contacts put in")

--@ chunk 13
-- the jug was reading as the lightest object; lay it in at its own value
pJugM2 = pile{{"cobalt blue", 2.6}, {"smalt", 1.6}, {"raw umber", 0.9}}

work(mJug, {hand="body", tool=bBody, pile=pJugM2, coverage=3, angle=1.2,
            edge="found", fill=true, clip=true, pressure={0.8, 1.0}})
work(mJugDkM, {hand="body", pile=pJugDk, coverage=2.6, angle=1.2,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})

-- a smaller, more selective light now
mJugLt2 = ((mJug * (ellipse(318, 372, 46, 96) + ellipse(316, 512, 38, 40))):blur(12))
work(mJugLt2, {hand="body", pile=pJugLt, coverage=1.5, angle=1.2,
               edge="lost", fill=true, clip=true, pressure={0.5, 1.0}})
work(mJugHi, {hand="detail", tool=bRound, pile=pJugLt, coverage=1.8,
              edge="lost", clip=true, pressure={0.7, 1.0}})
work(mJugW, {hand="glaze", pile=pJugW, coverage=1.2, edge="lost", clip=true})
work(mHTop, {hand="body", pile=pJugLt, coverage=1.4, edge="soft", clip=true})
work(mRim,  {hand="body", pile=pJugLt, coverage=1.8, edge="soft", clip=mJug})
work(mFoot, {hand="glaze", pile=pJugDk, coverage=1.4, edge="lost", clip=true})

-- and let the jug throw a proper shadow to the right
mJugCast = ((ellipse(486, 602, 210, 36) * mTable):blur(18))
work(mJugCast, {hand="glaze", pile=pWoodD, coverage=1.4, edge="lost", clip=true})
print("jug sat down into its value")

--@ chunk 14
-- SITTING 2, call 1: the jug. Fatter belly, and first scrub the pale halo
-- the light had feathered out around the whole contour, plus the haze on the table.
mJug2 = mJug + (ellipse(288, 500, 58, 80) + ellipse(432, 478, 40, 58) + ellipse(354, 306, 40, 36))
halo = ((mJug:grow(26) + ellipse(352, 594, 146, 30)) - mJug2):blur(7)
work(halo * mWall, {hand="body", pile=pWall2, coverage=2.4, edge="soft", fill=true})
work(halo * mTable, {hand="body", pile=pWood2, coverage=2.4, edge="soft", fill=true})
print("halo area:", halo:area(), "new jug area:", mJug2:area())

-- lay the form solid again on the wider silhouette
work(mJug2, {hand="body", tool=bBody, pile=pJugM2, coverage=3, angle=1.35,
             edge="found", fill=true, clip=true, pressure={0.8, 1.0}})
-- core shadow: terminator slanting back as it rises
mJugDk2 = ((mJug2 * mask(function(x, y)
  return clamp((x - 322 - 0.22 * (y - 420)) / 118, 0, 1)^1.5 end)):blur(10))
work(mJugDk2, {hand="body", pile=pJugDk, coverage=2.6, angle=1.35,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})
-- the light
mJugLt3 = ((mJug2 * (ellipse(316, 378, 48, 104) + ellipse(310, 514, 40, 44))):blur(12))
work(mJugLt3, {hand="body", pile=pJugLt, coverage=1.6, angle=1.35,
               edge="lost", fill=true, clip=true, pressure={0.5, 1.0}})
mJugHi2 = ((mJug2 * ellipse(266, 470, 12, 86)):blur(4))
work(mJugHi2, {hand="detail", tool=bRound, pile=pJugLt, coverage=2,
               edge="lost", clip=true, pressure={0.6, 1.0}})
-- warm bounce off the table and the orange, up the lower right
mJugW2 = ((mJug2 * mask(function(x, y)
  return clamp((x - 330)/80, 0, 1) * clamp((y - 440)/130, 0, 1) end)):blur(11))
work(mJugW2, {hand="glaze", pile=pJugW, coverage=1.2, edge="lost", clip=true})
-- rim and lip
mRim2  = ellipse(358, 271, 40, 9)
mRimU2 = (ellipse(358, 282, 37, 5)):blur(3)
work(mRim2, {hand="body", pile=pJugLt, coverage=1.8, edge="soft", clip=mJug2})
work(mRimU2, {hand="body", pile=pJugDk, coverage=2, edge="soft", clip=mJug2})
-- the foot: darkest darks, and the shadow it drops on the wood
mFoot2 = ((mJug2 * ellipse(354, 568, 122, 18)):blur(6))
work(mFoot2, {hand="glaze", pile=pJugDk, coverage=1.4, edge="lost", clip=true})
mContact3 = (ellipse(356, 580, 116, 10)):blur(4)
work(mContact3, {hand="body", pile=pWoodD, coverage=1.6, edge="soft", fill=true})
print("jug remodelled")

--@ chunk 15
-- SITTING 2, call 2: the cloth, redrawn as folded fabric with corners.
pClothLt = pile{{"lead white", 4}, {"yellow ochre", 0.5}}

clothPts2 = {{398,590},{452,556},{520,528},{596,512},{676,508},{748,520},{792,552},
             {784,596},{752,634},{686,656},{596,664},{498,652},{438,626}}
mCloth3 = (poly(clothPts2)):blur(3)

-- scrub the old cloth's soft fringe off the wood first
halo2 = ((mCloth2:grow(22) + ellipse(600, 674, 210, 26)) - mCloth3):blur(6)
work(halo2 * mTable, {hand="body", pile=pWood2, coverage=2.4, edge="soft", fill=true})

-- the mass, solid
work(mCloth3, {hand="body", tool=bBody, pile=pCloth2, coverage=2.6, angle=-0.25,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})

-- three fold planes: crest, trough under it, and the front lip
crest1 = ((ribbon({{404,586},{520,554},{648,538},{790,558}}, 30) * mCloth3):blur(3))
trough1 = ((ribbon({{402,616},{524,590},{656,576},{788,594}}, 34) * mCloth3):blur(3))
crest2 = ((ribbon({{420,634},{540,614},{668,610},{776,624}}, 22) * mCloth3):blur(3))
trough2 = ((ribbon({{430,656},{546,644},{664,642},{768,650}}, 26) * mCloth3):blur(3))

work(trough1, {hand="body", pile=pClothW, coverage=2, angle=-0.2,
               edge="found", fill=true, clip=true, pressure={0.6, 1.0}})
work(trough2, {hand="body", pile=pClothW, coverage=1.8, angle=-0.2,
               edge="found", fill=true, clip=true, pressure={0.6, 1.0}})
work(crest1, {hand="body", pile=pClothLt, coverage=1.3, angle=-0.2,
              edge="soft", fill=true, clip=true, pressure={0.4, 0.9}})
work(crest2, {hand="body", pile=pClothLt, coverage=1.2, angle=-0.2,
              edge="soft", fill=true, clip=true, pressure={0.4, 0.9}})

-- the front lip rolls over into shadow along the bottom edge
lip = ((ribbon({{426,650},{544,672},{672,676},{774,656}}, 24) * mCloth3):blur(4))
work(lip, {hand="body", pile=pClothW, coverage=1.5, edge="soft",
           fill=true, clip=true, pressure={0.5, 0.9}})
-- and the cloth's own shadow on the wood, to its right and front
mClothSh = ((mCloth3:grow(10) - mCloth3) * mTable):blur(7)
work(mClothSh, {hand="glaze", pile=pWoodD, coverage=1.2, edge="lost"})
print("cloth rebuilt")

--@ chunk 16
-- SITTING 2, call 3: the orange, bigger and genuinely round.
pOrangeR = pile{{"vermilion", 1.6}, {"yellow ochre", 1.0}, {"lead white", 1.8}}
pOrangeC = pile{{"red earth", 1.6}, {"raw umber", 1.4}, {"bone black", 0.4}}
pOrangeH = pile{{"lead white", 2}, {"chrome yellow", 1}}

mOrNew = ellipse(630, 596, 64, 60):blur(1)
-- scrub the old orange's ghost off the cloth
work((mOr:grow(20) - mOrNew) * mCloth3, {hand="body", pile=pCloth2,
     coverage=2.2, edge="soft", fill=true})
-- the mass
work(mOrNew, {hand="body", tool=bBody, pile=pOrange, coverage=2.6, angle=1.3,
              edge="found", fill=true, clip=true, pressure={0.75, 1.0}})

-- light it properly: try the solid + form scaffold, fall back to hand masks
local litM, shM = nil, nil
local okf = pcall(function()
  local s = body.ellipsoid({630, 596, 0}, {64, 60, 57})
  local f = form{ {s, dist = 0.3}, light = {from = {-1, -0.7}, front = 0.5, ambient = 0.2} }
  litM = (f:lit{parts = {1}, soft = 0.1}) * mOrNew
  shM  = (f:shadow{parts = {1}}) * mOrNew
end)
if okf and litM and shM and shM:area() > 500 then
  print("lit from the form scaffold; lit area", litM:area(), "shadow", shM:area())
else
  litM = ((mOrNew * ellipse(606, 566, 62, 58)):blur(7))
  shM  = (mOrNew - litM)
  print("form scaffold unavailable, hand masks")
end

work(litM, {hand="body", pile=pOrangeL, coverage=1.5, angle=1.3,
            edge="lost", fill=true, clip=true, pressure={0.4, 0.9}})
work(shM, {hand="body", pile=pOrangeD, coverage=2, angle=1.3,
           edge="soft", fill=true, clip=true, pressure={0.6, 1.0}})
-- the core of the shadow, right at the terminator
core = ((shM * ellipse(662, 632, 46, 36)):blur(6))
work(core, {hand="glaze", pile=pOrangeC, coverage=1.6, edge="lost", clip=true})
-- light bounced up off the cream cloth into the shadow side
refl = ((mOrNew * ellipse(652, 638, 27, 14)):blur(5))
work(refl, {hand="glaze", pile=pOrangeR, coverage=1.4, edge="lost", clip=true})
-- the small hard highlight, and the navel at the top
work((mOrNew * ellipse(602, 566, 15, 12)):blur(4),
     {hand="detail", tool=bRound, pile=pOrangeH, coverage=2,
      edge="lost", clip=true, pressure={0.7, 1.0}})
work((mOrNew * ellipse(628, 552, 8, 5)):blur(2),
     {hand="detail", pile=pOrangeC, coverage=1.6, edge="soft", clip=true})
-- where it sits on the cloth, and the shadow it throws to the right
work((ellipse(632, 652, 58, 12)):blur(4), {hand="body", pile=pClothW,
     coverage=1.6, edge="soft", fill=true})
work(((ribbon({{664, 638}, {724, 646}, {788, 654}}, 26)):blur(8)) * mCloth3,
     {hand="glaze", pile=pClothW, coverage=1.2, edge="lost"})
print("orange redrawn")

--@ chunk 17
-- SITTING 2, call 4: unification. Scrub the jug's halo again, calm the
-- cloth's mottle with one long glaze, then re-lay the folds with broad strokes.
halo3 = ((mJug2:grow(20) + ellipse(352, 592, 142, 26)) - mJug2):blur(6)
work(halo3 * mWall, {hand="body", pile=pWall2, coverage=2.4, edge="soft", fill=true})
work(halo3 * mTable, {hand="body", pile=pWood2, coverage=2.4, edge="soft", fill=true})

-- one long glaze over the cloth to settle the dabby texture back together
work(mCloth3, {hand="glaze", pile=pCloth2, coverage=0.9, edge="lost", clip=true})

-- the folds again, broad and firm this time
trough1 = ((ribbon({{402,616},{524,590},{656,576},{788,594}}, 34) * mCloth3):blur(2))
trough2 = ((ribbon({{430,656},{546,644},{664,642},{768,650}}, 26) * mCloth3):blur(2))
crest1 = ((ribbon({{404,586},{520,554},{648,538},{790,558}}, 30) * mCloth3):blur(3))
crest2 = ((ribbon({{420,634},{540,614},{668,610},{776,624}}, 22) * mCloth3):blur(3))
work(trough1, {hand="broad", tool=bBroad, pile=pClothW, coverage=1.9,
               length={70, 150}, edge="found", fill=true, clip=true,
               pressure={0.6, 1.0}})
work(trough2, {hand="broad", tool=bBroad, pile=pClothW, coverage=1.7,
               length={60, 130}, edge="found", fill=true, clip=true,
               pressure={0.6, 1.0}})
work(crest1, {hand="broad", tool=bBroad, pile=pClothLt, coverage=1.2,
              length={70, 150}, edge="soft", fill=true, clip=true,
              pressure={0.4, 0.9}})
work(crest2, {hand="broad", tool=bBroad, pile=pClothLt, coverage=1.0,
              length={60, 130}, edge="soft", fill=true, clip=true,
              pressure={0.4, 0.9}})

-- the jug: take the glare off the outer light side, deepen the core
work(((mJug2 * ellipse(336, 430, 74, 158)):blur(10)),
     {hand="glaze", pile=pJugM2, coverage=1, edge="lost", clip=true})
work(mJugDk2, {hand="glaze", pile=pJugDk, coverage=1.3, edge="lost", clip=true})
work(mJugW2, {hand="glaze", pile=pJugW, coverage=1, edge="lost", clip=true})

-- and the shadow the jug throws right across the wood
mJugSh2 = ((ribbon({{392, 592}, {480, 606}, {576, 618}}, 30) * mTable):blur(9))
work(mJugSh2, {hand="glaze", pile=pWoodD, coverage=1.3, edge="lost", clip=true})
print("unified")

--@ chunk 18
-- SITTING 2, call 5: repair. The brushes still carried the last colour and
-- dragged it across the wall and the cloth. Wipe them, reload before every
-- colour change, and rebuild the jug and the cloth. No broad hand near edges.
bBody:wipe(0.9); bBroad:wipe(0.9); bRound:wipe(0.9); bSoft:wipe(0.9); bRigger:wipe(0.9)

-- scrub the wall and the wood clean, well past the streaks, clipped to the band
scuff = ((mJug2:grow(46) + ellipse(600, 676, 214, 28)) - mJug2 - mCloth3):blur(5)
bBody:reload(pWall2, 0.7)
work(scuff * mWall, {hand="body", tool=bBody, pile=pWall2, coverage=2.6,
                     edge="soft", fill=true, clip=true})
bBody:reload(pWood2, 0.7)
work(scuff * mTable, {hand="body", tool=bBody, pile=pWood2, coverage=2.6,
                      edge="soft", fill=true, clip=true})

-- the jug, solid again
bBody:reload(pJugM2, 0.85)
work(mJug2, {hand="body", tool=bBody, pile=pJugM2, coverage=2.8, angle=1.35,
             edge="found", fill=true, clip=true, pressure={0.8, 1.0}})
bBody:reload(pJugDk, 0.8)
work(mJugDk2, {hand="body", pile=pJugDk, coverage=2.6, angle=1.35,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})
bBody:reload(pJugLt, 0.8)
work(mJugLt3, {hand="body", pile=pJugLt, coverage=1.6, angle=1.35,
               edge="lost", fill=true, clip=true, pressure={0.5, 1.0}})
bRound:reload(pJugLt, 0.9)
work(mJugHi2, {hand="detail", tool=bRound, pile=pJugLt, coverage=2,
               edge="lost", clip=true, pressure={0.6, 1.0}})
bBody:reload(pJugDk, 0.7)
work(mRimU2, {hand="body", pile=pJugDk, coverage=2, edge="soft", clip=mJug2})
work(mFoot2, {hand="glaze", pile=pJugDk, coverage=1.4, edge="lost", clip=true})
bBody:reload(pWoodD, 0.8)
work(mContact3, {hand="body", pile=pWoodD, coverage=1.6, edge="soft", fill=true})
bSoft:reload(pWoodD, 0.5)
work(mJugSh2, {hand="glaze", pile=pWoodD, coverage=1.3, edge="lost", clip=true})

-- the cloth, solid again, then its folds
bBody:reload(pCloth2, 0.85)
work(mCloth3, {hand="body", tool=bBody, pile=pCloth2, coverage=2.6, angle=-0.25,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})
bBody:reload(pClothW, 0.85)
work(trough1, {hand="body", pile=pClothW, coverage=1.9, angle=-0.2,
               edge="found", fill=true, clip=true, pressure={0.6, 1.0}})
work(trough2, {hand="body", pile=pClothW, coverage=1.7, angle=-0.2,
               edge="found", fill=true, clip=true, pressure={0.6, 1.0}})
work(lip, {hand="body", pile=pClothW, coverage=1.5, edge="soft",
           fill=true, clip=true, pressure={0.5, 0.9}})
bBody:reload(pClothLt, 0.8)
work(crest1, {hand="body", pile=pClothLt, coverage=1.3, angle=-0.2,
              edge="soft", fill=true, clip=true, pressure={0.4, 0.9}})
work(crest2, {hand="body", pile=pClothLt, coverage=1.1, angle=-0.2,
              edge="soft", fill=true, clip=true, pressure={0.4, 0.9}})
print("repaired")

--@ chunk 19
-- SITTING 2, call 6: last streaks off the jug, and the orange back.
bBody:wipe(0.9); bRound:wipe(0.9); bSoft:wipe(0.9)

-- scrub the halo round the neck and shoulder, then bury the streaks inside
neck = (mJug2 * ellipse(352, 330, 66, 96)):blur(6)
bBody:reload(pWall2, 0.7)
work(((mJug2:grow(16) - mJug2) * mWall), {hand="body", tool=bBody, pile=pWall2,
     coverage=2.4, edge="soft", fill=true, clip=true})
bBody:reload(pJugM2, 0.85)
work(mJug2, {hand="body", tool=bBody, pile=pJugM2, coverage=2.6, angle=1.35,
             edge="found", fill=true, clip=true, pressure={0.8, 1.0}})
bBody:reload(pJugDk, 0.8)
work(mJugDk2, {hand="body", pile=pJugDk, coverage=2.4, angle=1.35,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})
work(mHDk, {hand="body", pile=pJugDk, coverage=1.8, edge="soft", clip=true})
bBody:reload(pJugLt, 0.8)
work(mJugLt3, {hand="body", pile=pJugLt, coverage=1.5, angle=1.35,
               edge="found", fill=true, clip=true, pressure={0.5, 1.0}})
work(mJugHi2, {hand="detail", tool=bRound, pile=pJugLt, coverage=2,
               edge="lost", clip=true, pressure={0.6, 1.0}})
work(mRim2, {hand="body", pile=pJugLt, coverage=1.8, edge="soft", clip=mJug2})
work(mHTop, {hand="body", pile=pJugLt, coverage=1.4, edge="soft", clip=true})
bBody:reload(pJugDk, 0.7)
work(mRimU2, {hand="body", pile=pJugDk, coverage=2, edge="soft", clip=mJug2})
work(mFoot2, {hand="glaze", pile=pJugDk, coverage=1.4, edge="lost", clip=true})

-- the orange, again, on the cloth
mOrNew = ellipse(630, 596, 64, 60):blur(1)
bBody:reload(pOrange, 0.85)
work(mOrNew, {hand="body", tool=bBody, pile=pOrange, coverage=2.6, angle=1.3,
              edge="found", fill=true, clip=true, pressure={0.75, 1.0}})
local litM, shM = nil, nil
local okf = pcall(function()
  local s = body.ellipsoid({630, 596, 0}, {64, 60, 57})
  local f = form{ {s, dist = 0.3}, light = {from = {-1, -0.7}, front = 0.5, ambient = 0.2} }
  litM = (f:lit{parts = {1}, soft = 0.1}) * mOrNew
  shM  = (f:shadow{parts = {1}}) * mOrNew
end)
if not (okf and litM and shM and shM:area() > 500) then
  litM = ((mOrNew * ellipse(606, 566, 62, 58)):blur(7)); shM = (mOrNew - litM)
  print("hand masks")
else print("form scaffold") end
bBody:reload(pOrangeL, 0.8)
work(litM, {hand="body", pile=pOrangeL, coverage=1.5, angle=1.3,
            edge="found", fill=true, clip=true, pressure={0.4, 0.9}})
bBody:reload(pOrangeD, 0.8)
work(shM, {hand="body", pile=pOrangeD, coverage=2, angle=1.3,
           edge="soft", fill=true, clip=true, pressure={0.6, 1.0}})
bSoft:reload(pOrangeC, 0.4)
work(((shM * ellipse(662, 632, 46, 36)):blur(6)),
     {hand="glaze", pile=pOrangeC, coverage=1.6, edge="lost", clip=true})
bSoft:reload(pOrangeR, 0.4)
work(((mOrNew * ellipse(652, 638, 27, 14)):blur(5)),
     {hand="glaze", pile=pOrangeR, coverage=1.4, edge="lost", clip=true})
bRound:reload(pOrangeH, 0.9)
work((mOrNew * ellipse(602, 566, 15, 12)):blur(4),
     {hand="detail", tool=bRound, pile=pOrangeH, coverage=2,
      edge="lost", clip=true, pressure={0.7, 1.0}})
work((mOrNew * ellipse(628, 552, 8, 5)):blur(2),
     {hand="body", pile=pOrangeC, coverage=1.6, edge="soft", clip=true})
bBody:reload(pClothW, 0.8)
work((ellipse(632, 652, 58, 12)):blur(4), {hand="body", pile=pClothW,
     coverage=1.6, edge="soft", fill=true})
work(((ribbon({{664, 638}, {724, 646}, {788, 654}}, 26)):blur(8)) * mCloth3,
     {hand="body", pile=pClothW, coverage=1.2, edge="lost"})
print("jug cleaned, orange back")

--@ chunk 20
-- SITTING 3, call 1: scrub the jug's fringe off its surroundings, then
-- re-lay the jug's mass solid so the chalky light patch is buried.
bBody:wipe(0.9); bBroad:wipe(0.9); bRound:wipe(0.9); bSoft:wipe(0.9); bRigger:wipe(0.9)

haloA = ((mJug2:grow(24) + ellipse(354, 590, 152, 26)) - mJug2):blur(5)

-- cloth first (it sits on top of the wood), then wood, then wall
bBody:reload(pCloth2, 0.8)
work(haloA * mCloth3, {hand="body", tool=bBody, pile=pCloth2, coverage=2.0,
                       angle=-0.25, edge="soft", fill=true, clip=true})
bBody:reload(pWood2, 0.8)
work((haloA * mTable) - mCloth3, {hand="body", tool=bBody, pile=pWood2, coverage=2.4,
                                  angle=-0.05, edge="soft", fill=true, clip=true})
bBody:reload(pWall2, 0.8)
work(haloA * mWall, {hand="body", tool=bBody, pile=pWall2, coverage=2.2,
                     edge="soft", fill=true, clip=true})

-- the jug's own mass, solid: this buries the chalky dabs under an even film
bBody:reload(pJugM2, 0.85)
work(mJug2, {hand="body", tool=bBody, pile=pJugM2, coverage=2.6, angle=1.35,
             edge="found", fill=true, clip=true, pressure={0.8, 1.0}})

-- core shadow, terminator slanting back as it rises
bBody:reload(pJugDk, 0.8)
work(mJugDk2, {hand="body", pile=pJugDk, coverage=2.4, angle=1.35,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})
print("fringe scrubbed, jug massed;  haloA area", haloA:area())

--@ chunk 21
-- SITTING 3, call 2: give the belly real roundness with the form scaffold,
-- and a light that is cooler and less chalky than lead-white-heavy pJugLt.
pJugLt2 = pile{{"cobalt blue", 0.9}, {"pale smalt", 1.5}, {"lead white", 2.2}}

bellyOnly = mask(function(x, y) return clamp((y - 372) / 44, 0, 1) end)

local litM, shM = nil, nil
local okf = pcall(function()
  local s = body.ellipsoid({352, 480, 0}, {104, 110, 76})
  local f = form{ {s, dist = 0.3}, light = {from = {-1, -0.7}, front = 0.5, ambient = 0.2} }
  litM = (f:lit{parts = {1}, soft = 0.12}) * mJug2 * bellyOnly
  shM  = (f:shadow{parts = {1}}) * mJug2 * bellyOnly
end)
if not (okf and litM and shM and shM:area() > 800 and litM:area() > 800) then
  litM = ((mJug2 * ellipse(306, 480, 78, 108)):blur(9))
  shM  = ((mJug2 - litM) * bellyOnly)
  print("hand masks")
else print("form scaffold, lit", litM:area(), "shadow", shM:area()) end

-- the lit side: lighter but thin, so it reads as light and not as chalk
bBody:reload(pJugLt2, 0.7)
work(litM, {hand="body", pile=pJugLt2, coverage=1.1, angle=1.35,
            edge="lost", clip=true, pressure={0.35, 0.75}})
blend(litM, {angle=1.35})

-- the turn of the form into shadow
bBody:reload(pJugDk, 0.7)
work(shM, {hand="body", pile=pJugDk, coverage=0.9, angle=1.35,
           edge="soft", clip=true, pressure={0.5, 0.9}})

-- light up the shoulder and the near side of the neck
mJugLt4 = ((mJug2 * (ellipse(302, 396, 32, 72) + ellipse(332, 326, 24, 48))):blur(9))
bBody:reload(pJugLt2, 0.75)
work(mJugLt4, {hand="body", pile=pJugLt2, coverage=1.2, angle=1.35,
               edge="lost", clip=true, pressure={0.4, 0.85}})
blend(mJugLt4, {angle=1.35})

-- one small crisp highlight, high on the shoulder, not on the belly
bRound:reload(pJugLt, 0.9)
work(((mJug2 * ellipse(292, 396, 7, 14)):blur(3)),
     {hand="detail", tool=bRound, pile=pJugLt, coverage=1.6,
      edge="lost", clip=true, pressure={0.7, 1.0}})
-- and the old chalky streak on the belly is now only a faint sheen
bSoft:reload(pJugLt2, 0.35)
work(((mJug2 * ellipse(268, 476, 9, 74)):blur(6)),
     {hand="glaze", pile=pJugLt2, coverage=0.8, edge="lost", clip=true})

-- warm bounce up the lower right, off the wood and the orange
mJugW2 = ((mJug2 * mask(function(x, y)
  return clamp((x - 336)/76, 0, 1) * clamp((y - 452)/120, 0, 1) end)):blur(12))
bSoft:reload(pJugW, 0.5)
work(mJugW2, {hand="glaze", pile=pJugW, coverage=1.1, edge="lost", clip=true})

-- rim: a lit top plane and the dark line of the lip under it
bBody:reload(pJugLt2, 0.7)
work(mRim2, {hand="body", pile=pJugLt2, coverage=1.5, edge="found", clip=mJug2})
bBody:reload(pJugDk, 0.75)
work(mRimU2, {hand="body", pile=pJugDk, coverage=1.8, edge="found", clip=mJug2})
work(mHDk, {hand="body", pile=pJugDk, coverage=1.6, edge="soft", clip=true})
work(mFoot2, {hand="glaze", pile=pJugDk, coverage=1.2, edge="lost", clip=true})
print("belly round, light tamed")

--@ chunk 22
-- SITTING 3, call 3: crisp contour. Trace the jug's silhouette out of the
-- mask itself and run a rigger line round it.
L = {}; R = {}; B = {}
for y = 276, 576, 12 do
  local lx, rx = nil, nil
  for x = 200, 500, 2 do
    if mJug2:at(x, y) > 0.5 then lx = x end
  end
  for x = 500, 200, -2 do
    if mJug2:at(x, y) > 0.5 then rx = x end
  end
  if lx then L[#L + 1] = {lx + 3, y} end
  if rx then R[#R + 1] = {rx - 3, y} end
end
for i = 1, math.floor(#R / 2) do
  local t = R[i]; R[i] = R[#R + 1 - i]; R[#R + 1 - i] = t
end
for x = 250, 470, 12 do
  local by = nil
  for y = 596, 520, -2 do
    if mJug2:at(x, y) > 0.5 then by = y end
  end
  if by then B[#B + 1] = {x, by - 3} end
end
print("traced", #L, #R, #B)

-- the lit side in two runs: firm at the shoulder, barely there at the foot
Lhi = {}; Llo = {}
for i, p in ipairs(L) do
  if p[2] <= 480 then Lhi[#Lhi + 1] = p else Llo[#Llo + 1] = p end
end
bRigger:reload(pJugLt2, 0.6)
oLhi = outline{pts=Lhi, char="firm", amount=0.7}
oLhi:paint(bRigger, {pressure=0.75, dip={pJugLt2, 0.45}, every=4})
bRigger:reload(pJugLt2, 0.35)
oLlo = outline{pts=Llo, char="broken", amount=0.9}
oLlo:paint(bRigger, {pressure=0.22, dip={pJugLt2, 0.3}, every=6})

-- the shadow side: quiet under the rim, firmer down the belly
Rhi = {}; Rlo = {}
for i, p in ipairs(R) do
  if p[2] >= 300 then Rlo[#Rlo + 1] = p else Rhi[#Rhi + 1] = p end
end
bRigger:reload(pJugDk, 0.6)
oRlo = outline{pts=Rlo, char="firm", amount=0.7}
oRlo:paint(bRigger, {pressure=0.7, dip={pJugDk, 0.55}, every=4})
bRigger:reload(pJugDk, 0.4)
oRhi = outline{pts=Rhi, char="broken", amount=0.9}
oRhi:paint(bRigger, {pressure=0.25, dip={pJugDk, 0.3}, every=6})

-- the foot, where the form turns down into the wood
bRigger:reload(pJugDk, 0.6)
oB = outline{pts=B, char="firm", amount=0.6}
oB:paint(bRigger, {pressure=0.6, dip={pJugDk, 0.5}, every=5})
print("jug contoured")

--@ chunk 23
-- SITTING 3, call 4: the contour lines came out as wire. Repaint the jug solid
-- to bury them, then lay the edge as a narrow painted band inside the silhouette.
bBody:wipe(0.9); bRound:wipe(0.9); bSoft:wipe(0.9); bRigger:wipe(0.9)

bBody:reload(pJugM2, 0.85)
work(mJug2, {hand="body", tool=bBody, pile=pJugM2, coverage=2.6, angle=1.35,
             edge="found", fill=true, clip=true, pressure={0.8, 1.0}})
bBody:reload(pJugDk, 0.8)
work(mJugDk2, {hand="body", pile=pJugDk, coverage=2.2, angle=1.35,
               edge="soft", fill=true, clip=true, pressure={0.7, 1.0}})

litM4 = ((mJug2 * (ellipse(312, 474, 62, 98) + ellipse(300, 392, 30, 66)
                   + ellipse(330, 320, 22, 44))):blur(11))
bBody:reload(pJugLt2, 0.75)
work(litM4, {hand="body", pile=pJugLt2, coverage=1.3, angle=1.35,
             edge="lost", fill=true, clip=true, pressure={0.4, 0.85}})
mJugW2 = ((mJug2 * mask(function(x, y)
  return clamp((x - 336)/76, 0, 1) * clamp((y - 452)/120, 0, 1) end)):blur(12))
bSoft:reload(pJugW, 0.5)
work(mJugW2, {hand="glaze", pile=pJugW, coverage=1.1, edge="lost", clip=true})
bRound:reload(pJugLt, 0.9)
work((mJug2 * ellipse(292, 394, 7, 14)):blur(3),
     {hand="detail", tool=bRound, pile=pJugLt, coverage=1.6, edge="lost",
      clip=true, pressure={0.7, 1.0}})
bBody:reload(pJugLt2, 0.7)
work(mRim2, {hand="body", pile=pJugLt2, coverage=1.5, edge="found", clip=mJug2})
bBody:reload(pJugDk, 0.75)
work(mRimU2, {hand="body", pile=pJugDk, coverage=1.8, edge="found", clip=mJug2})
work(mFoot2, {hand="glaze", pile=pJugDk, coverage=1.2, edge="lost", clip=true})
print("wires buried, jug rebuilt")

-- the edge, as a band a couple of units wide, following the traced contour
Ls = {}; Rs = {}
for i = 1, #L, 2 do Ls[#Ls + 1] = L[i] end
for i = 1, #R, 2 do Rs[#Rs + 1] = R[i] end
eL = outline{pts=Ls, char="firm", amount=0.15, size=3}
eR = outline{pts=Rs, char="firm", amount=0.15, size=3}
mEdgeL = eL:mask() * mJug2:shrink(1)
Rs2 = {}; for i = 1, #R do if R[i][2] >= 296 then Rs2[#Rs2 + 1] = R[i] end end
mEdgeR = outline{pts=Rs2, char="firm", amount=0.15, size=3}:mask() * mJug2:grow(2)

bRigger:reload(pJugLt2, 0.55)
work(mEdgeL, {hand="detail", tool=bRigger, pile=pJugLt2, coverage=1.8,
              edge="lost", clip=true, pressure={0.6, 1.0}})
bRigger:reload(pJugDk, 0.7)
work(mEdgeR, {hand="detail", tool=bRigger, pile=pJugDk, coverage=2.0,
              edge="lost", clip=true, pressure={0.7, 1.0}})
print("edge bands laid; left", mEdgeL:area(), "right", mEdgeR:area())

--@ chunk 24
-- SITTING 3, call 5: final. Scrub the jug's surroundings clean of the stray
-- hairs and the blue bleed, then lay the contour with a blunt small brush.
bEdge = brush{kind="round", width=3.2, point=0.1, stiffness=0.6}
bBody:wipe(0.9); bSoft:wipe(0.9); bRound:wipe(0.9); bRigger:wipe(0.9); bEdge:wipe(0.9)

band = ((mJug2:grow(40) + ellipse(354, 592, 150, 26)) - mJug2):blur(4)
bBody:reload(pCloth2, 0.8)
work(band * mCloth3, {hand="body", tool=bBody, pile=pCloth2, coverage=2.0,
                      angle=-0.25, edge="soft", fill=true, clip=true})
bBody:reload(pWood2, 0.8)
work((band * mTable) - mCloth3, {hand="body", tool=bBody, pile=pWood2, coverage=2.6,
                                 angle=-0.05, edge="soft", fill=true, clip=true})
bBody:reload(pWall2, 0.8)
work(band * mWall, {hand="body", tool=bBody, pile=pWall2, coverage=2.4,
                    edge="soft", fill=true, clip=true})
print("surround scrubbed")

-- sink the jug a touch so it sits below the wall in value
bSoft:reload(pJugM2, 0.45)
work(mJug2, {hand="glaze", pile=pJugM2, coverage=0.7, edge="found", clip=true})

-- the contour, blunt and continuous this time
mEdgeL = eL:mask() * mJug2:shrink(1)
mEdgeR = outline{pts=Rs2, char="firm", amount=0.15, size=3}:mask() * mJug2:grow(2)
bEdge:reload(pJugLt2, 0.7)
work(mEdgeL, {hand="detail", tool=bEdge, pile=pJugLt2, coverage=1.3,
              edge="lost", clip=true, pressure={0.7, 1.0}})
bEdge:reload(pJugDk, 0.8)
work(mEdgeR, {hand="detail", tool=bEdge, pile=pJugDk, coverage=1.5,
              edge="lost", clip=true, pressure={0.8, 1.0}})

-- the foot: deepest darks, and the shadow it drops on the cloth and the wood
bEdge:reload(pJugDk, 0.7)
work((mJug2 * ellipse(354, 566, 120, 16)):blur(5),
     {hand="detail", tool=bEdge, pile=pJugDk, coverage=1.4, edge="lost",
      clip=true, pressure={0.7, 1.0}})
bBody:reload(pWoodD, 0.85)
work(mContact3, {hand="body", pile=pWoodD, coverage=1.8, edge="soft",
                 fill=true, clip=true})
work(((mJug2:grow(12) - mJug2) * mCloth3):blur(5),
     {hand="body", pile=pClothW, coverage=1.4, edge="soft", clip=true})
print("final edges and darks")
