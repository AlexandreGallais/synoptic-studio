//! Tests of a text read as a document (EN-152): its fields, its title, its sections cut at its
//! headings and the ids it cites. No file is read: every text is written here.

use knowledge::document::{Section, citations, is_citation, read_document, sections_of};

/// A story as the backlog writes one: front matter, a title, two sections, a fenced block whose
/// `#` line is code, and ids cited in the text and in the front matter.
const STORY: &str = "---
id: EN-900
epic: E49
feature: F48
title: Add, search, show and list
status: ready
---

# E49 · F48 · EN-900 — Add, search, show and list

The agent's one call to find a fact, as ADR-0081 and `REF-SQLITE-FTS5` say.

## Acceptance criteria

- A busy timeout waits for the other writer (EN-151.T1).

```text
# not a heading
```

## Tasks

- T1, after F48 and E49.
";

/// The sections' texts joined.
fn joined(sections: &[Section]) -> String {
    sections
        .iter()
        .map(|section| section.text.as_str())
        .collect()
}

/// A section of a heading and a text.
fn section(heading: &str, text: &str) -> Section {
    Section {
        heading: heading.to_owned(),
        text: text.to_owned(),
    }
}

#[test]
fn f48_ac2_cuts_a_story_at_its_headings_the_fenced_one_left_in_its_section() {
    let sections = sections_of(STORY);

    // The front matter above the first heading, then one section per heading; the `# not a
    // heading` line sits in a fenced block, so it stays in "Acceptance criteria".
    assert_eq!(
        sections,
        [
            section(
                "",
                "---\nid: EN-900\nepic: E49\nfeature: F48\ntitle: Add, search, show and list\nstatus: ready\n---\n\n"
            ),
            section(
                "E49 · F48 · EN-900 — Add, search, show and list",
                "# E49 · F48 · EN-900 — Add, search, show and list\n\nThe agent's one call to find a fact, as ADR-0081 and `REF-SQLITE-FTS5` say.\n\n"
            ),
            section(
                "Acceptance criteria",
                "## Acceptance criteria\n\n- A busy timeout waits for the other writer (EN-151.T1).\n\n```text\n# not a heading\n```\n\n"
            ),
            section("Tasks", "## Tasks\n\n- T1, after F48 and E49.\n"),
        ]
    );
    assert_eq!(joined(&sections), STORY);
}

#[test]
fn f48_ac2_gives_every_text_back_whole_once_its_sections_are_joined() {
    // Every order of these parts, two to four of them: a heading first or not, a text without a
    // heading, line ends of Windows, a fence of tildes, a last line without its line end.
    let parts = [
        "# Title\n",
        "text\r\n",
        "## Sub\r\n\r\n",
        "~~~\n## code\n~~~\n",
        "####### seven marks\n",
        "#tag\n",
        "last",
        "\n",
    ];
    let mut texts = 0;

    for first in parts {
        for second in parts {
            for third in parts {
                for fourth in ["", "## End\n", "end"] {
                    let text = format!("{first}{second}{third}{fourth}");

                    assert_eq!(joined(&sections_of(&text)), text, "{text:?}");
                    texts += 1;
                }
            }
        }
    }
    assert_eq!(texts, 8 * 8 * 8 * 3);
}

#[test]
fn f48_ac2_reads_a_heading_only_with_one_to_six_marks_and_a_space() {
    let sections = sections_of("#tag\n####### seven\n###### six\nbody\n#\n");

    // `#tag` lacks the space, `#######` has seven marks: both stay text. `#` alone is an empty
    // heading, as `page` reads it.
    assert_eq!(
        sections,
        [
            section("", "#tag\n####### seven\n"),
            section("six", "###### six\nbody\n"),
            section("", "#\n"),
        ]
    );
}

#[test]
fn f48_ac2_cuts_no_section_out_of_an_empty_text_and_one_out_of_a_text_without_heading() {
    assert_eq!(sections_of(""), Vec::<Section>::new());
    assert_eq!(
        sections_of("- `#run` EN-050: done.\n"),
        [section("", "- `#run` EN-050: done.\n")]
    );
    // A Windows line end is left out of the heading's text, kept in the section's.
    assert_eq!(sections_of("## A\r\n"), [section("A", "## A\r\n")]);
}

#[test]
fn f48_ac2_knows_the_ids_the_project_cites() {
    for id in [
        "REF-GIT-BRANCH",
        "REF-SQLITE-FTS5",
        "DERIV-bit",
        "DERIV-free-shape-stretch",
        "E49",
        "F48",
        "EN-152",
        "ADR-0081",
        "AUD-050",
        "JOURNAL-2026-10-25-3",
    ] {
        assert!(is_citation(id), "{id}");
    }
    for word in [
        "REF-",
        "REF-git",
        "DERIV-",
        "DERIV-Bit",
        "E",
        "F",
        "E2E",
        "T1",
        "AC4",
        "UTF-8",
        "EN-15",
        "X-123",
        "EN-152-",
        "SQLite",
        "en-152",
        "Ab-123",
        "EN-152-x",
        "",
    ] {
        assert!(!is_citation(word), "{word}");
    }
}

#[test]
fn f48_ac2_lists_the_ids_cited_once_each_in_the_order_first_met() {
    // `EN-151.T1` cites EN-151; `--EN-152--` its id without the dashes around it; `F48` twice.
    assert_eq!(
        citations("F48 and `REF-SQLITE-FTS5` (EN-151.T1), --EN-152--, F48's E2E.\n"),
        ["F48", "REF-SQLITE-FTS5", "EN-151", "EN-152"]
    );
    assert_eq!(citations("no id here"), Vec::<String>::new());
}

#[test]
fn f48_ac2_reads_a_storys_fields_title_sections_and_citations() {
    let document = read_document(STORY);

    assert_eq!(document.id.as_deref(), Some("EN-900"));
    assert_eq!(document.title, "Add, search, show and list");
    assert_eq!(
        (
            document.epic.as_str(),
            document.feature.as_str(),
            document.status.as_str()
        ),
        ("E49", "F48", "ready")
    );
    assert_eq!(document.date, None);
    assert_eq!(
        document.fields,
        [
            ("epic", "E49"),
            ("feature", "F48"),
            ("id", "EN-900"),
            ("status", "ready"),
            ("title", "Add, search, show and list"),
        ]
        .map(|(key, value)| (key.to_owned(), value.to_owned()))
    );
    assert_eq!(document.sections, sections_of(STORY));
    // Its own id, in its front matter and its title, is no citation of it.
    assert_eq!(
        document.citations,
        ["E49", "F48", "ADR-0081", "REF-SQLITE-FTS5", "EN-151"]
    );
}

#[test]
fn f48_ac2_titles_a_document_by_its_first_heading_else_its_first_line() {
    let note = read_document("---\ndate: 2026-10-25\n---\n\n# 0035 — The knowledge\n\nText.\n");
    let row = read_document("\n  - `#run` EN-050: done.  \nmore\n");

    assert_eq!(note.id, None);
    assert_eq!(note.title, "0035 — The knowledge");
    assert_eq!(note.date.as_deref(), Some("2026-10-25"));
    assert_eq!(row.title, "- `#run` EN-050: done.");
    assert_eq!(row.fields, Vec::<(String, String)>::new());
    assert_eq!(row.citations, ["EN-050"]);
}
