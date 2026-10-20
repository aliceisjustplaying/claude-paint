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
    console.log('studio picker keys: OK');
  } finally { await browser.close(); server.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
