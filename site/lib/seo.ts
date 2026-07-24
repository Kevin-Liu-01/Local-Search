const configuredSiteUrl = process.env.NEXT_PUBLIC_SITE_URL
  ?? (process.env.VERCEL_PROJECT_PRODUCTION_URL
    ? `https://${process.env.VERCEL_PROJECT_PRODUCTION_URL}`
    : "http://localhost:3500");

export const SITE_URL = configuredSiteUrl.replace(/\/$/, "");
export const SITE_NAME = "local-search";
export const SITE_TITLE = "local-search: Free Browser Search API for AI Agents";
export const SITE_DESCRIPTION = "Give Claude Code, Codex, Cursor, and other coding agents structured browser search as JSON—locally, with no API key, hosted account, or search bill today.";
export const SOCIAL_TITLE = "Browser Search API. No API Key. No Billing.";
export const SOCIAL_DESCRIPTION = "Structured Google, Bing, and DuckDuckGo results for Claude Code, Codex, Cursor, and any shell agent. Local JSON, with no API key or metered bill.";

export const REPOSITORY_URL = "https://github.com/Kevin-Liu-01/Local-Search";
export const CRATE_URL = "https://crates.io/crates/local-search";
export const AUTHOR_URL = "https://www.kevin-liu.tech/";
export const AUTHOR_GITHUB_URL = "https://github.com/Kevin-Liu-01";
export const AUTHOR_X_URL = "https://x.com/kevskgs";

export const FAQS = [
  {
    question: "What is local-search?",
    answer: "local-search is an open-source Rust CLI that turns a local Chrome or Chromium browser into a structured search interface for AI coding agents. The lsearch command searches the web and returns stable JSON containing ranked titles, URLs, domains, snippets, and optional page content.",
  },
  {
    question: "Does local-search need a search API key?",
    answer: "No. local-search uses the browser already running on your machine, so there is no hosted search account, metered search plan, or paid API key. Install it from crates.io and run searches from the shell.",
  },
  {
    question: "Which coding agents can use local-search?",
    answer: "Claude Code, OpenAI Codex, Cursor, and any other shell-capable coding agent can use local-search. The integration is one CLI command rather than an agent-specific SDK, and the result is returned through standard output as JSON.",
  },
  {
    question: "Which search engines does local-search support?",
    answer: "local-search supports Google, Bing, and DuckDuckGo. Searches run through a managed local browser profile, so results can reflect the region and signed-in browser state that you control.",
  },
  {
    question: "How does local-search compare with hosted search APIs?",
    answer: "In the July 21, 2026 matched-provider benchmark, local-search fulfilled 24 of 24 requested result depths, used 53.4 normalized tokens per result, recorded 148.7 milliseconds median latency, and consumed $0 in hosted API credits. The full benchmark runner and methodology are public in the repository.",
  },
  {
    question: "Does local-search send browser credentials to a hosted service?",
    answer: "local-search does not require a local-search cloud service or hosted account. It controls a browser profile on your machine, sends each query to the selected public search engine, and returns the structured result to the calling agent through standard output.",
  },
] as const;

export const KEYWORDS = [
  "browser search API",
  "local search API",
  "AI agent web search",
  "coding agent search",
  "Claude Code web search",
  "OpenAI Codex web search",
  "Cursor web search",
  "Rust search CLI",
  "structured search JSON",
  "Chrome search automation",
  "Google search CLI",
  "Bing search CLI",
  "DuckDuckGo search CLI",
  "search API without API key",
];

export const STRUCTURED_DATA = {
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "Person",
      "@id": `${SITE_URL}/#author`,
      name: "Kevin Liu",
      url: AUTHOR_URL,
      sameAs: [AUTHOR_GITHUB_URL, AUTHOR_X_URL],
      knowsAbout: ["Rust", "browser automation", "AI coding agents", "structured web search"],
    },
    {
      "@type": "WebSite",
      "@id": `${SITE_URL}/#website`,
      url: SITE_URL,
      name: SITE_NAME,
      alternateName: "lsearch",
      description: SITE_DESCRIPTION,
      inLanguage: "en-US",
      creator: { "@id": `${SITE_URL}/#author` },
    },
    {
      "@type": "WebPage",
      "@id": `${SITE_URL}/#webpage`,
      url: SITE_URL,
      name: SITE_TITLE,
      description: SITE_DESCRIPTION,
      isPartOf: { "@id": `${SITE_URL}/#website` },
      mainEntity: { "@id": `${SITE_URL}/#software` },
      about: [
        { "@type": "Thing", name: "Browser search API" },
        { "@type": "Thing", name: "AI coding agents" },
        { "@type": "Thing", name: "Structured web search" },
      ],
      author: { "@id": `${SITE_URL}/#author` },
      dateModified: "2026-07-23",
      inLanguage: "en-US",
    },
    {
      "@type": ["SoftwareApplication", "SoftwareSourceCode"],
      "@id": `${SITE_URL}/#software`,
      name: SITE_NAME,
      alternateName: ["lsearch", "local-search CLI"],
      description: SITE_DESCRIPTION,
      applicationCategory: "DeveloperApplication",
      applicationSubCategory: "Browser search CLI for AI agents",
      softwareVersion: "0.1.1",
      programmingLanguage: "Rust",
      runtimePlatform: "Chrome or Chromium",
      codeRepository: REPOSITORY_URL,
      downloadUrl: CRATE_URL,
      license: "https://opensource.org/license/mit",
      isAccessibleForFree: true,
      offers: {
        "@type": "Offer",
        price: "0",
        priceCurrency: "USD",
        availability: "https://schema.org/InStock",
        url: CRATE_URL,
      },
      featureList: [
        "Structured Google, Bing, and DuckDuckGo search results",
        "Stable JSON output for coding agents",
        "Managed local Chrome profile",
        "Optional page content extraction",
        "No hosted search API key or metered search bill",
      ],
      author: { "@id": `${SITE_URL}/#author` },
      sameAs: [REPOSITORY_URL, CRATE_URL],
    },
    {
      "@type": "FAQPage",
      "@id": `${SITE_URL}/#faq`,
      url: `${SITE_URL}/#faq`,
      mainEntity: FAQS.map(({ question, answer }) => ({
        "@type": "Question",
        name: question,
        acceptedAnswer: {
          "@type": "Answer",
          text: answer,
        },
      })),
    },
  ],
};
