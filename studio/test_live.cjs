const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const path = require('node:path');
const { spawn } = require('node:child_process');
const { chromium } = require('playwright');
const studio = process.env.LIVE_STUDIO;
const out = process.env.LIVE_PROOF_OUT;
if (!studio || !out) throw Error('Set LIVE_STUDIO to an empty exported easel and LIVE_PROOF_OUT to new scratch output');
fs.mkdirSync(out, { recursive:true });
const root = path.join(out, 'site');
fs.mkdirSync(root, { recursive:true });
fs.writeFileSync(path.join(root, 'gallery-sentinel'), 'gallery unchanged');
const requests = [], errors = [];
let unavailable = false, blockedObjects = 0;
const server = http.createServer((req, res) => {
  const u = new URL(req.url, 'http://localhost'); requests.push(u.pathname);
  if (unavailable && /\/objects\/[a-f0-9]+\.json$/.test(u.pathname)) { blockedObjects++; res.writeHead(503); return res.end(); }
  let file = path.resolve(root, '.' + decodeURIComponent(u.pathname));
  if (!file.startsWith(root + path.sep)) { res.writeHead(403); return res.end(); }
  if (fs.existsSync(file) && fs.statSync(file).isDirectory()) file = path.join(file, 'index.html');
  if (!fs.existsSync(file)) { res.writeHead(404); return res.end(); }
  const type = { '.html':'text/html', '.json':'application/json', '.js':'text/javascript', '.css':'text/css', '.png':'image/png', '.jpg':'image/jpeg', '.wasm':'application/wasm' }[path.extname(file)] || 'text/plain';
  res.writeHead(200, { 'Content-Type':type, 'Cache-Control':'no-store', 'X-Content-Type-Options':'nosniff' });
  fs.createReadStream(file).pipe(res);
});
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
async function fileReady(file) {
  for (let n = 0; n < 600; n++) { if (fs.existsSync(file)) return; await delay(100); }
  throw Error('Timed out waiting for ' + file);
}
(async () => {
  const painter = spawn('uv', ['run', '--no-project', '--script', path.join(__dirname, '../scripts/prove_live_studio'), '--studio', studio, '--out', out], { stdio:['ignore', 'pipe', 'pipe'] });
  let log = ''; painter.stdout.on('data', data => log += data); painter.stderr.on('data', data => log += data);
  const exited = new Promise(resolve => painter.on('exit', code => resolve(code)));
  let browser;
  try {
    await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
    await fileReady(path.join(out, 'phase1'));
    await fileReady(path.join(root, 'live/fixture/manifest.json'));
    browser = await chromium.launch({ channel:'chrome', headless:true });
    const context = await browser.newContext({ viewport:{ width:1440, height:1000 } });
    await context.addInitScript(() => localStorage.setItem('studio.intro', '1'));
    const page = await context.newPage(); page.on('pageerror', e => errors.push(String(e)));
    const url = `http://127.0.0.1:${server.address().port}/live/fixture/`;
    await page.goto(url);
    const ready = count => page.waitForFunction(count => events.filter(e => e.kind === 'image').length === count && document.getElementById('img').complete && document.getElementById('img').naturalWidth > 0, count);
    await ready(1);
    assert.equal(await page.locator('#gallery').getAttribute('aria-pressed'), 'true');
    assert.match(await page.locator('#cap').textContent(), /Gallery light/);
    const first = await page.locator('#img').getAttribute('src');
    assert.match(first, /^objects\/[a-f0-9]{64}\.png$/);
    await page.screenshot({ path:path.join(out, 'phase1-gallery.png'), fullPage:true });
    let navigations = 0; page.on('framenavigated', () => navigations++);
    unavailable = true;
    fs.writeFileSync(path.join(out, 'continue'), 'continue');
    await fileReady(path.join(out, 'painting-receipt.json'));
    for (let n = 0; n < 100 && !blockedObjects; n++) await delay(100);
    assert(blockedObjects > 0);
    assert.equal(await page.locator('#img').getAttribute('src'), first);
    unavailable = false;
    await ready(2);
    await page.waitForFunction(() => events.some(e => e.kind === 'paint' && e.err) && events.some(e => e.kind === 'jnote' && e.text === 'Second hosted fixture snapshot.'));
    const second = await page.locator('#img').getAttribute('src');
    assert.notEqual(second, first);
    assert.equal(navigations, 0);
    await page.screenshot({ path:path.join(out, 'phase2-gallery.png'), fullPage:true });
    const previous = await page.evaluate(() => events.findIndex(e => e.kind === 'image'));
    await page.evaluate(i => { live = false; show(i); }, previous);
    assert.equal(await page.locator('#img').getAttribute('src'), first);
    await page.evaluate(() => goLive());
    await ready(2);
    await page.goto(url + '?stream=1');
    await ready(2);
    assert.equal(await page.locator('#gallery').getAttribute('aria-pressed'), 'true');
    await page.screenshot({ path:path.join(out, 'stream-gallery.png'), fullPage:true });
    assert(!requests.some(u => u.startsWith('/api/') || u.startsWith('/img?')));
    assert.equal(fs.readFileSync(path.join(root, 'gallery-sentinel'), 'utf8'), 'gallery unchanged');
    assert.deepEqual(errors, []);
    fs.writeFileSync(path.join(out, 'done'), 'done');
    assert.equal(await exited, 0, log);
    fs.writeFileSync(path.join(out, 'browser-receipt.json'), JSON.stringify({ followed_without_reload:true, gallery_default:true, immutable_image_urls:true, interrupted_fetch_retains_previous_view:true, retry_recovers:true, replay_works:true, stream_works:true, api_requests:0, browser_errors:errors, manifest_requests:requests.filter(u => u.endsWith('/manifest.json')).length, screenshots:['phase1-gallery.png','phase2-gallery.png','stream-gallery.png'] }, null, 2));
    console.log('live studio: real cloud easel followed two hosted Gallery light snapshots without reload');
  } finally {
    if (browser) await browser.close();
    server.close();
    if (painter.exitCode === null) { fs.writeFileSync(path.join(out, 'done'), 'done'); painter.kill('SIGTERM'); }
    fs.writeFileSync(path.join(out, 'fixture.log'), log);
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
