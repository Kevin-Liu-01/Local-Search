use serde::Serialize;
use serde_json::{Value, json};

use crate::error::Error;

/// Prints a serializable command result to stdout.
///
/// # Errors
/// Returns a JSON serialization error when the result cannot be encoded.
pub fn print_json<T: Serialize>(value: &T, pretty: bool) -> crate::error::Result<()> {
    let rendered = if pretty {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    };
    println!("{rendered}");
    Ok(())
}

/// Renders a stable JSON error envelope for agents.
#[must_use]
pub fn render_error(error: &Error, pretty: bool) -> String {
    let value = json!({
        "ok": false,
        "error": {
            "code": error.code(),
            "message": error.to_string(),
        }
    });

    render_value(&value, pretty)
}

fn render_value(value: &Value, pretty: bool) -> String {
    if pretty {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    }
    .unwrap_or_else(|_| "{\"ok\":false,\"error\":{\"code\":\"serialization_error\"}}".to_owned())
}
