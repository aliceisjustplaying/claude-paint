import {RpcClient} from '~/.local/lib/node_modules/@earendil-works/pi-coding-agent/dist/modes/rpc/rpc-client.js';
import {appendFileSync} from 'node:fs';
const root='~/src/a/claude-paint-description';
const h='/private/tmp/bunny.KUeYlZ/fix/harness/painter';
const c=new RpcClient({cliPath:'~/.local/bin/pi',cwd:`${root}/.live/paint-studio-c10c`,provider:'opencode-go',model:'space-bunny-free',args:['--thinking','xhigh','--no-extensions','-e',`${h}/painter.ts`,'-e',`${h}/compaction.ts`,'--system-prompt',`${h}/system_prompt.md`,'--tools','paint,look,note,status,log,read','--no-context-files','--no-skills','--no-prompt-templates','--no-approve','--session-dir',`${root}/.live/sessions/paint-studio-c10c`]});
c.onEvent(e=>{if(!['message_update','tool_execution_update'].includes(e.type))appendFileSync(`${root}/verification/evidence/live-bunny/clock-provider.jsonl`,JSON.stringify(e,(k,v)=>k==='data'&&typeof v==='string'&&v.length>10000?'[image omitted]':v)+'\n');});
try{await c.start();const done=c.waitForIdle(300000);await c.prompt('Read BRIEF.md and perform its exact two-call clock check. Do not print the time or anything else from Lua.');await done;}finally{await c.stop();}
