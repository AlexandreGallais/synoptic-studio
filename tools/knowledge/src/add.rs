//! `knowledge add <kind>`, refused (E49, AUD-057): every record is a row of its table with its
//! columns, its text cut into an item table, rather than a Markdown document, so no free text is
//! left to add; the refusal names the table and the `knowledge sql` that writes its row.

/// The table of a kind of record, and its item table when its text is cut into one; `None` for a
/// kind the records do not hold.
#[must_use]
pub fn row_table(kind: &str) -> Option<(&'static str, Option<&'static str>)> {
    let tables = match kind {
        "story" | "review" => ("stories", Some("story_items")),
        "feature" => ("features", Some("feature_items")),
        "epic" => ("epics", Some("epic_items")),
        "decision" | "adr" => ("decisions", Some("decision_items")),
        "note" | "request" | "page" => ("notes", Some("note_findings")),
        "derivation" => ("derivations", Some("derivation_steps")),
        "source" | "ref" => ("sources", None),
        "rule" => ("rules", Some("rule_items")),
        "retrospective" => ("retrospectives", Some("retrospective_items")),
        "memory" => ("memories", None),
        "journal" => ("journal", None),
        "incident" => ("incidents", None),
        "improvement" => ("improvements", None),
        "feedback" => ("feedback", None),
        _ => return None,
    };

    Some(tables)
}

/// Why `add` refuses a kind: the table its rows go to, and its item table, by `knowledge sql`.
#[must_use]
pub fn refusal(kind: &str) -> String {
    match row_table(kind) {
        Some((table, Some(items))) => format!(
            "{kind}: a row of the table {table}, its text in rows of {items}, not a document: target/release/knowledge sql \"INSERT INTO {table} (…) VALUES (…)\", then one per item, \"INSERT INTO {items} (…) VALUES (…)\""
        ),
        Some((table, None)) => format!(
            "{kind}: a row of the table {table}, not a document: target/release/knowledge sql \"INSERT INTO {table} (…) VALUES (…)\""
        ),
        None => format!(
            "{kind}: no document is added any more, every record is a row of its table: target/release/knowledge sql \"SELECT name, purpose FROM tables_registry\" names them, then knowledge sql \"INSERT INTO <table> …\""
        ),
    }
}
