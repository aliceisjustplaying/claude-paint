-- Library replacements that keep a painting exact across processes and
-- rollbacks (see session.rs). Loaded once per session, before the easel's
-- verbs; painters see only the replaced globals.
--
-- `id(v)` is the session's creation serial of a table, closure, userdata or
-- thread (nil for anything else): objects are numbered as Lua allocates them,
-- so their order is the order the program made them, in every process.
local id, getmt, getinfo = ...
local rawnext, rawget, type, select, error, tostring = next, rawget, type, select, error, tostring
local rawload, pcall = load, pcall
local setmt = setmetatable
local sort, pack, unpack = table.sort, table.pack, table.unpack
local find, sub = string.find, string.sub
local tointeger = math.tointeger

-- ---------------------------------------------------------------- pairs, next

-- Lua hashes strings (fixed seed), numbers and booleans by value, so a table
-- keyed only by them walks in the same order in every process. Tables,
-- functions and userdata are hashed by address, which differs from process
-- to process, and they also move the value keys around in the table. A table
-- holding any such key is walked in a fixed order instead: booleans, numbers,
-- strings (each ascending), then objects in the order they were created.

-- light C functions (library functions) have no serial: they go by name
local names = {}
local function name_all(t, prefix)
  for k, v in rawnext, t do
    if type(v) == "function" and type(k) == "string" and id(v) == nil and names[v] == nil then
      names[v] = prefix .. k
    end
  end
end

local rank = { boolean = 1, number = 2, string = 3 }
local by_id, by_name
local function less_id(a, b) return by_id[a] < by_id[b] end
local function less_name(a, b) return by_name[a] < by_name[b] end
local function less_bool(a, b) return not a and b end

-- nil for a table walked by Lua's own `next`; else its keys in order
local function ordered(t)
  local k = rawnext(t)
  while k ~= nil do
    if not rank[type(k)] then break end
    k = rawnext(t, k)
  end
  if k == nil then return nil end
  local groups = { {}, {}, {}, {}, {}, {} }
  local sizes = { 0, 0, 0, 0, 0, 0 }
  local ids, nms = {}, {}
  for k in rawnext, t do
    local g = rank[type(k)]
    if not g then
      local n = id(k)
      if n then
        g = 4
        ids[k] = n
      elseif names[k] then
        g = 5
        nms[k] = names[k]
      else
        g = 6 -- no stable identity (never seen in practice): Lua's order
      end
    end
    local s = sizes[g] + 1
    sizes[g] = s
    groups[g][s] = k
  end
  sort(groups[1], less_bool)
  sort(groups[2])
  sort(groups[3])
  by_id, by_name = ids, nms
  sort(groups[4], less_id)
  sort(groups[5], less_name)
  by_id, by_name = nil, nil
  local keys, index, n = {}, {}, 0
  for g = 1, 6 do
    local grp = groups[g]
    for i = 1, sizes[g] do
      n = n + 1
      keys[n] = grp[i]
      index[grp[i]] = n
    end
  end
  keys.n, keys.index = n, index
  return keys
end

-- the order of a table's last traversal by `next`, per table; cleared before
-- every chunk (a rollback may have rewritten any table)
local box = { cache = setmt({}, { __mode = "k" }) }

local function from(t, keys, i)
  for j = i + 1, keys.n do
    local k = keys[j]
    local v = rawget(t, k)
    if v ~= nil then return k, v end
  end
  return nil
end

local function det_next(t, k)
  if type(t) ~= "table" then return rawnext(t, k) end
  local cache = box.cache
  local keys
  if k == nil then
    keys = ordered(t) or false
    cache[t] = keys
  else
    keys = cache[t]
    if keys == nil or (keys and keys.index[k] == nil) then
      keys = ordered(t) or false
      cache[t] = keys
    end
  end
  if not keys then return rawnext(t, k) end
  local i = 0
  if k ~= nil then
    i = keys.index[k]
    if i == nil then error("invalid key to 'next'", 2) end
  end
  return from(t, keys, i)
end

local function det_pairs(...)
  if select("#", ...) == 0 then error("bad argument #1 to 'pairs' (value expected)", 2) end
  local t = ...
  local mt = getmt(t)
  local h = mt ~= nil and type(mt) == "table" and rawget(mt, "__pairs") or nil
  if h ~= nil then
    local f, s, c, z = h(t)
    return f, s, c, z
  end
  if type(t) ~= "table" then return rawnext, t, nil end
  local keys = ordered(t)
  if not keys then return rawnext, t, nil end
  local i = 0
  return function()
    while true do
      i = i + 1
      if i > keys.n then return nil end
      local k = keys[i]
      local v = rawget(t, k)
      if v ~= nil then return k, v end
    end
  end, t, nil
end

-- ---------------------------------------------------------------- gmatch

-- Lua's `string.gmatch` keeps its position in C, where a rollback can't reach
-- it: an iterator kept in a global would stay advanced after a failed
-- chunk. This one keeps it in upvalues (which rollback restores) and
-- matches exactly as Lua 5.5's does.
local function gmatch(s, p, init)
  local ts, tp = type(s), type(p)
  if ts ~= "string" and ts ~= "number" then
    error("bad argument #1 to 'gmatch' (string expected, got " .. (s == nil and "no value" or ts) .. ")", 2)
  end
  if tp ~= "string" and tp ~= "number" then
    error("bad argument #2 to 'gmatch' (string expected, got " .. (p == nil and "no value" or tp) .. ")", 2)
  end
  s, p = tostring(s), tostring(p)
  local ls = #s
  local pos = 1
  if init ~= nil then
    local i = tointeger(init)
    if i == nil then
      error("bad argument #3 to 'gmatch' (number has no integer representation)", 2)
    end
    if i > 0 then pos = i
    elseif i == 0 or i < -ls then pos = 1
    else pos = ls + i + 1 end
  end
  -- gmatch doesn't anchor: a leading '^' is an ordinary character
  if sub(p, 1, 1) == "^" then p = "%" .. p end
  local last = -1 -- end (exclusive) of the last match
  local done = pos > ls + 1
  return function()
    while not done do
      local r = pack(find(s, p, pos))
      local st, e = r[1], r[2]
      if st == nil then
        pos = ls + 2
        done = true
      elseif e + 1 ~= last then
        pos, last = e + 1, e + 1
        if r.n == 2 then return sub(s, st, e) end
        return unpack(r, 3, r.n)
      else
        pos = st + 1
        done = pos > ls + 1
      end
    end
  end
end

-- ---------------------------------------------------------------- setmetatable

-- Finalizers (__gc) and weak tables (__mode) act when the collector gets to
-- them, and it gets there at other times in a live session (which holds a
-- snapshot of the heap) than in a replay: a painting can't use them. Lua
-- reads __mode at every collection, so it can't be added to a metatable
-- later either: `check` looks at every metatable set, after every chunk.
local REFUSED = "__gc and __mode aren't allowed in a painting: finalizers and weak tables act when the garbage collector gets to them, which differs between the live easel and a replay of the log"
local set = setmt({}, { __mode = "k" }) -- metatables given to setmetatable

local function refused(mt)
  return type(mt) == "table" and (rawget(mt, "__gc") ~= nil or rawget(mt, "__mode") ~= nil)
end

local function det_setmetatable(t, mt)
  if refused(mt) then error("setmetatable: " .. REFUSED, 2) end
  local r = setmt(t, mt)
  if type(mt) == "table" then set[mt] = true end
  return r
end

-- called after every chunk: an error fails the chunk
local function check()
  for mt in rawnext, set do
    if refused(mt) then error("a metatable holds __gc or __mode: " .. REFUSED, 0) end
  end
end

-- ---------------------------------------------------------------- load

-- Lua's `load` also takes precompiled (binary) chunks, which Lua doesn't
-- verify: a crafted one can crash the easel. A painting loads text only,
-- whatever mode it asks for (one without "t" loads nothing), from a string
-- or a reader function alike. Everything else is Lua's load: an environment
-- given, even nil, is passed on, and its complaints are worded and placed as
-- Lua's (the name it was called by, at the painting's call).
local function text_load(...)
  local n, mode = select("#", ...), select(3, ...)
  local m = mode
  if mode == nil then
    m = "t"
  elseif type(mode) == "string" or type(mode) == "number" then
    m = find(tostring(mode), "t", 1, true) and "t" or ""
  end
  local r
  if n == 0 then
    r = pack(pcall(rawload))
  elseif n >= 4 then
    local chunk, name, _, env = ...
    r = pack(pcall(rawload, chunk, name, m, env))
  else
    local chunk, name = ...
    r = pack(pcall(rawload, chunk, name, m))
  end
  if not r[1] then
    local e = r[2]
    -- load's own complaints carry no position under pcall; a reader's pass through
    if type(e) == "string" and not e:find("^[^\n]-:%d+: ") then
      local info = getinfo(1, "n")
      e = e:gsub("^bad argument (#%d+) to '%?'", function(k) return "bad argument " .. k .. " to '" .. (info.name or "load") .. "'" end)
      error(e, 2)
    end
    error(e, 0)
  end
  if r[2] == nil and m == "" and r[3] == "attempt to load a text chunk (mode is '')" then
    r[3] = "attempt to load a text chunk (mode is '" .. tostring(mode) .. "')"
  end
  return unpack(r, 2, r.n)
end

-- ---------------------------------------------------------------- install

next, pairs, string.gmatch, setmetatable, load = det_next, det_pairs, gmatch, det_setmetatable, text_load
-- no dumping functions into binary chunks either
string.dump = nil
name_all(_G, "")
for _, lib in ipairs { "string", "table", "math", "utf8" } do
  if type(_G[lib]) == "table" then name_all(_G[lib], lib .. ".") end
end

-- called before every chunk
local function fresh()
  box.cache = setmt({}, { __mode = "k" })
end

-- private state the heap snapshot must not walk
return fresh, { box, names, set }, check
