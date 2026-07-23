export type SearchResult = {
  rank: number;
  title: string;
  domain: string;
  url: string;
  snippet: string;
};

export type SearchTrace = {
  id: string;
  label: string;
  query: string;
  latency: string;
  command: string;
  searchUrl: string;
  results: SearchResult[];
};

export const traces: SearchTrace[] = [
  {
    id: "rust-browser",
    label: "Rust browser tools",
    query: "rust browser automation libraries",
    latency: "0.73s",
    command: 'lsearch search "rust browser automation libraries" --limit 3 --pretty',
    searchUrl: "https://html.duckduckgo.com/html/?q=rust+browser+automation+libraries",
    results: [
      {
        rank: 1,
        title: "browser_automation — Rust web dev library // Lib.rs",
        domain: "lib.rs",
        url: "https://lib.rs/crates/browser_automation",
        snippet:
          "A modular, Rust-based browser automation library leveraging the Fantoccini WebDriver client.",
      },
      {
        rank: 2,
        title: "GitHub - BB-fat/browser-use-rs",
        domain: "github.com",
        url: "https://github.com/BB-fat/browser-use-rs",
        snippet:
          "A Rust library for browser automation via Chrome DevTools Protocol with built-in MCP integration.",
      },
      {
        rank: 3,
        title: "GitHub - Skyvern-AI/rustwright",
        domain: "github.com",
        url: "https://github.com/Skyvern-AI/rustwright",
        snippet:
          "Playwright's API on a native Rust engine that speaks raw Chrome DevTools Protocol.",
      },
    ],
  },
  {
    id: "agent-search",
    label: "Search for agents",
    query: "structured web search for AI agents",
    latency: "1.06s",
    command: 'lsearch search "structured web search for AI agents" --limit 3 --pretty',
    searchUrl: "https://html.duckduckgo.com/html/?q=structured+web+search+for+AI+agents",
    results: [
      {
        rank: 1,
        title: "Best AI Search Engines for Agents and Workflows in 2026",
        domain: "firecrawl.dev",
        url: "https://www.firecrawl.dev/blog/best-ai-search-engines-agents",
        snippet:
          "AI search engines for agents return fresh, machine-readable results designed for reasoning loops.",
      },
      {
        rank: 2,
        title: "7 Free Web Search APIs for AI Agents",
        domain: "kdnuggets.com",
        url: "https://www.kdnuggets.com/7-free-web-search-apis-for-ai-agents",
        snippet:
          "A survey of search APIs for agentic AI, including real-time SERP data and RAG workflows.",
      },
      {
        rank: 3,
        title: "Cloudsway Search | Real-time Search for AI Agents",
        domain: "cloudsway.ai",
        url: "https://www.cloudsway.ai/product/search/",
        snippet:
          "Structured web search aimed at grounding agent decisions with current information.",
      },
    ],
  },
  {
    id: "tokio-runtime",
    label: "Docs-only search",
    query: "site:docs.rs tokio Runtime",
    latency: "1.18s",
    command: 'lsearch search "site:docs.rs tokio Runtime" --limit 3 --pretty',
    searchUrl: "https://html.duckduckgo.com/html/?q=site%3Adocs.rs+tokio+Runtime",
    results: [
      {
        rank: 1,
        title: "tokio::runtime - Rust - Docs.rs",
        domain: "docs.rs",
        url: "https://docs.rs/tokio/latest/tokio/runtime/",
        snippet:
          "The Tokio runtime supplies an I/O driver, scheduler, and timer for asynchronous applications.",
      },
      {
        rank: 2,
        title: "Runtime in tokio::runtime - Rust - Docs.rs",
        domain: "docs.rs",
        url: "https://docs.rs/tokio/latest/tokio/runtime/struct.Runtime.html",
        snippet:
          "Runtime provides the I/O driver, task scheduler, timer, and blocking pool.",
      },
      {
        rank: 3,
        title: "tokio - Rust - Docs.rs",
        domain: "docs.rs",
        url: "https://docs.rs/tokio/latest/tokio/",
        snippet:
          "An event-driven, non-blocking platform for writing asynchronous applications in Rust.",
      },
    ],
  },
];

export function traceJson(trace: SearchTrace) {
  return JSON.stringify(
    {
      engine: "duckduckgo",
      ok: true,
      query: trace.query,
      search: {
        blocked: false,
        results: trace.results,
        title: `${trace.query} at DuckDuckGo`,
        url: trace.searchUrl,
      },
    },
    null,
    2,
  );
}
