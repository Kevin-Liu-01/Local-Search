import { LocalSearchLogo } from "@/components/brand-logo";
import type { FAQS } from "@/lib/seo";

type FaqQuestion = (typeof FAQS)[number]["question"];

// Map by question, not position: changing the FAQ order must not mix up the art.
const artwork: Record<FaqQuestion, readonly string[]> = {
  "What is local-search?": [],
  "Can it use my logins?": ["/brand/chromium.svg"],
  "Which agents work with it?": [
    "/brand/claude.svg", "/brand/codex.svg", "/brand/cursor-mono.svg", "/brand/openclaw.svg",
  ],
  "Which search engines can I use?": [
    "/brand/google.svg", "/brand/bing.svg", "/brand/duckduckgo.svg", "/brand/brave.svg",
  ],
  "Does it bypass logins or CAPTCHAs?": ["/brand/chrome.svg"],
  "Where does my data go?": ["/brand/credentials.svg"],
  "How small and fast is it?": ["/brand/rust-mono.svg"],
  "Is it free and open source?": ["/brand/open-source.svg"],
};

export function FaqVisual({ question }: { question: FaqQuestion }) {
  const assets = artwork[question];

  return (
    <span className={`faq-visual${assets.length > 1 ? " faq-visual--group" : ""}`} aria-hidden="true">
      {assets.length === 0 ? <LocalSearchLogo className="faq-visual__brand" /> : assets.map((src) => (
        <span className="faq-visual__tile" key={src}>
          <span className="faq-visual__asset" style={{ backgroundImage: `url(${src})` }} />
        </span>
      ))}
    </span>
  );
}
