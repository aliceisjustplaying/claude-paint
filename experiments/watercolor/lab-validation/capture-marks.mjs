// MIT License, Copyright (c) 2026 Ryan McCarty.
// Headless harness adapted from Watercolor Lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../THIRD_PARTY_NOTICES.md.
import { createServer } from "node:http";
import { readFile, mkdir, writeFile } from "node:fs/promises";
import { extname, join, resolve } from "node:path";
import { createRequire } from "node:module";

const upstream = resolve(process.env.UPSTREAM ?? `${process.env.HOME}/.capy/work/watercolor-lab-port/upstream`);
const require = createRequire(join(upstream, "package.json"));
const { chromium } = require("playwright-core");
const dist = resolve(process.env.DIST ?? join(upstream, "dist"));
const out = resolve(process.argv[2] ?? `${process.env.HOME}/.capy/work/watercolor-lab-port/captures/original`);
const wanted = process.argv.slice(3);
const marks = JSON.parse(await readFile(new URL("./marks.json", import.meta.url), "utf8"));
const server = createServer(async (req, res) => {
  try {
    const path = (req.url ?? "/").split("?")[0];
    res.setHeader("content-type", { ".js": "text/javascript", ".css": "text/css", ".html": "text/html", ".wasm": "application/wasm" }[extname(path === "/" ? "index.html" : path)] ?? "application/octet-stream");
    res.end(await readFile(join(dist, path === "/" ? "index.html" : path)));
  } catch { res.statusCode = 404; res.end(); }
});
await new Promise((ok) => server.listen(0, "127.0.0.1", ok));
await mkdir(out, { recursive: true });
const browser = await chromium.launch({ executablePath: process.env.PW_CHROMIUM ?? "/usr/local/bin/chromium", args: ["--no-sandbox", "--hide-scrollbars"] });
const page = await browser.newPage({ viewport: { width: 1680, height: 1150 }, deviceScaleFactor: 1 });
page.setDefaultTimeout(300000);
page.setDefaultNavigationTimeout(300000);
const timings = [];

// Source: scripts/scenes.mjs:56–72 and scripts/shoot.mjs:64–75 at the pinned commit.
const run = (program) => {
  const api = window.__wash;
  for (const [kind, a, b] of program) {
    if (kind === "paper") api.setPaper(a);
    else if (kind === "tool") api.setTool(a);
    else if (kind === "pig") api.setPigment(a);
    else if (kind === "size") api.setSize(a);
    else if (kind === "water") api.setWater(a);
    else if (kind === "load") api.setLoad(a);
    else if (kind === "path") api.strokePath(a, b ?? 1);
    else if (kind === "step") api.step(a, b ?? 1);
  }
};

try {
  for (const mark of marks.marks.filter((m) => wanted.length === 0 || wanted.includes(m.name))) {
    const start = performance.now();
    await page.goto(`http://127.0.0.1:${server.address().port}/?sim=${marks.width}x${marks.height}&seed=${marks.seed}`);
    await page.waitForFunction(() => Boolean(window.__wash));
    await page.addStyleTag({ content: `canvas.sheet{width:${marks.width}px!important;height:${marks.height}px!important;max-width:none!important;max-height:none!important}` });
    await page.evaluate(run, mark.program);
    const stages = [];
    for (const stage of marks.stages) {
      await page.evaluate(run, stage.program);
      await page.evaluate(() => new Promise((ok) => requestAnimationFrame(() => requestAnimationFrame(ok))));
      await page.locator("canvas.sheet").screenshot({ path: join(out, `${mark.name}-${stage.name}.png`) });
      stages.push({ name: stage.name, elapsedMs: performance.now() - start });
      if (performance.now() - start > 300000) throw new Error(`${mark.name} exceeded five minutes; captures stopped`);
    }
    const elapsedMs = performance.now() - start;
    console.log(JSON.stringify({ mark: mark.name, elapsedMs }));
    timings.push({ mark: mark.name, elapsedMs, stages });
    await writeFile(join(out, "timings.json"), JSON.stringify({ width: marks.width, height: marks.height, seed: marks.seed, stagesAreFixedStepSamples: true, timings }, null, 2));
  }
} finally { await browser.close(); server.close(); }
