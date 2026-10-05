// Display-only formatting, off the UI thread. No painter files or session data are written.
import init, { Config, formatCode, IndentType, LuaVersion, OutputVerification, CallParenType } from './vendor/stylua/stylua_lib_web.js';

const ready = init();
// Live polling and replay commonly revisit the current and preceding source.
const cache = new Map();
function format(text) {
  if (cache.has(text)) return cache.get(text);
  const config = Config.new();
  config.syntax = LuaVersion.Lua54;
  config.column_width = 100;
  config.indent_type = IndentType.Spaces;
  config.indent_width = 2;
  config.call_parentheses = CallParenType.Input;
  const result = formatCode(text, config, undefined, OutputVerification.Full);
  cache.set(text, result);
  if (cache.size > 2) cache.delete(cache.keys().next().value);
  return result;
}
self.onmessage = async ({ data: { id, input, previous } }) => {
  let text = input, before = previous, formatted = false;
  try {
    await ready;
    const result = format(input);
    // Compare real before/after snapshots in the same representation for change highlights.
    before = previous === null ? null : format(previous);
    text = result;
    formatted = true;
  } catch (e) { /* Invalid or unsupported Lua (or unavailable WASM): retain the original bytes. */ }
  self.postMessage({ id, input, previous, text, before, formatted });
};
