//! The Markdown that moved into the records (E49, the move into the records): the decisions, the
//! research, the derivations, the bibliography, the logs, the retrospectives and the guide pages.
//! Each importer reads the files the trunk held into a table shaped for the agent's questions, one
//! row per decision, note, source or log entry, and writes each file as a merged document too, so
//! that `search` finds its text and `show` prints it by its id. Run again, each row and document
//! is replaced, never doubled: an import is idempotent.

use std::fs;
use std::path::Path;

use rusqlite::{Connection, params};

use crate::add::{Origin, put_merged};
use crate::backlog::section_body;

/// Where an import writes from: the branch and the day of its documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Base<'a> {
    /// The branch the documents are written on, as merged rows.
    pub branch: &'a str,
    /// The day of a document whose text gives none.
    pub today: &'a str,
}

/// What an import wrote: the rows of its tables, and its documents.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Moved {
    pub rows: usize,
    pub documents: usize,
}

impl Moved {
    /// The sum of two imports.
    #[must_use]
    pub const fn plus(self, other: Self) -> Self {
        Self {
            rows: self.rows + other.rows,
            documents: self.documents + other.documents,
        }
    }
}

/// The Markdown files of a folder, by name, with their text; none when it cannot be read.
#[must_use]
pub fn markdown_files(folder: &Path) -> Vec<(String, String)> {
    let mut names: Vec<String> = fs::read_dir(folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|name| {
                    Path::new(name)
                        .extension()
                        .is_some_and(|extension| extension == "md")
                })
                .collect()
        })
        .unwrap_or_default();

    names.sort();
    names
        .into_iter()
        .filter_map(|name| {
            let text = fs::read_to_string(folder.join(&name)).ok()?;

            Some((name, text))
        })
        .collect()
}

/// The text of a document's first heading of level one, after its id and a dash when it opens
/// with them: `# ADR-0081 — The private records` → `The private records`; empty without one.
#[must_use]
pub fn title_of(text: &str) -> String {
    let heading = text
        .lines()
        .find_map(|line| line.strip_prefix("# "))
        .unwrap_or_default()
        .trim();

    heading
        .split_once(" — ")
        .map_or(heading, |(_, title)| title)
        .trim()
        .to_owned()
}

/// The first date `YYYY-MM-DD` of a text; empty without one.
#[must_use]
pub fn first_date(text: &str) -> String {
    let bytes = text.as_bytes();

    (0..bytes.len().saturating_sub(9))
        .find(|&at| {
            bytes[at..at + 10]
                .iter()
                .enumerate()
                .all(|(position, byte)| {
                    if matches!(position, 4 | 7) {
                        *byte == b'-'
                    } else {
                        byte.is_ascii_digit()
                    }
                })
                && (at == 0 || !bytes[at - 1].is_ascii_digit())
                && bytes.get(at + 10).is_none_or(|byte| !byte.is_ascii_digit())
        })
        .map(|at| text[at..at + 10].to_owned())
        .unwrap_or_default()
}

/// The value of a line `**Label**: value` of a text, trimmed; empty without one.
#[must_use]
pub fn labelled(text: &str, label: &str) -> String {
    let start = format!("**{label}**:");

    text.lines()
        .find_map(|line| line.trim().strip_prefix(&start))
        .unwrap_or_default()
        .trim()
        .to_owned()
}

/// The first word of a status, in lower case, its punctuation left out: `Accepted (…)` →
/// `accepted`, `done (→ …)` → `done`.
#[must_use]
pub fn status_word(status: &str) -> String {
    status
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(|character: char| !character.is_alphanumeric() && character != '-')
        .to_lowercase()
}

/// The cells of a table's row, `| a | b \| c |` → `a`, `b \| c`, trimmed: split at each pipe
/// that no backslash escapes, the empty edges left out. None for a line that is no row.
#[must_use]
pub fn row_cells(line: &str) -> Vec<String> {
    let line = line.trim();

    if !line.starts_with('|') {
        return Vec::new();
    }

    let mut cells = Vec::new();
    let mut cell = String::new();
    let mut escaped = false;

    for character in line[1..].chars() {
        if character == '|' && !escaped {
            cells.push(cell.trim().to_owned());
            cell.clear();
        } else {
            cell.push(character);
        }
        escaped = character == '\\';
    }
    if !cell.trim().is_empty() {
        cells.push(cell.trim().to_owned());
    }
    cells
}

/// The first sentence of a text: up to its first period followed by a space or its end, the
/// bold marks and a list's number or dash left out.
#[must_use]
pub fn first_sentence(text: &str) -> String {
    let line = text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or_default();
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    let line = line[digits..]
        .trim_start_matches(['.', '-', '*', ' '])
        .replace("**", "");
    let end = line.find(". ").map_or(line.len(), |at| at + 1);

    line[..end].trim().to_owned()
}

/// Writes one document, merged, of a file the trunk held.
fn document(
    database: &Connection,
    base: Base<'_>,
    (id, kind, text, path): (&str, &str, &str, &str),
) -> rusqlite::Result<()> {
    put_merged(
        database,
        id,
        text,
        Origin {
            kind,
            path,
            branch: base.branch,
            today: base.today,
        },
        false,
    )
}

/// The four digits a numbered file's name opens with, `0081-the-records.md` → `0081`.
fn number_of(name: &str) -> Option<&str> {
    let digits = name.get(..4)?;

    (digits.bytes().all(|byte| byte.is_ascii_digit()) && name[4..].starts_with('-'))
        .then_some(digits)
}

/// The id of a page that explains a folder rather than records one item: its index
/// `PAGE-<folder>`, a template `PAGE-<folder>-<name>`.
#[must_use]
pub fn page_id(folder: &str, name: &str) -> String {
    let stem = name.strip_suffix(".md").unwrap_or(name);

    if stem == "README" {
        format!("PAGE-{folder}")
    } else {
        format!("PAGE-{folder}-{}", stem.trim_start_matches('_'))
    }
}

/// A decision as its file gives it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub id: String,
    pub title: String,
    pub status: String,
    pub date: String,
    pub principle: String,
}

/// A decision from its file's name and text, `None` for a file not named by its number.
#[must_use]
pub fn decision_of(name: &str, text: &str) -> Option<Decision> {
    let number = number_of(name)?;
    let status = labelled(text, "Status");
    let decision = section_body(text, "Decision");

    Some(Decision {
        id: format!("ADR-{number}"),
        title: title_of(text),
        status: status_word(&status),
        date: [first_date(&status), first_date(text)]
            .into_iter()
            .find(|date| !date.is_empty())
            .unwrap_or_default(),
        principle: first_sentence(&decision),
    })
}

/// Imports the folder of the ADRs into `decisions`, each also a document, its index a page.
///
/// # Errors
/// When SQLite fails.
pub fn import_decisions(
    database: &Connection,
    folder: &Path,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (name, text) in markdown_files(folder) {
        let path = folder.join(&name).to_string_lossy().into_owned();

        if let Some(decision) = decision_of(&name, &text) {
            database.execute(
                "INSERT OR REPLACE INTO decisions (id, title, status, date, principle, text)
                VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    decision.id,
                    decision.title,
                    decision.status,
                    decision.date,
                    decision.principle,
                    text
                ],
            )?;
            document(database, base, (&decision.id, "decision", &text, &path))?;
            moved.rows += 1;
        } else {
            document(
                database,
                base,
                (&page_id("adr", &name), "page", &text, &path),
            )?;
        }
        moved.documents += 1;
    }
    Ok(moved)
}

/// Writes a row of `notes` and its document.
fn note(
    database: &Connection,
    base: Base<'_>,
    (id, kind, text, path): (&str, &str, &str, &str),
) -> rusqlite::Result<()> {
    let status = status_word(&labelled(text, "Status"));

    database.execute(
        "INSERT OR REPLACE INTO notes (id, kind, title, status, date, text) VALUES (?, ?, ?, ?, ?, ?)",
        params![id, kind, title_of(text), status, first_date(text), text],
    )?;
    document(database, base, (id, kind, text, path))
}

/// Imports the research folder into `notes`: its notes `NOTE-<number>`, its requests
/// `REQUEST-<number>`, its protocol and the request's template as pages.
///
/// # Errors
/// When SQLite fails.
pub fn import_research(
    database: &Connection,
    folder: &Path,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (place, prefix, kind) in [
        ("", "", "page"),
        ("notes", "NOTE", "note"),
        ("requests", "REQUEST", "request"),
    ] {
        let at = folder.join(place);

        for (name, text) in markdown_files(&at) {
            let path = at.join(&name).to_string_lossy().into_owned();
            let id = match number_of(&name) {
                Some(number) if !prefix.is_empty() => format!("{prefix}-{number}"),
                _ if place.is_empty() => page_id("research", &name),
                _ => page_id(&format!("research-{place}"), &name),
            };
            let kind = if id.starts_with("PAGE-") {
                "page"
            } else {
                kind
            };

            note(database, base, (&id, kind, &text, &path))?;
            moved.rows += 1;
            moved.documents += 1;
        }
    }
    Ok(moved)
}

/// The stories a derivation's `Used by:` line names, up to its sources; empty without one.
#[must_use]
pub fn used_by(text: &str) -> String {
    text.lines()
        .find_map(|line| line.strip_prefix("Used by:"))
        .map(|rest| rest.split(" Sources:").next().unwrap_or(rest))
        .unwrap_or_default()
        .trim()
        .trim_end_matches('.')
        .to_owned()
}

/// Imports the derivations' folder into `derivations`, `DERIV-<name>` each, its index a page.
///
/// # Errors
/// When SQLite fails.
pub fn import_derivations(
    database: &Connection,
    folder: &Path,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (name, text) in markdown_files(folder) {
        let path = folder.join(&name).to_string_lossy().into_owned();
        let stem = name.strip_suffix(".md").unwrap_or(&name);

        if stem == "README" || stem.starts_with('_') {
            document(
                database,
                base,
                (&page_id("derivations", &name), "page", &text, &path),
            )?;
        } else {
            let id = format!("DERIV-{stem}");

            database.execute(
                "INSERT OR REPLACE INTO derivations (id, title, used_by, text) VALUES (?, ?, ?, ?)",
                params![id, title_of(&text), used_by(&text), text],
            )?;
            document(database, base, (&id, "derivation", &text, &path))?;
            moved.rows += 1;
        }
        moved.documents += 1;
    }
    Ok(moved)
}

/// A row of the bibliography.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    pub id: String,
    pub title: String,
    pub url: String,
    pub used_for: String,
    /// `verified` or `unverified`.
    pub status: String,
    /// The day it was read, empty when not known.
    pub verified: String,
    /// The heading of the bibliography's part it was in.
    pub topic: String,
    /// The reference's cell whole: what was read in it.
    pub notes: String,
}

/// The first address `<https://…>` of a cell, its brackets left out; empty without one.
fn first_address(cell: &str) -> String {
    cell.split('<')
        .skip(1)
        .filter_map(|part| part.split_once('>').map(|(address, _)| address))
        .find(|address| address.starts_with("http"))
        .unwrap_or_default()
        .to_owned()
}

/// A source's title: its reference's cell before what was read in it, a quotation or its
/// address, at most 200 characters.
fn source_title(cell: &str) -> String {
    let end = [": \"", ": “", " <http", ": `"]
        .iter()
        .filter_map(|mark| cell.find(mark))
        .min()
        .unwrap_or(cell.len());
    let title = cell[..end].trim().trim_end_matches(['.', ':', ';']);

    title.chars().take(200).collect()
}

/// A source from a row of the bibliography and the heading above it; `None` for a row that is no
/// source's: its first cell a `REF-` id in code font, then the reference, its use and its status.
#[must_use]
pub fn source_of(line: &str, topic: &str) -> Option<Source> {
    let cells = row_cells(line);
    let [id, reference, used_for, status, ..] = cells.as_slice() else {
        return None;
    };
    let id = id.trim_matches('`');

    if !id.starts_with("REF-") {
        return None;
    }

    Some(Source {
        id: id.to_owned(),
        title: source_title(reference),
        url: first_address(reference),
        used_for: used_for.clone(),
        status: if status.starts_with("[verified]") {
            "verified".to_owned()
        } else {
            "unverified".to_owned()
        },
        verified: if status.starts_with("[verified]") {
            first_date(status)
        } else {
            String::new()
        },
        topic: topic.to_owned(),
        notes: reference.clone(),
    })
}

/// The sources of a bibliography, each under its part's heading.
#[must_use]
pub fn sources_of(text: &str) -> Vec<Source> {
    let mut topic = String::new();
    let mut sources = Vec::new();

    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            heading.trim().clone_into(&mut topic);
        } else if let Some(source) = source_of(line, &topic) {
            sources.push(source);
        }
    }
    sources
}

/// The document of a source, which `show REF-…` prints.
#[must_use]
pub fn source_text(source: &Source) -> String {
    let verified = if source.verified.is_empty() {
        source.status.clone()
    } else {
        format!("{} {}", source.status, source.verified)
    };

    format!(
        "# {} — {}\n\n- Address: {}\n- Used for: {}\n- Status: {verified}\n- Topic: {}\n\n{}\n",
        source.id, source.title, source.url, source.used_for, source.topic, source.notes
    )
}

/// Imports the bibliography into `sources`, each source a document `REF-…`, the page's opening a
/// page of its own.
///
/// # Errors
/// When SQLite fails.
pub fn import_sources(
    database: &Connection,
    path: &str,
    text: &str,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();
    let opening: String = text.split("\n## ").next().unwrap_or_default().to_owned();

    document(database, base, ("PAGE-references", "page", &opening, path))?;
    moved.documents += 1;
    for source in sources_of(text) {
        database.execute(
            "INSERT OR REPLACE INTO sources (id, title, url, used_for, status, verified, topic, notes)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                source.id,
                source.title,
                source.url,
                source.used_for,
                source.status,
                source.verified,
                source.topic,
                source.notes
            ],
        )?;
        document(
            database,
            base,
            (&source.id, "source", &source_text(&source), path),
        )?;
        moved.rows += 1;
        moved.documents += 1;
    }
    Ok(moved)
}

/// An entry of a log written as bullets under headings: the heading's text, and the bullet's
/// lines, its continuation joined to it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bullet {
    pub heading: String,
    pub text: String,
}

/// The bullets of a text under its headings of level two, a continuation line, indented, joined
/// to its bullet by a line end; the bullets above the first such heading left out.
#[must_use]
pub fn bullets_of(text: &str) -> Vec<Bullet> {
    let mut heading: Option<String> = None;
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut open = false;

    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            heading = Some(title.trim().to_owned());
            open = false;
        } else if let (Some(under), Some(rest)) = (&heading, line.strip_prefix("- ")) {
            bullets.push(Bullet {
                heading: under.clone(),
                text: rest.trim().to_owned(),
            });
            open = true;
        } else if open && line.starts_with([' ', '\t']) && !line.trim().is_empty() {
            if let Some(last) = bullets.last_mut() {
                last.text.push('\n');
                last.text.push_str(line.trim());
            }
        } else {
            open = false;
        }
    }
    bullets
}

/// A journal's bullet's tag and text: ``- `#run` EN-050: …`` → `run`, `EN-050: …`; an untagged
/// bullet's tag empty.
#[must_use]
pub fn tagged(text: &str) -> (String, String) {
    text.strip_prefix("`#")
        .and_then(|rest| rest.split_once('`'))
        .map_or_else(
            || (String::new(), text.to_owned()),
            |(tag, rest)| (tag.to_owned(), rest.trim().to_owned()),
        )
}

/// The rows of a log's tables whose first cell is a date, each cut into its cells.
#[must_use]
pub fn dated_rows(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(row_cells)
        .filter(|cells| {
            cells
                .first()
                .is_some_and(|cell| cell.len() == 10 && first_date(cell) == *cell)
        })
        .collect()
}

/// The cell of a row at an index, empty past its end.
fn cell(cells: &[String], at: usize) -> &str {
    cells.get(at).map_or("", String::as_str)
}

/// The text of an incident card's bullet opening with a label, `- Used: …` → `…`; empty without
/// one.
fn card_part(text: &str, label: &str) -> String {
    text.strip_prefix(label)
        .and_then(|rest| rest.strip_prefix(':'))
        .unwrap_or_default()
        .trim()
        .to_owned()
}

/// Writes the rows of one log file into its table; the rows written.
fn log_rows(database: &Connection, name: &str, text: &str) -> rusqlite::Result<usize> {
    let mut rows = 0;

    match name {
        "journal.md" => {
            for bullet in bullets_of(text) {
                let (tag, entry) = tagged(&bullet.text);

                rows += database.execute(
                    "INSERT OR IGNORE INTO journal (date, session, tag, text) VALUES (?, ?, ?, ?)",
                    params![first_date(&bullet.heading), bullet.heading, tag, entry],
                )?;
            }
        }
        "improvements.md" => {
            for cells in dated_rows(text) {
                // A pipe the item held unescaped cuts it: its cells between the origin and the
                // kind joined back.
                let (item, kind) = match cells.len() {
                    0..=3 => (cell(&cells, 2).to_owned(), ""),
                    count => (cells[2..count - 1].join(" | "), cell(&cells, count - 1)),
                };

                rows += database.execute(
                    "INSERT OR REPLACE INTO improvements (date, origin, item, kind) VALUES (?, ?, ?, ?)",
                    params![cells[0], cell(&cells, 1), item, kind],
                )?;
            }
        }
        "feedback.md" => {
            for cells in dated_rows(text) {
                rows += database.execute(
                    "INSERT OR REPLACE INTO feedback (date, happened, should_have, prevented_by)
                    VALUES (?, ?, ?, ?)",
                    params![cells[0], cell(&cells, 1), cell(&cells, 2), cell(&cells, 3)],
                )?;
            }
        }
        "tool-incidents.md" => {
            let mut cards: Vec<(String, [String; 3])> = Vec::new();

            for bullet in bullets_of(text) {
                if cards
                    .last()
                    .is_none_or(|(heading, _)| *heading != bullet.heading)
                {
                    cards.push((bullet.heading.clone(), Default::default()));
                }
                if let Some((_, parts)) = cards.last_mut() {
                    for (at, label) in ["Used", "Failed", "Instead"].iter().enumerate() {
                        let part = card_part(&bullet.text, label);

                        if !part.is_empty() {
                            parts[at] = part;
                        }
                    }
                }
            }
            for (heading, [used, failed, instead]) in cards {
                let tool = heading
                    .split_once(" — ")
                    .map_or(heading.as_str(), |(_, tool)| tool);

                rows += database.execute(
                    "INSERT OR REPLACE INTO incidents (date, tool, used, failed, instead)
                    VALUES (?, ?, ?, ?, ?)",
                    params![first_date(&heading), tool, used, failed, instead],
                )?;
            }
        }
        _ => {}
    }
    Ok(rows)
}

/// The id of a log's document: `journal.md` → `LOG-journal`.
#[must_use]
pub fn log_id(name: &str) -> String {
    format!("LOG-{}", name.strip_suffix(".md").unwrap_or(name))
}

/// Imports the process's logs into their tables — `journal`, `improvements`, `feedback`,
/// `incidents` — each log also a document `LOG-<name>`, the history's among them; and its
/// retrospectives into `retrospectives`, `RETRO-<epic>` each, their index and template pages.
/// The process's other pages, which stay in the repository, are left alone.
///
/// # Errors
/// When SQLite fails.
pub fn import_logs(
    database: &Connection,
    folder: &Path,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (name, text) in markdown_files(folder) {
        if !matches!(
            name.as_str(),
            "journal.md" | "history.md" | "improvements.md" | "feedback.md" | "tool-incidents.md"
        ) {
            continue;
        }

        let path = folder.join(&name).to_string_lossy().into_owned();

        moved.rows += log_rows(database, &name, &text)?;
        document(database, base, (&log_id(&name), "log", &text, &path))?;
        moved.documents += 1;
    }

    let retrospectives = folder.join("retrospectives");

    for (name, text) in markdown_files(&retrospectives) {
        let path = retrospectives.join(&name).to_string_lossy().into_owned();

        if let Some(epic) = name.strip_suffix("-retrospective.md") {
            let id = format!("RETRO-{epic}");

            database.execute(
                "INSERT OR REPLACE INTO retrospectives (id, epic, title, text) VALUES (?, ?, ?, ?)",
                params![id, epic, title_of(&text), text],
            )?;
            document(database, base, (&id, "retrospective", &text, &path))?;
            moved.rows += 1;
        } else {
            document(
                database,
                base,
                (&page_id("retrospectives", &name), "page", &text, &path),
            )?;
        }
        moved.documents += 1;
    }
    Ok(moved)
}

/// The id of a guide's page: `e01-draw-shapes.md` → `GUIDE-e01-draw-shapes`, its index and its
/// template pages.
#[must_use]
pub fn guide_id(name: &str) -> String {
    let stem = name.strip_suffix(".md").unwrap_or(name);

    if stem == "README" || stem.starts_with('_') {
        page_id("guide", name)
    } else {
        format!("GUIDE-{stem}")
    }
}

/// Imports the guide's pages as documents, `GUIDE-<name>` each (E49, Decisions, Q6).
///
/// # Errors
/// When SQLite fails.
pub fn import_guide(
    database: &Connection,
    folder: &Path,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (name, text) in markdown_files(folder) {
        let path = folder.join(&name).to_string_lossy().into_owned();
        let id = guide_id(&name);
        let kind = if id.starts_with("PAGE-") {
            "page"
        } else {
            "guide"
        };

        document(database, base, (&id, kind, &text, &path))?;
        moved.documents += 1;
    }
    Ok(moved)
}
