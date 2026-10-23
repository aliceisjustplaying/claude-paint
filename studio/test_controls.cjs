const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { chromium, devices } = require('playwright');

const root = __dirname;
const picture = fs.readFileSync(path.join(root, 'fixtures/fable-palette-board.png'));
const stamp = new Date().toISOString();
const e = (kind, fields = {}) => ({ ts:stamp, kind, ...fields });
const events = [e('start', { cwd:'/fixture' }), e('paint', { code:'canvas{}', out:'day 1, 09:00' }), e('look', { text:'' }),
  e('image', { img:0, look:'', gallery:true, relief:true }), e('think', { text:'I look again at the color and the marks.' }),
  e('look', { text:'crop 390,390,810,555' }), e('image', { img:1, look:'crop 390,390,810,555', gallery:true, relief:true })];
const sessions = [{ p:'fixture-a', folder:'fixture-a', painter:true, active:false, sittings:1, mtime:Date.now() / 1000, title:'Fixture A', artist:'Claude Monet' }];
const server = http.createServer((req, res) => {
  const url = new URL(req.url, 'http://localhost');
  let body, type = 'application/json';
  if (url.pathname === '/') { body = fs.readFileSync(path.join(root, 'index.html')); type = 'text/html'; }
  else if (url.pathname === '/api/sessions') body = JSON.stringify(sessions);
  else if (url.pathname === '/api/events') body = JSON.stringify({ epoch:0, sittings:1, events:events.slice(+(url.searchParams.get('since') || 0)), updates:[], nupdates:0, total:events.length });
  else if (url.pathname === '/img') { body = picture; type = 'image/png'; }
  else if (url.pathname === '/paintings.json') body = '[]';
  else if (url.pathname === '/api/file') { body = ''; type = 'text/plain'; }
  else if (['code-display.js', 'code-format.js', 'stream.css'].includes(url.pathname.slice(1)) || url.pathname.startsWith('/vendor/')) {
    const file = path.join(root, url.pathname);
    if (fs.existsSync(file)) { body = fs.readFileSync(file); type = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : 'application/wasm'; }
  }
  if (body === undefined) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'Content-Type':type }); res.end(body);
});

const failures = [];
const check = async (name, fn) => {
  try { await fn(); console.log('ok   ' + name); }
  catch (error) { failures.push(name); console.log('FAIL ' + name + ': ' + error.message.split('\n')[0]); }
};

(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const origin = `http://127.0.0.1:${server.address().port}`;
  const browser = await chromium.launch({ channel:'chrome', headless:true });
  const errors = [];
  const open = async (width, height, device) => {
    const context = await browser.newContext({ ...(device ? devices[device] : {}), viewport:{ width, height } });
    await context.addInitScript(() => localStorage.setItem('studio.intro', '1'));
    const page = await context.newPage(); page.on('pageerror', error => errors.push(String(error)));
    await page.goto(`${origin}/?p=fixture-a`);
    await page.waitForFunction(() => events.length === 7 && document.getElementById('img').complete && document.getElementById('img').naturalWidth > 0);
    return { context, page };
  };
  const active = page => page.evaluate(() => document.activeElement.id || document.activeElement.tagName);

  await check('enlargement blocks event and code shortcuts until it closes', async () => {
    const { context, page } = await open(1440, 1000);
    await page.locator('#strip > *').first().click();
    await page.locator('#img').click();
    const before = await page.evaluate(() => [pos, live, codeShown(), $('zimg').src]);
    for (const key of ['ArrowRight', 'ArrowLeft', '[', ']', 'l', 'c']) {
      await page.keyboard.press(key);
      assert.deepEqual(await page.evaluate(() => [pos, live, codeShown(), $('zimg').src]), before, key);
      assert.equal(await page.locator('#zoom').isVisible(), true);
    }
    await page.keyboard.press('Shift+Tab');
    assert.equal(await active(page), 'zclose');
    await page.keyboard.press('Space');
    assert.equal(await page.locator('#zoom').isVisible(), false);
    assert.equal(await active(page), 'img');
    await context.close();
  });

  await check('reduced motion shows a new live thought immediately', async () => {
    const context = await browser.newContext({ viewport:{ width:1440, height:1000 }, reducedMotion:'reduce' });
    const page = await context.newPage();
    const text = 'A recorded thought about the next marks. '.repeat(100);
    const history = [...events, e('think', { text })];
    await page.route('**/api/events?*', route => route.fulfill({ json:{ epoch:0, sittings:1, events:history, updates:[], nupdates:0, total:history.length } }));
    await page.goto(`${origin}/?p=fixture-a`);
    await page.waitForFunction(() => events.length === 8);
    assert.equal((await page.locator('#jbody .th').last().textContent()).trim(), text.trim());
    assert.equal(await page.locator('#jbody .cursor').count(), 0);
    await context.close();
  });

  await check('the painting opens from the keyboard, and its enlargement keeps focus until it closes', async () => {
    const { context, page } = await open(1440, 1000);
    for (let n = 0; n < 40 && await active(page) !== 'img'; n++) await page.keyboard.press('Tab');
    assert.equal(await active(page), 'img');
    assert.equal(await page.locator('#img').getAttribute('aria-label'), 'Open the painting at full size');
    await page.keyboard.press('Enter');
    await page.waitForFunction(() => document.getElementById('zoom').style.display === 'flex');
    assert.equal(await active(page), 'zclose');
    assert.equal(await page.locator('#zclose').getAttribute('aria-label'), 'Close the enlarged picture');
    await page.keyboard.press('Tab');
    assert.equal(await active(page), 'zclose');
    await page.keyboard.press('Escape');
    assert.equal(await page.evaluate(() => document.getElementById('zoom').style.display), 'none');
    assert.equal(await active(page), 'img');
    await context.close();
  });

  await check('a look thumbnail is a named, focusable button that Space selects', async () => {
    const { context, page } = await open(1440, 1000);
    const thumbs = page.locator('#strip > *');
    assert.equal(await thumbs.count(), 2);
    for (let i = 0; i < 2; i++) {
      assert.equal(await thumbs.nth(i).getAttribute('role'), 'button');
      assert.equal(await thumbs.nth(i).evaluate(el => el.tabIndex), 0);
      assert.match(await thumbs.nth(i).getAttribute('aria-label'), /^Look .+, .*: show in the picture$/);
    }
    const first = await thumbs.nth(0).getAttribute('data-i');
    await thumbs.nth(0).focus();
    await page.keyboard.press(' ');
    await page.waitForFunction(first => document.querySelector('#strip > .on')?.dataset.i === first, first);
    await context.close();
  });

  await check('the gallery exit link and each lighting control are separately reachable at 390 by 844 and 375 by 667', async () => {
    for (const [width, height] of [[390, 844], [375, 667]]) {
      const { context, page } = await open(width, height, 'iPhone 13');
      await page.evaluate(() => { GALLERY = { 'fixture-a':{ url:'/paintings/fixture-a/', title:'Fixture A' } }; renderHeader(); });
      await page.waitForFunction(() => !document.getElementById('finished').hidden);
      const controls = await page.evaluate(() => [document.getElementById('finished'), ...document.querySelectorAll('#lighting button')].map(el => {
        const r = el.getBoundingClientRect(), hit = document.elementFromPoint(r.left + r.width / 2, r.top + r.height / 2);
        return { id:el.id || 'finished', rect:[r.left, r.top, r.right, r.bottom], reachable:el.contains(hit) };
      }));
      const overlap = (a, b) => a[0] < b[2] && b[0] < a[2] && a[1] < b[3] && b[1] < a[3];
      const exit = controls[0];
      for (const control of controls) assert.equal(control.reachable, true, `${width}x${height}: ${control.id} is covered`);
      for (const control of controls.slice(1)) assert.equal(overlap(exit.rect, control.rect), false, `${width}x${height}: exit link overlaps ${control.id}`);
      await context.close();
    }
  });

  await check('the first-visit explanation keeps focus inside and closes on Escape', async () => {
    const context = await browser.newContext({ viewport:{ width:1440, height:1000 } });
    const page = await context.newPage();
    await page.route(origin + '/?*', async route => route.fulfill({ contentType:'text/html',
      body:fs.readFileSync(path.join(root, 'index.html'), 'utf8').replace('<script src="code-display.js">', '<script>window.STUDIO_STATIC=true;</script><script src="code-display.js">') }));
    await page.goto(`${origin}/?p=fixture-a`);
    assert.equal(await page.locator('#intro').isVisible(), true);
    for (let i = 0; i < 4; i++) {
      await page.keyboard.press('Tab');
      assert.equal(await page.evaluate(() => !!document.activeElement.closest('#intro')), true, `Tab ${i + 1} left the explanation`);
    }
    await page.keyboard.press('Escape');
    assert.equal(await page.locator('#intro').isVisible(), false);
    assert.equal(await page.evaluate(() => localStorage.getItem('studio.intro')), '1');
    await context.close();
  });

  await check('a message over the picture keeps its own background', async () => {
    const { context, page } = await open(1440, 1000);
    const colors = await page.evaluate(() => {
      const note = document.getElementById('nopic');
      note.textContent = "Can't reach this painter's studio right now."; note.hidden = false;
      const style = getComputedStyle(note);
      return [style.color, style.backgroundColor].map(c => c.match(/[\d.]+/g).map(Number));
    });
    const [text, back] = colors;
    assert.equal(back.length === 3 || back[3] === 1, true, 'background is not opaque');
    const lum = c => { const [r, g, b] = c.slice(0, 3).map(v => { v /= 255; return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4; }); return 0.2126 * r + 0.7152 * g + 0.0722 * b; };
    const [hi, lo] = [lum(text), lum(back)].sort((a, b) => b - a);
    assert.equal((hi + 0.05) / (lo + 0.05) >= 4.5, true, 'text contrast below 4.5');
    await context.close();
  });

  server.close();
  await browser.close();
  assert.deepEqual(errors, [], 'page errors');
  if (failures.length) { console.log(`${failures.length} failing: ${failures.join('; ')}`); process.exit(1); }
  console.log('controls: all checks passed');
})().catch(error => { console.error(error); process.exit(1); });
