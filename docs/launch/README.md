# September launch kit

Updated September 29, 2026. The lead media is now a continuous Chrome demo,
not the earlier text-heavy slide animation. No social posts have been published
by this task. The browser workflow is available in 0.2.0. The context chart
measures that published release against fresh hosted-provider requests.

## Media order

All exports live in `../../site/public/social/2026-09/`.

| Placement | File | Purpose |
| --- | --- | --- |
| X post 1; README lead | chrome-demo.gif | A command types, Chrome searches, then a docs page opens |
| LinkedIn main post | chrome-demo.mp4 | Same 12-second sequence, 1280×720 |
| Minimal cover | chrome-cover.png | Brand, one line, install command |
| Product still | chrome-demo.png | Terminal and Chrome, no explanatory paragraphs |
| X post 6; README benchmark | benchmark-clean.png | Context per result across matched searches |
| Narrow-screen benchmark | benchmark-mobile.png | Same chart with stacked, readable provider rows |

Desktop stills are 1280×720; the mobile chart is 390px wide. The looping GIF is 960×540. The terminal and browser
are exactly 50–50 (580px each). They stay fixed while commands, navigation,
results, and returned data change. Results appear in under a second, then both
panels scroll through all three results. A
three-second closing card gives one install command. No slide deck or repeated
marketing headlines interrupts the workflow.

The terminal header shows Claude, Codex, Cursor, and OpenClaw. Chrome and
DuckDuckGo identify the illustrated browser and search; Rust identifies the
docs page. The closing frame groups agent logos and the four supported search
engines side by side. Monochrome Codex and Cursor marks use white on the dark
terminal and their original black on the light ending.

Timing is illustrative, not measured latency. Search uses the July 21, 2026
`tokio-runtime` trace in `site/lib/traces.ts`. The JSON view reveals the complete
recorded search response, formatted for readability, including all three results
and their titles, URLs, domains, and snippets. The docs view and
Markdown are shortened, authored excerpts based on the public
[Tokio runtime page](https://docs.rs/tokio/latest/tokio/runtime/), checked
September 29, 2026. This is not a fresh live search or signed-in access test.
The demo starts after browser setup. It contains no account data.

The older `agent-browser`, `browser-choice`, `hero`, and portrait assets remain
historical references. Do not use their text-heavy slides as the lead media.

## Copy

`posts.md` contains the copy-ready nine-post X thread, LinkedIn post, and first
comment. Keep the media with its matching post. No external posting has been
authorized. Check the draft's release wording before posting. The benchmark
evidence and revised media are versioned with the source; committing them does
not publish a social post or a package release.

Before posting, confirm the current published version. These drafts were
prepared against the verified 0.2.0 release on September 25, 2026. Do not use
"today" or claim a new release without rechecking that state.

## Rebuild

Requires Node.js, agent-browser with its browser installed, Python with Pillow,
and ffmpeg. It uses a fresh headless browser session, restricted to loopback.
It never attaches to everyday Chrome or reads signed-in content.

```sh
node scripts/render-chrome-demo.mjs
```

Add `--stills` to render the three PNGs and layout checks without re-encoding.
Use `--benchmark-only` to capture the desktop/mobile chart without changing the demo.
Use `--serve` for an editable, looping preview. `?t=2.3` freezes results loading,
`?t=4.2` shows the completed response, `?t=7.2` freezes reading, and `?t=10`
shows the cover. Stop the preview server when done.

Sources: `chrome-demo.html` and `benchmark-card.html`. Exact text remains editable
HTML, using bundled Manrope and the project palette. The benchmark reads the raw
JSON. All five context bars share a zero baseline and scale. The chart joins
successful query/depth pairs across providers; it never compares different
successful subsets. Frames stay in ignored
`artifacts/chrome-demo-frames/`. The previous slideshow renderer is retained only
to reproduce historical assets.

## Evidence and claims

- `../../benchmarks/search-2026-09-29.md`: complete methodology and outcomes.
- `../../benchmarks/results/search-comparison-2026-09-29.json`: ten matched
  three-result queries, common schema, 120-character snippet cap, o200k_base.
- Median tokens/result: local-search 54.7, Exa 68.5, Brave 71.0, Tavily 73.5,
  Firecrawl 70.3. This is context size, not a relevance or quality ranking.
- The 24-request run includes failed and underfilled hosted responses. The
  chart is explicitly the common successful subset, not all 24 requests.
- No speed-ranking or current-price claim. The latency run had high machine load.
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

Font retrieved September 25, 2026, unmodified. New window geometry and navigation
icons are original HTML/SVG in `chrome-demo.html` under the repository MIT license.
No stock-image plan, per-export fee, attribution requirement beyond the bundled
MIT/OFL notices, or account is involved. Editable source is delivered with the
renders; no third-party artwork ownership or trademark rights are transferred.

The user's existing repository logo assets are reused at their explicit request.
They identify the browser, supported engines, and compatible shell agents;
they are not partnership or endorsement badges. No new stock assets were
downloaded. See [logo asset receipt](logo-assets.md) for exact files and hashes.
Reference: [Google brand guidance](https://about.google/brand-resource-center/guidance/)
and [current icon guidance](https://partnermarketinghub.withgoogle.com/brands/google/branding-guidelines/how-to-show-googles-brand/#product-icons),
checked September 29, 2026; no displayed revision date or approval account.
Reference SVG SHA-256: `e97c9c44672f1b40ca0700751318a4aafcb6e69138575f68b0c203cb3ec55dfc`.
Third-party marks remain their owners' property. This reuse does not assert
ownership, endorsement, or a separate trademark license. No private captures
or generated concept pixels ship. The font license is bundled in `fonts/OFL.txt`.

## Alt text

- Workflow: Equal-width terminal and Chrome windows load and scroll through all
  three public search results and the complete JSON response. A read command opens the
  first docs page and returns a shortened Markdown excerpt. Illustrated timing.
- Cover: local-search. Your agent. Your Chrome. cargo install local-search.
- Choice: Existing Chrome keeps current logins with approval. A separate profile
  keeps agent sessions apart. The choice is saved, and disconnects return errors.
- Benchmark: Median context tokens per result for ten matched three-result
  queries: local-search 54.7, Exa 68.5, Brave Search 71.0, Tavily 73.5, Firecrawl
  70.3. Same 120-character snippet cap and common JSON schema. September 29, 2026.
