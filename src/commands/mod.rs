use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    io::{self, IsTerminal as _},
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
        ScrollDirection, SearchArgs, SearchEngine, SearchFormat, TabsCommand, WaitArgs,
    },
    config::{self, Config, Connection},
    error::{Error, IoContext, Result},
    output::print_json,
    ui::{self, Spinner},
};

pub async fn run(cli: Cli) -> Result<()> {
    match &cli.command {
        None => {
            let query = cli.query.join(" ");
            if query.trim().is_empty() {
                return ui::print_welcome();
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
                format: SearchFormat::Auto,
            };
            search_command(&cli, &args).await
        }
        Some(Command::UpdateCheck) => crate::updates::print_check(cli.pretty).await,
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
                Command::UpdateCheck
                | Command::Doctor
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
    if (args.existing || args.managed) && cli.cdp.is_some() {
        return Err(Error::InvalidArgument(
            "choose either --existing/--managed or --cdp, not both (also check LOCAL_SEARCH_CDP)"
                .to_owned(),
        ));
    }
    if cli.browser == BrowserKind::Safari {
        return Err(Error::Unsupported {
            backend: "safari".to_owned(),
            feature: "persistent browser connection".to_owned(),
        });
    }
    // Never overwrite a malformed selection as a side effect of connecting.
    let saved = config::load().await?;
    if args.managed {
        return launch(cli, &default_launch_args()).await;
    }
    let spinner = Spinner::start(if args.existing {
        "Waiting for Chrome approval — choose Allow in your browser"
    } else {
        "Verifying browser connection"
    });
    let (endpoint, connection) = if args.existing {
        let profile = args
            .profile
            .clone()
            .unwrap_or(discovery::existing_chrome_profile()?);
        let profile = std::fs::canonicalize(&profile).map_err(|_| existing_disconnected())?;
        let endpoint = discovery::endpoint_from_devtools_file(&profile.join("DevToolsActivePort"))
            .await
            .map_err(|_| existing_disconnected())?;
        (endpoint, Some(Connection::Existing { profile }))
    } else if let Some(value) = args.endpoint.as_deref().or(cli.cdp.as_deref()) {
        (
            resolve_endpoint(cli, value).await?,
            Some(Connection::Endpoint),
        )
    } else {
        (
            selected_endpoint(cli, &saved).await?,
            saved.connection.clone(),
        )
    };
    let timeout = if args.existing {
        cli.timeout.max(60_000)
    } else {
        cli.timeout
    };
    verify_connection(&endpoint, timeout).await.map_err(|_| {
        if matches!(connection, Some(Connection::Existing { .. })) {
            existing_disconnected()
        } else {
            Error::BrowserDisconnected(
                "connection could not be verified; check the endpoint and retry connect".to_owned(),
            )
        }
    })?;
    let path = config::save(&Config {
        endpoint: Some(endpoint.websocket_url.clone()),
        target_id: cli.target.clone(),
        connection,
    })
    .await?;
    spinner.success("Browser connected");
    print_json(
        &json!({ "ok": true, "endpoint": endpoint, "connection": config::load().await?.connection, "config": path.display().to_string() }),
        cli.pretty,
    )
}

async fn launch(cli: &Cli, args: &LaunchArgs) -> Result<()> {
    let saved = config::load().await?;
    let args = remembered_launch_args(args, &saved);
    let spinner = Spinner::start("Starting managed Chrome");
    let launched = start_managed_browser(&args).await?;
    verify_connection(&launched.endpoint, cli.timeout).await?;
    if !args.no_persist {
        save_managed_connection(&args, &launched).await?;
    }
    spinner.success(if launched.already_running {
        "Managed Chrome is ready"
    } else {
        "Managed Chrome started"
    });
    print_json(
        &json!({
            "ok": true,
            "alreadyRunning": launched.already_running,
            "pid": launched.pid,
            "profile": launched.profile.display().to_string(),
            "endpoint": launched.endpoint,
            "persisted": !args.no_persist,
            "mode": "managed",
        }),
        cli.pretty,
    )
}

fn remembered_launch_args(args: &LaunchArgs, saved: &Config) -> LaunchArgs {
    let mut args = args.clone();
    if let Some(Connection::Managed {
        profile,
        port,
        browser_path,
    }) = &saved.connection
    {
        args.profile.get_or_insert_with(|| profile.clone());
        args.port.get_or_insert(*port);
        if args.browser_path.is_none() {
            args.browser_path.clone_from(browser_path);
        }
    }
    args.port.get_or_insert(9322);
    args
}

struct ManagedLaunch {
    endpoint: BrowserEndpoint,
    pid: u32,
    profile: PathBuf,
    already_running: bool,
}

async fn start_managed_browser(args: &LaunchArgs) -> Result<ManagedLaunch> {
    let port = args.port.unwrap_or(9322);
    let profile = args
        .profile
        .clone()
        .unwrap_or(config::managed_profile_dir()?);
    std::fs::create_dir_all(&profile).at(config::display_path(&profile))?;
    let profile = std::fs::canonicalize(&profile).at(config::display_path(&profile))?;
    // Never relaunch a user's everyday Chrome directory with automation flags.
    if discovery::existing_chrome_profile()
        .ok()
        .and_then(|p| std::fs::canonicalize(p).ok())
        .as_ref()
        == Some(&profile)
    {
        return Err(Error::InvalidArgument("use connect --existing for your everyday Chrome; managed mode needs a separate user-data directory".to_owned()));
    }
    if let Ok(Ok(endpoint)) = tokio::time::timeout(
        Duration::from_secs(2),
        discovery::discover(BrowserKind::Chromium, Some(&port.to_string())),
    )
    .await
    {
        if !managed_process_matches(port, &profile)? {
            return Err(Error::InvalidArgument(format!(
                "port {port} belongs to a different or unverified browser; choose another --port"
            )));
        }
        return Ok(ManagedLaunch {
            endpoint,
            pid: 0,
            profile,
            already_running: true,
        });
    }

    // A non-CDP listener must not cause us to start Chrome against an occupied port.
    let probe =
        std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).map_err(|_| {
            Error::InvalidArgument(format!(
                "port {port} is already in use; choose another --port"
            ))
        })?;
    drop(probe);

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
        if let Ok(Ok(found)) = tokio::time::timeout(
            Duration::from_millis(200),
            discovery::discover(BrowserKind::Chromium, Some(&port.to_string())),
        )
        .await
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

    write_managed_pid(child.id(), port)?;

    Ok(ManagedLaunch {
        endpoint,
        pid: child.id(),
        profile,
        already_running: false,
    })
}

async fn save_managed_connection(args: &LaunchArgs, launched: &ManagedLaunch) -> Result<()> {
    config::save(&Config {
        endpoint: Some(launched.endpoint.websocket_url.clone()),
        target_id: None,
        connection: Some(Connection::Managed {
            profile: launched.profile.clone(),
            port: args.port.unwrap_or(9322),
            browser_path: args.browser_path.clone(),
        }),
    })
    .await?;
    Ok(())
}

fn managed_process_matches(port: u16, profile: &Path) -> Result<bool> {
    // Verify the actual listener, not just a PID marker which may be stale.
    let out = std::process::Command::new("lsof")
        .args(["-nP", &format!("-tiTCP:{port}"), "-sTCP:LISTEN"])
        .output()
        .at("lsof")?;
    let pids: Vec<u32> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|line| line.trim().parse().ok())
        .collect();
    Ok(!pids.is_empty() && pids.iter().all(|pid| process_uses_profile(*pid, profile)))
}

fn process_uses_profile(pid: u32, profile: &Path) -> bool {
    let argument = format!("--user-data-dir={} ", profile.display());
    std::process::Command::new("ps")
        .args(["-ww", "-p", &pid.to_string(), "-o", "args="])
        .output()
        .is_ok_and(|out| {
            out.status.success()
                && format!("{} ", String::from_utf8_lossy(&out.stdout).trim()).contains(&argument)
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
    let pid_file = config::managed_pid_file(args.port)?;

    let saved_endpoint_cleared = if args.kill {
        let canonical_profile = std::fs::canonicalize(&profile).unwrap_or_else(|_| profile.clone());
        let saved = config::load().await?;
        if matches!(saved.connection, Some(Connection::Existing { profile: ref selected }) if selected == &canonical_profile)
            || discovery::existing_chrome_profile()
                .ok()
                .and_then(|p| std::fs::canonicalize(p).ok())
                .as_ref()
                == Some(&canonical_profile)
            || pids
                .iter()
                .any(|pid| !process_uses_profile(*pid, &canonical_profile))
        {
            return Err(Error::InvalidArgument("cleanup only stops verified managed browsers; it will not stop an existing browser or clear its profile markers".to_owned()));
        }
        if !pids.is_empty() {
            if args.force {
                for pid in &pids {
                    force_kill_pid(*pid)?;
                }
            } else {
                // Browser.close lets Chrome flush cookies and profile data.
                // A signal (or an implicit SIGKILL) can lose recent sessions.
                let endpoint = resolve_endpoint(cli, &args.port.to_string()).await?;
                let mut client = verify_connection(&endpoint, cli.timeout).await?;
                let close = client.close_browser().await;
                if !wait_for_browser_exit(&pids, Duration::from_secs(5)).await {
                    close?;
                    return Err(Error::InvalidArgument(
                        "Chrome has not finished closing; retry cleanup or explicitly use --force (unsaved profile data may be lost)".to_owned(),
                    ));
                }
            }
            if !wait_for_browser_exit(&pids, Duration::from_secs(5)).await {
                return Err(Error::InvalidArgument(
                    "managed Chrome is still running; profile markers were left intact".to_owned(),
                ));
            }
        }
        for path in &marker_files {
            if path.exists() {
                std::fs::remove_file(path).at(config::display_path(path))?;
            }
        }
        if pid_file.exists() {
            std::fs::remove_file(&pid_file).at(config::display_path(&pid_file))?;
        }
        clear_saved_endpoint_for_port(args.port).await?
    } else {
        false
    };

    let actions = if args.kill {
        let mut actions = vec![
            "terminated managed browser pids",
            "removed stale profile marker files",
        ];
        if saved_endpoint_cleared {
            actions.push("cleared saved endpoint");
        }
        actions
    } else {
        vec!["pass --kill to terminate managed browser pids and remove stale profile marker files"]
    };

    print_json(
        &json!({
            "ok": true,
            "dryRun": !args.kill,
            "port": args.port,
            "pids": pids,
            "profile": config::display_path(&profile),
            "pidFile": config::display_path(&pid_file),
            "profileMarkerFiles": existing_marker_files,
            "actions": actions,
        }),
        cli.pretty,
    )
}

fn clear_stale_chrome_profile_markers(profile: &Path) -> Result<()> {
    if let Ok(lock) = std::fs::read_link(profile.join("SingletonLock"))
        && let Some(pid) = lock
            .to_string_lossy()
            .rsplit('-')
            .next()
            .and_then(|value| value.parse::<u32>().ok())
        && pid_is_live(pid)
    {
        return Err(Error::InvalidArgument("this profile is already running; reconnect to it instead of launching it on another port".to_owned()));
    }
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

fn write_managed_pid(pid: u32, port: u16) -> Result<()> {
    let path = config::managed_pid_file(port)?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).at(config::display_path(parent))?;
    }
    std::fs::write(&path, pid.to_string()).at(config::display_path(&path))
}

fn managed_browser_pids(port: u16) -> Result<Vec<u32>> {
    let mut pids = Vec::new();
    let pid_file = config::managed_pid_file(port)?;
    if let Ok(raw) = std::fs::read_to_string(&pid_file)
        && let Ok(pid) = raw.trim().parse::<u32>()
        && pid_is_live(pid)
    {
        pids.push(pid);
    }

    let output = std::process::Command::new("lsof")
        .args(["-nP", &format!("-tiTCP:{port}"), "-sTCP:LISTEN"])
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

async fn clear_saved_endpoint_for_port(port: u16) -> Result<bool> {
    let mut loaded = config::load().await?;
    if loaded
        .endpoint
        .as_deref()
        .is_some_and(|endpoint| endpoint_uses_local_port(endpoint, port))
    {
        // Keep the identity: stopping managed Chrome must not enable discovery
        // of another browser on the next command.
        if matches!(loaded.connection, Some(Connection::Managed { .. })) {
            loaded.endpoint = None;
            loaded.target_id = None;
            config::save(&loaded).await?;
        }
        return Ok(true);
    }
    Ok(false)
}

fn endpoint_uses_local_port(endpoint: &str, port: u16) -> bool {
    let Ok(url) = url::Url::parse(endpoint) else {
        return false;
    };
    matches!(url.host_str(), Some("127.0.0.1" | "localhost" | "::1"))
        && url.port_or_known_default() == Some(port)
}

fn pid_is_live(pid: u32) -> bool {
    std::process::Command::new("ps")
        .args(["-p", &pid.to_string()])
        .output()
        .is_ok_and(|output| output.status.success())
}

fn force_kill_pid(pid: u32) -> Result<()> {
    if pid == std::process::id() {
        return Ok(());
    }
    let status = std::process::Command::new("kill")
        .args(["-KILL", &pid.to_string()])
        .status()
        .map_err(|source| Error::Io {
            path: "kill".to_owned(),
            source,
        })?;
    if !status.success() && pid_is_live(pid) {
        return Err(Error::InvalidArgument(format!(
            "could not stop managed browser process {pid}"
        )));
    }
    Ok(())
}

async fn wait_for_browser_exit(pids: &[u32], timeout: Duration) -> bool {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        if pids.iter().all(|pid| !pid_is_live(*pid)) {
            return true;
        }
        if tokio::time::Instant::now() >= deadline {
            return false;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn chrome_launch_args(args: &LaunchArgs, profile: &Path) -> Vec<String> {
    let mut chrome_args = vec![
        "--remote-debugging-address=127.0.0.1".to_owned(),
        format!("--remote-debugging-port={}", args.port.unwrap_or(9322)),
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
    let (endpoint, mut client) = selected_client(cli).await?;
    let mut saved = config::load().await?;
    let stored = if cli.cdp.is_none()
        && saved.endpoint.as_deref() == Some(endpoint.websocket_url.as_str())
    {
        saved.target_id.clone()
    } else {
        None
    };
    if let Some(target_id) = cli.target.as_deref() {
        client.attach_or_create(Some(target_id)).await?;
    } else if let Some(target_id) = stored.as_deref() {
        match client.attach_or_create(Some(target_id)).await {
            Ok(_) => {}
            Err(Error::TargetNotFound(_)) => {
                let target = client.create_target("about:blank").await?;
                client.attach(&target.target_id).await?;
                saved.endpoint = Some(endpoint.websocket_url);
                saved.target_id = Some(target.target_id);
                config::save(&saved).await?;
            }
            Err(error) => return Err(error),
        }
    } else {
        // Work in a background tab, never navigate the user's arbitrary first tab.
        let target = client.create_target("about:blank").await?;
        client.attach(&target.target_id).await?;
        if cli.cdp.is_none() {
            saved.endpoint = Some(endpoint.websocket_url);
            saved.target_id = Some(target.target_id);
            config::save(&saved).await?;
        }
    }
    Ok(client)
}

async fn selected_client(cli: &Cli) -> Result<(BrowserEndpoint, CdpClient)> {
    if !matches!(cli.browser, BrowserKind::Auto | BrowserKind::Chromium) {
        return Err(Error::Unsupported {
            backend: "safari".to_owned(),
            feature: "persistent browser connection".to_owned(),
        });
    }
    if let Some(value) = &cli.cdp {
        let endpoint = resolve_endpoint(cli, value).await?;
        let client = verify_connection(&endpoint, cli.timeout).await?;
        return Ok((endpoint, client));
    }
    let saved = config::load().await?;
    let endpoint = selected_endpoint(cli, &saved).await?;
    let client = verify_connection(&endpoint, cli.timeout)
        .await
        .map_err(|_| disconnected(&saved))?;
    Ok((endpoint, client))
}

fn default_launch_args() -> LaunchArgs {
    LaunchArgs {
        port: None,
        profile: None,
        browser_path: None,
        headless: false,
        url: "about:blank".to_owned(),
        no_persist: false,
    }
}

fn existing_disconnected() -> Error {
    Error::BrowserDisconnected("open the selected Chrome (144+), enable chrome://inspect/#remote-debugging, then retry and Allow Chrome's connection prompt; use --timeout 60000 for more approval time".to_owned())
}

fn disconnected(saved: &Config) -> Error {
    match &saved.connection {
        Some(Connection::Existing { .. }) => existing_disconnected(),
        Some(Connection::Managed { profile, port, .. }) => Error::BrowserDisconnected(format!("restart the selected profile with lsearch launch --profile {:?} --port {port}", profile.display().to_string())),
        _ => Error::BrowserDisconnected("restart your selected browser and reconnect with lsearch connect <endpoint> if its endpoint changed".to_owned()),
    }
}

async fn resolve_endpoint(cli: &Cli, value: &str) -> Result<BrowserEndpoint> {
    tokio::time::timeout(
        Duration::from_millis(cli.timeout),
        discovery::discover(cli.browser, Some(value)),
    )
    .await
    .map_err(|_| Error::Timeout {
        operation: "browser endpoint discovery".to_owned(),
        timeout_ms: cli.timeout,
    })?
}

async fn selected_endpoint(cli: &Cli, saved: &Config) -> Result<BrowserEndpoint> {
    match &saved.connection {
        Some(Connection::Existing { profile }) => {
            discovery::endpoint_from_devtools_file(&profile.join("DevToolsActivePort"))
                .await
                .map_err(|_| disconnected(saved))
        }
        Some(Connection::Managed { .. }) => {
            let value = saved
                .endpoint
                .as_deref()
                .ok_or_else(|| disconnected(saved))?;
            // Pin the browser UUID, not just its port: another listener must
            // never inherit the selected browser's authority.
            resolve_endpoint(cli, value)
                .await
                .map_err(|_| disconnected(saved))
        }
        // Old endpoint-only configurations remain explicit selections; never scan
        // other profiles/ports if that endpoint stops responding.
        Some(Connection::Endpoint) | None => {
            let endpoint = saved.endpoint.as_deref().ok_or_else(|| {
                if saved.connection.is_some() {
                    disconnected(saved)
                } else {
                    Error::BrowserNotConfigured
                }
            })?;
            resolve_endpoint(cli, endpoint)
                .await
                .map_err(|_| disconnected(saved))
        }
    }
}

async fn verify_connection(endpoint: &BrowserEndpoint, timeout: u64) -> Result<CdpClient> {
    let mut client = CdpClient::connect(&endpoint.websocket_url, timeout).await?;
    client.verify().await?;
    Ok(client)
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
    // Even a cache hit must honor the chosen, currently connected browser.
    let mut client = cdp_client(cli).await?;
    if let Some(mut value) = load_search_cache(args, &client.cache_scope).await {
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
    let result_selector = match args.engine {
        SearchEngine::Google => "a h3",
        SearchEngine::Bing => "li.b_algo h2 a",
        SearchEngine::Brave => ".snippet[data-type=\"web\"] a.l1",
        SearchEngine::Duckduckgo => ".result__a",
    };
    let requested_results = args.limit.clamp(1, 3);
    client
        .wait_for_js(&scripts::search_ready(
            &args.query,
            result_selector,
            requested_results,
        ))
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
    if search.get("results")?.as_array()?.len() < args.limit {
        return None;
    }
    Some(search)
}

async fn save_search_cache(args: &SearchArgs, search: &Value, scope: &str) {
    if args.no_cache || args.cache_ttl == 0 {
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
        let _ = tokio::fs::write(path, bytes).await;
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
    let (endpoint, mut client) = selected_client(cli).await?;
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
            if cli.cdp.is_some() {
                return Err(Error::InvalidArgument("--cdp is a one-command override; use connect <endpoint> before saving a tab, or pass --target for this command".to_owned()));
            }
            if !client
                .targets()
                .await?
                .iter()
                .any(|target| &target.target_id == target_id)
            {
                return Err(Error::TargetNotFound(target_id.clone()));
            }
            let mut saved = config::load().await?;
            saved.endpoint = Some(endpoint.websocket_url);
            saved.target_id = Some(target_id.clone());
            config::save(&saved).await?;
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
    #[cfg(unix)]
    #[tokio::test]
    async fn shutdown_wait_never_escalates_to_a_forced_kill() {
        let mut child = std::process::Command::new("/bin/sleep")
            .arg("30")
            .spawn()
            .unwrap();
        let exited =
            super::wait_for_browser_exit(&[child.id()], std::time::Duration::from_millis(150))
                .await;
        let still_running = child.try_wait().unwrap().is_none();
        child.kill().unwrap();
        child.wait().unwrap();
        assert!(!exited);
        assert!(
            still_running,
            "only explicit --force may kill an unresponsive browser"
        );
    }

    use serde_json::json;

    use crate::cli::{SearchEngine, SearchFormat};

    use super::{
        endpoint_uses_local_port, resolve_search_format, search_engine_label, search_engine_url,
        truncate_search_snippets, validate_content_page,
    };

    #[test]
    fn launch_remembers_managed_settings_but_explicit_flags_win() {
        let saved = crate::config::Config {
            connection: Some(crate::config::Connection::Managed {
                profile: "/fixture/profile".into(),
                port: 9444,
                browser_path: Some("/fixture/chrome".into()),
            }),
            ..Default::default()
        };
        let mut args = super::default_launch_args();
        let remembered = super::remembered_launch_args(&args, &saved);
        assert_eq!(remembered.port, Some(9444));
        assert_eq!(
            remembered.profile.unwrap(),
            std::path::PathBuf::from("/fixture/profile")
        );
        assert_eq!(
            remembered.browser_path.unwrap(),
            std::path::PathBuf::from("/fixture/chrome")
        );
        args.port = Some(9322);
        args.profile = Some("/fixture/other".into());
        let overridden = super::remembered_launch_args(&args, &saved);
        assert_eq!(overridden.port, Some(9322));
        assert_eq!(
            overridden.profile.unwrap(),
            std::path::PathBuf::from("/fixture/other")
        );
    }

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
    fn saved_endpoint_cleanup_only_matches_the_requested_local_port() {
        assert!(endpoint_uses_local_port(
            "ws://127.0.0.1:9322/devtools/browser/id",
            9322
        ));
        assert!(endpoint_uses_local_port(
            "http://localhost:9444/json/version",
            9444
        ));
        assert!(!endpoint_uses_local_port(
            "ws://127.0.0.1:9322/devtools/browser/id",
            9444
        ));
        assert!(!endpoint_uses_local_port(
            "wss://remote.example.com:9322/devtools/browser/id",
            9322
        ));
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
