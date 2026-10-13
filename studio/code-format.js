// Display-only formatting, off the UI thread. No painter files or session data are written.
import init, { Config, formatCode, IndentType, LuaVersion, OutputVerification, CallParenType } from './vendor/stylua/stylua_lib_web.js';

import './code-display.js';
const { diffLines, lineHTML } = self.CodeDisplay;
const ready = init();
// Live polling and replay commonly revisit the current and preceding source.
const cache = new Map();
let cacheBytes = 0, pending = null, scheduled = false;
function format(text) {
  if (cache.has(text)) {
    const result = cache.get(text);
    cache.delete(text); cache.set(text, result);
    return result;
  }
  const config = Config.new();
  config.syntax = LuaVersion.Lua54;
  config.column_width = 100;
  config.indent_type = IndentType.Spaces;
  config.indent_width = 2;
  config.call_parentheses = CallParenType.Input;
  const result = formatCode(text, config, undefined, OutputVerification.Full);
  const bytes = 2 * (text.length + result.length);
  if (bytes <= 8 * 1024 * 1024) {
    while (cache.size && (cache.size >= 16 || cacheBytes + bytes > 8 * 1024 * 1024)) {
      const key = cache.keys().next().value;
      cacheBytes -= 2 * (key.length + cache.get(key).length); cache.delete(key);
    }
    cache.set(text, result); cacheBytes += bytes;
  }
  return result;
}
self.onmessage = ({ data }) => {
  pending = data.cancel ? null : data;
  if (!pending) return;
  if (scheduled) return;
  scheduled = true;
  ready.then(() => setTimeout(drain, 0), () => setTimeout(drain, 0));
};
function drain() {
  scheduled = false;
  if (!pending) return;
  const { id, input, previous, changed: rawChanged } = pending;
  pending = null;
  let text = input, before = previous, formatted = false;
  try {
    const result = format(input);
    // Compare real before/after snapshots in the same representation for change highlights.
    before = previous === null ? null : format(previous);
    text = result;
    formatted = true;
  } catch (e) { /* Invalid or unsupported Lua (or unavailable WASM): retain the original bytes. */ }
  const changed = formatted && rawChanged.length ? diffLines(before, text) : new Set(rawChanged);
  const chunks = [], lines = text.split('\n');
  for (let i = 0; i < lines.length; i += 200) {
    chunks.push(lines.slice(i, i + 200).map((line, k) => lineHTML(line, true, i + k, changed.has(i + k))).join(''));
  }
  self.postMessage({ id, input, previous, text, before, formatted, chunks, changed:[...changed] });
}
