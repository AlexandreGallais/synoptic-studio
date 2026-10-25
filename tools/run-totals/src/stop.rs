//! What `should-stop` says (EN-086, EN-154): a run never stops on its own any more — the stop at 90 %
//! of the week's usage was dropped by the Product Owner on 2026-10-25 — and a question never stops
//! one: it parks its story, which this module names, read from the records, for the run to skip it.

/// The statement `knowledge sql` runs for the parked stories, by id.
pub const PARKED: &str = "SELECT id FROM stories WHERE status = 'parked' ORDER BY id";

/// The first line of `should-stop`: the run goes on.
pub const GO: &str = "go: the week's usage stops no run (2026-10-25)";

/// What `should-stop` prints: [`GO`], then the parked stories when there are any.
#[must_use]
pub fn stop_text(parked: &[String]) -> String {
    if parked.is_empty() {
        format!("{GO}\n")
    } else {
        format!("{GO}\nparked: {}\n", parked.join(", "))
    }
}

/// The parked stories of the rows of [`PARKED`], each row's first cell.
#[must_use]
pub fn parked_of(rows: &[Vec<String>]) -> Vec<String> {
    rows.iter()
        .filter_map(|row| row.first())
        .filter(|id| !id.is_empty())
        .cloned()
        .collect()
}
