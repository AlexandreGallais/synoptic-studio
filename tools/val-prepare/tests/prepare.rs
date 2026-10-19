//! Tests of `val-prepare` (EN-056): the names and texts it writes, then the tool run on a copy of
//! the playground's checklists. `[F18.AC12]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use val_prepare::{
    Criterion, Feature, LINE_LIMIT, epic_number, feature_id, lines_module, parse_feature, plan,
    ts_string, with_checklist, with_exports, with_feature_lines,
};

const CHECKLISTS: &str = "../../playground/checklists";

fn feature_text(id: &str, epic: &str, count: usize) -> String {
    let criteria: String = (1..=count)
        .map(|number| format!("{number}. Criterion {number} of {id}.\n"))
        .collect::<Vec<_>>()
        .concat();

    format!(
        "---\nid: {id}\nepic: {epic}\ntitle: \"A title: with a colon\"\nstatus: in-progress\n---\n\n\
         # {epic} · {id}\n\n## Acceptance criteria\n\n{criteria}\n## Stories\n\n1. Not a criterion.\n"
    )
}
/// A temporary repository, removed when the test ends, even on a failure.
struct Root(PathBuf);

impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

impl std::ops::Deref for Root {
    type Target = PathBuf;

    fn deref(&self) -> &PathBuf {
        &self.0
    }
}

/// A repository of its own: a copy of the playground's checklists and one feature file per
/// `(id, epic, criteria)`.
fn repository(name: &str, features: &[(&str, &str, usize)]) -> Root {
    let root = std::env::temp_dir().join(format!("val-prepare-{}-{name}", std::process::id()));
    let checklists = root.join("playground/checklists");
    let backlog = root.join("docs/backlog/features");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&checklists).unwrap();
    fs::create_dir_all(&backlog).unwrap();

    for file in [
        "checklist-line.ts",
        "checklists.ts",
        "index.ts",
        "epic-15-checklist.ts",
    ] {
        fs::copy(Path::new(CHECKLISTS).join(file), checklists.join(file)).unwrap();
    }

    for (id, epic, count) in features {
        let file = backlog.join(format!("{epic}-{id}-a-feature.md"));
        fs::write(file, feature_text(id, epic, *count)).unwrap();
    }

    Root(root)
}

fn read(root: &Path, path: &str) -> String {
    fs::read_to_string(root.join(path)).unwrap()
}

#[test]
fn reads_a_feature_id_in_either_case() {
    assert_eq!(feature_id("f18").as_deref(), Some("F18"));
    assert_eq!(feature_id(" F05 ").as_deref(), Some("F05"));
    assert_eq!(feature_id("F"), None);
    assert_eq!(feature_id("E17"), None);
    assert_eq!(feature_id("F1a"), None);
}

#[test]
fn numbers_an_epic_as_the_playground_folders() {
    assert_eq!(epic_number("E17"), "17");
    assert_eq!(epic_number("E01"), "1");
    assert_eq!(epic_number("E00"), "0");
}

#[test]
fn escapes_quotes_and_backslashes_in_a_string() {
    assert_eq!(ts_string(r#"say "hi" \ `x`"#), r#""say \"hi\" \\ `x`""#);
}

#[test]
fn reads_the_numbered_criteria_only() {
    let feature = parse_feature(&feature_text("F18", "E17", 2)).unwrap();

    assert_eq!(
        feature,
        Feature {
            id: "F18".to_owned(),
            epic: "E17".to_owned(),
            title: "A title: with a colon".to_owned(),
            criteria: vec![
                Criterion {
                    number: 1,
                    text: "Criterion 1 of F18.".to_owned()
                },
                Criterion {
                    number: 2,
                    text: "Criterion 2 of F18.".to_owned()
                },
            ],
        }
    );
    assert!(parse_feature("# no front matter\n").is_err());
}

#[test]
fn writes_one_line_per_criterion_tagged() {
    let module = lines_module(&parse_feature(&feature_text("F18", "E17", 2)).unwrap());

    assert!(module.contains("export const EPIC_17_F18_LINES: readonly ChecklistLine[] = ["));
    assert!(module.contains("todo: \"F18.AC1: Criterion 1 of F18.\""));
    assert!(module.contains("see: \"What to see for F18.AC2, to write\""));
    assert_eq!(module.matches("feature: \"F18\"").count(), 2);
}

#[test]
fn adds_once_to_an_epic_checklist_the_map_and_the_index() {
    let epic = "import { A } from \"./a\";\n\nexport const E = [\n  ...A,\n];\n";
    let added = with_feature_lines(epic, "epic-15-f99-lines", "EPIC_15_F99_LINES").unwrap();

    assert_eq!(
        added,
        "import { EPIC_15_F99_LINES } from \"./epic-15-f99-lines\";\nimport { A } from \"./a\";\n\n\
         export const E = [\n  ...A,\n  ...EPIC_15_F99_LINES,\n];\n"
    );
    assert_eq!(
        with_feature_lines(&added, "epic-15-f99-lines", "EPIC_15_F99_LINES").unwrap(),
        added
    );
    assert!(with_feature_lines("no list", "x", "X").is_err());

    // The list as Prettier writes a short one, on one line.
    let short = "export const E = [...A];\n";

    assert_eq!(
        with_feature_lines(short, "b", "B").unwrap(),
        "import { B } from \"./b\";\nexport const E = [...A, ...B];\n"
    );

    let map = "export const C = new Map([\n  [\"E15\", EPIC_15_CHECKLIST],\n]);\n";
    let mapped = with_checklist(map, "E17").unwrap();

    assert!(
        mapped.contains("  [\"E15\", EPIC_15_CHECKLIST],\n  [\"E17\", EPIC_17_CHECKLIST],\n]);")
    );
    assert!(mapped.starts_with("import { EPIC_17_CHECKLIST } from \"./epic-17-checklist\";\n"));
    assert_eq!(with_checklist(&mapped, "E17").unwrap(), mapped);

    let index = with_exports("export * from \"./a\";\n", &["a", "b"]);

    assert_eq!(index, "export * from \"./a\";\nexport * from \"./b\";\n");
}

#[test]
fn f18_ac12_prepares_a_new_epic_checklist_and_its_address() {
    let root = repository("new-epic", &[("F99", "E99", 3)]);
    let prepared = plan(&root, "F99").unwrap();
    let paths: Vec<String> = prepared
        .writes
        .iter()
        .map(|write| write.path.display().to_string())
        .collect();

    assert_eq!(
        paths,
        [
            "playground/checklists/epic-99-f99-lines.ts",
            "playground/checklists/epic-99-checklist.ts",
            "playground/checklists/checklists.ts",
            "playground/checklists/index.ts",
        ]
    );
    assert!(paths.iter().all(|path| !path.contains("guided-steps")));
    assert_eq!(prepared.address, "http://localhost:5173/?checklist=E99");
    assert_eq!(prepared.warning, None);
    // Each generated module imports what it names from a file the playground holds or this run
    // writes, as the epic-15 modules do: the line type, the lines, the checklist.
    assert!(
        root.join("playground/checklists/checklist-line.ts")
            .exists()
    );
    assert!(
        prepared.writes[0]
            .text
            .starts_with("import type { ChecklistLine } from \"./checklist-line\";\n")
    );
    assert!(
        prepared.writes[1]
            .text
            .starts_with("import { EPIC_99_F99_LINES } from \"./epic-99-f99-lines\";\n")
    );
    assert!(
        prepared.writes[2]
            .text
            .contains("import { EPIC_99_CHECKLIST } from \"./epic-99-checklist\";\n")
    );
    assert!(
        prepared.writes[1]
            .text
            .contains("  ...EPIC_99_F99_LINES,\n];")
    );
    assert!(
        prepared.writes[2]
            .text
            .contains("[\"E99\", EPIC_99_CHECKLIST],")
    );
    assert!(prepared.writes[3].text.ends_with(
        "export * from \"./epic-99-checklist\";\nexport * from \"./epic-99-f99-lines\";\n"
    ));
}

#[test]
fn f18_ac12_adds_a_feature_to_its_epic_existing_checklist() {
    let root = repository("same-epic", &[("F99", "E15", 1)]);
    let prepared = plan(&root, "F99").unwrap();

    assert!(
        prepared.writes[1]
            .text
            .contains("  ...EPIC_15_COLOR_AND_FONT_LINES,\n  ...EPIC_15_F99_LINES,\n];")
    );
    // E15 is already in the map: only its import and entry once.
    assert_eq!(
        prepared.writes[2].text,
        read(&root, "playground/checklists/checklists.ts")
    );
}

#[test]
fn warns_above_twelve_criteria_and_not_at_twelve() {
    let root = repository("limit", &[("F01", "E01", 12), ("F02", "E01", 13)]);

    assert_eq!(plan(&root, "F01").unwrap().warning, None);
    assert_eq!(
        plan(&root, "F02").unwrap().warning.as_deref(),
        Some("F02 has 13 criteria: merge its lines down to 12, one per thing to see")
    );
}

#[test]
fn refuses_an_unknown_feature_one_not_refined_and_lines_already_written() {
    let root = repository("refusals", &[("F18", "E17", 1), ("F19", "E17", 0)]);

    assert_eq!(
        plan(&root, "F19").unwrap_err(),
        "F19 has no numbered acceptance criteria yet: refine it first"
    );

    assert!(
        plan(&root, "F77")
            .unwrap_err()
            .contains("no feature file for F77")
    );

    fs::write(
        root.join("playground/checklists/epic-17-f18-lines.ts"),
        "kept",
    )
    .unwrap();

    assert!(
        plan(&root, "F18")
            .unwrap_err()
            .contains("exists: its lines are kept")
    );
}

#[test]
fn f18_ac12_the_tool_writes_the_files_and_says_the_address() {
    let root = repository("run", &[("F18", "E17", 2)]);
    let output = Command::new(env!("CARGO_BIN_EXE_val-prepare"))
        .args([root.to_str().unwrap(), "f18"])
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "{stdout}");
    assert!(stdout.contains("http://localhost:5173/?checklist=E17"));
    assert!(read(&root, "playground/checklists/epic-17-f18-lines.ts").contains("F18.AC2"));
    assert!(read(&root, "playground/checklists/checklists.ts").contains("EPIC_17_CHECKLIST"));
    assert!(!root.join("playground/guided-steps").exists());

    let again = Command::new(env!("CARGO_BIN_EXE_val-prepare"))
        .args([root.to_str().unwrap(), "F18"])
        .output()
        .unwrap();

    assert!(!again.status.success());

    let usage = Command::new(env!("CARGO_BIN_EXE_val-prepare"))
        .output()
        .unwrap();

    assert_eq!(usage.status.code(), Some(2));
}

#[test]
fn warns_at_the_limit_the_playground_holds() {
    // One limit for both: the playground's test counts the lines a feature holds, this tool warns.
    let limit = fs::read_to_string(Path::new(CHECKLISTS).join("checklist-line-limit.ts")).unwrap();

    assert!(limit.contains(&format!(
        "export const CHECKLIST_LINE_LIMIT = {LINE_LIMIT};"
    )));
}

/// F20's criteria as its file wrote them on 2026-10-18: the fifth struck through and dropped.
const STRUCK: &str = "---\nid: F20\nepic: E17\ntitle: A context diet\n---\n\n## Acceptance criteria\n\n\
    4. A worker's first request is measured (EN-065).\n\
    5. ~~A claude.ai page shows a run's tree (EN-087).~~ Dropped on 2026-10-17: the page was judged not useful.\n\
    6. A hand-back names its commits (EN-088).\n\n## Stories\n";

#[test]
fn f32_ac2_writes_no_line_for_a_struck_criterion() {
    let feature = parse_feature(STRUCK).unwrap();

    // Criteria 4 and 6 kept with their own numbers, never renumbered 1 and 2; the fifth left out.
    assert_eq!(
        feature.criteria,
        [
            Criterion {
                number: 4,
                text: "A worker's first request is measured (EN-065).".to_owned()
            },
            Criterion {
                number: 6,
                text: "A hand-back names its commits (EN-088).".to_owned()
            },
        ]
    );

    let module = lines_module(&feature);

    assert!(module.contains("todo: \"F20.AC4: A worker's first request is measured (EN-065).\""));
    assert!(module.contains("see: \"What to see for F20.AC6, to write\""));
    assert!(!module.contains("AC5"));
    assert!(!module.contains("claude.ai"));
    assert_eq!(module.matches("feature: \"F20\"").count(), 2);
}

#[test]
fn f32_ac2_refuses_a_feature_whose_criteria_are_all_struck_and_counts_only_those_standing() {
    // A criterion struck and the rest of its line apart: a strike only inside the text stands.
    let text = "---\nid: F21\nepic: E17\ntitle: T\n---\n\n## Acceptance criteria\n\n\
        1. ~~Dropped.~~\n2. ~~Dropped too.~~ Moved to F22.\n\n## Stories\n";

    assert_eq!(parse_feature(text).unwrap().criteria, []);

    let inside = "---\nid: F21\nepic: E17\ntitle: T\n---\n\n## Acceptance criteria\n\n\
        1. A ~~struck~~ word.\n";

    assert_eq!(parse_feature(inside).unwrap().criteria.len(), 1);

    // F18's criterion 1 as its file writes it: its first half struck, its second standing.
    let half = "---\nid: F18\nepic: E17\ntitle: T\n---\n\n## Acceptance criteria\n\n\
        1. ~~Release pull request on `develop`~~, only the release on `main`. The first half superseded on 2026-10-18.\n\
        2. ~~A struck start~~ and the rest.\n\
        3. ~~Struck~~. Dropped on 2026-10-17.\n\
        4. ~~Struck, never closed.\n";

    assert_eq!(
        parse_feature(half)
            .unwrap()
            .criteria
            .iter()
            .map(|criterion| criterion.number)
            .collect::<Vec<_>>(),
        [1, 2, 4]
    );

    let root = repository("struck", &[]);

    fs::write(
        root.join("docs/backlog/features/E17-F21-a-feature.md"),
        text,
    )
    .unwrap();
    assert_eq!(
        plan(&root, "F21").unwrap_err(),
        "F21 has no numbered acceptance criteria yet: refine it first"
    );

    // Thirteen criteria, one struck: twelve stand, the limit itself, so no warning.
    let thirteen: String = (1..=13)
        .map(|number| {
            if number == 7 {
                "7. ~~Struck.~~\n".to_owned()
            } else {
                format!("{number}. Criterion {number}.\n")
            }
        })
        .collect();

    fs::write(
        root.join("docs/backlog/features/E17-F22-a-feature.md"),
        format!("---\nid: F22\nepic: E17\ntitle: T\n---\n\n## Acceptance criteria\n\n{thirteen}"),
    )
    .unwrap();
    assert_eq!(plan(&root, "F22").unwrap().warning, None);
}
