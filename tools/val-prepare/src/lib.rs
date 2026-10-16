//! `val:prepare <feature>` (EN-056): a feature's lines in its epic's checklist, one per acceptance
//! criterion to rewrite as what to try and what to see, the epic's checklist registered for
//! `?checklist=<epic>`, and no guided-steps file (Product Owner, 2026-10-15: the guided steps
//! written up to E15 stay in `?guide=epic` only).

use std::fs;
use std::path::{Path, PathBuf};

/// The most lines a feature holds in its epic's checklist; `checklist-line-limit.test.ts` fails
/// above it.
pub const LINE_LIMIT: usize = 12;

/// A feature as its backlog file gives it.
#[derive(Debug, PartialEq, Eq)]
pub struct Feature {
    /// `F18`.
    pub id: String,
    /// `E17`.
    pub epic: String,
    /// Its title.
    pub title: String,
    /// Its acceptance criteria, in order, each one line.
    pub criteria: Vec<String>,
}

/// A file to write: its path from the repository's root, and its whole text.
#[derive(Debug, PartialEq, Eq)]
pub struct Write {
    /// Path from the repository's root.
    pub path: PathBuf,
    /// The file's text.
    pub text: String,
}

/// What `val:prepare` does: the files to write, the checklist's address, and a warning.
#[derive(Debug, PartialEq, Eq)]
pub struct Plan {
    /// Each file written.
    pub writes: Vec<Write>,
    /// `http://localhost:5173/?checklist=E17`.
    pub address: String,
    /// Said when the feature has more criteria than lines allowed.
    pub warning: Option<String>,
}

const FEATURES: &str = "docs/backlog/features";
const CHECKLISTS: &str = "playground/checklists";

/// A feature id from the argument, in either case: `f18` gives `F18`.
#[must_use]
pub fn feature_id(argument: &str) -> Option<String> {
    let id = argument.trim().to_uppercase();
    let digits = id.strip_prefix('F')?;

    (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())).then_some(id)
}

/// The front matter's value of a key, between the two `---` fences.
fn field(text: &str, key: &str) -> Option<String> {
    let mut lines = text.lines();

    if lines.next() != Some("---") {
        return None;
    }

    lines
        .take_while(|line| *line != "---")
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.trim() == key)
        .map(|(_, value)| value.trim().trim_matches('"').to_owned())
}

/// The numbered items of the `## Acceptance criteria` section, their numbers removed.
fn criteria(text: &str) -> Vec<String> {
    text.lines()
        .skip_while(|line| line.trim() != "## Acceptance criteria")
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter_map(|line| {
            let (number, criterion) = line.split_once(". ")?;

            (!number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit()))
                .then(|| criterion.trim().to_owned())
        })
        .collect()
}

/// A feature from its backlog file's text.
///
/// # Errors
///
/// When its front matter lacks the id, the epic or the title.
pub fn parse_feature(text: &str) -> Result<Feature, String> {
    let read = |key: &str| field(text, key).ok_or(format!("the feature file has no `{key}`"));

    Ok(Feature {
        id: read("id")?,
        epic: read("epic")?,
        title: read("title")?,
        criteria: criteria(text),
    })
}

/// The epic's number without its leading zeros, `E01` giving `1`, as the playground's folders.
#[must_use]
pub fn epic_number(epic: &str) -> String {
    let digits = epic.trim_start_matches('E').trim_start_matches('0');

    if digits.is_empty() { "0" } else { digits }.to_owned()
}

/// A TypeScript string literal in double quotes.
#[must_use]
pub fn ts_string(text: &str) -> String {
    let escaped = text.replace('\\', "\\\\").replace('"', "\\\"");

    format!("\"{escaped}\"")
}

/// The file and constant of a feature's lines: `epic-17-f18-lines`, `EPIC_17_F18_LINES`.
#[must_use]
pub fn lines_names(feature: &Feature) -> (String, String) {
    let number = epic_number(&feature.epic);

    (
        format!("epic-{number}-{}-lines", feature.id.to_lowercase()),
        format!("EPIC_{number}_{}_LINES", feature.id),
    )
}

/// The file and constant of an epic's checklist: `epic-17-checklist`, `EPIC_17_CHECKLIST`.
#[must_use]
pub fn checklist_names(epic: &str) -> (String, String) {
    let number = epic_number(epic);

    (
        format!("epic-{number}-checklist"),
        format!("EPIC_{number}_CHECKLIST"),
    )
}

/// The module of a feature's lines: one per criterion, what to try being the criterion and what
/// to see a line to write, both rewritten before the validation.
#[must_use]
pub fn lines_module(feature: &Feature) -> String {
    let (_, constant) = lines_names(feature);
    let lines = feature
        .criteria
        .iter()
        .enumerate()
        .map(|(index, criterion)| {
            let tag = format!("{}.AC{}", feature.id, index + 1);

            format!(
                "  {{\n    feature: {},\n    todo: {},\n    see: {},\n  }},\n",
                ts_string(&feature.id),
                ts_string(&format!("{tag}: {criterion}")),
                ts_string(&format!("What to see for {tag}, to write")),
            )
        })
        .fold(String::new(), |mut all, item| {
            all.push_str(&item);
            all
        });

    format!(
        "import type {{ ChecklistLine }} from \"./checklist-line\";\n\n\
         /**\n * The lines of {epic}'s checklist for {id}, {title}.\n \
         *\n * Prepared by `val:prepare` from its acceptance criteria (EN-056), then written as what to\n \
         * try and what to see, at most {LINE_LIMIT}.\n \
         */\n\
         export const {constant}: readonly ChecklistLine[] = [\n{lines}];\n",
        epic = feature.epic,
        id = feature.id,
        title = feature.title,
    )
}

/// The module of an epic's checklist holding one feature's lines.
#[must_use]
pub fn checklist_module(epic: &str, lines_file: &str, lines_constant: &str) -> String {
    let (_, constant) = checklist_names(epic);

    format!(
        "import {{ {lines_constant} }} from \"./{lines_file}\";\n\n\
         import type {{ ChecklistLine }} from \"./checklist-line\";\n\n\
         /** {epic}'s checklist: its features' lines, in the order their validation was prepared (EN-056). */\n\
         export const {constant}: readonly ChecklistLine[] = [\n  ...{lines_constant},\n];\n"
    )
}

/// A module's text with an import line added before its first line, unless it already holds it.
fn with_import(text: &str, import: &str) -> String {
    if text.contains(import) {
        text.to_owned()
    } else {
        format!("{import}\n{text}")
    }
}

/// A module's text with an item added at the end of the list its last `closing` ends, unless
/// held: on a line of its own when the list spans lines, after a comma when Prettier wrote it on
/// one line (`[...EPIC_17_F18_LINES];`).
fn with_item(text: &str, item: &str, closing: &str) -> Result<String, String> {
    if text.contains(item) {
        return Ok(text.to_owned());
    }

    let at = text
        .rfind(closing)
        .ok_or(format!("no `{closing}` to add `{item}` before"))?;
    let (before, after) = text.split_at(at);

    Ok(if before.ends_with('\n') {
        format!("{before}  {item},\n{after}")
    } else {
        format!("{}, {item}{after}", before.trim_end_matches(','))
    })
}

/// An epic's checklist module with a feature's lines added at its end.
///
/// # Errors
///
/// When the module has no `];` line closing its list.
pub fn with_feature_lines(
    text: &str,
    lines_file: &str,
    lines_constant: &str,
) -> Result<String, String> {
    let import = format!("import {{ {lines_constant} }} from \"./{lines_file}\";");

    with_item(
        &with_import(text, &import),
        &format!("...{lines_constant}"),
        "];",
    )
}

/// The map of the checklists the playground opens with an epic's checklist added.
///
/// # Errors
///
/// When the module has no `]);` line closing its map.
pub fn with_checklist(text: &str, epic: &str) -> Result<String, String> {
    let (file, constant) = checklist_names(epic);
    let import = format!("import {{ {constant} }} from \"./{file}\";");

    with_item(
        &with_import(text, &import),
        &format!("[{}, {constant}]", ts_string(epic)),
        "]);",
    )
}

/// The folder's index with an export line per module, those missing added at its end.
#[must_use]
pub fn with_exports(text: &str, files: &[&str]) -> String {
    files.iter().fold(text.to_owned(), |index, file| {
        let line = format!("export * from \"./{file}\";");

        if index.contains(&line) {
            index
        } else {
            format!("{index}{line}\n")
        }
    })
}

fn read(root: &Path, path: &Path) -> Result<String, String> {
    fs::read_to_string(root.join(path)).map_err(|error| format!("{}: {error}", path.display()))
}

/// The feature's backlog file, `E17-F18-….md`.
fn feature_file(root: &Path, id: &str) -> Result<PathBuf, String> {
    let folder = Path::new(FEATURES);
    let marker = format!("-{id}-");
    let entries =
        fs::read_dir(root.join(folder)).map_err(|error| format!("{FEATURES}: {error}"))?;

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .find(|name| {
            name.contains(&marker)
                && Path::new(name)
                    .extension()
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        })
        .map(|name| folder.join(name))
        .ok_or(format!("no feature file for {id} in {FEATURES}"))
}

/// What `val:prepare` writes for a feature, read from the repository at `root`.
///
/// # Errors
///
/// When the feature is unknown, its lines already written, or a checklist module unreadable.
pub fn plan(root: &Path, id: &str) -> Result<Plan, String> {
    let feature = parse_feature(&read(root, &feature_file(root, id)?)?)?;
    let folder = Path::new(CHECKLISTS);
    let (lines_file, lines_constant) = lines_names(&feature);
    let (checklist_file, _) = checklist_names(&feature.epic);
    let lines_path = folder.join(format!("{lines_file}.ts"));
    let checklist_path = folder.join(format!("{checklist_file}.ts"));

    if feature.criteria.is_empty() {
        return Err(format!(
            "{id} has no numbered acceptance criteria yet: refine it first"
        ));
    }

    if root.join(&lines_path).exists() {
        return Err(format!(
            "{} exists: its lines are kept, never written again",
            lines_path.display()
        ));
    }

    let checklist = if root.join(&checklist_path).exists() {
        with_feature_lines(&read(root, &checklist_path)?, &lines_file, &lines_constant)?
    } else {
        checklist_module(&feature.epic, &lines_file, &lines_constant)
    };
    let map_path = folder.join("checklists.ts");
    let index_path = folder.join("index.ts");
    let writes = vec![
        Write {
            path: lines_path,
            text: lines_module(&feature),
        },
        Write {
            path: checklist_path,
            text: checklist,
        },
        Write {
            path: map_path.clone(),
            text: with_checklist(&read(root, &map_path)?, &feature.epic)?,
        },
        Write {
            path: index_path.clone(),
            text: with_exports(&read(root, &index_path)?, &[&checklist_file, &lines_file]),
        },
    ];
    let count = feature.criteria.len();

    Ok(Plan {
        writes,
        address: format!("http://localhost:5173/?checklist={}", feature.epic),
        warning: (count > LINE_LIMIT).then(|| {
            format!(
                "{id} has {count} criteria: merge its lines down to {LINE_LIMIT}, one per thing to see"
            )
        }),
    })
}
