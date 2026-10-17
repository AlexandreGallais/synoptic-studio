//! `progress-page [address <url>]` (`npm run progress:page`, EN-087). Without argument, writes the
//! run's progress page `.git/metrics/progress.html` from the feature branches on `origin`, the
//! commits since the run's start, the run's subagents and `.git/metrics/` (`run.json`,
//! `run-cache.json`, `usage.json`), then prints the file and the address to publish it at, kept
//! for this run, or that it is a new page. `address <url>` keeps the address the main thread got
//! when it published the page.

use std::env;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::UNIX_EPOCH;

use hooks::json::{Value, parse};
use hooks::time::now_ms;
use progress_page::address::{address_for, is_artifact_address, render_address};
use progress_page::agents::{Agent, agent_of, tokens_of_file};
use progress_page::backlog::{feature_of_branch, file_of, front_matter, story_rows};
use progress_page::commits::{LOG_FORMAT, parse_log};
use progress_page::page::{Page, RunView, Usage, render};
use progress_page::progress::{FeatureInput, features_progress};
use run_totals::marker::{Run, parse_run, whole};
use run_totals::totals::{Cache, count_run, projects_dir};

/// Without a run, the page covers the last day.
const NO_RUN_WINDOW_MS: i64 = 24 * 60 * 60 * 1000;
/// Bytes read at the start and at the end of a transcript.
const HEAD_BYTES: u64 = 16 * 1024;
const TAIL_BYTES: u64 = 64 * 1024;

fn git(words: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(words)
        .output()
        .map_err(|error| format!("git: {error}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(format!(
            "git {}: {}",
            words.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// The run's features: each `origin/feature/f<nn>-…` branch, its feature file and epic file read
/// from that branch, ordered by feature number.
fn features() -> Result<Vec<FeatureInput>, String> {
    let branches = git(&[
        "for-each-ref",
        "--format=%(refname:short)",
        "refs/remotes/origin/feature/",
    ])?;
    let mut found = Vec::new();

    for branch in branches.lines().filter(|line| !line.is_empty()) {
        let Some(id) = feature_of_branch(branch) else {
            continue;
        };
        let names: Vec<String> = git(&[
            "ls-tree",
            "--name-only",
            branch,
            "docs/backlog/features/",
            "docs/backlog/epics/",
        ])?
        .lines()
        .map(str::to_owned)
        .collect();
        let Some(feature_path) = file_of(&names, &id) else {
            continue;
        };
        let feature = git(&["show", &format!("{branch}:{feature_path}")])?;
        let epic = front_matter(&feature, "epic").unwrap_or_default();
        let epic_title = file_of(&names, &epic)
            .and_then(|path| git(&["show", &format!("{branch}:{path}")]).ok())
            .and_then(|text| front_matter(&text, "title"))
            .unwrap_or_default();

        found.push(FeatureInput {
            epic,
            epic_title,
            title: front_matter(&feature, "title").unwrap_or_default(),
            rows: story_rows(&feature),
            id,
        });
    }

    found.sort_by_key(|feature| feature.id[1..].parse::<u32>().unwrap_or(0));
    Ok(found)
}

/// Up to `limit` bytes of a file, from its start or from its end.
fn read_part(path: &Path, limit: u64, from_end: bool) -> String {
    let Ok(mut file) = File::open(path) else {
        return String::new();
    };
    let length = file.metadata().map_or(0, |data| data.len());
    let start = if from_end {
        length.saturating_sub(limit)
    } else {
        0
    };
    let mut bytes = Vec::new();

    if file.seek(SeekFrom::Start(start)).is_ok() {
        let _ = file.take(limit).read_to_end(&mut bytes);
    }

    String::from_utf8_lossy(&bytes).into_owned()
}

/// A file's last change, epoch milliseconds; 0 when unknown.
fn modified_ms(path: &Path) -> i64 {
    fs::metadata(path)
        .and_then(|data| data.modified())
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map_or(0, |elapsed| i64::try_from(elapsed.as_millis()).unwrap_or(0))
}

/// The subagents of the run's sessions written since `window`, their tokens from the cache.
fn agents(projects: &Path, sessions: &[String], window: i64, cache: &Cache) -> Vec<Agent> {
    let mut found = Vec::new();

    for session in sessions {
        let Ok(entries) = fs::read_dir(projects.join(session).join("subagents")) else {
            continue;
        };
        let mut paths: Vec<PathBuf> = entries
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
            .collect();

        paths.sort();

        for path in paths {
            let last = modified_ms(&path);
            let name = path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            let Some(id) = name
                .strip_prefix("agent-")
                .and_then(|rest| rest.strip_suffix(".jsonl"))
            else {
                continue;
            };

            if last < window {
                continue;
            }

            let meta = fs::read_to_string(path.with_extension("meta.json")).unwrap_or_default();
            let mut agent = agent_of(
                id,
                &meta,
                &read_part(&path, HEAD_BYTES, false),
                &read_part(&path, TAIL_BYTES, true),
                last,
            );

            agent.tokens = tokens_of_file(cache, &name);
            found.push(agent);
        }
    }

    found
}

/// The plan's usage, from `usage.json`, when the status line wrote one.
fn usage(metrics: &Path) -> Option<Usage> {
    let json = parse(&fs::read_to_string(metrics.join("usage.json")).ok()?)?;

    Some(Usage {
        five_hour: percent(json.get("fiveHour").get("usedPercent")),
        seven_day: percent(json.get("sevenDay").get("usedPercent")),
    })
}

/// The story ids that have a worktree.
fn worktrees(checkout: &Path) -> Vec<String> {
    fs::read_dir(checkout.join(".claude").join("worktrees"))
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default()
}

fn write_page(common: &Path, marker: Option<&Run>) -> Result<String, String> {
    let metrics = common.join("metrics");
    let checkout = common.parent().unwrap_or(common);
    let now = now_ms();
    let window = marker.map_or(now - NO_RUN_WINDOW_MS, |run| run.start);
    let home = PathBuf::from(env::var("HOME").unwrap_or_default());
    let projects = projects_dir(&home, common);
    // The run counted as `run-totals` counts it, its cache brought up to date for both.
    let counted = marker.map(|run| count_run(run, &metrics, &projects));
    let run_agents = match (marker, &counted) {
        (Some(run), Some((_, cache))) => agents(&projects, &run.sessions, window, cache),
        _ => Vec::new(),
    };
    let commits = parse_log(&git(&[
        "log",
        "--branches",
        "--remotes",
        &format!("--since=@{}", window / 1000),
        &format!("--format={LOG_FORMAT}"),
    ])?);
    let page = Page {
        now,
        run: marker.map(|run| RunView {
            start: run.start,
            end: run.end,
            tokens: counted.as_ref().map(|(tokens, _)| *tokens),
        }),
        usage: usage(&metrics),
        features: features_progress(
            &features()?,
            &commits,
            &run_agents,
            &worktrees(checkout),
            now,
        ),
        agents: run_agents,
    };
    let path = metrics.join("progress.html");

    fs::create_dir_all(&metrics).map_err(|error| error.to_string())?;
    fs::write(&path, render(&page)).map_err(|error| error.to_string())?;

    let saved = fs::read_to_string(metrics.join("progress-page.json")).unwrap_or_default();
    let next = match address_for(&saved, marker.map_or(0, |run| run.start)) {
        Some(url) => format!("publish it again at {url}"),
        None => "publish it as a new page, then: npm run progress:page -- address <url>".to_owned(),
    };

    Ok(format!("{}\n{next}\n", path.display()))
}

fn run(words: &[String]) -> Result<String, String> {
    let common =
        PathBuf::from(git(&["rev-parse", "--path-format=absolute", "--git-common-dir"])?.trim());
    let marker = fs::read_to_string(common.join("metrics").join("run.json"))
        .ok()
        .and_then(|text| parse_run(&text));

    match words.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [] => write_page(&common, marker.as_ref()),
        ["address", url] if is_artifact_address(url) => {
            let start = marker.map_or(0, |run| run.start);

            fs::write(
                common.join("metrics").join("progress-page.json"),
                render_address(start, url),
            )
            .map_err(|error| error.to_string())?;
            Ok(format!("address kept for this run: {url}\n"))
        }
        ["address", url] => Err(format!("progress-page: not a claude.ai address: {url}")),
        _ => Err("usage: progress-page [address <https://claude.ai/…>]".to_owned()),
    }
}

fn main() -> ExitCode {
    let words: Vec<String> = env::args().skip(1).collect();

    match run(&words) {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// A percent of `usage.json`, rounded as the status line rounds it.
fn percent(value: &Value) -> i64 {
    match value {
        Value::Number(number) => whole(&Value::Number(number.round())),
        _ => 0,
    }
}
