//! `jsdoc/check-alignment`: the `*` of a block's lines sit one column after its `/**`; the first
//! line out of place is reported, and every line moved there (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/checkAlignment.js`, no option). A blank line is out of place too.

use crate::finding::Finding;
use crate::jsdoc;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CheckAlignment;

/// The rule's message.
const MESSAGE: &str = "Expected JSDoc block to be aligned.";

/// The text before a line's first `*`, the whole line when it has none.
fn before_asterisk(line: &str) -> &str {
    line.split('*').next().unwrap_or_default()
}

impl Rule for CheckAlignment {
    fn name(&self) -> &'static str {
        "jsdoc/check-alignment"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            let level = doc.indent.encode_utf16().count() + 1;
            let misplaced = doc.text.split('\n').skip(1).position(|line| {
                let margin = before_asterisk(line);
                jsdoc::block::trim_start(margin).is_empty()
                    && margin.encode_utf16().count() != level
            });
            let Some(line) = misplaced else {
                continue;
            };
            let aligned: Vec<String> = doc
                .text
                .split('\n')
                .enumerate()
                .map(|(index, line)| {
                    if index == 0 || !jsdoc::block::trim_start(before_asterisk(line)).is_empty() {
                        line.to_owned()
                    } else {
                        format!("{} {}", doc.indent, jsdoc::block::trim_start(line))
                    }
                })
                .collect();
            let mut finding =
                doc.finding_at(source, self.name(), line + 1, None, MESSAGE.to_owned());
            finding.fix = Some(doc.replace(aligned.join("\n")));
            findings.push(finding);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CheckAlignment;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&CheckAlignment, "check-alignment", "src/a.ts", &[]);
    }
}
