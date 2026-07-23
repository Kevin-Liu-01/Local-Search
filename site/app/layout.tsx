import type { Metadata } from "next";
import { Manrope } from "next/font/google";
import "./globals.css";

const manrope = Manrope({
  subsets: ["latin"],
  display: "swap",
  variable: "--font-manrope",
});

export const metadata: Metadata = {
  title: "local-search — Free structured search for coding agents",
  description:
    "Give coding agents structured web search through your local browser. No API key or metered search bill.",
  openGraph: {
    title: "local-search — Your browser is already a search API",
    description: "Stable search JSON. Your local browser. Zero search API credits.",
    type: "website",
  },
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body className={manrope.variable}>{children}</body>
    </html>
  );
}
