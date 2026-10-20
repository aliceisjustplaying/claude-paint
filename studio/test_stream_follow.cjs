const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { chromium } = require('playwright');

const root = __dirname;
let sessions = [], rev = 0, servingStatic = false;
const painter = (p, look) => ({ p, folder:p, painter:true, active:true, sittings:1, mtime:Date.now() / 1000, look, title:p[0].toUpperCase() + p.slice(1) });
const served = () => JSON.stringify(sessions.map(s => ({ ...s, rev })));
const server = http.createServer((req, res) => {
  const url = new URL(req.url, 'http://localhost');
  let body, type = 'application/json';
  if (url.pathname === '/') {
    body = fs.readFileSync(path.join(root, 'index.html')).toString(); type = 'text/html';
    if (servingStatic) body = body.replace('<script>', `<script>window.STUDIO_STATIC = true; window.STUDIO_INITIAL = ${JSON.stringify(sessions[0])};</script><script>`);
  }
  else if (url.pathname === '/api/sessions' || url.pathname === '/data/sessions.json') body = served();
  else if (url.pathname === '/api/events' || url.pathname.endsWith('/events.json')) body = JSON.stringify({ epoch:0, sittings:1, events:[], updates:[], nupdates:0, total:0 });
  else if (url.pathname === '/paintings.json') body = '[]';
  else if (['/code-display.js', '/code-format.js', '/stream.css'].includes(url.pathname) || url.pathname.startsWith('/vendor/')) {
    const file = path.join(root, url.pathname);
    if (fs.existsSync(file)) { body = fs.readFileSync(file); type = file.endsWith('.js') ? 'text/javascript' : file.endsWith('.css') ? 'text/css' : 'application/wasm'; }
  }
  if (body === undefined) { res.writeHead(404); return res.end(); }
  res.writeHead(200, { 'Content-Type':type }); res.end(body);
});

(async () => {
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  const browser = await chromium.launch({ channel:'chrome', headless:true });
  const errors = [];
  try {
    for (const [mode, query, follows] of [['local', '?stream=1', true], ['static', '?stream=1', true], ['local', '', false], ['static', '', false],
      ['local', '?p=first&stream=1', false], ['static', '?p=first&stream=1', false], ['local', '?s=sit&stream=1', false], ['static', '?s=/sessions/first/sit.jsonl&stream=1', false]]) {
      const name = `${mode} ${query || 'without a query'}`;
      servingStatic = mode === 'static'; rev = 0;
      sessions = [painter('first', 3), painter('second', 2), painter('third', 1)];
      const page = await browser.newPage({ viewport:{ width:1920, height:1080 } }); page.on('pageerror', error => errors.push(String(error)));
      await page.clock.install({ time:new Date() });
      await page.goto(`http://127.0.0.1:${server.address().port}/${query}`);
      await page.waitForFunction(() => sessionsLoaded);
      const opened = query.includes('s=sit&') ? 's=sit' : 'p=first';
      assert.equal(await page.evaluate(() => Q), opened, name + ' opens its painter');
      const refresh = async change => {
        change(); rev++;
        await page.clock.fastForward(30000);
        await page.waitForFunction(rev => allSessions.every(s => s.rev === rev) && !busy, rev);
        assert.equal(await page.evaluate(() => location.search), query, name + ' keeps its address');
        return page.evaluate(() => Q);
      };
      const stale = p => () => { sessions.find(s => s.p === p).mtime -= 4000; };
      if (!follows) {
        assert.equal(await refresh(stale('first')), opened, name + ' keeps its painter when it stops');
      } else {
        assert.equal(await refresh(stale('first')), 'p=second', name + ' follows to the live painter furthest along');
        assert.equal(await refresh(() => { sessions.find(s => s.p === 'third').look = 9; }), 'p=second', name + ' stays while its painter is live');
        assert.equal(await refresh(stale('second')), 'p=third', name + ' follows again');
        assert.equal(await refresh(() => { sessions = sessions.filter(s => s.p !== 'third'); sessions.push(painter('fourth', 1)); }), 'p=fourth', name + ' follows when its painter leaves the list');
      }
      console.log('ok   ' + name + (follows ? ' follows the live painter' : ' keeps its painter'));
      await page.close();
    }
    assert.deepEqual(errors, []);
  } finally { await browser.close(); server.close(); server.closeAllConnections(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
