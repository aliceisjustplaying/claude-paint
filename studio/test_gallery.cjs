const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { chromium } = require('playwright');
const root = __dirname;
const evidence = process.env.GALLERY_SCREENSHOTS;
const renderRoot = process.env.GALLERY_RENDERS;
const scratchRoot = process.env.GALLERY_SCRATCH;
const tiny = Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAusB9Wl6XioAAAAASUVORK5CYII=', 'base64');
const image = (lit, i, p) => scratchRoot && (i === 2 || p === 'agedScratch' && [1, 5].includes(i)) ? fs.readFileSync(path.join(scratchRoot, lit ? '.gallery' : '', i === 2 ? 'look-0001.png' : 'look-0002.png')) : renderRoot ? fs.readFileSync(path.join(renderRoot, `${lit ? 'B' : 'A'}-${i === 1 ? 'houses' : 'full'}.png`)) : tiny;
const stamp = new Date().toISOString();
const e = (kind, fields = {}) => ({ ts:stamp, kind, ...fields });
const whole = [e('start', { cwd:'/fixture' }), e('paint', { code:'canvas{}', out:'day 1, 09:00' }), e('look', { text:'' }),
  e('image', { img:0, look:'', gallery:true }), e('think', { text:'I look again at the colour and the marks.' }),
  e('look', { text:'crop 390,390,810,555' }), e('image', { img:1, look:'crop 390,390,810,555', gallery:true })];
const fixtures = {
  normal:whole,
  scratch:[...whole.slice(0, 4), e('paint', { scratch:true, new_scratch:true, code:'canvas{}', out:'ok' }),
    e('look', { text:'scratch True' }), e('image', { img:2, look:'scratch True', gallery:true }), e('think', { text:'Trying a mark on the scratch canvas.' })],
  fallback:[...whole.slice(0, 3), e('image', { img:0, look:'' })],
  broken:[...whole.slice(0, 3), e('image', { img:3, look:'', gallery:true })],
  analytical:[...whole.slice(0, 4), e('look', { text:'mode value' }), e('image', { img:1, look:'mode value' })],
  agedScratch:[e('start', { cwd:'/fixture' }), e('look', { text:'' }),
    e('image', { img:0, look:'', gallery:true, ts:new Date(Date.parse(stamp) - 120000).toISOString() }),
    e('paint', { code:'work{}', out:'ok' }), e('paint', { scratch:true, new_scratch:true, code:'canvas{}', out:'ok' }),
    e('look', { text:'scratch True' }),
    e('image', { img:2, look:'scratch True', gallery:true, ts:new Date(Date.parse(stamp) - 60000).toISOString() }),
    e('paint', { scratch:true, code:'work{}', out:'ok' }), e('look', { text:'scratch True, crop 100,100,300,300' }),
    e('image', { img:1, look:'scratch True, crop 100,100,300,300', gallery:true }), e('think', { text:'A fresh mark after the whole-canvas look.' })],
};
const sessions = Object.keys(fixtures).map(p => ({ p, folder:p, painter:true, active:true, sittings:1, mtime:Date.now() / 1000, title:'Gallery lighting fixture' }));
let servingStatic = false;
const server = http.createServer((req, res) => {
  const u = new URL(req.url, 'http://localhost');
  const p = u.searchParams.get('p') || u.pathname.split('/')[2] || 'normal';
  let body, type = 'application/json';
  if (u.pathname === '/') {
    body = u.searchParams.has('before') && process.env.STUDIO_BASELINE ? fs.readFileSync(process.env.STUDIO_BASELINE) : fs.readFileSync(path.join(root, 'index.html'));
    if (servingStatic) body = Buffer.from(body.toString().replace('const STATIC = !!window.STUDIO_STATIC', 'const STATIC = true'));
    type = 'text/html';
  } else if (u.pathname === '/api/sessions' || u.pathname === '/data/sessions.json') body = JSON.stringify(sessions);
  else if (u.pathname === '/paintings.json') body = '[]';
  else if (u.pathname === '/api/events' || u.pathname.endsWith('/events.json')) {
    const all = fixtures[p] || whole;
    body = JSON.stringify({ epoch:0, sittings:1, events:all.slice(+(u.searchParams.get('since') || 0)), updates:[], nupdates:0, total:all.length });
  } else if (u.pathname === '/img' || /\/(?:img|gallery)\/\d+\.png$/.test(u.pathname)) {
    const i = +(u.searchParams.get('i') || path.basename(u.pathname).split('.')[0]);
    const lit = u.searchParams.has('gallery') || u.pathname.includes('/gallery/');
    if (lit && i === 3) { res.writeHead(404); return res.end(); }
    body = image(lit, i, p); type = 'image/png';
  } else if (u.pathname === '/api/file' || u.pathname.includes('/file/')) { body = ''; type = 'text/plain'; }
  else if (Object.hasOwn({ 'code-display.js':1, 'code-format.js':1, 'stream.css':1 }, u.pathname.slice(1)) || u.pathname.startsWith('/vendor/')) {
    const file = path.join(root, u.pathname);
    if (fs.existsSync(file)) { body = fs.readFileSync(file); type = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : 'application/wasm'; }
  }
  if (body === undefined) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'Content-Type':type }); res.end(body);
});
async function ready(page, which, lit) {
  await page.waitForFunction(({ which, lit }) => {
    const im = document.getElementById(which);
    return im.complete && im.naturalWidth > 0 && (lit ? /gallery/.test(im.src) : !/gallery/.test(im.src));
  }, { which, lit }).catch(async error => {
    console.error(await page.evaluate(() => ({ src:document.getElementById('img').src, cap:document.getElementById('cap').textContent, events:typeof events === 'undefined' ? null : events, query:location.search })));
    throw error;
  });
}
(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  const browser = await chromium.launch({ channel:'chrome', headless:true });
  const errors = [];
  try {
    if (evidence) fs.mkdirSync(evidence, { recursive:true });
    for (const staticMode of [false, true]) for (const stream of [false, true]) {
      servingStatic = staticMode;
      const context = await browser.newContext({ viewport:{ width:stream ? 1920 : 1440, height:stream ? 1080 : 1000 } });
      await context.addInitScript(() => localStorage.setItem('studio.intro', '1'));
      const page = await context.newPage(); page.on('pageerror', error => errors.push(String(error)));
      const url = p => `${origin}/?p=${p}${stream ? '&stream=1' : ''}`;
      const shot = async name => { if (evidence && !staticMode) { await page.waitForFunction(() => typingThought === null); await page.screenshot({ path:path.join(evidence, name + '.png'), fullPage:true }); } };
      if (evidence && process.env.STUDIO_BASELINE && !staticMode) {
        for (const p of ['normal', 'scratch']) { await page.goto(url(p) + '&before=1'); await ready(page, 'img', false); await shot(`before-${stream ? 'stream' : 'normal'}-${p}`); }
      }
      await page.goto(url('normal')); await ready(page, 'img', true); await ready(page, 'img2', true);
      assert.equal(await page.locator('#gallery').getAttribute('aria-pressed'), 'true');
      assert.match(await page.locator('#cap').textContent(), /Gallery light/);
      assert.match(await page.locator('#cap2').textContent(), /Gallery light/);
      await shot(`after-${stream ? 'stream' : 'normal'}-gallery`);
      await page.locator('#img2').click(); await ready(page, 'zimg', true); await page.locator('#zoom').click();
      await page.locator('#colour').click(); await ready(page, 'img', false); await ready(page, 'img2', false);
      assert.match(await page.locator('#cap').textContent(), /Colour/);
      await shot(`after-${stream ? 'stream' : 'normal'}-colour`);
      await page.reload(); await ready(page, 'img', false);
      assert.equal(await page.evaluate(() => STATIC), staticMode);
      assert.equal(await page.locator('#colour').getAttribute('aria-pressed'), 'true');
      await page.goto(url('scratch')); await ready(page, 'img', false);
      await page.locator('#gallery').click(); await ready(page, 'img', true); await ready(page, 'pipimg', true);
      assert.match(await page.locator('#cap').textContent(), /Scratch canvas.*Gallery light/);
      await shot(`after-${stream ? 'stream' : 'normal'}-scratch-gallery`);
      await page.locator('#colour').click(); await ready(page, 'img', false); await ready(page, 'pipimg', false);
      await shot(`after-${stream ? 'stream' : 'normal'}-scratch-colour`);
      await page.locator('#gallery').click();
      for (const p of ['fallback', 'broken']) {
        await page.goto(url(p)); await ready(page, 'img', false);
        await page.waitForFunction(() => document.getElementById('cap').textContent.includes('gallery light unavailable'));
        assert.equal(await page.locator('#gallery').getAttribute('aria-pressed'), 'true');
      }
      await page.goto(url('analytical')); await ready(page, 'img', true); await ready(page, 'img2', false);
      assert.match(await page.locator('#cap2').textContent(), /^Value check · /);
      assert.doesNotMatch(await page.locator('#cap2').textContent(), /unavailable/);
      for (const mode of ['normal,value', 'normal,mirror', 'normal,squint', 'normal, value, size 1000']) {
        fixtures.analytical = [...whole.slice(0, 4), e('look', { text:'mode ' + mode }), e('image', { img:1, look:'mode ' + mode })];
        await page.goto(url('analytical')); await ready(page, 'img', true); await ready(page, 'img2', false);
        assert.doesNotMatch(await page.locator('#cap2').textContent(), /unavailable/);
      }
      fixtures.analytical = [...whole.slice(0, 4), e('look', { text:'mode value' }), e('image', { img:1, look:'mode value' })];
      let holdLit = false, holdColour = false;
      const held = [];
      await page.route(u => u.pathname === '/img' || /\/(?:img|gallery)\/\d+\.png$/.test(u.pathname), route => {
        const u = new URL(route.request().url()), lit = u.searchParams.has('gallery') || u.pathname.includes('/gallery/');
        if (lit ? holdLit : holdColour) held.push(route); else return route.continue();
      });
      const release = async () => { for (const route of held.splice(0)) await route.continue(); };
      await page.locator('#colour').click(); await page.goto(url('normal')); await ready(page, 'img', false); await ready(page, 'img2', false);
      holdLit = true; await page.locator('#gallery').click();
      await page.waitForFunction(() => document.getElementById('img2').dataset.want.includes('gallery'));
      assert.equal(await page.locator('#img2').getAttribute('data-light'), 'colour');
      assert.doesNotMatch(await page.locator('#cap2').textContent(), /Gallery light/);
      await page.locator('#img2').click(); await ready(page, 'zimg', false); await page.locator('#zoom').click();
      await page.locator('#colour').click(); holdLit = false; await release(); await ready(page, 'img', false);
      assert.doesNotMatch(await page.locator('#cap').textContent(), /unavailable/);
      holdLit = true; await page.locator('#gallery').click();
      await page.evaluate(() => { events.push({ ts:new Date().toISOString(), kind:'image', img:4, look:'', gallery:true }); show(events.length - 1); });
      await page.locator('#img').click(); await ready(page, 'zimg', false);
      assert.match(await page.locator('#zimg').getAttribute('src'), staticMode ? /\/img\/0\.png$/ : /i=0$/);
      await page.locator('#zoom').click(); holdLit = false; await release(); await ready(page, 'img', true);
      await page.waitForFunction(() => /(?:i=4|\/4\.png)/.test(document.getElementById('img').src));
      holdColour = true; await page.locator('#colour').click();
      assert.equal(await page.locator('#img').getAttribute('data-light'), 'gallery');
      assert.match(await page.locator('#cap').textContent(), /Gallery light/);
      await page.locator('#img').click(); await ready(page, 'zimg', true); await page.locator('#zoom').click();
      holdColour = false; await release(); await ready(page, 'img', false);
      await page.locator('#gallery').click(); await ready(page, 'img', true);
      holdColour = true; await page.goto(url('fallback'));
      await page.waitForFunction(() => events.length > 0 && document.getElementById('img').dataset.want);
      await page.locator('#colour').click(); holdColour = false; await release(); await ready(page, 'img', false);
      assert.match(await page.locator('#cap').textContent(), / · Colour$/);
      assert.doesNotMatch(await page.locator('#cap').textContent(), /unavailable/);
      await page.locator('#gallery').click();
      await page.evaluate(() => { events.push({ ts:new Date().toISOString(), kind:'image', img:3, look:'', gallery:true }); show(events.length - 1); });
      await page.locator('#img').click(); await ready(page, 'zimg', false);
      assert.match(await page.locator('#zimg').getAttribute('src'), staticMode ? /\/img\/0\.png$/ : /i=0$/);
      await page.locator('#zoom').click();
      await page.waitForFunction(() => /(?:i=3|\/3\.png)/.test(document.getElementById('img').dataset.zoom || ''));
      assert.match(await page.locator('#cap').textContent(), /Colour · gallery light unavailable/);
      await page.locator('#img').click(); await ready(page, 'zimg', false);
      assert.match(await page.locator('#zimg').getAttribute('src'), staticMode ? /\/img\/3\.png$/ : /i=3$/);
      await page.locator('#zoom').click();
      await page.goto(url('agedScratch')); await ready(page, 'img', true); await ready(page, 'img2', true); await ready(page, 'pipimg', true);
      assert.match(await page.locator('#cap').textContent(), /^Scratch canvas · .* · 1 min earlier · Gallery light$/);
      assert.match(await page.locator('#pipcap').textContent(), /^The painting · .* · 2 min earlier · Gallery light$/);
      assert.match(await page.locator('#cap2').textContent(), /^Close-up · .* · Gallery light$/);
      assert.equal(await page.locator('#cropbox').isVisible(), false);
      await shot(`after-${stream ? 'stream' : 'normal'}-scratch-age`);
      await page.evaluate(() => { events.push({ ts:new Date().toISOString(), kind:'paint', scratch:true, new_scratch:true, err:true, code:'bad(', out:'error' }); show(events.length - 1); });
      assert.equal(await page.locator('#img').getAttribute('src'), null);
      assert.equal(await page.locator('#nopic').textContent(), 'No current scratch look');
      assert.equal(await page.locator('#cropbox').isVisible(), false);
      await shot(`after-${stream ? 'stream' : 'normal'}-scratch-reset`);
      await page.evaluate(() => { events.push({ ts:new Date().toISOString(), kind:'image', img:5, look:'scratch True, crop 100,100,300,300', gallery:true }); show(events.length - 1); });
      await ready(page, 'img', true);
      assert.match(await page.locator('#img').getAttribute('src'), staticMode ? /\/gallery\/5\.png$/ : /i=5&gallery=1$/);
      assert.match(await page.locator('#cap').textContent(), /^Scratch canvas · close-up · .* · Gallery light$/);
      assert.equal(await page.locator('#cropbox').isVisible(), false);
      await context.close();
    }
    assert.deepEqual(errors, []);
    console.log('PASS gallery default, colour toggle, persistence, normal/stream/static, scratch, close-up/zoom, missing/broken fallback, combined analytical modes, deferred loads and toggles');
  } finally { await browser.close(); await new Promise(resolve => server.close(resolve)); }
})().catch(error => { console.error(error); process.exitCode = 1; server.close(); });
