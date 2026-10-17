//! `jsdoc/require-throws`: a function whose body throws, outside a `try` block and any inner
//! function, has a `@throws` tag; one typed `never` does not throw (`eslint-plugin-jsdoc`
//! 65.3.1, `src/rules/requireThrows.js`). `async` functions reject instead: `require-rejects`.

use crate::finding::Finding;
use crate::jsdoc::{self, context, flow};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireThrows;

impl Rule for RequireThrows {
    fn name(&self) -> &'static str {
        "jsdoc/require-throws"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (node, doc) in context::documented(source, jsdoc::is_default_context) {
            if context::has_a_tag(&doc, &["abstract", "virtual", "type"])
                || context::avoids(source, &doc, node, true)
            {
                continue;
            }
            let Some(function) = flow::function_node(&node.kind()) else {
                continue;
            };
            let message = match doc.block.tags.iter().find(|tag| tag.tag == "throws") {
                Some(tag) if tag.r#type.trim() == "never" && flow::throws(function) => {
                    "JSDoc @throws declaration set to \"never\" but throw value found."
                }
                None if flow::throws(function) => "Missing JSDoc @throws declaration.",
                _ => continue,
            };
            findings.push(doc.finding(source, self.name(), message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireThrows;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireThrows, "require-throws", "src/a.ts", &[]);
    }
}
