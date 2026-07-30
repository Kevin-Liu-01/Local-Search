import type { MetadataRoute } from "next";
import { SITE_URL } from "@/lib/seo";

export const dynamic = "force-static";

const aiCrawlers = [
  "GPTBot",
  "ChatGPT-User",
  "OAI-SearchBot",
  "ClaudeBot",
  "Claude-Web",
  "anthropic-ai",
  "PerplexityBot",
  "Perplexity-User",
  "Google-Extended",
  "GoogleOther",
  "Amazonbot",
  "Applebot",
  "Applebot-Extended",
  "Meta-ExternalAgent",
  "Meta-ExternalFetcher",
  "FacebookBot",
  "CCBot",
  "Bytespider",
  "cohere-ai",
  "Diffbot",
  "ImagesiftBot",
  "YouBot",
  "Phind",
  "DuckAssistBot",
];

export default function robots(): MetadataRoute.Robots {
  return {
    rules: [
      {
        userAgent: "Twitterbot",
        allow: ["/", "/api/og", "/api/og-home", "/opengraph-image", "/twitter-image"],
      },
      {
        userAgent: "*",
        allow: "/",
      },
      {
        userAgent: aiCrawlers,
        allow: ["/", "/llms.txt", "/llms-full.txt", "/benchmarks.json"],
      },
    ],
    sitemap: `${SITE_URL}/sitemap.xml`,
    host: SITE_URL,
  };
}
