use super::{attach_client, selected_client, urlencoding, verify_selection};
use crate::{
    browser::{CdpClient, scripts},
    cli::{Cli, SearchArgs, SearchEngine, SearchFormat},
    config,
    error::{Error, Result},
    output::print_json,
    ui::{self, Spinner},
};
use serde_json::{Value, json};
use std::{
    io::{self, IsTerminal as _},
    path::PathBuf,
    time::Duration,
};

pub(super) async fn search_command(cli: &Cli, args: &SearchArgs) -> Result<()> {
    // Even a cache hit must honor the chosen, currently connected browser.
    // Verify the live browser, but defer page attachment until we need a page.
    let (endpoint, mut client) = selected_client(cli).await?;
    if let Some(mut value) = load_search_cache(args, &client.cache_scope).await {
        if args.with_content || cli.target.is_some() {
            client = attach_client(cli, endpoint, client).await?;
        } else {
            let _selection = config::selection_lock()?;
            verify_selection(cli, &endpoint).await?;
        }
        prepare_search_results(&mut value, args);
        if args.with_content {
            enrich_search_results(&mut client, &mut value, args.content_chars).await?;
        }
        let result_count = value
            .get("results")
            .and_then(Value::as_array)
            .map_or(0, Vec::len);
        ui::success(format!(
            "Returned {result_count} ranked results from the local cache"
        ));
        return print_search(cli, args, &value);
    }

    let engine = search_engine_label(args.engine);
    let spinner = Spinner::start(format!("Searching {engine} through your local browser"));
    let mut client = attach_client(cli, endpoint, client).await?;
    search(cli, &mut client, args, spinner).await
}

async fn search(
    cli: &Cli,
    client: &mut CdpClient,
    args: &SearchArgs,
    spinner: Spinner,
) -> Result<()> {
    let engine = search_engine_label(args.engine);
    let url = search_engine_url(args.engine, &args.query);
    if args.new_tab {
        let target = client.create_target("about:blank").await?;
        client.attach(&target.target_id).await?;
    }
    client.start_navigation(&url).await?;
    let requested_results = args.limit.max(1);
    client
        .wait_for_js(&scripts::search_ready(&args.query, requested_results))
        .await?;
    let mut value = client.evaluate(scripts::search_results(), true).await?;
    save_search_cache(args, &value, &client.cache_scope).await;
    prepare_search_results(&mut value, args);
    if args.with_content {
        enrich_search_results(client, &mut value, args.content_chars).await?;
    }
    let result_count = value
        .get("results")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    spinner.success(format!(
        "Returned {result_count} ranked results from {engine}"
    ));
    print_search(cli, args, &value)
}

fn search_engine_label(engine: SearchEngine) -> &'static str {
    match engine {
        SearchEngine::Google => "Google",
        SearchEngine::Bing => "Bing",
        SearchEngine::Brave => "Brave Search",
        SearchEngine::Duckduckgo => "DuckDuckGo",
    }
}

fn search_engine_url(engine: SearchEngine, query: &str) -> String {
    let query = urlencoding(query);
    match engine {
        SearchEngine::Google => format!("https://www.google.com/search?q={query}"),
        SearchEngine::Bing => format!("https://www.bing.com/search?q={query}"),
        SearchEngine::Brave => format!("https://search.brave.com/search?q={query}"),
        SearchEngine::Duckduckgo => format!("https://html.duckduckgo.com/html/?q={query}"),
    }
}

fn prepare_search_results(value: &mut Value, args: &SearchArgs) {
    if let Some(results) = value.get_mut("results").and_then(Value::as_array_mut) {
        results.truncate(args.limit);
        truncate_search_snippets(results, args.snippet_chars);
    }
}

fn print_search(cli: &Cli, args: &SearchArgs, value: &Value) -> Result<()> {
    let machine_engine = format!("{:?}", args.engine).to_lowercase();
    let envelope =
        json!({ "ok": true, "query": args.query, "engine": machine_engine, "search": value });
    let output = resolve_search_format(
        args.format,
        cli.json,
        cli.pretty,
        io::stdout().is_terminal(),
    );

    match output {
        SearchFormat::Auto => unreachable!("search output format must be resolved"),
        SearchFormat::Json => print_json(&envelope, cli.pretty),
        SearchFormat::Table => {
            ui::print_search_results(&args.query, search_engine_label(args.engine), value)
        }
    }
}

fn resolve_search_format(
    requested: SearchFormat,
    force_json: bool,
    pretty: bool,
    stdout_is_terminal: bool,
) -> SearchFormat {
    if force_json || pretty {
        return SearchFormat::Json;
    }
    match requested {
        SearchFormat::Auto if stdout_is_terminal => SearchFormat::Table,
        SearchFormat::Auto | SearchFormat::Json => SearchFormat::Json,
        SearchFormat::Table => SearchFormat::Table,
    }
}

async fn load_search_cache(args: &SearchArgs, scope: &str) -> Option<Value> {
    if args.no_cache || args.cache_ttl == 0 {
        return None;
    }
    let path = search_cache_path(args, scope).ok()?;
    let age = tokio::fs::metadata(&path)
        .await
        .ok()?
        .modified()
        .ok()?
        .elapsed()
        .ok()?;
    if age > Duration::from_secs(args.cache_ttl) {
        return None;
    }
    let record: Value = serde_json::from_slice(&tokio::fs::read(path).await.ok()?).ok()?;
    let engine = format!("{:?}", args.engine).to_lowercase();
    if record.get("scope").and_then(Value::as_str) != Some(scope)
        || record.get("query").and_then(Value::as_str) != Some(args.query.as_str())
        || record.get("engine").and_then(Value::as_str) != Some(engine.as_str())
    {
        return None;
    }
    let search = record.get("search")?.clone();
    if search.get("blocked").and_then(Value::as_bool) != Some(false)
        || search.get("results")?.as_array()?.len() < args.limit
    {
        return None;
    }
    Some(search)
}

async fn save_search_cache(args: &SearchArgs, search: &Value, scope: &str) {
    if args.no_cache
        || args.cache_ttl == 0
        || search.get("blocked").and_then(Value::as_bool) != Some(false)
    {
        return;
    }
    let Ok(path) = search_cache_path(args, scope) else {
        return;
    };
    let Some(parent) = path.parent() else {
        return;
    };
    if tokio::fs::create_dir_all(parent).await.is_err() {
        return;
    }
    let record = json!({
        "scope": scope,
        "engine": format!("{:?}", args.engine).to_lowercase(),
        "query": args.query,
        "search": search,
    });
    if let Ok(bytes) = serde_json::to_vec(&record) {
        // Readers see either the old complete record or the new complete record.
        let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
        if tokio::fs::write(&temporary, bytes).await.is_ok() {
            let _ = tokio::fs::rename(&temporary, &path).await;
        }
        let _ = tokio::fs::remove_file(temporary).await;
    }
}

fn search_cache_path(args: &SearchArgs, scope: &str) -> Result<PathBuf> {
    let engine = format!("{:?}", args.engine).to_lowercase();
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in scope
        .bytes()
        .chain([0])
        .chain(engine.bytes())
        .chain([0])
        .chain(args.query.bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    Ok(config::search_cache_dir()?.join(format!("{hash:016x}.json")))
}

fn truncate_search_snippets(results: &mut [Value], max_chars: usize) {
    for result in results {
        let Some(snippet) = result.get_mut("snippet") else {
            continue;
        };
        let Some(text) = snippet.as_str() else {
            continue;
        };
        if text.chars().count() > max_chars {
            *snippet = json!(text.chars().take(max_chars).collect::<String>());
        }
    }
}

async fn enrich_search_results(
    client: &mut CdpClient,
    search: &mut Value,
    content_chars: usize,
) -> Result<()> {
    let urls = search
        .get("results")
        .and_then(Value::as_array)
        .map(|results| {
            results
                .iter()
                .filter_map(|result| result.get("url").and_then(Value::as_str).map(str::to_owned))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let mut contents = read_search_results(client, &urls).await?;
    for page in &mut contents {
        if let Some(text) = page.get("text").and_then(Value::as_str) {
            page["text"] = json!(text.chars().take(content_chars).collect::<String>());
        }
    }
    search["contents"] = json!(contents);
    Ok(())
}

async fn read_search_results(client: &mut CdpClient, urls: &[String]) -> Result<Vec<Value>> {
    if urls.is_empty() {
        return Ok(Vec::new());
    }
    let target = client.create_target("about:blank").await?;
    let pages_result = async {
        client.attach(&target.target_id).await?;
        let mut pages = Vec::with_capacity(urls.len());
        for url in urls {
            // navigate now waits for the new document, not the previous page's
            // readyState. Do not replay an entire batch after genuine failures.
            client.navigate(url).await?;
            let page = client.evaluate(scripts::readable(), true).await?;
            validate_content_page(&page, url)?;
            pages.push(page);
        }
        Ok(pages)
    }
    .await;
    let close_result = client.close_target(&target.target_id).await;
    match pages_result {
        Ok(pages) => close_result.map(|_| pages),
        Err(error) => Err(error),
    }
}

fn validate_content_page(page: &Value, requested_url: &str) -> Result<()> {
    let loaded_url = page.get("url").and_then(Value::as_str).unwrap_or_default();
    if matches!(url::Url::parse(loaded_url), Ok(url) if matches!(url.scheme(), "http" | "https")) {
        return Ok(());
    }
    Err(Error::Protocol {
        method: "search --with-content".to_owned(),
        message: format!("expected {requested_url}, loaded {loaded_url}"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_cache_is_scoped_to_the_browser_session() {
        use clap::Parser;
        let cli = crate::cli::Cli::parse_from(["lsearch", "search", "fixture"]);
        let Some(crate::cli::Command::Search(args)) = cli.command else {
            panic!("search arguments")
        };
        let first =
            super::search_cache_path(&args, "ws://127.0.0.1:1/devtools/browser/first").unwrap();
        let second =
            super::search_cache_path(&args, "ws://127.0.0.1:1/devtools/browser/second").unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn brave_search_uses_the_public_browser_surface() {
        assert_eq!(search_engine_label(SearchEngine::Brave), "Brave Search");
        assert_eq!(
            search_engine_url(SearchEngine::Brave, "rust browser"),
            "https://search.brave.com/search?q=rust+browser"
        );
    }

    #[test]
    fn automatic_search_output_only_uses_tables_in_terminals() {
        assert_eq!(
            resolve_search_format(SearchFormat::Auto, false, false, true),
            SearchFormat::Table
        );
        assert_eq!(
            resolve_search_format(SearchFormat::Auto, false, false, false),
            SearchFormat::Json
        );
    }

    #[test]
    fn explicit_json_and_pretty_output_override_a_terminal_table() {
        assert_eq!(
            resolve_search_format(SearchFormat::Table, true, false, true),
            SearchFormat::Json
        );
        assert_eq!(
            resolve_search_format(SearchFormat::Table, false, true, true),
            SearchFormat::Json
        );
    }

    #[test]
    fn search_snippets_are_truncated_on_character_boundaries() {
        let mut results = vec![json!({ "snippet": "ab🦀cd" })];

        truncate_search_snippets(&mut results, 3);

        assert_eq!(results[0]["snippet"], "ab🦀");
    }

    #[test]
    fn short_search_snippets_are_unchanged() {
        let mut results = vec![json!({ "snippet": "short" })];

        truncate_search_snippets(&mut results, 10);

        assert_eq!(results[0]["snippet"], "short");
    }

    #[test]
    fn content_page_rejects_about_blank() {
        let error = validate_content_page(
            &json!({ "url": "about:blank" }),
            "https://example.com/article",
        )
        .unwrap_err();

        assert!(error.to_string().contains("loaded about:blank"));
    }

    #[test]
    fn content_page_accepts_http_redirects() {
        validate_content_page(
            &json!({ "url": "https://www.example.com/article" }),
            "https://example.com/article",
        )
        .unwrap();
    }
}
