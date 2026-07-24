import type { MetadataRoute } from "next";

export const dynamic = "force-static";

export default function manifest(): MetadataRoute.Manifest {
  return {
    name: "local-search — Browser Search API for AI Agents",
    short_name: "local-search",
    description: "Structured web search through your local browser, with no hosted search API key or metered bill.",
    start_url: "/",
    display: "standalone",
    background_color: "#f4f4f0",
    theme_color: "#111210",
    icons: [
      {
        src: "/icon.svg",
        sizes: "any",
        type: "image/svg+xml",
      },
    ],
  };
}
