const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { chromium } = require('playwright');

const server = http.createServer((req, res) => {
  const url = new URL(req.url, 'http://localhost');
  if (url.pathname === '/api/sessions' || url.pathname === '/paintings.json') {
    res.writeHead(200, { 'Content-Type':'application/json' });
    return res.end('[]');
  }
  const file = path.join(__dirname, url.pathname === '/' ? 'index.html' : url.pathname);
  if (!file.startsWith(__dirname + path.sep) || !fs.existsSync(file) || !fs.statSync(file).isFile()) {
    res.writeHead(404);
    return res.end();
  }
  res.writeHead(200, { 'Content-Type':file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : 'text/html' });
  res.end(fs.readFileSync(file));
});

(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const browser = await chromium.launch({ channel:'chrome', headless:true });
  try {
    for (const failure of ['http', 'network']) {
      const failed = await browser.newPage();
      await failed.route('**/api/sessions', route => failure === 'http' ? route.fulfill({ status:503, json:[] }) : route.abort());
      await failed.goto(`http://127.0.0.1:${server.address().port}/`);
      await failed.click('#who');
      await failed.waitForFunction(() => document.querySelector('#pkerror').textContent.includes("Can't load the painters"));
      assert.match(await failed.locator('#nopic').textContent(), /Can't load the painters/);
      await failed.unroute('**/api/sessions');
      await failed.evaluate(() => loadSessions());
      assert.equal(await failed.locator('#pklist button').count(), 0);
      assert.equal(await failed.locator('#pkerror').isVisible(), false);
      assert.equal(await failed.locator('#nopic').isVisible(), false);
      await failed.close();
    }
    const page = await browser.newPage();
    const errors = [];
    page.on('pageerror', error => errors.push(String(error)));
    await page.goto(`http://127.0.0.1:${server.address().port}/`);
    await page.waitForFunction(() => sessionsLoaded);
    await page.click('#who');
    await page.waitForFunction(() => !document.getElementById('picker').hidden);
    assert.equal(await page.locator('#pklist button').count(), 0, 'the picker has no cards');
    await page.focus('#pkclose');
    for (const key of ['Home', 'End', 'ArrowDown', 'ArrowUp']) await page.keyboard.press(key);
    assert.deepEqual(errors, []);
    assert.equal(await page.evaluate(() => document.activeElement.id), 'pkclose', 'the close button keeps focus');
    assert.equal(await page.locator('#picker').isVisible(), true, 'the picker stays open');

    await page.keyboard.press('Escape');
    await page.evaluate(() => {
      allSessions = ['a', 'b'].map(p => ({ p, folder:p, painter:true, mtime:0, title:p }));
    });
    await page.click('#who');
    const label = await page.locator('#allwrap').boundingBox();
    for (const checked of [true, false]) {
      await page.locator('#pklist button').first().focus();
      await page.mouse.move(label.x + label.width - 4, label.y + label.height / 2);
      await page.mouse.down();
      assert.equal(await page.locator('#picker').isVisible(), true, 'pressing the label text keeps the picker open');
      await page.mouse.up();
      assert.equal(await page.locator('#all').isChecked(), checked, 'clicking the label text toggles all sessions');
      assert.equal(await page.locator('#who').getAttribute('aria-expanded'), 'true');
    }
    await page.keyboard.press('Escape');
    for (const exit of ['Tab', 'blur']) {
      await page.click('#who');
      await page.keyboard.press('Tab');
      assert.equal(await page.locator('#pklist button').last().evaluate(b => b === document.activeElement), true);
      assert.equal(await page.locator('#picker').isVisible(), true, 'focus within the picker keeps it open');
      if (exit === 'Tab') await page.keyboard.press('Tab');
      else await page.locator('#pklist button').last().evaluate(b => b.blur());
      assert.equal(await page.locator('#picker').isVisible(), false, `${exit} out of the document closes the picker`);
      assert.equal(await page.locator('#who').getAttribute('aria-expanded'), 'false');
      assert.equal(await page.evaluate(() => document.activeElement.tagName), 'BODY', 'closing does not steal focus');
      await page.keyboard.press('Tab');
      assert.equal(await page.evaluate(() => document.activeElement.id), 'who');
      assert.equal(await page.locator('#picker').isVisible(), false, 'returning to the header keeps the picker closed');
    }
    assert.deepEqual(errors, []);
    console.log('studio picker keys: OK');
  } finally { await browser.close(); server.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
