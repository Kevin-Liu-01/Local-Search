use std::{collections::BTreeMap, path::Path, time::Duration};

use chrono::Utc;
use serde_json::{Value, json};

use crate::{
    browser::{CdpClient, discovery, scripts},
    cli::{
        BrowserKind, Cli, Command, ConnectArgs, CookiesCommand, OpenArgs, OptionalPathArgs,
        PathArgs, ReadArgs, ReadFormat, RecordArgs, RequestArgs, ScreenshotArgs, ScrollDirection,
        SearchArgs, SearchEngine, TabsCommand, WaitArgs,
    },
    config::{self, Config},
    error::{Error, IoContext, Result},
    output::print_json,
};

pub async fn run(cli: Cli) -> Result<()> {
    match &cli.command {
        Command::Doctor => print_json(&discovery::doctor().await, cli.pretty),
        Command::Connect(args) => connect(&cli, args).await,
        Command::Tabs(command) => tabs(&cli, command).await,
        command => {
            let mut client = cdp_client(&cli).await?;
            match command {
                Command::Search(args) => search(&cli, &mut client, args).await,
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
                Command::Doctor | Command::Connect(_) | Command::Tabs(_) => {
                    unreachable!("handled before cdp attach")
                }
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

async fn cdp_client(cli: &Cli) -> Result<CdpClient> {
    if !matches!(cli.browser, BrowserKind::Auto | BrowserKind::Chromium) {
        return Err(Error::Unsupported {
            backend: "safari".to_owned(),
            feature: "this command".to_owned(),
        });
    }
    let endpoint = discovery::discover(cli.browser, cli.cdp.as_deref()).await?;
    let mut client = CdpClient::connect(&endpoint.websocket_url, cli.timeout).await?;
    let stored = config::load().await.ok().and_then(|cfg| cfg.target_id);
    client
        .attach_or_create(cli.target.as_deref().or(stored.as_deref()))
        .await?;
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

async fn search(cli: &Cli, client: &mut CdpClient, args: &SearchArgs) -> Result<()> {
    let url = match args.engine {
        SearchEngine::Google => format!(
            "https://www.google.com/search?q={}",
            urlencoding(&args.query)
        ),
        SearchEngine::Bing => format!("https://www.bing.com/search?q={}", urlencoding(&args.query)),
        SearchEngine::Duckduckgo => {
            format!("https://duckduckgo.com/?q={}", urlencoding(&args.query))
        }
    };
    if args.new_tab {
        let target = client.create_target(&url).await?;
        client.attach(&target.target_id).await?;
        client.wait_ready().await?;
    } else {
        client.navigate(&url).await?;
    }
    client
        .wait_for_js("document.querySelectorAll('a[href]').length > 5 || document.body.innerText.length > 100")
        .await?;
    let mut value = client.evaluate(scripts::search_results(), true).await?;
    if let Some(results) = value.get_mut("results").and_then(Value::as_array_mut) {
        results.truncate(args.limit);
    }
    print_json(
        &json!({ "ok": true, "query": args.query, "engine": format!("{:?}", args.engine).to_lowercase(), "search": value }),
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
    let endpoint = discovery::discover(cli.browser, cli.cdp.as_deref()).await?;
    let mut client = CdpClient::connect(&endpoint.websocket_url, cli.timeout).await?;
    match command {
        TabsCommand::List => {
            let targets = client.targets().await?;
            print_json(&json!({ "ok": true, "tabs": targets }), cli.pretty)
        }
        TabsCommand::New { url } => {
            let target = client
                .create_target(url.as_deref().unwrap_or("about:blank"))
                .await?;
            config::save(&Config {
                endpoint: Some(endpoint.websocket_url),
                target_id: Some(target.target_id.clone()),
            })
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
            "creator": { "name": "local-browser", "version": env!("CARGO_PKG_VERSION") },
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
