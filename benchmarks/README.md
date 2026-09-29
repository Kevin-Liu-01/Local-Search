# Benchmarks

## Baseline versus candidate

`local_ab.py` measures two release binaries on the same machine. The order
alternates A/B then B/A for each repetition. Warmups, failures, and successful
samples remain separate. Reports contain binary hashes/sizes, environment,
individual timings, medians, and linearly interpolated p95. They never contain
page content, cookies, or raw errors.

Source and binary fingerprints are captured before setup and checked again at
the end. `changed_during_run: true` invalidates a stable-build comparison and
returns a nonzero exit. Do not rebuild a measured binary or edit Rust sources
during a run. Stable stderr error codes are retained, but error messages are
discarded.

```sh
cargo build --release --locked --bin lsearch
python3 benchmarks/local_ab.py \
  --baseline /absolute/path/to/previous/lsearch \
  --candidate target/release/lsearch \
  --samples 30 --warmups 3 \
  --output benchmarks/results/local-overhead.json
```

Add `--live` for reads and JavaScript evaluation against a synthetic localhost
HTML fixture in real headless Chrome. This is browser overhead, not live-web
latency or search quality. The runner launches one empty temporary managed
profile per binary, uses separate config/cache directories and free loopback
ports, removes inherited browser overrides, and gracefully closes only those
owned instances. It does not connect to your everyday Chrome. `--browser-path`
can specify the Chrome executable.

The `delayed_dom_wait` scenario measures two CLI calls together: `eval` cancels
the previous timer, removes the old marker, and requests a ready element after
75 ms; `wait --selector` then waits for that element. The browser may delay its
timer, so 75 ms is a request, not an asserted elapsed time. Both process launches and
connections are included in the total. The runner validates `armed` and a true
`result.ok` wait result, and records each step's duration without storing browser content.
Before this scenario, it selects the exact initial fixture tab in its owned
headless browser and verifies `document.visibilityState` is `visible`, avoiding
background-tab timer throttling. It restores the earlier work tab afterward.
This is a controlled asynchronous-DOM fixture to compare waiter overhead, not
a claim that real sites load in 75 ms. No runner sleep substitutes for readiness.

`--live` also runs `synthetic_cache_hit`: a known three-result fixture is seeded
in each private cache using that browser's verified WebSocket identity. The CLI
must still verify its live browser connection. The fixture must return unchanged
and the cache must not be rewritten. This isolates cached-command overhead when
public engines block the machine; it is **not** a live search, scraped data, or
a website speed claim. The fixture cache format is explicit in the runner and
must be updated if the CLI's cache format changes.

Add `--search --engine google` for public-network search samples. This implies
`--live`. `--query` defaults to `rust async cancellation`. Cold searches always
use `--no-cache`: the browser and OS remain warm. Cached searches use the same
query/depth after explicit priming. Setup or priming failures are failures, not
fast samples. Keep cold and cached medians separate; do not combine them into
one speed claim. The default search sample count is the same `--samples` value,
so begin with a small smoke run. After the first blocked search for a binary,
the runner skips its remaining public searches and does not prime its public
cache. Skipped slots are separate from attempted failures. Warmups have their
own summary, so a block during warmup stays visible. The runner never retries
blocked searches or bypasses verification.

The runner needs only Python's standard library. If `tiktoken` is installed,
it also records stdout tokens with `o200k_base`; otherwise it explicitly marks
tokens unavailable. These stdout-only tokens are not the hosted runner's
request-plus-normalized-response metric. Binary startup with `--version` is
warm process overhead, not a cold filesystem/OS boot.

Offline runner tests:

```sh
python3 -m unittest discover -s benchmarks -p 'test_*.py'
```

Browser timing is machine-specific and noisy. Preserve the JSON report and use
multiple repetitions; do not claim universal improvements from one fixture or
one query. A nonzero runner exit means one or more setup, warmup, measured, or
cleanup operations failed. Inspect those outcomes before citing speedups.

## Browser correctness and extraction fixtures

Run the shipped extraction and readiness JavaScript against deterministic DOM
fixtures (Node.js required, no npm packages):

```sh
python3 -m unittest discover -s benchmarks/tests -v
```

Run live navigation, waits, content extraction, and persistent-helper checks:

```sh
python3 benchmarks/browser_smoke.py --binary target/release/lsearch \
  --output artifacts/browser-smoke.json
```

This launches and closes its own empty Chrome profile. It never uses personal
cookies or a saved browser selection. It exercises the helper over traditional
local CDP, not Chrome's consent UI or signed-in accounts. Only sanitized check
names, pass/fail, timings, and build/browser versions appear in the report.

## Hosted search comparison

This benchmark runs the same 12 queries at 3-result and 10-result depths through
`lsearch`, Exa, Brave Search, Tavily, and Firecrawl. It measures:

- serialized request plus raw response tokens;
- the same responses normalized to `rank`, `title`, `url`, and `snippet`;
- latency, result count, success rate, reported usage, and estimated cost.

Native response token counts retain each provider's content. The separate
`equal_budget_*` metrics use the same `{query,limit}` request and the same result
fields, truncating every snippet to at most 120 characters (`--snippet-chars`
can lower that cap). Native request objects are counted once, not nested inside
another copy of query/depth. Equal maximum snippet budgets still do not measure
ranking quality or equalize the actual text each provider returns. Historical
normalized numbers used different snippet budgets and should not be presented
as an equal-content comparison.

The current runner disables local search caching on every request. It requires
an explicit `--config-dir` for an already connected, isolated profile, so it
cannot silently use the user's saved browser. Hosted requests are made for both
depths and consume provider usage. Provider order is sequential, not randomized.
The historical July 21 report used a different mixed-cache method; do not treat
its headline latency as directly comparable with a current uncached run. The old
README's claim that the 10-result pass necessarily hit a 3-result cache was
incorrect: cache reuse requires sufficient stored result depth.

Tokens use `o200k_base`. API keys stay in `.env.bench.local`, which is ignored by
git. The runner never prints keys or response bodies.

```sh
python3 -m pip install tiktoken
cargo build --release
```

Create `.env.bench.local`:

```dotenv
EXA_API_KEY=
BRAVE_SEARCH_API_KEY=
TAVILY_API_KEY=
FIRECRAWL_API_KEY=
```

Prepare an isolated browser explicitly, then run the full comparison (the hosted
providers make paid API calls when credentials are present):

```sh
export LOCAL_SEARCH_CONFIG_DIR="$(mktemp -d)"
export LOCAL_SEARCH_CACHE_DIR="$(mktemp -d)"
target/release/lsearch launch --headless --port 19322
python3 benchmarks/hosted_search.py --binary target/release/lsearch \
  --engine google --config-dir "$LOCAL_SEARCH_CONFIG_DIR" \
  --output benchmarks/results/hosted-search-current.json --include-rows
target/release/lsearch cleanup --kill --port 19322
```

Run selected providers or a one-query smoke test:

```sh
python3 benchmarks/hosted_search.py --providers lsearch,brave,tavily --config-dir "$LOCAL_SEARCH_CONFIG_DIR"
python3 benchmarks/hosted_search.py --providers brave --max-queries 1
```

At pricing published on 2026-07-21, the full 24-request run per provider uses an
estimated $0.324 of Exa Search plus highlights, $0.120 of Brave Web Search, 24
Tavily basic-search credits ($0.192 at pay-as-you-go pricing), and 48 Firecrawl
search credits (about $0.154 at the $16-per-5,000-credit Hobby-plan rate).
Firecrawl does not offer pay-as-you-go pricing, so this is a plan-equivalent
allocation rather than a marginal charge. Free or prepaid plan credits can
reduce the marginal charge.

Pricing remains explicitly dated July 21, 2026, even when the recording date
defaults to today. `--date YYYY-MM-DD` sets the recording date, not the pricing
date. Estimates are historical, not current price quotes or invoices. Failed
requests may still cost money. Missing credentials and blocked/empty/malformed
results produce a nonzero exit. Reports include only normalized metrics and
sanitized error categories, never provider error bodies or credentials.
