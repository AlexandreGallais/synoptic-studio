//! The business rules in the records (E49): each bullet of the domain's pages — its name, the rule
//! and its example — a row of `rules`, and each part of a page a document `RULE-<page>-<anchor>`,
//! so that the code cites a rule's part by a short id that `knowledge show` opens and
//! `knowledge check-ids` checks, rather than a page's path and anchor. The pages stay in the
//! repository for the Product Owner; an import reads them as they are, and may run again.

use std::path::Path;

use rusqlite::{Connection, params};

use crate::add::{Origin, put_merged};
use crate::moved::{Base, Moved, markdown_files};

/// A rule: a bullet of a part of a domain page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    /// `RULE-<page>-<anchor>-<name>`.
    pub id: String,
    /// The id of its part, `RULE-<page>-<anchor>`.
    pub section: String,
    /// Its name in bold, empty without one.
    pub name: String,
    /// What it rules.
    pub rule: String,
    /// Its example, its last sentence; empty when it has one sentence only.
    pub example: String,
    /// Its order in the page, from 1.
    pub position: usize,
}

/// A part of a domain page: its id, its heading, and its text down to the next heading of its
/// level or a higher one, its own parts included.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub id: String,
    pub heading: String,
    pub text: String,
}

/// The page's name in a rule's id: its file's stem, the index's `domain`.
#[must_use]
pub fn page_of(file: &str) -> String {
    match file.strip_suffix(".md").unwrap_or(file) {
        "README" => "domain".to_owned(),
        stem => stem.to_owned(),
    }
}

/// A heading's anchor as the site makes it, without the underscore it puts before a digit: lower
/// case, every run of other characters than letters and digits one dash, none at the ends.
/// `2. Selector` → `2-selector`; `Round shape (F10)` → `round-shape-f10`.
#[must_use]
pub fn slug(text: &str) -> String {
    let mut slug = String::new();

    for character in text.trim().to_lowercase().chars() {
        if character.is_alphanumeric() {
            slug.push(character);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    slug.trim_matches('-').to_owned()
}

/// The id of a page's part cited by an anchor of the site: `shapes`, `_2-corner-radius` →
/// `RULE-shapes-2-corner-radius`; the page itself without an anchor, `RULE-shapes`.
#[must_use]
pub fn rule_id(page: &str, anchor: &str) -> String {
    let anchor = anchor.trim_start_matches('_');

    if anchor.is_empty() {
        format!("RULE-{page}")
    } else {
        format!("RULE-{page}-{anchor}")
    }
}

/// A bullet's name, rule and example: `**Name**: the rule. The example.` → `Name`, `the rule.`,
/// `The example.`, the example its last sentence, cut at the last period followed by a space and
/// a capital, a digit, a sign or a quote.
#[must_use]
pub fn rule_parts(bullet: &str) -> (String, String, String) {
    let (name, rest) = bullet
        .strip_prefix("**")
        .and_then(|inner| inner.split_once("**"))
        .map_or(("", bullet), |(name, rest)| {
            (name, rest.trim_start_matches(':'))
        });
    let rest = rest.trim();
    let cut = rest
        .match_indices(". ")
        .map(|(at, _)| at)
        .filter(|at| {
            rest[at + 2..].chars().next().is_some_and(|next| {
                next.is_uppercase() || next.is_ascii_digit() || "−-+\"`(".contains(next)
            })
        })
        .last();

    match cut {
        Some(at) => (
            name.to_owned(),
            rest[..=at].trim().to_owned(),
            rest[at + 2..].trim().to_owned(),
        ),
        None => (name.to_owned(), rest.to_owned(), String::new()),
    }
}

/// The level of a heading line, 2 to 6, `None` for any other line.
fn level(line: &str) -> Option<usize> {
    let marks = line
        .chars()
        .take_while(|character| *character == '#')
        .count();

    ((2..=6).contains(&marks) && line[marks..].starts_with(' ')).then_some(marks)
}

/// The parts and the rules of a domain page: a part per heading of level two or more, its id from
/// its anchor, a second part of the same anchor numbered as the site does (`-1`); a rule per
/// bullet, its continuation's indented lines joined to it, its id its part's and its name's.
#[must_use]
pub fn rules_of(file: &str, text: &str) -> (Vec<Part>, Vec<Rule>) {
    let page = page_of(file);
    let lines: Vec<&str> = text.lines().collect();
    let mut parts: Vec<Part> = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();
    let mut section = rule_id(&page, "");
    let mut used: Vec<String> = Vec::new();

    for (at, line) in lines.iter().enumerate() {
        if let Some(depth) = level(line) {
            let heading = line[depth..].trim().to_owned();
            let mut id = rule_id(&page, &slug(&heading));
            let repeated = used.iter().filter(|known| **known == id).count();

            used.push(id.clone());
            if repeated > 0 {
                id = format!("{id}-{repeated}");
            }

            let end = lines[at + 1..]
                .iter()
                .position(|next| level(next).is_some_and(|other| other <= depth))
                .map_or(lines.len(), |offset| at + 1 + offset);

            parts.push(Part {
                id: id.clone(),
                heading,
                text: lines[at..end].join("\n") + "\n",
            });
            section = id;
        } else if let Some(bullet) = line.strip_prefix("- ") {
            let continued: Vec<&str> = lines[at + 1..]
                .iter()
                .take_while(|next| next.starts_with("  ") && !next.trim().is_empty())
                .map(|next| next.trim())
                .collect();
            let mut whole = bullet.trim().to_owned();

            for next in continued {
                whole.push('\n');
                whole.push_str(next);
            }

            let (name, rule, example) = rule_parts(&whole);
            let position = rules.len() + 1;
            // A rule without a name, or a name its part already holds, takes its position.
            let named = format!("{section}-{}", slug(&name));
            let id = if name.is_empty() || rules.iter().any(|known| known.id == named) {
                format!("{section}-{position}")
            } else {
                named
            };

            rules.push(Rule {
                id,
                section: section.clone(),
                name,
                rule,
                example,
                position,
            });
        }
    }
    (parts, rules)
}

/// Imports the domain's pages of a folder into `rules`, each part a merged document
/// `RULE-<page>-<anchor>` and each page's opening `RULE-<page>`; the path of each recorded as
/// `docs/domain/<file>`, whatever folder the pages were read from (a version of Git's among them).
///
/// # Errors
/// When SQLite fails.
pub fn import_rules(
    database: &Connection,
    folder: &Path,
    base: Base<'_>,
) -> rusqlite::Result<Moved> {
    let mut moved = Moved::default();

    for (file, text) in markdown_files(folder) {
        let page = page_of(&file);
        let path = format!("docs/domain/{file}");
        let origin = Origin {
            kind: "rule",
            path: &path,
            branch: base.branch,
            today: base.today,
        };
        let (parts, rules) = rules_of(&file, &text);
        let opening = text.split("\n## ").next().unwrap_or_default();

        database.execute("DELETE FROM rules WHERE page = ?", [&page])?;
        put_merged(database, &rule_id(&page, ""), opening, origin, false)?;
        moved.documents += 1;
        for part in &parts {
            put_merged(database, &part.id, &part.text, origin, false)?;
            moved.documents += 1;
        }
        for rule in &rules {
            database.execute(
                "INSERT OR REPLACE INTO rules (id, page, section, name, rule, example, position)
                VALUES (?, ?, ?, ?, ?, ?, ?)",
                params![
                    rule.id,
                    page,
                    rule.section,
                    rule.name,
                    rule.rule,
                    rule.example,
                    i64::try_from(rule.position).unwrap_or(0)
                ],
            )?;
            moved.rows += 1;
        }
    }
    Ok(moved)
}
