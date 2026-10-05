import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const home = os.homedir();
const runs = path.join(home, 'tmp/gallery-fcf9c110');
const destination = path.resolve(here, '../finished');
const statePath = path.join(here, 'state.json');
const opus = path.join(runs, 'r24.3/run');
const checkOnly = process.argv.includes('--check');
let state = fs.existsSync(statePath) ? JSON.parse(fs.readFileSync(statePath, 'utf8')) : {};
function save() {
  fs.writeFileSync(statePath + '.part', JSON.stringify(state, null, 2) + '\n');
  fs.renameSync(statePath + '.part', statePath);
}
function report(message) { console.log(new Date().toISOString(), message); }

function painterOutcome(run) {
  const file = path.join(run, 'INNS/p1_outcome.json');
  if (fs.existsSync(file)) return JSON.parse(fs.readFileSync(file, 'utf8'));
  // Existing runs recorded the runner's decision in this marker, before JSON outcomes existed.
  const marker = path.join(run, 'INNS/p1.painted');
  if (!fs.existsSync(marker)) return { status: 'unknown', reason: 'No runner outcome recorded' };
  const reason = fs.readFileSync(marker, 'utf8').trim();
  const status = /MAX_SITTINGS/.test(reason) ? 'cap_reached'
    : /MAX_CRASHES/.test(reason) ? 'crash_limit_reached'
    : /the painter is done:/.test(reason) && !/NOT FINISHED/.test(reason) ? 'finished' : 'unknown';
  return { status, reason };
}

// The runner writes p1.finished only after finish_painting succeeds.
function preserve(run, name) {
  const marker = path.join(run, 'INNS/p1.finished');
  if (!fs.existsSync(marker)) return null;
  const png = path.join(run, 'INNS1_finished.png');
  const lua = path.join(run, 'INNS1_finished.lua');
  const source = fs.readFileSync(lua, 'utf8');
  if (!source.includes('varnish{coats=0.4}') || !source.includes('cracks{}')) {
    throw new Error(`${name}: finished log lacks the requested varnish and crackle`);
  }
  const pixels = fs.readFileSync(png);
  if (pixels.length < 24 || pixels.subarray(0, 8).toString('hex') !== '89504e470d0a1a0a') {
    throw new Error(`${name}: finished render is not a PNG`);
  }
  fs.mkdirSync(destination, { recursive: true });
  const digest = createHash('sha256').update(pixels).digest('hex');
  const outcome = painterOutcome(run);
  if (state[name]?.sha256 !== digest || !fs.existsSync(path.join(destination, `${name}.png`))) {
    for (const [from, extension] of [[png, 'png'], [lua, 'lua']]) {
      const to = path.join(destination, `${name}.${extension}`);
      fs.copyFileSync(from, to + '.part');
      fs.renameSync(to + '.part', to);
    }
    state[name] = { sha256: digest, width: pixels.readUInt32BE(16), height: pixels.readUInt32BE(20), preservedAt: new Date().toISOString() };
    save();
    report(`${name}: preserved final PNG and Lua with varnish and crackle`);
  }
  if (JSON.stringify(state[name].painterOutcome) !== JSON.stringify(outcome) || !state[name].renderExported) {
    Object.assign(state[name], { painterOutcome: outcome, renderExported: true });
    save();
    report(`${name}: painter ${outcome.status}; final render exported`);
  }
  fs.writeFileSync(path.join(destination, `${name}.json`), JSON.stringify(state[name], null, 2) + '\n');
  return fs.statSync(marker).mtimeMs;
}

async function obs(requestType) {
  const config = JSON.parse(fs.readFileSync(path.join(home, 'Library/Application Support/obs-studio/plugin_config/obs-websocket/config.json'), 'utf8'));
  return new Promise((resolve, reject) => {
    const socket = new WebSocket(`ws://127.0.0.1:${config.server_port || 4455}`);
    const timer = setTimeout(() => finish(new Error('OBS request timed out')), 10000);
    let settled = false;
    function finish(error, result) {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      socket.close();
      error ? reject(error) : resolve(result);
    }
    socket.addEventListener('error', () => finish(new Error('OBS WebSocket connection failed')));
    socket.addEventListener('close', () => { if (!settled) finish(new Error('OBS closed before replying')); });
    socket.addEventListener('message', event => {
      try {
        const message = JSON.parse(event.data);
        if (message.op === 0) {
          const d = { rpcVersion: 1 };
          if (message.d.authentication) {
            const { salt, challenge } = message.d.authentication;
            const hash = value => createHash('sha256').update(value).digest('base64');
            d.authentication = hash(hash(config.server_password + salt) + challenge);
          }
          socket.send(JSON.stringify({ op: 1, d }));
        } else if (message.op === 2) {
          socket.send(JSON.stringify({ op: 6, d: { requestType, requestId: 'night-watch' } }));
        } else if (message.op === 7) {
          if (!message.d.requestStatus.result) throw new Error(`OBS ${requestType}: ${message.d.requestStatus.comment || message.d.requestStatus.code}`);
          finish(null, message.d.responseData || {});
        }
      } catch (error) { finish(error); }
    });
  });
}

async function tick() {
  if (state.finishedAt) {
    state.renderExportedAt ??= state.finishedAt;
    delete state.finishedAt;
    save();
  }
  preserve(path.join(runs, 'r24/run'), 'fable');
  const completedAt = preserve(opus, 'opus');
  if (completedAt !== null && !state.stopAt) {
    state.renderExportedAt = new Date(completedAt).toISOString();
    state.stopAt = new Date(completedAt + 15 * 60 * 1000).toISOString();
    save();
    report(`Opus final render exported (${state.opus.painterOutcome.status}); OBS stop scheduled at ${state.stopAt}`);
  }
  if (checkOnly) {
    const status = await obs('GetStreamStatus');
    console.log(JSON.stringify({ ...state, outputActive: status.outputActive, watcher: 'read-only OBS check' }, null, 2));
    return true;
  }
  if (state.stoppedAt) return true;
  if (state.stopAt && Date.now() >= Date.parse(state.stopAt)) {
    const before = await obs('GetStreamStatus');
    if (before.outputActive) await obs('StopStream');
    const after = await obs('GetStreamStatus');
    if (after.outputActive) throw new Error('OBS is still streaming after StopStream; will retry');
    state.stoppedAt = new Date().toISOString();
    state.outputActive = false;
    save();
    report('OBS stream confirmed stopped; watcher complete');
    return true;
  }
  return false;
}

report(checkOnly ? 'Checking watcher and OBS' : 'Watching Opus final render export; OBS remains live until 15 minutes afterward');
let lastError;
while (true) {
  try {
    if (await tick()) break;
    if (lastError) report('Watcher recovered');
    lastError = undefined;
  } catch (error) {
    if (checkOnly) throw error;
    if (error.message !== lastError) report(`ERROR: ${error.message}`);
    lastError = error.message;
  }
  await new Promise(resolve => setTimeout(resolve, 15000));
}
