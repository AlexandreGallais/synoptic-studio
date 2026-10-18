//! The library's three tools, `library_search`, `library_get` and `library_read`, as the MCP
//! server and the command line answer them: their descriptions and input schemas, their texts and
//! the use each records (`REF-MCP-TOOLS`). The answers are those `library-mcp.ts` gave, text for
//! text, so that a session sees no change of server.

use hooks::time::now_ms;
use rusqlite::Connection;
use rusqlite::types::Value as SqlValue;
use serde_json::{Value, json};

use crate::pages::{error_text, library_page, table_of_contents};
use crate::store::search_library;
use crate::text;
use crate::usage::{Use, record_use};

/// Most sections a search returns when the call does not say.
const DEFAULT_LIMIT: i64 = 8;

/// The tools as `tools/list` describes them: name, description, input schema.
#[must_use]
pub fn tool_list() -> Vec<(&'static str, &'static str, Value)> {
    vec![
        (
            "library_search",
            "Search the local library of documentation pages already read (full text, best sections first). Use before any web search or fetch.",
            json!({
                "properties": {
                    "limit": { "description": "most sections returned (default 8)", "type": "number" },
                    "query": { "description": "words to find", "type": "string" }
                },
                "required": ["query"],
                "type": "object"
            }),
        ),
        (
            "library_get",
            "Get a documentation page through the local library: stored and fresh, it is read from there; otherwise fetched online and stored. Returns its table of contents; read sections with library_read.",
            json!({
                "properties": {
                    "refresh": { "description": "read it online whatever its age", "type": "boolean" },
                    "url": { "description": "address of the page", "type": "string" }
                },
                "required": ["url"],
                "type": "object"
            }),
        ),
        (
            "library_read",
            "Read sections of a page stored in the local library, by their numbers in its table of contents.",
            json!({
                "properties": {
                    "sections": { "description": "section numbers", "items": { "type": "number" }, "type": "array" },
                    "url": { "description": "address of the page", "type": "string" }
                },
                "required": ["url", "sections"],
                "type": "object"
            }),
        ),
    ]
}

/// A string argument, empty when missing or of another type.
fn text_of<'a>(input: &'a Value, key: &str) -> &'a str {
    input.get(key).and_then(Value::as_str).unwrap_or_default()
}

/// A number as JavaScript's `Number()` reads a value: a string of a decimal or hexadecimal
/// number, `true` 1, `false` and `null` 0, empty or white space 0; anything else `NaN`.
#[must_use]
pub fn js_number(value: &Value) -> f64 {
    match value {
        Value::Number(number) => number.as_f64().unwrap_or(f64::NAN),
        Value::Bool(flag) => f64::from(u8::from(*flag)),
        Value::Null => 0.0,
        Value::String(written) => string_number(text::trim(written)),
        Value::Array(_) | Value::Object(_) => f64::NAN,
    }
}

/// A trimmed string read as `Number()` reads it.
fn string_number(written: &str) -> f64 {
    if written.is_empty() {
        return 0.0;
    }

    let radix = [
        ("0x", 16),
        ("0X", 16),
        ("0o", 8),
        ("0O", 8),
        ("0b", 2),
        ("0B", 2),
    ]
    .iter()
    .find_map(|(prefix, radix)| written.strip_prefix(prefix).map(|digits| (digits, *radix)));

    if let Some((digits, radix)) = radix {
        // Through a decimal string, as JavaScript reads a long one: rounded, never cut.
        return u128::from_str_radix(digits, radix).map_or(f64::NAN, |value| {
            value.to_string().parse().unwrap_or(f64::NAN)
        });
    }

    let unsigned = written.trim_start_matches(['+', '-']);
    let is_decimal = unsigned
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | 'E' | '+' | '-'));

    match unsigned {
        "Infinity" if written.starts_with('-') => f64::NEG_INFINITY,
        "Infinity" => f64::INFINITY,
        _ if is_decimal && written.len() - unsigned.len() <= 1 => {
            written.parse().unwrap_or(f64::NAN)
        }
        _ => f64::NAN,
    }
}

/// The most sections a search returns: the call's number, bound as SQLite would bind it from
/// JavaScript, else 8.
fn limit_of(input: &Value) -> SqlValue {
    match input.get("limit") {
        Some(Value::Number(number)) => number.as_i64().map_or_else(
            || SqlValue::Real(number.as_f64().unwrap_or(f64::NAN)),
            SqlValue::Integer,
        ),
        _ => SqlValue::Integer(DEFAULT_LIMIT),
    }
}

/// Answer of `library_search`: one line per section found, with its address, number, heading
/// and snippet.
fn search_answer(database: &Connection, project: &str, input: &Value) -> Result<String, String> {
    let query = text_of(input, "query");
    let hits =
        search_library(database, query, &limit_of(input)).map_err(|error| error_text(&error))?;
    let use_ = Use {
        origin: if hits.is_empty() { "none" } else { "found" },
        project,
        query,
        tool: "search",
        url: "",
    };

    record_use(database, &use_, now_ms()).map_err(|error| error_text(&error))?;

    if hits.is_empty() {
        return Ok("No section found: get the page with library_get.".to_owned());
    }

    Ok(hits
        .iter()
        .map(|hit| {
            format!(
                "{} §{} — {} › {}\n  {}",
                hit.url, hit.index, hit.title, hit.heading, hit.snippet
            )
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

/// Answer of `library_read`: the text of the sections asked, in page order.
fn read_answer(database: &Connection, project: &str, input: &Value) -> Result<String, String> {
    let wanted: Vec<f64> = input
        .get("sections")
        .and_then(Value::as_array)
        .map(|asked| asked.iter().map(js_number).collect())
        .unwrap_or_default();
    let entry = library_page(database, text_of(input, "url"), false)?;
    let use_ = Use {
        origin: entry.origin.word(),
        project,
        query: "",
        tool: "read",
        url: &entry.page.url,
    };

    record_use(database, &use_, now_ms()).map_err(|error| error_text(&error))?;

    #[expect(
        clippy::cast_precision_loss,
        reason = "a section number is far under 2^53"
    )]
    let is_wanted = |index: i64| wanted.contains(&(index as f64));

    Ok(entry
        .sections
        .iter()
        .filter(|section| is_wanted(section.index))
        .map(|section| section.text.as_str())
        .collect::<Vec<_>>()
        .join("\n\n"))
}

/// Answer of `library_get`: the page's table of contents.
fn get_answer(database: &Connection, project: &str, input: &Value) -> Result<String, String> {
    let is_refresh = input.get("refresh") == Some(&Value::Bool(true));
    let entry = library_page(database, text_of(input, "url"), is_refresh)?;
    let use_ = Use {
        origin: entry.origin.word(),
        project,
        query: "",
        tool: "get",
        url: &entry.page.url,
    };

    record_use(database, &use_, now_ms()).map_err(|error| error_text(&error))?;

    Ok(table_of_contents(&entry))
}

/// Text answer of a tool, the use recorded with the project that asks: `library_search`,
/// `library_get`, and `library_read` for any other name, as the TypeScript server did.
///
/// # Errors
///
/// When the page cannot be fetched or read, or SQLite fails, with the message to give back.
pub fn call_tool(
    database: &Connection,
    project: &str,
    name: &str,
    input: &Value,
) -> Result<String, String> {
    match name {
        "library_search" => search_answer(database, project, input),
        "library_get" => get_answer(database, project, input),
        _ => read_answer(database, project, input),
    }
}

/// Result of a tool call for the MCP server: its text, or the error it met with `true`. A
/// `library_get` whose fetch failed before any answer (a name, a certificate, a time-out) is
/// recorded as `failed`, so that the page hook lets the page be read otherwise (AUD-022).
#[must_use]
pub fn tool_result(
    database: &Connection,
    project: &str,
    name: &str,
    input: &Value,
) -> (String, bool) {
    match call_tool(database, project, name, input) {
        Ok(answer) => (answer, false),
        Err(error) => {
            if name == "library_get" {
                let use_ = Use {
                    origin: "failed",
                    project,
                    query: "",
                    tool: "get",
                    url: text_of(input, "url"),
                };

                // The error given back is the fetch's; a record that fails too adds nothing to it.
                let _ = record_use(database, &use_, now_ms());
            }

            (error, true)
        }
    }
}
