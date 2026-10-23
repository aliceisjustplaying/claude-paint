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

const failureErrors = [];
async function failureFixture(browser, hosted, failure = '') {
  const page = await browser.newPage();
  page.on('pageerror', error => failureErrors.push(String(error)));
  page.setDefaultTimeout(4000);
  const state = { revision:'a', events:structuredClone(events), source:'local old=1', failure, requests:0 };
  const object = key => 'objects/' + key.repeat(64) + '.json';
  await page.addInitScript(() => localStorage.setItem('studio.intro', '1'));
  const now = new Date();
  await page.clock.install({ time:now });
  await page.clock.pauseAt(now);
  await page.route('**/*', async route => {
    const u = new URL(route.request().url()), rel = u.pathname.slice(1);
    if (rel === '') {
      const html = fs.readFileSync(path.join(root, 'index.html'), 'utf8');
      return route.fulfill({ contentType:'text/html', body:hosted ? html.replace('<script src="code-display.js">', '<script>window.STUDIO_STATIC=true;window.STUDIO_LIVE=true;</script><script src="code-display.js">') : html });
    }
    if (rel === 'manifest.json') return route.fulfill({ status:state.failure === 'manifest' ? 503 : 200, json:{ version:1, revision:state.revision.repeat(64), snapshot:object('c') } });
    if (rel === object('c')) return route.fulfill({ status:state.failure === 'snapshot' ? 503 : 200, json:{ files:{
      'data/sessions.json':object('d'), 'data/fixture/events.json':object('e'), 'data/fixture/file/paintings/lua/painting.lua':object('f'),
      'data/fixture/file/paintings/lua/scratch.lua':object('f'), 'data/other/file/paintings/lua/painting.lua':object('f'),
    } } });
    if (rel === object('d') || rel === 'api/sessions') return route.fulfill({ json:[{ p:'fixture', folder:'fixture', painter:true, active:true, mtime:Date.now() / 1000, title:'Fixture' }] });
    if (rel === object('e') || rel === 'api/events') return route.fulfill({ json:{ epoch:0, sittings:1, events:state.events.slice(hosted ? 0 : +(u.searchParams.get('since') || 0)), updates:[], nupdates:0, total:state.events.length } });
    if (rel === object('f') || rel === 'api/file') {
      state.requests++;
      state.sourceStarted?.();
      if (state.failure === 'network') return route.abort();
      const failure = state.failure, source = state.source;
      if (state.sourceWait) await state.sourceWait;
      return route.fulfill({ status:failure === 'source' ? 503 : 200, contentType:'text/plain', body:source });
    }
    return route.continue();
  });
  await page.goto(`http://127.0.0.1:${server.address().port}/?p=fixture`);
  return { page, state };
}

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
    const failures = [];
    const check = async (name, fn) => {
      try { await fn(); console.log('ok   ' + name); }
      catch (error) { failures.push(name + ': ' + error.message); console.error('FAIL ' + name + ': ' + error.message); }
    };
    for (const outage of ['manifest', 'snapshot']) await check('hosted initial ' + outage + ' outage is unavailable, not unstarted', async () => {
      const { page, state } = await failureFixture(browser, true, outage);
      try {
        await page.waitForFunction(() => disconnected);
        assert.match(await page.locator('#nopic').textContent(), /Can't reach the studio's data/);
        assert.doesNotMatch(await page.locator('#nopic').textContent(), /hasn't started/);
        state.failure = '';
        await page.clock.runFor(2001);
        await page.waitForFunction(() => events.length > 0 && !disconnected);
        assert.doesNotMatch(await page.locator('#nopic').textContent(), /Can't reach/);
      } finally { await page.close(); }
    });
    for (const failure of ['source', 'network']) await check('hosted unchanged revision retries ' + failure + ' failure', async () => {
      const { page, state } = await failureFixture(browser, true, failure);
      try {
        await page.waitForFunction(() => events.length === 2 && codeBatch === null);
        assert.equal(state.requests, 1);
        state.failure = ''; state.source = 'local recovered=3';
        await page.clock.runFor(12001);
        await page.waitForFunction(() => liveText === 'local recovered=3');
        assert(state.requests > 1);
        assert.equal(await page.locator('#codestate').textContent(), '');
        const recovered = state.requests;
        await page.clock.runFor(2001);
        assert.equal(state.requests, recovered, 'an unchanged successful source is not fetched again');
      } finally { await page.close(); }
    });
    for (const revision of ['a', 'b']) await check('hosted slow source recovers after failure at ' + (revision === 'a' ? 'the same' : 'a new') + ' revision', async () => {
      const { page, state } = await failureFixture(browser, true, 'source');
      let finish;
      try {
        await page.waitForFunction(() => liveCodeFailed && !busy && codeBatch === null);
        const failedRequests = state.requests;
        state.failure = ''; state.revision = revision; state.source = 'local recovered=3';
        state.sourceWait = new Promise(resolve => { finish = resolve; });
        await page.clock.runFor(2001);
        await page.waitForFunction(() => !busy);
        const pendingRequests = state.requests;
        assert(pendingRequests > failedRequests, 'a retry starts');
        await page.clock.runFor(3001);
        await page.waitForFunction(() => !busy);
        assert.equal(state.requests, pendingRequests, 'manifest polls leave the slow successful download alone');
        finish();
        await page.waitForFunction(() => liveText === 'local recovered=3' && codeBatch === null);
        assert.match(await page.locator('#code').textContent(), /recovered/);
        assert.equal(await page.locator('#codestate').textContent(), '');
        await page.clock.runFor(2001);
        assert.equal(state.requests, pendingRequests, 'the recovered source is not fetched again');
      } finally { finish?.(); await page.close(); }
    });
    for (const identity of ['painter', 'path', 'revision']) await check('hosted pending source is superseded by a new ' + identity, async () => {
      const { page, state } = await failureFixture(browser, true);
      let finishOlder, finishNewer;
      try {
        await page.waitForFunction(() => liveText === 'local old=1' && codeBatch === null);
        const initial = state.requests;
        state.source = 'local older=2';
        state.sourceWait = new Promise(resolve => { finishOlder = resolve; });
        const olderStarted = new Promise(resolve => { state.sourceStarted = resolve; });
        await page.evaluate(() => { window.olderSource = refreshLiveCode(); });
        await olderStarted;
        state.source = 'local newer=3';
        state.sourceWait = new Promise(resolve => { finishNewer = resolve; });
        const newerStarted = new Promise(resolve => { state.sourceStarted = resolve; });
        await page.evaluate(identity => {
          if (identity === 'painter') Q = 'p=other';
          if (identity === 'path') events.push({ kind:'paint', ts:new Date().toISOString(), code:'local scratch=1', out:'ok', scratch:true });
          if (identity === 'revision') liveRevision = 'b'.repeat(64);
          window.newerSource = refreshLiveCode();
        }, identity);
        await newerStarted;
        finishOlder();
        await page.evaluate(() => window.olderSource);
        assert.equal(await page.evaluate(() => liveText), 'local old=1', 'the superseded source is discarded');
        assert(await page.evaluate(() => { const request = liveTextRequest; refreshLiveCode(); return liveTextRequest === request; }), 'the newer request remains current');
        assert.equal(state.requests, initial + 2, 'the older completion does not clear the newer pending request');
        finishNewer();
        await page.evaluate(() => window.newerSource);
        assert.equal(await page.evaluate(() => liveText), 'local newer=3');
      } finally { finishOlder?.(); finishNewer?.(); await page.close(); }
    });
    for (const hosted of [false, true]) await check((hosted ? 'hosted' : 'local') + ' cached source stays visible and labeled after failed refresh', async () => {
      const { page, state } = await failureFixture(browser, hosted);
      try {
        await page.waitForFunction(() => liveText === 'local old=1' && codeBatch === null);
        state.events.push({ kind:'paint', ts:new Date().toISOString(), code:'local newer=2', out:'ok' });
        state.revision = 'b'; state.failure = 'source';
        await page.clock.runFor(2001);
        await page.waitForFunction(() => events.length === 3 && !busy && codeBatch === null);
        assert.match(await page.locator('#code').textContent(), /old/);
        assert.equal(await page.locator('#codestate').textContent(), 'out of date');
        await page.evaluate(() => { live = false; show(1, true); });
        assert.equal(await page.locator('#codestate').textContent(), '');
        await page.evaluate(() => goLive());
        assert.equal(await page.locator('#codestate').textContent(), 'out of date');
        state.failure = '';
        await page.clock.runFor(2001);
        await page.waitForFunction(() => !liveCodeFailed);
        assert.equal(await page.locator('#codestate').textContent(), '', 'an identical recovered file clears the warning');
        state.revision = 'a'; state.source = 'local old=1\nlocal newer=2';
        await page.clock.runFor(2001);
        await page.waitForFunction(() => liveText.includes('newer') && codeBatch === null);
        assert.match(await page.locator('#code').textContent(), /newer/);
        assert.equal(await page.locator('#codestate').textContent(), '');
      } finally { await page.close(); }
    });
    assert.deepEqual(failureErrors, []);
    assert.deepEqual(failures, []);
  } finally {
    if (browser) await browser.close();
    server.close();
    server.closeAllConnections();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
