//! The binary as `npm run lint` runs it: from the root, on its targets, failing on a finding.

use std::fs;

/// Runs the linter in a folder on its targets: its output and whether it succeeded.
fn lint(folder: &std::path::Path, targets: &[&str]) -> (String, bool) {
    let output = test_support::command(env!("CARGO_BIN_EXE_lint"))
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

/// A formatted sheet whose fixes move a declaration, drop an overridden longhand and join a
/// redundant nesting.
const UNSORTED: &str = ".a {\n  color: var(--c);\n  margin-top: 0;\n  margin: 8px;\n  display: block;\n\n  .d {\n    .e {\n      display: none;\n    }\n  }\n}\n";

#[cfg(unix)]
#[test]
fn leaves_a_formatted_sheet_formatted_once_fixed() {
    let root = std::env::temp_dir().join(format!("lint-cli-format-{}", std::process::id()));
    let project = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("playground")).unwrap_or_default();
    let sheet = root.join("playground/a.scss");
    fs::write(&sheet, UNSORTED).unwrap_or_default();

    let _ = test_support::command(env!("CARGO_BIN_EXE_lint"))
        .args(["--fix", "playground"])
        .current_dir(&root)
        .output();
    let fixed = fs::read_to_string(&sheet).unwrap_or_default();
    let oxfmt = test_support::command(project.join("node_modules/.bin/oxfmt"))
        .arg("--list-different")
        .arg("-c")
        .arg(project.join(".oxfmtrc.json"))
        .arg("playground/a.scss")
        .current_dir(&root)
        .output()
        .unwrap_or_else(|error| panic!("cannot run oxfmt: {error}"));

    // `display` (Recess's 19th) first, `margin-top` dropped under `margin`, `color` last; `.e`'s
    // content one level up: nothing left for oxfmt to change.
    assert_eq!(
        fixed,
        ".a {\n  display: block;\n  margin: 8px;\n  color: var(--c);\n\n  .d .e {\n    display: none;\n  }\n}\n"
    );
    assert_eq!(String::from_utf8_lossy(&oxfmt.stdout), "");
    assert!(oxfmt.status.success());
}
