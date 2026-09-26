# Agent guide evidence

## Signed-in checks: September 25, 2026

Tested the current local source build on macOS, using existing Chrome and the
persistent approved helper. That test binary reported `0.1.4`; this evidence is
for the pre-release source, **not a claim about the published 0.1.4 package**.
The persistent-session feature targets the 0.2.0 release. These site checks
have not been rerun against the published 0.2.0 package.

| Site | Page | Observed result |
| --- | --- | --- |
| LinkedIn | `/feed/` | Redirected within the feed to `/feed/foryou/`; signed-in navigation present, no login form or challenge. `read` returned 4,574 characters. |
| GitHub | `/settings/profile` | Title “Your profile”, `logged-in` body class, no login form. An unauthenticated `curl` request redirected to `/login`. |
| Reddit | `/settings/` | Title “Settings - Account”, account email/delete labels present, no login redirect or blocked-page marker. `read` returned 529 characters. |

One already-approved existing-Chrome session handled these commands. The test
created and closed its own background tab. It did not change settings, post,
message anyone, copy cookies, or output page contents. The user kept the
connection afterward. Character counts describe that test, not typical page sizes.

The prior persistent-session test also confirmed cookie reuse with a synthetic
localhost cookie, public Google search, explicit disconnect, and failure without
automatic reconnection. A mock transport test measured three commands using one
upstream browser connection. See `tests/connection.rs` for automated coverage.

## What each visual represents

The synthetic fixture was also exercised with the installed local `lsearch`:
`extract` returned both titles and absolute link URLs; `snapshot` identified the
filter input as `@e1`; `fill @e1 docs` filtered the visible page; `read` returned
the remaining task; `request` returned HTTP 200; and `screenshot` wrote a PNG.
The test closed its own tab and did not change the saved browser selection.

| Image | Evidence type |
| --- | --- |
| `01-agent-browser.png` | Architecture illustration using documented commands; no runtime timing claim. |
| `02-browser-choice.png` | Setup illustration of existing and managed modes. |
| `03-signed-in-sites.png` | Privacy-safe summary of the live checks above; not account-page screenshots. |
| `04-search.png` | One shortened result from `site/lib/traces.ts`, recorded July 21, 2026; shown as a result fragment, not a complete response envelope. |
| `05-extract.png` | Synthetic cards from `fixtures/tasks.html`; selected returned fields omit the index and shorten absolute URLs for legibility. |
| `06-work-with-pages.png` | Documented command workflow, with a synthetic form and an explicitly illustrative API URL. |
| `07-control.png` | Disconnect response shape and stable error semantics from the current source. |

The PNG files are browser captures of `visual-guide.html`. They contain no
private browser screenshots or raw authenticated responses. Brand marks reuse
the site's collected assets; provenance is in `site/public/brand/SOURCES.md`.

## Reproduce the visuals

1. Have Node.js and the `agent-browser` CLI installed, with its browser available.
2. Build the site once (`cd site && pnpm build`) so the capture can reuse the
   locally bundled Manrope font. The script refuses to silently substitute fonts.
3. Run `node scripts/capture-agent-guide.mjs` from the repository root.

The script serves only the guide, fixture, local brand assets, and font on an
ephemeral loopback port. It uses a fresh, isolated `agent-browser` session,
checks desktop/mobile overflow and image loading, captures the seven frames,
then closes its browser and server. It never connects to your everyday Chrome.

For local inspection, `node scripts/capture-agent-guide.mjs --serve` prints a
preview URL. Stop it with Ctrl+C when finished.

## Release verification

The 0.2.0 release checklist and package-install evidence are tracked in
[releases/0.2.0.md](releases/0.2.0.md). The signed-in checks above remain
dated source-build evidence, separate from package installation checks.

### Published packages

Cargo `local-search` 0.2.0 and npm `@kevinliu01/localsearch` 0.2.0 are published.
The native crate was published first; the npm wrapper pins it exactly. Fresh
isolated installations of both distributions passed version, alias, and
fail-closed output checks. The Cargo install also passed a synthetic managed
browser read and graceful cleanup test. The guides now identify 0.2.0 as the
minimum version for persistent existing-Chrome sessions.

The three signed-in sites were not rechecked for publication. Recheck them with
permission before claiming newly recorded results. Site-specific access,
selectors, and account permissions can change. These checks do not establish
universal site compatibility. No external social posts were made.
