//! The binary as `npm run lint` runs it: from the root, on its targets, failing on a finding.

use std::fs;
use std::process::Command;

/// Runs the linter in a folder on its targets: its output and whether it succeeded.
fn lint(folder: &std::path::Path, targets: &[&str]) -> (String, bool) {
    let output = Command::new(env!("CARGO_BIN_EXE_lint"))
        .args(targets)
        .current_dir(folder)
        .output()
        .unwrap_or_else(|error| panic!("cannot run the linter: {error}"));

    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        output.status.success(),
    )
}

#[test]
fn succeeds_on_a_clean_tree_and_fails_on_a_finding() {
    let root = std::env::temp_dir().join(format!("lint-cli-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("scripts")).unwrap_or_default();
    fs::write(
        root.join("scripts/a.ts"),
        "/** One. */\nexport const A = 1;\n",
    )
    .unwrap_or_default();

    assert_eq!(
        lint(&root, &["scripts"]),
        ("lint: 0 finding(s) in 1 file(s)\n".to_owned(), true)
    );

    fs::write(root.join("scripts/a.ts"), "enum A { B }\n").unwrap_or_default();
    let (report, clean) = lint(&root, &["scripts"]);
    assert!(
        report.starts_with("scripts/a.ts:1:1 no-restricted-syntax "),
        "{report}"
    );
    assert!(!clean);
    assert!(!lint(&root, &[]).1);
}
