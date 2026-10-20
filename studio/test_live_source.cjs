const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { chromium } = require('playwright');

const root = __dirname;
const events = [
  { kind:'start', ts:new Date().toISOString(), cwd:'/fixture' },
  { kind:'paint', ts:new Date().toISOString(), code:'local initial=0', out:'ok' },
];
let nextSource = null;
const server = http.createServer((req, res) => {
  const url = new URL(req.url, 'http://localhost');
  let body, type = 'application/json';
  if (url.pathname === '/') { body = fs.readFileSync(path.join(root, 'index.html')); type = 'text/html'; }
  else if (url.pathname === '/api/sessions') body = JSON.stringify([{ p:'fixture', folder:'fixture', painter:true, sittings:1, mtime:0, title:'Fixture' }]);
  else if (url.pathname === '/api/events') body = JSON.stringify({ epoch:0, sittings:1, events:events.slice(+(url.searchParams.get('since') || 0)), updates:[], nupdates:0, total:events.length });
  else if (url.pathname === '/paintings.json') body = '[]';
  else if (url.pathname === '/api/file') {
    assert.equal(url.searchParams.get('f'), 'paintings/lua/painting.lua');
    if (nextSource) { const resolve = nextSource; nextSource = null; resolve(res); return; }
    body = 'local initial=0'; type = 'text/plain';
  } else if (['/code-display.js', '/code-format.js', '/stream.css'].includes(url.pathname) || url.pathname.startsWith('/vendor/')) {
    const file = path.join(root, url.pathname);
    if (fs.existsSync(file)) { body = fs.readFileSync(file); type = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : 'application/wasm'; }
  }
  if (body === undefined) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'Content-Type':type }); res.end(body);
});

(async () => {
  let browser;
  try {
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    browser = await chromium.launch({ channel:'chrome', headless:true });
    const page = await browser.newPage();
    const errors = [];
    page.on('pageerror', error => errors.push(String(error)));
    await page.addInitScript(() => {
      localStorage.setItem('studio.intro', '1');
      const original = window.fetch.bind(window);
      window.fetch = async (...args) => {
        const response = await original(...args);
        if (String(args[0]).startsWith('/api/file')) {
          const text = response.text.bind(response);
          response.text = () => { window.sourceTextStarted = true; return text(); };
        }
        return response;
      };
    });
    const now = new Date();
    await page.clock.install({ time:now });
    await page.clock.pauseAt(now);
    await page.goto(`http://127.0.0.1:${server.address().port}/?p=fixture`);
    await page.waitForFunction(() => liveText === 'local initial=0' && codeBatch === null);
    for (const boundary of ['fetch', 'text']) {
      const older = new Promise(resolve => { nextSource = resolve; });
      await page.evaluate(() => { window.sourceTextStarted = false; window.olderSource = refreshLiveCode(); });
      const first = await older;
      first.writeHead(200, { 'Content-Type':'text/plain' });
      if (boundary === 'text') {
        first.flushHeaders();
        await page.waitForFunction(() => window.sourceTextStarted);
      }
      const newer = new Promise(resolve => { nextSource = resolve; });
      await page.evaluate(() => { window.newerSource = refreshLiveCode(); });
      const second = await newer;
      second.writeHead(200, { 'Content-Type':'text/plain' });
      second.end('local newer=2');
      await page.evaluate(() => window.newerSource);
      await page.waitForFunction(() => codeBatch === null && document.querySelector('#code').textContent.includes('newer'));
      first.end('local older=1');
      await page.evaluate(() => window.olderSource);
      await page.waitForFunction(() => codeBatch === null);
      assert.equal(await page.evaluate(() => liveText), 'local newer=2', boundary + ' keeps the latest response cached');
      const code = await page.locator('#code .src').allTextContents();
      assert.match(code.join('\n'), /local newer\s*=\s*2/, boundary + ' keeps the latest response rendered');
      assert.doesNotMatch(code.join('\n'), /older/);
      console.log('ok   superseded same-path source rejected after ' + boundary);
    }
    assert.deepEqual(errors, []);
  } finally {
    if (browser) await browser.close();
    server.close();
    server.closeAllConnections();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
