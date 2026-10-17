//! `jsdoc/no-bad-blocks`: a block comment holding block tags opens with `/**`, not `/*` nor
//! three asterisks or more; the tags of the compiler and licence directives (`@ts-check`,
//! `@ts-expect-error`, `@ts-ignore`, `@ts-nocheck`, `@license`, `@license-end`, `@licstart`,
//! `@licend`, `@source`) and `eslint` directives are left alone (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/noBadBlocks.js`, its default options). Autofixed.

use crate::finding::{Finding, Fix};
use crate::jsdoc::{block, comment_text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoBadBlocks;

/// The tags of comments left alone.
const IGNORED: [&str; 9] = [
    "ts-check",
    "ts-expect-error",
    "ts-ignore",
    "ts-nocheck",
    "license",
    "license-end",
    "licstart",
    "licend",
    "source",
];

/// The asterisks opening a comment after its `/`.
fn stars(text: &str) -> usize {
    text[1..].bytes().take_while(|each| *each == b'*').count()
}

/// Whether a block comment reads as a `JSDoc` block opened wrongly: `/*` or three asterisks or
/// more, then block tags none of which is left alone; whether it opens with three or more.
fn is_bad(text: &str) -> Option<bool> {
    let count = stars(text);
    if count == 1 && block::trim_start(&text[2..]).starts_with("eslint") {
        return None;
    }
    if count == 2 {
        return None;
    }
    let tags = block::parse(&format!("/**{}", &text[1 + count..])).tags;
    (!tags.is_empty() && !tags.iter().any(|tag| IGNORED.contains(&tag.tag.as_str())))
        .then_some(count > 2)
}

impl Rule for NoBadBlocks {
    fn name(&self) -> &'static str {
        "jsdoc/no-bad-blocks"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let comments: Vec<(&oxc_ast::ast::Comment, &str)> = source
            .program
            .comments
            .iter()
            .filter(|comment| comment.is_block())
            .map(|comment| (comment, comment_text(source, comment)))
            .collect();
        let extra = comments.iter().any(|(_, text)| stars(text) > 2);

        for (comment, text) in comments {
            if is_bad(text).is_none() {
                continue;
            }
            let fixed = if extra {
                if stars(text) > 2 {
                    format!("/**{}", &text[1 + stars(text)..])
                } else {
                    text.to_owned()
                }
            } else {
                text.replacen("/*", "/**", 1)
            };
            findings.push(source.fixed(
                self.name(),
                comment.span.start,
                "Expected JSDoc-like comment to begin with two asterisks.".to_owned(),
                Fix {
                    start: comment.span.start,
                    end: comment.span.end,
                    text: fixed,
                },
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoBadBlocks;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&NoBadBlocks, "no-bad-blocks", "src/a.ts", &[]);
    }
}
