import { chromium, devices } from 'playwright';
import fs from 'fs';
const all = JSON.parse(fs.readFileSync(process.env.TMPDIR + '/piles.json', 'utf8')), piles = all.slice(-16);
const b = await chromium.launch();
const inject = async (p, phone) => p.evaluate(([piles, phone]) => {
  document.querySelector('#pal').hidden = true; document.querySelector('#view').classList.remove('pal');
  const st = document.createElement('style'); st.textContent = `
  #pband{flex:none;display:flex;align-items:flex-start;gap:${phone?6:8}px;padding:${phone?'5px 8px 6px':'7px 12px 9px'};overflow-x:auto;scrollbar-width:none;border-top:1px solid #23232b}
  #pband::-webkit-scrollbar{display:none}
  #pband .lbl{flex:none;writing-mode:vertical-rl;transform:rotate(180deg);font:600 ${phone?9:10}px -apple-system,sans-serif;letter-spacing:.12em;color:#9fd0b4;text-transform:uppercase;align-self:center}
  #pband .c{flex:none;display:flex;flex-direction:column;align-items:center;gap:3px;width:${phone?40:44}px}
  #pband .sw{width:${phone?34:40}px;height:${phone?24:30}px;border-radius:4px;overflow:hidden;display:flex;flex-direction:column;box-shadow:0 1px 3px rgba(0,0,0,.6)}
  #pband .sw div:first-child{flex:3} #pband .sw div:last-child{flex:2}
  #pband .n{font:${phone?9:10}px ui-monospace,Menlo,monospace;color:#bdbcc4;max-width:100%;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}`;
  document.head.appendChild(st);
  const band = document.createElement('div'); band.id = 'pband';
  band.innerHTML = '<span class="lbl">Palette</span>' + piles.map(p => `<div class="c" title="${p.n}"><div class="sw"><div style="background:${p.thick}"></div><div style="background:${p.thin}"></div></div><span class="n">${p.n}</span></div>`).join('');
  document.querySelector('#view').after(band);
}, [piles, phone]);
for (const [n, opt, phone] of [['desk',{viewport:{width:1400,height:900}},false],['phone',{...devices['iPhone 13']},true]]) {
  const p = await (await b.newContext(opt)).newPage();
  await p.addInitScript(() => localStorage.setItem('studio.intro', '1'));
  await p.goto('http://127.0.0.1:8766/?p=paint-studio-496bb9', {waitUntil:'networkidle'}); await p.waitForTimeout(2500);
  for (const [tag, find] of [['study', true], ['plain', false]]) {
    const k = await p.evaluate(f => { if (!f) return events.length - 1; for (let i = events.length - 1; i > 0; i--) if (events[i].kind === 'image' && /crop/.test(events[i].look || '')) return i + 1; return events.length - 1; }, find);
    await p.evaluate(a => { const s=document.querySelector('#scrub'); s.value=a; s.dispatchEvent(new Event('input')); }, k);
    await p.waitForTimeout(2000); await inject(p, phone); await p.waitForTimeout(400);
    await p.screenshot({path: `${process.env.TMPDIR}/m-${n}-${tag}.jpg`, type:'jpeg', quality:72});
    await p.evaluate(() => document.querySelector('#pband').remove());
  }
  // and today's version, for comparison
  const k = await p.evaluate(() => { for (let i = events.length - 1; i > 0; i--) if (events[i].kind === 'image' && /crop/.test(events[i].look || '')) return i + 1; return events.length - 1; });
  await p.evaluate(a => { const s=document.querySelector('#scrub'); s.value=a; s.dispatchEvent(new Event('input')); }, k - 1);
  await p.waitForTimeout(300);
  await p.evaluate(a => { const s=document.querySelector('#scrub'); s.value=a; s.dispatchEvent(new Event('input')); }, k);
  await p.waitForTimeout(2000);
  await p.screenshot({path: `${process.env.TMPDIR}/m-${n}-today.jpg`, type:'jpeg', quality:72});
}
await b.close();
