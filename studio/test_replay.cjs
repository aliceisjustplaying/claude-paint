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

async function viewer(browser, fixture, { old = false, staticMode = false, liveFile = true, streamMode = false, sittings = 2, codeReady = true, failing = false, listed = true } = {}) {
  const page = await browser.newPage();
  const state = { events:structuredClone(fixture), updates:[], epoch:'one', nupdates:0, file:'local live=99\n', files:0, sittings, failing };
  page.on('pageerror', e => report.pageErrors.push(String(e)));
  await page.route('**/*', async route => {
    const u = new URL(route.request().url());
    if (u.pathname === '/') {
      const html = old ? baseline : fs.readFileSync(candidate, 'utf8');
      return route.fulfill({ contentType:'text/html', body:staticMode ? html.replace('<script src="code-display.js">', '<script>window.STUDIO_STATIC=true;</script><script src="code-display.js">') : html });
    }
    if (u.pathname === '/api/sessions' || u.pathname === '/data/sessions.json') return route.fulfill({ json:listed ? [{ p:'fixture', folder:'fixture', painter:true, sittings, mtime:0, title:'Fixture' }] : [] });
    if (u.pathname === '/paintings.json') return route.fulfill({ json:[] });
    if (u.pathname === '/api/events' || u.pathname.endsWith('/events.json')) {
      if (state.failing) return route.fulfill({ status:503, body:'' });
      const since = staticMode || u.searchParams.has('epoch') && u.searchParams.get('epoch') !== state.epoch ? 0 : +(u.searchParams.get('since') || 0);
      const updates = state.updates.splice(0);
      return route.fulfill({ json:{ epoch:state.epoch, sittings:state.sittings, events:state.events.slice(since), updates, nupdates:state.nupdates, total:state.events.length } });
    }
    if (u.pathname === '/api/file' || u.pathname.includes('/file/')) { state.files++; return route.fulfill({ contentType:'text/plain', body:state.file }); }
    if (u.pathname === '/img' || u.pathname.includes('/img/')) return route.fulfill({ contentType:'image/svg+xml', body:'<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="800"><rect width="1000" height="800" fill="green"/></svg>' });
    const file = path.join(root, u.pathname);
    if (!u.pathname.includes('..') && fs.existsSync(file) && fs.statSync(file).isFile()) {
      const contentType = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.wasm') ? 'application/wasm' : file.endsWith('.css') ? 'text/css' : 'application/octet-stream';
      return route.fulfill({ contentType, body:fs.readFileSync(file) });
    }
    return route.fulfill({ status:404, body:'' });
  });
  await page.goto(origin + '/?p=fixture' + (streamMode ? '&stream=1' : ''));
  if (failing) return { page, state };
  await page.waitForFunction(codeReady => events.length > 0 && (!codeReady || !document.querySelector('#togglecode').disabled), codeReady);
  if (liveFile) await page.waitForFunction(() => document.querySelector('#code .src')?.textContent.includes('live'));
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
    const dieRoll = [ev('start', { cwd:'/painter' }), ev('tool', { name:'write_concepts', out:'Saved notes/concepts.md.' }, 1),
      ev('say', { text:'Saved the eight pictures and their estimates.' }, 2),
      ev('sitting', { n:1 }, 10), ev('start', { cwd:'/painter' }, 10), ev('paint', { code:'canvas{}', out:'ok' }, 12),
      ev('start', { cwd:'/painter' }, 20), ev('look', { text:'' }, 21),
      ev('sitting', { n:2 }, 60), ev('start', { cwd:'/painter' }, 60), ev('say', { text:'end' }, 62)];
    for (const staticMode of [false, true]) {
      const v = await viewer(browser, dieRoll, { staticMode });
      assert.deepEqual(await v.page.locator('#strip .sep').allTextContents(), ['sitting 1', 'sitting 2']);
      assert.equal(await v.page.locator('#clock').textContent(), '0:04:00');
      assert.match(await v.page.locator('#clocklbl').textContent(), /2 sittings/);
      for (const [i, clock, label] of [[2, '0:00:00', 'at'], [5, '0:02:00', 'at · sitting 1 of 2'], [7, '0:02:00', 'at · sitting 1 of 2']]) {
        await seek(v.page, i);
        assert.equal(await v.page.locator('#clock').textContent(), clock);
        assert.equal(await v.page.locator('#clocklbl').textContent(), label);
      }
      assert.match(await v.page.locator('#jbody').textContent(), /Saved the eight pictures/);
      await v.page.close();
    }
    const first = await viewer(browser, [ev('sitting', { n:1 }), ev('start', { cwd:'/painter' }), ev('paint', { code:'canvas{}', out:'ok' }, 2)], { sittings:1 });
    assert.equal(await first.page.locator('#strip .sep').textContent(), 'sitting 1');
    assert.equal(await first.page.locator('#clock').textContent(), '0:02:00');
    assert.equal(await first.page.locator('#clocklbl').textContent(), 'painted for');
    await first.page.close();
    const waiting = await viewer(browser, dieRoll.slice(0, 3).concat(dieRoll[4]), { staticMode:true, sittings:0, codeReady:false, liveFile:false });
    waiting.state.events = dieRoll.slice(0, 6); waiting.state.sittings = 1;
    await waiting.page.evaluate(async () => { lastStatic = 0; await poll(); });
    assert.equal(await waiting.page.locator('#strip .sep').textContent(), 'sitting 1');
    assert.equal(await waiting.page.locator('#clock').textContent(), '0:02:00');
    assert.equal(await waiting.page.locator('#clocklbl').textContent(), 'painted for');
    assert.equal(await waiting.page.evaluate(() => events.length), 6);
    await waiting.page.close();
    for (const staticMode of [false, true]) {
      const firstSitting = [ev('sitting', { n:1 }), ev('start', { cwd:'/painter' }), ev('paint', { code:'local first=1', out:'ok' }, 1),
        ev('look', { text:'' }, 2), ev('image', { img:0, look:'' }, 2), ev('start', { cwd:'/painter' }, 10)];
      const rewound = await viewer(browser, firstSitting, { staticMode, sittings:1 });
      await seek(rewound.page, 4);
      await rewound.page.waitForFunction(() => document.querySelector('#img').naturalWidth > 0);
      const canvas = (await rewound.page.locator('#img').getAttribute('src')).replace(/epoch=[^&]+/, 'epoch=');
      rewound.state.events.splice(5, 0, ev('sitting', { n:2 }, 10));
      rewound.state.events.push(ev('paint', { code:'local second=2', out:'ok' }, 12));
      rewound.state.sittings = 2;
      if (!staticMode) rewound.state.epoch = 'two';
      await rewound.page.evaluate(async () => { lastStatic = 0; await poll(); });
      await rewound.page.waitForFunction(() => !document.querySelector('#togglecode').disabled && codeBatch === null && document.querySelector('#img').naturalWidth > 0);
      assert.equal((await rewound.page.locator('#img').getAttribute('src')).replace(/epoch=[^&]+/, 'epoch='), canvas);
      assert.equal(await rewound.page.locator('#nopic').isVisible(), false);
      assert.equal(await rewound.page.evaluate(() => pos), 4);
      assert.equal(await rewound.page.evaluate(() => live), false);
      assert.match((await snapshot(rewound.page)).code.join('\n'), /local first\s*=\s*1/);
      assert.equal(await rewound.page.locator('#clocklbl').textContent(), 'at · sitting 1 of 2');
      await rewound.page.close();
    }
    report.checks.push('sitting labels and clocks exclude die rolls, startup-only sessions and gaps in API/static replay');
    const canvas = [ev('start'), ev('look', {text:'scratch True'}, 1), ev('image', {img:0, look:'scratch True'}, 2),
      ev('paint', {scratch:true, code:'work{}', out:'ok'}, 3), ev('look', {text:'scratch True, crop 100,100,300,300'}, 4),
      ev('image', {img:1, look:'scratch True, crop 100,100,300,300'}, 5), ev('look', {text:''}, 6), ev('image', {img:2, look:''}, 7),
      ev('paint', {scratch:true, new_scratch:true, code:'work{}', out:'ok'}, 8), ev('look', {text:'scratch True, crop 100,100,300,300'}, 9),
      ev('image', {img:3, look:'scratch True, crop 100,100,300,300'}, 10), ev('look', {text:'scratch True, palette True'}, 11),
      ev('image', {img:4, look:'scratch True, palette True'}, 12), ev('paint', {scratch:true, new_scratch:true, err:true, code:'bad(', out:'error'}, 13),
      ev('look', {text:'scratch True, crop 100,100,300,300'}, 14), ev('image', {img:5, look:'scratch True, crop 100,100,300,300'}, 15),
      ev('look', {text:'crop 100,100,300,300'}, 16), ev('image', {img:6, look:'crop 100,100,300,300'}, 17),
      ev('paint', {code:'work{}', out:'ok'}, 18), ev('look', {text:''}, 19), ev('image', {img:7, look:''}, 20), ev('say', {text:'done'}, 21)];
    for (const staticMode of [false, true]) for (const streamMode of [false, true]) {
      const v = await viewer(browser, canvas, {staticMode, streamMode, liveFile:false});
      const times = await v.page.evaluate(() => events.map(e => hhmm(e.ts)));
      assert.deepEqual(await v.page.evaluate(() => [new Date(2026, 0, 1, 9, 4, 5), new Date(2026, 0, 1, 21, 4, 5)].map(hhmm)), ['9:04 AM', '9:04 PM']);
      await seek(v.page, 5);
      assert.equal(await v.page.locator('#cap').textContent(), `Scratch canvas · ${times[2]} · 3 min earlier`);
      assert.equal(await v.page.locator('#cropbox').isVisible(), false);
      assert.equal(await v.page.locator('#img2').isVisible(), true);
      assert.equal(await v.page.locator('#cap2').textContent(), `Close-up · ${times[5]}`);
      assert.match(await v.page.locator('#cap2').getAttribute('title'), /No outline: the whole look is older or unavailable/);
      await seek(v.page, 8);
      assert.equal(await v.page.locator('#img').getAttribute('src'), null);
      assert.equal(await v.page.locator('#nopic').textContent(), 'No current scratch look');
      await seek(v.page, 10);
      assert.equal(await v.page.evaluate(() => viewI), 3);
      assert.equal(await v.page.locator('#cap').textContent(), `Scratch canvas · close-up · ${times[10]}`);
      assert.equal(await v.page.locator('#cropbox').isVisible(), false);
      await v.page.evaluate(() => { events[11] = {...events[11], kind:'paint', scratch:true, code:'work{}', out:'ok'}; show(11); });
      assert.equal(await v.page.locator('#cap').textContent(), `Scratch canvas · close-up · ${times[10]} · 1 min earlier`);
      await v.page.evaluate(e => { events[11] = e; }, canvas[11]);
      await seek(v.page, 13);
      assert.equal(await v.page.evaluate(() => picturesAt(pos, true, true)[0]), -1);
      assert.equal(await v.page.evaluate(() => paletteAt(pos)), -1);
      await v.page.evaluate(() => { delete events[13].new_scratch; pictures(); });
      assert.equal(await v.page.evaluate(() => picturesAt(pos, true, true)[0]), -1);
      assert.equal(await v.page.evaluate(() => paletteAt(pos)), -1);
      await seek(v.page, 15);
      assert.equal(await v.page.evaluate(() => viewI), 5);
      await seek(v.page, 17);
      assert.equal(await v.page.evaluate(() => viewI), 2);
      assert.equal(await v.page.locator('#cap').textContent(), `The painting · ${times[7]} · 10 min earlier`);
      assert.equal(await v.page.locator('#cropbox').isVisible(), false);
      await seek(v.page, 18);
      assert.equal(await v.page.locator('#cap').textContent(), `The painting · ${times[7]} · 11 min earlier`);
      assert.equal(await v.page.locator('#cap2').textContent(), `Close-up · ${times[17]} · 1 min earlier`);
      assert.equal(await v.page.locator('#pipcap').textContent(), `Scratch canvas · close-up · ${times[15]}`);
      assert.doesNotMatch(await v.page.locator('#cap2').getAttribute('title'), /looking at now/);
      await v.page.setViewportSize({width:390, height:844});
      for (const id of ['cap', 'cap2']) assert.equal(await v.page.locator('#' + id).evaluate(el => el.scrollWidth <= el.clientWidth), true, id + ' snapshot warning must wrap, not clip');
      await v.page.setViewportSize({width:1280, height:720});
      await seek(v.page, 20);
      assert.equal(await v.page.evaluate(() => viewI), 7);
      assert.equal(await v.page.locator('#cap').textContent(), `The painting · ${times[20]}`);
      await seek(v.page, 2);
      assert.equal(await v.page.evaluate(() => viewI), 0);
      assert.equal(await v.page.locator('#cap').textContent(), `Scratch canvas · ${times[2]}`);
      await v.page.evaluate(() => { live = false; show(10); });
      await pollUpdate(v, 8, {...canvas[8], new_scratch:undefined, out:'a fresh scratch canvas (the last one\'s log is paintings/lua/scratch-1.lua)\nok'});
      assert.equal(await v.page.evaluate(() => picturesAt(pos, true, true)[0]), 10);
      await v.page.evaluate(() => { events[8] = {...events[8], out:'ok'}; pictures(); });
      assert.equal(await v.page.evaluate(() => picturesAt(pos, true, true)[0]), 2);
      assert.equal(await v.page.locator('#cap').textContent(), `Scratch canvas · ${times[2]} · 8 min earlier`);
      await pollUpdate(v, 8, {...canvas[8], new_scratch:undefined, out:'x'.repeat(1500)});
      assert.equal(await v.page.evaluate(() => picturesAt(pos, true, true)[0]), 2);
      await pollUpdate(v, 8, {...canvas[8], new_scratch:undefined, scratch_reset:true, out:'x'.repeat(1500)});
      assert.equal(await v.page.evaluate(() => picturesAt(pos, true, true)[0]), 10);
      await v.page.close();
    }
    report.checks.push('canvas reset boundaries, stale whole-look ages, safe crop outlines and rewind in normal/stream and API/static views');
    const implicitReset = [ev('start', { cwd:'/painter' }), ev('paint', { scratch:true, new_scratch:true, code:'local old_scratch=1', out:'ok' }, 1),
      ev('paint', { scratch:true, err:true, code:'bad(', out:'error' }, 2),
      ev('paint', { scratch:true, scratch_reset:true, code:'local fresh_scratch=2', out:'a fresh scratch canvas (the last one\'s log is paintings/lua/scratch-1.lua)\nok' }, 3),
      ev('say', { text:'after the reset' }, 4)];
    const implicit = await viewer(browser, implicitReset, { liveFile:false });
    await seek(implicit.page, 4); await seek(implicit.page, 3);
    const reset = await snapshot(implicit.page);
    assert.equal(reset.file, 'scratch.lua');
    assert(reset.code.join('\n').includes('--@ chunk 1\nlocal fresh_scratch = 2'), 'a scratch reset reported by the result starts a new log');
    assert(!reset.code.join('\n').includes('old_scratch'), 'a scratch reset reported by the result drops the previous canvas');
    await seek(implicit.page, 2);
    assert((await snapshot(implicit.page)).code.join('\n').includes('--@ chunk 1\nlocal old_scratch = 1'), 'a failed scratch chunk without a reset keeps the earlier log');
    report.checks.push('implicit scratch resets start a new reconstructed log, failed chunks without a reset keep it'); await implicit.page.close();
    for (const streamMode of [false, true]) {
      const v = await viewer(browser, [ev('start'), ev('paint', {code:'work{}', out:'ok'}, 1),
        ev('look', {text:''}, 2), ev('image', {img:0, look:''}, 3)], {streamMode, liveFile:false});
      await v.page.waitForFunction(() => document.getElementById('img').naturalWidth > 0);
      const painting = await v.page.locator('#img').getAttribute('src');
      const remains = async () => assert(await v.page.evaluate(src => {
        const im = document.getElementById('img');
        return im.getAttribute('src') === src && im.complete && im.naturalWidth > 0 && document.getElementById('nopic').hidden;
      }, painting), 'the painting stays visible while awaiting the first scratch look');
      await v.page.evaluate(e => { events.push(e); live = true; show(events.length - 1); },
        ev('paint', {scratch:true, new_scratch:true, code:'work{}', out:'ok', ts:new Date().toISOString()}, 4));
      await remains();
      assert.equal(await v.page.locator('#cap').textContent(), 'The painting · awaiting first scratch look');
      assert.equal(await v.page.locator('#pip').isVisible(), false);
      let release;
      const held = new Promise(resolve => { release = resolve; });
      await v.page.route(u => u.pathname === '/img' && u.searchParams.get('i') === '1', async route => { await held; await route.fallback(); });
      const requested = v.page.waitForRequest(request => { const u = new URL(request.url()); return u.pathname === '/img' && u.searchParams.get('i') === '1'; });
      await v.page.evaluate(e => { events.push(e); show(events.length - 1); }, ev('image', {img:1, look:'scratch True', ts:new Date().toISOString()}, 5));
      await requested;
      await v.page.waitForFunction(() => document.getElementById('img').dataset.want.includes('i=1'));
      await remains();
      assert.equal(await v.page.locator('#cap').textContent(), 'The painting · awaiting first scratch look');
      assert.equal(await v.page.locator('#pip').isVisible(), false);
      release();
      await v.page.waitForFunction(() => { const im = document.getElementById('img'); return im.dataset.loadedWant === im.dataset.want && im.complete && im.naturalWidth > 0; });
      assert.notEqual(await v.page.locator('#img').getAttribute('src'), painting);
      assert.match(await v.page.locator('#cap').textContent(), /^Scratch canvas · /);
      await v.page.waitForFunction(() => { const im = document.getElementById('pipimg'); return !document.getElementById('pip').hidden && im.complete && im.naturalWidth > 0; });
      assert.equal(await v.page.locator('#pipimg').getAttribute('data-picture'), '0');
      await v.page.close();
    }
    report.checks.push('first scratch keeps the painting visible until the delayed scratch image loads in normal/stream views');
    const waits = JSON.parse(fs.readFileSync(path.join(root, 'fixtures/monet-waits.json'), 'utf8')).cases;
    for (const staticMode of [false, true]) {
      const sample = waits[1];
      const v = await viewer(browser, [ev('start'), ev('paint', { code:'work{}', out:sample.before }), sample.event], { staticMode, liveFile:false });
      await seek(v.page, 2);
      assert.deepEqual(await v.page.locator('#jbody .chip').allTextContents(), ['🖌 paints', '🖌 paints', '🕒 waits 1 week']);
      await v.page.evaluate(() => { const link = document.createElement('link'); link.rel = 'stylesheet'; link.href = 'stream.css'; document.head.append(link); document.body.classList.add('stream'); });
      assert.equal(await v.page.locator('#jbody .chip.wait').textContent(), '🕒 waits 1 week');
      const result = await v.page.evaluate(cases => {
        const saved = events;
        const real = cases.map(({ before, event }) => { events = before ? [{kind:'paint', scratch:event.scratch, out:before}, event] : [event]; return waitChip(event, events.length - 1)?.[2]; });
        const extract = (code, out = 'ok', fields = {}) => { const e = {kind:'paint', code, out, ...fields}; events = [e]; return waitChip(e, 0)?.[2] || null; };
        const fallback = ['wait(120)', 'wait(3*24*60)', 'wait(6*7*24*60)', 'wait(60); wait(60)', 'wait(0)', 'for i=1,3 do wait(60) end', 'wait(minutes)', 'wait(60); work{}', '-- wait(60)\nwork{}', 'print("wait(60)")', 'if false then wait(60) end', '--[=[ wait(60) ]=]\nwork{}'].map(code => extract(code));
        const failed = extract('wait(60)', 'day 2, 09:00\nok', {err:true});
        const missing = extract('wait(60)', 'day 2, 09:00\nok');
        const before = {kind:'paint', code:'work{}', out:'day 1, 09:00\nok'};
        const conditional = ['local skipped = false and wait(10080); work{}', 'local skipped = true or wait(10080); work{}', 'wait(0); work{}', 'other.wait(10080); work{}', 'local wait = print; wait(10080); work{}', 'wait(0*60); work{}', 'wait(60*0); work{}', 'wait(.0*60); work{}', 'local wait, x = print, 1; wait(10080); work{}', 'wait(minutes); work{}', 'wait(0+0); work{}'].map(code => {
          const e = {kind:'paint', code, out:'day 1, 09:19\nok'}; events = [before, e]; return waitChip(e, 1)?.[2] || null;
        });
        const resetWait = {kind:'paint', scratch:true, code:'wait(20160)', out:'day 15, 09:00\nok'};
        events = [{...before, scratch:true, out:'day 8, 09:00\nok'}, {kind:'paint', scratch:true, new_scratch:true, code:'error("failure")', err:true, out:'failure'}, resetWait];
        const reset = waitChip(resetWait, 2)?.[2];
        const fresh = {...resetWait, out:"[scratch canvas]\na scratch canvas beside the painting, set up as the painting's canvas was\nday 15, 09:00\nok"};
        events = [fresh]; const implicit = waitChip(fresh, 0)?.[2];
        const shadowed = {kind:'paint', code:'wait(10080); work{}', out:'day 1, 09:39\nok'};
        events = [{...before, code:'wait = print', out:'day 1, 09:19\nok'}, shadowed];
        const earlierShadow = waitChip(shadowed, 1)?.[2] || null;
        const prose = ['print("wait, then paint")', '-- wait, then paint\nprint("hello")', '--[=[wait = print]=]\nprint("hello")'].map(code => {
          const e = {kind:'paint', code:'wait(10080)', out:'day 8, 09:00\nok'};
          events = [{...before, code}, e]; return waitChip(e, 1)?.[2] || null;
        });
        events = saved;
        return {real, fallback, failed, missing, conditional, reset, implicit, earlierShadow, prose, durations:[1, 59, 60, 120, 1680, 4320, 60480, 90*1440, 525960, 0, -1].map(waitDuration)};
      }, waits);
      assert.deepEqual(result.real, ['waits 1 week on the scratch canvas', 'waits 1 week', 'waits 1 week on the scratch canvas', 'waits 1 week']);
      assert.deepEqual(result.fallback, ['waits 2 hours', 'waits 3 days', 'waits 6 weeks', 'waits 2 hours', null, null, null, null, null, null, null, null]);
      assert.equal(result.failed, null); assert.equal(result.missing, null);
      assert.deepEqual(result.conditional, Array(11).fill(null)); assert.equal(result.earlierShadow, null);
      assert.deepEqual(result.prose, ['waits 1 week', 'waits 1 week', 'waits 1 week']);
      assert.equal(result.reset, 'waits 2 weeks on the scratch canvas');
      assert.equal(result.implicit, 'waits 2 weeks on the scratch canvas');
      assert.deepEqual(result.durations, ['1 minute', '59 minutes', '1 hour', '2 hours', '1 day', '3 days', '6 weeks', '2 months', '1 year', null, null]);
      for (const code of ['local skipped = false and wait(10080); work{}', 'local wait = print; wait(10080); work{}', 'wait(0*60); work{}']) {
        await v.page.evaluate(code => {
          events = [{kind:'paint', code:'work{}', out:'day 1, 09:00\nok'}, {kind:'paint', code, out:'day 1, 09:19\nok'}].map(e => ({...e, ts:'2026-10-06T10:00:00Z'}));
          live = false; show(1);
        }, code);
        assert.equal(await v.page.locator('#jbody .chip.wait').count(), 0);
      }
      for (const code of ['print("wait, then paint")', '-- wait, then paint\nprint("hello")']) {
        await v.page.evaluate(code => {
          events = [{kind:'paint', code, out:'day 1, 09:59\nok'}, {kind:'paint', code:'wait(10080)', out:'day 8, 09:59\nok'}].map(e => ({...e, ts:'2026-10-06T10:00:00Z'}));
          show(1);
        }, code);
        assert.equal(await v.page.locator('#jbody .chip.wait').textContent(), '🕒 waits 1 week');
      }
      await v.page.evaluate(() => {
        events = [{kind:'paint', scratch:true, code:'wait(10080)', out:'day 8, 09:00\nok'}, {kind:'paint', scratch:true, new_scratch:true, code:'error("failure")', err:true, out:'failure'}, {kind:'paint', scratch:true, code:'wait(20160)', out:'day 15, 09:00\nok'}].map(e => ({...e, ts:'2026-10-06T10:00:00Z'}));
        show(2);
      });
      assert.equal(await v.page.locator('#jbody .chip.wait').last().textContent(), '🕒 waits 2 weeks on the scratch canvas');
      report.checks.push(staticMode ? 'static wait chips and real Monet clocks' : 'live wait chips, conservative fallback and duration formatting');
      await v.page.close();
    }
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
    // An edit with no complete copy before it has an unknown source: the current file is not shown as that history.
    const unknown = [ev('start', { cwd:'/painter' }), ev('edit', { path:'/painter/unknown.lua', edits:[{ oldText:'before', newText:'after' }], out:'ok' }, 1), ev('say', { text:'end' }, 2)];
    const u = await viewer(browser, unknown);
    await seek(u.page, 1);
    const unknownView = await u.page.evaluate(() => ({ code:document.querySelector('#code').textContent, label:document.querySelector('#codeformat').textContent, file:document.querySelector('#file').textContent }));
    assert.doesNotMatch(unknownView.code, /live\s*=\s*99/, 'an unknown historical source must not show the current file');
    assert.match(unknownView.label, /unknown/, 'an unknown historical source must say so in the header');
    assert.equal(unknownView.file, 'unknown.lua');
    report.checks.push('an edit with no complete copy before it is marked unknown, never shown as the current file'); await u.page.close();
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
    const queue = await viewer(browser, initial);
    await seek(queue.page, 2);
    let releaseWasm;
    const wasmReady = new Promise(resolve => { releaseWasm = resolve; });
    await queue.page.route('**/stylua_lib_bg.wasm', async route => {
      await wasmReady;
      await route.fulfill({ contentType:'application/wasm', body:fs.readFileSync(path.join(root, 'vendor/stylua/stylua_lib_bg.wasm')) });
    });
    await queue.page.evaluate(() => {
      window.queueResults = [];
      window.queueWorker = new Worker('code-format.js', { type:'module' });
      queueWorker.onmessage = ({ data }) => queueResults.push(data);
      for (let id = 1; id <= 40; id++) queueWorker.postMessage({ id, input:`local latest=${id}\n`, previous:null, changed:[] });
    });
    await queue.page.waitForTimeout(100);
    releaseWasm();
    await queue.page.waitForFunction(() => queueResults.some(r => r.id === 40));
    await queue.page.waitForTimeout(100);
    assert.deepEqual(await queue.page.evaluate(() => queueResults.map(r => r.id)), [40], 'superseded jobs are dropped before WASM formatting');
    assert.equal(await queue.page.evaluate(() => queueResults[0].text), 'local latest = 40\n');
    await queue.page.evaluate(() => {
      window.formatPosts = 0;
      const post = codeWorker.postMessage.bind(codeWorker);
      codeWorker.postMessage = data => { if (!data.cancel) formatPosts++; post(data); };
      live = true; playing = null; document.body.classList.add('nocode');
      renderCode('hidden.lua', 'local hidden=1', new Set());
      playing = true; document.body.classList.remove('nocode');
      renderCode('playing.lua', 'local playing=2', new Set());
    });
    assert.equal(await queue.page.evaluate(() => formatPosts), 0, 'hidden live code and active playback never request formatting');
    await queue.page.waitForFunction(() => document.querySelector('#file').textContent === 'playing.lua' && codeBatch === null);
    assert.deepEqual((await snapshot(queue.page)).code, ['local playing=2']);
    await queue.page.evaluate(() => { playing = null; live = false; document.body.classList.add('nocode'); renderCode('step.lua', 'local step=3', new Set(), null, true); });
    await queue.page.waitForFunction(() => document.querySelector('#file').textContent === 'step.lua' && !document.querySelector('#codeformat').textContent);
    assert.equal(await queue.page.evaluate(() => formatPosts), 1, 'manual steps can format a hidden pane');
    await queue.page.evaluate(() => { playing = true; document.querySelector('#play').click(); });
    await queue.page.waitForFunction(() => !playing && !document.querySelector('#codeformat').textContent);
    assert.equal(await queue.page.evaluate(() => formatPosts), 2, 'pause formats the current step');
    report.timings.queue = await queue.page.evaluate(() => ({ submitted:40, completed:queueResults.length, latest:queueResults.at(-1).id, superseded:40 - queueResults.length }));
    await queue.page.evaluate(() => queueWorker.terminate());
    report.checks.push('formatter queue drain, hidden-pane gating, raw playback, manual step and pause formatting');
    await queue.page.close();
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
    const previousFirst = [ev('start', { cwd:'/painter' }), ev('paint', { code:'local first=1', out:'ok' }, 1), ev('look', { text:'' }, 2), ev('image', { img:0, look:'' }, 2),
      ev('paint', { code:'local second=2', out:'ok' }, 3), ev('look', { text:'' }, 4), ev('image', { img:1, look:'' }, 4)];
    const previous = await viewer(browser, previousFirst);
    const previousFeedback = await previous.page.evaluate(() => {
      document.querySelector('#play').click();
      const replaying = document.querySelector('#badge').textContent;
      document.querySelector('#prevlook').click();
      return { replaying, rewound:document.querySelector('#badge').textContent, play:document.querySelector('#play').textContent, playing, pos };
    });
    assert.deepEqual(previousFeedback, { replaying:'REPLAY', rewound:'REWOUND', play:'▶ Replay', playing:null, pos:3 }, 'Previous at the first look refreshes the badge with the stopped timer');
    await previous.page.close();
    const fresh = () => initial.map(e => ({ ...e, ts:new Date().toISOString() }));
    const dropped = await viewer(browser, fresh());
    assert.equal(await dropped.page.locator('#badge').textContent(), 'LIVE');
    dropped.state.failing = true;
    await dropped.page.waitForFunction(() => document.getElementById('badge').className !== 'live' && document.getElementById('badge').textContent !== 'LIVE');
    assert.equal(await dropped.page.evaluate(() => events.length), initial.length, 'a failed refresh keeps what was loaded');
    dropped.state.failing = false;
    await dropped.page.waitForFunction(() => document.getElementById('badge').textContent === 'LIVE');
    await dropped.page.close();
    const unreached = await viewer(browser, [], { failing:true });
    await unreached.page.waitForFunction(() => !document.getElementById('nopic').hidden);
    assert.doesNotMatch(await unreached.page.locator('#nopic').textContent(), /hasn't started/, 'a failed first request for a listed painter is not a painter that has not started');
    const unlisted = await viewer(browser, [], { failing:true, listed:false });
    await unlisted.page.waitForFunction(() => /hasn't started/.test(document.getElementById('nopic').textContent));
    await unlisted.page.close();
    unreached.state.failing = false;
    await unreached.page.waitForFunction(() => document.getElementById('nopic').hidden);
    unreached.state.events.push(...fresh());
    await unreached.page.waitForFunction(() => document.getElementById('badge').textContent === 'LIVE');
    await unreached.page.close();
    report.checks.push('a failed refresh reads disconnected and keeps the loaded picture; a failed first load is not a painter that has not started');
    assert.deepEqual(report.pageErrors, []);
    fs.writeFileSync(path.join(root, 'replay-verification.json'), JSON.stringify(report, null, 2) + '\n');
    console.log(JSON.stringify(report));
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
