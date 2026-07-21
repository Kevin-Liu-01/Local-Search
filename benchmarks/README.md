# Hosted Search Benchmark

This benchmark runs the same 12 queries at 3-result and 10-result depths through
`lsearch`, Exa, Brave Search, Tavily, and Firecrawl. It measures:

- serialized request plus raw response tokens;
- the same responses normalized to `rank`, `title`, `url`, and `snippet`;
- latency, result count, success rate, reported usage, and estimated cost.

The runner gives `lsearch` a new temporary cache directory on every run. The
three-result pass is cold; the ten-result pass repeats the queries and measures
the CLI's normal five-minute local cache. Hosted requests are still made for
both depths because each request consumes provider usage.

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

Run the full comparison:

```sh
python3 benchmarks/hosted_search.py
```

Run selected providers or a one-query smoke test:

```sh
python3 benchmarks/hosted_search.py --providers lsearch,brave,tavily
python3 benchmarks/hosted_search.py --max-queries 1
```

At pricing published on 2026-07-21, the full 24-request run per provider uses an
estimated $0.324 of Exa Search plus highlights, $0.120 of Brave Web Search, 24
Tavily basic-search credits ($0.192 at pay-as-you-go pricing), and 48 Firecrawl
search credits. Free or prepaid plan credits can reduce the marginal charge.
