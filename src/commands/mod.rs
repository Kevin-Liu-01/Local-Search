use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

use chrono::Utc;
use serde_json::{Value, json};

use crate::{
    browser::{BrowserEndpoint, CdpClient, discovery, scripts},
    cli::{
        BrowserKind, Cli, Command, ConnectArgs, CookiesCommand, LaunchArgs, MapArgs, OpenArgs,
        OptionalPathArgs, PathArgs, ReadArgs, ReadFormat, RecordArgs, RequestArgs, ScreenshotArgs,
        ScrollDirection, SearchArgs, SearchEngine, TabsCommand, WaitArgs,
    },
    config::{self, Config},
    error::{Error, IoContext, Result},
    output::print_json,
};

pub async fn run(cli: Cli) -> Result<()> {
    match &cli.command {
        None => {
            let query = cli.query.join(" ");
            if query.trim().is_empty() {
                return Err(Error::InvalidArgument(
                    "provide a query or subcommand".to_owned(),
                ));
            }
            let args = SearchArgs {
                query,
                engine: SearchEngine::Google,
                limit: 10,
                snippet_chars: 120,
                cache_ttl: 300,
                no_cache: false,
                with_content: false,
                content_chars: 2_000,
                new_tab: false,
            };
            search_command(&cli, &args).await
        }
        Some(Command::Doctor) => print_json(&discovery::doctor().await, cli.pretty),
        Some(Command::Connect(args)) => connect(&cli, args).await,
        Some(Command::Launch(args)) => launch(&cli, args).await,
        Some(Command::Cleanup(args)) => cleanup(&cli, args).await,
        Some(Command::Tabs(command)) => tabs(&cli, command).await,
        Some(Command::Search(args)) => search_command(&cli, args).await,
        Some(command) => {
            let mut client = cdp_client(&cli).await?;
            match command {
                Command::Map(args) => map(&cli, &mut client, args).await,
                Command::Open(args) => open(&cli, &mut client, args).await,
                Command::Read(args) => read(&cli, &mut client, args).await,
                Command::Snapshot(args) => {
                    let value = client
                        .evaluate(&scripts::snapshot(args.all, args.limit), true)
                        .await?;
                    print_json(&json!({ "ok": true, "snapshot": value }), cli.pretty)
                }
                Command::Extract(args) => {
                    let fields = args
                        .fields
                        .iter()
                        .map(|field| scripts::parse_field(field).map_err(Error::InvalidArgument))
                        .collect::<Result<Vec<_>>>()?;
                    let value = client
                        .evaluate(&scripts::extract(&args.selector, &fields, args.limit), true)
                        .await?;
                    print_json(&json!({ "ok": true, "records": value }), cli.pretty)
                }
                Command::Eval(args) => {
                    let value = client.evaluate(&args.expression, true).await?;
                    print_json(&json!({ "ok": true, "value": value }), cli.pretty)
                }
                Command::Click(args) => {
                    let value = client.evaluate(&scripts::click(&args.target), true).await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Fill(args) => {
                    let value = client
                        .evaluate(&scripts::fill(&args.target, &args.value, false), true)
                        .await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Type(args) => {
                    let value = client
                        .evaluate(&scripts::fill(&args.target, &args.value, true), true)
                        .await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Press(args) => {
                    let value = client.press(&args.key).await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Hover(args) => {
                    let value = client.evaluate(&scripts::hover(&args.target), true).await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Select(args) => {
                    let value = client
                        .evaluate(&scripts::select(&args.target, &args.value), true)
                        .await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Scroll(args) => {
                    let direction = match args.direction {
                        ScrollDirection::Up => "up",
                        ScrollDirection::Down => "down",
                        ScrollDirection::Left => "left",
                        ScrollDirection::Right => "right",
                    };
                    let value = client
                        .evaluate(
                            &scripts::scroll(direction, args.amount, args.target.as_deref()),
                            true,
                        )
                        .await?;
                    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
                }
                Command::Wait(args) => wait(&cli, &mut client, args).await,
                Command::Screenshot(args) => screenshot(&cli, &mut client, args).await,
                Command::Pdf(args) => pdf(&cli, &mut client, args).await,
                Command::Mhtml(args) => mhtml(&cli, &mut client, args).await,
                Command::Html(args) => html(&cli, &mut client, args).await,
                Command::Request(args) => request(&cli, &mut client, args).await,
                Command::Record(args) => record(&cli, &mut client, args).await,
                Command::Back => {
                    let value = client.history(-1).await?;
                    print_json(&json!({ "ok": true, "page": value }), cli.pretty)
                }
                Command::Forward => {
                    let value = client.history(1).await?;
                    print_json(&json!({ "ok": true, "page": value }), cli.pretty)
                }
                Command::Reload => {
                    let value = client.reload().await?;
                    print_json(&json!({ "ok": true, "page": value }), cli.pretty)
                }
                Command::Cookies(command) => cookies(&cli, &mut client, command).await,
                Command::Doctor
                | Command::Connect(_)
                | Command::Launch(_)
                | Command::Cleanup(_)
                | Command::Search(_)
                | Command::Tabs(_) => unreachable!("handled before cdp attach"),
            }
        }
    }
}

async fn connect(cli: &Cli, args: &ConnectArgs) -> Result<()> {
    let endpoint =
        discovery::discover(cli.browser, args.endpoint.as_deref().or(cli.cdp.as_deref())).await?;
    let path = config::save(&Config {
        endpoint: Some(endpoint.websocket_url.clone()),
        target_id: cli.target.clone(),
    })
    .await?;
    print_json(
        &json!({ "ok": true, "endpoint": endpoint, "config": path.display().to_string() }),
        cli.pretty,
    )
}

async fn launch(cli: &Cli, args: &LaunchArgs) -> Result<()> {
    let launched = start_managed_browser(args).await?;
    print_json(
        &json!({
            "ok": true,
            "alreadyRunning": launched.already_running,
            "pid": launched.pid,
            "profile": launched.profile.display().to_string(),
            "endpoint": launched.endpoint,
            "persisted": !args.no_persist,
        }),
        cli.pretty,
    )
}

struct ManagedLaunch {
    endpoint: BrowserEndpoint,
    pid: u32,
    profile: PathBuf,
    already_running: bool,
}

async fn start_managed_browser(args: &LaunchArgs) -> Result<ManagedLaunch> {
    if let Ok(endpoint) =
        discovery::discover(BrowserKind::Chromium, Some(&args.port.to_string())).await
    {
        if !args.no_persist {
            config::save(&Config {
                endpoint: Some(endpoint.websocket_url.clone()),
                target_id: None,
            })
            .await?;
        }
        return Ok(ManagedLaunch {
            endpoint,
            pid: 0,
            profile: args
                .profile
                .clone()
                .unwrap_or(config::managed_profile_dir()?),
            already_running: true,
        });
    }

    let profile = args
        .profile
        .clone()
        .unwrap_or(config::managed_profile_dir()?);
    std::fs::create_dir_all(&profile).at(config::display_path(&profile))?;
    clear_stale_chrome_profile_markers(&profile)?;
    let browser_path = args
        .browser_path
        .clone()
        .or_else(default_chrome_path)
        .ok_or_else(|| {
            Error::InvalidArgument("could not find Chrome; pass --browser-path".to_owned())
        })?;

    let chrome_args = chrome_launch_args(args, &profile);
    let mut command = chrome_command(&browser_path, &chrome_args);
    let child = command.spawn().map_err(|source| Error::Io {
        path: config::display_path(&browser_path),
        source,
    })?;

    let mut endpoint = None;
    for _ in 0..50 {
        if let Ok(found) =
            discovery::discover(BrowserKind::Chromium, Some(&args.port.to_string())).await
        {
            endpoint = Some(found);
            break;
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    let endpoint = endpoint.ok_or_else(|| Error::Timeout {
        operation: "managed Chrome startup".to_owned(),
        timeout_ms: 10_000,
    })?;

    if !args.no_persist {
        config::save(&Config {
            endpoint: Some(endpoint.websocket_url.clone()),
            target_id: None,
        })
        .await?;
        write_managed_pid(child.id())?;
    }

    Ok(ManagedLaunch {
        endpoint,
        pid: child.id(),
        profile,
        already_running: false,
    })
}

async fn cleanup(cli: &Cli, args: &crate::cli::CleanupArgs) -> Result<()> {
    let profile = args
        .profile
        .clone()
        .unwrap_or(config::managed_profile_dir()?);
    let marker_files = managed_profile_marker_files(&profile);
    let existing_marker_files = marker_files
        .iter()
        .filter(|path| path.exists())
        .map(|path| config::display_path(path))
        .collect::<Vec<_>>();
    let pids = managed_browser_pids(args.port)?;
    let pid_file = config::managed_pid_file()?;

    if args.kill {
        for pid in &pids {
            terminate_pid(*pid, args.force).await?;
        }
        for path in &marker_files {
            if path.exists() {
                std::fs::remove_file(path).at(config::display_path(path))?;
            }
        }
        if pid_file.exists() {
            std::fs::remove_file(&pid_file).at(config::display_path(&pid_file))?;
        }
        config::save(&Config::default()).await?;
    }

    print_json(
        &json!({
            "ok": true,
            "dryRun": !args.kill,
            "port": args.port,
            "pids": pids,
            "profile": config::display_path(&profile),
            "pidFile": config::display_path(&pid_file),
            "profileMarkerFiles": existing_marker_files,
            "actions": if args.kill {
                json!(["terminated managed browser pids", "removed stale profile marker files", "cleared saved endpoint"])
            } else {
                json!(["pass --kill to terminate managed browser pids and remove stale profile marker files"])
            },
        }),
        cli.pretty,
    )
}

fn clear_stale_chrome_profile_markers(profile: &Path) -> Result<()> {
    for path in managed_profile_marker_files(profile) {
        if path.exists() {
            std::fs::remove_file(&path).at(config::display_path(&path))?;
        }
    }
    Ok(())
}

fn managed_profile_marker_files(profile: &Path) -> Vec<PathBuf> {
    [
        "DevToolsActivePort",
        "SingletonCookie",
        "SingletonLock",
        "SingletonSocket",
    ]
    .iter()
    .map(|name| profile.join(name))
    .collect()
}

fn write_managed_pid(pid: u32) -> Result<()> {
    let path = config::managed_pid_file()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).at(config::display_path(parent))?;
    }
    std::fs::write(&path, pid.to_string()).at(config::display_path(&path))
}

fn managed_browser_pids(port: u16) -> Result<Vec<u32>> {
    let mut pids = Vec::new();
    let pid_file = config::managed_pid_file()?;
    if let Ok(raw) = std::fs::read_to_string(&pid_file)
        && let Ok(pid) = raw.trim().parse::<u32>()
        && pid_is_live(pid)
    {
        pids.push(pid);
    }

    let output = std::process::Command::new("lsof")
        .args(["-nP", "-tiTCP", &format!(":{port}"), "-sTCP:LISTEN"])
        .output();
    if let Ok(output) = output
        && output.status.success()
    {
        let raw = String::from_utf8_lossy(&output.stdout);
        for line in raw.lines() {
            if let Ok(pid) = line.trim().parse::<u32>()
                && pid_is_live(pid)
                && !pids.contains(&pid)
            {
                pids.push(pid);
            }
        }
    }
    Ok(pids)
}

fn pid_is_live(pid: u32) -> bool {
    std::process::Command::new("ps")
        .args(["-p", &pid.to_string()])
        .output()
        .is_ok_and(|output| output.status.success())
}

async fn terminate_pid(pid: u32, force: bool) -> Result<()> {
    if pid == std::process::id() {
        return Ok(());
    }
    let signal = if force { "-KILL" } else { "-TERM" };
    std::process::Command::new("kill")
        .args([signal, &pid.to_string()])
        .status()
        .map_err(|source| Error::Io {
            path: "kill".to_owned(),
            source,
        })?;
    tokio::time::sleep(Duration::from_millis(750)).await;
    if !force && pid_is_live(pid) {
        std::process::Command::new("kill")
            .args(["-KILL", &pid.to_string()])
            .status()
            .map_err(|source| Error::Io {
                path: "kill".to_owned(),
                source,
            })?;
    }
    Ok(())
}

fn chrome_launch_args(args: &LaunchArgs, profile: &Path) -> Vec<String> {
    let mut chrome_args = vec![
        "--remote-debugging-address=127.0.0.1".to_owned(),
        format!("--remote-debugging-port={}", args.port),
        "--remote-allow-origins=*".to_owned(),
        format!("--user-data-dir={}", profile.display()),
        "--no-first-run".to_owned(),
        "--no-default-browser-check".to_owned(),
    ];
    if args.headless {
        chrome_args.push("--headless=new".to_owned());
    }
    chrome_args.push(args.url.clone());
    chrome_args
}

fn chrome_command(browser_path: &Path, chrome_args: &[String]) -> std::process::Command {
    let mut command = std::process::Command::new(browser_path);
    command
        .args(chrome_args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    command
}

async fn cdp_client(cli: &Cli) -> Result<CdpClient> {
    if !matches!(cli.browser, BrowserKind::Auto | BrowserKind::Chromium) {
        return Err(Error::Unsupported {
            backend: "safari".to_owned(),
            feature: "this command".to_owned(),
        });
    }
    let (endpoint, mut client) = connect_or_launch_managed(cli).await?;
    let stored = config::load().await.ok().and_then(|cfg| cfg.target_id);
    if let Some(target_id) = cli.target.as_deref() {
        client.attach_or_create(Some(target_id)).await?;
    } else if let Some(target_id) = stored.as_deref() {
        match client.attach_or_create(Some(target_id)).await {
            Ok(_) => {}
            Err(Error::TargetNotFound(_)) => {
                config::save(&Config {
                    endpoint: Some(endpoint.websocket_url),
                    target_id: None,
                })
                .await?;
                client.attach_or_create(None).await?;
            }
            Err(error) => return Err(error),
        }
    } else {
        client.attach_or_create(None).await?;
    }
    Ok(client)
}

async fn connect_or_launch_managed(cli: &Cli) -> Result<(BrowserEndpoint, CdpClient)> {
    if cli.cdp.is_some() || !matches!(cli.browser, BrowserKind::Auto | BrowserKind::Chromium) {
        let endpoint = discovery::discover(cli.browser, cli.cdp.as_deref()).await?;
        let client = CdpClient::connect(&endpoint.websocket_url, cli.timeout).await?;
        return Ok((endpoint, client));
    }

    let args = LaunchArgs {
        port: 9322,
        profile: None,
        browser_path: None,
        headless: false,
        url: "about:blank".to_owned(),
        no_persist: false,
    };
    let launched = start_managed_browser(&args).await?;
    let client = CdpClient::connect(&launched.endpoint.websocket_url, cli.timeout).await?;
    Ok((launched.endpoint, client))
}

async fn open(cli: &Cli, client: &mut CdpClient, args: &OpenArgs) -> Result<()> {
    if args.new_tab {
        let target = client.create_target(&args.url).await?;
        client.attach(&target.target_id).await?;
    } else {
        client.navigate(&args.url).await?;
    }
    let page = client.page_info().await?;
    print_json(
        &json!({ "ok": true, "page": page, "targetId": client.target_id() }),
        cli.pretty,
    )
}

async fn search_command(cli: &Cli, args: &SearchArgs) -> Result<()> {
    if let Some(mut value) = load_search_cache(args).await {
        prepare_search_results(&mut value, args);
        if args.with_content {
            let mut client = cdp_client(cli).await?;
            enrich_search_results(&mut client, &mut value, args.content_chars).await?;
        }
        return print_search(cli, args, &value);
    }

    let mut client = cdp_client(cli).await?;
    search(cli, &mut client, args).await
}

async fn search(cli: &Cli, client: &mut CdpClient, args: &SearchArgs) -> Result<()> {
    let url = match args.engine {
        SearchEngine::Google => format!(
            "https://www.google.com/search?q={}",
            urlencoding(&args.query)
        ),
        SearchEngine::Bing => format!("https://www.bing.com/search?q={}", urlencoding(&args.query)),
        SearchEngine::Duckduckgo => {
            format!(
                "https://html.duckduckgo.com/html/?q={}",
                urlencoding(&args.query)
            )
        }
    };
    if args.new_tab {
        let target = client.create_target("about:blank").await?;
        client.attach(&target.target_id).await?;
    }
    client.start_navigation(&url).await?;
    let result_selector = match args.engine {
        SearchEngine::Google => "a h3",
        SearchEngine::Bing => "li.b_algo h2 a",
        SearchEngine::Duckduckgo => ".result__a",
    };
    let requested_results = args.limit.clamp(1, 3);
    let query = serde_json::to_string(&args.query)?;
    client
        .wait_for_js(&format!(
            "(() => {{ const body = document.body?.innerText || ''; const onQuery = new URL(location.href).searchParams.get('q') === {query}; const resultsReady = document.querySelectorAll('{result_selector}').length >= {requested_results}; const genericReady = document.querySelectorAll('a[href]').length > 5 && body.length > 100; return (onQuery && (resultsReady || genericReady)) || /captcha|unusual traffic|verify you are human|solve the challenge|one last step/i.test(body); }})()"
        ))
        .await?;
    let mut value = client.evaluate(scripts::search_results(), true).await?;
    save_search_cache(args, &value).await;
    prepare_search_results(&mut value, args);
    if args.with_content {
        enrich_search_results(client, &mut value, args.content_chars).await?;
    }
    print_search(cli, args, &value)
}

fn prepare_search_results(value: &mut Value, args: &SearchArgs) {
    if let Some(results) = value.get_mut("results").and_then(Value::as_array_mut) {
        results.truncate(args.limit);
        truncate_search_snippets(results, args.snippet_chars);
    }
}

fn print_search(cli: &Cli, args: &SearchArgs, value: &Value) -> Result<()> {
    print_json(
        &json!({ "ok": true, "query": args.query, "engine": format!("{:?}", args.engine).to_lowercase(), "search": value }),
        cli.pretty,
    )
}

async fn load_search_cache(args: &SearchArgs) -> Option<Value> {
    if args.no_cache || args.cache_ttl == 0 {
        return None;
    }
    let path = search_cache_path(args).ok()?;
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
    if record.get("query").and_then(Value::as_str) != Some(args.query.as_str())
        || record.get("engine").and_then(Value::as_str) != Some(engine.as_str())
    {
        return None;
    }
    let search = record.get("search")?.clone();
    if search.get("results")?.as_array()?.len() < args.limit {
        return None;
    }
    Some(search)
}

async fn save_search_cache(args: &SearchArgs, search: &Value) {
    if args.no_cache || args.cache_ttl == 0 {
        return;
    }
    let Ok(path) = search_cache_path(args) else {
        return;
    };
    let Some(parent) = path.parent() else {
        return;
    };
    if tokio::fs::create_dir_all(parent).await.is_err() {
        return;
    }
    let record = json!({
        "engine": format!("{:?}", args.engine).to_lowercase(),
        "query": args.query,
        "search": search,
    });
    if let Ok(bytes) = serde_json::to_vec(&record) {
        let _ = tokio::fs::write(path, bytes).await;
    }
}

fn search_cache_path(args: &SearchArgs) -> Result<PathBuf> {
    let engine = format!("{:?}", args.engine).to_lowercase();
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in engine.bytes().chain([0]).chain(args.query.bytes()) {
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
    for attempt in 0..3 {
        let target = client.create_target("about:blank").await?;
        let pages_result = async {
            client.attach(&target.target_id).await?;
            let mut pages = Vec::with_capacity(urls.len());
            for url in urls {
                client.navigate(url).await?;
                client
                    .wait_for_js(
                        "location.href !== 'about:blank' && (document.readyState === 'interactive' || document.readyState === 'complete')",
                    )
                    .await?;
                let page = client.evaluate(scripts::readable(), true).await?;
                validate_content_page(&page, url)?;
                pages.push(page);
            }
            Ok(pages)
        }
        .await;
        let close_result = client.close_target(&target.target_id).await;
        let result = match pages_result {
            Ok(pages) => close_result.map(|_| pages),
            Err(error) => {
                let _ = close_result;
                Err(error)
            }
        };
        match result {
            Ok(pages) => return Ok(pages),
            Err(error) if attempt == 2 => return Err(error),
            Err(_) => {
                tokio::time::sleep(Duration::from_millis(100 * (attempt + 1))).await;
            }
        }
    }
    unreachable!("temporary page read attempts are non-empty");
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

async fn map(cli: &Cli, client: &mut CdpClient, args: &MapArgs) -> Result<()> {
    let start = url::Url::parse(&args.url)?;
    let origin = start.origin().unicode_serialization();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([(start.to_string(), 0_usize)]);
    let mut pages = Vec::new();

    while let Some((url, depth)) = queue.pop_front() {
        if pages.len() >= args.limit || !seen.insert(url.clone()) {
            continue;
        }
        client.navigate(&url).await?;
        let page = client.evaluate(scripts::map_links(), true).await?;
        if depth < args.depth
            && let Some(links) = page.get("links").and_then(Value::as_array)
        {
            for href in links
                .iter()
                .filter_map(|link| link.get("url").and_then(Value::as_str))
            {
                let Ok(parsed) = url::Url::parse(href) else {
                    continue;
                };
                if parsed.origin().unicode_serialization() == origin && !seen.contains(href) {
                    queue.push_back((href.to_owned(), depth + 1));
                }
                if seen.len() + queue.len() >= args.limit {
                    break;
                }
            }
        }
        pages.push(page);
    }

    print_json(
        &json!({ "ok": true, "url": args.url, "origin": origin, "pages": pages }),
        cli.pretty,
    )
}

async fn read(cli: &Cli, client: &mut CdpClient, args: &ReadArgs) -> Result<()> {
    if let Some(url) = &args.url {
        client.navigate(url).await?;
    }
    let value = client.evaluate(scripts::readable(), true).await?;
    match args.format {
        ReadFormat::Json => print_json(&json!({ "ok": true, "page": value }), cli.pretty),
        ReadFormat::Text => {
            println!(
                "{}",
                value.get("text").and_then(Value::as_str).unwrap_or("")
            );
            Ok(())
        }
        ReadFormat::Markdown => {
            let title = value
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or("Untitled");
            let url = value.get("url").and_then(Value::as_str).unwrap_or("");
            let text = value.get("text").and_then(Value::as_str).unwrap_or("");
            println!("# {title}\n\n<{url}>\n\n{text}");
            Ok(())
        }
    }
}

async fn wait(cli: &Cli, client: &mut CdpClient, args: &WaitArgs) -> Result<()> {
    if let Some(ms) = args.ms {
        tokio::time::sleep(Duration::from_millis(ms)).await;
        return print_json(&json!({ "ok": true, "waitedMs": ms }), cli.pretty);
    }
    let expression = if let Some(selector) = &args.selector {
        format!(
            "document.querySelector({}) !== null",
            scripts::string(selector)
        )
    } else if let Some(text) = &args.text {
        format!(
            "document.body.innerText.includes({})",
            scripts::string(text)
        )
    } else if let Some(url) = &args.url {
        format!("location.href.includes({})", scripts::string(url))
    } else {
        return Err(Error::InvalidArgument(
            "wait requires --ms, --selector, --text, or --url".to_owned(),
        ));
    };
    let value = client.wait_for_js(&expression).await?;
    print_json(&json!({ "ok": true, "result": value }), cli.pretty)
}

async fn screenshot(cli: &Cli, client: &mut CdpClient, args: &ScreenshotArgs) -> Result<()> {
    let data = client.screenshot(args.full_page).await?;
    write_bytes(&args.path, &data)?;
    print_json(
        &json!({ "ok": true, "path": args.path.display().to_string(), "bytes": data.len() }),
        cli.pretty,
    )
}

async fn pdf(cli: &Cli, client: &mut CdpClient, args: &PathArgs) -> Result<()> {
    let data = client.pdf().await?;
    write_bytes(&args.path, &data)?;
    print_json(
        &json!({ "ok": true, "path": args.path.display().to_string(), "bytes": data.len() }),
        cli.pretty,
    )
}

async fn mhtml(cli: &Cli, client: &mut CdpClient, args: &PathArgs) -> Result<()> {
    let data = client.mhtml().await?;
    write_bytes(&args.path, data.as_bytes())?;
    print_json(
        &json!({ "ok": true, "path": args.path.display().to_string(), "bytes": data.len() }),
        cli.pretty,
    )
}

async fn html(cli: &Cli, client: &mut CdpClient, args: &OptionalPathArgs) -> Result<()> {
    let html = client.evaluate(scripts::rendered_html(), true).await?;
    let html = html.as_str().unwrap_or("").to_owned();
    if let Some(path) = &args.path {
        write_bytes(path, html.as_bytes())?;
        print_json(
            &json!({ "ok": true, "path": path.display().to_string(), "bytes": html.len() }),
            cli.pretty,
        )
    } else {
        println!("{html}");
        Ok(())
    }
}

async fn request(cli: &Cli, client: &mut CdpClient, args: &RequestArgs) -> Result<()> {
    let headers = parse_headers(&args.headers)?;
    let url = url::Url::parse(&args.url)?;
    let temp_target = client
        .create_target(&url.origin().unicode_serialization())
        .await?;
    client.attach(&temp_target.target_id).await?;
    client.wait_ready().await?;
    let value = client
        .evaluate(
            &scripts::browser_fetch(
                &args.url,
                &args.method,
                &json!(headers),
                args.body.as_deref(),
            ),
            true,
        )
        .await?;
    client.close_target(&temp_target.target_id).await?;
    print_json(&json!({ "ok": true, "response": value }), cli.pretty)
}

async fn record(cli: &Cli, client: &mut CdpClient, args: &RecordArgs) -> Result<()> {
    client.enable_recording().await?;
    client.navigate(&args.url).await?;
    let events = client
        .drain_events_for(Duration::from_millis(args.duration))
        .await;
    let har = events_to_har(&args.url, &events);
    write_bytes(&args.har, serde_json::to_vec_pretty(&har)?.as_slice())?;
    let mut out =
        json!({ "ok": true, "har": args.har.display().to_string(), "eventCount": events.len() });
    if let Some(path) = &args.mhtml {
        let mhtml = client.mhtml().await?;
        write_bytes(path, mhtml.as_bytes())?;
        out["mhtml"] = json!(path.display().to_string());
    }
    print_json(&out, cli.pretty)
}

async fn tabs(cli: &Cli, command: &TabsCommand) -> Result<()> {
    let (endpoint, mut client) = connect_or_launch_managed(cli).await?;
    match command {
        TabsCommand::List => {
            let targets = client.targets().await?;
            print_json(&json!({ "ok": true, "tabs": targets }), cli.pretty)
        }
        TabsCommand::New { url } => {
            let target = client
                .create_target(url.as_deref().unwrap_or("about:blank"))
                .await?;
            print_json(&json!({ "ok": true, "tab": target }), cli.pretty)
        }
        TabsCommand::Use { target_id } => {
            config::save(&Config {
                endpoint: Some(endpoint.websocket_url),
                target_id: Some(target_id.clone()),
            })
            .await?;
            print_json(&json!({ "ok": true, "targetId": target_id }), cli.pretty)
        }
        TabsCommand::Close { target_id } => {
            let id = target_id
                .as_deref()
                .or(cli.target.as_deref())
                .ok_or_else(|| {
                    Error::InvalidArgument("close needs target id or --target".to_owned())
                })?;
            let value = client.close_target(id).await?;
            print_json(&json!({ "ok": true, "result": value }), cli.pretty)
        }
    }
}

async fn cookies(cli: &Cli, client: &mut CdpClient, command: &CookiesCommand) -> Result<()> {
    match command {
        CookiesCommand::List { url, show_values } => {
            let params = url
                .as_ref()
                .map_or_else(|| json!({}), |url| json!({ "urls": [url] }));
            let mut value = client.send_page("Network.getCookies", params).await?;
            if !show_values
                && let Some(cookies) = value.get_mut("cookies").and_then(Value::as_array_mut)
            {
                for cookie in cookies {
                    cookie["value"] = json!("<redacted>");
                }
            }
            print_json(&json!({ "ok": true, "cookies": value }), cli.pretty)
        }
        CookiesCommand::Set { name, value, url } => {
            let result = client
                .send_page(
                    "Network.setCookie",
                    json!({ "name": name, "value": value, "url": url }),
                )
                .await?;
            print_json(&json!({ "ok": true, "result": result }), cli.pretty)
        }
        CookiesCommand::Delete { name, url } => {
            let result = client
                .send_page("Network.deleteCookies", json!({ "name": name, "url": url }))
                .await?;
            print_json(&json!({ "ok": true, "result": result }), cli.pretty)
        }
    }
}

fn write_bytes(path: &Path, data: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).at(parent.display().to_string())?;
    }
    std::fs::write(path, data).at(path.display().to_string())
}

fn default_chrome_path() -> Option<PathBuf> {
    std::env::var_os("LOCAL_SEARCH_CHROME")
        .or_else(|| std::env::var_os("LOCAL_BROWSER_CHROME"))
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .or_else(|| {
            [
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                "/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary",
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
                "/usr/bin/google-chrome",
                "/usr/bin/chromium",
                "/usr/bin/chromium-browser",
            ]
            .iter()
            .map(PathBuf::from)
            .find(|path| path.exists())
        })
}

fn parse_headers(headers: &[String]) -> Result<BTreeMap<String, String>> {
    headers
        .iter()
        .map(|header| {
            header
                .split_once(':')
                .map(|(key, value)| (key.trim().to_owned(), value.trim().to_owned()))
                .ok_or_else(|| {
                    Error::InvalidArgument(format!("header must be Name: Value: {header}"))
                })
        })
        .collect()
}

fn events_to_har(url: &str, events: &[Value]) -> Value {
    let entries: Vec<Value> = events
        .iter()
        .filter(|event| {
            event
                .get("method")
                .and_then(Value::as_str)
                .is_some_and(|method| method.starts_with("Network."))
        })
        .cloned()
        .collect();
    json!({
        "log": {
            "version": "1.2",
            "creator": { "name": "local-search", "version": env!("CARGO_PKG_VERSION") },
            "pages": [{
                "startedDateTime": Utc::now().to_rfc3339(),
                "id": "page_1",
                "title": url,
                "pageTimings": {}
            }],
            "entries": entries
        }
    })
}

fn urlencoding(input: &str) -> String {
    url::form_urlencoded::byte_serialize(input.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{truncate_search_snippets, validate_content_page};

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
