import { readFileSync } from "node:fs";
import path from "node:path";

// The social card's backdrop: the hero's dither field and isometric scene, drawn
// as one SVG. Satori renders neither CSS classes nor color-mix(), so this mirrors
// the light theme in app/globals.css with resolved colors. Keep the geometry in
// step with HeroIsometricArtwork in app/page.tsx.

export const card = { width: 1200, height: 630, margin: 64 };

const page = "#f5f5f2";
const ink = "#151515";
const accent = "#168b90";
const accentSoft = "#c9f1ef";
const accentDeep = "#0d6f74";

function dataUri(relativePath: string, mimeType: string) {
  const encoded = readFileSync(path.join(process.cwd(), "public", relativePath)).toString("base64");
  return `data:${mimeType};base64,${encoded}`;
}

export function brandAsset(name: string) {
  return name.endsWith(".png")
    ? dataUri(`brand/${name}`, "image/png")
    : dataUri(`brand/${name}`, "image/svg+xml");
}

const LOGO_PATH =
  "M4 1h54l-2 7 7 5-4 9 4 7-7 14H5l2-7-6-5 4-9-4-7L4 1Zm7 33h6l11-24h-6L11 34Zm19-24h6v19h4v5H30V10Zm13 0h14v6h-8v3h8v15H43v-6h8v-3h-8V10Z";

// Inner-to-outer routing, as in the hero: the top box port feeds the lowest engine.
const connections = [
  "M306 351L350 376.4L350 384L450 441.7L566 374.8L566 357.8L549 348",
  "M306 369L344 390.9L344 402L444 459.7L576 383.5L576 275.6L549 260",
  "M306 387L338 405.5L338 420L438 477.7L586 392.3L586 193.4L549 172",
  "M306 405L332 420L332 438L432 495.7L596 401L596 111.1L549 84",
];

// Top to bottom. Each tile sits 88 units below the one above it.
const engines = [
  { mark: "google.svg", size: 58, markY: 46, top: "#ffffff", topOpacity: 0.91, sides: ["#eaeaea", "#e3e3e3", "#dedede"] },
  { mark: "bing.svg", size: 64, markY: 131, top: "#ecf9fb", topOpacity: 1, sides: ["#e7f5f7", "#e0f0f3", "#dbedf0"] },
  { mark: "brave.svg", size: 58, markY: 222, top: "#fff2ee", topOpacity: 1, sides: ["#f9eae5", "#f6e4dd", "#f4dfd8"] },
  { mark: "duckduckgo.svg", size: 58, markY: 310, top: "#fff3ee", topOpacity: 1, sides: ["#f8eae5", "#f5e4de", "#f3e1da"] },
];

function shift(points: string, dy: number) {
  return points
    .split(" ")
    .map((pair) => {
      const [x, y] = pair.split(",").map(Number);
      return `${x},${y + dy}`;
    })
    .join(" ");
}

function engineTile(engine: (typeof engines)[number], index: number) {
  const dy = index * 88;
  const sideStroke = `stroke="${ink}" stroke-width="1"`;
  return `
    <polygon points="${shift("434,132 351,84 351,96 434,144", dy)}" fill="${engine.sides[0]}" ${sideStroke} stroke-opacity=".24"/>
    <polygon points="${shift("466,132 434,132 434,144 466,144", dy)}" fill="${engine.sides[1]}" ${sideStroke} stroke-opacity=".25"/>
    <polygon points="${shift("549,84 466,132 466,144 549,96", dy)}" fill="${engine.sides[2]}" ${sideStroke} stroke-opacity=".26"/>
    <polygon points="${shift("434,18 466,18 549,66 549,84 466,132 434,132 351,84 351,66", dy)}" fill="${engine.top}" fill-opacity="${engine.topOpacity}" stroke="${ink}" stroke-opacity=".34" stroke-width="1"/>
    <image href="${brandAsset(engine.mark)}" width="${engine.size}" height="${engine.size}" opacity=".94" transform="matrix(.866 .5 -.866 .5 450 ${engine.markY})"/>`;
}

function agentLight(cx: number, cy: number, state: "active" | "idle" | "dim") {
  const side = state === "active" ? accentDeep : "#454642";
  const top = state === "active" ? accent : state === "dim" ? "#777873" : "#9a9b96";
  const rim = state === "active" ? 0.38 : 0.22;
  return `
      <ellipse cx="${cx}" cy="${cy + 1.6}" rx="5.2" ry="3" fill="${side}"/>
      <ellipse cx="${cx}" cy="${cy}" rx="5.2" ry="3" fill="${top}" stroke="#ffffff" stroke-opacity="${rim}" stroke-width=".65"/>
      <ellipse cx="${cx - 1.8}" cy="${cy - 1}" rx="1.25" ry=".65" fill="#ffffff" fill-opacity=".46"/>`;
}

function isometricScene() {
  const hex = (points: string, filled = false) =>
    filled
      ? `<polygon points="${points}" fill="#d7f2f0" stroke="${accent}" stroke-opacity=".6" stroke-width="1"/>`
      : `<polygon points="${points}" fill="${page}" fill-opacity=".86" stroke="${ink}" stroke-opacity=".36" stroke-width="1"/>`;
  const dot = (cx: number, cy: number) => `<circle cx="${cx}" cy="${cy}" r="2" fill="${accent}" fill-opacity=".66"/>`;
  const port = (points: string) =>
    `<polygon points="${points}" fill="${accent}" stroke="#ffffff" stroke-opacity=".65" stroke-width="1"/>`;

  return `
  <g filter="url(#scene-shadow)">
    <g>
      ${hex("292,82 301,87 301,97 292,102 283,97 283,87")}
      ${hex("310,98 319,103 319,113 310,118 301,113 301,103", true)}
      ${hex("292,114 301,119 301,129 292,134 283,129 283,119")}
      ${hex("564,24 574,30 574,41 564,47 554,41 554,30")}
      ${hex("584,42 594,48 594,59 584,65 574,59 574,48")}
      ${hex("564,60 574,66 574,77 564,83 554,77 554,66", true)}
      <g transform="translate(40 24)">
        ${hex("530,408 540,414 540,425 530,431 520,425 520,414")}
        ${hex("550,426 560,432 560,443 550,449 540,443 540,432", true)}
        ${hex("530,444 540,450 540,461 530,467 520,461 520,450")}
        ${dot(550, 461)}
      </g>
      ${dot(310, 139)}
      ${dot(584, 76)}
    </g>
    <g fill="${ink}" fill-opacity=".07">
      <polygon points="365,392 450,343 535,392 450,441"/>
      <polygon points="96,440 210,374 316,435 202,500"/>
    </g>
    <g fill="none" stroke="${ink}" stroke-opacity=".43" stroke-width="1.2">
      ${connections.map((d) => `<path d="${d}"/>`).join("")}
    </g>
    <g fill="none" stroke="${accent}" stroke-opacity=".78" stroke-width="1.6" stroke-dasharray="2 15" stroke-linecap="round">
      ${connections.map((d) => `<path d="${d}"/>`).join("")}
    </g>
    ${[...engines].reverse().map((engine) => engineTile(engine, engines.indexOf(engine))).join("")}
    <g>
      <polygon points="84,357 179,412 179,484 84,429" fill="#242422" stroke="#111111" stroke-width="1"/>
      <polygon points="179,412 306,339 306,411 179,484" fill="#171716" stroke="#0c0c0c" stroke-width="1"/>
      <polygon points="84,357 211,284 306,339 179,412" fill="#343432" stroke="#111111" stroke-width="1"/>
      <path d="m113 357 98-56 66 38-98 56z" fill="none" stroke="#ffffff" stroke-opacity=".1" stroke-width="1"/>
      <g transform="translate(-10 -6)">
        ${agentLight(247, 330.8, "active")}
        ${agentLight(260, 338.3, "idle")}
        ${agentLight(273, 345.8, "dim")}
      </g>
      <polygon points="190,420 288,363 288,398 190,455" fill="${accentSoft}" stroke="${accent}" stroke-opacity=".34" stroke-width="1"/>
      <g transform="matrix(.866 -.5 0 1 207 424) scale(1.125 .4545)">
        <path d="${LOGO_PATH}" fill="${ink}" fill-rule="evenodd"/>
      </g>
      <path d="M30 58H83 M30 62H83 M30 66H83" transform="matrix(1 -.574803 0 1 179 412)" fill="none" stroke="#ffffff" stroke-opacity=".17" stroke-width="1"/>
      <image href="${brandAsset("claude.svg")}" width="64" height="64" transform="matrix(.866 .5 -.866 .5 190 319)"/>
    </g>
    <g>
      ${port("549,78 554,81 554,87 549,90 544,87 544,81")}
      ${port("549,166 554,169 554,175 549,178 544,175 544,169")}
      ${port("549,254 554,257 554,263 549,266 544,263 544,257")}
      ${port("549,342 554,345 554,351 549,354 544,351 544,345")}
      ${port("306,345 311,348 311,354 306,357 301,354 301,348")}
      ${port("306,363 311,366 311,372 306,375 301,372 301,366")}
      ${port("306,381 311,384 311,390 306,393 301,390 301,384")}
      ${port("306,399 311,402 311,408 306,411 301,408 301,402")}
    </g>
  </g>`;
}

export function socialBackdrop() {
  const { width, height } = card;
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">
  <defs>
    <pattern id="dither" width="9" height="9" patternUnits="userSpaceOnUse">
      <circle cx="4.5" cy="4.5" r=".85" fill="#707871"/>
    </pattern>
    <radialGradient id="dither-fade" cx="50%" cy="50%" r="50%">
      <stop offset=".28" stop-color="#ffffff"/>
      <stop offset=".72" stop-color="#ffffff" stop-opacity="0"/>
    </radialGradient>
    <mask id="dither-mask"><rect x="540" y="40" width="660" height="560" fill="url(#dither-fade)"/></mask>
    <filter id="scene-shadow" x="-10%" y="-10%" width="130%" height="130%">
      <feDropShadow dx="10" dy="14" stdDeviation="8" flood-color="${ink}" flood-opacity=".08"/>
    </filter>
  </defs>
  <rect width="${width}" height="${height}" fill="${page}"/>
  <rect x="540" y="40" width="660" height="560" fill="url(#dither)" opacity=".26" mask="url(#dither-mask)"/>
  <g transform="translate(586 74) scale(.95)">${isometricScene()}</g>
</svg>`;
  return `data:image/svg+xml;base64,${Buffer.from(svg).toString("base64")}`;
}
