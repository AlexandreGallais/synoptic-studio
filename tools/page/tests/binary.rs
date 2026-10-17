//! The `page` command in a fixture repository of its own (EN-105): the page named or found from
//! the id, what it prints and its exit codes. It runs no `git`. `f21_ac8_…` marks a test proving
//! the feature's criterion.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A fixture repository, removed at the end of the test.
struct Repo(PathBuf);

impl Drop for Repo {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.0);
    }
}

const REFERENCES: &str = "# References\n\n## Hooks\n\n| ID | Reference |\n| --- | --- |\n| `REF-A` | first |\n| `REF-B` | second |\n";

const STORY: &str =
    "---\nid: EN-105\n---\n\n# E17 · F21 · EN-105 — One row\n\nWhy.\n\n## Criteria\n\n- one\n";

/// The fixture pages, in a folder named after the test.
fn repo(name: &str) -> Repo {
    let root = env::temp_dir().join(format!("page-bin-{}-{name}", std::process::id()));
    let files = [
        ("docs/references.md", REFERENCES),
        (
            "docs/adr/0069-fast.md",
            "# ADR-0069 — Fast\n\nAccepted.\n\n## Context\n\nSlow.\n",
        ),
        ("docs/derivations/bit.md", "# DERIV-bit — A bit\n\nSteps.\n"),
        ("docs/backlog/stories/E17-F21-EN-105-one-row.md", STORY),
        (
            "docs/backlog/stories/E17-F21-EN-1050-other.md",
            "# EN-1050\n",
        ),
        (
            "docs/backlog/features/E17-F21-tools.md",
            "# F21\n\n## Criteria\n",
        ),
        ("docs/backlog/epics/E17-fast.md", "# E17\n"),
        ("docs/backlog/epics/notes.txt", "E17-"),
    ];

    let _ = fs::remove_dir_all(&root);
    for (file, text) in files {
        let path = root.join(file);

        fs::create_dir_all(path.parent().expect("a folder")).expect("folders");
        fs::write(path, text).expect("a fixture page");
    }
    Repo(root)
}

fn page(root: &Path, words: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_page"))
        .current_dir(root)
        .args(words)
        .output()
        .expect("page runs")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn f21_ac8_a_row_of_a_named_page() {
    let repo = repo("row");
    let output = page(&repo.0, &["docs/references.md", "REF-B"]);

    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        "| ID | Reference |\n| `REF-B` | second |\n"
    );
}

#[test]
fn f21_ac8_a_section_of_a_named_page_with_its_line() {
    let repo = repo("section");
    let output = page(&repo.0, &["docs/references.md", "Hooks"]);

    assert!(output.status.success());
    assert_eq!(
        stdout(&output),
        format!("docs/references.md:3\n{}", &REFERENCES[14..])
    );
}

#[test]
fn f21_ac8_two_sections_of_one_heading_both_printed() {
    let repo = repo("twice");
    let output = page(&repo.0, &["docs/backlog/features/E17-F21-tools.md", "F21"]);
    let found = page(&repo.0, &["docs/adr/0069-fast.md", "Context"]);

    // One heading `F21`, one section, its subsection inside it; `Context` too.
    assert_eq!(
        stdout(&output),
        "docs/backlog/features/E17-F21-tools.md:1\n# F21\n\n## Criteria\n"
    );
    assert_eq!(
        stdout(&found),
        "docs/adr/0069-fast.md:5\n## Context\n\nSlow.\n"
    );

    fs::write(repo.0.join("twice.md"), "## A\n\none\n\n## A\n\ntwo\n").expect("a page");

    let both = page(&repo.0, &["twice.md", "A"]);

    assert_eq!(
        stdout(&both),
        "twice.md:1\n## A\n\none\n\ntwice.md:5\n## A\n\ntwo\n"
    );
}

#[test]
fn f21_ac8_nothing_found_exits_1_naming_the_file() {
    let repo = repo("missing");
    let named = page(&repo.0, &["docs/references.md", "REF-C"]);
    let found = page(&repo.0, &["REF-C"]);

    assert_eq!(named.status.code(), Some(1));
    assert_eq!(
        stderr(&named),
        "REF-C: no row starting with it nor heading of it in docs/references.md\n"
    );
    assert_eq!(stdout(&named), "");
    assert_eq!(found.status.code(), Some(1));
    assert_eq!(
        stderr(&found),
        "REF-C: no row starting with it in docs/references.md\n"
    );
}

#[test]
fn f21_ac8_a_ref_id_alone_is_found_in_the_references() {
    let repo = repo("ref");
    let output = page(&repo.0, &["REF-A"]);

    assert!(output.status.success());
    assert_eq!(stdout(&output), "| ID | Reference |\n| `REF-A` | first |\n");
}

#[test]
fn f21_ac8_an_adr_a_derivation_and_a_backlog_item_give_their_first_section() {
    let repo = repo("files");
    let cases = [
        (
            "ADR-0069",
            "docs/adr/0069-fast.md:1\n# ADR-0069 — Fast\n\nAccepted.\n",
        ),
        (
            "DERIV-bit",
            "docs/derivations/bit.md:1\n# DERIV-bit — A bit\n\nSteps.\n",
        ),
        (
            "EN-105",
            "docs/backlog/stories/E17-F21-EN-105-one-row.md:1\n---\nid: EN-105\n---\n\n# E17 · F21 · EN-105 — One row\n\nWhy.\n",
        ),
        ("F21", "docs/backlog/features/E17-F21-tools.md:1\n# F21\n"),
        ("E17", "docs/backlog/epics/E17-fast.md:1\n# E17\n"),
    ];

    for (id, expected) in cases {
        let output = page(&repo.0, &[id]);

        assert!(output.status.success(), "{id}: {}", stderr(&output));
        assert_eq!(stdout(&output), expected, "{id}");
    }
}

#[test]
fn an_id_without_its_file_exits_1_naming_where_it_was_looked_for() {
    let repo = repo("nofile");
    let cases = [
        ("ADR-0001", "ADR-0001: no file docs/adr/0001-*.md\n"),
        (
            "DERIV-none",
            "DERIV-none: no file docs/derivations/none.md\n",
        ),
        (
            "US-001",
            "US-001: no file in docs/backlog/stories named with it\n",
        ),
        (
            "F99",
            "F99: no file in docs/backlog/features named with it\n",
        ),
        ("E99", "E99: no file in docs/backlog/epics named with it\n"),
    ];

    for (id, expected) in cases {
        let output = page(&repo.0, &[id]);

        assert_eq!(output.status.code(), Some(1), "{id}");
        assert_eq!(stderr(&output), expected);
    }
}

#[test]
fn a_word_of_no_kind_asks_for_the_page() {
    let repo = repo("kind");

    for word in ["Hooks", "en-105", "EN-", "-1", "E", "F2x", "Ex", ""] {
        let output = page(&repo.0, &[word]);

        assert_eq!(output.status.code(), Some(1), "{word}");
        assert!(stderr(&output).contains("name the page"), "{word}");
    }
}

#[test]
fn a_wrong_call_or_a_missing_page_exits_2() {
    let repo = repo("usage");
    let none = page(&repo.0, &[]);
    let three = page(&repo.0, &["a", "b", "c"]);
    let missing = page(&repo.0, &["docs/none.md", "REF-A"]);

    assert_eq!(none.status.code(), Some(2));
    assert!(stderr(&none).starts_with("usage: page [<file>] <id or heading>"));
    assert_eq!(three.status.code(), Some(2));
    assert_eq!(missing.status.code(), Some(2));
    assert!(stderr(&missing).starts_with("docs/none.md: "));
}

#[test]
fn a_missing_references_page_exits_2() {
    let repo = repo("noref");

    fs::remove_file(repo.0.join("docs/references.md")).expect("removed");

    let output = page(&repo.0, &["REF-A"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(stderr(&output).starts_with("docs/references.md: "));
}

#[test]
fn a_missing_adr_folder_finds_no_file() {
    let repo = repo("noadr");

    fs::remove_dir_all(repo.0.join("docs/adr")).expect("removed");

    let output = page(&repo.0, &["ADR-0069"]);

    assert_eq!(output.status.code(), Some(1));
}
