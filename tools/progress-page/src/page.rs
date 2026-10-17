//! The page itself (EN-087): one static HTML file, its style inline, no script, no font nor image
//! from elsewhere, as an Artifact reaches no local file (`REF-CC-ARTIFACTS`). It names stories,
//! agents' types and commits, never a person, a path nor a session id.

use std::fmt::Write;

use hooks::time::short_duration;

use run_totals::report::grouped;
use run_totals::tally::Tokens;

use crate::agents::{Agent, is_running};
use crate::progress::{FeatureProgress, StoryProgress};

/// The run as its marker and its totals give it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RunView {
    /// Its start, epoch milliseconds.
    pub start: i64,
    /// Its end, 0 while it is open.
    pub end: i64,
    /// Its tokens, every transcript counted; `None` before `run-totals` counted them.
    pub tokens: Option<Tokens>,
}

/// The plan's usage, in percent, from `usage.json`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usage {
    /// Of the five-hour window.
    pub five_hour: i64,
    /// Of the week.
    pub seven_day: i64,
}

/// What the page shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    /// When it is written, epoch milliseconds.
    pub now: i64,
    /// The run, `None` without a marker.
    pub run: Option<RunView>,
    /// The plan's usage, when known.
    pub usage: Option<Usage>,
    /// The run's features.
    pub features: Vec<FeatureProgress>,
    /// The run's subagents.
    pub agents: Vec<Agent>,
}

/// Text made safe for HTML.
#[must_use]
pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// The date of a day count from 1970-01-01, proleptic Gregorian (H. Hinnant, `civil_from_days`,
/// the inverse of `hooks::time::days_from_civil`).
#[must_use]
pub fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };

    (year_of_era + era * 400 + i64::from(month <= 2), month, day)
}

/// An instant written `2026-10-17 19:53 UTC`.
#[must_use]
pub fn utc_text(epoch_ms: i64) -> String {
    let minutes = epoch_ms.div_euclid(60_000);
    let (year, month, day) = civil_from_days(minutes.div_euclid(1440));
    let minute_of_day = minutes.rem_euclid(1440);

    format!(
        "{year}-{month:02}-{day:02} {:02}:{:02} UTC",
        minute_of_day / 60,
        minute_of_day % 60
    )
}

/// A duration, `—` when there is none.
fn duration_text(milliseconds: i64) -> String {
    if milliseconds > 0 {
        short_duration(milliseconds)
    } else {
        "—".to_owned()
    }
}

/// A token count, `—` when there is none.
fn tokens_text(tokens: &Tokens) -> String {
    if tokens.total() > 0 {
        grouped(tokens.total())
    } else {
        "—".to_owned()
    }
}

const STYLE: &str = "body{font:14px/1.45 system-ui,sans-serif;margin:1.5rem;color:#1d1d1f;background:#fff}\
h1{font-size:1.4rem;margin:0}h2{font-size:1.15rem;margin:1.6rem 0 .4rem}h3{font-size:1rem;margin:1.2rem 0 .3rem}\
table{border-collapse:collapse;width:100%}th,td{text-align:left;padding:.25rem .5rem;border-bottom:1px solid #ddd;vertical-align:top}\
th{font-weight:600;background:#f5f5f7}td.n{text-align:right;white-space:nowrap}.muted{color:#6e6e73}\
.done{color:#1a7f37}.dropped{color:#8c8c8c;text-decoration:line-through}.now{background:#eef4ff}.now td:first-child{font-weight:600}\
code{font-size:.9em}details ul{margin:.2rem 0;padding-left:1.1rem}summary{cursor:pointer}";

/// The run's section.
fn run_section(page: &Page, html: &mut String) {
    html.push_str("<h2>Run</h2>");

    let Some(run) = &page.run else {
        html.push_str(
            "<p class=\"muted\">No run recorded: <code>npm run run:start</code> opens one.</p>",
        );
        return;
    };
    let duration = short_duration((if run.end == 0 { page.now } else { run.end }) - run.start);
    let state = if run.end == 0 { "open" } else { "ended" };
    let tokens = run.tokens.unwrap_or_default();

    let _ = write!(
        html,
        "<table><tr><th>Started</th><th>Duration</th><th>Tokens</th><th>Input</th><th>Cache written</th><th>Cache read</th><th>Output</th></tr>\
<tr><td>{}</td><td>{duration}, {state}</td><td class=\"n\">{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td></tr></table>",
        utc_text(run.start),
        tokens_text(&tokens),
        grouped(tokens.input),
        grouped(tokens.cache_write),
        grouped(tokens.cache_read),
        grouped(tokens.output),
    );

    if let Some(usage) = page.usage {
        let _ = write!(
            html,
            "<p class=\"muted\">Plan used: {} % of the five hours, {} % of the week.</p>",
            usage.five_hour, usage.seven_day
        );
    }
}

/// The agents still working.
fn agents_section(page: &Page, html: &mut String) {
    let running: Vec<&Agent> = page
        .agents
        .iter()
        .filter(|agent| is_running(agent, page.now))
        .collect();

    html.push_str("<h2>Agents running</h2>");

    if running.is_empty() {
        html.push_str("<p class=\"muted\">None.</p>");
        return;
    }

    html.push_str(
        "<table><tr><th>Story</th><th>Type</th><th>Task</th><th>For</th><th>Tokens</th></tr>",
    );

    for agent in running {
        let _ = write!(
            html,
            "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td></tr>",
            escape(agent.story.as_deref().unwrap_or("—")),
            escape(&agent.kind),
            escape(&agent.description),
            duration_text(page.now - agent.start),
            tokens_text(&agent.tokens),
        );
    }

    html.push_str("</table>");
}

/// A story's row of its feature's table.
fn story_row(story: &StoryProgress, html: &mut String) {
    let status = if story.in_progress {
        "in progress".to_owned()
    } else {
        story.row.status.clone()
    };
    let class = if story.in_progress {
        " class=\"now\""
    } else {
        ""
    };
    let commits = if story.commits.is_empty() {
        "—".to_owned()
    } else {
        let mut items = String::new();

        for commit in &story.commits {
            let _ = write!(
                items,
                "<li><code>{}</code> {}</li>",
                escape(commit.hash.get(..8).unwrap_or(&commit.hash)),
                escape(&commit.subject)
            );
        }

        format!(
            "<details><summary>{}</summary><ul>{items}</ul></details>",
            story.commits.len()
        )
    };

    let _ = write!(
        html,
        "<tr{class}><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td class=\"n\">{}</td><td class=\"n\">{}</td><td>{commits}</td></tr>",
        escape(&story.row.id),
        escape(&story.row.kind),
        escape(&story.row.title),
        escape(&story.row.status),
        escape(&status),
        duration_text(story.duration_ms()),
        tokens_text(&story.tokens),
    );
}

/// The tree: each epic, its features, their stories.
fn tree_section(page: &Page, html: &mut String) {
    let mut epic = String::new();

    if page.features.is_empty() {
        html.push_str(
            "<h2>Features</h2><p class=\"muted\">No feature branch on <code>origin</code>.</p>",
        );
    }

    for feature in &page.features {
        let input = &feature.feature;

        if input.epic != epic {
            epic.clone_from(&input.epic);
            let _ = write!(
                html,
                "<h2>{} · {}</h2>",
                escape(&input.epic),
                escape(&input.epic_title)
            );
        }

        let done = feature
            .stories
            .iter()
            .filter(|story| matches!(story.row.status.as_str(), "done" | "dropped"))
            .count();

        let _ = write!(
            html,
            "<h3>{} · {}</h3><p class=\"muted\">{done} of {} stories closed · {} · {} tokens · {} commits</p>\
<table><tr><th>ID</th><th>Type</th><th>Title</th><th>Status</th><th>Duration</th><th>Tokens</th><th>Commits</th></tr>",
            escape(&input.id),
            escape(&input.title),
            feature.stories.len(),
            duration_text(feature.duration_ms()),
            tokens_text(&feature.tokens()),
            feature.commit_count(),
        );

        for story in &feature.stories {
            story_row(story, html);
        }

        html.push_str("</table>");
    }
}

/// The page's HTML.
#[must_use]
pub fn render(page: &Page) -> String {
    let mut html = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
<title>Run progress</title><style>{STYLE}</style></head><body><h1>Run progress</h1><p class=\"muted\">Updated {}</p>",
        utc_text(page.now)
    );

    run_section(page, &mut html);
    agents_section(page, &mut html);
    tree_section(page, &mut html);

    let stories: i64 = page
        .features
        .iter()
        .map(|feature| feature.tokens().total())
        .sum();

    if let Some(total) = page.run.as_ref().and_then(|run| run.tokens) {
        let _ = write!(
            html,
            "<p class=\"muted\">Tokens of the main thread and of agents outside the stories: {}.</p>",
            grouped(total.total() - stories)
        );
    }

    html.push_str("</body></html>\n");
    html
}
