//! Pages of the local source library (`the records' conventions, area library`): a page is read from the library
//! when it is fresh, otherwise fetched online (a conditional request when it is stored) by the
//! `curl` command, converted to Markdown, split into sections and stored. What the agent reads is
//! a table of contents, then only the sections it needs.

use std::process::Command;

use calendar::now_ms;
use rusqlite::Connection;

use crate::chunks::{Chunk, PageAge, is_stale, page_chunks};
use crate::html::{PageFacts, html_to_markdown, page_facts};
use crate::store::{StoredPage, find_page, page_sections, save_page, touch_page};
use crate::text;
use crate::time::{day_of, epoch_of};

/// Where a page came from: the library, the site just now, or `failed` when the site refused it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Origin {
    Failed,
    Library,
    Online,
}

impl Origin {
    /// The word the uses record and the table of contents shows.
    #[must_use]
    pub fn word(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Library => "library",
            Self::Online => "online",
        }
    }
}

/// A page as the library gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LibraryPage {
    pub origin: Origin,
    pub page: StoredPage,
    pub sections: Vec<Chunk>,
}

/// An HTTP response as the library reads it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub content_type: String,
    pub etag: String,
    pub last_modified: String,
    pub body: String,
}

/// HTTP status of a conditional request answered "not modified".
const NOT_MODIFIED: u16 = 304;

/// Agent string sent with each request.
const USER_AGENT: &str = "synoptic-studio-library/1 (documentation cache of a coding agent)";

/// The line `curl` writes before the response's facts, on its error output.
const FACTS_MARK: &str = "library-curl-facts";

/// Redirects followed, as many as `fetch` follows.
const MOST_REDIRECTS: &str = "20";

/// The title of a Markdown body: its first line `# …`, or empty.
fn markdown_title(body: &str) -> &str {
    body.split(['\n', '\r', '\u{2028}', '\u{2029}'])
        .find_map(|line| line.strip_prefix("# ").filter(|title| !title.is_empty()))
        .unwrap_or_default()
}

/// Title and last change of a body: from its first `#` heading for Markdown, from its meta tags
/// for HTML.
fn body_facts(body: &str, is_markdown: bool) -> Result<PageFacts, String> {
    if is_markdown {
        Ok(PageFacts {
            modified: String::new(),
            title: markdown_title(body).to_owned(),
        })
    } else {
        page_facts(body)
    }
}

/// A page read online, as stored in the library: Markdown and plain text kept as served, HTML
/// converted; an untitled page named by its address.
///
/// # Errors
///
/// When the page states a date Temporal could not read, or an entity beyond Unicode.
pub fn page_from_response(url: &str, response: &Response, now: i64) -> Result<StoredPage, String> {
    let is_markdown =
        response.content_type.contains("markdown") || response.content_type.contains("text/plain");
    let facts = body_facts(&response.body, is_markdown)?;
    let markdown = if is_markdown {
        text::trim(&response.body).to_owned()
    } else {
        html_to_markdown(&response.body)?
    };

    Ok(StoredPage {
        etag: response.etag.clone(),
        fetched_at: now,
        last_modified: response.last_modified.clone(),
        markdown,
        modified_at: epoch_of(&facts.modified)?,
        title: if facts.title.is_empty() {
            url.to_owned()
        } else {
            facts.title
        },
        url: url.to_owned(),
    })
}

/// Headers of a request for a page: a conditional request when the page is stored with an entity
/// tag or a date.
#[must_use]
pub fn request_headers(stored: &StoredPage) -> Vec<(&'static str, String)> {
    [
        ("user-agent", USER_AGENT.to_owned()),
        ("if-none-match", stored.etag.clone()),
        ("if-modified-since", stored.last_modified.clone()),
    ]
    .into_iter()
    .filter(|(_, value)| !value.is_empty())
    .collect()
}

/// The arguments of `curl` for a request: no `~/.curlrc` read, which only a first `--disable`
/// keeps out (`REF-CURL`, AUD-023), HTTP and HTTPS only, redirects followed, a minute at most, the
/// body on the standard output and the response's facts after a mark on the error output.
#[must_use]
pub fn curl_arguments(url: &str, headers: &[(&'static str, String)]) -> Vec<String> {
    let mut arguments: Vec<String> = [
        "--disable",
        "--silent",
        "--show-error",
        "--location",
        "--max-redirs",
        MOST_REDIRECTS,
        "--compressed",
        "--proto",
        "=http,https",
        "--proto-redir",
        "=http,https",
        "--max-time",
        "60",
        "--write-out",
    ]
    .iter()
    .map(|&argument| argument.to_owned())
    .collect();

    arguments.push(format!(
        "%{{stderr}}\n{FACTS_MARK}\n%{{http_code}}\n%header{{content-type}}\n%header{{etag}}\n%header{{last-modified}}\n"
    ));

    for (name, value) in headers {
        arguments.push("--header".to_owned());
        arguments.push(format!("{name}: {value}"));
    }

    arguments.push("--url".to_owned());
    arguments.push(url.to_owned());

    arguments
}

/// The response `curl` gave: its exit, its body and its error output, read; an error with
/// `curl`'s own message when no response came back (a name, a certificate, a refused connection).
///
/// # Errors
///
/// When `curl` failed, or wrote no facts.
pub fn curl_response(success: bool, body: &[u8], errors: &str) -> Result<Response, String> {
    let (message, facts) = errors
        .rsplit_once(&format!("\n{FACTS_MARK}\n"))
        .unwrap_or((errors, ""));
    let message = text::trim(message);

    if !success {
        return Err(format!(
            "Error: {}",
            if message.is_empty() {
                "curl failed"
            } else {
                message
            }
        ));
    }

    let mut facts = facts.split('\n');
    let status = facts
        .next()
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| "Error: curl gave no HTTP status".to_owned())?;
    let mut fact = || facts.next().unwrap_or_default().to_owned();
    let body = body.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(body);

    Ok(Response {
        status,
        content_type: fact(),
        etag: fact(),
        last_modified: fact(),
        body: String::from_utf8_lossy(body).into_owned(),
    })
}

/// A page fetched online by `curl`.
fn fetch(url: &str, stored: &StoredPage) -> Result<Response, String> {
    let output = Command::new("curl")
        .args(curl_arguments(url, &request_headers(stored)))
        .output()
        .map_err(|error| format!("Error: curl could not start: {error}"))?;

    curl_response(
        output.status.success(),
        &output.stdout,
        &String::from_utf8_lossy(&output.stderr),
    )
}

/// What the library gives when the site refuses a page: the stored page and its sections, or an
/// empty page titled with the status.
fn failed_page(
    database: &Connection,
    stored: StoredPage,
    status: u16,
) -> Result<LibraryPage, String> {
    if stored.fetched_at == 0 {
        Ok(LibraryPage {
            origin: Origin::Failed,
            page: StoredPage {
                title: format!("HTTP {status}"),
                ..stored
            },
            sections: Vec::new(),
        })
    } else {
        let sections = page_sections(database, &stored.url).map_err(|error| error_text(&error))?;

        Ok(LibraryPage {
            origin: Origin::Failed,
            page: stored,
            sections,
        })
    }
}

/// An error of SQLite as JavaScript wrote it.
#[must_use]
pub fn error_text(error: &rusqlite::Error) -> String {
    format!("Error: {error}")
}

/// A page read online and stored; the stored page kept when the site says it did not change or
/// refuses it.
fn fetched_page(
    database: &Connection,
    url: &str,
    stored: StoredPage,
    fetch: &dyn Fn(&str, &StoredPage) -> Result<Response, String>,
) -> Result<LibraryPage, String> {
    let now = now_ms();
    let response = fetch(url, &stored)?;

    if response.status == NOT_MODIFIED {
        touch_page(database, url, now).map_err(|error| error_text(&error))?;

        return Ok(LibraryPage {
            origin: Origin::Online,
            sections: page_sections(database, url).map_err(|error| error_text(&error))?,
            page: StoredPage {
                fetched_at: now,
                ..stored
            },
        });
    }

    if !(200..300).contains(&response.status) {
        return failed_page(
            database,
            StoredPage {
                url: url.to_owned(),
                ..stored
            },
            response.status,
        );
    }

    let page = page_from_response(url, &response, now)?;
    let sections = page_chunks(&page.markdown);

    save_page(database, &page, &sections).map_err(|error| error_text(&error))?;

    Ok(LibraryPage {
        origin: Origin::Online,
        page,
        sections,
    })
}

/// A page from the library, fetched first by a given function when missing, stale or asked fresh.
///
/// # Errors
///
/// When the fetch gives no response, the page cannot be read, or SQLite fails.
pub fn library_page_with(
    database: &Connection,
    url: &str,
    is_refresh: bool,
    fetch: &dyn Fn(&str, &StoredPage) -> Result<Response, String>,
) -> Result<LibraryPage, String> {
    let stored = find_page(database, url).map_err(|error| error_text(&error))?;
    let now = now_ms();

    match stored {
        Some(page)
            if !is_refresh
                && !is_stale(PageAge {
                    fetched_at: page.fetched_at,
                    modified_at: page.modified_at,
                    now,
                }) =>
        {
            Ok(LibraryPage {
                origin: Origin::Library,
                sections: page_sections(database, url).map_err(|error| error_text(&error))?,
                page,
            })
        }
        stored => fetched_page(database, url, stored.unwrap_or_default(), fetch),
    }
}

/// A page from the library, fetched online by `curl` first when missing, stale or asked fresh.
///
/// # Errors
///
/// When `curl` gives no response, the page cannot be read, or SQLite fails.
pub fn library_page(
    database: &Connection,
    url: &str,
    is_refresh: bool,
) -> Result<LibraryPage, String> {
    library_page_with(database, url, is_refresh, &fetch)
}

/// Table of contents of a page, for the agent to choose the sections to read: its title, address,
/// dates and one line per section, its number, heading and length.
#[must_use]
pub fn table_of_contents(entry: &LibraryPage) -> String {
    let page = &entry.page;
    let mut lines = vec![
        format!("# {}", page.title),
        format!(
            "{} — read {} ({}), changed {}",
            page.url,
            day_of(page.fetched_at),
            entry.origin.word(),
            day_of(page.modified_at)
        ),
        String::new(),
    ];

    lines.extend(entry.sections.iter().map(|section| {
        format!(
            "{}. {} — {} characters",
            section.index,
            if section.heading.is_empty() {
                "(top)"
            } else {
                &section.heading
            },
            text::length(&section.text)
        )
    }));

    lines.join("\n")
}
