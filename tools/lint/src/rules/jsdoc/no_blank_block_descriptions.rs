//! `jsdoc/no-blank-block-descriptions`: a block's description opens with no blank line before its
//! tags, and with one at most when no tag follows (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/noBlankBlockDescriptions.js`). Autofixed, the blank lines dropped.

use crate::finding::Finding;
use crate::jsdoc::block::{Line, Tokens, trim};
use crate::jsdoc::{self, Doc};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoBlankBlockDescriptions;

/// The lines of a block's description, after its `/**` line and before its first tag or its
/// `*/` line: their range in the block.
fn description_lines(doc: &Doc) -> Option<(usize, usize)> {
    let source = &doc.block.source;
    let start = source
        .iter()
        .position(|line| line.tokens.delimiter != "/**")?;
    let end = (start..source.len())
        .find(|index| {
            let tokens = &source[*index].tokens;
            !tokens.tag.is_empty() || !tokens.end.is_empty()
        })
        .unwrap_or(source.len());
    (end > start).then_some((start, end))
}

impl NoBlankBlockDescriptions {
    /// The finding of one block, if any.
    fn check(&self, source: &Source, doc: &Doc) -> Option<Finding> {
        let (start, end) = description_lines(doc)?;
        let lines = &doc.block.source[start..end];
        let blank = lines
            .iter()
            .take_while(|line| trim(&line.tokens.description).is_empty())
            .count();
        let (drop, at, message) = if doc.block.tags.is_empty() {
            let drop = if blank == lines.len() {
                lines.len() - 1
            } else {
                blank
            };
            (
                drop,
                start + drop,
                "There should be no extra blank lines in block descriptions not followed by tags.",
            )
        } else {
            (
                blank,
                (start + blank).saturating_sub(1),
                "There should be no blank lines in block descriptions followed by tags.",
            )
        };
        if drop == 0 {
            return None;
        }
        let first = &doc.block.source[start].tokens;
        let kept: Vec<Line> = lines[drop..]
            .iter()
            .map(|line| Line {
                number: 0,
                source: String::new(),
                tokens: Tokens {
                    delimiter: first.delimiter.clone(),
                    start: first.start.clone(),
                    post_delimiter: if line.tokens.description.is_empty() {
                        String::new()
                    } else {
                        line.tokens.post_delimiter.clone()
                    },
                    description: line.tokens.description.clone(),
                    ..Tokens::default()
                },
            })
            .collect();
        let mut block = doc.block.clone();
        block.source.splice(start..end, kept);
        let mut finding = doc.finding_at(source, self.name(), at, None, message.to_owned());
        finding.fix = Some(doc.rewrite(&block));
        Some(finding)
    }
}

impl Rule for NoBlankBlockDescriptions {
    fn name(&self) -> &'static str {
        "jsdoc/no-blank-block-descriptions"
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
    use super::NoBlankBlockDescriptions;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(
            &NoBlankBlockDescriptions,
            "no-blank-block-descriptions",
            "src/a.ts",
            &[],
        );
    }
}
