//! Tests of the report's statistics and tables: the cases of `scripts/metrics-tables.test.ts`.

use metrics::tables::{TimedCall, call_stats, command_key, js_prefix, markdown_table};

#[test]
fn names_a_command_by_its_words_without_directory_change_variables_flags_or_quotes() {
    assert_eq!(
        command_key("cd \"/repo\" && npm run check:all 2>&1 | tail"),
        "npm run check:all"
    );
    assert_eq!(command_key("S=/tmp/x; git commit -q -m 'x'"), "git commit");
    assert_eq!(command_key("gh pr checks 12 --watch"), "gh pr checks");
    assert_eq!(command_key("npm install -D eslint"), "npm install");
    assert_eq!(command_key("grep -n foo bar.ts"), "grep");
    assert_eq!(command_key("npm"), "npm");
}

#[test]
fn f32_ac1_names_the_command_after_the_brief_s_cd_or_exit() {
    // RET-004 (I91): `exit` was the first command by count, 10,573 calls, every worker's call
    // opening with `cd "<worktree>" || exit 9`; the `exit` after `||` runs only when the `cd`
    // fails, so the command named is the one after it, behind a `;` or a line break.
    assert_eq!(command_key("cd \"/w\" || exit 9; ls"), "ls");
    assert_eq!(
        command_key("cd \"/w\" || exit 9\ntarget/release/git-sequences sync"),
        "target/release/git-sequences"
    );
    assert_eq!(command_key("cd \"/w\" || exit 9;git status"), "git status");
    // A command before `||` is the one named, whatever follows it.
    assert_eq!(command_key("make || exit 1"), "make");
    // An `exit` that is not after `||` is a command that runs.
    assert_eq!(command_key("exit 3"), "exit");
    assert_eq!(command_key("cd /w; exit 3"), "exit");
}

#[test]
fn is_empty_when_nothing_runs() {
    assert_eq!(command_key("cd \"/repo\""), "");
    assert_eq!(command_key(""), "");
    assert_eq!(command_key("cd \"/w\" || exit 9"), "");
}

#[test]
fn counts_sums_and_ranks_the_calls_of_each_group() {
    let call = |duration_ms: f64, is_ok: bool, summary: &str| TimedCall {
        duration_ms,
        is_ok,
        summary: summary.to_owned(),
    };
    let calls = [
        call(1000.0, true, "npm run check"),
        call(3000.0, false, "npm run check"),
        call(200.0, true, "ls"),
    ];
    let stats = call_stats(&calls, |call| command_key(&call.summary));

    // npm run check: 1 000 and 3 000, median 2 000, total 4 000, one failure; then ls.
    assert_eq!(stats.len(), 2);
    assert_eq!(
        (stats[0].key.as_str(), stats[0].count, stats[0].failures),
        ("npm run check", 2, 1)
    );
    assert!((stats[0].median_ms - 2000.0).abs() < f64::EPSILON);
    assert!((stats[0].total_ms - 4000.0).abs() < f64::EPSILON);
    assert!((stats[0].max_ms - 3000.0).abs() < f64::EPSILON);
    assert_eq!((stats[1].key.as_str(), stats[1].count), ("ls", 1));
}

#[test]
fn writes_a_header_row_a_separator_and_escapes_pipes() {
    assert_eq!(
        markdown_table(&["A", "B"], &[vec!["x | y".to_owned(), "1".to_owned()]]),
        "| A | B |\n| --- | --- |\n| x \\| y | 1 |"
    );
}

#[test]
fn cuts_a_text_in_utf16_units_as_javascript_does() {
    assert_eq!(js_prefix("abcdef", 3), "abc");
    assert_eq!(js_prefix("ab", 8), "ab");
    // `é` is one unit; `😀` two, so a cut after one of them leaves a replacement character.
    assert_eq!(js_prefix("é😀x", 2), "é\u{fffd}");
    assert_eq!(js_prefix("é😀x", 3), "é😀");
}
