//! Tests of what the pre-commit hook runs on which files (EN-108): lint-staged's matching,
//! chunking and readings of Git's `-z` outputs, each example computed by hand.

use std::collections::BTreeMap;

use git_hooks::staged::{
    Program, StatusEntry, Step, changed_by_tasks, chunks, flatten, matches, parse_tasks,
    partly_staged, patches_by_file, plan, repository_tasks, staged_from_raw, status_entries,
    z_fields,
};

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|&item| item.to_owned()).collect()
}

#[test]
fn matches_a_pattern_without_a_slash_on_the_base_name_dot_files_included() {
    assert!(matches("*.ts", "src/model/a.ts"));
    assert!(matches("*.ts", ".claude/hooks/guard.ts"));
    assert!(!matches("*.ts", "a.tsx"));
    assert!(!matches("*.md", "md"));
    // A list of extensions in braces.
    assert!(matches("*.{json,yml}", ".github/workflows/ci.yml"));
    assert!(matches("*.{json,yml}", "package.json"));
    assert!(!matches("*.{json,yml}", "Cargo.toml"));
    // `*` matches every file, a dot file inside a dot folder too.
    assert!(matches("*", ".github/dependabot.yml"));
    assert!(matches("*", ".githooks/pre-commit"));
    // A pattern holding `/` matches the whole path.
    assert!(matches("docs/a.md", "docs/a.md"));
    assert!(!matches("docs/a.md", "b/docs/a.md"));
}

#[test]
fn f21_ac4_plans_the_tasks_of_lint_staged_s_configuration() {
    let tasks = repository_tasks();
    let files = strings(&["src/a.ts", "README.md", "package.json", "logo.png"]);
    let planned = plan(&tasks, &files);
    let patterns: Vec<(&str, Vec<String>)> = planned
        .iter()
        .map(|(task, matched)| (task.pattern.as_str(), matched.clone()))
        .collect();

    assert_eq!(
        patterns,
        vec![
            ("*.ts", strings(&["src/a.ts"])),
            ("*.md", strings(&["README.md"])),
            (
                "*.{cjs,css,html,json,scss,toml,yaml,yml}",
                strings(&["package.json"])
            ),
            ("*", files.clone()),
        ]
    );

    let titles: Vec<Vec<String>> = tasks
        .iter()
        .map(|task| task.steps.iter().map(Step::title).collect())
        .collect();

    assert_eq!(
        titles,
        vec![
            strings(&[
                "format --write",
                "oxlint --fix --deny-warnings \
                 --report-unused-disable-directives-severity=error --no-error-on-unmatched-pattern",
                "lint --fix",
            ]),
            strings(&["format --write", "lint --fix"]),
            strings(&["format --write"]),
            strings(&["secret-scan"]),
        ]
    );
}

#[test]
fn leaves_out_a_task_that_matches_nothing() {
    let tasks = repository_tasks();
    let planned = plan(&tasks, &strings(&["logo.png"]));

    assert_eq!(planned.len(), 1);
    assert_eq!(planned[0].0.pattern, "*");
}

#[test]
fn reads_tasks_from_a_file() {
    let tasks = parse_tasks("# tests\n*.md ./upper one\n\n*.md ./check\n* ./scan\n").unwrap();

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].pattern, "*.md");
    assert_eq!(
        tasks[0].steps,
        vec![
            Step {
                program: Program::Path("./upper".to_owned()),
                arguments: strings(&["one"]),
            },
            Step {
                program: Program::Path("./check".to_owned()),
                arguments: Vec::new(),
            },
        ]
    );
    assert_eq!(tasks[1].steps[0].title(), "./scan");
    assert_eq!(
        parse_tasks("*.md\n").unwrap_err(),
        "line 1: a pattern without a command"
    );
}

#[test]
fn cuts_the_files_in_halves_until_each_chunk_fits() {
    let files = strings(&["aa", "bb", "cc"]);

    // "cmd aa bb cc" is 3 + 1 + 2 + 1 + 2 + 1 + 2 = 12 characters, over 9: the first half takes
    // two files, "cmd aa bb" being 9, the second one.
    assert_eq!(
        chunks("cmd", &files, 9),
        vec![strings(&["aa", "bb"]), strings(&["cc"])]
    );
    // Under the limit: one chunk.
    assert_eq!(chunks("cmd", &files, 12), vec![files.clone()]);
    // A single file over the limit is kept whole: nothing smaller can be sent.
    assert_eq!(
        chunks("cmd", &strings(&["a-long-name"]), 4),
        vec![strings(&["a-long-name"])]
    );
    // No file: one chunk without files.
    assert_eq!(chunks("cmd", &[], 1), vec![Vec::<String>::new()]);
}

#[test]
fn reads_the_fields_of_a_z_output() {
    assert_eq!(z_fields("a\0b c\0"), strings(&["a", "b c"]));
    assert_eq!(z_fields(""), Vec::<String>::new());
}

#[test]
fn reads_the_staged_files_without_submodules_nor_links() {
    let raw = ":100644 100644 abc def M\0src/a.ts\0\
               :000000 160000 000 111 A\0module\0\
               :100644 100644 123 456 R086\0old.md\0new.md\0\
               :000000 120000 000 222 A\0link\0";

    assert_eq!(staged_from_raw(raw), strings(&["src/a.ts", "new.md"]));
    assert_eq!(staged_from_raw(""), Vec::<String>::new());
}

#[test]
fn reads_the_partly_staged_files_of_git_status() {
    let entries = status_entries("MM a.ts\0RM new.md\0old.md\0 M b.ts\0M  c.ts\0");

    assert_eq!(
        entries[1],
        StatusEntry {
            code: "RM".to_owned(),
            path: "new.md".to_owned(),
            from: Some("old.md".to_owned()),
        }
    );
    assert_eq!(entries.len(), 4);

    let partly = partly_staged(&entries);

    // `MM` and `RM`: changed in the index and in the work tree; ` M` and `M ` are not.
    assert_eq!(
        flatten(&partly, true),
        strings(&["a.ts", "old.md", "new.md"])
    );
    assert_eq!(flatten(&partly, false), strings(&["a.ts", "new.md"]));
}

#[test]
fn gives_each_file_its_patch_and_names_those_the_tasks_changed() {
    // As `git diff --raw -z --patch` prints it: the raw part, two NULs, then the patch.
    let before = ":100644 100644 7898192 0000000 M\0x.ts\0:100644 100644 6178079 0000000 M\0y.ts\0\0\
                  diff --git a/x.ts b/x.ts\n@@ -1 +1 @@\n-a\n+a2\n\
                  diff --git a/y.ts b/y.ts\n@@ -1 +1 @@\n-b\n+b2\n";
    let patches = patches_by_file(before);

    assert_eq!(
        patches.get("x.ts").map(String::as_str),
        Some("a/x.ts b/x.ts\n@@ -1 +1 @@\n-a\n+a2\n")
    );
    assert_eq!(
        patches.get("y.ts").map(String::as_str),
        Some("a/y.ts b/y.ts\n@@ -1 +1 @@\n-b\n+b2\n")
    );

    // After the tasks: y.ts unchanged, x.ts's diff gone (fixed and identical to the index), z.ts new.
    let after = ":100644 100644 6178079 0000000 M\0y.ts\0:100644 100644 1 0000000 M\0z.ts\0\0\
                 diff --git a/y.ts b/y.ts\n@@ -1 +1 @@\n-b\n+b2\n\
                 diff --git a/z.ts b/z.ts\n@@ -1 +1 @@\n-c\n+C\n";

    assert_eq!(
        changed_by_tasks(&patches, &patches_by_file(after)),
        strings(&["x.ts", "z.ts"])
    );
    assert_eq!(patches_by_file(""), BTreeMap::new());
}
