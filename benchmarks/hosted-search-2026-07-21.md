# Hosted Search Results: 2026-07-21

The live benchmark ran 12 queries at 3-result and 10-result depths through
`lsearch`, Exa, Brave Search, Tavily, and Firecrawl. Calls were sequential. All
tokens use `o200k_base` and include the serialized request shape plus response.
The `lsearch` runner used a fresh temporary cache: its first 12 requests were
cold and its repeated ten-result requests were cache hits.

Responses were also normalized to `rank`, `title`, `url`, and `snippet`:

- `lsearch`: search snippets
- Exa: highlights
- Brave Search: descriptions
- Tavily: basic-search content
- Firecrawl: descriptions without `scrapeOptions`

| Provider | Runs | Successful | Depth fulfilled | Median raw tokens | Median normalized tokens/result | Median latency | Full-run usage |
|---|---:|---:|---:|---:|---:|---:|---:|
| **`lsearch`** | **24** | **24** | **24** | **413.5** | **53.4** | **148.7 ms** | **$0** |
| Exa | 24 | 24 | 24 | 4,472.5 | 881.2 | 501.9 ms | $0.324 |
| Brave Search | 24 | 24 | 24 | 13,104 | 108.6 | 322.0 ms | $0.120 |
| Tavily | 24 | 24 | 17 | 1,163 | 259.5 | 1,184.4 ms | 24 credits ($0.192 PAYG) |
| Firecrawl | 24 | 24 | 24 | 509 | 74.8 | 1,520.8 ms | 48 credits |

`lsearch` cold searches had a 384.5 ms median. Repeated depth requests had a
6.5 ms median. All 24 requests fulfilled their requested result count.

## Published Pricing Used

- [Exa](https://exa.ai/pricing): $0.007/search up to 10 results plus
  $0.001/highlighted page.
- [Brave Search](https://api-dashboard.search.brave.com/documentation/pricing):
  $0.005/search request.
- [Tavily](https://docs.tavily.com/documentation/api-credits): one credit/basic
  search and $0.008/credit at pay-as-you-go pricing.
- [Firecrawl](https://www.firecrawl.dev/pricing): two search credits per 10
  results, rounded up.

Re-run with [`hosted_search.py`](hosted_search.py).
