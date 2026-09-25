import type { MetadataRoute } from "next";
import { SITE_LAST_UPDATED, SITE_URL } from "@/lib/seo";

export const dynamic = "force-static";

export default function sitemap(): MetadataRoute.Sitemap {
  return [
    {
      url: SITE_URL,
      lastModified: new Date(SITE_LAST_UPDATED),
      changeFrequency: "weekly",
      priority: 1,
    },
    {
      url: `${SITE_URL}/docs`,
      lastModified: new Date("2026-09-25"),
      changeFrequency: "weekly",
      priority: 0.8,
    },
  ];
}
