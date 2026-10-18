//! Tests of `backlog-index --check` on a backlog of its own, never the real one (AUD-020):
//! `[F18.AC5]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A temporary backlog, removed when the test ends, even on a failure.
struct Backlog(PathBuf);

impl Drop for Backlog {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A backlog of one epic, one feature and two stories, its index written by the tool.
fn backlog(name: &str) -> Backlog {
    let root = std::env::temp_dir().join(format!("backlog-index-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);

    for folder in ["epics", "features", "stories"] {
        fs::create_dir_all(root.join(folder)).unwrap();
    }

    let write = |path: &str, text: &str| fs::write(root.join(path), text).unwrap();
    write(
        "epics/E01-an-epic.md",
        "---\nid: E01\ntitle: An epic\nstatus: in-progress\n---\n\n# E01\n\n| ID | Feature |\n| --- | --- |\n| [F01](../features/E01-F01-a-feature.md) | A feature |\n",
    );
    write(
        "features/E01-F01-a-feature.md",
        "---\nid: F01\nepic: E01\ntitle: A feature\nstatus: in-progress\n---\n\n# F01\n\n| ID |\n| --- |\n| [SP-001](../stories/E01-F01-SP-001-research.md) |\n| [US-001](../stories/E01-F01-US-001-a-story.md) |\n",
    );
    write(
        "stories/E01-F01-SP-001-research.md",
        "---\nid: SP-001\nepic: E01\nfeature: F01\ntitle: Research\nstatus: done\n---\n",
    );
    write(
        "stories/E01-F01-US-001-a-story.md",
        "---\nid: US-001\nepic: E01\nfeature: F01\ntitle: A story\nstatus: ready\n---\n",
    );

    let backlog = Backlog(root);
    assert!(run(&backlog, &[]).status.success());
    backlog
}

/// The tool run on the backlog, with the extra arguments.
fn run(backlog: &Backlog, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_backlog-index"))
        .arg(&backlog.0)
        .args(arguments)
        .output()
        .unwrap()
}

fn index(root: &Path) -> String {
    fs::read_to_string(root.join("stories/README.md")).unwrap()
}

#[test]
fn f18_ac5_check_accepts_the_index_the_story_files_give() {
    let backlog = backlog("same");
    let output = run(&backlog, &["--check"]);

    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
}

#[test]
fn f18_ac5_check_refuses_a_title_changed_in_its_file_and_names_the_row() {
    let backlog = backlog("title");
    let before = index(&backlog.0);
    let story = backlog.0.join("stories/E01-F01-US-001-a-story.md");
    let text = fs::read_to_string(&story).unwrap();
    fs::write(
        &story,
        text.replace("title: A story", "title: Another story"),
    )
    .unwrap();

    let output = run(&backlog, &["--check"]);
    let message = String::from_utf8_lossy(&output.stderr);

    // Rows after the heading: 1 the heading, 2 the header, 3 SP-001, 4 US-001.
    assert_eq!(output.status.code(), Some(1));
    assert!(message.contains("at row 4"), "{message}");
    assert!(message.contains("Another story"), "{message}");
    assert!(message.contains("npm run backlog:index"), "{message}");
    assert_eq!(index(&backlog.0), before);
}
