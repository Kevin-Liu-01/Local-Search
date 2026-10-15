/* eslint-disable @next/next/no-img-element */

import { ImageResponse } from "next/og";
import { readFileSync } from "node:fs";
import path from "node:path";
import { LocalSearchLogo } from "@/components/brand-logo";
import { SITE_URL, SOCIAL_IMAGE_ALT } from "@/lib/seo";
import { brandAsset, card, socialBackdrop } from "@/lib/social-scene";

export const socialImageSize = { width: card.width, height: card.height };
export const socialImageContentType = "image/png";
export const socialImageAlt = SOCIAL_IMAGE_ALT;

const color = {
  ink: "#151515",
  muted: "#5e625c",
  line: "#d7d7d1",
  panel: "#ffffff",
  accentSoft: "#c9f1ef",
  accentDeep: "#0d6f74",
};

// Static instances of the site's Manrope (docs/launch/fonts), since Satori
// cannot read a variable font's weight axis. The 400 instance has no kerning
// table: Satori widens the space after commas when it applies Manrope's.
function font(weight: 400 | 620 | 750) {
  return readFileSync(path.join(process.cwd(), "assets", "fonts", `Manrope-${weight}.ttf`));
}

const fonts = [
  { name: "Manrope", data: font(400), weight: 400 as const, style: "normal" as const },
  { name: "Manrope", data: font(620), weight: 600 as const, style: "normal" as const },
  { name: "Manrope", data: font(750), weight: 700 as const, style: "normal" as const },
];

const backdrop = socialBackdrop();
const proof = [
  ["Built with Rust", brandAsset("rust-mono.svg")],
  ["Compact JSON", brandAsset("json-schema.svg")],
  ["Your browser, your choice", brandAsset("chromium.svg")],
];
const host = new URL(SITE_URL).host;

export function createSocialImage() {
  const { margin } = card;

  return new ImageResponse(
    (
      <div
        style={{
          width: "100%",
          height: "100%",
          display: "flex",
          position: "relative",
          color: color.ink,
          fontFamily: "Manrope",
        }}
      >
        <img alt="" src={backdrop} width={card.width} height={card.height} style={{ position: "absolute", inset: 0 }} />

        <div
          style={{
            position: "absolute",
            top: 0,
            left: margin,
            right: margin,
            height: 112,
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
          }}
        >
          <div style={{ display: "flex", alignItems: "center" }}>
            <LocalSearchLogo style={{ width: 40, height: 30, color: color.ink }} />
            <div style={{ marginLeft: 12, display: "flex", fontSize: 22, fontWeight: 700, letterSpacing: -0.66 }}>
              local-search
            </div>
          </div>
          <div style={{ display: "flex", color: color.muted, fontSize: 18, fontWeight: 600, letterSpacing: -0.2 }}>
            {host}
          </div>
        </div>

        <div
          style={{
            position: "absolute",
            top: 150,
            left: margin,
            width: 620,
            display: "flex",
            flexDirection: "column",
            alignItems: "flex-start",
          }}
        >
          <div style={{ display: "flex", flexDirection: "column", fontSize: 66, lineHeight: 1.09, fontWeight: 600, letterSpacing: -3.3 }}>
            <div style={{ display: "flex" }}>A local browser API</div>
            <div style={{ display: "flex", color: color.accentDeep }}>for your agents.</div>
          </div>

          <div style={{ marginTop: 22, display: "flex", width: 610, color: color.muted, fontSize: 22, lineHeight: 1.55, fontWeight: 400 }}>
            Let your agent search, read pages, and use signed-in sites through your browser. You approve access.
          </div>

          <div
            style={{
              marginTop: 26,
              padding: 7,
              display: "flex",
              alignItems: "center",
              border: `1px solid ${color.line}`,
              borderRadius: 8,
              backgroundColor: color.panel,
            }}
          >
            <div
              style={{
                padding: 5,
                display: "flex",
                border: "1px solid #addcda",
                borderRadius: 6,
                backgroundColor: color.accentSoft,
              }}
            >
              <div
                style={{
                  padding: "9px 16px",
                  display: "flex",
                  alignItems: "center",
                  borderRadius: 4,
                  backgroundColor: color.panel,
                  boxShadow: "0 1px 3px rgba(13, 111, 116, .12)",
                  fontSize: 18,
                  fontWeight: 600,
                }}
              >
                <img alt="" src={brandAsset("cargo.png")} width={20} height={20} style={{ marginRight: 8 }} />
                Cargo
              </div>
              <div style={{ padding: "9px 16px", display: "flex", alignItems: "center", color: color.accentDeep, fontSize: 18, fontWeight: 600 }}>
                <img alt="" src={brandAsset("npm.svg")} width={18} height={18} style={{ marginRight: 8 }} />
                npm
              </div>
            </div>
            <div style={{ marginLeft: 18, marginRight: 14, display: "flex", fontSize: 19, fontWeight: 600, letterSpacing: -0.2 }}>
              cargo install local-search
            </div>
          </div>

          <div style={{ marginTop: 24, display: "flex", alignItems: "center", color: color.muted, fontSize: 17 }}>
            {proof.map(([label, source], index) => (
              <div key={label} style={{ marginLeft: index === 0 ? 0 : 22, display: "flex", alignItems: "center" }}>
                <img alt="" src={source} width={20} height={20} style={{ marginRight: 8 }} />
                {label}
              </div>
            ))}
          </div>
        </div>
      </div>
    ),
    { ...socialImageSize, fonts },
  );
}
