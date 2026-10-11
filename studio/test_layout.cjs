const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { chromium, devices } = require('playwright');

const stamp = '2026-10-10T23:48:00Z';
const event = (kind, fields = {}) => ({ ts:stamp, kind, ...fields });
const palette = ['DRESSL', 'SKINSH', 'WALKL', 'WALKM', 'SHADE', 'SASH', 'FWALL', 'WALLT', 'SKTN', 'HAIR', 'WALKD', 'CREAM', 'ROOM', 'FWIN', 'FWALK', 'DARKB'].map((name, i) => ({ name, thick:`rgb(${96 + i * 6}, ${84 + i * 5}, ${62 + i * 4})`, thin:`rgb(${126 + i * 6}, ${114 + i * 5}, ${92 + i * 4})` }));
const events = [event('start', { cwd:'/fixture' }), event('paint', { scratch:true, new_scratch:true, code:'canvas{}', out:'ok' }),
  event('look', { text:'scratch True' }), event('image', { img:0, look:'scratch True', gallery:true }), event('paint', { code:'canvas{}', out:'day 1, 09:00' }),
  event('look', { text:'' }),
  event('image', { img:1, look:'', gallery:true, ts:'2026-10-10T23:46:00Z' }),
  event('look', { text:'palette True' }), event('image', { img:2, look:'palette True', palette }),
  event('look', { text:'crop 380,380,880,560' }),
  event('image', { img:3, look:'crop 380,380,880,560', gallery:true }),
  event('think', { text:('## Looking at the painting\n\nThe light falls across the counter. I look at the color and the edges, then try another mark on the scratch canvas.\n\n').repeat(18) }),
  event('say', { text:'The next mark is ready.' })];
const shots = process.env.LAYOUT_SCREENSHOTS;
const baseline = process.env.STUDIO_BASELINE;
const images = process.env.LAYOUT_IMAGES;
const fixture = fs.readFileSync(path.join(__dirname, 'fixtures/fable-palette-board.png'));
const server = http.createServer((req, res) => {
  const url = new URL(req.url, 'http://localhost');
  let body, type = 'application/json';
  if (url.pathname === '/') {
    body = fs.readFileSync(baseline || path.join(__dirname, 'index.html')); type = 'text/html';
  } else if (url.pathname === '/api/sessions') body = JSON.stringify([{ p:'fixture', folder:'fixture', painter:true, active:false, sittings:1, title:'The night counter', model:'Hopper' }]);
  else if (url.pathname === '/api/events') body = JSON.stringify({ epoch:0, sittings:1, events:events.slice(+(url.searchParams.get('since') || 0)), updates:[], nupdates:0, total:events.length });
  else if (url.pathname === '/paintings.json') body = '[]';
  else if (url.pathname === '/img') {
    body = images ? fs.readFileSync(path.join(images, `${url.searchParams.get('i')}.png`)) : fixture; type = 'image/png';
  } else if (url.pathname === '/api/file') { body = ''; type = 'text/plain'; }
  else if (['stream.css', 'code-display.js', 'code-format.js'].includes(url.pathname.slice(1)) || url.pathname.startsWith('/vendor/')) {
    const file = path.join(__dirname, url.pathname);
    if (fs.existsSync(file)) { body = fs.readFileSync(file); type = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : 'application/wasm'; }
  }
  if (body === undefined) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'Content-Type':type }); res.end(body);
});

const sizes = [[1280, 900], [1700, 1000], [1920, 1080], [393, 852], [402, 874], [440, 956], [375, 667], [874, 402]];
(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const browser = await chromium.launch({ channel:'chrome', headless:true });
  const errors = [];
  try {
    if (shots) fs.mkdirSync(shots, { recursive:true });
    for (const [width, height, stream] of [...sizes, [1920, 1080, true]]) {
      const mobile = width < 1000;
      const label = `${stream ? 'stream-' : ''}${width}x${height}`;
      const context = await browser.newContext({ ...(mobile ? devices['iPhone 13'] : {}), viewport:{ width, height } });
      await context.addInitScript(() => localStorage.setItem('studio.intro', '1'));
      const page = await context.newPage(); page.on('pageerror', error => errors.push(String(error)));
      await page.goto(`http://127.0.0.1:${server.address().port}/?p=fixture${stream ? '&stream=1' : ''}`);
      await page.waitForFunction(() => events.length === 13 && document.getElementById('pipimg').complete && document.getElementById('pipimg').naturalWidth > 0 && document.querySelectorAll('.palette-chip').length === 16);
      await page.waitForFunction(() => ['img', 'pipimg'].every(id => document.getElementById(id).complete && document.getElementById(id).naturalWidth > 0));
      const scroller = page.locator(mobile ? 'main' : '#jbody');
      const gap = () => scroller.evaluate(el => el.scrollHeight - el.clientHeight - el.scrollTop);
      if (!baseline) assert(await gap() <= 24, label + ': initial transcript shows the selected event');
      await page.evaluate(() => { live = false; show(events.length - 1); });
      await page.waitForFunction(() => typingThought === null);
      await page.waitForFunction(() => ['img', 'img2', 'pipimg'].every(id => document.getElementById(id).complete && document.getElementById(id).naturalWidth > 0));
      if (mobile) {
        if (baseline) await page.locator('.palette-toggle').evaluate(el => el.click());
        else await page.locator('.palette-toggle').click();
        await page.locator('main').evaluate(el => el.scrollTop = 0);
      }
      const metrics = await page.evaluate(() => {
        const rect = id => document.getElementById(id).getBoundingClientRect();
        const pip = rect('pip');
        const overlaps = r => pip.left < r.right && pip.right > r.left && pip.top < r.bottom && pip.bottom > r.top;
        return {
          overlap:['cap', 'cap2', 'cap3', 'pband'].filter(id => !document.getElementById(id).hidden && overlaps(rect(id))),
          paletteOverflow:document.getElementById('pband').scrollWidth > document.getElementById('pband').clientWidth,
          pageOverflow:document.documentElement.scrollWidth > innerWidth,
          verticalScrollers:[...document.querySelectorAll('body, main, #jbody')].filter(el => /auto|scroll/.test(getComputedStyle(el).overflowY) && el.scrollHeight > el.clientHeight + 1).map(el => el.id || el.tagName.toLowerCase()),
          journalOverflow:getComputedStyle(document.getElementById('jbody')).overflowY,
        };
      });
      console.log(label, JSON.stringify(metrics));
      if (shots) {
        await page.screenshot({ path:path.join(shots, label + '.png') });
        if (mobile) {
          await page.locator('main').evaluate(el => el.scrollTop = el.scrollHeight);
          await page.screenshot({ path:path.join(shots, label + '-journal.png') });
        }
      }
      if (!baseline) {
        assert.deepEqual(metrics.overlap, [], label + ': PiP must not cover captions or palette');
        assert.equal(metrics.paletteOverflow, false, label + ': palette fits');
        assert.equal(metrics.pageOverflow, false, label + ': page fits');
        if (mobile) {
          assert.deepEqual(metrics.verticalScrollers, ['main'], label + ': one vertical scroller');
          assert.equal(metrics.journalOverflow, 'visible', label + ': transcript is in normal flow');
        }
      }
      await page.evaluate(() => palette(-1));
      const captionOverlap = await page.evaluate(() => {
        const pip = document.getElementById('pip').getBoundingClientRect();
        return ['cap', 'cap2'].filter(id => {
          const r = document.getElementById(id).getBoundingClientRect();
          return pip.left < r.right && pip.right > r.left && pip.top < r.bottom && pip.bottom > r.top;
        });
      });
      if (!baseline) assert.deepEqual(captionOverlap, [], label + ': caption-only view stays clear');
      if (shots) {
        await page.locator('main').evaluate(el => el.scrollTop = 0);
        await page.screenshot({ path:path.join(shots, label + '-captions.png') });
        if (stream) {
          await page.evaluate(() => pip(-1, false));
          await page.screenshot({ path:path.join(shots, label + '-unchanged.png') });
        }
      }
      if (!baseline) {
        const appendThought = async () => {
          await page.evaluate(() => {
            live = true;
            events.push({ kind:'think', ts:new Date().toISOString(), text:('## A live thought\n\nI am looking at the light and the next brushstroke.\n\n').repeat(40) });
            show(events.length - 1);
          });
          await page.waitForFunction(() => typingThought !== null && document.querySelector('.th.last').textContent.length > 30);
        };
        await scroller.evaluate(el => el.scrollTop = el.scrollHeight);
        await appendThought();
        assert(await gap() <= 24, label + ': follows the first typewriter ticks');
        await page.waitForFunction(() => typingThought === null);
        assert(await gap() <= 24, label + ': follows the complete live append');
        if (shots) await page.screenshot({ path:path.join(shots, label + '-live-append.png') });
        if (mobile) {
          await scroller.evaluate(el => el.scrollTop = 0);
          await appendThought();
          await page.waitForFunction(() => typingThought === null);
          assert.equal(await scroller.evaluate(el => el.scrollTop), 0, label + ': a live append preserves the painting position');
          await scroller.evaluate(el => el.scrollTop = el.scrollHeight);
          await appendThought();
          assert(await gap() <= 24, label + ': resumes following at the bottom');
          await scroller.evaluate(el => el.scrollTop = 0);
          await page.waitForFunction(() => typingThought === null);
          assert.equal(await scroller.evaluate(el => el.scrollTop), 0, label + ': scrolling up interrupts typewriter following');
        }
        await page.evaluate(() => { live = false; show(11); });
        await page.waitForFunction(() => typingThought === null);
        assert(await gap() <= 24, label + ': backward seek shows the selected thought');
        await scroller.evaluate(el => el.scrollTop = 0);
        await page.evaluate(() => show(events.length - 1));
        assert(await gap() <= 24, label + ': forward seek shows the selected thought');
        if (shots) await page.screenshot({ path:path.join(shots, label + '-seek.png') });
      }
      await context.close();
    }
    assert.deepEqual(errors, []);
    console.log('studio layout: OK');
  } finally { await browser.close(); server.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
