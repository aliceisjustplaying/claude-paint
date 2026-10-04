// External clock/timer adapter around unchanged production client; no default constants changed.
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';import {join} from 'node:path';import {pathToFileURL} from 'node:url';
const [source,out]=process.argv.slice(2);const {atEasel,WAIT_MS}=await import(pathToFileURL(join(source,'harness/painter/easel-client.ts')));
const originalNow=Date.now,originalTimer=globalThis.setTimeout;const start=originalNow();let timers=[];
Date.now=()=>start+(originalNow()-start)*1000;
globalThis.setTimeout=(fn,ms,...a)=>{timers.push(ms);return originalTimer(fn,Math.max(1,ms/1000),...a)};
const results=[];
for(const [id,args,body] of [
 ['ordinary',['status'],'sleep 5'],
 ['paint',['do','-'],'sleep 5'],
 ['rebuild',['look'],'if [ ! -f seen ]; then touch seen; echo "rebuilding from the log (1 of 3 chunks)"; else sleep 5; fi'],
 ['opening-included',['look'],'if [ "$1" = status ]; then exit 1; fi; if [ "$1" = open ]; then sleep .10; echo ready; else sleep .15; echo image; fi']
]){
 const root=mkdtempSync(join(tmpdir(),'budget-'));mkdirSync(join(root,'bin'));writeFileSync(join(root,'bin/easel'),'#!/bin/sh\necho "$1" >> calls\n'+body+'\n',{mode:0o755});timers=[];const began=originalNow();let value;
 try{value=await atEasel(root,args,'marker=true')}catch(e){value=String(e)}
 results.push({id,default_budgets:{...WAIT_MS},scheduled_ms:timers,elapsed_real_ms:originalNow()-began,result:value,calls:readFileSync(join(root,'calls'),'utf8')});rmSync(root,{recursive:true,force:true});
}
Date.now=originalNow;globalThis.setTimeout=originalTimer;writeFileSync(out,JSON.stringify({clock_scale:1000,warning:'Scheduled budgets with accelerated external clock, not elapsed default-duration runs',results},null,2));
