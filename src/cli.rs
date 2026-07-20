use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

/// Free structured web search for agents through a local browser.
#[derive(Debug, Parser)]
#[command(name = "lsearch", version, about, long_about = None)]
pub struct Cli {
    /// CDP port, HTTP endpoint, or browser WebSocket URL.
    #[arg(long, global = true, env = "LOCAL_SEARCH_CDP")]
    pub cdp: Option<String>,

    /// Browser transport to use.
    #[arg(long, global = true, value_enum, default_value_t = BrowserKind::Auto)]
    pub browser: BrowserKind,

    /// Existing browser target id to control.
    #[arg(long, global = true)]
    pub target: Option<String>,

    /// Timeout for browser operations in milliseconds.
    #[arg(long, global = true, default_value_t = 15_000)]
    pub timeout: u64,

    /// Pretty-print JSON output.
    #[arg(long, global = true)]
    pub pretty: bool,

    /// Search query. Used when no subcommand is provided.
    #[arg(value_name = "QUERY")]
    pub query: Vec<String>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum BrowserKind {
    Auto,
    Chromium,
    Safari,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Inspect local browser support and active automation endpoints.
    Doctor,
    /// Discover and persist a browser connection.
    Connect(ConnectArgs),
    /// Start a managed local Chrome profile for prompt-free automation.
    Launch(LaunchArgs),
    /// Inspect or stop the managed local-search browser instance.
    Cleanup(CleanupArgs),
    /// Search the web in the local browser and return normalized results.
    Search(SearchArgs),
    /// Map links from a website without a hosted crawler.
    Map(MapArgs),
    /// Navigate the current tab.
    Open(OpenArgs),
    /// Extract readable article/page content.
    Read(ReadArgs),
    /// Return compact interactive elements with stable @eN refs.
    Snapshot(SnapshotArgs),
    /// Extract repeated structured records with CSS selectors.
    Extract(ExtractArgs),
    /// Evaluate JavaScript in the page.
    Eval(EvalArgs),
    /// Click a CSS selector or @eN ref.
    Click(TargetArgs),
    /// Focus an element and replace its value.
    Fill(ValueArgs),
    /// Focus an element and insert text like keyboard input.
    Type(ValueArgs),
    /// Dispatch a keyboard key to the active page.
    Press(PressArgs),
    /// Hover a CSS selector or @eN ref.
    Hover(TargetArgs),
    /// Select an option by value.
    Select(ValueArgs),
    /// Scroll the page or an element.
    Scroll(ScrollArgs),
    /// Wait for a selector, text, URL fragment, or duration.
    Wait(WaitArgs),
    /// Capture a PNG screenshot.
    Screenshot(ScreenshotArgs),
    /// Print the page to PDF.
    Pdf(PathArgs),
    /// Save the rendered page and subresources as MHTML.
    Mhtml(PathArgs),
    /// Return or save the current rendered HTML.
    Html(OptionalPathArgs),
    /// Make an authenticated fetch inside a temporary browser tab.
    Request(RequestArgs),
    /// Navigate while recording an agent-readable HAR and console log.
    Record(RecordArgs),
    /// Go back in history.
    Back,
    /// Go forward in history.
    Forward,
    /// Reload the current page.
    Reload,
    /// Inspect or control tabs.
    #[command(subcommand)]
    Tabs(TabsCommand),
    /// Inspect or modify browser cookies.
    #[command(subcommand)]
    Cookies(CookiesCommand),
}

#[derive(Debug, Args)]
pub struct ConnectArgs {
    /// Optional endpoint; auto-discovery is used when omitted.
    pub endpoint: Option<String>,
}

#[derive(Debug, Args)]
pub struct LaunchArgs {
    /// CDP port for the managed local browser.
    #[arg(long, default_value_t = 9322)]
    pub port: u16,

    /// Persistent profile directory. Defaults to the local-search config dir.
    #[arg(long)]
    pub profile: Option<PathBuf>,

    /// Chrome executable path.
    #[arg(long)]
    pub browser_path: Option<PathBuf>,

    /// Start Chrome in headless mode.
    #[arg(long)]
    pub headless: bool,

    /// Initial page to open.
    #[arg(long, default_value = "about:blank")]
    pub url: String,

    /// Do not persist this endpoint as the default.
    #[arg(long)]
    pub no_persist: bool,
}

#[derive(Debug, Args)]
pub struct CleanupArgs {
    /// CDP port for the managed local browser.
    #[arg(long, default_value_t = 9322)]
    pub port: u16,

    /// Persistent profile directory. Defaults to the local-search config dir.
    #[arg(long)]
    pub profile: Option<PathBuf>,

    /// Stop the managed browser and remove stale profile marker files.
    #[arg(long)]
    pub kill: bool,

    /// Use SIGKILL instead of SIGTERM.
    #[arg(long, short = 'f')]
    pub force: bool,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    pub query: String,
    #[arg(long, value_enum, default_value_t = SearchEngine::Duckduckgo)]
    pub engine: SearchEngine,
    #[arg(long, default_value_t = 10)]
    pub limit: usize,
    /// Also read each result page and include local extracted content.
    #[arg(long)]
    pub with_content: bool,
    /// Maximum characters of extracted content per result.
    #[arg(long, default_value_t = 2_000)]
    pub content_chars: usize,
    /// Use a new tab and leave the current tab untouched.
    #[arg(long)]
    pub new_tab: bool,
}

#[derive(Debug, Args)]
pub struct MapArgs {
    pub url: String,
    /// Maximum same-origin link traversal depth.
    #[arg(long, default_value_t = 1)]
    pub depth: usize,
    /// Maximum URLs to return.
    #[arg(long, default_value_t = 100)]
    pub limit: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum SearchEngine {
    Google,
    Bing,
    Duckduckgo,
}

#[derive(Debug, Args)]
pub struct OpenArgs {
    pub url: String,
    #[arg(long)]
    pub new_tab: bool,
}

#[derive(Debug, Args)]
pub struct ReadArgs {
    pub url: Option<String>,
    #[arg(long, value_enum, default_value_t = ReadFormat::Markdown)]
    pub format: ReadFormat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ReadFormat {
    Json,
    Markdown,
    Text,
}

#[derive(Debug, Args)]
pub struct SnapshotArgs {
    /// Include non-interactive headings and landmarks.
    #[arg(long)]
    pub all: bool,
    #[arg(long, default_value_t = 250)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct ExtractArgs {
    /// CSS selector matching each record.
    pub selector: String,
    /// Field mapping such as title=text, url=href, or price=@data-price.
    #[arg(long = "field", required = true)]
    pub fields: Vec<String>,
    #[arg(long, default_value_t = 100)]
    pub limit: usize,
}

#[derive(Debug, Args)]
pub struct EvalArgs {
    pub expression: String,
}

#[derive(Debug, Args)]
pub struct TargetArgs {
    #[arg(id = "element", value_name = "SELECTOR_OR_REF")]
    pub target: String,
}

#[derive(Debug, Args)]
pub struct ValueArgs {
    #[arg(id = "element", value_name = "SELECTOR_OR_REF")]
    pub target: String,
    pub value: String,
}

#[derive(Debug, Args)]
pub struct PressArgs {
    pub key: String,
}

#[derive(Debug, Args)]
pub struct ScrollArgs {
    #[arg(value_enum, default_value_t = ScrollDirection::Down)]
    pub direction: ScrollDirection,
    #[arg(default_value_t = 600)]
    pub amount: i64,
    #[arg(long = "element")]
    pub target: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Args)]
pub struct WaitArgs {
    #[arg(long)]
    pub selector: Option<String>,
    #[arg(long)]
    pub text: Option<String>,
    #[arg(long)]
    pub url: Option<String>,
    #[arg(long)]
    pub ms: Option<u64>,
}

#[derive(Debug, Args)]
pub struct ScreenshotArgs {
    pub path: PathBuf,
    #[arg(long)]
    pub full_page: bool,
}

#[derive(Debug, Args)]
pub struct PathArgs {
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct OptionalPathArgs {
    pub path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct RequestArgs {
    pub url: String,
    #[arg(long, default_value = "GET")]
    pub method: String,
    #[arg(long = "header")]
    pub headers: Vec<String>,
    #[arg(long)]
    pub body: Option<String>,
}

#[derive(Debug, Args)]
pub struct RecordArgs {
    pub url: String,
    #[arg(long)]
    pub har: PathBuf,
    #[arg(long)]
    pub mhtml: Option<PathBuf>,
    #[arg(long, default_value_t = 2_000)]
    pub duration: u64,
}

#[derive(Debug, Subcommand)]
pub enum TabsCommand {
    List,
    New { url: Option<String> },
    Use { target_id: String },
    Close { target_id: Option<String> },
}

#[derive(Debug, Subcommand)]
pub enum CookiesCommand {
    List {
        #[arg(long)]
        url: Option<String>,
        /// Reveal cookie values. Values are redacted by default.
        #[arg(long)]
        show_values: bool,
    },
    Set {
        name: String,
        value: String,
        #[arg(long)]
        url: String,
    },
    Delete {
        name: String,
        #[arg(long)]
        url: String,
    },
}
