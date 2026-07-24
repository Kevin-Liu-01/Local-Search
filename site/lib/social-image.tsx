import { ImageResponse } from "next/og";

export const socialImageSize = { width: 1200, height: 630 };
export const socialImageContentType = "image/png";
export const socialImageAlt = "local-search — Browser Search API. No API Key. No Billing. Structured local browser search for AI coding agents.";

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
          color: "#111210",
          backgroundColor: "#f4f4f0",
          backgroundImage: "radial-gradient(circle at center, rgba(13,111,116,.13) 0 1px, transparent 1.4px)",
          backgroundSize: "12px 12px",
          fontFamily: "sans-serif",
        }}
      >
        <div style={{ position: "absolute", inset: "46px", display: "flex", border: "1px solid #cfd0ca" }} />
        <div style={{ position: "absolute", top: 0, left: 46, width: 1, height: 630, display: "flex", background: "#cfd0ca" }} />
        <div style={{ position: "absolute", top: 0, right: 46, width: 1, height: 630, display: "flex", background: "#cfd0ca" }} />
        <div style={{ position: "absolute", top: 46, left: 0, width: 1200, height: 1, display: "flex", background: "#cfd0ca" }} />
        <div style={{ position: "absolute", bottom: 46, left: 0, width: 1200, height: 1, display: "flex", background: "#cfd0ca" }} />

        <div style={{ position: "absolute", top: 39, left: 39, width: 15, height: 15, display: "flex" }}>
          <div style={{ position: "absolute", top: 7, left: 0, width: 15, height: 1, display: "flex", background: "#111210" }} />
          <div style={{ position: "absolute", top: 0, left: 7, width: 1, height: 15, display: "flex", background: "#111210" }} />
        </div>
        <div style={{ position: "absolute", top: 39, right: 39, width: 15, height: 15, display: "flex" }}>
          <div style={{ position: "absolute", top: 7, left: 0, width: 15, height: 1, display: "flex", background: "#111210" }} />
          <div style={{ position: "absolute", top: 0, left: 7, width: 1, height: 15, display: "flex", background: "#111210" }} />
        </div>
        <div style={{ position: "absolute", bottom: 39, left: 39, width: 15, height: 15, display: "flex" }}>
          <div style={{ position: "absolute", top: 7, left: 0, width: 15, height: 1, display: "flex", background: "#111210" }} />
          <div style={{ position: "absolute", top: 0, left: 7, width: 1, height: 15, display: "flex", background: "#111210" }} />
        </div>
        <div style={{ position: "absolute", right: 39, bottom: 39, width: 15, height: 15, display: "flex" }}>
          <div style={{ position: "absolute", top: 7, left: 0, width: 15, height: 1, display: "flex", background: "#111210" }} />
          <div style={{ position: "absolute", top: 0, left: 7, width: 1, height: 15, display: "flex", background: "#111210" }} />
        </div>

        <div style={{ position: "absolute", top: 83, left: 84, width: 640, display: "flex", flexDirection: "column" }}>
          <div style={{ display: "flex", alignItems: "center", fontSize: 16, fontWeight: 700, letterSpacing: 2.6, color: "#4d5551" }}>
            <div style={{ width: 40, height: 25, marginRight: 13, display: "flex", alignItems: "center", justifyContent: "center", color: "white", background: "#111210", fontSize: 14, fontWeight: 900, letterSpacing: -1 }}>/LS</div>
            OPEN-SOURCE RUST CLI
          </div>
          <div style={{ marginTop: 36, display: "flex", flexDirection: "column", fontSize: 70, lineHeight: 0.98, letterSpacing: -4.2 }}>
            <div style={{ display: "flex", fontWeight: 800 }}>Browser Search API.</div>
            <div style={{ marginTop: 12, display: "flex", fontWeight: 400 }}>No key. No billing.</div>
          </div>
          <div style={{ width: 610, marginTop: 27, display: "flex", color: "#656963", fontSize: 23, lineHeight: 1.45 }}>
            Structured Google, Bing, and DuckDuckGo results for Claude Code, Codex, Cursor, and any shell agent.
          </div>
          <div style={{ width: 510, height: 54, marginTop: 31, padding: "0 18px", display: "flex", alignItems: "center", color: "#f4f4f0", background: "#161817", border: "1px solid #111210", fontSize: 19 }}>
            <span style={{ marginRight: 10, display: "flex", color: "#82d6d1" }}>$</span>
            cargo install local-search
          </div>
        </div>

        <div style={{ position: "absolute", top: 92, right: 59, width: 455, height: 470, display: "flex", alignItems: "center", justifyContent: "center" }}>
          <svg width="455" height="470" viewBox="0 0 455 470" fill="none" xmlns="http://www.w3.org/2000/svg">
            <g opacity="0.26" stroke="#0d6f74" strokeWidth="1">
              <path d="M72 344L126 376V420L392 266V80L354 58" />
              <path d="M72 363L140 402V436L405 282V183L354 154" />
              <path d="M72 382L154 429V451L418 298V285L354 248" />
            </g>
            <g stroke="#168b90" strokeWidth="1.4" strokeDasharray="2 11" strokeLinecap="round">
              <path d="M72 344L126 376V420L392 266V80L354 58" />
              <path d="M72 363L140 402V436L405 282V183L354 154" />
              <path d="M72 382L154 429V451L418 298V285L354 248" />
            </g>
            <g>
              <polygon points="244,266 309,228 374,266 309,304" fill="#e8d7d0" />
              <polygon points="244,250 309,212 374,250 309,288" fill="#fff8f4" stroke="#b9b9b4" />
              <circle cx="309" cy="250" r="20" fill="#DE5833" opacity=".9" />
              <circle cx="305" cy="248" r="9" fill="white" />
              <path d="M312 247L326 252L313 256Z" fill="#F6B33A" />
              <circle cx="302" cy="245" r="1.8" fill="#222" />
            </g>
            <g>
              <polygon points="244,176 309,138 374,176 309,214" fill="#cfe3e7" />
              <polygon points="244,160 309,122 374,160 309,198" fill="#eef9fb" stroke="#b9b9b4" />
              <circle cx="309" cy="160" r="20" fill="#168B90" opacity=".9" />
              <path d="M302 145V169L311 175L325 167L313 161L307 164V147Z" fill="white" />
            </g>
            <g>
              <polygon points="244,86 309,48 374,86 309,124" fill="#deded9" />
              <polygon points="244,70 309,32 374,70 309,108" fill="white" stroke="#b9b9b4" />
              <circle cx="309" cy="70" r="20" fill="#ffffff" stroke="#d2d2ce" />
              <path d="M322 69H309V76H316C314 79 311 81 307 81C300 81 295 76 295 69C295 62 300 57 307 57C311 57 314 58 317 61L322 56C318 52 313 50 307 50C296 50 288 58 288 69C288 80 296 88 307 88C318 88 325 81 325 70C325 69 325 69 325 69Z" fill="#4285F4" />
            </g>
            <g>
              <polygon points="28,323 109,276 181,318 100,365" fill="#454744" />
              <polygon points="28,323 100,365 100,430 28,388" fill="#292b29" />
              <polygon points="100,365 181,318 181,383 100,430" fill="#151716" />
              <polygon points="112,371 168,339 168,369 112,401" fill="#c9f1ef" />
              <path d="M120 382L127 378L142 350L135 354Z" fill="#111210" />
              <path d="M140 369L147 365V374L156 369V376L140 385Z" fill="#111210" />
              <path d="M151 357L169 347V354L159 360L169 361V369L151 379V372L162 366L151 365Z" fill="#111210" />
            </g>
            <g fill="#168b90" stroke="white" strokeWidth="1">
              <circle cx="354" cy="58" r="4" /><circle cx="354" cy="154" r="4" /><circle cx="354" cy="248" r="4" />
              <circle cx="72" cy="344" r="4" /><circle cx="72" cy="363" r="4" /><circle cx="72" cy="382" r="4" />
            </g>
          </svg>
        </div>
      </div>
    ),
    socialImageSize,
  );
}
