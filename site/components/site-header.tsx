import { LocalSearchLogo } from "@/components/brand-logo";
import Link from "next/link";
import { ArrowUpRightIcon, CratesIoBrandIcon, GithubBrandIcon } from "@/components/icons";
import { ThemeToggle } from "@/components/theme-toggle";
import { CRATE_URL, REPOSITORY_URL } from "@/lib/seo";

export function SiteHeader({ docs = false }: { docs?: boolean }) {
  const home = docs ? "/" : "";
  return (
    <header className="site-header">
      <Link className="brand" href={docs ? "/" : "#top"} aria-label="local-search home">
        <LocalSearchLogo className="brand-logo" /><span className="brand-name">local-search</span>
      </Link>
      <nav aria-label="Main navigation">
        <a href={`${home}#demo`}>Demo</a>
        <a href={`${home}#benchmarks`}>Benchmarks</a>
        <a href={`${home}#faq`}>FAQ</a>
        <Link className="header-docs" href="/docs" aria-current={docs ? "page" : undefined}>Docs</Link>
      </nav>
      <div className="header-actions">
        <ThemeToggle />
        <a className="nav-cta" href={REPOSITORY_URL} target="_blank" rel="noreferrer" aria-label="View local-search on GitHub" title="GitHub">
          <GithubBrandIcon size={20} /><span className="nav-cta__label">GitHub</span><ArrowUpRightIcon size={16} />
        </a>
        <a className="nav-cta nav-cta--crates" href={CRATE_URL} target="_blank" rel="noreferrer" aria-label="View local-search on crates.io" title="crates.io">
          <CratesIoBrandIcon size={24} /><span className="nav-cta__label">crates.io</span><ArrowUpRightIcon size={16} />
        </a>
      </div>
    </header>
  );
}
