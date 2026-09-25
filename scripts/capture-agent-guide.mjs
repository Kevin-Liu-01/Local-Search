#!/usr/bin/env node
// Capture only authored, public-safe documentation. Never attach to user Chrome.
import assert from 'node:assert/strict';
import { execFile } from 'node:child_process';
import { createServer } from 'node:http';
import { mkdir, readFile, readdir } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const exec = promisify(execFile);
const frames = [
  '01-agent-browser', '02-browser-choice', '03-signed-in-sites', '04-search',
  '05-extract', '06-work-with-pages', '07-control',
];

async function findFont() {
  const chunks = path.join(root, 'site/out/_next/static/chunks');
  for (const file of await readdir(chunks)) {
    if (!file.endsWith('.css')) continue;
    const css = await readFile(path.join(chunks, file), 'utf8');
    for (const face of css.matchAll(/@font-face\{[^}]+\}/g)) {
      if (!face[0].includes('font-family:Manrope;') || !/unicode-range:U\+(?:0-FF|\?\?)/.test(face[0])) continue;
      const src = face[0].match(/url\(([^)]+\.woff2)\)/)?.[1];
      if (src?.startsWith('/_next/static/media/')) {
        return readFile(path.join(root, 'site/out', src));
      }
    }
  }
  throw new Error('Build site/ first: the guide reuses its self-hosted Latin Manrope font.');
}

const font = await findFont();
const server = createServer(async (req, res) => {
  try {
    const url = new URL(req.url, 'http://127.0.0.1');
    if (url.pathname === '/__guide-font.woff2') {
      res.writeHead(200, { 'Content-Type': 'font/woff2' });
      res.end(font);
      return;
    }
    const allowed = ['/docs/visual-guide.html', '/docs/fixtures/tasks.html'];
    if (!allowed.includes(url.pathname) && !/^\/site\/public\/brand\/[\w-]+\.svg$/.test(url.pathname)) {
      res.writeHead(404); res.end('Not found'); return;
    }
    const contents = await readFile(path.join(root, url.pathname));
    res.writeHead(200, { 'Content-Type': url.pathname.endsWith('.svg') ? 'image/svg+xml' : 'text/html; charset=utf-8' });
    res.end(contents);
  } catch {
    res.writeHead(500); res.end('Cannot load guide asset');
  }
});
await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
const base = `http://127.0.0.1:${server.address().port}/docs/visual-guide.html`;

if (process.argv.includes('--serve')) {
  console.log(`Visual guide: ${base}`);
  console.log('Local, synthetic documentation only. Stop with Ctrl+C.');
  for (const signal of ['SIGINT', 'SIGTERM']) process.once(signal, () => server.close());
} else {
  const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('AGENT_BROWSER_')));
  const session = `lsearch-docs-${process.pid}`;
  const args = ['--config', path.join(root, 'docs/capture-browser.json'), '--namespace', session,
    '--session', session, '--allowed-domains', '127.0.0.1', '--idle-timeout', '2m'];
  const run = async (...commands) => {
    const { stdout } = await exec('agent-browser', [...args, ...commands], { env, timeout: 60000, maxBuffer: 2_000_000 });
    return stdout;
  };
  const check = async (expression) => {
    const result = JSON.parse(await run('--json', 'eval', expression));
    assert.equal(result.success, true, JSON.stringify(result));
    return result.data.result;
  };
  let started = false;
  try {
    await mkdir(path.join(root, 'docs/images'), { recursive: true });
    started = true;
    await run('batch', '--bail', `open ${base}`, 'set viewport 1440 1080');
    for (const frame of frames) {
      await run('batch', '--bail', `open ${base}?frame=${frame}`, 'eval document.fonts.ready.then(()=>true)');
      assert.equal(await check(`(() => {
        const s = document.querySelector('.screen:not([hidden])');
        return document.fonts.check('22px Manrope') && s.scrollHeight <= 1080 &&
          document.documentElement.scrollWidth <= 1440 &&
          [...s.querySelectorAll('img')].every(i => i.complete && i.naturalWidth > 0);
      })()`), true, `${frame}: font, image, or frame overflow check failed`);
      await run('screenshot', path.join(root, 'docs/images', `${frame}.png`));
      console.log(`Captured ${frame}.png (1440 × 1080)`);
    }
    await run('open', base);
    for (const width of [320, 390, 768, 1440]) {
      await run('set', 'viewport', String(width), '960');
      assert.equal(await check('document.documentElement.scrollWidth <= window.innerWidth'), true, `Overflow at ${width}px`);
      assert.equal(await check(`!Array.from(document.querySelectorAll('h1,h2,h3,p,code,pre,.footer,.top p')).some(n => parseFloat(getComputedStyle(n).fontSize) < 16)`), true, `Small text at ${width}px`);
      console.log(`Readable layout: ${width}px`);
    }
  } finally {
    try { if (started) await run('close'); }
    finally { await new Promise(resolve => server.close(resolve)); }
  }
}
