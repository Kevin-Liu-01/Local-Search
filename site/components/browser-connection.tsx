import "./browser-connection.css";
import { LocalSearchLogo } from "@/components/brand-logo";
import { CopyCommand } from "@/components/copy-command";
import { CheckIcon, ChevronDownIcon, ChromeColorBrandIcon, KeyIcon, ShieldIcon, SiteBrandIcon } from "@/components/icons";

const siteExamples = [
  { category: "Communities", sites: [
    { name: "Reddit", icon: "reddit" },
    { name: "LinkedIn", icon: "linkedin" },
    { name: "Discord", icon: "discord" },
  ] },
  { category: "Docs", sites: [
    { name: "GitHub", icon: "github" },
    { name: "Notion", icon: "notion" },
    { name: "Google Docs", icon: "google-docs" },
  ] },
  { category: "Work apps", sites: [
    { name: "Slack", icon: "slack" },
    { name: "Linear", icon: "linear" },
    { name: "Figma", icon: "figma" },
  ] },
] as const;

export function BrowserConnection() {
  return (
    <section className="browser-connection" id="browser-choice" aria-labelledby="browser-choice-title">
      <header className="connection-heading">
        <h3 id="browser-choice-title">Choose your browser.</h3>
        <p>Use your current logins or keep agent work separate.</p>
      </header>

      <div className="connection-options">
        <article className="connection-option connection-option--existing">
          <div className="connection-visual">
            <div className="connection-window-bar"><ChromeColorBrandIcon size={24} /><span>Your everyday Chrome</span><span className="connection-session"><CheckIcon size={16} />Signed in</span></div>
            <div className="connection-sites" role="list" aria-label="Example sites in your browser">
              {siteExamples.map(({ category, sites }) => (
                <div className="connection-site-group" role="listitem" key={category}>
                  <b>{category}</b>
                  <div className="connection-site-logos">
                    {sites.map(({ name, icon }) => (
                      <span className="connection-site-logo" role="img" aria-label={name} title={name} key={icon}>
                        <SiteBrandIcon name={icon} size={30} />
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
            <div className="connection-visual-note"><ShieldIcon size={18} />Chrome asks for your approval</div>
          </div>
          <div className="connection-option-copy">
            <h4>Use your current logins.</h4>
            <p>Connect to Chrome and use the sites you’re already signed into.</p>
            <CopyCommand value="lsearch connect --existing" />
            <details className="connection-instructions">
              <summary>How to connect <ChevronDownIcon size={20} /></summary>
              <ol>
                <li>In Chrome 144+, open <code>chrome://inspect/#remote-debugging</code> and enable remote debugging.</li>
                <li>Run the command above, then approve Chrome’s connection prompt.</li>
              </ol>
              <p>On macOS and Linux, approve once per connection, not per search. Run <code>lsearch disconnect</code> to end access without closing Chrome. Reconnecting asks for approval again.</p>
            </details>
          </div>
        </article>

        <article className="connection-option connection-option--managed">
          <div className="connection-visual" aria-hidden="true">
            <div className="connection-window-bar"><LocalSearchLogo /><span>A separate profile</span></div>
            <div className="connection-profiles">
              <span><ChromeColorBrandIcon size={32} /><b>Your Chrome</b></span>
              <i />
              <span className="connection-profile-agent"><LocalSearchLogo /><b>Agent profile</b></span>
            </div>
            <div className="connection-visual-note"><KeyIcon size={18} />Its own logins, saved locally</div>
          </div>
          <div className="connection-option-copy">
            <h4>Keep agent work separate.</h4>
            <p>Sign in once in a browser profile just for your agent.</p>
            <CopyCommand value="lsearch connect --managed" />
            <details className="connection-instructions">
              <summary>How to set up <ChevronDownIcon size={20} /></summary>
              <ol>
                <li>Run the command above to open the local-search browser.</li>
                <li>Sign into the sites your agent needs in that browser.</li>
              </ol>
              <p>Your everyday Chrome is untouched. Logins last until you or the site sign out.</p>
            </details>
          </div>
        </article>
      </div>

      <div className="connection-promise"><ShieldIcon size={24} /><p><strong>Your choice is saved.</strong> If it disconnects, lsearch tells you. It never switches browsers.</p></div>
    </section>
  );
}
