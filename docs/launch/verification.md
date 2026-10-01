# Launch media verification

## September 30 benchmark presentation

- Added a matching July latency chart before the token chart in the README.
  Desktop and mobile exports retain the mixed-cache label and local cold/cache
  breakdown. Automated assertions check those disclosures, provider medians,
  and proportional bars. No fresh latency test or universal speed claim.

- The README now selects the July 21 historical comparison at the user's request.
  Separate July exports preserve the September equal-budget chart and evidence.
  The chart labels its recorded date and unequal snippet lengths; values and
  baseline ratios are verified against the committed July summary.

- README chart matches the site's dark, logo-led layout. It retains September 29
  matched-query evidence, not the older July values in the visual reference.
- Verified the 1280×720 export and full-height 390px mobile chart visually.
  All five values, baseline ratios, proportional bar widths, and decoded logos
  are checked against the source data. No horizontal overflow on mobile.
- Renderer syntax and whitespace checks pass. The demo cleanup remains intact;
  chart-only rendering does not change the GIF or video.

## September 29 revision

- Replaced the slide-style README lead with a 12-second terminal/Chrome sequence.
  Editable HTML has deterministic `renderFrame(t)` timing and a reduced-motion
  still preview. No logged-in browser is used by the renderer.
- Exported H.264 video: 1280×720, 20 fps, 240 frames, 12 seconds.
- Exported looping GIF: 960×540, 180 decoded frames, 12 seconds, loop=0.
- Inspected search, read, and ending frames. Checked font loading, frame bounds,
  command wrapping, terminal content fit, and clearance below the third result.
  Both windows measure 580px. Seven timeline checkpoints cover loading, streamed
  output, completed search, reading, and the ending. The completed JSON is parsed
  and checked for `ok: true` and result ranks `[1, 2, 3]`; the terminal reaches the
  final line and Chrome scrolls far enough to show the full third result.
- Three stills: UI poster, minimal cover, and simplified benchmark comparison.
  The benchmark values are read from the committed JSON, not typed by hand.
- Website lint, typecheck, and production build pass. The renderer syntax check,
  GIF full-frame decode, MP4 full decode, and `git diff --check` also pass.
- The public results retain their July 21 provenance. The shortened docs excerpt
  was checked against docs.rs on September 29. No new performance or live-session
  claim is made. Illustrated timing and shortened page text are documented in
  surrounding copy, not overlaid on the animation. September 30 removed the
  numbered chapter labels and bottom-left captions; every checked frame
  asserts those elements are absent.
- At the user's request, restored the repository's existing browser, engine,
  and agent logos. All images must decode before capture. The ending checks
  require four agent and four engine logos with clearance above the footer.
  See `logo-assets.md` for the reused assets and provenance boundary.
- README and social media pointers now use the new exports. This revision
  updates repository media only; it does not post externally or release a package.

The per-frame layout evidence is `site/public/social/2026-09/chrome-demo-checks.json`.
The renderer source is `scripts/render-chrome-demo.mjs`; frames are ignored.

## Previous slide kit

Prepared September 25, 2026 (Pacific) in the core-redesign worktree.

## Passed

- Eleven PNG compositions rendered with the bundled Manrope font: nine landscape
  scenes and two portrait graphics.
- Layout checks cover frame bounds, footer clearance, and terminal/browser
  content clipping. Landscape supporting type is at least 22px at 1200px width.
- Visual inspection covered the product, search, result, signed-in, approval,
  install, choice, and benchmark states, including portrait variants.
- Workflow GIF: 960×720, 160 decoded frames, 16 seconds, 2,033,401 bytes.
- Browser-choice GIF: 960×720, 120 decoded frames, 12 seconds, 1,357,327 bytes.
- Matching 1200×900 H.264 MP4s decoded without errors.
- All nine X posts fit 280 characters including their `1/9` numbering.
- New social copy has no em dashes or en dashes. CLI flag hyphens are preserved.
- Renderer syntax checks and `git diff --check` passed.
- Two independent review passes covered claims and copy/design. Corrections
  included the unreleased recording cap, existing-browser-only approval wording,
  evidence-publication status, flag wrapping, clipped result text, disconnect
  wording, and readable approval-panel padding.

Machine-readable layout and encoding checks are beside the exported media.
No website application code or Rust implementation changed in this media pass;
the earlier core-redesign verification remains separate.

## Publication boundary

Media and copy are local drafts. No social posts, package releases, commits, or
pushes were performed in this pass. The README's new raw-GitHub asset URLs will
resolve after these files reach main. Publish benchmark evidence with the
graphic, then replace its publication-pending caption and regenerate.

The animations are authored illustrations, not real-time recordings or new
live account tests. They contain only previously recorded public search results,
generic browser UI, and omitted private-content placeholders.
