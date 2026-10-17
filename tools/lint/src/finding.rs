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

impl fmt::Display for Finding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}:{} {} {}",
            self.path, self.line, self.column, self.rule, self.message
        )
    }
}

/// A text with its fixes applied, from the last to the first; a fix overlapping one already
/// applied is left for the next run.
#[must_use]
pub fn apply(text: &str, fixes: &[&Fix]) -> String {
    let mut sorted: Vec<&Fix> = fixes.to_vec();
    sorted.sort_by_key(|fix| std::cmp::Reverse(fix.start));
    let mut result = text.to_owned();
    let mut floor = u32::MAX;

    for fix in sorted {
        if fix.end <= floor {
            result.replace_range(fix.start as usize..fix.end as usize, &fix.text);
            floor = fix.start;
        }
    }
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
    fn applies_fixes_from_the_end_and_skips_an_overlap() {
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
        assert_eq!(apply("abcde", &[&last, &overlapping]), "abcdE!");
    }
}
