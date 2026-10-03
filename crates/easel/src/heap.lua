-- Snapshot and restore of the Lua heap a painting can reach, for exact
-- rollback (see session.rs). Loaded with a private copy of the debug
-- library; painters never see it. `canon`: engine 3 on (session.rs
-- `canonical_tables`), where prelude.lua walks every table in a fixed order.
local dbg, canon = ...
local getupvalue, setupvalue, getinfo = dbg.getupvalue, dbg.setupvalue, dbg.getinfo
local getmt, setmt = dbg.getmetatable, dbg.setmetatable
local next, type, rawset, rawequal, rawlen, select = next, type, rawset, rawequal, rawlen, select
local rawget, mtype = rawget, math.type

-- Everything reachable from the roots through tables (keys, values,
-- metatables) and Lua functions (upvalues): each table's contents and
-- metatable, each function's upvalues. Userdata are the engine's and are
-- immutable (brushes are restored by the session). `skip` lists private
-- objects (prelude.lua's caches) that are neither walked nor restored.
local function snap(skip, ...)
  local tabs, funs, seen = {}, {}, {}
  for i = 1, #skip do seen[skip[i]] = true end
  local stack, n = {}, 0
  local function push(v)
    local t = type(v)
    if (t == "table" or t == "function") and not seen[v] then
      seen[v] = true
      n = n + 1
      stack[n] = v
    end
  end
  for i = 1, select("#", ...) do push((select(i, ...))) end
  local ntab, nfun = 0, 0
  while n > 0 do
    local v = stack[n]
    stack[n] = nil
    n = n - 1
    if type(v) == "table" then
      local copy, order, m = {}, {}, 0
      for k, x in next, v do
        copy[k] = x
        m = m + 1
        order[m] = k
        push(k)
        push(x)
      end
      local mt = getmt(v)
      push(mt)
      -- (from engine 3 no `#`: it moves the table's length hint, which a
      -- replay, taking no snapshots, would not)
      tabs[v] = { copy, mt, order, m, not canon and rawlen(v) or nil }
      ntab = ntab + 1
    else
      local info = getinfo(v, "Su")
      if info.what ~= "C" and info.nups > 0 then
        local ups = { n = info.nups }
        for i = 1, info.nups do
          local _, x = getupvalue(v, i)
          ups[i] = x
          push(x)
        end
        funs[v] = ups
        nfun = nfun + 1
      end
    end
  end
  return { tabs, funs, ntab, nfun }
end

-- as snapped, walked by `next` in the same order (a table that grew and
-- shrank again during a failed chunk can change traversal order or its sparse
-- array length even when the entries are identical)
local function untouched(t, rec)
  local copy, order = rec[1], rec[3]
  if not rawequal(getmt(t), rec[2]) or (not canon and rawlen(t) ~= rec[5]) then return false end
  local i = 0
  for k, x in next, t do
    i = i + 1
    if not rawequal(order[i], k) or not rawequal(copy[k], x) then return false end
  end
  return i == rec[4]
end

local function same(t, copy, mt)
  if not rawequal(getmt(t), mt) then return false end
  local k1 = 0
  for k, x in next, t do
    if not rawequal(copy[k], x) then return false end
    k1 = k1 + 1
  end
  for _ in next, copy do k1 = k1 - 1 end
  return k1 == 0
end

-- More than one border (`#t` may be any of them, and which one Lua finds
-- depends on the table's layout): its positive integer keys aren't 1..n.
local function holey(t)
  local n = 0
  for k in next, t do
    if mtype(k) == "integer" and k > 0 then n = n + 1 end
  end
  for i = 1, n do
    if rawget(t, i) == nil then return true end
  end
  return false
end

-- Put every table and upvalue back as it was; tables that didn't change are
-- left alone (their internal layout, so their `pairs` order, is untouched).
-- Returns how many tables may still act differently from a replay's
-- (session.rs then rebuilds): entries are back, but not necessarily their
-- layout, which Lua doesn't let a program set.
-- - Engines 1 and 2: every table whose `next` order or `#` differs, since
--   `pairs` walks tables keyed by values in layout order.
-- - From engine 3 `pairs` walks every table in a fixed order (prelude.lua),
--   so layout shows only in `#` of a table with more than one border: a
--   table the chunk changed (entries or layout) that is holey now.
local function restore(s)
  local changed = 0
  local touched, nt = canon and {} or nil, 0
  for t, rec in next, s[1] do
    local copy, mt = rec[1], rec[2]
    if not untouched(t, rec) then
      if canon then
        nt = nt + 1
        touched[nt] = t
      end
      if not same(t, copy, mt) then
        local keys, m = {}, 0
        for k in next, t do
          m = m + 1
          keys[m] = k
        end
        for i = 1, m do rawset(t, keys[i], nil) end
        for k, x in next, copy do rawset(t, k, x) end
        setmt(t, mt)
      end
    end
  end
  for f, ups in next, s[2] do
    for i = 1, ups.n do
      local _, x = getupvalue(f, i)
      if not rawequal(x, ups[i]) then setupvalue(f, i, ups[i]) end
    end
  end
  if canon then
    for i = 1, nt do
      if holey(touched[i]) then changed = changed + 1 end
    end
    return changed
  end
  for t, rec in next, s[1] do
    if not untouched(t, rec) then changed = changed + 1 end
  end
  return changed
end

return snap, restore
