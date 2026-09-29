# Launch media verification

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
