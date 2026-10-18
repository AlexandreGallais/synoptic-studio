//! Tests of the rules' findings read from tool results (EN-107): each report form, the hooks'
//! refusals, and the table per epic.

use metrics::findings::{findings_table, hook_refusal, line_finding, plain, result_findings};

/// The kind and rule a line reports.
fn found(line: &str) -> Option<(&'static str, String)> {
    line_finding(line)
}

/// A finding of `kind` and `rule`.
fn finding(kind: &'static str, rule: &str) -> (&'static str, String) {
    (kind, rule.to_owned())
}

#[test]
fn reads_each_linters_report_form() {
    // ESLint's: the place, the level, the message, two spaces, the rule.
    assert_eq!(
        found("  50:160  error  Label reference 'x' not found  markdown/no-missing-label-refs"),
        Some(finding("lint", "markdown/no-missing-label-refs"))
    );
    assert_eq!(
        found("   3:1   warning  Too complex  complexity"),
        Some(finding("lint", "complexity"))
    );
    // oxlint's, on one line or in its graphical form; ESLint's own rules without a plugin.
    assert_eq!(
        found("scripts/a.ts:386:2: error eslint(max-lines): File has too many lines"),
        Some(finding("lint", "max-lines"))
    );
    assert_eq!(
        found("  × typescript-eslint(no-explicit-any): Unexpected any."),
        Some(finding("lint", "typescript/no-explicit-any"))
    );
    assert_eq!(
        found("  ⚠ eslint-plugin-unicorn(no-null): Do not use null"),
        Some(finding("lint", "unicorn/no-null"))
    );
    // The project's linter, and markdownlint.
    assert_eq!(
        found("src/a.ts:3:44 local/kind-limits A procedure of 16 statements"),
        Some(finding("lint", "local/kind-limits"))
    );
    assert_eq!(
        found("docs/x.md:78:81 error MD013/line-length Line length [Expected: 100]"),
        Some(finding("lint", "MD013/line-length"))
    );
}

#[test]
fn reads_type_errors_clippy_lints_and_failed_checks() {
    assert_eq!(
        found("src/a.ts(3,5): error TS2322: Type 'x' is not assignable"),
        Some(finding("type", "TS2322"))
    );
    assert_eq!(
        found("src/a.ts:3:5 - error TS2345: Argument"),
        Some(finding("type", "TS2345"))
    );
    assert_eq!(
        found(
            "   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.99.0/index.html#too_many_lines"
        ),
        Some(finding("clippy", "clippy::too_many_lines"))
    );
    assert_eq!(found("✗ test 26.2 s"), Some(finding("check", "test")));
}

#[test]
fn reads_nothing_in_text_that_only_looks_like_a_finding() {
    // A search result's place, its text after a colon; a place followed by prose; a passed check.
    assert_eq!(found("src/a.ts:12:5:  // see docs/x"), None);
    assert_eq!(found("src/a.ts:12:5 some words here"), None);
    assert_eq!(found("✓ test 26.2 s"), None);
    assert_eq!(found("✗ test passed"), None);
    assert_eq!(found("  1:1  error  Parsing error: Unexpected token"), None);
    assert_eq!(found("an error TS without a code"), None);
    assert_eq!(found("  × tan(0)"), None);
    assert_eq!(found(""), None);
}

#[test]
fn names_a_hooks_refusal_by_its_event_and_the_first_clause_of_its_reason() {
    assert_eq!(
        hook_refusal(
            "PreToolUse:Bash hook error: A worktree refuses a chained Git command: run `cd`"
        ),
        Some("PreToolUse:Bash: A worktree refuses a chained Git command".to_owned())
    );
    // A colon without a space after does not end the reason; a period does, and goes.
    assert_eq!(
        hook_refusal(
            "PreToolUse:Bash hook error: The Git hooks (check:all) are never skipped. Fix it"
        ),
        Some("PreToolUse:Bash: The Git hooks (check:all) are never skipped".to_owned())
    );
    assert_eq!(
        hook_refusal(
            "PreToolUse:Bash hook error: Story pull requests are merged by the Product Owner."
        ),
        Some("PreToolUse:Bash: Story pull requests are merged by the Product Owner".to_owned())
    );
    // Code quotes are left out, so that the table's code span holds.
    assert_eq!(
        hook_refusal("PreToolUse:Agent hook error: Run `x` alone\nmore"),
        Some("PreToolUse:Agent: Run x alone".to_owned())
    );
    assert_eq!(hook_refusal("Exit code 1\nhook error: x"), None);
    assert_eq!(hook_refusal("Some text hook error: x"), None);
}

#[test]
fn counts_a_results_findings_once_per_rule_a_refusal_whatever_the_tool() {
    let output = "\u{1b}[31m  ×\u{1b}[0m eslint(id-length): short\n  × eslint(id-length): short\nsrc/a.ts:1:1 local/kind-naming x\n";

    assert_eq!(plain("\u{1b}[1;31mred\u{1b}[0m"), "red");
    assert_eq!(
        result_findings(output, true, true),
        [
            ("lint".to_owned(), "id-length".to_owned(), 2),
            ("lint".to_owned(), "local/kind-naming".to_owned(), 1)
        ]
    );
    // The same text read back by another tool than the shell is no finding.
    assert_eq!(
        result_findings(output, true, false),
        [] as [(std::string::String, std::string::String, i64); 0]
    );
    // A refusal is one finding of its hook, for any tool; only an error result is one.
    let refusal = "PreToolUse:SubagentHandback hook error: Hand-back refused once (check-handback)";

    assert_eq!(
        result_findings(refusal, true, false),
        [(
            "hook".to_owned(),
            "PreToolUse:SubagentHandback: Hand-back refused once (check-handback)".to_owned(),
            1
        )]
    );
    assert_eq!(
        result_findings(refusal, false, false),
        [] as [(std::string::String, std::string::String, i64); 0]
    );
}

#[test]
fn tables_each_rule_with_its_count_per_epic_the_most_found_first() {
    let row = |epic: &str, kind: &str, rule: &str, count: i64| {
        (epic.to_owned(), kind.to_owned(), rule.to_owned(), count)
    };
    let table = findings_table(&[
        row("E17", "lint", "local/kind-limits", 3),
        row("E15", "lint", "local/kind-limits", 5),
        row("E17", "lint", "local/kind-limits", 1),
        row("E17", "type", "TS2322", 2),
        row("develop", "hook", "PreToolUse:Bash: x | y", 2),
    ]);

    // kind-limits: 3 + 5 + 1 = 9, E15 5 then E17 4; the two of 2 by kind, hook before type.
    assert_eq!(
        table.lines().collect::<Vec<_>>(),
        [
            "| Kind | Rule | Findings | Per epic |",
            "| --- | --- | --- | --- |",
            "| lint | `local/kind-limits` | 9 | E15 5 · E17 4 |",
            "| hook | `PreToolUse:Bash: x \\| y` | 2 | develop 2 |",
            "| type | `TS2322` | 2 | E17 2 |",
        ]
    );
}
