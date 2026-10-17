//! `jsdoc/multiline-blocks`: a block over several lines holds no text on its `/**` line, then
//! none on its `*/` line (`eslint-plugin-jsdoc` 65.3.1, `src/rules/multilineBlocks.js`, its
//! default options: one-line blocks allowed). Autofixed, the text moved to a line of its own.

use crate::finding::Finding;
use crate::jsdoc::block::{Line, Tokens, trim, trim_end};
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct MultilineBlocks;

/// A block line holding tokens.
fn line(tokens: Tokens) -> Line {
    Line {
        number: 0,
        source: String::new(),
        tokens,
    }
}

impl MultilineBlocks {
    /// The finding of one block, if any.
    fn check(&self, source: &Source, doc: &Doc) -> Option<Finding> {
        let mut block = doc.block.clone();
        if block.source.len() == 1 {
            return None;
        }
        let first = block.source[0].tokens.clone();
        if !first.tag.is_empty() || !first.description.is_empty() {
            let next = block.source[1].tokens.clone();
            block.source[0].tokens = Tokens {
                delimiter: first.delimiter.clone(),
                line_end: jsdoc::new_line_end(source),
                ..Tokens::default()
            };
            block.source.insert(
                1,
                line(Tokens {
                    delimiter: next.delimiter,
                    start: next.start,
                    ..first
                }),
            );
            let mut finding = doc.finding(
                source,
                self.name(),
                "Should have no text on the \"0th\" line (after the `/**`).".to_owned(),
            );
            finding.fix = Some(doc.rewrite(&block));
            return Some(finding);
        }
        let last = block.source.len() - 1;
        let final_tokens = block.source[last].tokens.clone();
        if trim(&final_tokens.description).is_empty() {
            return None;
        }
        block.source[last].tokens = Tokens {
            start: final_tokens.start.clone(),
            end: final_tokens.end.clone(),
            ..Tokens::default()
        };
        block.source.insert(
            last,
            line(Tokens {
                description: trim_end(&final_tokens.description).to_owned(),
                end: String::new(),
                line_end: jsdoc::new_line_end(source),
                ..final_tokens
            }),
        );
        let mut finding = doc.finding(
            source,
            self.name(),
            "Should have no text on the final line (before the `*/`).".to_owned(),
        );
        finding.fix = Some(doc.rewrite(&block));
        Some(finding)
    }
}

impl Rule for MultilineBlocks {
    fn name(&self) -> &'static str {
        "jsdoc/multiline-blocks"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        findings.extend(
            jsdoc::all(source)
                .iter()
                .filter_map(|doc| self.check(source, doc)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::MultilineBlocks;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&MultilineBlocks, "multiline-blocks", "src/a.ts");
    }
}
