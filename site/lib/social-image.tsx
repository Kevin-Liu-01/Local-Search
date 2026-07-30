import { ImageResponse } from "next/og";
import { LocalSearchLogo } from "@/components/brand-logo";

export const socialImageSize = { width: 1200, height: 630 };
export const socialImageContentType = "image/png";
export const socialImageAlt =
  "local-search — a 1.06 MB open-source Rust browser search API for coding agents, with no API key or metered billing.";

const color = {
  background: "#101414",
  cyan: "#baf4f0",
  cyanDeep: "#138489",
  ink: "#101414",
  line: "rgba(230, 245, 242, 0.22)",
  muted: "rgba(237, 245, 243, 0.60)",
  panel: "#161b1a",
  white: "#f4f7f5",
};

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
          color: color.white,
          backgroundColor: color.background,
          backgroundImage:
            "radial-gradient(circle at center, rgba(186,244,240,.12) 0 1px, transparent 1.35px)",
          backgroundSize: "13px 13px",
          fontFamily: "Manrope, Arial, sans-serif",
        }}
      >
        <div
          style={{
            position: "absolute",
            top: 40,
            left: 40,
            right: 40,
            bottom: 40,
            display: "flex",
            border: `1px solid ${color.line}`,
          }}
        />
        <div
          style={{
            position: "absolute",
            top: 40,
            left: 40,
            width: 1120,
            height: 78,
            display: "flex",
            borderBottom: `1px solid ${color.line}`,
            backgroundColor: "rgba(16,20,20,.84)",
          }}
        />

        {[
          { left: 32, top: 32 },
          { right: 32, top: 32 },
          { bottom: 32, left: 32 },
          { bottom: 32, right: 32 },
        ].map((position, index) => (
          <div
            key={index}
            style={{ position: "absolute", width: 17, height: 17, display: "flex", ...position }}
          >
            <div
              style={{
                position: "absolute",
                top: 8,
                left: 0,
                width: 17,
                height: 1,
                display: "flex",
                background: color.white,
              }}
            />
            <div
              style={{
                position: "absolute",
                top: 0,
                left: 8,
                width: 1,
                height: 17,
                display: "flex",
                background: color.white,
              }}
            />
          </div>
        ))}

        <div
          style={{
            position: "absolute",
            top: 61,
            left: 72,
            display: "flex",
            alignItems: "center",
          }}
        >
          <LocalSearchLogo style={{ width: 48, height: 34, color: color.cyan }} />
          <div
            style={{
              marginLeft: 14,
              display: "flex",
              fontSize: 22,
              fontWeight: 700,
              letterSpacing: -0.5,
            }}
          >
            local-search
          </div>
        </div>
        <div
          style={{
            position: "absolute",
            top: 63,
            right: 72,
            display: "flex",
            alignItems: "center",
            color: color.muted,
            fontSize: 14,
            fontWeight: 700,
            letterSpacing: 2.2,
          }}
        >
          <div
            style={{
              width: 8,
              height: 8,
              marginRight: 11,
              display: "flex",
              backgroundColor: color.cyan,
              transform: "rotate(45deg)",
            }}
          />
          OPEN-SOURCE RUST CLI · 1.06 MB
        </div>

        <div
          style={{
            position: "absolute",
            top: 158,
            left: 76,
            width: 660,
            display: "flex",
            flexDirection: "column",
          }}
        >
          <div
            style={{
              display: "flex",
              fontSize: 65,
              lineHeight: 1,
              fontWeight: 760,
              letterSpacing: -3.8,
            }}
          >
            Browser Search API.
          </div>
          <div
            style={{
              marginTop: 14,
              display: "flex",
              color: color.cyan,
              fontSize: 53,
              lineHeight: 1,
              fontWeight: 450,
              letterSpacing: -3,
            }}
          >
            No key. No billing.
          </div>
          <div
            style={{
              width: 610,
              marginTop: 26,
              display: "flex",
              color: color.muted,
              fontSize: 20,
              lineHeight: 1.45,
            }}
          >
            Give Claude Code, Codex, Cursor, or any shell agent structured Google, Bing,
            DuckDuckGo, and Brave Search results through the browser on your machine.
          </div>

          <div
            style={{
              width: 570,
              height: 58,
              marginTop: 26,
              padding: "0 18px",
              display: "flex",
              alignItems: "center",
              border: `1px solid ${color.line}`,
              backgroundColor: color.panel,
              fontSize: 18,
            }}
          >
            <span style={{ marginRight: 12, display: "flex", color: color.cyan }}>$</span>
            cargo install local-search
            <span
              style={{
                marginLeft: "auto",
                display: "flex",
                color: color.muted,
                fontSize: 11,
                fontWeight: 700,
                letterSpacing: 1.4,
              }}
            >
              THEN RUN LSEARCH
            </span>
          </div>
        </div>

        <div
          style={{
            position: "absolute",
            top: 137,
            right: 62,
            width: 416,
            height: 378,
            display: "flex",
            borderLeft: `1px solid ${color.line}`,
            backgroundColor: "rgba(19,132,137,.08)",
            backgroundImage:
              "radial-gradient(circle at center, rgba(186,244,240,.30) 0 1px, transparent 1.35px)",
            backgroundSize: "9px 9px",
          }}
        >
          <svg width="416" height="378" viewBox="0 0 416 378" fill="none">
            <g opacity="0.36" stroke={color.cyan} strokeWidth="1">
              <path d="M132 279H182V331H363V82H337" />
              <path d="M132 299H199V346H379V176H337" />
              <path d="M132 319H216V361H395V270H337" />
            </g>
            <g stroke={color.cyan} strokeWidth="1.4" strokeDasharray="2 9" strokeLinecap="round">
              <path d="M132 279H182V331H363V82H337" />
              <path d="M132 299H199V346H379V176H337" />
              <path d="M132 319H216V361H395V270H337" />
            </g>

            <g>
              <path d="M229 42L246 32H320L337 42V100L320 110H246L229 100V42Z" fill="#222927" />
              <path d="M229 32L246 22H320L337 32V90L320 100H246L229 90V32Z" fill="#f7f8f5" stroke="#92a09d" />
              <path
                d="M298 60H283V68H291C289 73 285 75 280 75C270 75 264 68 264 59C264 50 271 43 280 43C286 43 290 45 294 48L300 42C295 37 288 34 280 34C266 34 255 45 255 59C255 73 266 84 280 84C294 84 303 74 303 60H298Z"
                fill="#4285F4"
              />
            </g>
            <g>
              <path d="M229 136L246 126H320L337 136V194L320 204H246L229 194V136Z" fill="#193438" />
              <path d="M229 126L246 116H320L337 126V184L320 194H246L229 184V126Z" fill="#dff3f5" stroke="#92a09d" />
              <path
                d="M272 136V171L284 179L302 168L286 161L279 166V138L272 136Z"
                fill="#0C87B8"
              />
            </g>
            <g>
              <path d="M229 230L246 220H320L337 230V288L320 298H246L229 288V230Z" fill="#3d2620" />
              <path d="M229 220L246 210H320L337 220V278L320 288H246L229 278V220Z" fill="#fff0eb" stroke="#92a09d" />
              <circle cx="283" cy="249" r="22" fill="#E76F42" />
              <circle cx="278" cy="246" r="10" fill="#fff" />
              <path d="M286 244L300 249L287 254Z" fill="#F5B642" />
              <circle cx="275" cy="243" r="2" fill="#171a19" />
            </g>

            <g>
              <path d="M25 264L83 230L139 262V326L81 360L25 328V264Z" fill="#151918" />
              <path d="M25 264L83 230L139 262L81 296L25 264Z" fill="#3a4140" />
              <path d="M25 264L81 296V360L25 328V264Z" fill="#262b2a" />
              <path d="M81 296L139 262V326L81 360V296Z" fill="#111413" />
              <path d="M91 303L129 281V314L91 336V303Z" fill={color.cyan} />
              <path
                d="M96 325H101L109 306H104L96 325ZM111 306H116V320H120V325H111V306ZM122 306H133V311H127V314H133V325H122V320H128V318H122V306Z"
                fill={color.ink}
              />
              <circle cx="112" cy="260" r="4" fill={color.cyanDeep} />
              <circle cx="123" cy="266" r="4" fill="#a4aaa8" />
            </g>

            <g fill={color.cyan} stroke={color.background} strokeWidth="2">
              <circle cx="132" cy="279" r="5" />
              <circle cx="132" cy="299" r="5" />
              <circle cx="132" cy="319" r="5" />
              <circle cx="337" cy="82" r="5" />
              <circle cx="337" cy="176" r="5" />
              <circle cx="337" cy="270" r="5" />
            </g>
          </svg>
          <div
            style={{
              position: "absolute",
              top: 23,
              left: 24,
              display: "flex",
              color: color.cyan,
              opacity: 0.68,
              fontSize: 11,
              fontWeight: 700,
              letterSpacing: 2,
            }}
          >
            LOCAL BROWSER ROUTING
          </div>
        </div>

        <div
          style={{
            position: "absolute",
            left: 76,
            bottom: 60,
            width: 624,
            height: 60,
            display: "flex",
            borderTop: `1px solid ${color.line}`,
          }}
        >
          {[
            ["1.06 MB", "NATIVE BINARY"],
            ["$0", "SEARCH API BILL"],
            ["JSON", "STABLE STDOUT"],
          ].map(([value, label], index) => (
            <div
              key={label}
              style={{
                width: 208,
                paddingTop: 14,
                display: "flex",
                alignItems: "baseline",
                borderRight: index < 2 ? `1px solid ${color.line}` : "none",
              }}
            >
              <span style={{ display: "flex", color: color.cyan, fontSize: 20, fontWeight: 750 }}>
                {value}
              </span>
              <span
                style={{
                  marginLeft: 9,
                  display: "flex",
                  color: color.muted,
                  fontSize: 9,
                  fontWeight: 700,
                  letterSpacing: 1.3,
                }}
              >
                {label}
              </span>
            </div>
          ))}
        </div>
      </div>
    ),
    socialImageSize,
  );
}
