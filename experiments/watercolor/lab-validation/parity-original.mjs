// MIT License, Copyright (c) 2026 Ryan McCarty.
// Headless harness adapted from Watercolor Lab at a19bfa2a026856a98ceb400a8eec41f091f04a11.
// Full license: ../THIRD_PARTY_NOTICES.md.
import { createServer } from "node:http";
import { mkdir, writeFile } from "node:fs/promises";
import { resolve, join } from "node:path";
import { createRequire } from "node:module";
const root = resolve(import.meta.dirname);
const upstream = resolve(process.env.UPSTREAM ?? `${process.env.HOME}/.capy/work/watercolor-lab-port/upstream`);
const require = createRequire(join(upstream, "package.json"));
const { build } = require("esbuild");
const { chromium } = require("playwright-core");
const result = await build({ entryPoints: [join(root, "parity-page.js")], bundle: true, write: false, format: "iife", platform: "browser" });
const bundle = result.outputFiles[0].text;
const server = createServer((req, res) => {
  res.setHeader("content-type", req.url === "/parity.js" ? "text/javascript" : "text/html");
  res.end(req.url === "/parity.js" ? bundle : '<canvas width="16" height="12"></canvas><script src="/parity.js"></script>');
});
await new Promise((ok) => server.listen(0, "127.0.0.1", ok));
const browser = await chromium.launch({ executablePath: process.env.PW_CHROMIUM ?? "/usr/local/bin/chromium", args: ["--no-sandbox"] });
try {
  const page = await browser.newPage();
  await page.goto(`http://127.0.0.1:${server.address().port}/`);
  const data = await page.evaluate(() => window.runParity());
  const out = resolve(process.argv[2] ?? `${process.env.HOME}/.capy/work/watercolor-lab-port/captures/original-parity.json`);
  await mkdir(resolve(out, ".."), { recursive: true });
  await writeFile(out, JSON.stringify(data));
  console.log(JSON.stringify({ out, renderer: data.renderer, cases: data.cases.map((c) => ({ name: c.name, passes: c.captures.length })) }));
} finally { await browser.close(); server.close(); }
