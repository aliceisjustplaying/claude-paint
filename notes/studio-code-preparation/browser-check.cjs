const {chromium}=require('playwright');const fs=require('fs');const assert=require('node:assert/strict');
const dir=__dirname;fs.mkdirSync(dir,{recursive:true});
(async()=>{const browser=await chromium.launch({channel:'chrome',headless:true});const checks=[],errors=[];
try {
for(const [mode,width,height,stream] of [['regular',1440,900,false],['stream',1920,1080,true],['narrow',390,844,false],['narrow-stream',390,844,true]]) {
 const page=await browser.newPage({viewport:{width,height}});page.on('pageerror',e=>errors.push(String(e)));let fileRequests=0;page.on('request',r=>{if(r.url().includes('/api/file'))fileRequests++});
 await page.goto(`http://100.82.115.34:8769/?p=paint-studio-12e572${stream?'&stream=1':''}`);await page.waitForFunction(()=>document.querySelector('#codeformat').textContent.includes('formatted'));
 // The regression: formatted current code exists before the first opening, including when replaying closed.
 assert.equal(await page.locator('#togglecode').getAttribute('aria-expanded'),'false');
 const before=fileRequests;
 const open=await page.evaluate(async()=>{const code=document.querySelector('#code'),first=code.firstChild;const t=performance.now();document.querySelector('#togglecode').click();const handlerMs=performance.now()-t;await new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)));return {handlerMs,paintMs:performance.now()-t,sameDOM:code.firstChild===first};});
 assert(open.sameDOM);assert.equal(fileRequests,before);
 const geometry=await page.evaluate(()=>{const pane=document.querySelector('#codepane'),code=document.querySelector('#code'),r=pane.getBoundingClientRect();return {paneWidth:r.width,left:r.left,right:r.right,scrollWidth:code.scrollWidth,clientWidth:code.clientWidth,lines:code.children.length,formatted:document.querySelector('#codeformat').textContent,gutterMap:[...code.children].every((row,i)=>row.querySelector('.ln').textContent===String(i+1)),wrappedRows:[...code.querySelectorAll('.src')].slice(0,80).filter(el=>el.getBoundingClientRect().height>25).length,inert:pane.inert};});
 assert.equal(geometry.lines,3164);assert(geometry.gutterMap);assert(geometry.wrappedRows>0);assert(geometry.scrollWidth<=geometry.clientWidth);assert(geometry.right<=width);assert(!geometry.inert);assert.equal(geometry.paneWidth,width<600?width-12:720);
 await page.screenshot({path:`${dir}/${mode}.png`});
 await page.getByRole('button',{name:'Close code',exact:true}).click();assert.equal(await page.locator('#togglecode').getAttribute('aria-expanded'),'false');assert(await page.locator('#codepane').evaluate(el=>el.inert));
 await page.keyboard.press('c');await page.keyboard.press('Escape');assert.equal(await page.locator('#togglecode').getAttribute('aria-expanded'),'false');
 let replay=null;
 if(mode==='regular') {
  const old=await page.locator('#code').textContent();
  await page.locator('#scrub').evaluate(el=>{el.value=String(+el.max-4);el.dispatchEvent(new Event('input',{bubbles:true}));});
  await page.waitForFunction(old=>document.querySelector('#codeformat').textContent.includes('formatted')&&document.querySelector('#code').textContent!==old,old);
  const reopen=await page.evaluate(()=>{const first=document.querySelector('#code').firstChild;document.querySelector('#togglecode').click();return document.querySelector('#code').firstChild===first;});assert(reopen);
  replay=await page.evaluate(()=>({pos,total:events.length,highlighted:document.querySelectorAll('#code .new').length,formatted:document.querySelector('#codeformat').textContent}));assert(replay.highlighted>0);await page.screenshot({path:`${dir}/replay.png`});
 }
 checks.push({mode,width,height,open,geometry,replay});await page.close();
}
// Raw fallback through the real source transport, without writing a painter file.
for(const [name,body,block] of [['invalid-lua','local x = "<script>"\nfunction broken(\n',false],['worker-unavailable','local x = "<script>"\nreturn x\n',true]]) {
 const page=await browser.newPage();await page.route('**/api/file?**',r=>r.fulfill({contentType:'text/plain',body}));
 if(block)await page.route('**/code-format.js',r=>r.abort());
 await page.goto('http://100.82.115.34:8769/?p=paint-studio-12e572');await page.waitForFunction(()=>document.querySelector('#codeformat').textContent==='raw source'&&document.querySelector('#code .src')?.textContent.includes('local x'));
 const rows=await page.locator('#code .src').allTextContents();assert.deepEqual(rows,body.split('\n').map(l=>l||' '));assert.equal(await page.locator('#code script').count(),0);checks.push({mode:name,rawRetained:true,htmlEscaped:true});await page.close();
}
// Rust and stale formatting use the same display boundary; neither source is written.
const page=await browser.newPage();await page.goto('http://100.82.115.34:8769/?p=paint-studio-12e572');await page.waitForFunction(()=>document.querySelector('#codeformat').textContent.includes('formatted'));
await page.evaluate(()=>{live=false;renderCode('test.lua','local stale = 1\n'.repeat(4000),new Set());renderCode('test.rs','fn main() { println!("<tag>"); }\n',new Set());});await page.waitForFunction(()=>document.querySelector('#file').textContent==='test.rs');await page.waitForTimeout(800);assert.deepEqual(await page.locator('#code .src').allTextContents(),['fn main() { println!("<tag>"); }',' ']);checks.push({mode:'rust-and-stale-reply',rawRetained:true,staleReplyIgnored:true});
await page.evaluate(()=>{const a='local x=1\nlocal s="a   b ; <tag>" -- words   stay\n';const b='local x=2\nlocal s="a   b ; <tag>" -- words   stay\n';renderCode('test.lua',b,new Set([0]),a);});await page.waitForFunction(()=>document.querySelector('#file').textContent==='test.lua');const text=await page.locator('#code .src').allTextContents();assert(text.join('\n').includes('a   b ; <tag>'));assert(text.join('\n').includes('-- words   stay'));assert.equal(await page.locator('#code .new').count(),1);checks.push({mode:'literals-and-highlights',literalContentsPreserved:true,changedDisplayLines:1});await page.close();
assert.deepEqual(errors,[]);fs.writeFileSync(`${dir}/browser-checks.json`,JSON.stringify({url:'http://100.82.115.34:8769/?p=paint-studio-12e572',browser:browser.version(),checks,pageErrors:errors},null,2));console.log(JSON.stringify({passed:checks.length,checks}));
}finally{await browser.close()}})().catch(e=>{console.error(e);process.exitCode=1});
