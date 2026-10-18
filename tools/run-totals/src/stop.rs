//! When a run must stop on its own (EN-086): the week's usage reaching its limit. A question never
//! stops a run: it parks its story, which this module only names, for the run to skip it.

/// The week's usage, in percent, at which a run stops (decided on 2026-10-17, to keep a margin of
/// the subscription's week for the Product Owner).
pub const WEEK_LIMIT_PERCENT: i64 = 90;

/// The id of a story whose front matter, opened and closed by `---`, says `status: parked`; `None`
/// for any other file.
#[must_use]
pub fn parked_story(text: &str) -> Option<String> {
    let mut lines = text.lines();

    if lines.next() != Some("---") {
        return None;
    }

    let front: Vec<&str> = lines.take_while(|line| *line != "---").collect();
    let is_closed = text.lines().skip(1).any(|line| line == "---");

    if !is_closed || !front.contains(&"status: parked") {
        return None;
    }

    front
        .iter()
        .find_map(|line| line.strip_prefix("id: "))
        .map(|id| id.trim().to_owned())
}

/// Whether the run stops, and the text that says why: its first word `stop` or `go`, then the
/// parked stories when there are any.
#[must_use]
pub fn stop_text(week_percent: Option<i64>, parked: &[String]) -> (bool, String) {
    let (stops, first) = match week_percent {
        Some(percent) => {
            let stops = percent >= WEEK_LIMIT_PERCENT;
            let word = if stops { "stop" } else { "go" };

            (
                stops,
                format!("{word}: week {percent} % used, the limit is {WEEK_LIMIT_PERCENT} %\n"),
            )
        }
        None => (false, "go: week usage unknown\n".to_owned()),
    };

    if parked.is_empty() {
        (stops, first)
    } else {
        (stops, format!("{first}parked: {}\n", parked.join(", ")))
    }
}
