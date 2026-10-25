//! The `page` command in a fixture repository of its own (EN-105): the page named, what it prints
//! and its exit codes, and an id alone shown by `knowledge show` (EN-152). `f21_ac8_…` and
//! `f48_ac2_…` mark a test proving a feature's criterion.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use page::fallback::{knowledge_beside, shown_by};

/// A fixture repository, removed at the end of the test.
struct Repo(PathBuf);

impl Drop for Repo {
    fn drop(&mut self) {
        // Already gone is as good as removed.
        let _ = fs::remove_dir_all(&self.0);
    }
}

const REFERENCES: &str = "# References\n\n## Hooks\n\n| ID | Reference |\n| --- | --- |\n| `REF-A` | first |\n| `REF-B` | second |\n";

/// The fixture pages, in a folder named after the test.
fn repo(name: &str) -> Repo {
    let root = env::temp_dir().join(format!("page-bin-{}-{name}", std::process::id()));
    let files = [
        ("docs/references.md", REFERENCES),
        (
            "docs/adr/0069-fast.md",
            "# ADR-0069 — Fast\n\nAccepted.\n\n## Context\n\nSlow.\n",
        ),
        ("docs/tools.md", "# F21\n\n## Criteria\n"),
    ];

    let _ = fs::remove_dir_all(&root);
    for (file, text) in files {
        let path = root.join(file);

        fs::create_dir_all(path.parent().expect("a folder")).expect("folders");
        fs::write(path, text).expect("a fixture page");
    }
    Repo(root)
}

/// `page` run in a fixture: no `GIT_*` variable nor records root inherited, since an id alone runs
/// `knowledge`, which runs Git and opens the records (EN-152).
fn page(root: &Path, words: &[&str]) -> Output {
    test_support::command(env!("CARGO_BIN_EXE_page"))
        .current_dir(root)
        .env_remove("AGENT_RECORDS_ROOT")
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
    let output = page(&repo.0, &["docs/tools.md", "F21"]);
    let found = page(&repo.0, &["docs/adr/0069-fast.md", "Context"]);

    // One heading `F21`, one section, its subsection inside it; `Context` too.
    assert_eq!(stdout(&output), "docs/tools.md:1\n# F21\n\n## Criteria\n");
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

    assert_eq!(named.status.code(), Some(1));
    assert_eq!(
        stderr(&named),
        "REF-C: no row starting with it nor heading of it in docs/references.md\n"
    );
    assert_eq!(stdout(&named), "");
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

/// A copy of `page` in a folder of its own, beside a `knowledge` script that shows `NOTE-0035`
/// alone and fails on any other id, as the real binary does on an id it reads nowhere.
#[cfg(unix)]
fn page_beside_knowledge(repo: &Path) -> PathBuf {
    use std::os::unix::fs::PermissionsExt as _;

    let folder = repo.join("bin");
    let page = folder.join("page");
    let knowledge = folder.join("knowledge");

    fs::create_dir_all(&folder).expect("a folder");
    fs::copy(env!("CARGO_BIN_EXE_page"), &page).expect("page copied");
    fs::write(
        &knowledge,
        "#!/bin/sh\n[ \"$1 $2\" = 'show NOTE-0035' ] && { printf '# 0035 — Knowledge\\n\\nMoved.\\n'; exit 0; }\necho \"knowledge: $2: no document of it\" >&2\nexit 1\n",
    )
    .expect("knowledge written");
    fs::set_permissions(&knowledge, fs::Permissions::from_mode(0o755)).expect("executable");
    page
}

#[cfg(unix)]
#[test]
fn f48_ac2_an_id_alone_is_shown_by_knowledge_never_by_a_docs_page() {
    let repo = repo("knowledge");
    let page = page_beside_knowledge(&repo.0);
    let run = |id: &str| {
        test_support::command(&page)
            .current_dir(&repo.0)
            .env_remove("AGENT_RECORDS_ROOT")
            .arg(id)
            .output()
            .expect("page runs")
    };
    let (moved, missing, filed) = (run("NOTE-0035"), run("ADR-0001"), run("ADR-0069"));

    // Its document is in the records: knowledge's text, exit 0.
    assert!(moved.status.success(), "{}", stderr(&moved));
    assert_eq!(stdout(&moved), "# 0035 — Knowledge\n\nMoved.\n");
    // No document: page's own message, knowledge's left out.
    assert_eq!(missing.status.code(), Some(1));
    assert_eq!(
        stderr(&missing),
        "ADR-0001: no document of it in the records (knowledge show ADR-0001)\n"
    );
    // A former docs page of the id is never read: the records alone are asked.
    assert_eq!(filed.status.code(), Some(1));
    assert_eq!(stdout(&filed), "");
}

#[test]
fn f48_ac2_finds_knowledge_beside_page() {
    assert_eq!(
        knowledge_beside(Path::new("target/release/page")),
        PathBuf::from(format!(
            "target/release/knowledge{}",
            std::env::consts::EXE_SUFFIX
        ))
    );
    // A binary that cannot start shows nothing.
    assert_eq!(shown_by(Path::new("target/none/knowledge"), "EN-152"), None);
}
