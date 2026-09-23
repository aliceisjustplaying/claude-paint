-- Snapshot and restore of the Lua heap a painting can reach, for exact
-- rollback (see session.rs). Loaded with a private copy of the debug
-- library; painters never see it.
local dbg = ...
local getupvalue, setupvalue, getinfo = dbg.getupvalue, dbg.setupvalue, dbg.getinfo
local getmt, setmt = dbg.getmetatable, dbg.setmetatable
local next, type, rawset, rawequal, select = next, type, rawset, rawequal, select

-- Everything reachable from the roots through tables (keys, values,
-- metatables) and Lua functions (upvalues): each table's contents and
-- metatable, each function's upvalues. Userdata are the engine's and are
-- immutable (brushes are restored by the session).
local function snap(...)
  local tabs, funs, seen = {}, {}, {}
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
      local copy = {}
      for k, x in next, v do
        copy[k] = x
        push(k)
        push(x)
      end
      local mt = getmt(v)
      push(mt)
      tabs[v] = { copy, mt }
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

-- Put every table and upvalue back as it was; tables that didn't change are
-- left alone (their internal layout, so their `pairs` order, is untouched).
local function restore(s)
  local changed = 0
  for t, rec in next, s[1] do
    local copy, mt = rec[1], rec[2]
    if not same(t, copy, mt) then
      local keys, m = {}, 0
      for k in next, t do
        m = m + 1
        keys[m] = k
      end
      for i = 1, m do rawset(t, keys[i], nil) end
      for k, x in next, copy do rawset(t, k, x) end
      setmt(t, mt)
      changed = changed + 1
    end
  end
  for f, ups in next, s[2] do
    for i = 1, ups.n do
      local _, x = getupvalue(f, i)
      if not rawequal(x, ups[i]) then setupvalue(f, i, ups[i]) end
    end
  end
  return changed
end

return snap, restore
