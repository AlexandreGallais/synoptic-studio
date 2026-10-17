//! `jsdoc/require-template-description`: a `@template` tag describes its type parameters
//! (`eslint-plugin-jsdoc` 65.3.1, `src/index.js`, the selector
//! `JsdocBlock:has(JsdocTag[tag=template]:not([description!=""]))` on any block).

use crate::finding::Finding;
use crate::jsdoc::{self, block::trim};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireTemplateDescription;

impl Rule for RequireTemplateDescription {
    fn name(&self) -> &'static str {
        "jsdoc/require-template-description"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            if doc
                .block
                .tags
                .iter()
                .any(|tag| tag.tag == "template" && trim(&tag.description).is_empty())
            {
                findings.push(doc.finding(
                    source,
                    self.name(),
                    "@template should have a description".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireTemplateDescription;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(
            &RequireTemplateDescription,
            "require-template-description",
            "src/a.ts",
        );
    }
}
