//! `jsdoc/require-yields-check`: a function documented with `@yields` yields a value, and one
//! documented `@yields {never}` yields none; one `@yields` tag at most (`eslint-plugin-jsdoc`
//! 65.3.1, `src/rules/requireYieldsCheck.js`, `@next` not checked).

use crate::finding::Finding;
use crate::jsdoc::{self, context, flow};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireYieldsCheck;

/// The tags that leave a block alone: abstract functions, constructors and interfaces.
const VOIDING: [&str; 5] = ["abstract", "virtual", "class", "constructor", "interface"];

impl Rule for RequireYieldsCheck {
    fn name(&self) -> &'static str {
        "jsdoc/require-yields-check"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (node, doc) in context::documented(source, jsdoc::is_default_context) {
            if context::has_a_tag(&doc, &VOIDING)
                || context::is_constructor(source, node)
                || context::class_has_tag(source, node, "interface")
            {
                continue;
            }
            let Some(function) = flow::function_node(&node.kind()) else {
                continue;
            };
            let tags: Vec<_> = doc
                .block
                .tags
                .iter()
                .filter(|tag| tag.tag == "yields")
                .collect();
            let message = match tags.as_slice() {
                [] => continue,
                [tag] if tag.r#type.trim() == "never" => {
                    if !flow::yields(function) {
                        continue;
                    }
                    "JSDoc @yields declaration set with \"never\" but yield expression is present in function."
                }
                [tag] => {
                    if flow::may_be_undefined(&tag.r#type) || flow::yields(function) {
                        continue;
                    }
                    "JSDoc @yields declaration present but yield expression not available in function."
                }
                _ => "Found more than one @yields declaration.",
            };
            findings.push(doc.finding(source, self.name(), message.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireYieldsCheck;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireYieldsCheck, "require-yields-check", "src/a.ts", &[]);
    }
}
