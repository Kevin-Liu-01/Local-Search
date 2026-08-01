/* eslint-disable @next/next/no-img-element */

import { ImageResponse } from "next/og";
import { readFileSync } from "node:fs";
import path from "node:path";
import { LocalSearchLogo } from "@/components/brand-logo";
import { SOCIAL_IMAGE_ALT } from "@/lib/seo";

export const socialImageSize = { width: 1200, height: 630 };
export const socialImageContentType = "image/png";
export const socialImageAlt = SOCIAL_IMAGE_ALT;

const color = {
  background: "#f5f5f2",
  accent: "#168b90",
  accentSoft: "#c9f1ef",
  ink: "#151515",
  line: "#d7d7d1",
  muted: "#6f6f69",
  white: "#ffffff",
};

function publicAsset(relativePath: string, mimeType: string) {
  const encoded = readFileSync(path.join(process.cwd(), "public", relativePath)).toString("base64");
  return `data:${mimeType};base64,${encoded}`;
}

const diagram = publicAsset("social/local-search-isometric.png", "image/png");
const cargo = publicAsset("brand/cargo.png", "image/png");

const searchEngines = [
  ["Google", publicAsset("brand/google.svg", "image/svg+xml")],
  ["Bing", publicAsset("brand/bing.svg", "image/svg+xml")],
  ["Brave", publicAsset("brand/brave.svg", "image/svg+xml")],
  ["DuckDuckGo", publicAsset("brand/duckduckgo.svg", "image/svg+xml")],
];

const agents = [
  ["Claude Code", publicAsset("brand/claude.svg", "image/svg+xml")],
  ["Codex", publicAsset("brand/codex.svg", "image/svg+xml")],
  ["Cursor", publicAsset("brand/cursor-mono.svg", "image/svg+xml")],
  ["OpenClaw", publicAsset("brand/openclaw.svg", "image/svg+xml")],
];

function LogoRow({
  items,
  size,
}: {
  items: string[][];
  size: number;
}) {
  return (
    <div style={{ display: "flex", alignItems: "center" }}>
      {items.map(([label, source], index) => (
        <div
          key={label}
          style={{
            width: size,
            height: size,
            marginRight: index === items.length - 1 ? 0 : 8,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            overflow: "hidden",
            border: `1px solid ${color.line}`,
            borderRadius: 10,
            backgroundColor: "rgba(255,255,255,.82)",
          }}
        >
          <img alt={label} src={source} width={size - 16} height={size - 16} />
        </div>
      ))}
    </div>
  );
}

export function createSocialImage() {
  return new ImageResponse(
    (
      <div
        style={{
          width: "100%",
          height: "100%",
          display: "flex",
          position: "relative",
          overflow: "hidden",
          color: color.ink,
          backgroundColor: color.background,
          fontFamily: "Manrope, Arial, sans-serif",
        }}
      >
        <img
          alt=""
          src={diagram}
          width="982"
          height="630"
          style={{
            position: "absolute",
            top: 0,
            right: 0,
            width: 982,
            height: 630,
            objectFit: "contain",
          }}
        />

        <div
          style={{
            position: "absolute",
            top: 55,
            left: 58,
            display: "flex",
            alignItems: "center",
          }}
        >
          <LocalSearchLogo style={{ width: 52, height: 36, color: color.accent }} />
          <div
            style={{
              marginLeft: 12,
              display: "flex",
              fontSize: 23,
              fontWeight: 750,
              letterSpacing: -0.7,
            }}
          >
            local-search
          </div>
        </div>

        <div
          style={{
            position: "absolute",
            top: 158,
            left: 58,
            width: 660,
            display: "flex",
            flexDirection: "column",
            alignItems: "flex-start",
          }}
        >
          <div style={{ display: "flex", alignItems: "baseline" }}>
            <div
              style={{
                display: "flex",
                color: color.accent,
                fontSize: 84,
                lineHeight: 1,
                fontWeight: 800,
                letterSpacing: -4.2,
              }}
            >
              $0
            </div>
            <div
              style={{
                marginLeft: 17,
                display: "flex",
                fontSize: 56,
                lineHeight: 1,
                fontWeight: 720,
                letterSpacing: -3,
              }}
            >
              Browser Search API
            </div>
          </div>

          <div
            style={{
              marginTop: 7,
              display: "flex",
              color: color.accent,
              fontSize: 64,
              lineHeight: 1.1,
              fontWeight: 500,
              letterSpacing: -2.4,
            }}
          >
            No API Key. No Billing.
          </div>

          <div style={{ marginTop: 24, display: "flex", alignItems: "center" }}>
            <div
              style={{
                width: 150,
                display: "flex",
                color: color.muted,
                fontSize: 16,
                fontWeight: 650,
                letterSpacing: -0.1,
              }}
            >
              Search using...
            </div>
            <LogoRow items={searchEngines} size={54} />
          </div>

          <div style={{ marginTop: 10, display: "flex", alignItems: "center" }}>
            <div
              style={{
                width: 150,
                display: "flex",
                color: color.muted,
                fontSize: 16,
                fontWeight: 650,
                letterSpacing: -0.1,
              }}
            >
              Compatible with...
            </div>
            <LogoRow items={agents} size={54} />
          </div>
        </div>

        <div
          style={{
            position: "absolute",
            left: 58,
            bottom: 35,
            display: "flex",
            alignItems: "center",
            color: color.ink,
            fontSize: 19,
            fontWeight: 650,
            letterSpacing: -0.3,
          }}
        >
          <img
            alt="Cargo"
            src={cargo}
            width="35"
            height="35"
            style={{ width: 35, height: 35, objectFit: "contain" }}
          />
          <span style={{ marginLeft: 13, marginRight: 10, display: "flex", color: color.accent }}>
            $
          </span>
          cargo install local-search
        </div>
      </div>
    ),
    socialImageSize,
  );
}
