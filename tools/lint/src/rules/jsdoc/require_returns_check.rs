//! `jsdoc/require-returns-check`: a function documented with `@returns` returns a value — on
//! every branch when the tag names a type — unless its type may be `undefined`, it is `async` or
//! a generator; one documented `@returns {never}` returns none; one `@returns` tag at most
//! (`eslint-plugin-jsdoc` 65.3.1, `src/rules/requireReturnsCheck.js`, its default options).

use crate::finding::Finding;
use crate::jsdoc::{self, Doc, context, flow};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireReturnsCheck;

/// The tags that leave a block alone: abstract functions, constructors and interfaces.
const VOIDING: [&str; 5] = ["abstract", "virtual", "class", "constructor", "interface"];

/// The tags whose name defines a type, other than parameters and properties
/// (`getDocumentNamepathDefiningTags`), with their types.
fn typedefs(docs: &[Doc]) -> Vec<(String, String)> {
    const DEFINING: [&str; 21] = [
        "typedef",
        "callback",
        "class",
        "constructor",
        "const",
        "constant",
        "enum",
        "event",
        "external",
        "host",
        "exports",
        "function",
        "func",
        "method",
        "interface",
        "alias",
        "member",
        "var",
        "mixin",
        "name",
        "namespace",
    ];
    docs.iter()
        .flat_map(|doc| doc.block.tags.iter())
        .filter(|tag| DEFINING.contains(&tag.tag.as_str()))
        .map(|tag| (tag.name.clone(), tag.r#type.clone()))
        .collect()
}

impl Rule for RequireReturnsCheck {
    fn name(&self) -> &'static str {
        "jsdoc/require-returns-check"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut defined: Option<Vec<(String, String)>> = None;
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
            if function.is_async() {
                continue;
            }
            let tags: Vec<_> = doc
                .block
                .tags
                .iter()
                .filter(|tag| tag.tag == "returns")
                .collect();
            let tag = match tags.as_slice() {
                [] => continue,
                [tag] => tag,
                _ => {
                    findings.push(doc.finding(
                        source,
                        self.name(),
                        "Found more than one @returns declaration.".to_owned(),
                    ));
                    continue;
                }
            };
            let written = tag.r#type.trim();
            if written
                .find("asserts")
                .is_some_and(|at| written[at + 7..].starts_with(jsdoc::block::is_space))
            {
                continue;
            }
            if written == "never" {
                if flow::returns_value_or_resolves(function, false, false) {
                    findings.push(doc.finding(
                        source,
                        self.name(),
                        "JSDoc @returns declaration set with \"never\" but return expression is present in function.".to_owned(),
                    ));
                }
                continue;
            }
            let typedefs = defined.get_or_insert_with(|| typedefs(&jsdoc::all(source)));
            let returned = if tag.r#type.is_empty() {
                flow::returns_value_or_resolves(function, true, false)
            } else {
                flow::returns_value_or_resolves(function, true, true)
            };
            if !flow::return_may_be_undefined(&tag.r#type, typedefs)
                && !returned
                && !function.is_generator()
            {
                findings.push(doc.finding(
                    source,
                    self.name(),
                    "JSDoc @returns declaration present but return expression not available in function.".to_owned(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireReturnsCheck;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireReturnsCheck, "require-returns-check", "src/a.ts");
    }
}
