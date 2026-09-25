const configuredSiteUrl = process.env.NEXT_PUBLIC_SITE_URL ?? "https://www.lsearch.dev";

export const SITE_URL = configuredSiteUrl.replace(/\/$/, "");
export const SITE_LAST_UPDATED = "2026-09-24";
export const SITE_NAME = "local-search";
export const SITE_TITLE = "local-search: A Local Browser API for AI Agents";
export const SITE_DESCRIPTION = "Let your agent search, read pages, and use signed-in sites through your browser. Choose existing Chrome with approval or a separate profile. No cookie export.";
export const SOCIAL_TITLE = "A Local Browser API for Agents";
export const SOCIAL_DESCRIPTION = "Let your agent search, read pages, and use signed-in sites through your local browser. You choose the profile.";
export const SOCIAL_IMAGE_ALT = "local-search connects AI agents to a browser on your machine for web search, reading pages, and using signed-in sites.";

export const REPOSITORY_URL = "https://github.com/Kevin-Liu-01/Local-Search";
export const CRATE_URL = "https://crates.io/crates/local-search";
export const NPM_URL = "https://www.npmjs.com/package/@kevinliu01/localsearch";
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
    answer: "A command-line tool that lets agents search the web, read pages, extract data, and use websites through Chrome or Chromium on your machine.",
  },
  {
    question: "Can it use my logins?",
    answer: "Yes. On macOS/Linux, approve one existing-Chrome connection and reuse it across commands. Run lsearch disconnect to end access. Reconnecting requires approval again. Or sign in once in a separate persistent profile.",
  },
  {
    question: "Which agents work with it?",
    answer: "Claude Code, Codex, Cursor, OpenClaw, or any agent that can run a shell command. Results come back as compact JSON.",
  },
  {
    question: "Which search engines can I use?",
    answer: "Google, Bing, Brave, or DuckDuckGo. Choose with --engine. All return the same JSON fields: rank, title, URL, domain, and snippet.",
  },
  {
    question: "Does it bypass logins or CAPTCHAs?",
    answer: "No. Your account still needs access. If a site blocks a search or asks for verification, local-search reports it.",
  },
  {
    question: "Where does my data go?",
    answer: "Your browser sends cookies to the sites you visit. local-search has no hosted service. Page content goes to your agent and may reach its model provider. Only grant access to accounts you trust it with.",
  },
  {
    question: "How small and fast is it?",
    answer: `Our July 27, 2026 arm64 macOS test measured a ${RELEASE_AUDIT.binarySize} binary and ${RELEASE_AUDIT.startupMedianMs.toFixed(2)} ms median warm startup over ${RELEASE_AUDIT.startupSamples} launches. This measures CLI startup, not search speed. Results vary by build and machine.`,
  },
  {
    question: "Is it free and open source?",
    answer: "Yes. MIT licensed, with no local-search subscription or per-search fee. You need a local browser and internet access. Each site’s terms still apply.",
  },
] as const;

export const KEYWORDS = [
  "browser search API",
  "browser API for agents",
  "local browser API",
  "authenticated browser automation",
  "AI agent browser access",
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
        { "@type": "Thing", name: "Local browser API" },
        { "@type": "Thing", name: "AI coding agents" },
        { "@type": "Thing", name: "Structured web search" },
      ],
      author: { "@id": `${SITE_URL}/#author` },
      dateModified: SITE_LAST_UPDATED,
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
      applicationSubCategory: "Local browser API for AI agents",
      softwareVersion: "0.1.4",
      operatingSystem: "macOS, Linux, and Windows with Chrome or Chromium",
      runtimePlatform: "Chrome or Chromium",
      downloadUrl: [CRATE_URL, NPM_URL],
      installUrl: [CRATE_URL, NPM_URL],
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
        "One local browser interface for shell-capable agents",
        "Readable page extraction and structured record extraction",
        "Browser-authenticated requests through the local Chrome profile you choose",
        "One local CLI across four public search engines",
        "Structured Google, Bing, DuckDuckGo, and Brave Search results",
        "Stable JSON output for coding agents",
        "Existing Chrome with approval or a separate persistent local profile",
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
      sameAs: [REPOSITORY_URL, CRATE_URL, NPM_URL],
      subjectOf: { "@id": `${SITE_URL}/#benchmarks` },
    },
    {
      "@type": "SoftwareSourceCode",
      "@id": `${SITE_URL}/#source`,
      name: "local-search source code",
      description: "MIT-licensed Rust source code for the local-search browser API for agents.",
      codeRepository: REPOSITORY_URL,
      programmingLanguage: "Rust",
      runtimePlatform: "Chrome or Chromium",
      version: "0.1.4",
      dateModified: SITE_LAST_UPDATED,
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
