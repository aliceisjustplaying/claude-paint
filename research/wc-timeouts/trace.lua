local source = assert(arg[1])
local metrics = {strokes=0, washes=0, stamps=0, length=0, duration=0, waits=0, drops=0, brushes=0}
local function stroke(self, path, attrs, wash)
  attrs = attrs or {}
  local length = 0
  local stamps = 1
  local carry = 0
  local spacing = math.max(self.width * 0.11, 0.9)
  for i=2,#path do
    local a,b = path[i-1],path[i]
    local distance = math.sqrt((b[1]-a[1])^2 + (b[2]-a[2])^2)
    length = length + distance
    if distance > 1e-6 then
      local along = spacing-carry
      while along <= distance do stamps=stamps+1; along=along+spacing end
      carry=distance-(along-spacing)
    end
  end
  metrics.strokes = metrics.strokes+1
  metrics.stamps = metrics.stamps+stamps
  metrics.length = metrics.length+length
  local duration = attrs.duration or math.max(length/(wash and 2800 or 800),wash and 0.015625 or 0.0625)
  if path[#path][4] then duration=math.max(duration,path[#path][4]) end
  metrics.duration = metrics.duration+duration
  if metrics.strokes > 100000 then error("more than 100000 stroke calls; probably nonterminating") end
end
local methods = {}
function methods:load(t) end
function methods:status() return {water=0,pigments={}} end
function methods:stroke(path,attrs) stroke(self,path,attrs,false) end
function methods:wash(t)
  metrics.washes=metrics.washes+1
  local rows=math.max(1,math.ceil(t.height/(self.width*0.7)))
  for r=0,rows-1 do
    local y=t.y+(r+0.5)*t.height/rows
    stroke(self,{{t.x,y},{t.x+t.width,y}},t,true)
  end
end
function methods:lift(path) stroke(self,path,nil,false) end
function brush(t)
  metrics.brushes=metrics.brushes+1
  return setmetatable(t,{__index=methods})
end
function drop(t) metrics.drops=metrics.drops+1 end
function wait(t) metrics.waits=metrics.waits+t end
function clock() return metrics.duration+metrics.waits end
function balance() return {dry=true} end
function paper(t) end
local loaded=assert(loadfile(source))
local ok,err=pcall(loaded)
for k,v in pairs(metrics) do io.write(k,"=",string.format("%.9f",v)," ") end
io.write("\n")
if not ok then io.write(tostring(err),"\n"); os.exit(2) end
