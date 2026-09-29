# Browser core redesign

Development build, measured September 25, 2026 (America/Los_Angeles).
This is not a published release. The comparison baseline is the installed
crates.io `local-search` 0.2.0 binary, not a rebuilt approximation.

## What changed

| Area | Previous shape | Risk | New interface | Proof |
|---|---|---|---|---|
| Search | Search and cache logic inside the general command dispatcher | Cache hits attach to pages unnecessarily | Dedicated search module; verify the live browser before deciding whether a page is needed | Live-cache integration test requires exactly one `Browser.getVersion` call |
| Readiness | Repeated CDP polling and readiness from whichever document is visible | Stale pages, hidden script errors, extra round trips | New-loader navigation gate and bounded browser-side observer | Mock transport tests and real Chrome delayed-page/history checks |
| Transport | Timeout restarts while events arrive; unbounded retained events | Hung commands and memory growth | Absolute deadlines; newest 4,096 events / 8 MiB | Event-flood, timeout, and queued-recording tests |
| Approved session | Pending IDs outlive canceled commands | Repeated cancellations eventually close the approved connection | Clear ended-lease requests; detach late unowned sessions | 160 canceled leases followed by successful reuse |
| Extraction | Broad challenge regex and URL substring filtering | False CAPTCHA reports and missing document fragments | Valid-result precedence, structural URL checks, UTF-8 redirects | Executable fixtures for all four engines |
| Content reads | Retry the whole result batch three times | Repeated work and hidden genuine failures | Single bounded pass; always close the temporary tab | Live direct/helper content and tab-cleanup checks |

The CLI flags, three executable names, selected-browser rules, and JSON envelopes
are unchanged. No runtime dependencies were added. Normal commands never switch
profiles or reconnect an existing browser implicitly.

## Controlled A/B results

Source: [60-sample report](../benchmarks/results/local-ab-2026-09-25.json).
Each row has 60 measured samples per binary, five excluded warmups, and alternating
A/B order. Both binaries used empty, separate headless Chrome profiles on this
arm64 macOS machine. Each timed command includes process startup.

| Scenario | Published 0.2.0 median | Candidate median | Published p95 | Candidate p95 |
|---|---:|---:|---:|---:|
| Warm CLI startup (`--version`) | 6.64 ms | 6.45 ms | 11.14 ms | 10.97 ms |
| Local browser evaluation | 13.49 ms | 12.82 ms | 30.25 ms | 31.87 ms |
| Read a synthetic localhost page | 23.42 ms | 24.00 ms | 39.60 ms | 37.39 ms |
| Synthetic cache hit, live browser verified | 10.18 ms | 8.69 ms | 14.49 ms | 12.09 ms |

All 480 measured operations passed, as did warmups and owned-browser cleanup.
Cache-hit median latency was 14.6% lower in this run. Simple page-read latency
did not improve. Small startup/evaluation differences are not evidence of a
universal speedup. Output token counts were unchanged for these fixtures.

The candidate binary is 1,272,432 bytes versus 1,255,776 bytes for the baseline
(16,656 bytes larger, about 1.3%). Both binary hashes and the candidate source
hash are in the reports. The final source and binaries stayed unchanged during
the controlled run.

A [second 60-sample run](../benchmarks/results/local-ab-2026-09-25-waits.json)
also measures a delayed DOM update. It selects a visible tab inside the owned
headless browser, requests a 75 ms timer, and measures two CLI calls together:
arm the update, then wait for its selector. Browser timer delivery is not assumed
to be exactly 75 ms. The previous work tab is restored afterward.

| Scenario | Published median | Candidate median | Median change | Published p95 | Candidate p95 |
|---|---:|---:|---:|---:|---:|
| Delayed DOM wait | 130.57 ms | 89.43 ms | 31.5% lower | 147.01 ms | 95.91 ms |
| Synthetic cache hit | 16.55 ms | 13.60 ms | 17.8% lower | 30.38 ms | 26.44 ms |
| Local page read | 26.73 ms | 28.93 ms | 8.2% higher | 48.90 ms | 50.01 ms |
| Local evaluation | 11.62 ms | 11.14 ms | 4.1% lower | 18.55 ms | 17.74 ms |
| Warm CLI startup | 9.52 ms | 9.37 ms | 1.6% lower | 16.59 ms | 18.34 ms |

All 600 measured operations passed. The two runs show a repeatable cache gain
and a faster controlled DOM wait, not faster operations across the board. Page
reads were 2.5% and 8.2% slower respectively, and some tail timings regressed.
Keep that tradeoff visible. These tests do not measure real search-engine latency.

## Real search and comparison limits

- [Google exploratory run](../benchmarks/results/local-ab-2026-09-25-google-blocked.json):
  both binaries received verification pages. This intermediate build produced no
  valid search speed comparison. The runner was then changed to stop public
  requests for a binary after its first detected block.
- [DuckDuckGo run](../benchmarks/results/local-ab-2026-09-25-duckduckgo.json):
  both binaries were blocked on their first warmup. Remaining search attempts and
  cache priming were skipped. Neither block was treated as a fast success.
- Exa, Brave API, Tavily, and Firecrawl were not rerun because their API keys were
  unavailable. No hosted-provider charges were incurred by this work.
- [Brave browser search](../benchmarks/results/local-ab-2026-09-25-brave.json):
  both binaries were blocked on the first warmup; remaining searches were skipped.
- [Bing smoke](../benchmarks/results/local-ab-2026-09-25-bing.json): both binaries
  completed all three measured cold searches and three cache hits. This is a
  small single-query smoke test, not evidence for a universal search speed claim.

The [expanded Bing run](../benchmarks/results/local-ab-2026-09-25-bing-expanded.json)
used 20 measured samples per binary and two excluded warmups, with alternating
order. All 40 uncached searches and 40 cache hits passed. The query was
`rust async cancellation`, depth three. "Uncached" disables lsearch's result
cache; Chrome and the OS remain warm.

| Bing scenario | Published median | Candidate median | Median change | Published p95 | Candidate p95 |
|---|---:|---:|---:|---:|---:|
| Uncached search | 219.60 ms | 175.65 ms | 20.0% lower | 294.07 ms | 328.19 ms |
| Cached search | 23.91 ms | 16.46 ms | 31.1% lower | 64.50 ms | 37.35 ms |

Median stdout tokens were 210 for both builds in both scenarios. The candidate's
uncached p95 was 11.6% higher, so this is a median improvement, not a blanket
latency win. The earlier three-sample smoke had a slower candidate cold median;
both reports are retained. One query on one engine does not establish performance
across providers, query types, result depths, machines, or signed-in profiles.

The hosted runner now separates native response tokens from a common maximum
snippet budget, isolates browser configuration, separates result-count fulfillment
from nonempty success, and counts potentially charged failed attempts. Its pricing
assumptions remain explicitly dated July 21, 2026, not verified current prices.

Existing website/README provider figures remain historical. Do not replace them
with synthetic cache or localhost results, or describe this work as proving the
fastest live search API.

## Verification

- Rust formatting, 87 Rust tests, and all-target/all-feature Clippy passed.
- 15 executable extraction/readiness fixtures passed.
- 16 benchmark runner unit tests passed.
- [13 live browser checks](../benchmarks/results/browser-smoke-2026-09-25.json)
  passed, including temporary content tabs, persistent-helper reuse, navigation,
  timeout/error behavior, and disconnect without closing Chrome.

The live checks used owned, empty Chrome profiles. They did not exercise Chrome's
consent dialog or personal signed-in accounts. No personal cookies, pages,
screenshots, or credentials are in these reports. Every owned test browser and
helper was closed after its run. The user's existing browser was untouched.

See [benchmark instructions](../benchmarks/README.md) to reproduce the checks.
Managed/direct commands still share their selected work tab; this change does
not add parallel independent-page execution to that mode. The approved helper
continues to serialize command leases.
