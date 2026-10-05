import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';

test('exported pixels do not imply completion and unchanged exports refresh outcomes', (t) => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'paint-watch-'));
  try {
    const watcher = path.join(root, 'night-watch');
    const destination = path.join(root, 'finished');
    fs.mkdirSync(watcher); fs.mkdirSync(destination);
    fs.copyFileSync(new URL('./watch.mjs', import.meta.url), path.join(watcher, 'watch.mjs'));
    const png = Buffer.alloc(24);
    Buffer.from('89504e470d0a1a0a', 'hex').copy(png);
    png.writeUInt32BE(2400, 16); png.writeUInt32BE(1600, 20);
    const sha256 = createHash('sha256').update(png).digest('hex');
    const state = { stoppedAt: 'already stopped', stopAt: 'previous scheduled stop', finishedAt: 'old export time' };
    for (const [round, name, reason] of [
      ['r24', 'fable', 'the painter is done: sitting 2 added no painting'],
      ['r24.3', 'opus', 'NOT FINISHED: stopped after 4 sittings (MAX_SITTINGS)'],
    ]) {
      const run = path.join(root, 'tmp/gallery-fcf9c110', round, 'run');
      fs.mkdirSync(path.join(run, 'INNS'), { recursive: true });
      fs.writeFileSync(path.join(run, 'INNS/p1.finished'), 'render exists');
      fs.writeFileSync(path.join(run, 'INNS/p1.painted'), reason);
      fs.writeFileSync(path.join(run, 'INNS1_finished.png'), png);
      fs.writeFileSync(path.join(run, 'INNS1_finished.lua'), 'varnish{coats=0.4}\ncracks{}');
      fs.writeFileSync(path.join(destination, `${name}.png`), png);
      fs.writeFileSync(path.join(destination, `${name}.lua`), 'already preserved');
      state[name] = { sha256, preservedAt: 'original preservation' };
    }
    fs.writeFileSync(path.join(watcher, 'state.json'), JSON.stringify(state));
    const runWatcher = () => execFileSync(process.execPath, [path.join(watcher, 'watch.mjs')],
      { env: { ...process.env, HOME: root }, encoding: 'utf8' });
    const output = runWatcher();
    const refreshed = JSON.parse(fs.readFileSync(path.join(watcher, 'state.json')));
    assert.equal(refreshed.finishedAt, undefined);
    assert.equal(refreshed.renderExportedAt, 'old export time');
    assert.match(output, /opus: painter cap_reached; final render exported/);
    const opus = JSON.parse(fs.readFileSync(path.join(destination, 'opus.json')));
    assert.equal(opus.painterOutcome.status, 'cap_reached');
    assert.equal(opus.renderExported, true);
    assert.equal(opus.preservedAt, 'original preservation');
    t.diagnostic(JSON.stringify({ evidence_kind: 'Offline synthetic export files, no painting launched',
      watcher_output: output.trim().split('\n'), exported_metadata: opus }));
    assert.equal(JSON.parse(fs.readFileSync(path.join(destination, 'fable.json'))).painterOutcome.status, 'finished');
    const updated = { status: 'finished', reason: 'the painter is done: sitting 5 reviewed whole and detail views and added no painting' };
    fs.writeFileSync(path.join(root, 'tmp/gallery-fcf9c110/r24.3/run/INNS/p1_outcome.json'), JSON.stringify(updated));
    assert.match(runWatcher(), /opus: painter finished; final render exported/);
    assert.deepEqual(JSON.parse(fs.readFileSync(path.join(destination, 'opus.json'))).painterOutcome, updated);
    assert.equal(fs.readFileSync(path.join(destination, 'opus.lua'), 'utf8'), 'already preserved');
  } finally { fs.rmSync(root, { recursive: true, force: true }); }
});
