const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');
const { test } = require('node:test');

const html = fs.readFileSync(path.join(__dirname, 'index.html'), 'utf8');
const classifiers = html.slice(html.indexOf('const isWhole ='), html.indexOf('let viewI3 ='));
const selector = html.match(/^function picturesAt\([\s\S]*?^}/m)[0];
function pictures(events, rest = false) {
  const context = vm.createContext({ events, hasLooks:events.some(e => e.kind === 'look'), displayLight:'color' });
  vm.runInContext(classifiers + '\n' + selector, context);
  return Array.from(vm.runInContext(`picturesAt(events.length - 1, ${rest})`, context));
}
const mask = [{ kind:'look', text:'mask sky' }, { kind:'image', img:1, look:'mask sky' }];

test('a first mask stays a study without becoming the painting', () => {
  assert.deepEqual(pictures(mask), [-1, 1]);
  assert.deepEqual(pictures(mask, true), [-1, -1]);
  assert.deepEqual(pictures([...mask, { kind:'sitting' }]), [-1, -1]);
});

test('a mask keeps the earlier legacy whole painting', () => {
  assert.deepEqual(pictures([
    { kind:'image', img:0, whole:true },
    { kind:'image', img:2, whole:false },
    ...mask,
  ]), [0, 3]);
});

test('a mask keeps the legacy read-image fallback when no whole is marked', () => {
  assert.deepEqual(pictures([{ kind:'image', img:0, whole:false }, ...mask]), [0, 2]);
  assert.deepEqual(pictures([{ kind:'image', img:0 }, ...mask]), [0, 2]);
});

test('a mask keeps an earlier modern whole painting', () => {
  assert.deepEqual(pictures([{ kind:'look', text:'' }, { kind:'image', img:0, look:'' }, ...mask]), [1, 3]);
});

test('non-mask image fallbacks and references are preserved', () => {
  assert.deepEqual(pictures([{ kind:'image', img:0, whole:false }]), [0, -1]);
  assert.deepEqual(pictures([{ kind:'look', text:'crop 0,0,50,50' }, { kind:'image', img:0, look:'crop 0,0,50,50' }]), [1, -1]);
  assert.deepEqual(pictures([{ kind:'image', img:0, ref:true }]), [-1, 0]);
  assert.deepEqual(pictures([{ kind:'look', text:'' }, { kind:'image', img:0, look:'' }, { kind:'image', img:2 }]), [1, 2]);
});
