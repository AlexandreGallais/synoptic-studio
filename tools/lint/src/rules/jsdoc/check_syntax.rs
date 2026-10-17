//! `jsdoc/check-syntax`: no type of a tag ends with the `=` of Google Closure Compiler's optional
//! types; the block's first such tag is reported (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/checkSyntax.js`, typescript mode, no fixer).

use crate::finding::Finding;
use crate::jsdoc;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CheckSyntax;

impl Rule for CheckSyntax {
    fn name(&self) -> &'static str {
        "jsdoc/check-syntax"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            if let Some(tag) = doc.block.tags.iter().find(|tag| tag.r#type.ends_with('=')) {
                findings.push(doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0],
                    None,
                    "Syntax should not be Google Closure Compiler style.".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CheckSyntax;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&CheckSyntax, "check-syntax", "src/a.ts", &[]);
    }
}
