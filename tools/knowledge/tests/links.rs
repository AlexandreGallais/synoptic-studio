//! Tests of `knowledge link-ids` (EN-154): a link or a bare path to a backlog file written as its
//! id, read from the file's name; a link to an index keeping its text; anything else left as it is.

use knowledge::links::{id_of_file, link_text, with_ids};

#[test]
fn reads_the_id_a_backlog_files_name_gives() {
    assert_eq!(
        id_of_file("E49-F48-EN-151-the-tables.md").as_deref(),
        Some("EN-151")
    );
    assert_eq!(
        id_of_file("E02-REV-009-review-of-2026-10-25.md").as_deref(),
        Some("REV-009")
    );
    assert_eq!(
        id_of_file("E49-F48-the-knowledge-database.md").as_deref(),
        Some("F48")
    );
    assert_eq!(id_of_file("E01-draw-shapes.md").as_deref(), Some("E01"));
    for other in [
        "README.md",
        "_story.md",
        "E49.md",
        "x-E49-F48.md",
        "E49-F48-EN-151.txt",
    ] {
        assert_eq!(id_of_file(other), None, "{other}");
    }
}

#[test]
fn keeps_a_links_text_when_it_says_more_than_the_id() {
    assert_eq!(link_text("EN-151", "EN-151"), "EN-151");
    assert_eq!(link_text("`EN-151`", "EN-151"), "EN-151");
    assert_eq!(link_text("`E49-F48-EN-151-x.md`", "EN-151"), "EN-151");
    assert_eq!(link_text("E49 · F48", "F48"), "E49 · F48");
    assert_eq!(link_text("the audit", "AUD-050"), "the audit (AUD-050)");
}

#[test]
fn writes_each_link_to_a_backlog_file_as_its_id() {
    let text = "See [EN-151](../backlog/stories/E49-F48-EN-151-the-tables.md#tasks), \
                [the feature](../../docs/backlog/features/E49-F48-the-database.md) and \
                [E49](./backlog/epics/E49-keep.md).\n\
                | [AUD-050](../backlog/stories/E49-F48-AUD-050-audit-f48.md) | done |\n";

    assert_eq!(
        with_ids(text),
        "See EN-151, the feature (F48) and E49.\n| AUD-050 | done |\n"
    );
    // A link alone between parentheses: no second pair nested.
    assert_eq!(
        with_ids("a criterion of F19 ([feature](../backlog/features/E17-F19-oxc.md))."),
        "a criterion of F19 (feature F19)."
    );
}

#[test]
fn writes_a_bare_path_as_its_id_and_a_link_to_an_index_as_its_text() {
    let text = " * @see docs/backlog/stories/E01-F07-US-010-anchor-picker.md\n\
                `docs/backlog/features/E17-F21-agent-tools.md`, then\n\
                [the stories](../backlog/stories/README.md) and [epics](../backlog/epics/).\n";

    assert_eq!(
        with_ids(text),
        " * @see US-010\n`F21`, then\nthe stories and epics.\n"
    );
}

#[test]
fn leaves_every_other_link_and_path_as_it_is() {
    let text = "[agile](../process/agile.md), [x](https://example.org/backlog/stories/a), \
                docs/backlog/README.md, docs/backlog/epics/ and [a](b) [c] (d)\n\
                docs/backlog/stories/E99-*.md and docs/backlog/features/x.txt\n";

    assert_eq!(with_ids(text), text);
}

#[test]
fn reads_a_test_or_a_fixture_as_no_text_whose_ids_are_checked() {
    use knowledge::paths::is_test_path;

    for path in [
        "playground/a.test.ts",
        "tools/knowledge/tests/backlog.rs",
        "tests/backlog.test.ts",
        "tools/lint/tests/fixtures/x.txt",
    ] {
        assert!(is_test_path(path), "{path}");
    }
    for path in [
        "docs/tooling/knowledge.md",
        "src/a.ts",
        "playground/page/b.ts",
    ] {
        assert!(!is_test_path(path), "{path}");
    }
}

#[test]
fn writes_each_link_and_path_to_a_moved_doc_as_its_id() {
    let text = "[note 0035](../research/notes/0035-e49.md), [ADR-0081](../adr/0081-records.md#decision), \
                [the ADRs](../adr/), [request](./research/requests/0004-polygons.md), \
                [the bibliography](../references.md#git), [journal](./journal.md) \
                [log](../process/journal.md), [E01's](./retrospectives/E01-retrospective.md) \
                [record](../process/retrospectives/E17-retrospective.md), \
                [guide](../guide/e01-draw-shapes.md), [notes](../research/notes/)\n\
                `docs/derivations/text-box.md`, docs/adr/README.md, docs/research/README.md#2 and \
                docs/process/tool-incidents.md; stays: docs/process/agile.md, derivations/ alone, \
                docs/adr/{{n}}-*.md, [process](../process/README.md), [x](https://example.org/adr/)\n";

    assert_eq!(
        with_ids(text),
        "note 0035 (NOTE-0035), ADR-0081, the ADRs (PAGE-adr), request (REQUEST-0004), \
         the bibliography (PAGE-references), [journal](./journal.md) log (LOG-journal), \
         [E01's](./retrospectives/E01-retrospective.md) record (RETRO-E17), \
         guide (GUIDE-e01-draw-shapes), notes\n\
         `DERIV-text-box`, PAGE-adr, PAGE-research and LOG-tool-incidents; stays: \
         docs/process/agile.md, derivations/ alone, docs/adr/{{n}}-*.md, [process](../process/README.md), \
         [x](https://example.org/adr/)\n"
    );
}

#[test]
fn writes_a_see_of_a_domain_page_as_its_rules_id_elsewhere_the_path_kept() {
    let text = " * @see docs/domain/shapes.md#_2-corner-radius\n\
                 * @see docs/domain/interaction.md#text and docs/domain/README.md.\n\
                 * @see docs/domain/\n\
                 the rules: docs/domain/shapes.md#text\n";

    assert_eq!(
        with_ids(text),
        " * @see RULE-shapes-2-corner-radius\n\
          * @see RULE-interaction-text and RULE-domain.\n\
          * @see docs/domain/\n\
          the rules: docs/domain/shapes.md#text\n"
    );
}
