# local-search site design contract

## Product point of view

The browser the developer already has is the product asset. The site should feel like a modern,
direct piece of agent infrastructure: explicit, local, inspectable, and fast. Product proof is the
interactive agent trace and structured output—not decorative marketing imagery.

## Visual grammar

- Cursor-inspired marketing stack: claim, actions, full-width product demo, proof, features, and data
  are placed vertically rather than split into side-by-side hero columns.
- Manrope is the only font family, including commands and agent interfaces.
- Off-black and cool off-white are the foundation. Lavender is the sole marketing field; acid is
  reserved for benchmark proof and green for successful runtime states.
- The hero uses measured side rails, corner reticles, and hatched spacer bands as its construction
  system. Decorative connector diagrams or generic agent nodes do not belong behind the copy.
- Dither fields are deterministic dot grids built from 5, 8, and 13 pixel spacing and elliptical
  masks. Reuse them across major sections at low contrast instead of adding background line art.
- Agent shells use the MIT-licensed `theswerd/brainless` Claude/Codex primitives. Cursor remains a
  locally owned recreation because brainless does not ship Cursor components.
- Rounded geometry is limited to interactive product surfaces and faithful agent chrome.
- No gradient orb, fake testimonial, decorative card grid, or ring spinner.

## Motion

- Agent traces advance only after a user starts or replays a captured call.
- Runtime steps progress at a readable cadence, and expandable command output uses native details.
- Pending states use captured agent shimmer/dot treatments from brainless rather than ring spinners.
- Controls resolve within 100ms and all motion respects reduced-motion preferences.

## Composition

- Hero content is centered and stacked above the full-width interactive playground.
- Each following section keeps the same order: heading, short explanation, then product proof.
- Metrics use tabular numerals. Long technical material belongs in the output and benchmark panels.
- Mobile keeps the primary CTA above the fold and collapses agent chrome without horizontal overflow.

## Performance contract

- Use the latest stable Next.js and React releases; preview/canary releases are not production defaults.
- Keep the route, hero, navigation, preview, proof, and marketing sections as Server Components.
- Hydrate only copy controls and the demo loader on first paint. Load the full brainless playground and
  its utility CSS when the demo enters the viewport.
- Keep a server-rendered demo preview in the initial HTML so deferred JavaScript never creates an empty
  section or layout shift.
- Keep Manrope to one Latin variable WOFF2, preloaded and self-hosted by `next/font`.
- Use inline SVG components instead of shipping a general-purpose icon package.
- Inline the small critical stylesheet in production; keep agent-library utilities in the deferred demo
  chunk. Re-evaluate Next.js `experimental.inlineCss` on every framework upgrade.
- Apply `content-visibility: auto` to substantial below-the-fold sections and preserve an intrinsic size.
- Preserve reduced-motion support and never hide the LCP headline behind an entrance animation.
- Treat horizontal overflow, initial demo hydration, and route build output as release checks.

## Proof checklist

- Render and inspect desktop and mobile.
- Exercise all three agent tabs and copy controls.
- Check overflow, text fit, focus, active, hover, and reduced-motion behavior.
- Keep all visual constants in semantic CSS variables.
