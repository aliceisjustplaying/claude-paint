import { chromium } from 'playwright';
import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import { resolve } from 'node:path';

const browser = await chromium.launch({executablePath: process.env.CHROME_PATH ?? '/usr/local/bin/chromium', headless: true, args: ['--no-sandbox']});
try {
  const page = await browser.newPage({viewport: {width: 1000, height: 900}});
  const errors = [];
  const requests = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('request', request => requests.push(request.url()));
  await page.goto(pathToFileURL(resolve(process.argv[2])).href);
  await page.waitForFunction(() => [...document.images].every(image => image.complete && image.naturalWidth > 0));
  assert.equal(await page.locator('.mark').count(), 6);
  assert.equal(await page.locator('.stage').count(), 28);
  assert.equal(await page.locator('img').count(), 84);
  assert.equal(await page.locator('input[name=view][value=gallery]').isChecked(), true);
  assert.equal(await page.locator('.mark').first().locator('img.gallery').first().isVisible(), true);
  await page.locator('input[name=dry-ours][value=Pass]').check();
  await page.locator('input[name=dry-wet-edge][value=Fail]').check();
  await page.locator('textarea[name=dry-notes]').fill('Example feedback for the copy check.');
  await page.locator('#copy').click();
  const feedback = await page.locator('#feedback').inputValue();
  assert.match(feedback, /Ours: Pass\nWet Edge: Fail/);
  assert.match(feedback, /Example feedback for the copy check/);
  assert.match(feedback, /6\. Flat wash/);
  await page.reload();
  assert.equal(await page.locator('input[name=dry-ours][value=Pass]').isChecked(), true);
  assert.equal(await page.locator('textarea[name=dry-notes]').inputValue(), 'Example feedback for the copy check.');
  await page.locator('input[name=view][value=color]').check();
  assert.equal(await page.locator('.mark').first().locator('img.color').first().isVisible(), true);
  assert.equal(await page.locator('.mark').first().locator('img.gallery').first().isVisible(), false);
  await page.locator('input[name=view][value=gallery]').check();
  const dimensions = await page.locator('.pair').first().locator('.paper').evaluateAll(nodes => nodes.map(node => ({width: node.getBoundingClientRect().width, height: node.getBoundingClientRect().height})));
  assert.deepEqual(dimensions[0], dimensions[1]);
  if (process.argv[3]) await page.locator('.stage').first().screenshot({path: process.argv[3]});
  await page.setViewportSize({width: 390, height: 844});
  const mobile = await page.locator('.pair').first().locator('.paper').evaluateAll(nodes => nodes.map(node => ({width: node.getBoundingClientRect().width, height: node.getBoundingClientRect().height})));
  assert.deepEqual(mobile[0], mobile[1]);
  assert.equal(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), true);
  assert.deepEqual(errors, []);
  assert.equal(requests.every(url => url.startsWith('file:') || url.startsWith('data:')), true);
  console.log(JSON.stringify({marks: 6, setups: 7, stages: 28, images: 84, desktop: dimensions, mobile, feedback: 'collected and persisted', errors, externalRequests: 0}, null, 2));
} finally {
  await browser.close();
}
