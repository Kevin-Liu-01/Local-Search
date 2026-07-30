import type { Metadata } from "next";
import { Manrope } from "next/font/google";
import { JsonLd } from "@/components/json-ld";
import {
  AUTHOR_URL,
  KEYWORDS,
  SITE_DESCRIPTION,
  SITE_NAME,
  SITE_TITLE,
  SITE_URL,
  SOCIAL_DESCRIPTION,
  SOCIAL_TITLE,
  STRUCTURED_DATA,
} from "@/lib/seo";
import "./globals.css";

const manrope = Manrope({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-manrope",
});

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: {
    default: SITE_TITLE,
    template: `%s | ${SITE_NAME}`,
  },
  description: SITE_DESCRIPTION,
  applicationName: SITE_NAME,
  authors: [{ name: "Kevin Liu", url: AUTHOR_URL }],
  creator: "Kevin Liu",
  publisher: "Kevin Liu",
  category: "technology",
  keywords: KEYWORDS,
  referrer: "origin-when-cross-origin",
  formatDetection: {
    address: false,
    email: false,
    telephone: false,
  },
  alternates: {
    canonical: "/",
    types: {
      "text/plain": [
        { url: "/llms.txt", title: "local-search summary for AI agents" },
        { url: "/llms-full.txt", title: "local-search full reference for AI agents" },
      ],
      "application/json": [
        { url: "/benchmarks.json", title: "local-search benchmark data" },
      ],
    },
  },
  robots: {
    index: true,
    follow: true,
    nocache: false,
    googleBot: {
      index: true,
      follow: true,
      noimageindex: false,
      "max-image-preview": "large",
      "max-snippet": -1,
      "max-video-preview": -1,
    },
  },
  openGraph: {
    title: SOCIAL_TITLE,
    description: SOCIAL_DESCRIPTION,
    type: "website",
    url: "/",
    siteName: SITE_NAME,
    locale: "en_US",
  },
  twitter: {
    card: "summary_large_image",
    title: SOCIAL_TITLE,
    description: SOCIAL_DESCRIPTION,
    site: "@kevskgs",
    creator: "@kevskgs",
  },
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <head>
        <JsonLd data={STRUCTURED_DATA} />
      </head>
      <body className={manrope.variable}>{children}</body>
    </html>
  );
}
