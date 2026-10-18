//! `epic-check <epic> <first day>` (`npm run epic:check`, RET-002, EN-103): the full check of what
//! an epic produced, at its start and its end — every check of the code and the site, the branches
//! and pull requests left, the code scanning alerts on GitHub, the calls that failed during the epic
//! and why (the worktree isolation's refusals apart), calls lost at a force-quit, the backlog of the
//! epic, the last save of the private records. A failed `check:all` is named by its failing checks, their lanes
//! and their failing tests (EN-055). Prints a Markdown table to copy into the retrospective's
//! record, from the repository's root.

use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::SystemTime;

use calendar::now_ms;
use hooks::json::{Value, parse};
use repo_tools::epic_check::{
    Finding, check_all_finding, failed_calls, left_branches, lost_calls, open_items, table,
};
use repo_tools::records_save::{LAST_SAVE, ROOT_VARIABLE, project_name, records_folder};

/// A command's trimmed output and whether it succeeded: its own output when it failed — check:all
/// names the checks that failed — else the error, as Node.js wrote it.
fn run(root: &Path, command: &str, arguments: &[&str]) -> (bool, String) {
    match Command::new(command)
        .args(arguments)
        .current_dir(root)
        .output()
    {
        Ok(output) if output.status.success() => (
            true,
            String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        ),
        Ok(output) => {
            let out = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            let said = if out.is_empty() {
                format!(
                    "Error: Command failed: {command} {}\n{}",
                    arguments.join(" "),
                    String::from_utf8_lossy(&output.stderr)
                )
            } else {
                out
            };

            (false, said.trim().to_owned())
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            (false, format!("Error: spawnSync {command} ENOENT"))
        }
        Err(error) => (false, format!("Error: {error}")),
    }
}

/// The parsed lines of the metrics log, none when it is missing.
fn metrics_log(metrics: &Path) -> Vec<Value> {
    fs::read_to_string(metrics.join("tool-calls.jsonl"))
        .unwrap_or_default()
        .split('\n')
        .filter(|line| line.starts_with('{'))
        .filter_map(parse)
        .collect()
}

/// The running calls' records and how old each file is, in milliseconds.
fn running_calls(metrics: &Path) -> Vec<(i64, Value)> {
    let Ok(entries) = fs::read_dir(metrics.join("running")) else {
        return Vec::new();
    };
    let now = now_ms();

    entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let record = parse(&fs::read_to_string(entry.path()).ok()?)?;
            let modified = entry.metadata().ok()?.modified().ok()?;
            let since = modified
                .duration_since(SystemTime::UNIX_EPOCH)
                .ok()?
                .as_millis();

            Some((now - i64::try_from(since).ok()?, record))
        })
        .collect()
}

/// The features' and stories' files, with their text.
fn backlog_files(root: &Path) -> Vec<(String, String)> {
    ["features", "stories"]
        .iter()
        .flat_map(|folder| {
            let folder = root.join("docs").join("backlog").join(folder);
            let mut names: Vec<String> = fs::read_dir(&folder)
                .map(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .map(|entry| entry.file_name().to_string_lossy().into_owned())
                        .collect()
                })
                .unwrap_or_default();

            names.sort();
            names
                .into_iter()
                .map(|name| {
                    let text = fs::read_to_string(folder.join(&name)).unwrap_or_default();

                    (name, text)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

/// What GitHub holds open: the pull requests, the issues, the code scanning alerts.
fn github_findings(root: &Path) -> Vec<Finding> {
    let prs = run(
        root,
        "gh",
        &[
            "pr",
            "list",
            "--json",
            "number,title",
            "-q",
            r##".[] | "#\(.number) \(.title)""##,
        ],
    );
    let issues = run(
        root,
        "gh",
        &["issue", "list", "--json", "number", "-q", "length"],
    );
    let alerts = run(
        root,
        "gh",
        &[
            "api",
            "repos/{owner}/{repo}/code-scanning/alerts?state=open&per_page=100",
            "-q",
            r#".[] | "\(.rule.id) \(.most_recent_instance.location.path) on \(.most_recent_instance.ref)""#,
        ],
    );
    let listed = |out: &str| {
        if out.is_empty() {
            "none".to_owned()
        } else {
            out.replace('\n', "; ")
        }
    };

    vec![
        Finding {
            what: "pull requests open".to_owned(),
            is_ok: true,
            detail: listed(&prs.1),
        },
        Finding {
            what: "issues open".to_owned(),
            is_ok: issues.1 == "0",
            detail: issues.1.clone(),
        },
        Finding {
            what: "code scanning alerts open (GitHub, Security)".to_owned(),
            is_ok: alerts.0 && alerts.1.is_empty(),
            detail: listed(&alerts.1),
        },
    ]
}

/// The private records: their folder and the last save (ADR-0081).
fn records_finding(root: &Path) -> Finding {
    let origin = run(root, "git", &["config", "--get", "remote.origin.url"]).1;
    let main = Path::new(
        &run(
            root,
            "git",
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )
        .1,
    )
    .parent()
    .map_or_else(PathBuf::new, Path::to_path_buf);
    let folder = records_folder(
        env::var(ROOT_VARIABLE).ok().as_deref(),
        &project_name(&origin, &main),
    );
    let last = folder
        .as_ref()
        .map(|folder| fs::read_to_string(folder.join(LAST_SAVE)).unwrap_or_default());

    Finding {
        what: "private records".to_owned(),
        is_ok: last.as_ref().is_ok_and(|last| !last.is_empty()),
        detail: match (&folder, &last) {
            (Ok(_), Ok(last)) if !last.is_empty() => format!("last save {}", last.trim()),
            (Ok(folder), _) => format!("never saved in {}", folder.display()),
            (Err(error), _) => error.clone(),
        },
    }
}

fn work() -> u8 {
    let arguments: Vec<String> = env::args().skip(1).collect();
    let epic = arguments.first().map_or("", String::as_str);
    let since = arguments.get(1).map_or("", String::as_str);
    let root = env::current_dir().unwrap_or_default();
    let (_, common) = run(
        &root,
        "git",
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    );
    let metrics = Path::new(&common).join("metrics");
    let running = running_calls(&metrics);
    let checks = run(&root, "npm", &["run", "--silent", "check:all"]);
    let branches = run(&root, "git", &["branch", "-a", "--format=%(refname:short)"]).1;
    let current = run(&root, "git", &["branch", "--show-current"]).1;
    let mut findings = vec![
        check_all_finding(checks.0, &checks.1),
        left_branches(
            &branches.split('\n').map(str::to_owned).collect::<Vec<_>>(),
            &current,
        ),
    ];

    findings.extend(github_findings(&root));
    findings.extend([
        failed_calls(&metrics_log(&metrics), since),
        lost_calls(&running),
        open_items(epic, &backlog_files(&root)),
        records_finding(&root),
        Finding {
            what: "golden file of refactors".to_owned(),
            is_ok: true,
            detail: if metrics.join("golden.json").exists() {
                "present".to_owned()
            } else {
                "none written yet".to_owned()
            },
        },
    ]);

    println!("{}", table(&findings));
    u8::from(!findings.iter().all(|finding| finding.is_ok))
}

fn main() -> ExitCode {
    tool_use::run("epic-check", "", work)
}
