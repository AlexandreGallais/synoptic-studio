//! The front matter of a backlog file: its `key: value` lines between two `---` fences.

use std::collections::HashMap;

/// Fields of a file's front matter, a value's surrounding double quotes removed.
///
/// A file without front matter gives no field.
pub fn front_matter(text: &str) -> HashMap<String, String> {
    let mut lines = text.lines();

    if lines.next() != Some("---") {
        return HashMap::new();
    }

    lines
        .take_while(|line| *line != "---")
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| (key.trim().to_owned(), unquoted(value.trim()).to_owned()))
        .collect()
}

/// A value without the double quotes YAML needs around a value holding `: `.
fn unquoted(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .unwrap_or(value)
}

#[cfg(test)]
mod tests {
    use super::front_matter;

    #[test]
    fn reads_the_fields_between_the_fences() {
        let fields = front_matter("---\nid: EN-049\nfeature: F18\n---\n\n# Title\nkey: no\n");

        assert_eq!(fields.get("id").map(String::as_str), Some("EN-049"));
        assert_eq!(fields.get("feature").map(String::as_str), Some("F18"));
        assert_eq!(fields.get("key"), None);
    }

    #[test]
    fn removes_the_quotes_and_keeps_a_colon_inside() {
        let fields = front_matter("---\ntitle: \"`a:b` one: two\"\n---\n");

        assert_eq!(
            fields.get("title").map(String::as_str),
            Some("`a:b` one: two")
        );
    }

    #[test]
    fn gives_nothing_without_front_matter() {
        assert!(front_matter("# Stories\n---\nid: X\n---\n").is_empty());
    }
}
