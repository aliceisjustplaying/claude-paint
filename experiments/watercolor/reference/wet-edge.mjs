import { chromium } from 'playwright';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const manifest = JSON.parse(await readFile(`${here}/marks.json`, 'utf8'));
const output = resolve(process.argv[2]);
const browser = await chromium.launch({
  executablePath: process.env.CHROME_PATH ?? '/usr/local/bin/chromium',
  headless: true,
  args: ['--no-sandbox', '--enable-unsafe-webgpu', '--use-angle=swiftshader', '--disable-gpu-watchdog', '--enable-features=Vulkan', '--use-vulkan=swiftshader']
});
try {
  const page = await browser.newPage({viewport: {width: 1200, height: 800}});
  page.on('pageerror', e => console.error('PAGEERROR', e.message));
  await page.route('**/src/main.js', async route => {
    const response = await route.fetch();
    const source = await response.text();
    if (!source.includes('const W = 1024, H = 768, N = W * H;')) throw new Error('Unrecognized upstream dimensions');
    await route.fulfill({response, body: source.replace('const W = 1024, H = 768, N = W * H;', 'const W = 96, H = 64, N = W * H;')});
  });
  await page.route('**/src/minds.js', async route => {
    const response = await route.fetch();
    const source = await response.text();
    if (!source.includes('const SW = 1024, SH = 768;')) throw new Error('Unrecognized upstream mind dimensions');
    await route.fulfill({response, body: source.replace('const SW = 1024, SH = 768;', 'const SW = 96, SH = 64;')});
  });
  await page.exposeFunction('__saveLook', async (name, b64) => {
    await writeFile(`${output}/${name}.png`, Buffer.from(b64, 'base64'));
  });
  await page.goto(process.env.APP_URL ?? 'http://127.0.0.1:8765');
  await page.waitForFunction(() => window.__sim?.open, {timeout: 60000});
  for (const test of manifest.cases) {
    await mkdir(`${output}/${test.id}`, {recursive: true});
    const result = await page.evaluate(async ({test, manifest}) => {
      const S = window.__sim, h = S.headless, V = S.values;
      h.begin();
      h.setPaper('coldPress', 71);
      h.setTone('natural');
      S.clear();
      h.setBrushPreset('round');
      V.brushRadius = manifest.brush.width_mm / .4;
      V.brushWater = manifest.brush.water_film_mm;
      V.brushPigment = manifest.brush.wet_edge_strength;
      V.simSpeed = 600;
      V.dipLoad = 1;
      const point = [48, 32];
      const sense = async (x = 9.6, y = 6.4) => {
        const sensed = await S.sense(x / .2, y / .2, 3);
        const cell = await S.cell(Math.round(x / .2), Math.round(y / .2));
        const a = await S.read();
        let maxWater = 0, maxPore = 0;
        for (let i = 0; i < a.length; i += 4) {maxWater = Math.max(maxWater, a[i]); maxPore = Math.max(maxPore, a[i + 3]);}
        return {...sensed, time: cell.time, max_water: maxWater, max_pore: maxPore};
      };
      const waitStage = async stage => {
        const skipped = await S.skipTo(stage, {points: [point], maxS: 1800});
        h.begin();
        if (skipped < 0) throw new Error(`Failed to reach ${stage}`);
        return skipped;
      };
      const stroke = async (pigment, y, x0 = 2.4, x1 = 16.8) => {
        S.rinse();
        S.tool(pigment ? 'paint' : 'water');
        if (pigment) h.setBrush(pigment);
        h.lift();
        const n = manifest.brush.segments;
        for (let k = 0; k < n; k++) {
          const t = (k + .5) / n;
          const pressure = Math.min(1, t / .2, (1 - t) / .2);
          await h.paint((x0 + (x1 - x0) * k / n) / .2, y / .2,
                        (x0 + (x1 - x0) * (k + 1) / n) / .2, y / .2,
                        manifest.brush.frames_per_segment, pressure);
        }
        h.lift();
      };
      S.path = async (points, framesPerSeg = 4) => {
        h.begin(); h.lift();
        for (let k = 1; k < points.length; k++) {
          const [ax, ay, ap = 1] = points[k - 1], [bx, by, bp = 1] = points[k];
          for (let f = 0; f < framesPerSeg; f++) {
            const a = f / framesPerSeg, b = (f + 1) / framesPerSeg;
            await h.paint(ax + (bx - ax) * a, ay + (by - ay) * a,
                          ax + (bx - ax) * b, ay + (by - ay) * b, 1, ap + (bp - ap) * ((a + b) / 2));
          }
        }
        h.lift();
      };
      const receipt = {id: test.id, initial: await sense(), adapter: await navigator.gpu.requestAdapter().then(a => ({vendor: a.info.vendor, architecture: a.info.architecture}))};
      if (['damp', 'shiny'].includes(test.paper_stage)) {
        for (const y of [4.8, 6.4, 8]) await stroke(null, y);
        if (test.paper_stage === 'damp') await waitStage('damp');
        receipt.before_paint = await sense();
      }
      if (test.kind === 'stroke') {
        await stroke('French Ultramarine', 6.4);
      } else if (test.kind === 'touch') {
        await stroke('French Ultramarine', 5.2);
        receipt.before_second = await sense(9.6, 5.2);
        await stroke('Burnt Sienna', 7.6);
      } else if (test.kind === 'overlap') {
        await stroke('French Ultramarine', 6.4, 2.4, 12);
        if (test.second_stage === 'damp') await waitStage('damp');
        receipt.before_second = await sense();
        await stroke('Burnt Sienna', 6.4, 7.2, 16.8);
      } else if (test.kind === 'wash') {
        h.setBrush('French Ultramarine');
        await S.wash([[6, 10], [90, 10], [90, 54], [6, 54]], {kind: 'flat', paper: 'moist', dampen: false, even: true});
        h.begin();
      }
      receipt.stages = [];
      for (const label of ['laid', 'satin', 'damp', 'dry']) {
        const skipped = label === 'laid' ? 0 : await waitStage(label);
        await S.look(`${test.id}/${label}`);
        receipt.stages.push({label, skipped, ...await sense()});
        console.log('[reference]', test.id, label, JSON.stringify(receipt.stages.at(-1)));
      }
      return receipt;
    }, {test, manifest});
    await writeFile(`${output}/${test.id}/receipt.json`, JSON.stringify(result, null, 2));
    console.log('DONE', test.id, result.stages.map(s => [s.label, s.time, s.stage, s.max_water, s.max_pore]));
  }
} finally {
  await browser.close();
}
