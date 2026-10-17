//! `jsdoc/no-types`: the blocks of functions and method signatures hold no type on `@param`,
//! `@arg`, `@argument`, `@returns` nor `@return`, and a class's block none on `@prop` nor
//! `@property`: TypeScript holds the types (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/noTypes.js`). Autofixed, the type removed.

use oxc_ast::AstKind;

use crate::finding::Finding;
use crate::jsdoc::{self, context};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoTypes;

/// The tags of a function's block that hold no type.
const FUNCTION_TAGS: [&str; 5] = ["param", "arg", "argument", "returns", "return"];

/// Whether a node is a function, a method signature or a class declaration.
fn is_context(kind: &AstKind) -> bool {
    jsdoc::is_default_context(kind)
        || matches!(kind, AstKind::TSMethodSignature(_))
        || matches!(kind, AstKind::Class(class) if class.is_declaration())
}

impl Rule for NoTypes {
    fn name(&self) -> &'static str {
        "jsdoc/no-types"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for (node, doc) in context::documented(source, is_context) {
            let is_class = matches!(node.kind(), AstKind::Class(_));
            let mut block = doc.block.clone();
            let checked = doc.block.tags.iter().filter_map(|tag| {
                if FUNCTION_TAGS.contains(&tag.tag.as_str()) {
                    Some((tag, ""))
                } else if is_class && (tag.tag == "prop" || tag.tag == "property") {
                    Some((tag, " in the supplied context"))
                } else {
                    None
                }
            });
            let checked: Vec<_> = checked.filter(|(tag, _)| !tag.r#type.is_empty()).collect();
            for (tag, context) in checked {
                for index in &tag.lines {
                    let tokens = &mut block.source[*index].tokens;
                    tokens.post_tag.clear();
                    tokens.r#type.clear();
                }
                let mut finding = doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0],
                    None,
                    format!("Types are not permitted on @{}{context}.", tag.tag),
                );
                finding.fix = Some(doc.rewrite(&block));
                findings.push(finding);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoTypes;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&NoTypes, "no-types", "src/a.ts", &[]);
    }
}
