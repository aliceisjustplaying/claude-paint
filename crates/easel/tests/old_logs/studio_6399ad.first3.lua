-- easel session "painting": a painting replayed chunk by chunk.
-- Each "--@ chunk" line starts one chunk as it was run at the easel.

--@ chunk 1
canvas{
  size = 450,
  aspect = 1.25,
  linen = {16, 14},
  seed = 42,
  ground = {
    {pile = {{"lead white", 5}, {"raw umber", 1}}, um = 90, apply = "knife", texture = 0.22},
    {pile = {{"lead white", 4}, {"yellow ochre", 2}, {"raw umber", 1}}, um = 30, apply = "brush"}
  }
}

--@ chunk 2
local c = chalk()
-- Table ledge: shelf line
c:line({{0, 565}, {1000, 568}}, {pressure = {0.3, 0.4, 0.35}})
-- Shelf front bevel
c:line({{0, 580}, {1000, 583}}, {pressure = {0.25, 0.3, 0.25}})

-- Pitcher outline
c:sketch({
  {250, 260}, {235, 275}, {230, 310}, {215, 360}, {200, 420}, {205, 480}, {225, 530}, {260, 555},
  {310, 555}, {345, 530}, {365, 480}, {370, 420}, {355, 360}, {340, 310}, {335, 275}, {320, 260}, {250, 260}
}, {pressure = 0.35, wander = 2})

-- Pitcher rim ellipse
c:sketch({
  {250, 260}, {285, 252}, {320, 260}, {285, 268}, {250, 260}
}, {pressure = 0.3, wander = 1})

-- Pitcher handle
c:sketch({
  {345, 315}, {385, 330}, {400, 380}, {385, 430}, {355, 450}
}, {pressure = 0.3, wander = 1.5})

-- First quince (upright, center)
c:sketch({
  {510, 430}, {485, 470}, {470, 520}, {480, 560}, {530, 575}, {585, 565}, {605, 520}, {590, 465}, {550, 425}, {510, 430}
}, {pressure = 0.35, wander = 2})

-- Second quince (tilted, right)
c:sketch({
  {660, 525}, {635, 560}, {645, 595}, {680, 610}, {735, 605}, {775, 575}, {770, 540}, {725, 515}, {660, 525}
}, {pressure = 0.35, wander = 2})

-- Knife on the ledge
c:line({{350, 555}, {420, 572}}, {pressure = 0.3}) -- handle
c:line({{420, 572}, {505, 592}}, {pressure = 0.35}) -- blade

--@ chunk 3
-- Piles for underpainting and environment
p_bg_dark = pile{{"bone black", 3}, {"raw umber", 4}, {"red earth", 1}, medium = 0.2}
p_bg_warm = pile{{"raw umber", 4}, {"yellow ochre", 2}, {"bone black", 1}, medium = 0.25}
p_stone_top = pile{{"lead white", 4}, {"yellow ochre", 2}, {"raw umber", 2}, medium = 0.15}
p_stone_face = pile{{"raw umber", 4}, {"bone black", 2}, {"red earth", 1}, {"lead white", 1}, medium = 0.2}
p_shadow = pile{{"raw umber", 4}, {"bone black", 1}, medium = 0.3}

-- Object masks
o_pitcher = outline{
  {250, 260}, {235, 275}, {230, 310}, {215, 360}, {200, 420}, {205, 480}, {225, 530}, {260, 555},
  {310, 555}, {345, 530}, {365, 480}, {370, 420}, {355, 360}, {340, 310}, {335, 275}, {320, 260},
  closed = true, char = "firm"
}
m_pitcher = o_pitcher:mask()
m_handle = ribbon({{345, 315}, {385, 330}, {400, 380}, {385, 430}, {355, 450}}, 22)

o_quince1 = outline{
  {510, 430}, {485, 470}, {470, 520}, {480, 560}, {530, 575}, {585, 565}, {605, 520}, {590, 465}, {550, 425},
  closed = true, char = "soft"
}
m_quince1 = o_quince1:mask()

o_quince2 = outline{
  {660, 525}, {635, 560}, {645, 595}, {680, 610}, {735, 605}, {775, 575}, {770, 540}, {725, 515},
  closed = true, char = "soft"
}
m_quince2 = o_quince2:mask()

m_objects = (m_pitcher + m_handle + m_quince1 + m_quince2):shrink(4)

-- Background regions
local m_bg_all = rect(0, 0, 1000, 568) - m_objects
local m_bg_left = m_bg_all * mask(function(x, y) return clamp((500 - x) / 250, 0, 1) end)
local m_bg_right = m_bg_all * mask(function(x, y) return clamp((x - 300) / 300, 0, 1) end)

-- Lay in background: darker toward right/top, warmer behind pitcher
work(m_bg_right, {hand = "broad", pile = p_bg_dark, angle = -0.35, coverage = 1.8, fill = true})
work(m_bg_left, {hand = "broad", pile = p_bg_warm, angle = 0.25, coverage = 1.6, fill = true})
blend(m_bg_all, {angle = 0, coverage = 1.0})

-- Stone shelf
local m_shelf_top = rect(0, 565, 1000, 18) - m_objects
local m_shelf_face = rect(0, 580, 1000, 220)

work(m_shelf_top, {hand = "body", tool = "filbert 16", pile = p_stone_top, angle = 0.05, coverage = 1.8, fill = true})
work(m_shelf_face, {hand = "body", tool = "filbert 20", pile = p_stone_face, angle = 1.57, coverage = 2.0, fill = true})
blend(m_shelf_face, {angle = 1.57, coverage = 0.8})
