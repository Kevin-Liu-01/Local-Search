import { AgentPlayground } from "@/components/agent-playground";
import { AgentCopyButton } from "@/components/agent-copy-button";
import { LocalSearchLogo } from "@/components/brand-logo";
import { CopyCommand } from "@/components/copy-command";
import { InstallCommand } from "@/components/install-command";
import { FAQS, RELEASE_AUDIT } from "@/lib/seo";
import type { ReactNode } from "react";
import {
  ArrowUpRightIcon,
  CheckIcon,
  ChevronRightIcon,
  BraveBrandIcon,
  ClockIcon,
  CompactIcon,
  ChromiumBrandIcon,
  DocumentIcon,
  ExaBrandIcon,
  FirecrawlBrandIcon,
  GithubBrandIcon,
  GlobeIcon,
  CratesIoBrandIcon,
  JsonSchemaBrandIcon,
  RustOfficialIcon,
  ResultsIcon,
  SchemaIcon,
  SearchIcon,
  TavilyBrandIcon,
  TerminalIcon,
  ZeroCostIcon,
} from "@/components/icons";

function FaqAsset({ src, className = "" }: { src: string; className?: string }) {
  return <span className={`faq-asset ${className}`.trim()} style={{ backgroundImage: `url(${src})` }} />;
}

function FaqVisual({ index }: { index: number }) {
  if (index === 0) {
    return (
      <span className="faq-card__visual faq-card__visual--product" aria-hidden="true">
        <span className="faq-visual__plate faq-visual__plate--local"><LocalSearchLogo /></span>
      </span>
    );
  }

  if (index === 1) {
    return (
      <span className="faq-card__visual faq-card__visual--api" aria-hidden="true">
        <span className="faq-visual__plate faq-visual__plate--api"><FaqAsset src="/brand/api.svg" /></span>
      </span>
    );
  }

  if (index === 2) {
    return (
      <span className="faq-card__visual faq-card__visual--agents" aria-hidden="true">
        <span className="faq-visual__tile faq-visual__tile--claude"><FaqAsset src="/brand/claude.svg" /></span>
        <span className="faq-visual__tile faq-visual__tile--codex"><FaqAsset src="/brand/codex.svg" /></span>
        <span className="faq-visual__tile faq-visual__tile--cursor"><FaqAsset src="/brand/cursor-mono.svg" /></span>
        <span className="faq-visual__tile faq-visual__tile--openclaw"><FaqAsset src="/brand/openclaw.svg" /></span>
      </span>
    );
  }

  if (index === 3) {
    return (
      <span className="faq-card__visual faq-card__visual--engines" aria-hidden="true">
        <span className="faq-visual__tile faq-visual__tile--google"><FaqAsset src="/brand/google.svg" /></span>
        <span className="faq-visual__tile faq-visual__tile--bing"><FaqAsset src="/brand/bing.svg" /></span>
        <span className="faq-visual__tile faq-visual__tile--duck"><FaqAsset src="/brand/duckduckgo.svg" /></span>
        <span className="faq-visual__tile faq-visual__tile--brave"><FaqAsset src="/brand/brave.svg" /></span>
      </span>
    );
  }

  if (index === 4) {
    return (
      <span className="faq-card__visual faq-card__visual--benchmark" aria-hidden="true">
        <span className="faq-visual__provider faq-visual__provider--exa"><FaqAsset src="/brand/exa.svg" /></span>
        <span className="faq-visual__provider faq-visual__provider--brave"><FaqAsset src="/brand/brave.svg" /></span>
        <span className="faq-visual__provider faq-visual__provider--tavily"><FaqAsset src="/brand/tavily.svg" /></span>
        <span className="faq-visual__provider faq-visual__provider--firecrawl"><FaqAsset src="/brand/firecrawl.png" /></span>
      </span>
    );
  }

  if (index === 5) {
    return (
      <span className="faq-card__visual faq-card__visual--credentials" aria-hidden="true">
        <span className="faq-visual__plate faq-visual__plate--credentials"><FaqAsset src="/brand/credentials.svg" /></span>
      </span>
    );
  }

  if (index === 6) {
    return (
      <span className="faq-card__visual faq-card__visual--rust" aria-hidden="true">
        <span className="faq-visual__plate faq-visual__plate--rust"><FaqAsset src="/brand/rust-mono.svg" /></span>
      </span>
    );
  }

  return (
    <span className="faq-card__visual faq-card__visual--open-source" aria-hidden="true">
      <span className="faq-visual__plate faq-visual__plate--osi"><FaqAsset src="/brand/open-source.svg" /></span>
    </span>
  );
}

const sampleJson = JSON.stringify({
  engine: "duckduckgo",
  ok: true,
  query: "rust browser automation libraries",
  search: {
    blocked: false,
    results: [
      {
        rank: 1,
        title: "browser_automation — Rust web dev library",
        domain: "lib.rs",
        url: "https://lib.rs/crates/browser_automation",
        snippet: "A modular Rust browser automation library…",
      },
    ],
  },
}, null, 4);

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
          <a href="#faq">FAQ</a>
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
              <h1 id="hero-title">
                <span className="hero-title__line hero-title__line--primary">Browser Search API</span>
                <span className="hero-title__line hero-title__line--secondary">No API Key, No Billing</span>
              </h1>
              <p className="hero-copy">
                Give Claude Code, Codex, Cursor, or any shell-capable agent structured web search
                <br className="hero-line-break" /> through the browser on your machine. No API key. No metered search bill.
              </p>
              <div className="hero-actions">
                <a className="primary-button" href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer">
                  <GithubBrandIcon size={17} /> View on GitHub
                </a>
                <AgentCopyButton />
                <a className="secondary-button" href="https://crates.io/crates/local-search" target="_blank" rel="noreferrer">
                  <CratesIoBrandIcon size={22} /> crates.io
                </a>
              </div>
              <InstallCommand />
              <div className="hero-proof" aria-label="Product benefits">
                <span><RustOfficialIcon size={15} /> Built with Rust</span>
                <span><JsonSchemaBrandIcon size={16} /> Stable JSON</span>
                <span><ChromiumBrandIcon size={16} /> Authenticated browser state</span>
              </div>
            </div>
          </div>
          <ReticleSpacer />
        </section>

        <AgentPlayground />
        <ReticleSpacer />

        <section className="metric-strip" aria-label="local-search product summary">
          <article><b>$0</b><span>per local search</span></article>
          <article><b>4</b><span>search engines</span></article>
          <article><b>50.3 KiB</b><span>crates.io package</span></article>
          <article><b>0</b><span>API keys</span></article>
        </section>
        <ReticleSpacer />

        <section className="content-section output-workflow-section" id="output">
          <div className="output-bento" id="workflow">
            <div className="output-bento__intro">
              <h2><span>Ask. Search.</span><span>Return JSON.</span></h2>
              <p><code>lsearch</code> bridges a coding agent and the browser already on your machine. One shell call goes in; compact search context comes back.</p>
            </div>
            <div className="output-bento__command">
              <span className="bento-label">One native command</span>
              <h3>Search from any shell-capable agent.</h3>
              <CopyCommand value={'lsearch search "rust browser automation" --limit 3 --pretty'} />
              <p>No SDK, API key, hosted search account, or agent-specific integration.</p>
            </div>
            <div className="workflow-timeline">
              <svg className="workflow-circuit workflow-circuit--desktop" viewBox="0 0 1120 96" preserveAspectRatio="none" aria-hidden="true">
                <g className="workflow-circuit__base">
                  <path d="M0 55h148l28-16h216l28 16h102l28-16h216l28 16h102l28-16h196" />
                  <path d="M0 67h160l28-16h216l28 16h102l28-16h216l28 16h102l28-16h184" />
                  <path d="M0 79h172l28-16h216l28 16h102l28-16h216l28 16h102l28-16h172" />
                </g>
                <g className="workflow-circuit__signal">
                  <path d="M0 55h148l28-16h216l28 16h102l28-16h216l28 16h102l28-16h196" />
                  <path d="M0 67h160l28-16h216l28 16h102l28-16h216l28 16h102l28-16h184" />
                  <path d="M0 79h172l28-16h216l28 16h102l28-16h216l28 16h102l28-16h172" />
                </g>
                <g className="workflow-circuit__ports">
                  <polygon points="230,33 235,36 235,42 230,45 225,42 225,36" />
                  <polygon points="230,45 235,48 235,54 230,57 225,54 225,48" />
                  <polygon points="230,57 235,60 235,66 230,69 225,66 225,60" />
                  <polygon points="603,33 608,36 608,42 603,45 598,42 598,36" />
                  <polygon points="603,45 608,48 608,54 603,57 598,54 598,48" />
                  <polygon points="603,57 608,60 608,66 603,69 598,66 598,60" />
                  <polygon points="976,33 981,36 981,42 976,45 971,42 971,36" />
                  <polygon points="976,45 981,48 981,54 976,57 971,54 971,48" />
                  <polygon points="976,57 981,60 981,66 976,69 971,66 971,60" />
                </g>
              </svg>
              <svg className="workflow-circuit workflow-circuit--mobile" viewBox="0 0 96 660" preserveAspectRatio="none" aria-hidden="true">
                <g className="workflow-circuit__base">
                  <path d="M30 0v60l16 20v140l-16 20v160l16 20v140l-16 20v80" />
                  <path d="M42 0v60l16 20v140l-16 20v160l16 20v140l-16 20v80" />
                  <path d="M54 0v60l16 20v140l-16 20v160l16 20v140l-16 20v80" />
                </g>
                <g className="workflow-circuit__signal">
                  <path d="M30 0v60l16 20v140l-16 20v160l16 20v140l-16 20v80" />
                  <path d="M42 0v60l16 20v140l-16 20v160l16 20v140l-16 20v80" />
                  <path d="M54 0v60l16 20v140l-16 20v160l16 20v140l-16 20v80" />
                </g>
              </svg>
              <div className="workflow-list" aria-label="How local-search works">
                <article>
                  <div className="workflow-top"><span className="workflow-index">Agent</span><div className="workflow-icon"><TerminalIcon size={22} /></div></div>
                  <div><h3>Run one command</h3><p>Any shell-capable agent runs <code>lsearch</code>.</p></div>
                  <code className="workflow-code">query + limit</code>
                </article>
                <article>
                  <div className="workflow-top"><span className="workflow-index">Browser</span><div className="workflow-icon"><GlobeIcon size={22} /></div></div>
                  <div><h3>Search locally</h3><p>Managed Chrome searches with your local state.</p></div>
                  <span className="workflow-state"><i /> Runs locally</span>
                </article>
                <article>
                  <div className="workflow-top"><span className="workflow-index">Agent context</span><div className="workflow-icon"><SearchIcon size={22} /></div></div>
                  <div><h3>Return clean JSON</h3><p>Only stable result fields reach the agent.</p></div>
                  <span className="workflow-state"><CheckIcon size={13} /> Compact + valid</span>
                </article>
              </div>
            </div>
            <div className="output-bento__json-copy">
              <span className="bento-label">Returned agent context</span>
              <h3>Stable fields, not search-page chrome.</h3>
              <p>The browser stays local. Your agent receives only the result data it can act on.</p>
              <div className="json-field-list" aria-label="Returned JSON fields"><code>rank</code><code>title</code><code>url</code><code>domain</code><code>snippet</code><code>content?</code></div>
            </div>
            <div className="json-panel">
              <div className="panel-bar"><span><i /> returned to the agent</span><span>stdout · application/json</span></div>
              <pre><JsonCode value={sampleJson} /></pre>
            </div>
          </div>
        </section>
        <ReticleSpacer />

        <section className="content-section performance-section" id="benchmarks">
          <div className="performance-intro">
            <div>
              <h2><span>Searches, tokens.</span><span>Latency and cost.</span></h2>
            </div>
            <p>Two search benchmarks cover 72 local searches and 24 matched requests per provider. A separate July 27 release audit measured a {RELEASE_AUDIT.binarySize} arm64 macOS binary and {RELEASE_AUDIT.startupReduction} faster warm CLI startup.</p>
          </div>
          <div className="benchmark-card">
            <div className="benchmark-chapter benchmark-chapter--tokens">
              <div className="benchmark-chapter__copy">
                <div className="benchmark-block-heading">
                  <div><span>Benchmark 01</span><h3>Local JSON vs. rendered search-page snapshots</h3></div>
                  <p>36 searches per depth · 12 queries × 3 engines</p>
                </div>
                <div className="benchmark-facts" aria-label="Local search reliability checks">
                  <article>
                    <div className="benchmark-fact-icon"><ResultsIcon size={18} /></div>
                    <div className="benchmark-fact-copy"><span>Requested depth fulfilled</span><p>Every cross-engine search returned the requested number of results.</p></div>
                    <b>72/72</b>
                  </article>
                  <article>
                    <div className="benchmark-fact-icon"><SchemaIcon size={18} /></div>
                    <div className="benchmark-fact-copy"><span>Schema-valid stability runs</span><p>58/60 repeated runs also kept the exact same top-three URLs.</p></div>
                    <b>60/60</b>
                  </article>
                  <article>
                    <div className="benchmark-fact-icon"><DocumentIcon size={18} /></div>
                    <div className="benchmark-fact-copy"><span>Content pages extracted</span><p>Every page returned the full configured 1,200-character text cap.</p></div>
                    <b>72/72</b>
                  </article>
                </div>
              </div>
              <div className="benchmark-chapter__visual">
                <div className="token-chart" aria-label="Visible token comparison">
                  <div className="token-chart__legend">
                    <span><i className="token-key token-key--local" />local-search JSON</span>
                    <span><i className="token-key token-key--snapshot" />interactive page snapshot</span>
                    <em>Visible command + stdout · o200k_base tokens</em>
                  </div>
                  <div className="token-row">
                    <div className="token-row__label"><b>3 results</b><span>36/36 usable searches</span></div>
                    <div className="token-series">
                      <div><span>Page snapshot</span><i className="token-bar token-bar--snapshot" /><b>8,760.5</b></div>
                      <div><span>local-search</span><i className="token-bar token-bar--local-3" /><b>309</b></div>
                    </div>
                    <div className="token-reduction"><b>96.5%</b><span>less context</span></div>
                  </div>
                  <div className="token-row">
                    <div className="token-row__label"><b>10 results</b><span>36/36 usable searches</span></div>
                    <div className="token-series">
                      <div><span>Page snapshot</span><i className="token-bar token-bar--snapshot-10" /><b>8,708</b></div>
                      <div><span>local-search</span><i className="token-bar token-bar--local-10" /><b>891</b></div>
                    </div>
                    <div className="token-reduction"><b>89.8%</b><span>less context</span></div>
                  </div>
                </div>
              </div>
            </div>
            <div className="benchmark-chapter benchmark-chapter--providers" id="compare">
              <div className="benchmark-chapter__copy">
                <div className="comparison-context">
                  <div><span>Benchmark 02</span><h3>local-search vs. hosted search APIs</h3></div>
                  <p>12 identical queries × two depths · 24 requests per provider</p>
                </div>
                <div className="provider-highlights" aria-label="local-search hosted benchmark summary">
                  <article><div className="provider-highlight-icon"><ResultsIcon size={18} /></div><span>Depth fulfilled</span><b>24/24</b></article>
                  <article><div className="provider-highlight-icon"><CompactIcon size={18} /></div><span>Normalized size</span><b>53.4 <small>tokens / result</small></b></article>
                  <article><div className="provider-highlight-icon"><ClockIcon size={18} /></div><span>Median latency</span><b>148.7 <small>ms</small></b></article>
                  <article><div className="provider-highlight-icon"><ZeroCostIcon size={18} /></div><span>API usage</span><b>$0</b></article>
                </div>
              </div>
              <div className="benchmark-chapter__visual benchmark-chapter__visual--providers">
                <div className="comparison-table-wrap">
                  <table>
                    <thead><tr><th>Provider</th><th>Depth fulfilled</th><th>Request + response tokens</th><th>Tokens / result</th><th>Latency</th><th>24-request usage</th></tr></thead>
                    <tbody>
                      <tr className="featured-row">
                        <th scope="row"><span className="provider-name provider-name--winner"><LocalSearchLogo className="brand-logo brand-logo--table" /><span>local-search</span><em>Best overall</em></span></th>
                        <td data-label="Depth fulfilled">24/24</td>
                        <td data-label="Request + response tokens"><strong>413.5</strong><small>fewest</small></td>
                        <td data-label="Tokens / result"><strong>53.4</strong><small>fewest</small></td>
                        <td data-label="Latency"><strong>148.7 ms</strong><small>fastest</small></td>
                        <td data-label="24-request usage"><strong>$0</strong><small>no credits</small></td>
                      </tr>
                      <tr><th scope="row"><span className="provider-name"><ExaBrandIcon size={22} />Exa</span></th><td data-label="Depth fulfilled">24/24</td><td data-label="Request + response tokens">4,472.5</td><td data-label="Tokens / result">881.2</td><td data-label="Latency">501.9 ms</td><td data-label="24-request usage">$0.324</td></tr>
                      <tr><th scope="row"><span className="provider-name"><BraveBrandIcon size={22} />Brave Search</span></th><td data-label="Depth fulfilled">24/24</td><td data-label="Request + response tokens">13,104</td><td data-label="Tokens / result">108.6</td><td data-label="Latency">322.0 ms</td><td data-label="24-request usage">$0.120</td></tr>
                      <tr><th scope="row"><span className="provider-name"><TavilyBrandIcon size={22} />Tavily</span></th><td data-label="Depth fulfilled">17/24</td><td data-label="Request + response tokens">1,163</td><td data-label="Tokens / result">259.5</td><td data-label="Latency">1,184.4 ms</td><td data-label="24-request usage">24 credits · $0.192 PAYG</td></tr>
                      <tr><th scope="row"><span className="provider-name"><FirecrawlBrandIcon size={22} />Firecrawl</span></th><td data-label="Depth fulfilled">24/24</td><td data-label="Request + response tokens">509</td><td data-label="Tokens / result">74.8</td><td data-label="Latency">1,520.8 ms</td><td data-label="24-request usage">48 credits · ≈$0.154 Hobby equivalent</td></tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
            <p className="methodology">Search source build, July 21, 2026. Benchmark 01 compares visible command text plus stdout with compact interactive snapshots. Benchmark 02 counts serialized requests plus raw responses; all token counts use o200k_base. local-search measured 384.5 ms cold and 6.5 ms from its five-minute local cache. The July 27 native audit used Rust {RELEASE_AUDIT.rustVersion} on {RELEASE_AUDIT.platform}; startup is process launch time, not search latency. <a href="https://github.com/Kevin-Liu-01/local-search/tree/main/benchmarks" target="_blank" rel="noreferrer">Full methodology and runner ↗</a> · <a href="/benchmarks.json">Machine-readable data ↗</a></p>
          </div>
        </section>
        <ReticleSpacer />

        <section className="content-section faq-section" id="faq" aria-labelledby="faq-title">
          <header className="faq-header">
            <h2 id="faq-title">Frequently asked questions.</h2>
            <p>Quick answers about installation, compatible agents, search engines, privacy, and how local-search compares.</p>
          </header>
          <div className="faq-grid">
            {FAQS.map(({ question, answer }, index) => {
              const questionId = `faq-question-${index + 1}`;
              return (
                <article key={question} aria-labelledby={questionId}>
                  <span className="faq-index">{String(index + 1).padStart(2, "0")}</span>
                  <h3 id={questionId}>{question}</h3>
                  <p>{answer}</p>
                  <FaqVisual index={index} />
                </article>
              );
            })}
          </div>
        </section>
        <ReticleSpacer />

        <section className="closing-cta">
          <h2><span>Give agents the web.</span><span>Skip search bills.</span></h2>
          <InstallCommand />
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
          <a href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer"><span>GitHub</span><ArrowUpRightIcon size={11} /></a>
          <a href="https://crates.io/crates/local-search" target="_blank" rel="noreferrer"><span>crates.io</span><ArrowUpRightIcon size={11} /></a>
          <a href="https://www.npmjs.com/package/@kevinliu01/localsearch" target="_blank" rel="noreferrer"><span>npm</span><ArrowUpRightIcon size={11} /></a>
          <a href="https://rustfoundation.org/policy/rust-trademark-policy/" target="_blank" rel="noreferrer"><span>Rust/Cargo attribution</span><ArrowUpRightIcon size={11} /></a>
          <a href="https://github.com/Kevin-Liu-01" target="_blank" rel="noreferrer"><span>Built by Kevin Liu</span><ArrowUpRightIcon size={11} /></a>
        </div>
      </footer>
    </>
  );
}

function JsonCode({ value }: { value: string }) {
  const pattern = /("(?:\\.|[^"\\])*")(?=\s*:)|("(?:\\.|[^"\\])*")|(-?\d+(?:\.\d+)?)|\b(true|false|null)\b|([{}\[\],:])/g;
  const tokens: ReactNode[] = [];
  let cursor = 0;
  let match: RegExpExecArray | null;

  while ((match = pattern.exec(value)) !== null) {
    if (match.index > cursor) tokens.push(value.slice(cursor, match.index));
    const kind = match[1] ? "key" : match[2] ? "string" : match[3] ? "number" : match[4] ? "literal" : "punctuation";
    tokens.push(<span className={`json-token json-token--${kind}`} key={`${match.index}-${kind}`}>{match[0]}</span>);
    cursor = pattern.lastIndex;
  }

  if (cursor < value.length) tokens.push(value.slice(cursor));
  return <code>{tokens}</code>;
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
          <g transform="translate(0 24)">
            <polygon points="530,408 540,414 540,425 530,431 520,425 520,414" />
            <polygon className="is-filled" points="550,426 560,432 560,443 550,449 540,443 540,432" />
            <polygon points="530,444 540,450 540,461 530,467 520,461 520,450" />
            <circle cx="550" cy="461" r="2" />
          </g>
          <circle cx="310" cy="139" r="2" />
          <circle cx="584" cy="76" r="2" />
        </g>

        <g className="hero-iso-shadows">
          <polygon points="365,392 450,343 535,392 450,441" />
          <polygon points="96,440 210,374 316,435 202,500" />
        </g>

        <g className="hero-iso-wires hero-iso-wires--base">
          <path d="M306 351l60 35v20l54 31 150-87V96l-21-12" />
          <path d="M306 369l54 31v26l54 31 164-95V189l-29-17" />
          <path d="M306 387l48 28v31l54 31 178-103V281l-37-21" />
          <path d="M306 405l42 24v37l54 31 192-111V374l-45-26" />
        </g>
        <g className="hero-iso-wires hero-iso-wires--signal">
          <path d="M306 351l60 35v20l54 31 150-87V96l-21-12" />
          <path d="M306 369l54 31v26l54 31 164-95V189l-29-17" />
          <path d="M306 387l48 28v31l54 31 178-103V281l-37-21" />
          <path d="M306 405l42 24v37l54 31 192-111V374l-45-26" />
        </g>
        <g className="hero-iso-engine hero-iso-engine--duck">
          <polygon className="hero-iso-engine__side-left" points="434,396 351,348 351,360 434,408" />
          <polygon className="hero-iso-engine__side-center" points="466,396 434,396 434,408 466,408" />
          <polygon className="hero-iso-engine__side-right" points="549,348 466,396 466,408 549,360" />
          <polygon className="hero-iso-engine__top" points="434,282 466,282 549,330 549,348 466,396 434,396 351,348 351,330" />
          <image className="hero-iso-engine__mark" href="/brand/duckduckgo.svg" width="58" height="58" transform="matrix(.866 .5 -.866 .5 450 310)" />
        </g>
        <g className="hero-iso-engine hero-iso-engine--brave">
          <polygon className="hero-iso-engine__side-left" points="434,308 351,260 351,272 434,320" />
          <polygon className="hero-iso-engine__side-center" points="466,308 434,308 434,320 466,320" />
          <polygon className="hero-iso-engine__side-right" points="549,260 466,308 466,320 549,272" />
          <polygon className="hero-iso-engine__top" points="434,194 466,194 549,242 549,260 466,308 434,308 351,260 351,242" />
          <image className="hero-iso-engine__mark" href="/brand/brave.svg" width="58" height="58" transform="matrix(.866 .5 -.866 .5 450 222)" />
        </g>
        <g className="hero-iso-engine hero-iso-engine--bing">
          <polygon className="hero-iso-engine__side-left" points="434,220 351,172 351,184 434,232" />
          <polygon className="hero-iso-engine__side-center" points="466,220 434,220 434,232 466,232" />
          <polygon className="hero-iso-engine__side-right" points="549,172 466,220 466,232 549,184" />
          <polygon className="hero-iso-engine__top" points="434,106 466,106 549,154 549,172 466,220 434,220 351,172 351,154" />
          <image className="hero-iso-engine__mark" href="/brand/bing.svg" width="64" height="64" transform="matrix(.866 .5 -.866 .5 450 131)" />
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
          <g className="hero-iso-agent__lights" transform="translate(-10 -6)" aria-hidden="true">
            <g className="hero-iso-agent__light hero-iso-agent__light--active">
              <ellipse className="hero-iso-agent__light-side" cx="247" cy="332.4" rx="5.2" ry="3" />
              <ellipse className="hero-iso-agent__light-top" cx="247" cy="330.8" rx="5.2" ry="3" />
              <ellipse className="hero-iso-agent__light-glint" cx="245.2" cy="329.8" rx="1.25" ry=".65" />
            </g>
            <g className="hero-iso-agent__light">
              <ellipse className="hero-iso-agent__light-side" cx="260" cy="339.9" rx="5.2" ry="3" />
              <ellipse className="hero-iso-agent__light-top" cx="260" cy="338.3" rx="5.2" ry="3" />
              <ellipse className="hero-iso-agent__light-glint" cx="258.2" cy="337.3" rx="1.25" ry=".65" />
            </g>
            <g className="hero-iso-agent__light hero-iso-agent__light--dim">
              <ellipse className="hero-iso-agent__light-side" cx="273" cy="347.4" rx="5.2" ry="3" />
              <ellipse className="hero-iso-agent__light-top" cx="273" cy="345.8" rx="5.2" ry="3" />
              <ellipse className="hero-iso-agent__light-glint" cx="271.2" cy="344.8" rx="1.25" ry=".65" />
            </g>
          </g>
          <g className="hero-iso-tape">
            <polygon points="190,420 288,363 288,398 190,455" />
            <LocalSearchLogo preserveAspectRatio="none" x="0" y="0" width="72" height="20" transform="matrix(.866 -.5 0 1 207 424)" />
          </g>
          <path className="hero-iso-agent__vents" d="m209 457 53-31m-45 36 53-31m-45 36 53-31" />
        </g>

        <g className="hero-iso-ports">
          <polygon points="549,78 554,81 554,87 549,90 544,87 544,81" />
          <polygon points="549,166 554,169 554,175 549,178 544,175 544,169" />
          <polygon points="549,254 554,257 554,263 549,266 544,263 544,257" />
          <polygon points="549,342 554,345 554,351 549,354 544,351 544,345" />
          <polygon points="306,345 311,348 311,354 306,357 301,354 301,348" />
          <polygon points="306,363 311,366 311,372 306,375 301,372 301,366" />
          <polygon points="306,381 311,384 311,390 306,393 301,390 301,384" />
          <polygon points="306,399 311,402 311,408 306,411 301,408 301,402" />
        </g>
      </svg>
    </div>
  );
}
