//! Tests of `knowledge` as a binary (EN-151): the file it opens, named after the repository's
//! `origin`, in a records folder of the temporary folder. `AGENT_RECORDS_ROOT` is always set to that
//! folder or removed, and `HOME` pointed there too, so that the real records folder is never read
//! (`[F48.AC4]`).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use knowledge::store::{SCHEMA_VERSION, open_store, write};

/// The usage the binary prints for a wrong call.
const USAGE: &str = "Usage: target/release/knowledge init | path | search <words> [<filters>] | show <id> [<heading>] [--all] | list [<filters>] | sql \"<statement>\" [--purpose \"<text>\"] | add <kind> (refused: knowledge sql \"INSERT …\") | sync | snapshot | export-criteria <file or -> | export-data [--check] | check-ids <paths> | branch-state <branch> merged|dropped | import-memories, import-metrics, import-archives, import-transcripts <folder> | import-library, import-saves <file>; the filters: --kind, --epic, --feature, --status, --since, --branch, --state <value>, --all";

/// A folder of its own: `records/` for the root, `repo/` a repository whose `origin` names
/// `some-project`, `home/` for `HOME`.
fn place(name: &str) -> PathBuf {
    let folder =
        std::env::temp_dir().join(format!("knowledge-binary-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    fs::create_dir_all(folder.join("records")).unwrap();
    fs::create_dir_all(folder.join("home")).unwrap();
    fs::create_dir_all(folder.join("repo")).unwrap();
    git(&folder.join("repo"), &["init", "-q", "-b", "develop"]);
    git(
        &folder.join("repo"),
        &[
            "remote",
            "add",
            "origin",
            "https://example.org/owner/some-project.git",
        ],
    );
    folder
}

/// Runs Git in a folder, no `GIT_*` variable inherited.
fn git(cwd: &Path, words: &[&str]) {
    let status = test_support::command("git")
        .args(words)
        .current_dir(cwd)
        .status()
        .unwrap();

    assert!(status.success(), "git {words:?}");
}

/// The binary, run in a folder with `HOME` in the place and neither the records root nor the
/// synced folder inherited.
fn knowledge(place: &Path, cwd: &Path) -> Command {
    let mut command = test_support::command(env!("CARGO_BIN_EXE_knowledge"));

    command
        .current_dir(cwd)
        .env("HOME", place.join("home"))
        .env_remove("AGENT_RECORDS_ROOT")
        .env_remove("AGENT_KNOWLEDGE_SNAPSHOT");
    command
}

#[test]
fn f48_ac4_runs_the_binary_without_the_real_records_root_nor_the_real_synced_folder() {
    let place = place("variables");
    let command = knowledge(&place, &place.join("repo"));
    let removed: Vec<String> = command
        .get_envs()
        .filter(|(_, value)| value.is_none())
        .map(|(name, _)| name.to_string_lossy().into_owned())
        .filter(|name| name.starts_with("AGENT_"))
        .collect();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(removed, ["AGENT_KNOWLEDGE_SNAPSHOT", "AGENT_RECORDS_ROOT"]);
}

/// The output's standard output and error, as text.
fn text(output: &Output) -> (String, String) {
    (
        String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    )
}

#[test]
fn f48_ac1_makes_records_sqlite_in_the_folder_named_after_origin() {
    let place = place("made");
    let output = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();
    let file = place.join("records/some-project.sqlite");
    let made = file.is_file();
    let (stdout, _) = text(&output);

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success());
    assert!(made);
    assert_eq!(stdout, format!("{}, schema 6", file.display()));
}

#[test]
fn f48_ac1_opens_the_same_file_from_a_linked_worktree() {
    let place = place("worktree");
    let repo = place.join("repo");

    git(
        &repo,
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.org",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "start",
        ],
    );
    git(
        &repo,
        &["worktree", "add", "-q", "-b", "chore/en-151", "../linked"],
    );

    let path = |cwd: &Path| {
        let output = knowledge(&place, cwd)
            .arg("path")
            .env("AGENT_RECORDS_ROOT", place.join("records"))
            .output()
            .unwrap();
        text(&output).0
    };
    let (main, linked) = (path(&repo), path(&place.join("linked")));

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        main,
        place
            .join("records/some-project.sqlite")
            .display()
            .to_string()
    );
    assert_eq!(linked, main);
}

#[test]
fn f48_ac1_fails_without_the_variable_and_says_what_to_set() {
    let place = place("unset");
    let unset = knowledge(&place, &place.join("repo"))
        .arg("init")
        .output()
        .unwrap();
    let empty = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", "")
        .output()
        .unwrap();
    let nothing = fs::read_dir(place.join("records")).unwrap().count();

    let _ = fs::remove_dir_all(&place);
    for output in [unset, empty] {
        assert_eq!(output.status.code(), Some(1));
        assert_eq!(
            text(&output).1,
            "knowledge: AGENT_RECORDS_ROOT is not set: name the folder of the records in the env of ~/.claude/settings.json."
        );
    }
    assert_eq!(nothing, 0);
}

#[test]
fn f48_ac1_fails_on_a_records_root_that_does_not_exist_and_makes_none() {
    let place = place("missing-root");
    let root = place.join("mistyped");
    let output = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", &root)
        .output()
        .unwrap();
    let made = root.exists();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        text(&output).1,
        format!(
            "knowledge: The records' folder {} does not exist: create it, or correct AGENT_RECORDS_ROOT.",
            root.display()
        )
    );
    assert!(!made);
}

#[test]
fn f48_ac1_reads_a_newer_file_and_says_it_is_read_only() {
    let place = place("newer");
    let file = place.join("records/some-project.sqlite");

    open_store(&file)
        .unwrap()
        .pragma_update(None, "user_version", SCHEMA_VERSION + 1)
        .unwrap();

    let output = knowledge(&place, &place.join("repo"))
        .arg("init")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert!(output.status.success());
    assert_eq!(
        text(&output).0,
        format!(
            "{}, schema 7, newer than this binary's 6: read only",
            file.display()
        )
    );
}

#[test]
fn prints_its_usage_for_an_unknown_command() {
    let place = place("usage");
    let output = knowledge(&place, &place.join("repo"))
        .arg("drop")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(text(&output).1, format!("knowledge: {USAGE}"));
}

/// Commits nothing in a repository, as a test's author.
fn commit(repo: &Path, message: &str) {
    git(
        repo,
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.org",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            message,
        ],
    );
}

/// A place whose repository has one commit on `develop`, then checks out a new branch.
fn on_branch(name: &str, branch: &str) -> PathBuf {
    let place = place(name);

    commit(&place.join("repo"), "start");
    git(&place.join("repo"), &["checkout", "-q", "-b", branch]);
    place
}

/// The binary run in the place's repository, its records root set to the place's.
fn run(place: &Path, words: &[&str]) -> Output {
    knowledge(place, &place.join("repo"))
        .args(words)
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap()
}

/// The place's records file.
fn records(place: &Path) -> PathBuf {
    place.join("records/some-project.sqlite")
}

/// Writes rows into the place's records, in one write.
fn seed(place: &Path, statements: &str) {
    let database = open_store(&records(place)).unwrap();

    write(&database, |transaction| {
        transaction.execute_batch(statements)
    })
    .unwrap();
}

/// A story of tasks and an item, a decision and a note of the epic E49.
const RECORDS: &str = "
  INSERT INTO epics (id, title, status, branch, state) VALUES
    ('E49', 'Keep', 'in-progress', 'develop', 'merged');
  INSERT INTO stories (id, epic, feature, kind, title, status, summary, criteria, branch, state) VALUES
    ('EN-900', 'E49', 'F48', 'EN', 'Add, search, show and list', 'ready', 'As ADR-0900 says.',
      '- A busy timeout waits for the other writer.', 'chore/en-900-read', 'draft');
  INSERT INTO tasks (story, number, title, done) VALUES ('EN-900', 1, 'The tests', 0);
  INSERT INTO story_items (story, section, n, kind, item) VALUES
    ('EN-900', 'Findings', 1, 'bullet', 'One waits.');
  INSERT INTO decisions (id, title, status, date, principle) VALUES
    ('ADR-0900', 'One file', 'accepted', '2026-10-20', 'One file for all.');
  INSERT INTO notes (id, kind, title, status, date) VALUES
    ('NOTE-0900', 'note', 'The wait of a writer', 'done', '2026-10-24');
";

#[test]
fn f48_ac2_shows_searches_and_lists_the_records_from_their_tables() {
    let place = on_branch("read", "chore/en-900-read");

    seed(&place, RECORDS);

    let shown = run(&place, &["show", "EN-900"]);
    let section = run(&place, &["show", "EN-900", "Tasks"]);
    let found = run(&place, &["search", "busy", "timeout", "--kind", "story"]);
    let other_kind = run(&place, &["search", "busy", "--kind", "note"]);
    let listed = run(&place, &["list", "--epic", "E49"]);
    let decision = run(&place, &["show", "ADR-0900"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&shown).0,
        "# EN-900 — Add, search, show and list\n\nstory EN · epic E49 · feature F48 · status ready · draft on chore/en-900-read\n\n## Summary\n\nAs ADR-0900 says.\n\n## Acceptance criteria\n\n- A busy timeout waits for the other writer.\n\n## Tasks\n\n- [ ] T1 — The tests\n\n## Findings\n\n- One waits."
    );
    // `## Tasks` on line 13: the title, the facts and two parts of three lines and a blank one.
    assert_eq!(
        text(&section).0,
        "EN-900:13\n## Tasks\n\n- [ ] T1 — The tests"
    );
    assert_eq!(
        text(&found).0,
        "EN-900 › story\n    Add, search, show and list As ADR-0900 says. - A [busy] [timeout] waits for the other writer."
    );
    assert_eq!(text(&other_kind), (String::new(), String::new()));
    assert_eq!(
        text(&listed).0,
        "E49 [epic, in-progress] Keep — merged on develop\nEN-900 [story, ready] Add, search, show and list — draft on chore/en-900-read"
    );
    assert_eq!(
        text(&decision).0,
        "# ADR-0900 — One file\n\ndecision · status accepted · date 2026-10-20\n\n## Principle\n\nOne file for all."
    );
}

/// Merges a branch into the one checked out by a merge commit, as a pull request does.
fn merge(repo: &Path, branch: &str) {
    git(
        repo,
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.org",
            "-c",
            "commit.gpgsign=false",
            "merge",
            "-q",
            "--no-ff",
            "-m",
            "Merge",
            branch,
        ],
    );
}

#[test]
fn f48_ac2_marks_merged_the_rows_of_the_branches_merged_into_develop_and_no_other() {
    let place = on_branch("sync", "feature/a");
    let repo = place.join("repo");

    // feature/a's tip is develop's, no commit of its own: not merged. chore/c is one commit
    // ahead: not merged. chore/m's commit is merged into develop by a merge commit: merged.
    git(&repo, &["checkout", "-q", "-b", "chore/c"]);
    commit(&repo, "ahead");
    git(&repo, &["checkout", "-q", "-b", "chore/m", "develop"]);
    commit(&repo, "merged");
    git(&repo, &["checkout", "-q", "develop"]);
    merge(&repo, "chore/m");
    git(&repo, &["checkout", "-q", "chore/c"]);

    // The memories' documents of the schemas before 6, a draft of each branch.
    let document = |id: &str, branch: &str| {
        format!(
            "INSERT INTO documents (id, kind, title, epic, feature, status, date, branch, state, private, path)
            VALUES ('{id}', 'memory', '{id}', '', '', '', '2026-10-25', '{branch}', 'draft', 1, '');"
        )
    };

    seed(
        &place,
        &[
            document("MEMORY-1", "feature/a"),
            document("MEMORY-2", "develop"),
            document("MEMORY-3", "chore/c"),
            document("MEMORY-5", "chore/m"),
        ]
        .concat(),
    );

    let synced = run(&place, &["sync"]);
    let database = open_store(&records(&place)).unwrap();
    let states: Vec<String> = database
        .prepare("SELECT id || ' ' || state FROM documents ORDER BY id")
        .unwrap()
        .query_map([], |row| row.get(0))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();

    seed(&place, &document("MEMORY-4", "develop"));

    let one = run(&place, &["sync"]);

    let _ = fs::remove_dir_all(&place);
    // develop's row and chore/m's; feature/a, at develop's start, merged nothing of its own.
    assert_eq!(
        text(&synced).0,
        "2 rows marked merged, of the 2 branches merged into develop"
    );
    assert_eq!(
        states,
        [
            "MEMORY-1 draft",
            "MEMORY-2 merged",
            "MEMORY-3 draft",
            "MEMORY-5 merged"
        ]
    );
    // The row added on develop.
    assert_eq!(
        text(&one).0,
        "1 row marked merged, of the 2 branches merged into develop"
    );
}

#[test]
fn f48_ac2_fails_aloud_on_a_kind_added_or_an_id_it_reads_nowhere() {
    let place = on_branch("wrong", "chore/en-900-wrong");
    let failures = [
        run(&place, &["show", "EN-999"]),
        run(&place, &["show", "EN-999", "Tasks"]),
        run(&place, &["list", "--colour", "red"]),
        run(&place, &["sync", "--kind", "story"]),
    ]
    .map(|output| (output.status.code(), text(&output).1));
    // No record is a document: every kind is refused, named by its table.
    let added = [
        run(&place, &["add", "decision", "-"]),
        run(&place, &["add", "incident"]),
        run(&place, &["add", "anything", "file.md"]),
    ]
    .map(|output| (output.status.code(), text(&output).1));

    seed(&place, RECORDS);

    let no_heading = run(&place, &["show", "EN-900", "Missing"]);
    let usages = [
        run(&place, &["add"]),
        run(&place, &["search"]),
        run(&place, &["search", "--kind", "story"]),
        run(&place, &["show"]),
        run(&place, &["show", "EN-900", "Tasks", "more"]),
    ]
    .map(|output| (output.status.code(), text(&output).1));

    // A repository without develop: sync cannot ask Git which branches are merged into it.
    git(&place.join("repo"), &["branch", "-m", "develop", "trunk"]);

    let no_develop = run(&place, &["sync"]);

    let _ = fs::remove_dir_all(&place);

    assert_eq!(
        failures,
        [
            "knowledge: EN-999: no record of it in the records".to_owned(),
            "knowledge: EN-999: no record of it in the records".to_owned(),
            "knowledge: --colour: no such filter".to_owned(),
            format!("knowledge: {USAGE}"),
        ]
        .map(|message| (Some(1), message))
    );
    assert_eq!(
        added,
        [
            "knowledge: decision: a row of the table decisions, its text in rows of decision_items, not a document: target/release/knowledge sql \"INSERT INTO decisions (…) VALUES (…)\", then one per item, \"INSERT INTO decision_items (…) VALUES (…)\"",
            "knowledge: incident: a row of the table incidents, not a document: target/release/knowledge sql \"INSERT INTO incidents (…) VALUES (…)\"",
            "knowledge: anything: no document is added any more, every record is a row of its table: target/release/knowledge sql \"SELECT name, purpose FROM tables_registry\" names them, then knowledge sql \"INSERT INTO <table> …\"",
        ]
        .map(|message| (Some(1), message.to_owned()))
    );
    assert_eq!(
        (no_heading.status.code(), text(&no_heading).1),
        (
            Some(1),
            "knowledge: EN-900: no heading Missing in it".to_owned()
        )
    );
    assert_eq!(
        usages,
        [(); 5].map(|()| (Some(1), format!("knowledge: {USAGE}")))
    );
    assert_eq!(no_develop.status.code(), Some(1));
    assert!(
        text(&no_develop).1.ends_with(
            "knowledge: git branch --merged develop --format=%(refname:short) %(objectname) failed in ."
        ),
        "{}",
        text(&no_develop).1
    );
}

#[test]
fn f48_ac1_names_the_file_after_the_main_checkouts_folder_without_a_remote() {
    let place = place("no-remote");
    let repo = place.join("repo");

    git(&repo, &["remote", "remove", "origin"]);

    let output = knowledge(&place, &repo)
        .arg("path")
        .env("AGENT_RECORDS_ROOT", place.join("records"))
        .output()
        .unwrap();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(
        text(&output).0,
        place.join("records/repo.sqlite").display().to_string()
    );
}

/// One epic, one done feature of two criteria and one story, as the trunk holds them.
const BACKLOG: &str = "
  INSERT INTO epics (id, title, status, branch, state) VALUES ('E49', 'Keep', 'in-progress', 'develop', 'merged');
  INSERT INTO features (id, epic, title, status, branch, state) VALUES
    ('F48', 'E49', 'The database', 'done', 'develop', 'merged');
  INSERT INTO criteria (id, feature, number, text) VALUES
    ('F48.AC1', 'F48', 1, 'One file.'), ('F48.AC2', 'F48', 2, 'A snapshot.');
  INSERT INTO stories (id, epic, feature, kind, title, status, branch, state) VALUES
    ('EN-151', 'E49', 'F48', 'EN', 'The tables', 'done', 'develop', 'merged');
";

#[test]
fn reads_and_writes_the_backlog_by_sql_and_exports_its_criteria() {
    let place = on_branch("backlog", "chore/en-154-x");

    seed(&place, BACKLOG);

    let read = run(&place, &["sql", "SELECT id, status FROM stories"]);
    let created = run(
        &place,
        &[
            "sql",
            "CREATE TABLE ideas (id TEXT, branch TEXT NOT NULL DEFAULT '', state TEXT NOT NULL DEFAULT 'draft')",
            "--purpose",
            "a note",
        ],
    );
    let inserted = run(&place, &["sql", "INSERT INTO ideas (id) VALUES ('a')"]);
    let refused = run(&place, &["sql", "DROP TABLE stories"]);
    let purpose_on_a_reading = run(&place, &["sql", "SELECT 1", "--purpose", "x"]);
    let wrong_flag = run(&place, &["sql", "SELECT 1", "--kind", "x"]);
    let merged = run(&place, &["branch-state", "chore/en-154-x", "merged"]);
    let dropped = run(&place, &["branch-state", "chore/en-154-x", "dropped"]);
    let wrong_state = run(&place, &["branch-state", "chore/en-154-x", "gone"]);
    let notes = run(&place, &["sql", "SELECT id, branch, state FROM ideas"]);
    let printed = run(&place, &["export-criteria", "-"]);
    let written = run(&place, &["export-criteria", "criteria.json"]);
    let file = fs::read_to_string(place.join("repo/criteria.json")).unwrap_or_default();

    let _ = fs::remove_dir_all(&place);
    assert_eq!(text(&read).0, "id\tstatus\nEN-151\tdone");
    assert_eq!(text(&created).0, "0 rows changed, ideas registered");
    assert_eq!(text(&inserted).0, "1 row changed");
    assert_eq!(
        text(&refused).1,
        "knowledge: refused: DROP TABLE stories: a core table of the records, kept by the tool"
    );
    assert_eq!(
        text(&purpose_on_a_reading).1,
        "knowledge: --purpose names a table a statement creates"
    );
    assert_eq!(text(&wrong_flag).1, format!("knowledge: {USAGE}"));
    // The note's draft merged; dropping the branch after leaves its merged rows alone.
    assert_eq!(text(&merged).0, "1 row of chore/en-154-x marked merged");
    assert_eq!(text(&dropped).0, "0 rows of chore/en-154-x marked dropped");
    assert_eq!(text(&wrong_state).1, format!("knowledge: {USAGE}"));
    assert_eq!(
        text(&notes).0,
        "id\tbranch\tstate\na\tchore/en-154-x\tmerged"
    );
    assert_eq!(
        text(&printed).0,
        "[\n  { \"id\": \"F48.AC1\" },\n  { \"id\": \"F48.AC2\" }\n]"
    );
    assert_eq!(text(&written).0, "criteria.json written: 2 criteria");
    assert_eq!(file, format!("{}\n", text(&printed).0));
}

#[test]
fn checks_the_ids_a_page_cites_against_every_table_of_ids() {
    let place = on_branch("ids", "chore/en-154-y");

    seed(&place, BACKLOG);
    seed(
        &place,
        "INSERT INTO decisions (id, title, status, date, principle) VALUES
          ('ADR-0081', 'Private records', 'accepted', '2026-10-15', 'Outside.');
        INSERT INTO rules (id, page, section, name, rule, example, position) VALUES
          ('RULE-shapes-box-wrap', 'shapes', 'RULE-shapes-box', 'Wrap', 'At spaces.', 'a b', 1);",
    );
    fs::create_dir_all(place.join("repo/docs")).unwrap();
    fs::write(
        place.join("repo/docs/page.md"),
        "EN-151 and EN-999.\nADR-0081, ADR-9999, RULE-shapes-box and RULE-shapes-nope.\n",
    )
    .unwrap();
    // A test's made-up id is not read.
    fs::write(place.join("repo/docs/page.test.ts"), "// EN-998\n").unwrap();

    let missing = run(&place, &["check-ids", "docs"]);

    fs::write(
        place.join("repo/docs/page.md"),
        "EN-151, F48 and ADR-0081.\n",
    )
    .unwrap();

    let found = run(&place, &["check-ids", "docs"]);
    let nowhere = run(&place, &["check-ids", "none"]);
    let gone = run(&place, &["link-ids", "docs"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(missing.status.code(), Some(1));
    assert_eq!(
        text(&missing).1,
        "knowledge: docs/page.md:1: EN-999 is not in the records\ndocs/page.md:2: ADR-9999 is not in the records\ndocs/page.md:2: RULE-shapes-nope is not in the records"
    );
    assert_eq!(
        text(&found).0,
        "every id cited is in the records, 1 file read"
    );
    assert_eq!(text(&nowhere).1, "knowledge: none: no such file or folder");
    // The one-shot rewrite of links is gone with the files it rewrote.
    assert_eq!(text(&gone).1, format!("knowledge: {USAGE}"));
}

#[test]
fn imports_the_private_records_by_their_commands_a_missing_source_refused() {
    let place = on_branch("imports", "chore/en-155-x");
    let repo = place.join("repo");

    fs::create_dir_all(repo.join("memory")).unwrap();
    fs::write(
        repo.join("memory/feedback-x.md"),
        "---\nname: feedback-x\ndescription: Short replies\n---\n\nBe terse.\n",
    )
    .unwrap();
    fs::write(repo.join("last-save.txt"), "2026-10-25T11:32:57Z\n").unwrap();

    let imported = run(&place, &["import-memories", "memory"]);
    let again = run(&place, &["import-memories", "memory"]);
    let saves = run(&place, &["import-saves", "last-save.txt"]);
    let shown = run(&place, &["show", "MEMORY-feedback-x", "Text"]);
    let no_folder = run(&place, &["import-metrics", "nowhere"]);
    let no_file = run(&place, &["import-library", "memory"]);
    let itself = run(
        &place,
        &["import-library", "../records/some-project.sqlite"],
    );
    let two_words = run(&place, &["import-memories", "a", "b"]);
    // The one-shot importers of the Markdown are gone with it.
    let gone = run(&place, &["import-decisions", "memory"]);

    let _ = fs::remove_dir_all(&place);
    assert_eq!(text(&imported).0, "1 row imported from memory");
    assert_eq!(text(&again), text(&imported));
    assert_eq!(text(&saves).0, "1 row imported from last-save.txt");
    // The memory's file whole, its front matter in.
    assert_eq!(
        text(&shown).0,
        "MEMORY-feedback-x:5\n## Text\n\n---\nname: feedback-x\ndescription: Short replies\n---\n\nBe terse."
    );
    assert_eq!(text(&no_folder).1, "knowledge: nowhere: no such folder");
    assert_eq!(text(&no_file).1, "knowledge: memory: no such file");
    assert_eq!(
        text(&itself).1,
        "knowledge: ../records/some-project.sqlite: the records themselves, nothing to import"
    );
    assert_eq!(text(&two_words).1, format!("knowledge: {USAGE}"));
    assert_eq!(text(&gone).1, format!("knowledge: {USAGE}"));
}

#[test]
fn writes_the_data_files_the_ci_reads_and_names_one_that_differs_from_the_records() {
    let place = on_branch("data", "chore/aud-054-x");

    fs::create_dir_all(place.join("repo/tests/data")).unwrap();
    // Without the tables, the export names the file it cannot write.
    let missing = run(&place, &["export-data"]);

    run(
        &place,
        &[
            "sql",
            "CREATE TABLE lint_exceptions (id INTEGER PRIMARY KEY, file TEXT, item TEXT, rule TEXT, reason TEXT)",
            "--purpose",
            "exceptions",
        ],
    );
    run(
        &place,
        &[
            "sql",
            "CREATE TABLE dependencies (project TEXT, name TEXT, kind TEXT, version TEXT, purpose TEXT)",
            "--purpose",
            "dependencies",
        ],
    );
    run(
        &place,
        &[
            "sql",
            "INSERT INTO lint_exceptions (file, item, rule, reason) VALUES ('a.ts', 'f', 'x', 'why \"so\"')",
        ],
    );
    run(
        &place,
        &[
            "sql",
            "INSERT INTO dependencies VALUES ('root', 'vite', 'npm', '8.3.4', 'the server'), ('root', 'regex', 'crate', '1.13.1', 'patterns')",
        ],
    );

    let written = run(&place, &["export-data"]);
    let agrees = run(&place, &["export-data", "--check"]);

    fs::write(place.join("repo/tests/data/dependencies.json"), "[]\n").unwrap();

    let differs = run(&place, &["export-data", "--check"]);
    let wrong = run(&place, &["export-data", "--all"]);
    let exceptions =
        fs::read_to_string(place.join("repo/tests/data/lint-exceptions.json")).unwrap_or_default();

    let _ = fs::remove_dir_all(&place);
    assert!(
        text(&missing)
            .1
            .starts_with("knowledge: tests/data/lint-exceptions.json: no such table"),
        "{missing:?}"
    );
    assert_eq!(
        text(&written).0,
        "tests/data/lint-exceptions.json written\ntests/data/dependencies.json written"
    );
    assert_eq!(
        exceptions,
        "[\n  { \"file\": \"a.ts\", \"item\": \"f\", \"rule\": \"x\", \"reason\": \"why \\\"so\\\"\" }\n]\n"
    );
    assert_eq!(
        text(&agrees).0,
        "every data file the CI reads agrees with the records"
    );
    assert_eq!(
        text(&differs).1,
        "knowledge: data files the CI reads differ from the records: run target/release/knowledge export-data and commit them:\ntests/data/dependencies.json"
    );
    assert_eq!(text(&wrong).1, format!("knowledge: {USAGE}"));
}
