//! Tests of the business rules in the records (E49): a domain page's parts and bullets read into
//! `rules` and documents `RULE-<page>-<anchor>`, imported twice alike from a folder of the
//! system's temporary folder. The real records folder is never opened.

use std::fs;

use knowledge::moved::{Base, Moved};
use knowledge::read::{Filters, show};
use knowledge::rules::{import_rules, page_of, rule_id, rule_parts, rules_of, slug};
use knowledge::store::{open_store, write};

/// A page of two parts, the second with a part of its own, a bullet continued, one unnamed and a
/// name repeated.
const PAGE: &str = "# Shapes\n\nThe shapes.\n\n## 2. Corner radius\n\n- **Typed**: a radius is entered as a number. 12 typed rounds the corner by 12.\n- **Per vertex**: each vertex its radius; a mode:\n  | Mode | Stored |\n  | --- | --- |\n- plain rule without a name\n\n### Radius clamping (ADR-0007)\n\n- **Typed**: clamped to half the edge. A radius of 80 on an edge of 100 gives 50.\n\n## 2. Corner radius\n\n- **Typed**: again.\n";

#[test]
fn makes_the_anchor_of_a_heading_and_the_id_of_a_part() {
    assert_eq!(slug("2. Selector"), "2-selector");
    assert_eq!(slug("Round shape (F10)"), "round-shape-f10");
    assert_eq!(slug("  Text — fonts  "), "text-fonts");
    assert_eq!(
        rule_id("shapes", "_2-corner-radius"),
        "RULE-shapes-2-corner-radius"
    );
    assert_eq!(rule_id("shapes", ""), "RULE-shapes");
    assert_eq!(page_of("README.md"), "domain");
    assert_eq!(page_of("symbols-and-views.md"), "symbols-and-views");
}

#[test]
fn cuts_a_bullet_into_its_name_rule_and_example() {
    assert_eq!(
        rule_parts(
            "**Clockwise, 0 to 359**: a positive angle turns clockwise. 370 gives 10, −90 gives 270."
        ),
        (
            "Clockwise, 0 to 359".to_owned(),
            "a positive angle turns clockwise.".to_owned(),
            "370 gives 10, −90 gives 270.".to_owned()
        )
    );
    // A decimal point is no sentence's end; one sentence alone has no example.
    assert_eq!(
        rule_parts("**Five decimals**: at most 5 decimals. 2.50000 is written `2.5`."),
        (
            "Five decimals".to_owned(),
            "at most 5 decimals.".to_owned(),
            "2.50000 is written `2.5`.".to_owned()
        )
    );
    assert_eq!(
        rule_parts("**Schematic**: a synoptic view stays readable."),
        (
            "Schematic".to_owned(),
            "a synoptic view stays readable.".to_owned(),
            String::new()
        )
    );
    assert_eq!(
        rule_parts("no name, no example"),
        (
            String::new(),
            "no name, no example".to_owned(),
            String::new()
        )
    );
}

#[test]
fn reads_the_parts_and_rules_of_a_page() {
    let (parts, rules) = rules_of("shapes.md", PAGE);
    let ids: Vec<&str> = parts.iter().map(|part| part.id.as_str()).collect();
    let rule_ids: Vec<(&str, &str)> = rules
        .iter()
        .map(|rule| (rule.id.as_str(), rule.section.as_str()))
        .collect();

    // The second part of the same anchor numbered as the site numbers it.
    assert_eq!(
        ids,
        [
            "RULE-shapes-2-corner-radius",
            "RULE-shapes-radius-clamping-adr-0007",
            "RULE-shapes-2-corner-radius-1"
        ]
    );
    // A part's text runs to the next heading of its level, its own parts included.
    assert!(parts[0].text.contains("### Radius clamping"));
    assert!(!parts[1].text.contains("again"));
    assert_eq!(
        rule_ids,
        [
            (
                "RULE-shapes-2-corner-radius-typed",
                "RULE-shapes-2-corner-radius"
            ),
            (
                "RULE-shapes-2-corner-radius-per-vertex",
                "RULE-shapes-2-corner-radius"
            ),
            (
                "RULE-shapes-2-corner-radius-3",
                "RULE-shapes-2-corner-radius"
            ),
            (
                "RULE-shapes-radius-clamping-adr-0007-typed",
                "RULE-shapes-radius-clamping-adr-0007"
            ),
            (
                "RULE-shapes-2-corner-radius-1-typed",
                "RULE-shapes-2-corner-radius-1"
            ),
        ]
    );
    assert_eq!(
        rules[1].rule,
        "each vertex its radius; a mode:\n| Mode | Stored |\n| --- | --- |"
    );
    assert_eq!(rules[0].example, "12 typed rounds the corner by 12.");
    assert_eq!(rules[4].position, 5);
}

#[test]
fn imports_the_domains_pages_twice_alike() {
    let folder = std::env::temp_dir().join(format!("knowledge-rules-{}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    fs::create_dir_all(folder.join("domain")).unwrap();
    fs::write(folder.join("domain/shapes.md"), PAGE).unwrap();

    let database = open_store(&folder.join("records.sqlite")).unwrap();
    let base = Base {
        branch: "chore/en-155-x",
        today: "2026-10-25",
    };
    let path = folder.join("domain");
    let first = write(&database, |done| import_rules(done, &path, base)).unwrap();
    let again = write(&database, |done| import_rules(done, &path, base)).unwrap();
    let count: i64 = database
        .query_row("SELECT count(*) FROM rules", [], |row| row.get(0))
        .unwrap();
    let opening = show(&database, "develop", "RULE-shapes", &Filters::default()).unwrap();
    let part = show(
        &database,
        "develop",
        "RULE-shapes-radius-clamping-adr-0007",
        &Filters::default(),
    )
    .unwrap();
    let paths: String = database
        .query_row("SELECT DISTINCT path FROM documents", [], |row| row.get(0))
        .unwrap();

    let _ = fs::remove_dir_all(&folder);
    // 5 rules; the page's opening and its 3 parts.
    assert_eq!(
        first,
        Moved {
            rows: 5,
            documents: 4
        }
    );
    assert_eq!(again, first);
    assert_eq!(count, 5);
    assert_eq!(opening.as_deref(), Some("# Shapes\n\nThe shapes.\n"));
    assert_eq!(
        part.as_deref(),
        Some(
            "### Radius clamping (ADR-0007)\n\n- **Typed**: clamped to half the edge. A radius of 80 on an edge of 100 gives 50.\n\n"
        )
    );
    assert_eq!(paths, "docs/domain/shapes.md");
}
