// Real viewer boundary: synthetic transports, actual DOM, formatting worker and input handlers.
// NODE_PATH must include an existing Playwright installation. STUDIO_BASELINE enables old/new comparisons.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { chromium } = require('playwright');
const root = __dirname;
const candidate = process.env.STUDIO_INDEX || path.join(root, 'index.html');
const origin = 'http://studio.test';
const baseline = process.env.STUDIO_BASELINE && fs.readFileSync(process.env.STUDIO_BASELINE, 'utf8');
const report = { checks:[], comparisons:0, pageErrors:[], timings:{} };
const stamp = k => new Date(Date.UTC(2026, 9, 6, 10, k)).toISOString();
const ev = (kind, fields = {}, k = 0) => ({ kind, ts:stamp(k), ...fields });
const initial = [ev('start', { cwd:'/painter' }), ev('think', { text:'# First\n\n**Paint** & `code` <safe>\n\n- first\n  continued\n- second' }, 1),
  ev('paint', { code:'local first=1', out:'ok' }, 2), ev('look', { text:'mode normal' }, 3),
  ev('think', { text:'A new **thought**.' }, 9), ev('paint', { scratch:true, new_scratch:true, code:'local scratch=2', out:'ok' }, 10),
  ev('paint', { scratch:true, new_scratch:true, code:'bad(', err:true, out:'error' }, 11), ev('paint', { scratch:true, code:'local fresh=3', out:'ok' }, 12),
  ev('look', { text:'mode normal' }, 13), ev('paint', { code:'local pending=4' }, 14),
  ev('sitting', { n:2 }, 20), ev('say', { text:'## Done\n\n3. ordered\n4. list\n\n---\n\n_literal_ <tag>' }, 21)];

async function viewer(browser, fixture, { old = false, staticMode = false } = {}) {
  const page = await browser.newPage();
  const state = { events:structuredClone(fixture), updates:[], epoch:'one', nupdates:0, file:'local live=99\n', files:0 };
  page.on('pageerror', e => report.pageErrors.push(String(e)));
  await page.route('**/*', async route => {
    const u = new URL(route.request().url());
    if (u.pathname === '/') {
      const html = old ? baseline : fs.readFileSync(candidate, 'utf8');
      return route.fulfill({ contentType:'text/html', body:staticMode ? html.replace('<script src="code-display.js">', '<script>window.STUDIO_STATIC=true;</script><script src="code-display.js">') : html });
    }
    if (u.pathname === '/api/sessions' || u.pathname === '/data/sessions.json') return route.fulfill({ json:[{ p:'fixture', folder:'fixture', painter:true, sittings:2, mtime:0, title:'Fixture' }] });
    if (u.pathname === '/paintings.json') return route.fulfill({ json:[] });
    if (u.pathname === '/api/events' || u.pathname.endsWith('/events.json')) {
      const since = staticMode || u.searchParams.has('epoch') && u.searchParams.get('epoch') !== state.epoch ? 0 : +(u.searchParams.get('since') || 0);
      const updates = state.updates.splice(0);
      return route.fulfill({ json:{ epoch:state.epoch, sittings:2, events:state.events.slice(since), updates, nupdates:state.nupdates, total:state.events.length } });
    }
    if (u.pathname === '/api/file' || u.pathname.includes('/file/')) { state.files++; return route.fulfill({ contentType:'text/plain', body:state.file }); }
    const file = path.join(root, u.pathname);
    if (!u.pathname.includes('..') && fs.existsSync(file) && fs.statSync(file).isFile()) {
      const contentType = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.wasm') ? 'application/wasm' : file.endsWith('.css') ? 'text/css' : 'application/octet-stream';
      return route.fulfill({ contentType, body:fs.readFileSync(file) });
    }
    return route.fulfill({ status:404, body:'' });
  });
  await page.goto(origin + '/?p=fixture');
  await page.waitForFunction(() => events.length > 0 && !document.querySelector('#togglecode').disabled);
  await page.waitForFunction(() => document.querySelector('#code .src')?.textContent.includes('live'));
  return { page, state };
}
async function seek(page, i) {
  await page.locator('#scrub').evaluate((el, i) => { el.value = String(i); el.dispatchEvent(new Event('input', { bubbles:true })); }, i);
  // Let the bounded DOM batches and the warmed formatting worker settle before reading the display.
  await page.waitForFunction(i => pos === i && codeBatch === null, i);
  await page.waitForFunction(() => !document.querySelector('#togglecode').disabled);
  await page.evaluate(() => new Promise(resolve => {
    let timer; const observer = new MutationObserver(() => { clearTimeout(timer); timer = setTimeout(done, 80); });
    const done = () => { observer.disconnect(); resolve(); };
    observer.observe(document.querySelector('#jbody'), { childList:true, subtree:true, attributes:true });
    observer.observe(document.querySelector('#code'), { childList:true, subtree:true });
    timer = setTimeout(done, 80);
  }));
}
async function snapshot(page) {
  return page.evaluate(() => ({ journal:document.querySelector('#jbody').innerHTML,
    code:[...document.querySelectorAll('#code .src')].map(n => n.textContent),
    changed:[...document.querySelectorAll('#code .new')].map(n => +n.querySelector('.ln').textContent),
    file:document.querySelector('#file').textContent }));
}
async function compare(page, other) {
  const a = await snapshot(page), b = await snapshot(other);
  assert.deepEqual({ code:a.code, changed:a.changed, file:a.file }, { code:b.code, changed:b.changed, file:b.file });
  if (a.journal !== b.journal) {
    let at = 0; while (at < Math.min(a.journal.length,b.journal.length) && a.journal[at] === b.journal[at]) at++;
    assert.fail('journal mismatch at position ' + await page.evaluate(() => pos) + ', offset ' + at + ': ' + JSON.stringify({ candidate:a.journal.slice(at,at+160), baseline:b.journal.slice(at,at+160), lengths:[a.journal.length,b.journal.length] }));
  }
  report.comparisons++;
}
async function pollUpdate(v, i, replacement) {
  v.state.events[i] = replacement; v.state.updates = [[i, replacement]]; v.state.nupdates++;
  await v.page.evaluate(async () => { lastStatic = 0; await poll(); });
}

(async () => {
  const browser = await chromium.launch({ channel:'chrome', headless:true });
  try {
    for (const staticMode of [false, true]) {
      const v = await viewer(browser, initial, { staticMode });
      const old = baseline && await viewer(browser, initial, { old:true, staticMode });
      for (const i of [2, 3, 4, 5, 6, 7, 8, 9, 2, 7, 3, 11]) {
        await seek(v.page, i); if (old) { await seek(old.page, i); await compare(v.page, old.page); }
        const result = await snapshot(v.page);
        if (i === 6) assert.deepEqual(result.code, [], 'failed new scratch resets the log');
        if (i === 7) { assert.equal(result.file, 'scratch.lua'); assert(result.code.join('\n').includes('--@ chunk 1\nlocal fresh = 3')); assert(!result.code.join('\n').includes('scratch = 2')); }
        if (i === 9) assert(!result.code.join('\n').includes('pending'), 'pending chunks are not executed source');
      }
      await seek(v.page, 8); await v.page.evaluate(() => { window.savedThought = document.querySelector('.th[data-k="4"]'); window.savedCode = document.querySelector('#code').firstChild; });
      await seek(v.page, 9);
      assert(await v.page.evaluate(() => window.savedThought === document.querySelector('.th[data-k="4"]') && window.savedCode === document.querySelector('#code').firstChild), 'irrelevant events preserve prepared journal and code DOM');
      const pending = { ...v.state.events[9], out:'ok' };
      await pollUpdate(v, 9, pending); if (old) await pollUpdate(old, 9, pending);
      await seek(v.page, 9); if (old) { await seek(old.page, 9); await compare(v.page, old.page); }
      assert((await snapshot(v.page)).code.join('\n').includes('local pending = 4'), 'late results invalidate an already consumed source prefix');
      const failed = { ...pending, err:true, out:'error' };
      await pollUpdate(v, 9, failed); if (old) await pollUpdate(old, 9, failed);
      await seek(v.page, 9); if (old) { await seek(old.page, 9); await compare(v.page, old.page); }
      assert(!(await snapshot(v.page)).code.join('\n').includes('pending'));
      assert.equal(await v.page.locator('.chip.fail').count(), 2, 'late errors update journal chips');
      await seek(v.page, 7); await seek(v.page, 9);
      v.state.events.push(ev('paint', { code:'local appended=5', out:'ok' }, 22), ev('say', { text:'after appended paint' }, 23));
      await v.page.evaluate(async () => { lastStatic = 0; await poll(); });
      await seek(v.page, 12);
      assert((await snapshot(v.page)).code.join('\n').includes('local appended = 5'), 'new events extend a cached source');
      const opening = await v.page.evaluate(() => { const code = document.querySelector('#code'), first = code.firstChild; document.querySelector('#togglecode').click(); return { same:first === code.firstChild, inert:document.querySelector('#codepane').inert }; });
      assert.deepEqual(opening, { same:true, inert:false });
      await v.page.evaluate(() => { live = false; renderCode('stale.lua', 'local stale=1\n'.repeat(3000), new Set()); renderCode('latest.rs', 'fn main() {}', new Set()); });
      await v.page.waitForFunction(() => document.querySelector('#file').textContent === 'latest.rs');
      await v.page.evaluate(() => new Promise(resolve => setTimeout(resolve, 100)));
      assert.deepEqual((await snapshot(v.page)).code, ['fn main() {}']);
      // Generation reset and a rebuilt stream replace every cache, even with the same source path.
      await v.page.evaluate(() => { live = true; });
      v.state.epoch = 'two'; v.state.events = [ev('start', { cwd:'/painter' }), ev('paint', { code:'local rebuilt=6', out:'ok' }, 1), ev('say', { text:'Rebuilt journal' }, 2)];
      await v.page.evaluate(async () => { lastStatic = 0; await poll(); }); await seek(v.page, 1);
      const rebuilt = await snapshot(v.page); assert(rebuilt.code.join('\n').includes('local rebuilt = 6')); assert(!rebuilt.journal.includes('First'));
      report.checks.push(staticMode ? 'static replay, updates, reset, opening, stale formatting' : 'dynamic replay, updates, reset, opening, stale formatting');
      await v.page.close(); if (old) await old.page.close();
    }
    // Writes, ordered edits, missing substitutions, failed results and file selection ties.
    const edits = [ev('start', { cwd:'/painter' }), ev('write', { path:'/painter/a.rs', content:'fn a() { one(); }', out:'ok' }, 1),
      ev('write', { path:'/painter/b.rs', content:'fn b() {}', out:'ok' }, 2), ev('edit', { path:'/painter/b.rs', edits:[{ oldText:'b()', newText:'bee()' }], out:'ok' }, 3),
      ev('edit', { path:'/painter/a.rs', edits:[{ oldText:'one()', newText:'two()' }, { oldText:'absent', newText:'ignored' }], out:'ok' }, 4),
      ev('edit', { path:'/painter/a.rs', edits:[{ oldText:'two()', newText:'bad()' }], out:'error', err:true }, 5), ev('say', { text:'end' }, 6)];
    const v = await viewer(browser, edits), old = baseline && await viewer(browser, edits, { old:true });
    for (const i of [1, 2, 3, 4, 5, 2, 4]) { await seek(v.page, i); if (old) { await seek(old.page, i); await compare(v.page, old.page); } }
    const r = await snapshot(v.page); assert.equal(r.file, 'a.rs'); assert.deepEqual(r.code, ['fn a() { two(); }']); assert.deepEqual(r.changed, [1]);
    report.checks.push('write/edit reconstruction and first-file tie ordering'); await v.page.close(); if (old) await old.page.close();
    // A sliding 251-event journal retains exact grouping and stable block nodes.
    const large = [ev('start', { cwd:'/painter' }), ev('paint', { code:'local stable=1', out:'ok' }, 1)];
    for (let k = 2; k < 330; k++) large.push(ev(k % 3 === 0 ? 'think' : 'cmd', { text:k % 3 === 0 ? ('## Thought ' + k + '\n\n' + 'A **long** line with `code` & <safe>.\n').repeat(30) : 'echo ' + k, out:'ok' }, k));
    const largeV = await viewer(browser, large), largeOld = baseline && await viewer(browser, large, { old:true });
    for (const i of [249, 250, 251, 299, 300, 120, 301]) { await seek(largeV.page, i); if (largeOld) { await seek(largeOld.page, i); await compare(largeV.page, largeOld.page); } }
    await seek(largeV.page, 300); await largeV.page.evaluate(() => { window.retained = document.querySelector('.th[data-k="297"]'); }); await seek(largeV.page, 301);
    assert(await largeV.page.evaluate(() => window.retained === document.querySelector('.th[data-k="297"]')));
    for (const [label, page] of [['candidate', largeV.page], ['baseline', largeOld?.page]]) if (page) {
      await seek(page, 100);
      report.timings[label] = await page.evaluate(() => { const samples = []; live = false; for (let k = 100; k < 130; k++) { const t = performance.now(); show(k); samples.push(performance.now() - t); } return { samplesMs:samples, totalMs:samples.reduce((a,b) => a+b, 0) }; });
    }
    report.checks.push('sliding journal equivalence and retained blocks'); await largeV.page.close(); if (largeOld) await largeOld.page.close();
    // Check every visible typewriter tick, including partial markdown and escaped literals.
    const thought = '# Title\n\nplain **bold** and _italic_ <tag> & `code`\nsecond line\n\n3. item **one**\n  continued\n4. next\n\n---\n\nFinal *word*.';
    const typedFixture = [ev('start', { cwd:'/painter' }), ev('paint', { code:'local x=1', out:'ok' }, 1), ev('say', { text:'Before typing' }, 2)];
    const typing = await viewer(browser, typedFixture);
    await typing.page.clock.install({ time:new Date('2026-10-06T10:00:00Z') });
    await typing.page.clock.pauseAt(new Date('2026-10-06T10:00:00Z'));
    await typing.page.evaluate(e => { events.push(e); live = true; show(events.length - 1); }, ev('think', { text:thought }, 3));
    const originalMd = baseline && baseline.slice(baseline.indexOf('function inline(s) {'), baseline.indexOf('// one line without its markdown marks'));
    if (originalMd) await typing.page.evaluate(({ src, esc }) => { window.oldMarkdown = new Function('text', esc + '\n' + src + '\nreturn md(text);'); }, { src:originalMd, esc:baseline.split('\n').find(line => line.startsWith('const esc = ')) });
    for (let n = 3; n <= thought.length + 2; n += 3) {
      await typing.page.clock.runFor(30);
      const frame = await typing.page.evaluate(({ text, n }) => { const end = Math.min(n, text.length), body = document.querySelector('.th.last').innerHTML; const expected = window.oldMarkdown ? oldMarkdown(text.slice(0,end)) + (end < text.length ? '<span class="cursor"></span>' : '') : null; return { body, expected }; }, { text:thought, n });
      if (frame.expected !== null) assert.equal(frame.body, frame.expected, 'typewriter frame ' + n);
    }
    assert.equal(await typing.page.locator('.th.last').innerHTML(), '<div class="mh">Title</div><p>plain <b>bold</b> and <i>italic</i> &lt;tag&gt; &amp; <code>code</code><br>second line</p><ol start="3"><li>item <b>one</b> continued</li><li>next</li></ol><hr><p>Final <i>word</i>.</p>');
    assert.equal(await typing.page.locator('.th.last .cursor').count(), 0);
    assert.equal(await typing.page.locator('.th.last script').count(), 0);
    report.checks.push('every typewriter prefix preserves markdown'); await typing.page.close();
    assert.deepEqual(report.pageErrors, []);
    fs.writeFileSync(path.join(root, 'replay-verification.json'), JSON.stringify(report, null, 2) + '\n');
    console.log(JSON.stringify(report));
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
