//! `jsdoc/lines-before-block`: a block over several lines is set apart from the code or comment
//! on the line above it by a blank line, unless it opens a group (after `(`, `=`, `[` or `{`), sits
//! on the same line as that code, or holds a `@type` tag (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/linesBeforeBlock.js`, its default options). Autofixed.

use crate::finding::{Finding, Fix};
use crate::jsdoc::{self, Before};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct LinesBeforeBlock;

/// Whether the code character ending at an offset is the punctuator `(`, `=`, `[` or `{` alone,
/// not the end of a longer operator nor of a template's `${`.
fn opens_group(source: &Source, character: char, at: u32) -> bool {
    let previous = source.text[..at as usize].chars().next_back();
    match character {
        '(' | '[' => true,
        '{' => previous != Some('$'),
        '=' => !previous.is_some_and(|each| "=!<>+-*/%&|^?".contains(each)),
        _ => false,
    }
}

impl Rule for LinesBeforeBlock {
    fn name(&self) -> &'static str {
        "jsdoc/lines-before-block"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            if doc
                .block
                .tags
                .iter()
                .any(|tag| tag.tag.eq_ignore_ascii_case("type"))
            {
                continue;
            }
            let start = doc.comment.span.start;
            let end = match jsdoc::before(source, start) {
                Before::Start => continue,
                Before::Code(character, at) if opens_group(source, character, at) => continue,
                Before::Code(character, at) => {
                    at + u32::try_from(character.len_utf8()).unwrap_or(1)
                }
                Before::Comment(comment) => comment.span.end,
            };
            let line = source.line(start);
            let above = source.line(end.saturating_sub(1));
            if above + 1 < line || above == line || !doc.text.contains('\n') {
                continue;
            }
            let mut finding = doc.finding(
                source,
                self.name(),
                "Required 1 line(s) before JSDoc block".to_owned(),
            );
            finding.fix = Some(Fix {
                start: end,
                end,
                text: "\n".to_owned(),
            });
            findings.push(finding);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinesBeforeBlock;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&LinesBeforeBlock, "lines-before-block", "src/a.ts");
    }
}
