//! Pages read through the library (`docs/tooling/sources.md`, EN-101): a `WebFetch`, or a `curl`
//! or `wget` printing a web page, is refused with the `library_get` call to make instead, unless
//! the library returned that page empty — stored with fewer than 300 characters, or refused by its
//! site — which the hook reads from the library's file.

use std::path::PathBuf;

/// Characters under which a stored page counts as empty: a page built in the browser by
/// JavaScript, its shell only (`docs/tooling/sources.md`, Limits). Of 417 pages stored on
/// 2026-10-17, 8 held fewer, from 0 to 235 characters.
pub const EMPTY_PAGE: usize = 300;

/// What the library holds of a page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Library {
    /// Never asked, or the library cannot be read.
    Missing,
    /// Stored, with the characters of its Markdown.
    Stored { characters: usize },
    /// Not stored: its site refused the library's last request.
    Failed,
}

/// Hosts of the machine: the playground, the docs site, a local server.
const LOCAL_HOSTS: [&str; 4] = ["localhost", "127.0.0.1", "[::1]", "0.0.0.0"];

/// Extensions of an address that names a page, not a file to download.
const PAGE_EXTENSIONS: [&str; 11] = [
    "html", "htm", "xhtml", "shtml", "md", "markdown", "txt", "php", "asp", "aspx", "jsp",
];

/// Options by which `curl` or `wget` asks something other than a page's text.
const NOT_A_READ: [&str; 12] = [
    "-I",
    "--head",
    "-d",
    "-F",
    "--form",
    "-T",
    "--upload-file",
    "--json",
    "--spider",
    "--post-data",
    "--post-file",
    "--method",
];

/// Words that may precede the command itself: settings, `env`, `command`, `time`.
fn is_prefix(word: &str) -> bool {
    matches!(word, "env" | "command" | "time" | "sudo" | "exec")
        || (word.contains('=') && !word.starts_with('-'))
}

/// The command's lines without the bodies of its here-documents, which are data, never run.
fn without_heredocs(command: &str) -> String {
    let mut kept = Vec::new();
    let mut end: Option<String> = None;

    for line in command.lines() {
        if let Some(word) = &end {
            if line.trim() == word {
                end = None;
            }
            continue;
        }
        kept.push(line);
        end = line.split_once("<<").map(|(_, rest)| {
            rest.trim_start_matches('-')
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches(['\'', '"'])
                .to_owned()
        });
        end = end.filter(|word| !word.is_empty());
    }
    kept.join("\n")
}

/// The words of one segment of a shell command, quotes removed.
fn words(segment: &str) -> Vec<String> {
    segment
        .split_whitespace()
        .map(|word| word.trim_matches(['\'', '"']).to_owned())
        .collect()
}

/// Whether some words ask headers, send data, or use a method other than `GET`.
fn asks_other_than_a_read(words: &[String]) -> bool {
    words.iter().enumerate().any(|(index, word)| {
        let short = word.starts_with('-') && !word.starts_with("--");

        NOT_A_READ.contains(&word.as_str())
            || word.starts_with("--data")
            || (short && word.contains('I'))
            || ((word == "-X" || word == "--request")
                && words.get(index + 1).is_some_and(|method| method != "GET"))
    })
}

/// Whether an address names a web page: a host of the web, not an interface for programs, its
/// path without extension or with a page's.
fn is_web_page(url: &str) -> bool {
    let rest = url.split_once("://").map_or("", |(_, rest)| rest);
    let (authority, path) = rest.split_at(rest.find(['/', '?', '#']).unwrap_or(rest.len()));
    let host = authority.rsplit('@').next().unwrap_or("");
    let host = if host.starts_with('[') {
        host.split_inclusive(']').next().unwrap_or(host)
    } else {
        host.split(':').next().unwrap_or(host)
    };
    let path = path.split(['?', '#']).next().unwrap_or("");
    let last = path.rsplit('/').next().unwrap_or("");
    let is_page_file = last
        .rsplit_once('.')
        .is_none_or(|(_, extension)| PAGE_EXTENSIONS.contains(&extension.to_lowercase().as_str()));

    !host.is_empty()
        && !LOCAL_HOSTS.contains(&host)
        && !host.starts_with("api.")
        && !path.contains("/api/")
        && is_page_file
}

/// The page a segment reads with `curl` or `wget`, if it prints one.
fn page_of_segment(segment: &str) -> Option<String> {
    let words = words(segment);
    let start = words.iter().position(|word| !is_prefix(word))?;
    let tool = words[start].rsplit('/').next().unwrap_or("");
    let options = &words[start + 1..];
    // `wget` saves to a file unless told to print (`-O-`, `-qO-`, `-O -`).
    let prints = tool == "curl"
        || options.iter().enumerate().any(|(index, word)| {
            word.ends_with("O-")
                || word == "--output-document=-"
                || (word == "-O" && options.get(index + 1).is_some_and(|next| next == "-"))
        });

    if !matches!(tool, "curl" | "wget") || !prints || asks_other_than_a_read(options) {
        return None;
    }

    options
        .iter()
        .find(|word| word.starts_with("http://") || word.starts_with("https://"))
        .filter(|url| is_web_page(url))
        .cloned()
}

/// The web page a shell command reads with `curl` or `wget`, if any: printed, not saved as a file,
/// a host of the web, no data sent.
#[must_use]
pub fn page_of_command(command: &str) -> Option<String> {
    without_heredocs(command)
        .split(['\n', ';', '|', '&'])
        .find_map(page_of_segment)
}

/// The query giving what the library holds of a page: its Markdown's characters (-1 when not
/// stored) and `failed` when a `get` of it was refused by its site. It reads `uses`, a table before EN-101 and a view
/// since, so that it works on either file.
#[must_use]
pub fn library_query(url: &str) -> String {
    let literal = format!("'{}'", url.replace('\'', "''"));

    format!(
        "SELECT COALESCE((SELECT length(markdown) FROM pages WHERE url = {literal}), -1), \
         COALESCE((SELECT origin FROM uses WHERE url = {literal} AND tool = 'get' AND origin = 'failed' \
         LIMIT 1), '');"
    )
}

/// What the library holds of a page, from the output of `library_query`.
#[must_use]
pub fn library_state(output: &str) -> Library {
    let line = output.lines().next().unwrap_or("");
    let (characters, origin) = line.split_once('|').unwrap_or((line, ""));

    match characters.trim().parse::<i64>() {
        Ok(count) if count >= 0 => Library::Stored {
            characters: usize::try_from(count).unwrap_or(usize::MAX),
        },
        Ok(_) if origin.trim() == "failed" => Library::Failed,
        _ => Library::Missing,
    }
}

/// Why a page read is refused, with the call to make instead; `None` when the library returned
/// the page empty, so that another way may be tried.
#[must_use]
pub fn refusal(url: &str, library: Library) -> Option<String> {
    match library {
        Library::Stored { characters } if characters < EMPTY_PAGE => None,
        Library::Failed => None,
        Library::Stored { .. } => Some(format!(
            "The library holds this page: call library_get with url \"{url}\", then library_read \
             the sections needed (docs/tooling/sources.md, EN-101)."
        )),
        Library::Missing => Some(format!(
            "Pages are read through the library: call library_get with url \"{url}\" (it fetches \
             and stores the page), then library_read the sections needed. WebFetch only once the \
             library returns the page empty (docs/tooling/sources.md, EN-101)."
        )),
    }
}

/// The library's file, as its scripts open it: `LIBRARY_PATH`, else a file of the user's data
/// folder; `None` without either.
#[must_use]
pub fn library_file(library_path: Option<String>, home: Option<String>) -> Option<PathBuf> {
    library_path.map(PathBuf::from).or_else(|| {
        home.map(|home| {
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("agent-library")
                .join("library.sqlite")
        })
    })
}
