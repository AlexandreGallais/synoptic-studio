//! `jsdoc/require-rejects`: a function that rejects a promise — an `async` function that throws,
//! a call to `Promise.reject()` or to `reject()`, a `new Promise` whose executor rejects — has a
//! `@rejects` tag (`eslint-plugin-jsdoc` 65.3.1, `src/rules/requireRejects.js`).

use crate::finding::Finding;
use crate::jsdoc::{self, context, flow};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireRejects;

impl Rule for RequireRejects {
    fn name(&self) -> &'static str {
        "jsdoc/require-rejects"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (node, doc) in context::documented(source, jsdoc::is_default_context) {
            if context::has_a_tag(&doc, &["abstract", "virtual", "type"])
                || context::avoids(source, &doc, node, false)
                || doc.block.tags.iter().any(|tag| tag.tag == "rejects")
            {
                continue;
            }
            if flow::function_node(&node.kind()).is_some_and(flow::rejects) {
                findings.push(doc.finding(
                    source,
                    self.name(),
                    "Promise-rejecting function requires `@rejects` tag".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireRejects;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireRejects, "require-rejects", "src/a.ts");
    }
}
