use serde_json::Value;

pub fn string(value: &str) -> String {
    serde_json::to_string(value).expect("string serialization cannot fail")
}

pub fn snapshot(all: bool, limit: usize) -> String {
    format!(
        r#"(() => {{
const all = {all};
const limit = {limit};
const visible = (el) => {{
  const style = getComputedStyle(el);
  const rect = el.getBoundingClientRect();
  return style && style.visibility !== 'hidden' && style.display !== 'none' && rect.width > 0 && rect.height > 0;
}};
const roleOf = (el) => el.getAttribute('role') || (el.tagName || '').toLowerCase();
const textOf = (el) => (el.innerText || el.textContent || el.getAttribute('aria-label') || el.getAttribute('title') || '').trim().replace(/\s+/g, ' ').slice(0, 240);
const selector = all ? 'a,button,input,textarea,select,[role],h1,h2,h3,main,nav,form' : 'a,button,input,textarea,select,[role=button],[role=link],[contenteditable=true]';
let index = 1;
const elements = Array.from(document.querySelectorAll(selector)).filter(visible).slice(0, limit).map((el) => {{
  const ref = `e${{index++}}`;
  el.setAttribute('data-local-browser-ref', ref);
  const rect = el.getBoundingClientRect();
  return {{
    ref: `@${{ref}}`,
    role: roleOf(el),
    text: textOf(el),
    tag: el.tagName.toLowerCase(),
    href: el.href || null,
    value: 'value' in el ? el.value : null,
    bounds: {{ x: Math.round(rect.x), y: Math.round(rect.y), width: Math.round(rect.width), height: Math.round(rect.height) }}
  }};
}});
return {{ url: location.href, title: document.title, elements }};
}})()"#
    )
}

pub fn resolve_target(target: &str) -> String {
    if let Some(stripped) = target.strip_prefix('@') {
        format!(
            "[data-local-browser-ref=\"{}\"]",
            stripped.replace('"', "\\\"")
        )
    } else {
        target.to_owned()
    }
}

pub fn click(target: &str) -> String {
    let selector = resolve_target(target);
    format!(
        r#"(() => {{
const el = document.querySelector({});
if (!el) throw new Error('element not found');
el.scrollIntoView({{ block: 'center', inline: 'center' }});
el.click();
return {{ clicked: true, target: {} }};
}})()"#,
        string(&selector),
        string(target)
    )
}

pub fn fill(target: &str, value: &str, append: bool) -> String {
    let selector = resolve_target(target);
    format!(
        r#"(() => {{
const el = document.querySelector({});
if (!el) throw new Error('element not found');
el.focus();
if ('value' in el) {{
  el.value = {} ? el.value + {} : {};
  el.dispatchEvent(new InputEvent('input', {{ bubbles: true, inputType: 'insertText', data: {} }}));
  el.dispatchEvent(new Event('change', {{ bubbles: true }}));
}} else {{
  document.execCommand('insertText', false, {});
}}
return {{ value: 'value' in el ? el.value : el.textContent }};
}})()"#,
        string(&selector),
        append,
        string(value),
        string(value),
        string(value),
        string(value)
    )
}

pub fn hover(target: &str) -> String {
    let selector = resolve_target(target);
    format!(
        r#"(() => {{
const el = document.querySelector({});
if (!el) throw new Error('element not found');
el.dispatchEvent(new MouseEvent('mouseover', {{ bubbles: true }}));
return {{ hovered: true }};
}})()"#,
        string(&selector)
    )
}

pub fn select(target: &str, value: &str) -> String {
    let selector = resolve_target(target);
    format!(
        r#"(() => {{
const el = document.querySelector({});
if (!el) throw new Error('element not found');
el.value = {};
el.dispatchEvent(new Event('input', {{ bubbles: true }}));
el.dispatchEvent(new Event('change', {{ bubbles: true }}));
return {{ value: el.value }};
}})()"#,
        string(&selector),
        string(value)
    )
}

pub fn scroll(direction: &str, amount: i64, target: Option<&str>) -> String {
    let (dx, dy) = match direction {
        "up" => (0, -amount),
        "left" => (-amount, 0),
        "right" => (amount, 0),
        _ => (0, amount),
    };
    let selector = target.map(resolve_target);
    format!(
        r#"(() => {{
const el = {};
(el || window).scrollBy({}, {});
return {{ x: window.scrollX, y: window.scrollY }};
}})()"#,
        selector.as_deref().map_or_else(
            || "null".to_owned(),
            |s| { format!("document.querySelector({})", string(s)) }
        ),
        dx,
        dy
    )
}

pub fn readable() -> &'static str {
    r#"(() => {
const text = (node) => (node && (node.innerText || node.textContent) || '').trim().replace(/\n{3,}/g, '\n\n');
const meta = (name) => document.querySelector(`meta[name="${name}"], meta[property="${name}"]`)?.content || null;
const main = document.querySelector('article, main, [role=main]') || document.body;
const headings = Array.from(main.querySelectorAll('h1,h2,h3')).slice(0, 50).map(h => ({ level: h.tagName.toLowerCase(), text: text(h) }));
const links = Array.from(main.querySelectorAll('a[href]')).slice(0, 100).map(a => ({ text: text(a).slice(0, 120), url: a.href }));
return { url: location.href, title: document.title, description: meta('description') || meta('og:description'), text: text(main), headings, links };
})()"#
}

pub fn rendered_html() -> &'static str {
    "document.documentElement.outerHTML"
}

pub fn search_results() -> &'static str {
    r#"(() => {
const clean = (s) => (s || '').trim().replace(/\s+/g, ' ');
const abs = (href) => { try { return new URL(href, location.href).href; } catch { return null; } };
const candidates = Array.from(document.querySelectorAll('a[href]')).map((a) => {
  const url = abs(a.href);
  const title = clean(a.innerText || a.textContent);
  if (!url || !title || title.length < 3) return null;
  if (/google\..*\/search|bing\.com\/search|duckduckgo\.com\/?|javascript:|#/.test(url)) return null;
  const container = a.closest('article, li, div, section') || a.parentElement;
  const snippet = clean(container?.innerText || '').replace(title, '').slice(0, 500);
  return { title, url, snippet, domain: new URL(url).hostname.replace(/^www\./, '') };
}).filter(Boolean);
const seen = new Set();
const results = [];
for (const item of candidates) {
  if (seen.has(item.url)) continue;
  seen.add(item.url);
  results.push({ rank: results.length + 1, ...item });
}
return { url: location.href, title: document.title, results, blocked: /captcha|unusual traffic|verify you are human/i.test(document.body.innerText || '') };
})()"#
}

pub fn extract(selector: &str, fields: &[Field], limit: usize) -> String {
    let fields_json = serde_json::to_string(fields).expect("fields serialize");
    format!(
        r#"(() => {{
const selector = {};
const fields = {};
const clean = (s) => (s || '').trim().replace(/\s+/g, ' ');
const valueFor = (root, spec) => {{
  const el = spec.selector ? root.querySelector(spec.selector) : root;
  if (!el) return null;
  if (spec.source === 'text') return clean(el.innerText || el.textContent);
  if (spec.source === 'html') return el.innerHTML;
  if (spec.source === 'href') return el.href || el.getAttribute('href');
  if (spec.source === 'src') return el.src || el.getAttribute('src');
  if (spec.source.startsWith('@')) return el.getAttribute(spec.source.slice(1));
  return el.getAttribute(spec.source);
}};
return Array.from(document.querySelectorAll(selector)).slice(0, {}).map((root, i) => {{
  const out = {{ index: i }};
  for (const field of fields) out[field.name] = valueFor(root, field);
  return out;
}});
}})()"#,
        string(selector),
        fields_json,
        limit
    )
}

#[derive(Debug, serde::Serialize)]
pub struct Field {
    pub name: String,
    pub selector: Option<String>,
    pub source: String,
}

pub fn parse_field(input: &str) -> Result<Field, String> {
    let Some((name, spec)) = input.split_once('=') else {
        return Err(format!("field must be name=source: {input}"));
    };
    let (selector, source) = spec
        .split_once("=>")
        .map_or((None, spec), |(selector, source)| {
            (Some(selector.to_owned()), source)
        });
    Ok(Field {
        name: name.to_owned(),
        selector,
        source: source.to_owned(),
    })
}

pub fn browser_fetch(url: &str, method: &str, headers: &Value, body: Option<&str>) -> String {
    format!(
        r#"(async () => {{
const response = await fetch({}, {{ method: {}, headers: {}, body: {} }});
const text = await response.text();
return {{
  url: response.url,
  status: response.status,
  statusText: response.statusText,
  headers: Object.fromEntries(response.headers.entries()),
  text
}};
}})()"#,
        string(url),
        string(method),
        headers,
        body.map_or_else(|| "undefined".to_owned(), string)
    )
}
