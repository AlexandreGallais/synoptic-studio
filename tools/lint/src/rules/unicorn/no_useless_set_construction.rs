//! `unicorn/no-useless-set-construction`: no `new Set(…)` around a `Set` method's argument, its
//! receiver or its result, since those methods take any Set-like and return fresh Sets
//! (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{
    Kind, MethodCall, has_comment, has_side_effect, is_global, kind_of, method_call, method_named,
    object_text, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessSetConstruction;

/// The methods that return a new `Set`.
const RETURNING: [&str; 4] = ["union", "intersection", "difference", "symmetricDifference"];
/// Every `Set` method that takes a Set-like.
const METHODS: [&str; 7] = [
    "union",
    "intersection",
    "difference",
    "symmetricDifference",
    "isSubsetOf",
    "isSupersetOf",
    "isDisjointFrom",
];

/// A plain call of one of some `Set` methods, with one argument, on a known `Set`.
fn set_method_call<'b, 'a>(
    source: &Source,
    expression: &'b Expression<'a>,
    names: &[&str],
) -> Option<MethodCall<'b, 'a>> {
    method_named(expression, names).filter(|method| {
        method.is_plain()
            && matches!(method.arguments().as_deref(), Some([_]))
            && kind_of(source, method.object) == Some(Kind::Set)
    })
}

/// The collection an argument already is: a known `Set`, or `.keys()` or `.values()` of one,
/// or `.keys()` of a known `Map`.
fn collection<'b, 'a>(
    source: &Source,
    expression: &'b Expression<'a>,
) -> Option<&'b Expression<'a>> {
    if kind_of(source, expression) == Some(Kind::Set) {
        return Some(expression);
    }
    let method = method_named(expression, &["keys", "values"])
        .filter(|method| method.is_plain() && method.call.arguments.is_empty())?;
    match kind_of(source, method.object) {
        Some(Kind::Set) => Some(method.object),
        Some(Kind::Map) if method.name == "keys" => Some(method.object),
        _ => None,
    }
}

impl Rule for NoUselessSetConstruction {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-set-construction"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::NewExpression(new) = node.kind() else {
            return;
        };
        let [argument] = new.arguments.as_slice() else {
            return;
        };
        let Some(argument) = argument.as_expression() else {
            return;
        };
        if !is_global(source, &new.callee, "Set") || new.type_arguments.is_some() {
            return;
        }
        let nodes = source.semantic.nodes();
        let parent = nodes.parent_node(node.id());
        let found = if let Some(result) = set_method_call(source, argument, &RETURNING) {
            Some((argument, result.name))
        } else {
            match parent.kind() {
                AstKind::CallExpression(call) if call.callee.node_id() != node.id() => {
                    method_call(call)
                        .filter(|outer| {
                            METHODS.contains(&outer.name)
                                && outer.is_plain()
                                && call.arguments.len() == 1
                                && kind_of(source, outer.object) == Some(Kind::Set)
                        })
                        .and_then(|outer| Some((collection(source, argument)?, outer.name)))
                }
                AstKind::StaticMemberExpression(member) if member.object.node_id() == node.id() => {
                    let AstKind::CallExpression(call) = nodes.parent_kind(parent.id()) else {
                        return;
                    };
                    let Some(outer) = method_call(call).filter(|outer| {
                        METHODS.contains(&outer.name)
                            && outer.is_plain()
                            && call.callee.node_id() == parent.id()
                    }) else {
                        return;
                    };
                    let Some(&[other]) = outer.arguments().as_deref() else {
                        return;
                    };
                    let replacement = collection(source, argument)
                        .filter(|each| kind_of(source, each) == Some(Kind::Set));
                    let other_known = matches!(kind_of(source, other), Some(Kind::Map | Kind::Set));
                    replacement
                        .filter(|_| other_known && !has_side_effect(other))
                        .map(|each| (each, outer.name))
                }
                _ => None,
            }
        };
        let Some((replacement, method)) = found else {
            return;
        };
        let message = format!("This `Set` construction is unnecessary for `Set#{method}()`.");
        if has_comment(source, new.span) {
            findings.push(source.finding(self.name(), new.span.start, message));
            return;
        }
        let as_object = matches!(parent.kind(), AstKind::StaticMemberExpression(member) if member.object.node_id() == node.id());
        let replaced = if as_object {
            object_text(source, replacement)
        } else {
            text(source, replacement.span()).to_owned()
        };
        findings.push(source.fixed(
            self.name(),
            new.span.start,
            message,
            Fix {
                start: new.span.start,
                end: new.span.end,
                text: replaced,
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessSetConstruction;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The declarations the cases read.
    const SETUP: &str = "const selected = new Set([1, 2]);\nconst records = new Map([[2, 'value']]);\nconst other = new Set([2, 3]);\n";

    /// The messages on a piece of code after the declarations.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &NoUselessSetConstruction,
            "src/a.ts",
            &format!("{SETUP}{code}"),
        ))
    }

    /// The code after the fixes, the declarations left out.
    fn fix(code: &str) -> String {
        let all = format!("{SETUP}{code}");
        fixed(&all, &lint(&NoUselessSetConstruction, "src/a.ts", &all))[SETUP.len()..].to_owned()
    }

    #[test]
    fn reports_and_fixes_the_documented_arguments() {
        assert_eq!(
            check("selected.intersection(new Set(records.keys()));"),
            ["This `Set` construction is unnecessary for `Set#intersection()`."]
        );
        assert_eq!(
            fix("selected.intersection(new Set(records.keys()));"),
            "selected.intersection(records);"
        );
        assert_eq!(
            fix("selected.union(new Set(other));"),
            "selected.union(other);"
        );
        assert_eq!(
            fix("selected.difference(new Set(other.values()));"),
            "selected.difference(other);"
        );
    }

    #[test]
    fn reports_and_fixes_the_documented_receivers_and_results() {
        assert_eq!(
            check("new Set(selected).union(other);"),
            ["This `Set` construction is unnecessary for `Set#union()`."]
        );
        assert_eq!(
            fix("new Set(selected).union(other);"),
            "selected.union(other);"
        );
        assert_eq!(
            fix("new Set(selected.keys()).isSubsetOf(other);"),
            "selected.isSubsetOf(other);"
        );
        assert_eq!(
            fix("const combined = new Set(selected.union(other));"),
            "const combined = selected.union(other);"
        );
    }

    #[test]
    fn accepts_needed_copies() {
        for code in [
            "selected.intersection(new Set(records.values()));",
            "selected.intersection(new Set(records.entries()));",
            "selected.intersection(new Set(records));",
            "selected.union(new Set(other.entries()));",
            "selected.union(new Set(items));",
            "new Set(selected).union((selected.clear(), other));",
            "new Set(selected).union(items);",
            "new Set(records.keys()).union(other);",
            "const copy = new Set(selected);",
            "const copy = new Set(selected.isSubsetOf(other));",
            "selected?.union(new Set(other));",
            "selected.union(new Set(other), extra);",
            "selected.union(new Set<number>(other));",
            "selected.union(new Set(other) as Set<number>);",
            "items.union(new Set(other));",
            "new Set(selected).add(1);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
