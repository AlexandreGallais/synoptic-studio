//! Which lines of a page sit in a fenced code block, where a `#` or a `|` is code, not Markdown.

/// For each line, whether it belongs to a fenced code block, its opening and closing fences
/// included. A fence is a line whose text starts with three backticks or three tildes; it closes on
/// the next line that starts with the same three characters.
#[must_use]
pub fn fenced(lines: &[&str]) -> Vec<bool> {
    let mut open: Option<&str> = None;

    lines
        .iter()
        .map(|line| {
            let text = line.trim_start();
            let marker = ["```", "~~~"]
                .into_iter()
                .find(|marker| text.starts_with(marker));

            match (open, marker) {
                (None, Some(marker)) => {
                    open = Some(marker);
                    true
                }
                (Some(current), Some(marker)) if current == marker => {
                    open = None;
                    true
                }
                (current, _) => current.is_some(),
            }
        })
        .collect()
}
