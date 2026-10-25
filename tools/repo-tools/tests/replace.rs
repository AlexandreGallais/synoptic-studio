//! `replace` run on files of a folder of its own, its blocks on its standard input.

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Output, Stdio};

/// A new folder holding `a.md` and `b.md`.
fn folder(name: &str) -> PathBuf {
    let folder = env::temp_dir().join(format!("replace-{}-{name}", std::process::id()));

    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).expect("folder");
    fs::write(folder.join("a.md"), "# A\n\nold a\n").expect("a");
    fs::write(folder.join("b.md"), "# B\n\nold b\n").expect("b");
    folder
}

fn replace(folder: &Path, spec: &str) -> Output {
    let mut child = test_support::command(env!("CARGO_BIN_EXE_replace"))
        .current_dir(folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("replace starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(spec.as_bytes())
        .expect("spec");
    child.wait_with_output().expect("replace ends")
}

fn read(folder: &Path, file: &str) -> String {
    fs::read_to_string(folder.join(file)).expect("file")
}

#[test]
fn edits_two_files_and_says_how_many_edits() {
    let folder = folder("two");
    let output = replace(
        &folder,
        "@@ a.md\n<<<\nold a\n===\nnew a\n>>>\n@@ b.md\n<<<\nold b\n===\nnew b\n>>>\n@@ a.md\n<<<\n# A\n===\n# A, renamed\n>>>\n",
    );
    let (a, b) = (read(&folder, "a.md"), read(&folder, "b.md"));

    fs::remove_dir_all(&folder).expect("cleaned");
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "3 edits in 2 files\n"
    );
    assert_eq!(a, "# A, renamed\n\nnew a\n");
    assert_eq!(b, "# B\n\nnew b\n");
}

#[test]
fn f21_ac2_refuses_the_whole_input_for_a_block_it_cannot_read_and_edits_nothing() {
    let folder = folder("unread");
    // The second block lost its `@@ b.md` line: the script edited a.md and said "1 edits".
    let output = replace(
        &folder,
        "@@ a.md\n<<<\nold a\n===\nnew a\n>>>\n<<<\nold b\n===\nnew b\n>>>\n",
    );
    let a = read(&folder, "a.md");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "block 2, line 7: cannot read `<<<`: each block is `@@ <file>`, `<<<`, old text, `===`, new text, `>>>`, and no text holds a line `<<<` or `>>>`\nnothing written\n"
    );
    assert_eq!(a, "# A\n\nold a\n");
}

#[test]
fn writes_nothing_when_one_old_text_is_missing_and_names_each_file_in_trouble() {
    let folder = folder("missing");
    let output = replace(
        &folder,
        "@@ a.md\n<<<\nold a\n===\nnew a\n>>>\n@@ b.md\n<<<\nnone\n===\nx\n>>>\n@@ c.md\n<<<\nx\n===\ny\n>>>\n",
    );
    let a = read(&folder, "a.md");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.starts_with("b.md: edit 1: found 0 times\nc.md: cannot be read: "),
        "{stderr}"
    );
    assert!(stderr.ends_with("\nnothing written\n"));
    assert_eq!(a, "# A\n\nold a\n");
}

#[test]
fn says_when_it_read_no_block() {
    let folder = folder("none");
    let output = replace(&folder, "\n\n");

    fs::remove_dir_all(&folder).expect("cleaned");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "no edit block read\nnothing written\n"
    );
}

/// `replace` called with arguments, its standard input empty and closed.
fn replace_with(arguments: &[&str]) -> Output {
    test_support::command(env!("CARGO_BIN_EXE_replace"))
        .args(arguments)
        .stdin(Stdio::null())
        .output()
        .expect("replace runs")
}

#[test]
fn f31_ac2_prints_its_usage_for_help_and_exits_0() {
    let output = replace_with(&["--help"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_eq!(output.status.code(), Some(0));
    assert!(
        stdout.starts_with("usage: target/release/replace < <blocks>"),
        "{stdout}"
    );
    assert!(stdout.contains("\n  @@ path/to/file\n  <<<\n"), "{stdout}");
    assert_eq!(output.stderr, b"");
}

#[test]
fn f31_ac2_refuses_an_argument_with_its_usage_and_exits_2() {
    let output = replace_with(&["edits.txt"]);

    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("usage: target/release/replace"));
}

#[test]
fn f31_ac2_ends_at_once_on_a_closed_input_without_an_argument() {
    // The agent's Bash tool gives no terminal: called alone, `replace` reads a closed input and
    // ends at once, saying it read no block (AUD-033).
    let output = replace_with(&[]);

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, b"");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "no edit block read\nnothing written\n"
    );
}
