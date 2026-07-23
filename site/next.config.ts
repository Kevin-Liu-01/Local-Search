import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  output: "export",
  experimental: {
    inlineCss: true,
  },
};

export default nextConfig;
