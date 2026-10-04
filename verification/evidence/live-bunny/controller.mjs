import { RpcClient } from '~/.local/lib/node_modules/@earendil-works/pi-coding-agent/dist/modes/rpc/rpc-client.js';
import { appendFileSync, writeFileSync } from 'node:fs';
const root = '~/src/a/claude-paint-description';
const studio = `${root}/.live/paint-studio-babb1e`;
const evidence = `${root}/verification/evidence/live-bunny`;
const harness = '~/src/a/claude-paint/harness/painter';
let phase = 'initial';
const record = (value) => appendFileSync(`${evidence}/events.jsonl`, JSON.stringify({at:new Date().toISOString(),phase,...value}, (k,v)=>k==='data' && typeof v==='string' && v.length>10000 ? `[image payload omitted: ${v.length} characters]`:v)+'\n');
const client = (recovery=false) => {
 const c = new RpcClient({cliPath:'~/.local/bin/pi',cwd:studio,provider:'opencode-go',model:'space-bunny-free',env:recovery?{PAINTER_SITTING_RECOVERY:'1'}:{},args:['--thinking','xhigh','--no-extensions','-e',`${harness}/painter.ts`,'-e',`${harness}/compaction.ts`,'--system-prompt',`${harness}/system_prompt.md`,'--tools','paint,look,note,status,log,read','--no-context-files','--no-skills','--no-prompt-templates','--no-approve','--session-dir',`${root}/.live/sessions/paint-studio-babb1e`]});
 c.onEvent(e=>{if(e.type!=='message_update'&&e.type!=='tool_execution_update')record(e); if(['tool_execution_start','tool_execution_end','agent_settled'].includes(e.type)) console.log(JSON.stringify({phase,type:e.type,tool:e.toolName,isError:e.isError}));});
 return c;
};
let c=client();
async function turn(message){
 record({type:'probe_prompt',message});
 const done=c.waitForIdle(30*60*1000);
 await c.prompt(message); await done;
 record({type:'probe_state',state:await c.getState()});
}
try {
 await c.start();
 record({type:'probe_state',state:await c.getState()});
 await turn('Read BRIEF.md and the easel guide, then paint the first sitting. Use about 12 successful paint calls and look at the canvas several times as you work. Keep your palette and brush names in persistent Lua globals. Leave a journal note with those names and where you stopped. End this sitting with a look and a short description.');
 phase='compaction'; record({type:'probe_compaction',result:await c.compact()});
 await turn('Continue the same painting for a second sitting. Inspect the canvas and your journal, reuse the existing Lua globals and make 3 to 5 successful paint calls refining the jug, orange and cloth. Look both close up and at the whole canvas. Update your journal and end with a whole-canvas look.');
 await c.stop();
 phase='fresh-recovery';c=client(true);await c.start();
 await turn('Resume your painting for a final short sitting. Inspect the canvas, studio notes and existing state. Reuse the previous palette and brush globals where useful. Make 3 to 5 successful paint calls to improve the light and edges, update your journal and end with a whole-canvas look.');
 record({type:'probe_complete'});
} catch(e) {record({type:'probe_error',error:String(e)}); console.error(String(e));process.exitCode=1;} finally {await c.stop();writeFileSync(`${evidence}/stderr.txt`,c.getStderr());}
