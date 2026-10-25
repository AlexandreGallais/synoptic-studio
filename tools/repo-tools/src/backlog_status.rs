//! A backlog item's status changed in the records (RET-002, EN-103, EN-154): one row of `epics`,
//! `features` or `stories`, by the form of its id, which every branch reads at once; no file, no
//! index nor parent's table to write since the backlog moved into the records.

/// The statuses an item may take (`docs/backlog/README.md`, How work is planned).
pub const STATUSES: [&str; 6] = ["draft", "ready", "in-progress", "parked", "done", "dropped"];

/// Why a status is refused, `None` for one of [`STATUSES`].
#[must_use]
pub fn status_refusal(status: &str) -> Option<String> {
    (!STATUSES.contains(&status))
        .then(|| format!("`{status}` is not a status: {}", STATUSES.join(", ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_the_statuses_of_the_backlog_and_no_other() {
        for status in STATUSES {
            assert_eq!(status_refusal(status), None, "{status}");
        }
        assert_eq!(
            status_refusal("Done").as_deref(),
            Some("`Done` is not a status: draft, ready, in-progress, parked, done, dropped")
        );
        assert!(status_refusal("").is_some());
    }
}
