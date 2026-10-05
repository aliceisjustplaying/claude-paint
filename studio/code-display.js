// Shared display markup: worker preparation and raw fallback use the same line mapping.
(() => {
const esc = s => String(s).replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
function diffLines(a, b) { if (a == null) return new Set(); const A = new Set(a.split('\n')); const out = new Set(); b.split('\n').forEach((l, i) => { if (!A.has(l) && l.trim()) out.add(i); }); return out; }
const KW = /\b(fn|let|mut|if|else|for|in|while|loop|match|return|use|pub|struct|enum|impl|const|static|move|as|ref|mod|where|break|continue|local|function|end|then|do|elseif|nil|and|or|not|repeat|until|true|false|Some|None|Ok|Err|self|Self)\b/;
function hl(line, lua) {
  const re = lua ? /(--.*$)|("(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*')|\b(\d+(?:\.\d+)?(?:e-?\d+)?)\b|(\b[A-Za-z_]\w*\b)(?=\s*\()|(\b[A-Za-z_]\w*\b)/g
                 : /(\/\/.*$)|("(?:\\.|[^"\\])*")|\b(\d+(?:\.\d+)?(?:e-?\d+)?(?:f32|f64|usize|u8|u32|i32|u64)?)\b|(\b[A-Za-z_]\w*!?)(?=\s*[(!])|(\b[A-Za-z_]\w*\b)/g;
  let out = '', last = 0, m;
  while ((m = re.exec(line))) { out += esc(line.slice(last, m.index)); const t = esc(m[0]);
    out += m[1] ? `<span class="c">${t}</span>` : m[2] ? `<span class="s">${t}</span>` : m[3] ? `<span class="n">${t}</span>`
         : m[4] ? (KW.test(m[4]) ? `<span class="k">${t}</span>` : m[4].endsWith('!') ? `<span class="m">${t}</span>` : `<span class="f">${t}</span>`)
         : KW.test(m[5]) && m[5].match(KW)[0] === m[5] ? `<span class="k">${t}</span>` : /^[A-Z]/.test(m[5]) ? `<span class="m">${t}</span>` : t;
    last = m.index + m[0].length; if (!m[0].length) re.lastIndex++; }
  return out + esc(line.slice(last));
}
function lineHTML(line, lua, i, changed) {
  return '<div class="code-line' + (changed ? ' new' : '') + '"><span class="ln">' + (i + 1) + '</span><span class="src">' + (hl(line, lua) || ' ') + '</span></div>';
}
self.CodeDisplay = { diffLines, lineHTML };
})();
