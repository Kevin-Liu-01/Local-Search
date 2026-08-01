"use client";

import "./brainless.css";

import { ClaudeHeader } from "@/components/brainless/claude/claude-header";
import { ClaudeMessage } from "@/components/brainless/claude/claude-message";
import { ClaudePrompt } from "@/components/brainless/claude/claude-prompt";
import { ClaudeThinking } from "@/components/brainless/claude/claude-thinking";
import { ClaudeToolCall } from "@/components/brainless/claude/claude-tool-call";
import { CodexExec } from "@/components/brainless/codex/codex-exec";
import { CodexHeader } from "@/components/brainless/codex/codex-header";
import { CodexMessage } from "@/components/brainless/codex/codex-message";
import { CodexPrompt } from "@/components/brainless/codex/codex-prompt";
import { CodexWorking } from "@/components/brainless/codex/codex-working";
import {
  CheckIcon,
  ChevronDownIcon,
  ClaudeBrandIcon,
  CodexBrandIcon,
  CursorBrandIcon,
  PlayIcon,
} from "@/components/icons";
import { traces, traceJson, type SearchTrace } from "@/lib/traces";
import { useEffect, useRef, useState } from "react";

type Agent = "claude" | "codex" | "cursor";
type DemoPhase = "typing" | "running" | "complete" | "leaving";

const agents: { id: Agent; label: string }[] = [
  { id: "claude", label: "Claude Code" },
  { id: "codex", label: "Codex" },
  { id: "cursor", label: "Cursor" },
];

const agentIcons = {
  claude: ClaudeBrandIcon,
  codex: CodexBrandIcon,
  cursor: CursorBrandIcon,
};

export function AgentPlaygroundInteractive() {
  const surfaceRef = useRef<HTMLDivElement>(null);
  const [cycleIndex, setCycleIndex] = useState(0);
  const [sequenceKey, setSequenceKey] = useState(0);
  const [skipTyping, setSkipTyping] = useState(false);
  const [phase, setPhase] = useState<DemoPhase>("typing");
  const [promptValue, setPromptValue] = useState("");
  const [step, setStep] = useState(0);
  const agent = agents[cycleIndex].id;
  const trace = traces[cycleIndex] ?? traces[0];
  const promptText = `Search for “${trace.query}” and return three results.`;
  const running = step > 0 && step < 4;

  useEffect(() => {
    let cancelled = false;
    let typingFrame = 0;
    const timers: number[] = [];
    const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    const schedule = (callback: () => void, delay: number) => {
      const timer = window.setTimeout(() => {
        if (!cancelled) callback();
      }, delay);
      timers.push(timer);
    };

    const beginRun = () => {
      setPromptValue("");
      setPhase("running");
      setStep(1);
      schedule(() => setStep(2), 480);
      schedule(() => setStep(3), 1_050);
      schedule(() => {
        setStep(4);
        setPhase("complete");
      }, 1_700);
      schedule(() => setPhase("leaving"), 3_600);
      schedule(() => {
        setSkipTyping(false);
        setCycleIndex((current) => (current + 1) % agents.length);
      }, 4_050);
    };

    schedule(() => {
      setStep(0);
      setPhase("typing");
      setPromptValue(skipTyping ? promptText : "");

      if (skipTyping) {
        schedule(beginRun, 120);
      } else if (reduceMotion) {
        setPromptValue(promptText);
        schedule(beginRun, 650);
      } else {
        schedule(() => {
          const startedAt = window.performance.now();
          const duration = Math.max(1_100, promptText.length * 17);
          let previousLength = 0;

          const typePrompt = (now: number) => {
            if (cancelled) return;
            const progress = Math.min((now - startedAt) / duration, 1);
            const nextLength = Math.min(
              promptText.length,
              Math.floor(progress * promptText.length),
            );
            if (nextLength !== previousLength) {
              previousLength = nextLength;
              setPromptValue(promptText.slice(0, nextLength));
            }
            if (progress < 1) {
              typingFrame = window.requestAnimationFrame(typePrompt);
            } else {
              schedule(beginRun, 460);
            }
          };

          typingFrame = window.requestAnimationFrame(typePrompt);
        }, 320);
      }
    }, 0);

    return () => {
      cancelled = true;
      timers.forEach((timer) => window.clearTimeout(timer));
      window.cancelAnimationFrame(typingFrame);
    };
  }, [cycleIndex, promptText, sequenceKey, skipTyping]);

  useEffect(() => {
    const surface = surfaceRef.current;
    if (!surface) return;

    const viewport = agent === "cursor"
      ? surface.querySelector<HTMLElement>(".cursor-thread__body")
      : surface;
    if (!viewport) return;

    const frame = window.requestAnimationFrame(() => {
      viewport.scrollTop = viewport.scrollHeight;
    });
    return () => window.cancelAnimationFrame(frame);
  }, [agent, phase, step, trace.id]);

  function runTraceNow() {
    setSkipTyping(true);
    setSequenceKey((current) => current + 1);
  }

  function selectAgent(index: number) {
    setSkipTyping(false);
    setCycleIndex(index);
    setSequenceKey((current) => current + 1);
  }

  return (
    <div className="playground__frame">
      <div className={`agent-window agent-window--${phase}`}>
        <div className="agent-window__bar">
          <span className="window-dots" aria-hidden><i /><i /><i /></span>
          <div className="agent-tabs agent-tabs--terminal" role="tablist" aria-label="Coding agent interface">
            {agents.map((item, index) => {
              const AgentIcon = agentIcons[item.id];
              return (
                <button
                  key={item.id}
                  type="button"
                  role="tab"
                  aria-selected={agent === item.id}
                  className={agent === item.id ? "agent-tab is-active" : "agent-tab"}
                  onClick={() => selectAgent(index)}
                >
                  <AgentIcon size={15} />
                  {item.label}
                  <span className="agent-tab__signal" aria-hidden />
                </button>
              );
            })}
          </div>
          <span>~/repos/Local-Search</span>
        </div>
        <div
          ref={surfaceRef}
          className={`agent-surface agent-cycle agent-cycle--${phase}`}
          role="tabpanel"
          aria-busy={running}
        >
          <div className="agent-cycle__content" key={`${agent}-${sequenceKey}`}>
            {agent === "claude" && (
              <ClaudeTrace
                trace={trace}
                step={step}
                running={running}
                promptValue={promptValue}
                phase={phase}
                onRun={runTraceNow}
              />
            )}
            {agent === "codex" && (
              <CodexTrace
                trace={trace}
                step={step}
                running={running}
                promptValue={promptValue}
                phase={phase}
                onRun={runTraceNow}
              />
            )}
            {agent === "cursor" && (
              <CursorTrace
                trace={trace}
                step={step}
                running={running}
                promptValue={promptValue}
                phase={phase}
                onRun={runTraceNow}
              />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

function ClaudeTrace({ trace, step, running, promptValue, phase, onRun }: TraceProps) {
  return (
    <div className="trace-stack trace-stack--claude">
      <ClaudeHeader
        user="Kevin"
        model="Opus 4.8 · Claude Max"
        org="local-search playground"
        cwd="~/repos/Local-Search"
        tips={["Use lsearch when current web context is required"]}
        whatsNew={["Structured browser search with zero API credits"]}
      />
      {step >= 1 && <ClaudeMessage role="user">Search for “{trace.query}” and return three results.</ClaudeMessage>}
      {step >= 1 && <ClaudeMessage>I’ll search through the managed local browser and keep the output compact.</ClaudeMessage>}
      {step >= 2 && (
        <ClaudeToolCall
          tool="Bash"
          arg={trace.command}
          result={step >= 4 ? `3 results in ${trace.latency}` : "Running in local Chrome…"}
          status={step >= 4 ? "success" : "pending"}
          defaultOpen={step >= 4}
        >
          {traceJson(trace)}
        </ClaudeToolCall>
      )}
      {running && <ClaudeThinking verbs={["Searching", "Parsing", "Normalizing"]} showTokens={false} />}
      {step >= 3 && <TraceSteps trace={trace} complete={completeStep(step)} />}
      {step >= 4 && <TraceResults trace={trace} />}
      <ClaudePrompt value={promptValue} readOnly inputClassName={phase === "typing" ? "is-autotyping" : undefined} placeholder={running ? "local-search is running…" : "Next search will start automatically"} onKeyDown={(event) => event.key === "Enter" && onRun()} />
    </div>
  );
}

function CodexTrace({ trace, step, running, promptValue, phase, onRun }: TraceProps) {
  return (
    <div className="trace-stack trace-stack--codex">
      <CodexHeader model="gpt-5.6-sol high" directory="~/repos/Local-Search" />
      {step >= 1 && <CodexMessage role="user">Search for “{trace.query}” and return three results.</CodexMessage>}
      {step >= 1 && <CodexMessage>I’ll use the local browser and return the normalized output.</CodexMessage>}
      {step >= 2 && (
        <CodexExec
          command={trace.command}
          result={step >= 4 ? `→ 3 results · ${trace.latency}` : "→ running"}
          status={step >= 4 ? "ok" : "run"}
          defaultOpen={step >= 4}
        >
          {traceJson(trace)}
        </CodexExec>
      )}
      {running && <CodexWorking label={step < 3 ? "Searching" : "Normalizing"} />}
      {step >= 3 && <TraceSteps trace={trace} complete={completeStep(step)} />}
      {step >= 4 && <TraceResults trace={trace} />}
      <CodexPrompt value={promptValue} readOnly inputClassName={phase === "typing" ? "is-autotyping" : undefined} directory="~/repos/Local-Search" placeholder={running ? "local-search is running…" : "Next search will start automatically"} onKeyDown={(event) => event.key === "Enter" && onRun()} />
    </div>
  );
}

function CursorTrace({ trace, step, running, promptValue, phase, onRun }: TraceProps) {
  return (
    <div className="cursor-trace">
      <aside className="cursor-rail">
        <b>LOCAL-SEARCH</b>
        <span className="is-selected">⌁ Search with local browser</span>
        <span>◫ Compare result schemas</span>
        <span>✓ Benchmark token usage</span>
        <small>main · local</small>
      </aside>
      <div className="cursor-thread">
        <div className="cursor-thread__header"><b>Search with local browser</b><span>Agent · Auto</span></div>
        <div className="cursor-thread__body">
          {step >= 1 && <div className="cursor-user">Search for “{trace.query}” and return three results.</div>}
          {step >= 1 && <p>I’ll run the query through your managed browser and inspect the normalized response.</p>}
          {step >= 2 && (
            <details className="cursor-tool" open={step >= 4}>
              <summary><span>›_</span><b>Run local search</b><small>{step >= 4 ? `✓ ${trace.latency}` : "running"}</small><ChevronDownIcon size={13} /></summary>
              <pre>{trace.command}</pre>
              {step >= 4 && <pre className="cursor-json">{traceJson(trace)}</pre>}
            </details>
          )}
          {running && <p className="cursor-working"><span />{step < 3 ? "Searching the web…" : "Normalizing three results…"}</p>}
          {step >= 3 && <TraceSteps trace={trace} complete={completeStep(step)} />}
          {step >= 4 && <TraceResults trace={trace} />}
        </div>
        <div className="cursor-composer">
          <input aria-label="Cursor prompt" value={promptValue} readOnly className={phase === "typing" ? "is-autotyping" : undefined} placeholder={running ? "local-search is running…" : "Next search will start automatically"} onKeyDown={(event) => event.key === "Enter" && onRun()} />
          <button type="button" onClick={onRun} aria-label="Run trace"><PlayIcon size={14} /></button>
        </div>
      </div>
    </div>
  );
}

function TraceSteps({ trace, complete }: { trace: SearchTrace; complete: boolean }) {
  return (
    <div className="trace-steps" aria-label="local-search execution trace">
      <span><CheckIcon size={12} /> Attach managed Chrome</span>
      <span><CheckIcon size={12} /> Navigate DuckDuckGo</span>
      <span><CheckIcon size={12} /> Extract {trace.results.length} organic results</span>
      <span className={complete ? "is-complete" : "is-pending"}>{complete ? <CheckIcon size={12} /> : <span className="mini-loader" />} Normalize JSON stdout</span>
    </div>
  );
}

function TraceResults({ trace }: { trace: SearchTrace }) {
  return (
    <div className="trace-results">
      {trace.results.map((result) => (
        <a key={result.url} href={result.url} target="_blank" rel="noreferrer">
          <span>{String(result.rank).padStart(2, "0")}</span>
          <div><b>{result.title}</b><small>{result.domain} · {result.snippet}</small></div>
        </a>
      ))}
    </div>
  );
}

function completeStep(step: number) {
  return step >= 4;
}

type TraceProps = {
  trace: SearchTrace;
  step: number;
  running: boolean;
  promptValue: string;
  phase: DemoPhase;
  onRun: () => void;
};
