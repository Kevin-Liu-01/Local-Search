# September launch kit

Status: local drafts and exports. Nothing in this kit has been published.
The browser workflow is available in 0.2.0. Performance figures describe the
unreleased core redesign. Do not call it a new package release.

## Media order

All exports live in `../../site/public/social/2026-09/`.

| Placement | File | Purpose |
| --- | --- | --- |
| X post 1; README lead | agent-browser.gif | Agent command, browser results, signed-in reading, install |
| X post 4 | browser-choice.gif | Choose a profile, approve, reuse, disconnect |
| X post 6 | benchmark.png | Two measured medians and the p95 regression |
| LinkedIn main post | agent-browser.mp4 | Same workflow, with crisp video text |
| Static alternative | hero.png | Product premise without animation |
| Browser choice still | choice.png | Two browser modes and exact commands |
| Mobile-feed still | hero-portrait.png | 1080×1350 product explanation |
| Mobile-feed proof | benchmark-portrait.png | Stacked, readable comparison |

MP4 copies of both GIFs are included. PNG keyframes also serve as still
alternatives. Animation timings explain the steps; they do not represent
measured CLI latency. The public search results come from the July 21 trace in
`site/lib/traces.ts`. Titles are shortened in the illustration. Output is an
excerpt, not the full success envelope. The signed-in illustration contains no
account data and makes no guarantee of access to a particular site.

## Copy

`posts.md` contains the copy-ready nine-post X thread, LinkedIn post, and first
comment. Keep the media with its matching post. No external posting has been
authorized. Keep the unpublished benchmark report link out of public copy
until the evidence and assets are pushed. The first comment intentionally
states that boundary instead of linking to a nonexistent main-branch file.

Before posting, confirm the current published version. These drafts were
prepared against the verified 0.2.0 release on September 25, 2026. Do not use
"today" or claim a new release without rechecking that state.

## Rebuild

Requires Node.js, agent-browser with its browser installed, Python with Pillow,
and ffmpeg. It uses a fresh headless browser session, restricted to loopback.
It never attaches to everyday Chrome or reads signed-in content.

```sh
node scripts/render-launch-media.mjs
```

To inspect the editable composition, run the same command with `--serve`.
Use `?scene=hero`, `search`, `results`, `read`, `choice`, `connect`, `disconnect`,
`outro`, or `benchmark`. Stop the preview server when done.

Source: `media.html`. Real text, embedded inline vector primitives, Manrope,
project colors. The benchmark bars derive from the raw JSON, share a zero
baseline within each comparison, and scale proportionally. The cache and
uncached comparisons use independent scales and explicitly labeled values.
The image-generation study was used only for composition exploration. Its
invented logo, commands, results, gradients, and decoration are not included.

## Evidence and claims

- `../core-redesign.md`: redesign changes, tests, measurements, limitations.
- `../../benchmarks/results/local-ab-2026-09-25-bing-expanded.json`: one Bing
  query, three results, 20 samples per build per mode, two excluded warmups,
  alternating A/B, warm browser and OS, arm64 macOS, Chrome 154.0.8037.57.
- Uncached means local-search `--no-cache`. It does not flush Chrome or OS caches.
- Median: 219.597 to 175.653 ms uncached; 23.908 to 16.4625 ms cached.
- Uncached p95: 294.068 to 328.185 ms, a regression kept on the graphic.
- No new hosted-provider speed, pricing, or token superiority claim.
- Existing-session behavior: root `SKILL.md` and `../verification.md`.
- External primary reference checked September 25:
  https://developer.chrome.com/docs/devtools/agents/get-started/configuration

## Asset provenance

| Asset | Source and rights | SHA-256 |
| --- | --- | --- |
| local-search mark | Own repository `site/components/brand-logo.tsx`; repository MIT license | Source path retained |
| Terminal/browser/data icons | Original inline paths in media.html; repository MIT license | Source path retained |
| Manrope.ttf | Google Fonts, SIL OFL 1.1; notice bundled in fonts/OFL.txt | 3ae11c49db0455a3cc33e37d380f20fdb8c7f8b41dc07625c177e3d87a9d6ae6 |
| OFL.txt | Manrope Project Authors copyright notice and OFL; trailing whitespace normalized | d6a309fcfb963d329a93baf9df66fa5f3c23a4be2b5f67edf24d6cafecde12ac |

Font file: https://raw.githubusercontent.com/google/fonts/main/ofl/manrope/Manrope%5Bwght%5D.ttf

License: https://github.com/google/fonts/blob/main/ofl/manrope/OFL.txt

Retrieved September 25, 2026. The font is unmodified. No third-party brand
artwork, stock images, private captures, or generated concept pixels are shipped.

## Alt text

- Workflow: An agent runs lsearch, local Chrome searches, and compact results
  return. Another example reads a signed-in page using an approved Chrome
  session. Illustrated timing; dated public output; private content omitted.
- Choice: Existing Chrome keeps current logins with approval. A separate profile
  keeps agent sessions apart. The choice is saved, and disconnects return errors.
- Benchmark: Development build versus 0.2.0 on one Bing query. Median uncached
  search 219.6 to 175.7 ms; cached 23.9 to 16.5 ms. Uncached p95 worsened from
  294.1 to 328.2 ms. Twenty samples per build and mode on arm64 macOS.
