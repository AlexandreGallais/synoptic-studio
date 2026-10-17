//! The page's address, kept in `.git/metrics/progress-page.json` with the start of the run it
//! belongs to (EN-087): publishing from a new session updates the same page only when given its
//! address (`REF-CC-ARTIFACTS`), and a new run gets a new page.

use hooks::json::{parse, quote};

use run_totals::marker::whole;

/// Whether `url` is the address of a claude.ai Artifact: `https://claude.ai/…`, no space.
#[must_use]
pub fn is_artifact_address(url: &str) -> bool {
    url.strip_prefix("https://claude.ai/")
        .is_some_and(|rest| !rest.is_empty() && !rest.contains(char::is_whitespace))
}

/// The address file's text: the run's start and the page's address.
#[must_use]
pub fn render_address(run_start: i64, url: &str) -> String {
    format!("{{\"run\":{run_start},\"url\":{}}}\n", quote(url))
}

/// The page's address for the run started at `run_start`; `None` when the file is missing, not
/// one, or kept for another run.
#[must_use]
pub fn address_for(text: &str, run_start: i64) -> Option<String> {
    let saved = parse(text)?;
    let url = saved.get("url").text();

    (whole(saved.get("run")) == run_start && is_artifact_address(url)).then(|| url.to_owned())
}
