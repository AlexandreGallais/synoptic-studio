//! The findings of `epic:check` (RET-002, EN-055, EN-103), read from logs and outputs: the
//! branches left, the failed calls (the worktree isolation's refusals apart), the calls lost at a
//! force-quit, the checks of a failed `check:all`, each named with its lane and its failing tests,
//! and the table they make for the retrospective's record.

use calendar::{HOUR_MS, iso_minute};
use hooks::json::Value;

use crate::check_all::LANES;

/// One line of the check's table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub what: String,
    pub is_ok: bool,
    pub detail: String,
}

/// A running call's file older than this is stale: one hour.
const STALE_MS: i64 = HOUR_MS;

/// Characters of an error kept to group the failed calls.
const ERROR_TEXT: usize = 60;

/// Share of failed calls accepted, in percent.
const FAILED_PERCENT: usize = 2;

/// Error lines shown for the failed calls.
const ERRORS_SHOWN: usize = 5;

/// Characters of a failed `check:all`'s output kept when no failing check is named.
const OUTPUT_KEPT: usize = 300;

/// Failing tests named per check, the others counted.
const TESTS_SHOWN: usize = 10;

/// Characters of a lost call's command shown.
const SUMMARY_SHOWN: usize = 60;

/// Start of the error Claude Code returns when a worktree subagent's command leaves its worktree:
/// a refusal before the command runs, not a command that failed (549 of them in E15).
const ISOLATION_REFUSAL: &str = "This agent is isolated in the worktree";

/// The first characters of a text.
fn first_chars(text: &str, count: usize) -> &str {
    text.char_indices()
        .nth(count)
        .map_or(text, |(at, _)| &text[..at])
}

/// The last characters of a text.
fn last_chars(text: &str, count: usize) -> &str {
    let total = text.chars().count();

    text.char_indices()
        .nth(total.saturating_sub(count))
        .map_or(text, |(at, _)| &text[at..])
}

/// A string member of a JSON object, empty when missing or not a string.
fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value.get(key).text()
}

/// Branches left besides `main`, `develop` and the one checked out, local and on GitHub.
#[must_use]
pub fn left_branches(branches: &[String], current: &str) -> Finding {
    let kept = [
        "main".to_owned(),
        "develop".to_owned(),
        "origin/main".to_owned(),
        "origin/develop".to_owned(),
        "origin/HEAD".to_owned(),
        "origin".to_owned(),
        current.to_owned(),
        format!("origin/{current}"),
    ];
    let others: Vec<&str> = branches
        .iter()
        .filter(|branch| !kept.contains(branch))
        .map(String::as_str)
        .collect();

    Finding {
        what: "branches left".to_owned(),
        is_ok: others.is_empty(),
        detail: if others.is_empty() {
            "only main, develop and the retrospective's".to_owned()
        } else {
            others.join(", ")
        },
    }
}

/// A text without its terminal styles: the escape sequences Vitest may print (`ESC [ … m`, an
/// operating system command ended by `BEL` or `ESC \`, or an escape and one character).
#[must_use]
pub fn without_styles(text: &str) -> String {
    let mut kept = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(char) = chars.next() {
        match char {
            '\u{1b}' if chars.peek() == Some(&'[') => {
                chars.next();
                // Parameters and intermediates, up to the final byte, `@` to `~`.
                for next in chars.by_ref() {
                    if ('@'..='~').contains(&next) {
                        break;
                    }
                }
            }
            '\u{9b}' => {
                for next in chars.by_ref() {
                    if ('@'..='~').contains(&next) {
                        break;
                    }
                }
            }
            '\u{1b}' if chars.peek() == Some(&']') => {
                while let Some(next) = chars.next() {
                    if next == '\u{7}' || (next == '\u{1b}' && chars.next_if_eq(&'\\').is_some()) {
                        break;
                    }
                }
            }
            '\u{1b}' => {
                chars.next();
            }
            _ => kept.push(char),
        }
    }
    kept
}

/// A check's line in `check:all`'s output, `✓ lint 23.9 s` or `✗ test 26.2 s`: whether it failed
/// and its name.
fn check_line(line: &str) -> Option<(bool, &str)> {
    let (mark, rest) = if let Some(rest) = line.strip_prefix("✓ ") {
        (false, rest)
    } else {
        (true, line.strip_prefix("✗ ")?)
    };
    let rest = rest.strip_suffix(" s")?;
    let (name, time) = rest.split_once(' ')?;

    (!name.is_empty()
        && !name.chars().any(char::is_whitespace)
        && !time.is_empty()
        && time
            .chars()
            .all(|char| char.is_ascii_digit() || char == '.'))
    .then_some((mark, name))
}

/// A failing test as Vitest (`FAIL  file > suite > test`) or `cargo test` (`test x ... FAILED`)
/// names it.
fn failing_test(line: &str) -> Option<&str> {
    let line = line.trim();

    if let Some(rest) = line.strip_prefix("FAIL")
        && rest.starts_with(char::is_whitespace)
        && !rest.trim().is_empty()
    {
        return Some(rest.trim());
    }

    let name = line.strip_prefix("test ")?.strip_suffix(" ... FAILED")?;

    (!name.is_empty() && !name.chars().any(char::is_whitespace)).then_some(name)
}

/// The checks that failed in `check:all`'s output, each with its lane and its failing tests (ten
/// at most, the others counted): `test (lane test): a.test.ts > adds`.
#[must_use]
pub fn failing_checks(output: &str) -> Vec<String> {
    let clean = without_styles(output);
    let lines: Vec<&str> = clean.split('\n').collect();
    let checks: Vec<(usize, bool, &str)> = lines
        .iter()
        .enumerate()
        .filter_map(|(at, line)| check_line(line).map(|(failed, name)| (at, failed, name)))
        .collect();

    checks
        .iter()
        .enumerate()
        .filter(|(_, (_, failed, _))| *failed)
        .map(|(rank, (at, _, name))| {
            let end = checks.get(rank + 1).map_or(lines.len(), |next| next.0);
            let lane = LANES
                .iter()
                .find(|lane| lane.contains(name))
                .map_or_else(|| (*name).to_owned(), |lane| lane.join(" → "));
            let mut tests: Vec<&str> = Vec::new();

            for test in lines[at + 1..end]
                .iter()
                .filter_map(|line| failing_test(line))
            {
                if !tests.contains(&test) {
                    tests.push(test);
                }
            }

            let mut named: Vec<String> = tests
                .iter()
                .take(TESTS_SHOWN)
                .map(|&test| test.to_owned())
                .collect();

            if tests.len() > TESTS_SHOWN {
                named.push(format!("{} more", tests.len() - TESTS_SHOWN));
            }
            if named.is_empty() {
                format!("{name} (lane {lane})")
            } else {
                format!("{name} (lane {lane}): {}", named.join(", "))
            }
        })
        .collect()
}

/// The calls whose start was written but whose end never was: Claude Code quit, or was forced to,
/// while they ran. Each running call's record and how old its file is; those over an hour old are
/// listed as lost, never a failure.
#[must_use]
pub fn lost_calls(calls: &[(i64, Value)]) -> Finding {
    let mut lost: Vec<(i64, String)> = calls
        .iter()
        .filter(|(age, _)| *age > STALE_MS)
        .map(|(_, record)| {
            let start = match record.get("start") {
                Value::Number(number) => format!("{number:.0}").parse().unwrap_or(0),
                _ => 0,
            };

            (
                start,
                format!(
                    "{} {} {}",
                    iso_minute(start),
                    text(record, "tool"),
                    first_chars(text(record, "summary"), SUMMARY_SHOWN)
                ),
            )
        })
        .collect();

    lost.sort_by_key(|(start, _)| *start);

    Finding {
        what: "calls lost: started, never ended (a force-quit), over an hour old".to_owned(),
        is_ok: true,
        detail: if lost.is_empty() {
            "none".to_owned()
        } else {
            format!(
                "{} lost: {}",
                lost.len(),
                lost.into_iter()
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("; ")
            )
        },
    }
}

/// The calls of the metrics log that failed since a day (`YYYY-MM-DD`), and their most frequent
/// errors; over 2 % failed is a finding to look at.
#[must_use]
pub fn failed_calls(records: &[Value], since: &str) -> Finding {
    let all: Vec<&Value> = records
        .iter()
        .filter(|record| text(record, "time") >= since)
        .collect();
    let refusals = all
        .iter()
        .filter(|record| text(record, "error").starts_with(ISOLATION_REFUSAL))
        .count();
    let recent: Vec<&&Value> = all
        .iter()
        .filter(|record| !text(record, "error").starts_with(ISOLATION_REFUSAL))
        .collect();
    let failed: Vec<&&&Value> = recent
        .iter()
        .filter(|record| !text(record, "error").is_empty())
        .collect();
    let mut errors: Vec<(String, usize)> = Vec::new();

    for record in &failed {
        let key = format!(
            "{}: {}",
            text(record, "tool"),
            first_chars(text(record, "error"), ERROR_TEXT)
        );

        match errors.iter_mut().find(|(error, _)| *error == key) {
            Some((_, times)) => *times += 1,
            None => errors.push((key, 1)),
        }
    }
    // A stable sort: errors as frequent keep the order they were first met in.
    errors.sort_by_key(|(_, times)| std::cmp::Reverse(*times));

    let detail = std::iter::once(format!("{} of {}", failed.len(), recent.len()))
        .chain(
            errors
                .iter()
                .take(ERRORS_SHOWN)
                .map(|(error, times)| format!("{times} × {error}")),
        )
        .chain(std::iter::once(format!(
            "apart: {refusals} refused by the worktree isolation"
        )))
        .collect::<Vec<_>>()
        .join("; ");

    Finding {
        what: "calls failed during the epic (2 % at most)".to_owned(),
        is_ok: failed.len() * 100 <= recent.len() * FAILED_PERCENT,
        detail,
    }
}

/// The finding of `check:all`: its last line when it passed, else its failing checks, or its
/// output's end when it names none.
#[must_use]
pub fn check_all_finding(is_ok: bool, out: &str) -> Finding {
    let detail = if is_ok {
        out.split('\n').next_back().unwrap_or_default().to_owned()
    } else {
        let failing = failing_checks(out);

        if failing.is_empty() {
            last_chars(out, OUTPUT_KEPT).to_owned()
        } else {
            failing.join("; ")
        }
    };

    Finding {
        what: "check:all (code, site, playground, dependencies)".to_owned(),
        is_ok,
        detail,
    }
}

/// The backlog items of an epic not `done`, its retrospective left out: each file's name, without
/// `.md`, and its text.
#[must_use]
pub fn open_items(epic: &str, files: &[(String, String)]) -> Finding {
    let start = format!("{epic}-");
    let open: Vec<&str> = files
        .iter()
        .filter(|(name, _)| name.starts_with(&start) && !name.contains("-RET-"))
        .filter(|(_, text)| !text.lines().any(|line| line == "status: done"))
        .map(|(name, _)| name.strip_suffix(".md").unwrap_or(name))
        .collect();

    Finding {
        what: format!("backlog of {epic}"),
        is_ok: open.is_empty(),
        detail: if open.is_empty() {
            "every feature and story done".to_owned()
        } else {
            open.join(", ")
        },
    }
}

/// The Markdown table of the findings, for the retrospective's record.
#[must_use]
pub fn table(findings: &[Finding]) -> String {
    let rows = findings.iter().map(|finding| {
        format!(
            "| {} | {} | {} |",
            finding.what,
            if finding.is_ok {
                "ok"
            } else {
                "**to look at**"
            },
            finding.detail.replace('|', "\\|")
        )
    });

    [
        "| Check | Result | Detail |".to_owned(),
        "| --- | --- | --- |".to_owned(),
    ]
    .into_iter()
    .chain(rows)
    .collect::<Vec<_>>()
    .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use hooks::json::parse;

    /// One hour and one second, in milliseconds: a running call's file older than an hour.
    const OVER_AN_HOUR: i64 = 3_601_000;

    fn record(error: &str, time: &str, tool: &str) -> Value {
        parse(&format!(
            r#"{{"error":{},"time":"{time}","tool":"{tool}"}}"#,
            hooks::json::quote(error)
        ))
        .expect("JSON")
    }

    /// Calls of one day, the first one failed.
    fn one_failed_of(count: usize) -> Vec<Value> {
        std::iter::once(record("Exit code 1", "2026-10-13T10:00:00Z", "Bash"))
            .chain((1..count).map(|_| record("", "2026-10-13T10:00:00Z", "Bash")))
            .collect()
    }

    #[test]
    fn counts_the_calls_failed_since_the_epic_first_day_the_frequent_errors_first() {
        let records = [
            record("", "2026-10-12T10:00:00Z", "Bash"),
            record("Exit code 1", "2026-10-13T10:00:00Z", "Bash"),
            record("Exit code 1", "2026-10-13T11:00:00Z", "Bash"),
            record("File not found", "2026-10-14T10:00:00Z", "Read"),
            record("", "2026-10-14T11:00:00Z", "Bash"),
        ];

        // 3 of the 4 calls since 2026-10-13 failed: far over 2 %.
        assert_eq!(
            failed_calls(&records, "2026-10-13"),
            Finding {
                what: "calls failed during the epic (2 % at most)".to_owned(),
                is_ok: false,
                detail: "3 of 4; 2 × Bash: Exit code 1; 1 × Read: File not found; apart: 0 refused by the worktree isolation".to_owned(),
            }
        );
        assert!(failed_calls(&[], "2026-10-13").is_ok);
    }

    #[test]
    fn f18_ac11_counts_the_worktree_isolation_refusals_apart_from_the_failed_calls() {
        let refusal = "This agent is isolated in the worktree /repo/.claude/worktrees/us-061";
        let records: Vec<Value> = (0..60)
            .map(|_| record("", "2026-10-13T10:00:00Z", "Bash"))
            .chain([
                record(refusal, "2026-10-13T10:00:00Z", "Bash"),
                record(refusal, "2026-10-13T11:00:00Z", "Monitor"),
                record("Exit code 1", "2026-10-13T12:00:00Z", "Bash"),
            ])
            .collect();

        // 1 failed of 61 calls, the 2 refusals left out of both: 1.6 %, within 2 %.
        assert_eq!(
            failed_calls(&records, "2026-10-13"),
            Finding {
                what: "calls failed during the epic (2 % at most)".to_owned(),
                is_ok: true,
                detail:
                    "1 of 61; 1 × Bash: Exit code 1; apart: 2 refused by the worktree isolation"
                        .to_owned(),
            }
        );
    }

    #[test]
    fn keeps_five_errors_cut_at_sixty_characters() {
        let long = "x".repeat(70);
        let records: Vec<Value> = ["a", "b", "c", "d", "e", "f", long.as_str()]
            .iter()
            .map(|error| record(error, "2026-10-13T10:00:00Z", "Bash"))
            .collect();
        let detail = failed_calls(&records, "2026-10-13").detail;

        // Seven errors met once each: the first five, in the order met.
        assert_eq!(
            detail,
            "7 of 7; 1 × Bash: a; 1 × Bash: b; 1 × Bash: c; 1 × Bash: d; 1 × Bash: e; apart: 0 refused by the worktree isolation"
        );
        assert_eq!(first_chars(&long, 60).len(), 60);
        assert_eq!(first_chars("é", 60), "é");
        assert_eq!(last_chars("abcé", 2), "cé");
        assert_eq!(last_chars("ab", 5), "ab");
    }

    #[test]
    fn accepts_exactly_two_percent_of_failed_calls_and_a_call_an_hour_old_as_not_yet_lost() {
        // 1 of 50 is 2 %, the limit itself: accepted; 1 of 49 is 2.04 %: over it.
        assert!(failed_calls(&one_failed_of(50), "2026-10-13").is_ok);
        assert!(!failed_calls(&one_failed_of(49), "2026-10-13").is_ok);

        // 3 600 000 ms is one hour exactly: not over an hour, so not lost.
        let an_hour = (
            3_600_000,
            parse(r#"{"start":0,"summary":"ls","tool":"Bash"}"#).expect("JSON"),
        );

        assert_eq!(lost_calls(&[an_hour]).detail, "none");
    }

    #[test]
    fn f18_ac11_names_each_failing_check_its_lane_and_its_failing_tests() {
        let output = [
            "✓ lint 23.9 s",
            "✓ format:check 2.0 s",
            "✗ test 26.2 s",
            "",
            " FAIL  scripts/epic-check.test.ts > epic-check > lists the branches",
            "AssertionError: expected 1 to be 2",
            "\u{1b}[31m FAIL \u{1b}[39m src/math/unit.test.ts > unit > keeps a zero vector",
            " FAIL  scripts/epic-check.test.ts > epic-check > lists the branches",
            "✗ check:rust 4.1 s",
            "test heavy_lock_waits ... FAILED",
            "test heavy_lock_takes ... ok",
            "✗ playground:build 3.0 s",
            "error during build",
            "check:all failed in 30.1 s: test, check:rust, playground:build",
        ]
        .join("\n");

        assert_eq!(
            failing_checks(&output),
            [
                "test (lane test): scripts/epic-check.test.ts > epic-check > lists the branches, src/math/unit.test.ts > unit > keeps a zero vector",
                "check:rust (lane check:rust): heavy_lock_waits",
                "playground:build (lane docs:build → playground:build)",
            ]
        );
    }

    #[test]
    fn f18_ac11_puts_the_failing_checks_in_check_all_line_its_last_line_when_it_passed() {
        assert_eq!(
            check_all_finding(false, "✗ lint 1.0 s\nerror\ncheck:all failed"),
            Finding {
                what: "check:all (code, site, playground, dependencies)".to_owned(),
                is_ok: false,
                detail: "lint (lane lint)".to_owned(),
            }
        );
        // No check named (the lock failed before any check ran): the output's end.
        assert_eq!(
            check_all_finding(false, "heavy lock: cannot start").detail,
            "heavy lock: cannot start"
        );
        assert_eq!(
            check_all_finding(false, &"x".repeat(310)).detail,
            "x".repeat(300)
        );
        assert_eq!(
            check_all_finding(true, "✓ lint 1.0 s\ncheck:all passed in 1.0 s"),
            Finding {
                what: "check:all (code, site, playground, dependencies)".to_owned(),
                is_ok: true,
                detail: "check:all passed in 1.0 s".to_owned(),
            }
        );
    }

    #[test]
    fn names_no_check_when_check_all_passed_and_at_most_ten_tests_of_one_check() {
        assert_eq!(
            failing_checks("✓ lint 1.0 s\ncheck:all passed in 1.0 s"),
            Vec::<String>::new()
        );
        assert_eq!(failing_checks(""), Vec::<String>::new());

        // Ten tests are all named, without a count.
        let ten = (0..10)
            .map(|index| format!("test t{index} ... FAILED"))
            .collect::<Vec<_>>()
            .join("\n");

        assert!(!failing_checks(&format!("✗ check:rust 1.0 s\n{ten}"))[0].contains("more"));

        let many: Vec<String> = (0..12)
            .map(|index| format!(" FAIL  a.test.ts > t{index}"))
            .collect();
        let output = std::iter::once("✗ test 1.0 s".to_owned())
            .chain(many)
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(
            failing_checks(&output),
            [
                "test (lane test): a.test.ts > t0, a.test.ts > t1, a.test.ts > t2, a.test.ts > t3, a.test.ts > t4, a.test.ts > t5, a.test.ts > t6, a.test.ts > t7, a.test.ts > t8, a.test.ts > t9, 2 more"
            ]
        );
    }

    #[test]
    fn reads_only_the_lines_of_a_check_and_of_a_failing_test() {
        // The `.git/config` line of check:all names no check; a lane-less name is its own lane.
        assert_eq!(
            check_line("✗ /r/.git/config changed while the checks ran, restored nothing:"),
            None
        );
        assert_eq!(check_line("✓ lint 1.0 s"), Some((false, "lint")));
        assert_eq!(check_line("✗ x 1 s"), Some((true, "x")));
        assert_eq!(check_line("✗ x  s"), None);
        assert_eq!(check_line("✗ x 1a s"), None);
        assert_eq!(failing_checks("✗ odd 1.0 s"), ["odd (lane odd)"]);
        assert_eq!(failing_test("FAIL"), None);
        assert_eq!(failing_test("FAILED x"), None);
        assert_eq!(failing_test("  FAIL\ta > b  "), Some("a > b"));
        assert_eq!(failing_test("test a b ... FAILED"), None);
        assert_eq!(failing_test("test  ... FAILED"), None);
    }

    #[test]
    fn removes_the_terminal_styles() {
        assert_eq!(without_styles("\u{1b}[1;31mred\u{1b}[0m"), "red");
        assert_eq!(without_styles("a\u{1b}]8;;http://x\u{7}b"), "ab");
        assert_eq!(without_styles("a\u{1b}]0;t\u{1b}\\b"), "ab");
        assert_eq!(without_styles("a\u{9b}2Kb"), "ab");
        assert_eq!(without_styles("a\u{1b}Mb"), "ab");
        assert_eq!(without_styles("plain ✓"), "plain ✓");
    }

    #[test]
    fn f18_ac11_reports_the_calls_started_over_an_hour_ago_and_never_ended_as_lost() {
        let calls = [
            (
                OVER_AN_HOUR,
                parse(r#"{"agent":"main","start":1792112400000,"summary":"npm run check:all","tool":"Bash"}"#)
                    .expect("JSON"),
            ),
            (
                1000,
                parse(r#"{"start":1792190000000,"summary":"ls","tool":"Bash"}"#).expect("JSON"),
            ),
            (
                OVER_AN_HOUR * 2,
                parse(r#"{"start":1792108800000,"summary":"docs/a.md","tool":"Read"}"#)
                    .expect("JSON"),
            ),
        ];

        // 2026-10-16T00:00Z is 20 742 days × 86 400 000 ms = 1 792 108 800 000 ms; one hour
        // later, 3 600 000 ms more.
        assert_eq!(
            lost_calls(&calls),
            Finding {
                what: "calls lost: started, never ended (a force-quit), over an hour old"
                    .to_owned(),
                is_ok: true,
                detail: "2 lost: 2026-10-16T00:00 Read docs/a.md; 2026-10-16T01:00 Bash npm run check:all".to_owned(),
            }
        );
        assert_eq!(lost_calls(&[]).detail, "none");
    }

    #[test]
    fn lists_the_branches_left_besides_main_develop_and_the_one_checked_out() {
        let branches: Vec<String> = [
            "develop",
            "main",
            "docs/ret-002",
            "origin/docs/ret-002",
            "origin/HEAD",
            "feat/old",
        ]
        .map(str::to_owned)
        .to_vec();

        assert_eq!(
            left_branches(&branches, "docs/ret-002"),
            Finding {
                what: "branches left".to_owned(),
                is_ok: false,
                detail: "feat/old".to_owned(),
            }
        );
        assert_eq!(
            left_branches(&branches[..2], "x").detail,
            "only main, develop and the retrospective's"
        );
    }

    #[test]
    fn lists_the_open_items_of_an_epic_and_writes_the_table() {
        let files = [
            ("E14-F01-x.md".to_owned(), "status: done\n".to_owned()),
            (
                "E14-F01-US-001-a.md".to_owned(),
                "status: ready\n".to_owned(),
            ),
            ("E14-RET-002-r.md".to_owned(), "status: draft\n".to_owned()),
            ("E15-F02-y.md".to_owned(), "status: draft\n".to_owned()),
        ];
        let open = open_items("E14", &files);

        assert_eq!(open.detail, "E14-F01-US-001-a");
        assert!(!open.is_ok);
        assert_eq!(
            open_items("E14", &files[..1]).detail,
            "every feature and story done"
        );
        assert_eq!(
            table(&[
                open,
                Finding {
                    what: "w".to_owned(),
                    is_ok: true,
                    detail: "a | b".to_owned()
                }
            ]),
            "| Check | Result | Detail |\n| --- | --- | --- |\n| backlog of E14 | **to look at** | E14-F01-US-001-a |\n| w | ok | a \\| b |"
        );
    }
}
