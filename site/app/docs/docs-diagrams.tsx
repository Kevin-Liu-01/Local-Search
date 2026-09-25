import { LocalSearchLogo } from "@/components/brand-logo";
import Image from "next/image";
import {
  CheckIcon, ChromeColorBrandIcon,
  ClaudeBrandIcon, CodexBrandIcon, CursorBrandIcon, DocumentIcon,
  KeyIcon, ResultsIcon,
  SchemaIcon, ShieldIcon, SiteBrandIcon, StopIcon,
} from "@/components/icons";

export function BrowserBridge() {
  return <div className="docs-bridge" aria-label="Your agent sends lsearch commands to your browser. JSON or page text returns to the agent.">
    <div className="docs-bridge__nodes">
      <div className="docs-bridge__node"><div className="docs-agent-marks" aria-label="Claude, Codex, Cursor, and OpenClaw"><ClaudeBrandIcon size={28} /><CodexBrandIcon size={28} /><CursorBrandIcon size={28} /><span className="docs-openclaw" /></div><strong>Your agent</strong><span>Runs a command</span></div>
      <div className="docs-bridge__node docs-bridge__node--core"><LocalSearchLogo className="docs-bridge__logo" /><strong>lsearch</strong><span>Connects locally</span></div>
      <div className="docs-bridge__node"><ChromeColorBrandIcon size={38} /><strong>Your browser</strong><span>Opens the site</span></div>
    </div>
    <div className="docs-return"><span aria-hidden="true">←</span><SchemaIcon size={20} /><span>JSON or page text back to your agent</span></div>
  </div>;
}

export function SignedInDiagram() {
  return <div className="docs-browser-diagram">
    <div className="docs-browser-diagram__bar"><span><ChromeColorBrandIcon size={24} /> Your chosen browser</span><span><KeyIcon size={18} /> Your logins</span></div>
    <div className="docs-site-row">
      <div><span className="docs-logo-tile"><SiteBrandIcon name="linkedin" size={32} /></span><strong>LinkedIn</strong><span>Feed</span></div>
      <div><span className="docs-logo-tile"><SiteBrandIcon name="github" size={32} /></span><strong>GitHub</strong><span>Profile settings</span></div>
      <div><span className="docs-logo-tile"><SiteBrandIcon name="reddit" size={32} /></span><strong>Reddit</strong><span>Account settings</span></div>
    </div>
    <div className="docs-browser-diagram__result"><CheckIcon size={20} /><span>Read page → text + source URL</span></div>
  </div>;
}

export function SearchDiagram() {
  return <div className="docs-search-diagram" aria-label="Google, Bing, Brave, and DuckDuckGo return the same search fields.">
    <div className="docs-engine-row">
      {["Google", "Bing", "Brave", "DuckDuckGo"].map(name => <div key={name}><span className="docs-logo-tile"><Image src={`/brand/${name.toLowerCase()}.svg`} alt="" width={32} height={32} unoptimized /></span><span>{name}</span></div>)}
    </div>
    <div className="docs-field-output"><SchemaIcon size={24} /><strong>Same JSON fields</strong><div>{["rank", "title", "url", "domain", "snippet"].map(field => <code key={field}>{field}</code>)}</div></div>
  </div>;
}

export function ExtractDiagram() {
  return <div className="docs-extract-diagram">
    <div className="docs-example-page"><span><DocumentIcon size={20} /> Page links</span><div><strong>Setup guide</strong><span>https://example.com/start</span></div><div><strong>API docs</strong><span>https://example.com/api</span></div></div>
    <span className="docs-extract-arrow" aria-hidden="true">→</span>
    <div className="docs-example-record"><span><ResultsIcon size={20} /> Selected fields</span><pre>{'{\n  "title": "Setup guide",\n  "url": "https://example.com/start"\n}'}</pre><span>Example data · one record shown</span></div>
  </div>;
}

export function ActionDiagram() {
  return <ol className="docs-action-flow" aria-label="Page interaction steps">
    <li><DocumentIcon size={28} /><strong>1. Inspect</strong><span>Find the element.</span></li>
    <li><KeyIcon size={28} /><strong>2. Act</strong><span>Use its current ref.</span></li>
    <li><ShieldIcon size={28} /><strong>3. Check</strong><span>Read the new state.</span></li>
  </ol>;
}

export function DisconnectDiagram() {
  return <div className="docs-disconnect-diagram"><div><StopIcon size={28} /><strong>Agent access ends</strong></div><div><ChromeColorBrandIcon size={30} /><strong>Chrome stays open</strong></div><div><KeyIcon size={28} /><strong>You stay signed in</strong></div></div>;
}
