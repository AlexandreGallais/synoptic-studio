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

    #[test]
    fn f20_ac6_parks_a_story_waiting_for_an_answer_and_the_run_goes_on() {
        // A question parks its own story (`parked`, its question in its row) and sets it `ready`
        // again once answered; nothing stops the run, the week's usage no more (AUD-051).
        assert_eq!(status_refusal("parked"), None);
        assert_eq!(status_refusal("ready"), None);
        assert!(status_refusal("stopped").is_some());
    }
}
