import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";

// Run after the static build. Checks the artifact visitors and crawlers receive.
const site = new URL("../", import.meta.url);
const root = new URL("../../", import.meta.url);
const html = await readFile(new URL("out/docs.html", site), "utf8");
const homepage = await readFile(new URL("out/index.html", site), "utf8");
const sitemap = await readFile(new URL("out/sitemap.xml", site), "utf8");
assert.match(html, /<h1[\s>]/);
assert.equal([...html.matchAll(/<h1[\s>]/g)].length, 1);
assert.match(html, /rel="canonical" href="https?:\/\/[^" ]+\/docs"/);
assert.match(html, /"@type":"TechArticle"/);
assert.doesNotMatch(html, /"@type":"FAQPage"/);
assert.match(homepage, /"@type":"FAQPage"/);
assert.match(homepage, /href="\/docs"/);
assert.match(sitemap, /<loc>https?:\/\/[^<]+\/docs<\/loc>/);
assert.match(html, /awaiting release/);
assert.match(html, /does not enforce per-action approval/);
assert.match(html, /may reach its model provider/);
for (const diagram of ["bridge", "browser-diagram", "search-diagram", "extract-diagram", "action-flow", "disconnect-diagram"]) {
  assert.match(html, new RegExp(`class="docs-${diagram}"`), `Missing static diagram: ${diagram}`);
}
assert.match(html, /Example data · one record shown/);
assert.match(html, /<details class="docs-more"><summary>Connection errors/);
assert.equal([...html.matchAll(/<details class="docs-more">/g)].length, 8, "Keep one extras panel per section");
assert.equal([...html.matchAll(/<summary>Screenshots/g)].length, 1, "Group screenshots in one gallery");
for (const [, heading] of html.matchAll(/<h[123]\b[^>]*>([\s\S]*?)<\/h[123]>/g)) {
  assert.doesNotMatch(heading, /<br\b/, "Titles must not have forced line breaks");
}
for (const engine of ["google", "bing", "brave", "duckduckgo"]) {
  assert.match(html, new RegExp(`src="/brand/${engine}\\.svg"`), `Missing original ${engine} logo`);
}

const ids = [...html.matchAll(/\bid="([^"]+)"/g)].map((match) => match[1]);
assert.equal(ids.length, new Set(ids).size, "Duplicate HTML IDs");
for (const [, target] of html.matchAll(/href="#([^"]+)"/g)) {
  assert(ids.includes(target), `Missing anchor: ${target}`);
}
const images = [...new Set([...html.matchAll(/src="\/docs-assets\/([^"?]+\.png)"/g)].map((match) => match[1]))];
assert.equal(images.length, 7, "All seven public-safe walkthroughs must ship");
for (const name of images) {
  const source = await readFile(new URL(`docs/images/${name}`, root));
  const exported = await readFile(new URL(`out/docs-assets/${name}`, site));
  assert(source.equals(exported), `Stale screenshot: ${name}`);
}
const guide = await readFile(new URL("out/docs-assets/agent-guide.md", site), "utf8");
const skill = await readFile(new URL("out/docs-assets/skill.md", site), "utf8");
assert.match(guide, /\]\(\/docs-assets\/01-agent-browser.png\)/);
assert.match(guide, /\]\(\/docs-assets\/skill.md\)/);
assert.doesNotMatch(guide, /\]\(\.\.\//);
assert.match(skill, /browser_approval_timeout/);
assert.match(skill, /lsearch disconnect/);
assert.match(skill, /\]\(\/docs-assets\/agent-guide.md\)/);
console.log("Docs export: metadata, navigation, safety notices, seven screenshots, and Markdown downloads passed.");
