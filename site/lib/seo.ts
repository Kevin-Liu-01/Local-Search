const configuredSiteUrl = process.env.NEXT_PUBLIC_SITE_URL
  ?? (process.env.VERCEL_PROJECT_PRODUCTION_URL
    ? `https://${process.env.VERCEL_PROJECT_PRODUCTION_URL}`
    : "https://local-search-xi.vercel.app");

export const SITE_URL = configuredSiteUrl.replace(/\/$/, "");
export const SITE_NAME = "local-search";
export const SITE_TITLE = "local-search: Free Browser Search API for AI Agents";
export const SITE_DESCRIPTION = "Open-source Rust CLI for structured Google, Bing, DuckDuckGo, and Brave Search through your local browser. Built for coding agents, with no API key or search bill.";
export const SOCIAL_TITLE = "Browser Search API. 1.06 MB. No API Key.";
export const SOCIAL_DESCRIPTION = "A tiny Rust CLI that returns structured Google, Bing, DuckDuckGo, and Brave Search results to coding agents through your local browser.";

export const REPOSITORY_URL = "https://github.com/Kevin-Liu-01/Local-Search";
export const CRATE_URL = "https://crates.io/crates/local-search";
export const AUTHOR_URL = "https://www.kevin-liu.tech/";
export const AUTHOR_GITHUB_URL = "https://github.com/Kevin-Liu-01";
export const AUTHOR_X_URL = "https://x.com/kevskgs";

export const RELEASE_AUDIT = {
  date: "2026-07-27",
  platform: "arm64 macOS (aarch64-apple-darwin)",
  rustVersion: "1.97.1",
  binaryBytes: 1_055_072,
  binarySize: "1.06 MB",
  baselineBinaryBytes: 4_623_984,
  sizeReduction: "77.2%",
  startupMedianMs: 4.61,
  baselineStartupMedianMs: 7.20,
  startupReduction: "36%",
  startupSamples: 200,
} as const;

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
    answer: "Claude Code, OpenAI Codex, Cursor, OpenClaw, and any other shell-capable coding agent can use local-search. The integration is one CLI command rather than an agent-specific SDK, and the result is returned through standard output as JSON.",
  },
  {
    question: "Which search engines does local-search support?",
    answer: "local-search supports Google, Bing, DuckDuckGo, and Brave Search. Searches run through a managed local browser profile, so results can reflect the region and signed-in browser state that you control.",
  },
  {
    question: "How does local-search compare with hosted search APIs?",
    answer: "In the July 21, 2026 matched-provider benchmark, local-search fulfilled 24 of 24 requested result depths, used 53.4 normalized tokens per result, recorded 148.7 milliseconds median latency, and consumed $0 in hosted API credits. The full benchmark runner and methodology are public in the repository.",
  },
  {
    question: "Does local-search send browser credentials to a hosted service?",
    answer: "local-search does not require a local-search cloud service or hosted account. It controls a browser profile on your machine, sends each query to the selected public search engine, and returns the structured result to the calling agent through standard output.",
  },
  {
    question: "How small and fast is the local-search Rust binary?",
    answer: `The compressed local-search 0.1.2 crates.io package is 54.3 KiB. In the July 27, 2026 arm64 macOS release audit, the compiled lsearch executable measured ${RELEASE_AUDIT.binaryBytes.toLocaleString("en-US")} bytes (${RELEASE_AUDIT.binarySize}), down from ${RELEASE_AUDIT.baselineBinaryBytes.toLocaleString("en-US")} bytes. Median warm CLI process startup fell from ${RELEASE_AUDIT.baselineStartupMedianMs.toFixed(2)} to ${RELEASE_AUDIT.startupMedianMs.toFixed(2)} milliseconds across ${RELEASE_AUDIT.startupSamples} alternating launches. These figures describe that machine and build, not search latency.`,
  },
  {
    question: "Is local-search free and open source?",
    answer: "Yes. local-search is MIT-licensed open-source software published on GitHub and crates.io. It does not require a local-search subscription, hosted search account, or paid API dependency. Queries still use the selected public search engine through the browser on your machine.",
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
  "Brave Search CLI",
  "search API without API key",
  "search API alternatives",
  "local browser search",
  "tiny Rust CLI",
  "no API key web search",
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
      dateModified: RELEASE_AUDIT.date,
      inLanguage: "en-US",
      hasPart: [
        { "@id": `${SITE_URL}/#benchmarks` },
        { "@id": `${SITE_URL}/#faq` },
      ],
    },
    {
      "@type": "SoftwareApplication",
      "@id": `${SITE_URL}/#software`,
      name: SITE_NAME,
      alternateName: ["lsearch", "local-search CLI"],
      description: SITE_DESCRIPTION,
      applicationCategory: "DeveloperApplication",
      applicationSubCategory: "Browser search CLI for AI agents",
      softwareVersion: "0.1.1",
      operatingSystem: "macOS, Linux, and Windows with Chrome or Chromium",
      runtimePlatform: "Chrome or Chromium",
      downloadUrl: CRATE_URL,
      installUrl: CRATE_URL,
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
        "Structured Google, Bing, DuckDuckGo, and Brave Search results",
        "Stable JSON output for coding agents",
        "Managed local Chrome profile",
        "Optional page content extraction",
        "No hosted search API key or metered search bill",
        "1.06 MB arm64 macOS release binary in the July 27, 2026 audit",
      ],
      additionalProperty: [
        {
          "@type": "PropertyValue",
          name: "arm64 macOS release binary size",
          value: RELEASE_AUDIT.binaryBytes,
          unitText: "bytes",
          measurementTechnique: "Cargo release build with fat LTO, opt-level z, symbol stripping, panic abort, and one codegen unit",
        },
        {
          "@type": "PropertyValue",
          name: "Median warm CLI process startup",
          value: RELEASE_AUDIT.startupMedianMs,
          unitText: "milliseconds",
          measurementTechnique: `${RELEASE_AUDIT.startupSamples} alternating warm lsearch --version process launches on ${RELEASE_AUDIT.platform}`,
        },
      ],
      author: { "@id": `${SITE_URL}/#author` },
      sameAs: [REPOSITORY_URL, CRATE_URL],
      subjectOf: { "@id": `${SITE_URL}/#benchmarks` },
    },
    {
      "@type": "SoftwareSourceCode",
      "@id": `${SITE_URL}/#source`,
      name: "local-search source code",
      description: "MIT-licensed Rust source code for the local-search browser search CLI.",
      codeRepository: REPOSITORY_URL,
      programmingLanguage: "Rust",
      runtimePlatform: "Chrome or Chromium",
      version: "0.1.1",
      license: "https://opensource.org/license/mit",
      author: { "@id": `${SITE_URL}/#author` },
      targetProduct: { "@id": `${SITE_URL}/#software` },
    },
    {
      "@type": "Dataset",
      "@id": `${SITE_URL}/#benchmarks`,
      name: "local-search benchmark and native release audit",
      description: "Recorded token, reliability, latency, cost, release-binary size, and CLI startup measurements for local-search.",
      url: `${SITE_URL}/#benchmarks`,
      dateModified: RELEASE_AUDIT.date,
      creator: { "@id": `${SITE_URL}/#author` },
      license: "https://opensource.org/license/mit",
      measurementTechnique: "Public Rust benchmark runners plus alternating warm process-launch timing; qualifications and environment are included with each measurement.",
      variableMeasured: [
        "visible agent context tokens",
        "requested search depth fulfilled",
        "median search latency",
        "hosted API usage",
        "release binary bytes",
        "warm CLI process startup milliseconds",
      ],
      distribution: {
        "@type": "DataDownload",
        encodingFormat: "application/json",
        contentUrl: `${SITE_URL}/benchmarks.json`,
      },
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
