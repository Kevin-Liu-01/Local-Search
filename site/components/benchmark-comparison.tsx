"use client";

import { useState } from "react";
import { LocalSearchLogo } from "@/components/brand-logo";
import { BraveBrandIcon, ExaBrandIcon, FirecrawlBrandIcon, TavilyBrandIcon, ArrowUpRightIcon } from "@/components/icons";

// Recorded July 21, 2026: benchmarks/hosted-search-2026-07-21.md.
const providers = [
  { name: "local-search", tokens: 53.4, latency: 148.7, cost: 0, costLabel: "$0", note: "No hosted API credits", icon: null },
  { name: "Exa", tokens: 881.2, latency: 501.9, cost: 0.324, costLabel: "$0.324", note: "", icon: ExaBrandIcon },
  { name: "Brave Search", tokens: 108.6, latency: 322.0, cost: 0.120, costLabel: "$0.120", note: "", icon: BraveBrandIcon },
  { name: "Tavily", tokens: 259.5, latency: 1184.4, cost: 0.192, costLabel: "$0.192†", note: "24 credits", icon: TavilyBrandIcon },
  { name: "Firecrawl", tokens: 74.8, latency: 1520.8, cost: 0.154, costLabel: "≈$0.154‡", note: "48 credits · plan equivalent", icon: FirecrawlBrandIcon },
];
const metrics = {
  tokens: { label: "Context", value: "53.4", unit: "tokens / result", title: "Less context per result.", description: "Normalized tokens per result. Lower is better.", max: 900, maxLabel: "900 tokens", valueUnit: "tokens" },
  latency: { label: "Latency", value: "148.7", unit: "ms median", title: "Search response time.", description: "Median response time. Lower is better.", max: 1600, maxLabel: "1,600 ms", valueUnit: "ms" },
  cost: { label: "API cost", value: "$0", unit: "for 24 requests", title: "Search without the API bill.", description: "API cost across 24 requests.", max: 0.35, maxLabel: "$0.35", valueUnit: "" },
};
type Metric = keyof typeof metrics;
const decimal = new Intl.NumberFormat("en-US", { minimumFractionDigits: 1, maximumFractionDigits: 1 });

export function BenchmarkComparison() {
  const [metric, setMetric] = useState<Metric>("tokens");
  const selected = metrics[metric];
  return (
    <section className="content-section evidence-section" id="benchmarks" aria-labelledby="evidence-title">
      <header className="section-heading evidence-heading">
        <h2 id="evidence-title">Small output.<br /><span>Big difference.</span></h2>
        <p>The same searches and result format, compared across five tools.</p>
      </header>

      <div className="benchmark-dashboard" id="compare">
        <div className="benchmark-tabs" role="group" aria-label="Choose a benchmark metric">
          {(Object.keys(metrics) as Metric[]).map((key) => <button key={key} type="button" aria-pressed={metric === key} aria-controls="benchmark-chart" onClick={() => setMetric(key)}><span>{metrics[key].label}</span><strong>{metrics[key].value}</strong><span>{metrics[key].unit}</span></button>)}
        </div>
        <div className="benchmark-chart-heading">
          <div><h3>{selected.title}</h3><p>{selected.description}</p></div>
          <span className="benchmark-sample">12 queries · 24 requests / provider</span>
        </div>
        <div className="benchmark-axis" aria-hidden="true"><span>0</span><span>{selected.maxLabel}</span></div>
        <ol className="benchmark-bars" id="benchmark-chart" aria-label={selected.description}>
          {providers.map((provider, index) => {
            const Icon = provider.icon;
            const value = metric === "cost" ? provider.costLabel : decimal.format(provider[metric]);
            const comparison = metric === "cost" ? provider.note : index === 0 ? "Baseline" : `${decimal.format(provider[metric] / providers[0][metric])}× baseline`;
            return (
              <li key={provider.name} className={index === 0 ? "benchmark-row is-local" : "benchmark-row"}>
                <span className="benchmark-provider">{Icon ? <Icon size={26} /> : <LocalSearchLogo />}<b>{provider.name}</b></span>
                <div className="benchmark-track" aria-hidden="true"><i style={{ transform: `scaleX(${provider[metric] / selected.max})` }} />{provider[metric] === 0 && <span className="benchmark-zero" />}</div>
                <div className="benchmark-value"><strong>{value}<span className="sr-only"> {selected.valueUnit}</span></strong>{comparison && <span>{comparison}</span>}</div>
              </li>
            );
          })}
        </ol>
        <p className="benchmark-recorded">Recorded July 21, 2026. Local timing includes fresh and cached searches: <strong>384.5 ms cold · 6.5 ms cached.</strong></p>
        {metric === "cost" && <div className="benchmark-cost-notes"><p>† Tavily: 24 credits × $0.008.</p><p>‡ Firecrawl: 48 credits at the $16 / 5,000-credit Hobby-plan rate. Not pay-as-you-go.</p></div>}
      </div>

      <div className="context-proof">
        <div className="context-proof__claim"><strong>96.5<span>%</span></strong><h3>less agent context.</h3><p>Three results instead of a whole search page.</p></div>
        <div className="context-proof__chart" role="img" aria-label="local-search command and JSON: 309 tokens. Interactive page snapshot: 8,760.5 tokens. 96.5 percent less visible agent context.">
          <div className="context-proof__row is-local"><div><span><LocalSearchLogo />local-search JSON</span><b>309 <span>tokens</span></b></div><div className="context-proof__track"><i style={{ transform: `scaleX(${309 / 8760.5})` }} /></div></div>
          <div className="context-proof__row"><div><span>Browser page snapshot</span><b>8,760.5 <span>tokens</span></b></div><div className="context-proof__track"><i /></div></div>
          <p>Command + JSON vs. text snapshots of the page. Not images.</p>
        </div>
      </div>

      <details className="benchmark-methodology">
        <summary>How we measured <span aria-hidden="true">+</span></summary>
        <div>
          <p>The hosted comparison uses 12 identical queries at 3- and 10-result depths. Responses are normalized to a shared result schema. Token counts use o200k_base. Pricing is recorded as of July 21, 2026.</p>
          <p>Local-search uses its normal five-minute cache: 384.5 ms cold and 6.5 ms cached, with a combined 148.7 ms median. These are recorded results for this workload, not a guarantee for every search.</p>
          <p>The context comparison covers 36 searches at three-result depth. It compares visible command text plus JSON output with compact interactive browser snapshots. It does not measure screenshot image tokens.</p>
          <p>Local-search fulfilled 24/24 requested depths in the hosted comparison. Tavily fulfilled 17/24; the other providers fulfilled 24/24.</p>
          <div className="benchmark-data" tabIndex={0} role="region" aria-label="Full benchmark data; scroll horizontally on small screens"><table><caption>Recorded July 21, 2026</caption><thead><tr><th scope="col">Provider</th><th scope="col">Tokens / result</th><th scope="col">Median (ms)</th><th scope="col">24-request usage</th></tr></thead><tbody>{providers.map((provider) => <tr key={provider.name}><th scope="row">{provider.name}</th><td>{decimal.format(provider.tokens)}</td><td>{decimal.format(provider.latency)}</td><td>{provider.costLabel}</td></tr>)}</tbody></table></div>
          <p>† Tavily: 24 credits at $0.008 per credit. ‡ Firecrawl: Hobby-plan equivalent, 48 × ($16 / 5,000). No pay-as-you-go plan.</p>
        </div>
      </details>
      <div className="evidence-links">
        <a href="https://github.com/Kevin-Liu-01/Local-Search/tree/main/benchmarks" target="_blank" rel="noreferrer">Read the methodology <ArrowUpRightIcon size={18} /></a>
        <a href="/social/local-search-benchmark.png" target="_blank" rel="noreferrer">Original graphic <ArrowUpRightIcon size={18} /></a>
        <a href="/benchmarks.json">Get the data <ArrowUpRightIcon size={18} /></a>
      </div>
    </section>
  );
}
