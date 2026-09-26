"use client";

import { useEffect, useRef, useState, useSyncExternalStore } from "react";
import { AgentSearchPanel, type DemoAgent } from "@/components/agent-playground-interactive";
import { CheckIcon, ClaudeBrandIcon, CodexBrandIcon, CursorBrandIcon, GlobeIcon, SearchIcon } from "@/components/icons";
import { traces } from "@/lib/traces";

const agents: { id: DemoAgent; label: string; icon: typeof ClaudeBrandIcon }[] = [
  { id: "claude", label: "Claude Code", icon: ClaudeBrandIcon },
  { id: "codex", label: "Codex", icon: CodexBrandIcon },
  { id: "cursor", label: "Cursor", icon: CursorBrandIcon },
];
const steps = ["Your agent gets the prompt", "Your agent runs lsearch", "Your browser searches", "Results return as JSON", "Your agent has the answer"];

function subscribeMotion(callback: () => void) {
  const media = window.matchMedia("(prefers-reduced-motion: reduce)");
  media.addEventListener("change", callback);
  return () => media.removeEventListener("change", callback);
}
const prefersReducedMotion = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

function Asset({ name }: { name: string }) {
  return <span className="sim-asset" aria-hidden="true" style={{ backgroundImage: `url(/brand/${name}.svg)` }} />;
}

export function BrowserWorkflow() {
  const root = useRef<HTMLDivElement>(null);
  const [active, setActive] = useState(0);
  const [phase, setPhase] = useState(4);
  const [run, setRun] = useState(0);
  const [instant, setInstant] = useState(false);
  const [visible, setVisible] = useState(false);
  const [foreground, setForeground] = useState(true);
  const reduceMotion = useSyncExternalStore(subscribeMotion, prefersReducedMotion, () => true);
  const stage = reduceMotion ? 4 : phase;
  const agent = agents[active];
  const trace = traces[active];
  const command = `lsearch "${trace.query}" --engine duckduckgo --limit 3 --json`;

  useEffect(() => {
    const element = root.current;
    if (!element) return;
    const observer = new IntersectionObserver(([entry]) => setVisible(entry.isIntersecting), { threshold: 0.1 });
    observer.observe(element);
    const onVisibility = () => setForeground(!document.hidden);
    document.addEventListener("visibilitychange", onVisibility);
    return () => { observer.disconnect(); document.removeEventListener("visibilitychange", onVisibility); };
  }, []);

  useEffect(() => {
    if (!visible || !foreground || reduceMotion) return;
    // Keyboard selections show the answer immediately, then rejoin the loop.
    const timers = instant ? [] : [
      window.setTimeout(() => setPhase(0), 0),
      window.setTimeout(() => setPhase(1), 180),
      window.setTimeout(() => setPhase(2), 380),
      window.setTimeout(() => setPhase(3), 750),
      window.setTimeout(() => setPhase(4), 1100),
    ];
    timers.push(window.setTimeout(() => {
      setPhase(0);
      setInstant(false);
      setActive((index) => (index + 1) % agents.length);
    }, 6000));
    return () => timers.forEach(window.clearTimeout);
  }, [active, run, visible, foreground, instant, reduceMotion]);

  function chooseAgent(index: number, keyboard = false) {
    setActive(index);
    setPhase(keyboard || reduceMotion ? 4 : 0);
    setInstant(keyboard || reduceMotion);
    setRun((value) => value + 1);
  }

  return (
    <div className="search-simulation paired-demo" id="demo" ref={root} data-running={visible && foreground && !reduceMotion} aria-label="Simulated coding agent and local browser using recorded search results">
      <div className="sim-panels" id="output" data-phase={stage} data-agent={agent.id} data-instant={instant || reduceMotion}>
        <div className={`paired-agent paired-agent--${agent.id}`}>
          <div className="paired-agent-bar">
            <span className="window-dots" aria-hidden="true"><i /><i /><i /></span>
            <div className="paired-agent-tabs" role="tablist" aria-label="Choose a coding agent">
              {agents.map((item, index) => {
                const Icon = item.icon;
                return <button
                  key={item.id}
                  type="button"
                  role="tab"
                  id={`paired-tab-${item.id}`}
                  aria-controls="paired-agent-panel"
                  aria-label={item.label}
                  aria-selected={active === index}
                  tabIndex={active === index ? 0 : -1}
                  onClick={(event) => chooseAgent(index, event.detail === 0)}
                  onKeyDown={(event) => {
                    const next = event.key === "ArrowRight" ? (index + 1) % agents.length
                      : event.key === "ArrowLeft" ? (index + agents.length - 1) % agents.length
                      : event.key === "Home" ? 0 : event.key === "End" ? agents.length - 1 : null;
                    if (next === null) return;
                    event.preventDefault();
                    chooseAgent(next, true);
                    document.getElementById(`paired-tab-${agents[next].id}`)?.focus();
                  }}
                ><Icon size={20} /><span>{item.id === "claude" ? <>Claude<span className="paired-agent-name-suffix"> Code</span></> : item.label}</span></button>;
              })}
            </div>
          </div>
          <div className="paired-agent-panel" role="tabpanel" id="paired-agent-panel" aria-labelledby={`paired-tab-${agent.id}`}>
            <AgentSearchPanel key={`${agent.id}-${run}`} agent={agent.id} trace={trace} command={command} stage={stage} instant={instant || reduceMotion} />
          </div>
          <div className="paired-agent-footer"><span><i aria-hidden="true" /><span className="paired-agent-status">{steps.map((step, index) => <span key={step} aria-hidden={stage !== index}>{step}</span>)}</span></span><span>Simulation</span></div>
        </div>

        <div className="sim-transfer" aria-hidden="true" />

        <div className="sim-browser">
          <div className="sim-browser-tabs"><Asset name="chrome" /><span>Local Chrome</span><span className="sim-tab-close" aria-hidden="true">×</span></div>
          <div className="sim-address-row"><span aria-hidden="true">←</span><div><GlobeIcon size={16} /><span>{stage >= 2 ? "duckduckgo.com/?q=" + encodeURIComponent(trace.query) : "New tab"}</span></div></div>
          <div className="sim-browser-page">
            <div className="sim-search-field"><Asset name="duckduckgo" /><span className="sim-search-query"><span aria-hidden={stage < 2}>{trace.query}</span><span aria-hidden={stage >= 2}>Search the web</span></span><SearchIcon size={18} /></div>
            <div className="sim-search-nav" aria-hidden="true"><span>All</span><span>Images</span><span>Videos</span><span>News</span></div>
            <div className="sim-page-content" key={trace.id}>
              {stage < 3 ? <div className="sim-loading" aria-label={stage < 2 ? "Waiting for the agent" : "Loading search results"}><SearchIcon size={30} /><p>{stage < 2 ? "Your browser, ready to search." : "Searching DuckDuckGo…"}</p><div className="sim-loading-lines"><i /><i /><i /></div></div> : (
                <div className="sim-search-results">
                  {trace.results.map((result, index) => <a key={result.url} href={result.url} target="_blank" rel="noreferrer" style={{ animationDelay: `${index * 70}ms` }}><span className="sim-result-domain"><GlobeIcon size={16} />{result.domain}</span><strong>{result.title}</strong><p>{result.snippet}</p></a>)}
                </div>
              )}
            </div>
          </div>
          <div className="sim-browser-footer"><CheckIcon size={16} />Runs on your machine<span>Not a hosted browser</span></div>
        </div>
      </div>
      <div className="sim-caption"><span>Recorded results · July 21, 2026. <a href="https://brainless.swerdlow.dev" target="_blank" rel="noreferrer">Agent shells by brainless</a>.</span><span className="sim-supported" aria-label="Supports Google, Bing, Brave, and DuckDuckGo"><Asset name="google" /><Asset name="bing" /><Asset name="brave" /><Asset name="duckduckgo" /></span></div>
    </div>
  );
}
