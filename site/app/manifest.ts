import type { MetadataRoute } from "next";

export const dynamic = "force-static";

export default function manifest(): MetadataRoute.Manifest {
  return {
    name: "local-search: A Local Browser API for Agents",
    short_name: "local-search",
    description: "Let agents search, read, extract, interact, and use signed-in sites through a local Chrome profile.",
    start_url: "/",
    id: "/",
    scope: "/",
    display: "standalone",
    background_color: "#f4f4f0",
    theme_color: "#111210",
    categories: ["developer tools", "productivity", "utilities"],
    icons: [
      {
        src: "/icon.svg",
        sizes: "any",
        type: "image/svg+xml",
      },
    ],
  };
}
