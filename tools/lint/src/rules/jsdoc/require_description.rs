//! `jsdoc/require-description` with `body`: a function's block opens with a description, not an
//! `@description` tag (`eslint-plugin-jsdoc` 65.3.1, `src/rules/requireDescription.js`,
//! constructors, getters and setters checked).

use crate::finding::Finding;
use crate::jsdoc::{self, context, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireDescription;

impl Rule for RequireDescription {
    fn name(&self) -> &'static str {
        "jsdoc/require-description"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (node, doc) in context::documented(source, jsdoc::is_default_context) {
            if context::avoids(source, &doc, node, true) {
                continue;
            }
            let description = text::description(&doc.block).text;
            if jsdoc::block::trim(&description)
                .split('\n')
                .any(|line| !line.is_empty())
            {
                continue;
            }
            let message = doc
                .block
                .tags
                .iter()
                .find(|tag| tag.tag == "desc" || tag.tag == "description")
                .map_or_else(
                    || "Missing JSDoc block description.".to_owned(),
                    |tag| {
                        format!(
                            "Remove the @{0} tag to leave a plain block description or add additional description text above the @{0} line.",
                            tag.tag
                        )
                    },
                );
            findings.push(doc.finding(source, self.name(), message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireDescription;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireDescription, "require-description", "src/a.ts");
    }
}
