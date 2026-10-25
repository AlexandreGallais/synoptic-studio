//! Tests of the records read through the `knowledge` binary (EN-154): the binary found beside, a
//! row's cells read back, a word checked before it enters a statement, and a script standing for
//! `knowledge`, so that no test reads the real records.

use std::fs;
use std::path::Path;

use knowledge_call::{escaped, is_plain, knowledge_beside, rows_by, rows_of, unescaped};

#[test]
fn finds_the_knowledge_binary_beside_another() {
    assert_eq!(
        knowledge_beside(Path::new("/repo/target/release/worktree-new")),
        Path::new("/repo/target/release")
            .join(format!("knowledge{}", std::env::consts::EXE_SUFFIX))
    );
}

#[test]
fn reads_a_rows_cells_back_as_they_were_written() {
    let written = "a\tb\nc\\d\r";

    assert_eq!(escaped(written), "a\\tb\\nc\\\\d\\r");
    assert_eq!(unescaped(&escaped(written)), written);
    // A lone backslash at the end is kept.
    assert_eq!(unescaped("x\\"), "x\\");
    assert_eq!(
        rows_of("kind\tfeature\nEN\tF48\nUS\t\n"),
        [
            vec!["EN".to_owned(), "F48".to_owned()],
            vec!["US".to_owned(), String::new()]
        ]
    );
    assert_eq!(rows_of("kind\n"), Vec::<Vec<String>>::new());
}

#[test]
fn lets_only_an_id_or_a_branch_into_a_statement() {
    for word in ["EN-154", "feature/f49-the-backlog", "F48", "a_b.c"] {
        assert!(is_plain(word), "{word}");
    }
    for word in ["", "x' OR '1'='1", "a b", "a;b", "é"] {
        assert!(!is_plain(word), "{word}");
    }
}

#[cfg(unix)]
#[test]
fn reads_the_rows_a_knowledge_binary_prints_and_its_error_when_it_fails() {
    use std::os::unix::fs::PermissionsExt;

    let folder = std::env::temp_dir().join(format!("knowledge-call-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).unwrap();

    let script = folder.join("knowledge");

    // Stands for `knowledge sql`: rows for a statement naming EN-154, else the error of no records.
    fs::write(
        &script,
        "#!/bin/sh\ncase \"$2\" in *EN-154*) printf 'kind\\ttopic\\nEN\\tthe\\\\tbacklog\\n';; *) echo 'knowledge: AGENT_RECORDS_ROOT is not set' >&2; exit 1;; esac\n",
    )
    .unwrap();
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();

    let found = rows_by(
        &script,
        &folder,
        "SELECT kind, topic FROM stories WHERE id = 'EN-154'",
    );
    let failed = rows_by(&script, &folder, "SELECT 1");
    let missing = rows_by(&folder.join("none"), &folder, "SELECT 1");

    let _ = fs::remove_dir_all(&folder);
    assert_eq!(
        found,
        Ok(vec![vec!["EN".to_owned(), "the\tbacklog".to_owned()]])
    );
    assert_eq!(
        failed,
        Err("knowledge: AGENT_RECORDS_ROOT is not set".to_owned())
    );
    assert!(missing.is_err());
}
