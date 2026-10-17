//! `unicorn/no-global-object-property-assignment`: a property written on the global object,
//! `globalThis`, `window`, `self` or `global` not declared in the file, by an assignment, an
//! update, a destructuring or a loop's target (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-global-object-property-assignment.md`). A `delete` and a computed key whose
//! value is unknown are left. No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::globals::{GLOBAL_OBJECTS, Write, member_name, written};
use crate::expressions::is_global;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::unwrap;
use crate::source::Source;

/// The rule.
pub struct NoGlobalObjectPropertyAssignment;

/// The rule's message.
const MESSAGE: &str = "Do not assign properties on the global object.";

impl Rule for NoGlobalObjectPropertyAssignment {
    fn name(&self) -> &'static str {
        "unicorn/no-global-object-property-assignment"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if !matches!(
            node.kind(),
            AstKind::StaticMemberExpression(_) | AstKind::ComputedMemberExpression(_)
        ) {
            return;
        }
        let Some((object, _, _)) = member_name(source, node) else {
            return;
        };
        let Expression::Identifier(object) = unwrap(object) else {
            return;
        };
        if GLOBAL_OBJECTS.contains(&object.name.as_str())
            && is_global(source, object)
            && written(source, node.id()) == Some(Write::Assigned)
        {
            findings.push(source.finding(
                self.name(),
                node.kind().span().start,
                MESSAGE.to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoGlobalObjectPropertyAssignment;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoGlobalObjectPropertyAssignment, "playground/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_property_written_on_the_global_object() {
        // The documentation's three cases, then the other ways to write.
        assert_eq!(places("globalThis.foo = value;"), [(1, 1)]);
        assert_eq!(places("window.foo += 1;"), [(1, 1)]);
        assert_eq!(places("self.foo ||= value;"), [(1, 1)]);
        assert_eq!(places("global['foo'] = 1;"), [(1, 1)]);
        assert_eq!(places("globalThis.count++;"), [(1, 1)]);
        assert_eq!(places("({ a: globalThis.a } = b);"), [(1, 7)]);
        assert_eq!(places("[globalThis.a = 1] = b;"), [(1, 2)]);
        assert_eq!(places("for (globalThis.a of b) {}"), [(1, 6)]);
        assert_eq!(places("(globalThis as any).foo = 1;"), [(1, 1)]);
        assert_eq!(places("(globalThis.foo as number) = 1;"), [(1, 2)]);
    }

    #[test]
    fn leaves_reads_deletes_locals_and_unknown_keys() {
        for code in [
            "export const foo = value;",
            "const singleton = { foo: value };",
            "globalThis.foo;",
            "delete globalThis.foo;",
            "const window = {};\nwindow.foo = 1;",
            "globalThis[key] = 1;",
            "other.foo = 1;",
            "a = globalThis.foo;",
            "[a = globalThis.b] = c;",
            "for (const a of globalThis.b) {}",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
