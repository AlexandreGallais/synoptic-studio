//! The formatter run on a temporary repository with the project's oxfmt (EN-073, F19.AC12): a
//! table whose widest cell grows changes on that row only, paragraphs stay on their lines, and
//! `--check` refuses a padded table or an unformatted file without changing them.

#![cfg(unix)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A Markdown page with a table, a paragraph on two lines and a list item on two lines.
const PAGE: &str = "# Page\n\n| Name | Size |\n| --- | ---: |\n| a | 1 |\n| bb | 22 |\n\nA paragraph\nwritten on two lines.\n\n- an item\n  continued\n";

/// The same page once its first cell has grown past every other cell of its column.
const GROWN: &str = "# Page\n\n| Name | Size |\n| --- | ---: |\n| a cell much wider than the others | 1 |\n| bb | 22 |\n\nA paragraph\nwritten on two lines.\n\n- an item\n  continued\n";

/// A temporary repository holding the project's oxfmt and a configuration.
fn repository(name: &str) -> PathBuf {
    let folder = std::env::temp_dir().join(format!("format-test-{name}-{}", std::process::id()));
    let project = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    if folder.exists() {
        fs::remove_dir_all(&folder).unwrap();
    }

    fs::create_dir_all(&folder).unwrap();
    std::os::unix::fs::symlink(
        project.join("node_modules").canonicalize().unwrap(),
        folder.join("node_modules"),
    )
    .unwrap();
    fs::write(folder.join(".gitignore"), "node_modules\n").unwrap();
    fs::write(
        folder.join(".oxfmtrc.json"),
        "{\n  \"printWidth\": 100,\n  \"proseWrap\": \"preserve\",\n  \"ignorePatterns\": [\"CHANGELOG.md\"]\n}\n",
    )
    .unwrap();
    git(&folder, &["init", "--quiet"]);

    folder
}

/// Runs Git in a folder, without the variables a hook may set (`GIT_DIR`).
fn git(folder: &Path, arguments: &[&str]) {
    let mut command = Command::new("git");

    for (key, _) in std::env::vars().filter(|(key, _)| key.starts_with("GIT_")) {
        command.env_remove(key);
    }

    assert!(
        command
            .args(arguments)
            .current_dir(folder)
            .status()
            .unwrap()
            .success()
    );
}

/// Runs the formatter in a folder.
fn format(folder: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_format"))
        .args(arguments)
        .current_dir(folder)
        .output()
        .unwrap()
}

/// The lines of two texts that differ, by position.
fn changed_lines(before: &str, after: &str) -> Vec<usize> {
    before
        .lines()
        .zip(after.lines())
        .enumerate()
        .filter(|(_, (left, right))| left != right)
        .map(|(index, _)| index)
        .collect()
}

#[test]
fn f19_ac12_a_wider_cell_changes_its_own_row_only() {
    let folder = repository("grow");
    let page = folder.join("page.md");

    fs::write(&page, PAGE).unwrap();
    assert!(format(&folder, &["--write", "*"]).status.success());

    let before = fs::read_to_string(&page).unwrap();

    // Written without padding, the paragraph and the item kept on their two lines.
    assert_eq!(before, PAGE);

    fs::write(&page, GROWN).unwrap();
    assert!(format(&folder, &["--write", "*"]).status.success());

    let after = fs::read_to_string(&page).unwrap();

    assert_eq!(changed_lines(&before, &after), vec![4]);
    assert_eq!(before.lines().count(), after.lines().count());
    fs::remove_dir_all(&folder).unwrap();
}

#[test]
fn f19_ac12_check_refuses_a_padded_table_and_changes_nothing() {
    let folder = repository("padded");
    let page = folder.join("page.md");
    let padded = "# Page\n\n| Name | Size |\n| ---- | ---: |\n| a    |    1 |\n";

    fs::write(&page, padded).unwrap();

    let output = format(&folder, &["--check", "*"]);

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("page.md: not formatted"));
    assert_eq!(fs::read_to_string(&page).unwrap(), padded);

    assert!(format(&folder, &["--write", "page.md"]).status.success());
    assert!(format(&folder, &["--check", "*"]).status.success());
    fs::remove_dir_all(&folder).unwrap();
}

#[test]
fn f19_ac5_check_refuses_an_unformatted_file_and_skips_ignored_pages() {
    let folder = repository("code");
    let code = folder.join("code.ts");
    let ignored = "| a | b |\n| ---- | - |\n";

    fs::write(&code, "const  a = {b:1}\n").unwrap();
    fs::write(folder.join("CHANGELOG.md"), ignored).unwrap();

    assert!(!format(&folder, &["--check", "*"]).status.success());
    assert!(format(&folder, &["--write", "*"]).status.success());
    assert_eq!(fs::read_to_string(&code).unwrap(), "const a = { b: 1 };\n");
    assert_eq!(
        fs::read_to_string(folder.join("CHANGELOG.md")).unwrap(),
        ignored
    );
    assert!(format(&folder, &["--check", "*"]).status.success());
    fs::remove_dir_all(&folder).unwrap();
}

#[test]
fn refuses_a_call_without_targets() {
    let folder = repository("usage");

    assert_eq!(format(&folder, &["--write"]).status.code(), Some(2));
    assert_eq!(
        format(&folder, &["--check", "**/*.md"]).status.code(),
        Some(2)
    );
    fs::remove_dir_all(&folder).unwrap();
}
