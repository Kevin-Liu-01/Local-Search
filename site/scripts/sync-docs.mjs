import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";

const root = new URL("../../", import.meta.url);
const output = new URL("../public/docs-assets/", import.meta.url);
const repository = "https://github.com/Kevin-Liu-01/Local-Search/blob/main/";
export const screenshots = [
  "01-agent-browser.png", "02-browser-choice.png", "03-signed-in-sites.png",
  "04-search.png", "05-extract.png", "06-work-with-pages.png", "07-control.png",
];

// Only authored, public-safe assets. Never copy captured account pages or artifacts/.
await mkdir(output, { recursive: true });
await Promise.all(screenshots.map((name) => copyFile(new URL(`docs/images/${name}`, root), new URL(name, output))));
const guide = (await readFile(new URL("docs/agent-guide.md", root), "utf8"))
  .replaceAll("](images/", "](/docs-assets/")
  .replaceAll("](../SKILL.md)", "](/docs-assets/skill.md)")
  .replaceAll("](../SECURITY.md)", `](${repository}SECURITY.md)`)
  .replace(/\]\((verification\.md|visual-guide\.html|fixtures\/tasks\.html)\)/g, `](${repository}docs/$1)`);
const skill = (await readFile(new URL("SKILL.md", root), "utf8"))
  .replaceAll("](docs/agent-guide.md)", "](/docs-assets/agent-guide.md)")
  .replace(/\]\((docs\/[^)]+|README\.md|SECURITY\.md)\)/g, `](${repository}$1)`);
await writeFile(new URL("agent-guide.md", output), guide);
await writeFile(new URL("skill.md", output), skill);
console.log(`Synced ${screenshots.length} docs screenshots and two Markdown guides.`);
