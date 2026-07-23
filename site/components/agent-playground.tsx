import { AgentPlaygroundLoader } from "@/components/agent-playground-loader";
import {
  ClaudeBrandIcon,
  CodexBrandIcon,
  CursorBrandIcon,
  SearchIcon,
} from "@/components/icons";
import { traces } from "@/lib/traces";

const agents = [
  { label: "Claude Code", icon: ClaudeBrandIcon },
  { label: "Codex", icon: CodexBrandIcon },
  { label: "Cursor", icon: CursorBrandIcon },
];

export function AgentPlayground() {
  return (
    <section className="playground" id="demo" aria-label="Interactive local-search traces">
      <div className="playground__workspace">
        <div className="playground__workspace-bar">
          <span className="window-dots" aria-hidden><i /><i /><i /></span>
          <span>local-search / agent trace</span>
          <span>Claude Code · Codex · Cursor</span>
        </div>
        <div className="playground__workspace-canvas">
          <AgentPlaygroundLoader fallback={<AgentPlaygroundPreview />} />
        </div>
      </div>

      <p className="trace-provenance">
        Captured July 21, 2026 with the managed local Chrome profile. Results can change with the search engine.
      </p>
    </section>
  );
}

function AgentPlaygroundPreview() {
  const trace = traces[0];

  return (
    <div className="playground__frame" aria-label="Claude Code local-search preview">
      <div className="playground__toolbar">
        <div className="agent-tabs" aria-label="Coding agent interface">
          {agents.map((agent, index) => {
            const AgentIcon = agent.icon;
            return (
              <span className={index === 0 ? "agent-tab is-active" : "agent-tab"} key={agent.label}>
                <AgentIcon size={15} />{agent.label}
              </span>
            );
          })}
        </div>
        <span className="playground__credit">Agent shells by brainless ↗</span>
      </div>

      <div className="trace-presets" aria-label="Captured search traces">
        {traces.map((item, index) => (
          <div className={index === 0 ? "trace-preset is-active" : "trace-preset"} key={item.id}>
            <SearchIcon size={14} />
            <span><b>{item.label}</b><small>{item.query}</small></span>
            <span className="trace-latency">{item.latency}</span>
          </div>
        ))}
      </div>

      <div className="agent-window">
        <div className="agent-window__bar">
          <span className="window-dots" aria-hidden><i /><i /><i /></span>
          <span>Terminal — claude</span>
          <span>~/repos/Local-Search</span>
        </div>
        <div className="agent-surface demo-preview">
          <div className="demo-preview__header">
            <span className="demo-preview__logo"><ClaudeBrandIcon size={25} /></span>
            <div><b>Claude Code</b><small>Opus 4.8 · Claude Max</small></div>
          </div>
          <div className="demo-preview__rule" />
          <p className="demo-preview__meta">Kevin · local-search playground · ~/repos/Local-Search</p>
          <div className="demo-preview__message">
            <span>❯</span>
            <p>Search for “{trace.query}” and return three results.</p>
          </div>
          <div className="demo-preview__spacer" />
          <div className="demo-preview__prompt"><span>❯</span>Select a capture above or run this trace</div>
          <p className="demo-preview__mode">⏵⏵ auto mode on · ← for agents</p>
        </div>
      </div>

      <div className="playground__footer">
        <div><span className="status-dot" />Ready to run</div>
        <span className="run-button">▶ Run search</span>
      </div>
    </div>
  );
}
