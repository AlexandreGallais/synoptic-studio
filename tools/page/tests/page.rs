//! The rows, the sections and the fences of a page, on fixture pages written here (EN-105).
//! `f21_ac8_…` marks a test proving the feature's criterion.

use page::fence::fenced;
use page::section::{Section, first_section, heading, sections};
use page::table::{first_cell, rows};

/// Two tables, a fenced block that looks like Markdown, and nested headings.
const PAGE: &str = "\
# References

Intro.

## Hooks and tools

| ID | Reference | Status |
| --- | --- | --- |
| `REF-CC-HOOKS` | Hooks reference | [verified] |
| `REF-CC-HOOKS-GUIDE` | Hooks guide, names `REF-CC-HOOKS` | [verified] |

### Details

Text of the details.

```sh
# Hooks
| `REF-CC-HOOKS` | in a code block |
```

## Backlog

| Story | Feature |
| --- | --- |
| [EN-092](./EN-092.md) | F20 |

## Hooks

Second section of that name.


";

#[test]
fn f21_ac8_a_row_is_printed_with_its_table_header_only() {
    // The row of `REF-CC-HOOKS` alone: not the row of `REF-CC-HOOKS-GUIDE`, whose id only starts
    // with it, nor the row that names it in its second cell, nor the line in the code block.
    assert_eq!(
        rows(PAGE, "REF-CC-HOOKS"),
        vec!["| ID | Reference | Status |\n| `REF-CC-HOOKS` | Hooks reference | [verified] |\n"]
    );
}

#[test]
fn f21_ac8_a_linked_id_is_found_under_the_header_of_its_own_table() {
    assert_eq!(
        rows(PAGE, "EN-092"),
        vec!["| Story | Feature |\n| [EN-092](./EN-092.md) | F20 |\n"]
    );
}

#[test]
fn f21_ac8_an_id_in_no_row_finds_nothing() {
    assert_eq!(rows(PAGE, "REF-NONE"), Vec::<String>::new());
    // An id named in a later cell only does not start a row.
    assert_eq!(
        rows("| a | b |\n| --- | --- |\n| x | EN-1 |\n", "EN-1"),
        Vec::<String>::new()
    );
}

#[test]
fn a_header_found_by_its_first_cell_is_printed_once() {
    assert_eq!(rows(PAGE, "Story"), vec!["| Story | Feature |\n"]);
}

#[test]
fn a_row_found_twice_is_given_twice() {
    let page = "| a |\n| --- |\n| x |\n\n| b |\n| --- |\n| x |\n";

    assert_eq!(rows(page, "x"), vec!["| a |\n| x |\n", "| b |\n| x |\n"]);
}

#[test]
fn the_first_cell_loses_its_backticks_and_its_link() {
    assert_eq!(first_cell("| `REF-A` | b |"), Some("REF-A"));
    assert_eq!(
        first_cell("  | [ADR-0069](./0069.md) | b |"),
        Some("ADR-0069")
    );
    assert_eq!(first_cell("| plain |"), Some("plain"));
    assert_eq!(first_cell("|"), Some(""));
    assert_eq!(first_cell("text | not a row"), None);
}

#[test]
fn f21_ac8_a_section_runs_to_the_next_heading_of_its_level_or_higher() {
    // `## Hooks and tools` is not `Hooks`; the first `## Hooks` is the heading in the code block,
    // skipped; the real one, line 27, runs to the end of the page, its trailing blank lines left
    // out.
    assert_eq!(
        sections(PAGE, "Hooks and tools"),
        vec![Section {
            line: 5,
            text: PAGE
                .lines()
                .skip(4)
                .take(15)
                .fold(String::new(), |text, line| text + line + "\n"),
        }]
    );
    assert_eq!(
        sections(PAGE, "Hooks"),
        vec![Section {
            line: 27,
            text: "## Hooks\n\nSecond section of that name.\n".to_owned(),
        }]
    );
}

#[test]
fn f21_ac8_a_heading_found_twice_gives_both_with_their_lines() {
    let page = "# T\n\n## Hooks\n\none\n\n## Other\n\n### Hooks\n\ntwo\n## End\n";

    assert_eq!(
        sections(page, "Hooks"),
        vec![
            Section {
                line: 3,
                text: "## Hooks\n\none\n".to_owned(),
            },
            Section {
                line: 9,
                text: "### Hooks\n\ntwo\n".to_owned(),
            },
        ]
    );
}

#[test]
fn a_subsection_stays_inside_its_section() {
    let found = sections(PAGE, "Hooks and tools");

    assert!(found[0].text.contains("### Details"));
    assert!(!found[0].text.contains("## Backlog"));
}

#[test]
fn a_heading_in_backticks_is_found_by_its_text() {
    let page = "## `mutate`\n\nRuns.\n";

    assert_eq!(sections(page, "mutate")[0].text, page);
    assert_eq!(sections(page, "`mutate`")[0].line, 1);
}

#[test]
fn a_missing_heading_finds_nothing() {
    assert_eq!(sections(PAGE, "Nowhere"), Vec::new());
    assert_eq!(sections("", "Hooks"), Vec::new());
}

#[test]
fn a_heading_has_one_to_six_hashes_then_a_space() {
    assert_eq!(heading("## Hooks "), Some((2, "Hooks")));
    assert_eq!(heading("#"), Some((1, "")));
    assert_eq!(heading("###### Six"), Some((6, "Six")));
    assert_eq!(heading("####### Seven"), None);
    assert_eq!(heading("#hashtag"), None);
    assert_eq!(heading("text # no"), None);
}

#[test]
fn the_first_section_ends_at_the_second_heading() {
    let story = "---\nid: EN-1\n---\n\n# EN-1 — Title\n\nWhy.\n\n## Criteria\n\n- one\n";

    assert_eq!(
        first_section(story),
        "---\nid: EN-1\n---\n\n# EN-1 — Title\n\nWhy.\n"
    );
}

#[test]
fn a_page_of_one_heading_or_none_is_its_own_first_section() {
    assert_eq!(first_section("# Title\n\nAll.\n"), "# Title\n\nAll.\n");
    assert_eq!(first_section("No heading.\n"), "No heading.\n");
    assert_eq!(first_section(""), "");
    // A page of blank lines keeps none of them.
    assert_eq!(first_section("\n  \n"), "");
    // A page opening on its second heading's line keeps nothing before it.
    assert_eq!(first_section("# A\n# B\n"), "# A\n");
}

#[test]
fn a_fence_closes_on_its_own_marker_only() {
    let lines = ["a", "```", "~~~", "# in", "```", "b", "  ~~~", "c"];

    assert_eq!(
        fenced(&lines),
        vec![false, true, true, true, true, false, true, true]
    );
}
