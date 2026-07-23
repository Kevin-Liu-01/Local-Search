import { AgentPlayground } from "@/components/agent-playground";
import { LocalSearchLogo } from "@/components/brand-logo";
import { CopyCommand } from "@/components/copy-command";
import {
  ArrowUpRightIcon,
  CheckIcon,
  ChevronRightIcon,
  GithubBrandIcon,
  GlobeIcon,
  CratesIoBrandIcon,
  RustOfficialIcon,
  SearchIcon,
  TerminalIcon,
} from "@/components/icons";

const sampleJson = `{
  "engine": "duckduckgo",
  "ok": true,
  "query": "rust browser automation libraries",
  "search": {
    "blocked": false,
    "results": [
      {
        "rank": 1,
        "title": "browser_automation — Rust web dev library",
        "domain": "lib.rs",
        "url": "https://lib.rs/crates/browser_automation",
        "snippet": "A modular Rust browser automation library…"
      }
    ]
  }
}`;

export default function Home() {
  return (
    <>
      <a className="skip-link" href="#main">Skip to content</a>
      <header className="site-header">
        <a className="brand" href="#top" aria-label="local-search home"><LocalSearchLogo className="brand-logo" />local-search</a>
        <nav aria-label="Main navigation">
          <a href="#demo">Demo</a>
          <a href="#output">Output</a>
          <a href="#benchmarks">Benchmarks</a>
          <a href="#compare">Compare</a>
        </nav>
        <a className="nav-cta" href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer">
          <GithubBrandIcon size={16} />
          <span>GitHub</span>
          <ArrowUpRightIcon size={14} />
        </a>
      </header>

      <main id="main">
        <section className="hero-shell" id="top" aria-labelledby="hero-title">
          <ReticleSpacer />
          <div className="hero-stage">
            <div className="hero-dither" aria-hidden="true">
              <i /><i /><i />
              <HeroIsometricArtwork />
            </div>
            <div className="hero">
              <p className="hero-kicker"><RustOfficialIcon size={20} /> Built with Rust</p>
              <h1 id="hero-title">
                Your browser is already
                <br className="hero-line-break" /> a search API.
              </h1>
              <p className="hero-copy">
                Give Claude Code, Codex, Cursor, or any shell-capable agent structured web search
                <br className="hero-line-break" /> through the browser on your machine. No API key. No metered search bill.
              </p>
              <div className="hero-actions">
                <a className="primary-button" href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer">
                  <GithubBrandIcon size={17} /> View on GitHub
                </a>
                <a className="secondary-button" href="https://crates.io/crates/local-search" target="_blank" rel="noreferrer">
                  <CratesIoBrandIcon size={22} /> crates.io
                </a>
              </div>
              <CopyCommand value="cargo install local-search" />
              <div className="hero-proof" aria-label="Product benefits">
                <span><CheckIcon size={14} /> Stable JSON</span>
                <span><CheckIcon size={14} /> Authenticated browser state</span>
                <span><CheckIcon size={14} /> Google, Bing & DuckDuckGo</span>
              </div>
            </div>
          </div>
          <ReticleSpacer />
        </section>

        <AgentPlayground />
        <ReticleSpacer />

        <section className="metric-strip" aria-label="local-search product summary">
          <article><b>$0</b><span>per local search</span></article>
          <article><b>3</b><span>search engines</span></article>
          <article><b>1</b><span>native binary</span></article>
          <article><b>0</b><span>API keys</span></article>
        </section>
        <ReticleSpacer />

        <section className="content-section" id="output">
          <div className="section-copy section-copy--centered">
            <p className="eyebrow">Structured output</p>
            <h2>One command. Clean JSON.</h2>
            <p>Search-page plumbing stays in the browser. Your agent gets only the normalized result set.</p>
          </div>
          <div className="output-stack">
            <CopyCommand value={'lsearch search "rust browser automation" --limit 3 --pretty'} />
            <div className="json-panel">
              <div className="panel-bar"><span><i /> search.json</span><span>stdout · application/json</span></div>
              <pre><code>{sampleJson}</code></pre>
            </div>
          </div>
        </section>
        <ReticleSpacer />

        <section className="content-section workflow-section" id="workflow">
          <div className="section-copy section-copy--centered">
            <p className="eyebrow">How it works</p>
            <h2>Search runs where you work.</h2>
            <p>No hosted search account sits between your agent and the browser session you already control.</p>
          </div>
          <div className="workflow-list">
            <article>
              <span className="workflow-index">01</span>
              <div className="workflow-icon"><TerminalIcon size={22} /></div>
              <div><h3>The agent calls <code>lsearch</code></h3><p>Any shell-capable coding agent can invoke the native CLI and request a stable result shape.</p></div>
              <code className="workflow-code">lsearch &quot;your query&quot;</code>
            </article>
            <article>
              <span className="workflow-index">02</span>
              <div className="workflow-icon"><GlobeIcon size={22} /></div>
              <div><h3>Your local browser searches</h3><p>A managed Chrome profile opens DuckDuckGo, Google, or Bing with your regional and signed-in context.</p></div>
              <span className="workflow-state"><i /> Local Chrome</span>
            </article>
            <article>
              <span className="workflow-index">03</span>
              <div className="workflow-icon"><SearchIcon size={22} /></div>
              <div><h3>Only useful fields come back</h3><p>Rank, title, URL, domain, snippet, and optional page content are normalized into JSON.</p></div>
              <span className="workflow-state"><CheckIcon size={13} /> Schema valid</span>
            </article>
          </div>
        </section>
        <ReticleSpacer />

        <section className="benchmark-section" id="benchmarks">
          <div className="section-copy section-copy--centered section-copy--light">
            <p className="eyebrow">Measured context savings</p>
            <h2>96.5% less context.</h2>
            <p>Normalized search JSON keeps full rendered-page snapshots out of the agent’s visible context.</p>
          </div>
          <div className="benchmark-card">
            <div className="chart-legend"><span><i className="legend-local" />local-search</span><span><i className="legend-snapshot" />interactive snapshot</span></div>
            <div className="chart-row">
              <div><b>3 results</b><small>median visible tokens</small></div>
              <div className="bar-stack"><span className="bar-snapshot"><i>8,760.5</i></span><span className="bar-local bar-local--3"><i>309</i></span></div>
              <strong>96.5%<small>less</small></strong>
            </div>
            <div className="chart-row">
              <div><b>10 results</b><small>median visible tokens</small></div>
              <div className="bar-stack"><span className="bar-snapshot bar-snapshot--10"><i>8,708</i></span><span className="bar-local bar-local--10"><i>891</i></span></div>
              <strong>89.8%<small>less</small></strong>
            </div>
            <div className="benchmark-facts">
              <article><b>72/72</b><span>cross-engine searches fulfilled the requested result count</span></article>
              <article><b>60/60</b><span>schema-valid stability runs; 58 kept identical top-three URLs</span></article>
              <article><b>72/72</b><span>content pages returned the full 1,200-character text cap</span></article>
            </div>
          </div>
          <p className="methodology">
            Source build, July 21, 2026 · 12 queries × 3 engines × 2 result depths · visible command text and stdout compared with compact interactive snapshots using o200k_base. <a href="https://github.com/Kevin-Liu-01/local-search#token-benchmarks" target="_blank" rel="noreferrer">Read the methodology ↗</a>
          </p>
        </section>
        <ReticleSpacer />

        <section className="content-section comparison-section" id="compare">
          <div className="section-copy section-copy--centered">
            <p className="eyebrow">Matched hosted benchmark</p>
            <h2>Faster, smaller, and zero API credits.</h2>
            <p>The same 12 queries and result depths ran sequentially through local-search, Exa, Brave Search, Tavily, and Firecrawl.</p>
          </div>
          <div className="comparison-table-wrap">
            <table>
              <thead><tr><th>Provider</th><th>Depth fulfilled</th><th>Median raw tokens</th><th>Normalized / result</th><th>Median latency</th><th>24-run usage</th></tr></thead>
              <tbody>
                <tr className="featured-row"><th><LocalSearchLogo className="brand-logo brand-logo--table" />local-search</th><td>24/24</td><td>413.5</td><td>53.4</td><td>148.7 ms</td><td>$0</td></tr>
                <tr><th>Exa</th><td>24/24</td><td>4,472.5</td><td>881.2</td><td>501.9 ms</td><td>$0.324</td></tr>
                <tr><th>Brave Search</th><td>24/24</td><td>13,104</td><td>108.6</td><td>322.0 ms</td><td>$0.120</td></tr>
                <tr><th>Tavily</th><td>17/24</td><td>1,163</td><td>259.5</td><td>1,184.4 ms</td><td>24 credits · $0.192 PAYG</td></tr>
                <tr><th>Firecrawl</th><td>24/24</td><td>509</td><td>74.8</td><td>1,520.8 ms</td><td>48 credits</td></tr>
              </tbody>
            </table>
          </div>
          <p className="comparison-note">Request plus response tokens use o200k_base. local-search: 384.5 ms cold median and 6.5 ms warm-cache median. Provider pricing published July 21, 2026. <a href="https://github.com/Kevin-Liu-01/local-search/tree/main/benchmarks" target="_blank" rel="noreferrer">Full benchmark and runner ↗</a></p>
        </section>
        <ReticleSpacer />

        <section className="closing-cta">
          <p className="eyebrow">Your browser. Your search.</p>
          <h2>Give your agent the web without another bill.</h2>
          <CopyCommand value="cargo install local-search" />
          <div className="closing-links">
            <a href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer">Get started on GitHub <ChevronRightIcon size={16} /></a>
            <a href="https://crates.io/crates/local-search" target="_blank" rel="noreferrer">View the crate <ChevronRightIcon size={16} /></a>
          </div>
        </section>
      </main>

      <ReticleSpacer />
      <footer className="site-footer">
        <a className="brand" href="#top"><LocalSearchLogo className="brand-logo" />local-search</a>
        <p>Free structured web search for agents, powered by your local browser.</p>
        <div>
          <a href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer">GitHub ↗</a>
          <a href="https://crates.io/crates/local-search" target="_blank" rel="noreferrer">crates.io ↗</a>
          <a href="https://rustfoundation.org/policy/rust-trademark-policy/" target="_blank" rel="noreferrer">Rust/Cargo marks · CC BY ↗</a>
          <a href="https://github.com/Kevin-Liu-01" target="_blank" rel="noreferrer">Built by Kevin Liu ↗</a>
        </div>
      </footer>
    </>
  );
}

function ReticleSpacer() {
  return (
    <div className="hero-spacer" aria-hidden="true">
      <i className="reticle reticle--tl" />
      <i className="reticle reticle--tr" />
      <i className="reticle reticle--bl" />
      <i className="reticle reticle--br" />
    </div>
  );
}

function HeroIsometricArtwork() {
  return (
    <div className="hero-isometric-art">
      <svg className="hero-isometric-scene" viewBox="0 0 620 500">
        <g className="hero-iso-hexes">
          <polygon points="292,82 301,87 301,97 292,102 283,97 283,87" />
          <polygon className="is-filled" points="310,98 319,103 319,113 310,118 301,113 301,103" />
          <polygon points="292,114 301,119 301,129 292,134 283,129 283,119" />
          <polygon points="564,24 574,30 574,41 564,47 554,41 554,30" />
          <polygon points="584,42 594,48 594,59 584,65 574,59 574,48" />
          <polygon className="is-filled" points="564,60 574,66 574,77 564,83 554,77 554,66" />
          <polygon points="530,408 540,414 540,425 530,431 520,425 520,414" />
          <polygon className="is-filled" points="550,426 560,432 560,443 550,449 540,443 540,432" />
          <polygon points="530,444 540,450 540,461 530,467 520,461 520,450" />
          <circle cx="310" cy="139" r="2" />
          <circle cx="584" cy="76" r="2" />
          <circle cx="550" cy="461" r="2" />
        </g>

        <g className="hero-iso-shadows">
          <polygon points="365,304 450,255 535,304 450,353" />
          <polygon points="96,440 210,374 316,435 202,500" />
        </g>

        <g className="hero-iso-wires hero-iso-wires--base">
          <path d="M306 369 330 383v61l242-140V97l-23-13" />
          <path d="M306 387 342 408v48l242-140V204l-35-20" />
          <path d="M306 405 354 433v35l242-140v-17l-47-27" />
        </g>
        <g className="hero-iso-wires hero-iso-wires--signal">
          <path d="M306 369 330 383v61l242-140V97l-23-13" />
          <path d="M306 387 342 408v48l242-140V204l-35-20" />
          <path d="M306 405 354 433v35l242-140v-17l-47-27" />
        </g>
        <g className="hero-iso-engine hero-iso-engine--duck">
          <polygon className="hero-iso-engine__side-left" points="434,332 351,284 351,296 434,344" />
          <polygon className="hero-iso-engine__side-center" points="466,332 434,332 434,344 466,344" />
          <polygon className="hero-iso-engine__side-right" points="549,284 466,332 466,344 549,296" />
          <polygon className="hero-iso-engine__top" points="434,218 466,218 549,266 549,284 466,332 434,332 351,284 351,266" />
          <image className="hero-iso-engine__mark" href="/brand/duckduckgo.svg" width="58" height="58" transform="matrix(.866 .5 -.866 .5 450 246)" />
        </g>
        <g className="hero-iso-engine hero-iso-engine--bing">
          <polygon className="hero-iso-engine__side-left" points="434,232 351,184 351,196 434,244" />
          <polygon className="hero-iso-engine__side-center" points="466,232 434,232 434,244 466,244" />
          <polygon className="hero-iso-engine__side-right" points="549,184 466,232 466,244 549,196" />
          <polygon className="hero-iso-engine__top" points="434,118 466,118 549,166 549,184 466,232 434,232 351,184 351,166" />
          <image className="hero-iso-engine__mark" href="/brand/bing.svg" width="64" height="64" transform="matrix(.866 .5 -.866 .5 450 143)" />
        </g>
        <g className="hero-iso-engine hero-iso-engine--google">
          <polygon className="hero-iso-engine__side-left" points="434,132 351,84 351,96 434,144" />
          <polygon className="hero-iso-engine__side-center" points="466,132 434,132 434,144 466,144" />
          <polygon className="hero-iso-engine__side-right" points="549,84 466,132 466,144 549,96" />
          <polygon className="hero-iso-engine__top" points="434,18 466,18 549,66 549,84 466,132 434,132 351,84 351,66" />
          <image className="hero-iso-engine__mark" href="/brand/google.svg" width="58" height="58" transform="matrix(.866 .5 -.866 .5 450 46)" />
        </g>

        <g className="hero-iso-agent">
          <polygon className="hero-iso-agent__left" points="84,357 179,412 179,484 84,429" />
          <polygon className="hero-iso-agent__front" points="179,412 306,339 306,411 179,484" />
          <polygon className="hero-iso-agent__top" points="84,357 211,284 306,339 179,412" />
          <path className="hero-iso-agent__groove" d="m113 357 98-56 66 38-98 56zM128 366l83-48 50 29-83 48z" />
          <circle cx="247" cy="331" r="4" /><circle cx="260" cy="338" r="4" /><circle cx="273" cy="346" r="4" />
          <g className="hero-iso-tape">
            <polygon points="190,420 288,363 288,398 190,455" />
            <LocalSearchLogo preserveAspectRatio="none" x="0" y="0" width="72" height="20" transform="matrix(.866 -.5 0 1 207 424)" />
          </g>
          <path className="hero-iso-agent__vents" d="m209 457 53-31m-45 36 53-31m-45 36 53-31" />
        </g>

        <g className="hero-iso-ports">
          <polygon points="549,78 554,81 554,87 549,90 544,87 544,81" />
          <polygon points="549,178 554,181 554,187 549,190 544,187 544,181" />
          <polygon points="549,278 554,281 554,287 549,290 544,287 544,281" />
          <polygon points="306,363 311,366 311,372 306,375 301,372 301,366" />
          <polygon points="306,381 311,384 311,390 306,393 301,390 301,384" />
          <polygon points="306,399 311,402 311,408 306,411 301,408 301,402" />
        </g>
      </svg>
    </div>
  );
}
