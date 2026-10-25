//! Pages read through the library (`the records' conventions, area library`, EN-101): a `WebFetch`, or a `curl`
//! or `wget` printing a web page, is refused with the `library_get` call to make instead, unless
//! the library returned that page empty — stored with fewer than 300 characters, or refused by its
//! site — which the hook reads from the library's file.

use std::path::PathBuf;

use git_repository::records_place::records_of_folder;

/// Characters under which a stored page counts as empty: a page built in the browser by
/// JavaScript, its shell only (`the records' conventions, area library`, Limits). Of 417 pages stored on
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

/// The word ending the here-document a line opens, `None` when it opens none: a `<<` outside an
/// arithmetic `$(( ))`, where it shifts, followed by a word opening with a letter or `_`, which a
/// here-string `<<<` is not.
fn heredoc_end(line: &str) -> Option<String> {
    line.match_indices("<<").find_map(|(index, _)| {
        let before = &line[..index];
        let after = &line[index + 2..];

        if before.matches("$((").count() > before.matches("))").count() {
            return None;
        }

        let word = after
            .trim_start_matches('-')
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_matches(['\'', '"']);

        word.chars()
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
            .then(|| word.to_owned())
    })
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
        end = heredoc_end(line);
    }
    kept.join("\n")
}

/// A shell command cut where it is not quoted: into its commands (at a line break, `;`, `&`,
/// `&&`, `||`), each into the stages of its pipeline (at a single `|`).
fn pipelines(command: &str) -> Vec<Vec<String>> {
    let mut all = Vec::new();
    let mut stages = Vec::new();
    let mut stage = String::new();
    let mut quote: Option<char> = None;
    let mut characters = command.chars().peekable();

    while let Some(character) = characters.next() {
        match (quote, character) {
            (Some(open), _) if character == open => quote = None,
            (None, '\'' | '"') => quote = Some(character),
            (None, '|') if characters.peek() != Some(&'|') => {
                stages.push(std::mem::take(&mut stage));
                continue;
            }
            (None, '\n' | ';' | '&' | '|') => {
                if matches!(characters.peek(), Some(&'&' | &'|')) {
                    characters.next();
                }
                stages.push(std::mem::take(&mut stage));
                all.push(std::mem::take(&mut stages));
                continue;
            }
            _ => {}
        }
        stage.push(character);
    }
    stages.push(stage);
    all.push(stages);
    all
}

/// The words of one stage of a pipeline, quotes removed.
fn words(segment: &str) -> Vec<String> {
    segment
        .split_whitespace()
        .map(|word| word.trim_matches(['\'', '"']).to_owned())
        .collect()
}

/// The command a stage runs, its settings and `env`, `command`, `time` skipped: `curl`.
fn tool_of(words: &[String]) -> Option<(&str, &[String])> {
    let start = words.iter().position(|word| !is_prefix(word))?;
    let tool = words[start].rsplit('/').next().unwrap_or("");

    Some((tool, &words[start + 1..]))
}

/// Whether a later stage of a pipeline runs what `curl` printed, or unpacks it: an installer
/// script or an archive, not a page read.
fn runs_or_unpacks(stages: &[String]) -> bool {
    stages.iter().skip(1).any(|stage| {
        tool_of(&words(stage))
            .is_some_and(|(tool, _)| matches!(tool, "sh" | "bash" | "zsh" | "tar"))
    })
}

/// Whether `curl` writes the response to a file that is not a page: `-o /dev/null` for a link
/// check, `-o font.ttf` for a download. A page saved, `-o page.html`, is still a read.
fn saves_a_file(options: &[String]) -> bool {
    options.iter().enumerate().any(|(index, word)| {
        let is_output = word == "--output"
            || (word.starts_with('-') && !word.starts_with("--") && word.ends_with('o'));

        is_output
            && options.get(index + 1).is_some_and(|file| {
                let last = file.rsplit('/').next().unwrap_or("");

                file == "/dev/null"
                    || last.rsplit_once('.').is_some_and(|(_, extension)| {
                        !PAGE_EXTENSIONS.contains(&extension.to_lowercase().as_str())
                    })
            })
    })
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

/// Whether an address names a web page: a host of the web, not an interface for programs
/// (`api.`, `registry.`, `/api/`), its path without extension or with a page's.
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
        && !host.starts_with("registry.")
        && !path.contains("/api/")
        && is_page_file
}

/// The page a stage of a pipeline reads with `curl` or `wget`, if it prints one that no later
/// stage runs.
fn page_of_pipeline(stages: &[String]) -> Option<String> {
    (0..stages.len()).find_map(|index| page_of_stage(&stages[index..]))
}

/// The page the first of these stages reads, if it prints one that no later stage runs.
fn page_of_stage(stages: &[String]) -> Option<String> {
    let first = stages.first()?;
    let words = words(first);
    let (tool, options) = tool_of(&words)?;
    // `wget` saves to a file unless told to print (`-O-`, `-qO-`, `-O -`).
    let prints = (tool == "curl" && !saves_a_file(options))
        || options.iter().enumerate().any(|(index, word)| {
            word.ends_with("O-")
                || word == "--output-document=-"
                || (word == "-O" && options.get(index + 1).is_some_and(|next| next == "-"))
        });

    if !matches!(tool, "curl" | "wget")
        || !prints
        || asks_other_than_a_read(options)
        || runs_or_unpacks(stages)
    {
        return None;
    }

    options
        .iter()
        .find(|word| word.starts_with("http://") || word.starts_with("https://"))
        .filter(|url| is_web_page(url))
        .cloned()
}

/// The web page a shell command reads with `curl` or `wget`, if any: printed, not saved as a file
/// nor run, a host of the web, no data sent.
#[must_use]
pub fn page_of_command(command: &str) -> Option<String> {
    pipelines(&without_heredocs(command))
        .iter()
        .find_map(|stages| page_of_pipeline(stages))
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
             the sections needed (the records' conventions, area library, EN-101)."
        )),
        Library::Missing => Some(format!(
            "Pages are read through the library: call library_get with url \"{url}\" (it fetches \
             and stores the page), then library_read the sections needed. WebFetch only once the \
             library returns the page empty (the records' conventions, area library, EN-101)."
        )),
    }
}

/// Whether an agent's file gives it the library's tools: its front matter's `tools:` names
/// `library_get`, or it has no `tools:` and inherits every tool.
#[must_use]
pub fn has_library_tools(agent_file: &str) -> bool {
    agent_file
        .lines()
        .skip(1)
        .take_while(|line| line.trim() != "---")
        .find_map(|line| line.strip_prefix("tools:"))
        .is_none_or(|tools| tools.contains("library_get"))
}

/// Why a page read is refused to an agent without the library's tools: it cannot call
/// `library_get`, so it hands the page to its caller.
#[must_use]
pub fn refusal_without_library(url: &str) -> String {
    format!(
        "Pages are read through the library, and this agent has no library tools: name \"{url}\" \
         in your report, for your caller to read through the library (the records' conventions, area library, \
         EN-101)."
    )
}

/// The variable naming the library's file. Not `LIBRARY_PATH`, the C toolchain's library search
/// path, which Apple's `python3` sets to `/usr/local/lib` for the processes it starts (SP-023).
pub const FILE_VARIABLE: &str = "AGENT_LIBRARY_FILE";

/// The library's file, the one function the hooks, the library's server and its tools open it by
/// (AUD-023): the value of `AGENT_LIBRARY_FILE`, else the project's records, which hold the
/// library's tables beside the knowledge (E49), else a file of the user's data folder; `None`
/// without any. An empty value counts as none. Never `LIBRARY_PATH`, the C toolchain's search
/// path (EN-106).
#[must_use]
pub fn library_file(
    variable: Option<String>,
    records: Option<PathBuf>,
    home: Option<String>,
) -> Option<PathBuf> {
    variable
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or(records)
        .or_else(|| {
            home.filter(|home| !home.is_empty()).map(|home| {
                PathBuf::from(home)
                    .join(".local")
                    .join("share")
                    .join("agent-library")
                    .join("library.sqlite")
            })
        })
}

/// The library's file for this process: `AGENT_LIBRARY_FILE`, else the records file of the
/// repository of the working folder, in the folder `AGENT_RECORDS_ROOT` names, else the data folder
/// of the home `HOME` names, or `USERPROFILE` on Windows.
#[must_use]
pub fn library_file_of_process() -> Option<PathBuf> {
    library_file(
        std::env::var(FILE_VARIABLE).ok(),
        records_of_folder(
            std::env::var("AGENT_RECORDS_ROOT").ok().as_deref(),
            std::path::Path::new("."),
        ),
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .ok(),
    )
}
