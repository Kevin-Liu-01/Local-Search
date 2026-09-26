import { AgentCopyButton } from "@/components/agent-copy-button";
import Link from "next/link";
import { BenchmarkComparison } from "@/components/benchmark-comparison";
import { LocalSearchLogo } from "@/components/brand-logo";
import { BrowserConnection } from "@/components/browser-connection";
import { BrowserWorkflow } from "@/components/browser-workflow";
import { FaqVisual } from "@/components/faq-visual";
import { HeroAgentSwitch } from "@/components/hero-agent-switch";
import { InstallCommand } from "@/components/install-command";
import { SiteHeader } from "@/components/site-header";
import { JsonLd } from "@/components/json-ld";
import { FAQS, STRUCTURED_DATA } from "@/lib/seo";
import {
  ArrowUpRightIcon,
  ChevronRightIcon,
  ChromiumBrandIcon,
  DocumentIcon,
  JsonSchemaBrandIcon,
  RustOfficialIcon,
} from "@/components/icons";

export default function Home() {
  return (
    <>
      <a className="skip-link" href="#main">Skip to content</a>
      <SiteHeader />
      <JsonLd data={STRUCTURED_DATA} />

      <main id="main">
        <section className="hero-shell" id="top" aria-labelledby="hero-title">
          <ReticleSpacer />
          <div className="hero-stage">
            <div className="hero">
              <h1 id="hero-title">A local browser API<br /><span>for your agents.</span></h1>
              <p className="hero-copy">Let your agent search, read pages, and use signed-in sites through your browser. You approve access.</p>
              <div className="hero-actions">
                <Link className="primary-button" href="/docs">
                  <DocumentIcon size={20} /><span>Documentation</span>
                </Link>
                <AgentCopyButton />
              </div>
              <InstallCommand />
              <div className="hero-proof" aria-label="Product benefits">
                <span><RustOfficialIcon size={20} /> Built with Rust</span>
                <span><JsonSchemaBrandIcon size={20} /> Compact JSON</span>
                <span><ChromiumBrandIcon size={20} /> Your browser, your choice</span>
              </div>
            </div>
            <div className="hero-dither">
              <i /><i /><i />
              <HeroIsometricArtwork />
            </div>
          </div>
          <ReticleSpacer />
        </section>

        <section className="content-section workflow-section" id="workflow" aria-labelledby="workflow-title">
          <header className="section-heading">
            <h2 id="workflow-title">Your agent asks.<br />Your browser answers.</h2>
            <p>Your agent runs a command. Your browser searches and returns the results.</p>
          </header>
          <BrowserWorkflow />
          <BrowserConnection />
        </section>
        <ReticleSpacer />

        <BenchmarkComparison />
        <ReticleSpacer />

        <section className="content-section faq-section" id="faq" aria-labelledby="faq-title">
          <header className="section-heading">
            <h2 id="faq-title">Before you install.</h2>
          </header>
          <div className="faq-list">
            {FAQS.map(({ question, answer }) => (
              <details key={question}>
                <summary>
                  <span className="faq-question">{question}</span>
                  <FaqVisual question={question} />
                  <ChevronRightIcon className="faq-chevron" size={22} />
                </summary>
                <p>{answer}</p>
              </details>
            ))}
          </div>
        </section>
        <ReticleSpacer />

        <section className="closing-cta">
          <LocalSearchLogo className="closing-logo" />
          <h2>Your browser.<br /><span>Ready for your agent.</span></h2>
          <p>Open source. Runs locally. No hosted search API key.</p>
          <InstallCommand />
          <div className="closing-links">
            <a href="/docs">Read the docs <ChevronRightIcon size={20} /></a>
            <a href="https://github.com/Kevin-Liu-01/Local-Search/blob/main/SKILL.md" target="_blank" rel="noreferrer">Get the agent skill <ChevronRightIcon size={20} /></a>
          </div>
        </section>
      </main>

      <ReticleSpacer />
      <footer className="site-footer">
        <a className="brand" href="#top"><LocalSearchLogo className="brand-logo" />local-search</a>
        <p>A local browser API for agents.</p>
        <div>
          <a href="https://github.com/Kevin-Liu-01/local-search" target="_blank" rel="noreferrer">GitHub<ArrowUpRightIcon size={16} /></a>
          <a href="https://www.npmjs.com/package/@kevinliu01/localsearch" target="_blank" rel="noreferrer">npm<ArrowUpRightIcon size={16} /></a>
          <a href="https://rustfoundation.org/policy/rust-trademark-policy/" target="_blank" rel="noreferrer">Rust attribution<ArrowUpRightIcon size={16} /></a>
          <a href="https://github.com/Kevin-Liu-01" target="_blank" rel="noreferrer">Kevin Liu<ArrowUpRightIcon size={16} /></a>
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
  // Inner-to-outer routing pairs the top box port with the lowest engine.
  // The static wires and moving signals share these paths so they cannot drift.
  const connections = [
    { engine: "duckduckgo", path: "M306 351L350 376.4L350 384L450 441.7L566 374.8L566 357.8L549 348" },
    { engine: "brave", path: "M306 369L344 390.9L344 402L444 459.7L576 383.5L576 275.6L549 260" },
    { engine: "bing", path: "M306 387L338 405.5L338 420L438 477.7L586 392.3L586 193.4L549 172" },
    { engine: "google", path: "M306 405L332 420L332 438L432 495.7L596 401L596 111.1L549 84" },
  ];

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
          <g transform="translate(40 24)">
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
          {connections.map(({ engine, path }) => <path key={engine} data-engine={engine} d={path} />)}
        </g>
        <g className="hero-iso-wires hero-iso-wires--signal">
          {connections.map(({ engine, path }) => <path key={engine} data-engine={engine} d={path} />)}
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

        <HeroAgentSwitch>
          <polygon className="hero-iso-agent__left" points="84,357 179,412 179,484 84,429" />
          <polygon className="hero-iso-agent__front" points="179,412 306,339 306,411 179,484" />
          <polygon className="hero-iso-agent__top" points="84,357 211,284 306,339 179,412" />
          <path className="hero-iso-agent__groove" d="m113 357 98-56 66 38-98 56z" />
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
        </HeroAgentSwitch>

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
