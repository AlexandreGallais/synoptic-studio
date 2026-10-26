//! What a rule reports: a place, the rule, a message, and for an autofixed rule the text that
//! replaces a range of the file.

use std::fmt;

/// A replacement of a range of the file's text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix {
    /// The range's first byte.
    pub start: u32,
    /// The byte after the range.
    pub end: u32,
    /// The new text.
    pub text: String,
}

/// One finding of a rule in a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// The file, from the root, `/`-separated.
    pub path: String,
    /// The line, from 1.
    pub line: usize,
    /// The column, from 1, in UTF-16 units as `ESLint` counts them.
    pub column: usize,
    /// The rule's name.
    pub rule: &'static str,
    /// What is wrong, and what to do.
    pub message: String,
    /// The fix, for an autofixed rule.
    pub fix: Option<Fix>,
}

/// The finding of a file that cannot be read, under `lint/parse`, so that it never passes
/// silently.
#[must_use]
pub fn unreadable(path: &str, error: &std::io::Error) -> Finding {
    Finding {
        path: path.to_owned(),
        line: 1,
        column: 1,
        rule: crate::PARSE_RULE,
        message: format!("Cannot read the file: {error}."),
        fix: None,
    }
}

impl fmt::Display for Finding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{} {} {}",
            self.path, self.line, self.column, self.rule, self.message
        )
    }
}

/// A text with its fixes applied in `ESLint`'s order, from the first range to the last; a fix
/// starting at or before the end of one already applied is left for the next run.
#[must_use]
pub fn apply(text: &str, fixes: &[&Fix]) -> String {
    let mut sorted: Vec<&Fix> = fixes.to_vec();
    sorted.sort_by_key(|fix| (fix.start, fix.end));
    let mut result = String::with_capacity(text.len());
    let mut last: Option<u32> = None;

    for fix in sorted {
        if last.is_some_and(|end| end >= fix.start) || fix.start > fix.end {
            continue;
        }
        let from = last.unwrap_or(0) as usize;
        result.push_str(text.get(from..fix.start as usize).unwrap_or_default());
        result.push_str(&fix.text);
        last = Some(fix.end);
    }
    result.push_str(text.get(last.unwrap_or(0) as usize..).unwrap_or_default());
    result
}

#[cfg(test)]
mod tests {
    use super::{Finding, Fix, apply};

    #[test]
    fn prints_file_line_column_rule_and_message() {
        let finding = Finding {
            path: "src/a.ts".into(),
            line: 3,
            column: 10,
            rule: "local/require-kind",
            message: "No kind.".into(),
            fix: None,
        };

        assert_eq!(
            finding.to_string(),
            "src/a.ts:3:10 local/require-kind No kind."
        );
    }

    #[test]
    fn applies_fixes_from_the_first_and_skips_an_overlap_or_a_touch() {
        let first = Fix {
            start: 0,
            end: 1,
            text: "A".into(),
        };
        let last = Fix {
            start: 4,
            end: 5,
            text: "E!".into(),
        };
        let overlapping = Fix {
            start: 3,
            end: 5,
            text: "x".into(),
        };

        assert_eq!(apply("abcde", &[&first, &last]), "AbcdE!");
        assert_eq!(apply("abcde", &[&last, &overlapping]), "abcx");
        let touching = Fix {
            start: 1,
            end: 4,
            text: "-".into(),
        };
        assert_eq!(apply("abcde", &[&first, &touching, &last]), "AbcdE!");
        assert_eq!(apply("abcde", &[&touching]), "a-e");
    }
}
