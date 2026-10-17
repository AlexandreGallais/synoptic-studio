//! `jsdoc/require-next-description`: a `@next` tag holds a name or a description
//! (`eslint-plugin-jsdoc` 65.3.1, `src/index.js`, the selector
//! `JsdocBlock:has(JsdocTag[tag=next]:not([name!=""]):not([description!=""]))` on any block).

use crate::finding::Finding;
use crate::jsdoc::{self, block::trim};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireNextDescription;

impl Rule for RequireNextDescription {
    fn name(&self) -> &'static str {
        "jsdoc/require-next-description"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for doc in jsdoc::all(source) {
            if doc.block.tags.iter().any(|tag| {
                tag.tag == "next" && tag.name.is_empty() && trim(&tag.description).is_empty()
            }) {
                findings.push(doc.finding(
                    source,
                    self.name(),
                    "@next should have a description".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireNextDescription;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(
            &RequireNextDescription,
            "require-next-description",
            "src/a.ts",
            &[],
        );
    }
}
