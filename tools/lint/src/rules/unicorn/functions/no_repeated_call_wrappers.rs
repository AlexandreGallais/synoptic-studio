//! `unicorn/no-repeated-call-wrappers`: a local function whose every call — two at least, none
//! inside itself — is wrapped in the same one-argument constructor, `new Set(f(x))`, or the same
//! primitive conversion, `Number(f(x))`, is reported once, on its name (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/no-repeated-call-wrappers.md`). The wrapper must be the same binding
//! everywhere, reachable inside the function too, with the same type arguments. Async functions
//! and generators are left. No fix: moving the wrapper changes the function's return type.

use super::calls::{inside, targets};
use crate::expressions::{symbol, text};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;
use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression};

/// The rule.
pub struct NoRepeatedCallWrappers;

/// The conversions a call can be wrapped in.
const CONVERSIONS: [&str; 4] = ["String", "Number", "Boolean", "BigInt"];

/// A wrapper: whether it is a `new`, its callee's name and variable, its type arguments.
#[derive(PartialEq, Eq)]
struct Wrapper<'a> {
    is_new: bool,
    name: &'a str,
    symbol: Option<oxc_semantic::SymbolId>,
    type_arguments: Option<&'a str>,
}

/// The wrapper around a call, when it is a one-argument `new` or conversion of it alone.
fn wrapper<'a>(
    source: &Source<'_, 'a>,
    call: oxc_semantic::NodeId,
    span: oxc_span::Span,
) -> Option<Wrapper<'a>> {
    let nodes = source.semantic.nodes();
    let AstKind::CallExpression(inner) = nodes.kind(call) else {
        return None;
    };
    if inner.optional
        || matches!(
            inner.callee.get_inner_expression(),
            Expression::ChainExpression(_)
        )
    {
        return None;
    }
    let only = |arguments: &[Argument]| matches!(arguments, [Argument::CallExpression(each)] if each.span == span);
    match nodes.parent_kind(call) {
        AstKind::NewExpression(outer) => match &outer.callee {
            Expression::Identifier(callee) if only(&outer.arguments) => Some(Wrapper {
                is_new: true,
                name: callee.name.as_str(),
                symbol: symbol(source, callee),
                type_arguments: outer
                    .type_arguments
                    .as_ref()
                    .map(|each| text(source, each.span)),
            }),
            _ => None,
        },
        AstKind::CallExpression(outer) if !outer.optional => match &outer.callee {
            Expression::Identifier(callee)
                if only(&outer.arguments) && CONVERSIONS.contains(&callee.name.as_str()) =>
            {
                Some(Wrapper {
                    is_new: false,
                    name: callee.name.as_str(),
                    symbol: symbol(source, callee),
                    type_arguments: outer
                        .type_arguments
                        .as_ref()
                        .map(|each| text(source, each.span)),
                })
            }
            _ => None,
        },
        _ => None,
    }
}

impl Rule for NoRepeatedCallWrappers {
    fn name(&self) -> &'static str {
        "unicorn/no-repeated-call-wrappers"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let scoping = source.semantic.scoping();
        for target in targets(source) {
            if target.callable.pauses()
                || target.calls.len() < 2
                || target
                    .calls
                    .iter()
                    .any(|call| inside(call.span, target.callable.span()))
            {
                continue;
            }
            let wrappers: Option<Vec<Wrapper>> = target
                .calls
                .iter()
                .map(|call| {
                    if call.is_call {
                        wrapper(source, call.id, call.span)
                    } else {
                        None
                    }
                })
                .collect();
            let Some(wrappers) = wrappers else {
                continue;
            };
            let first = &wrappers[0];
            let reachable = target
                .callable
                .scope()
                .map(|scope| scoping.find_binding(scope, first.name.into()))
                .is_some_and(|found| found == first.symbol);
            if !reachable || wrappers.iter().any(|each| each != first) {
                continue;
            }
            let shown = format!("{}{}()", if first.is_new { "new " } else { "" }, first.name);
            findings.push(source.finding(
                self.name(),
                target.name_at,
                format!("Every call wraps the result of this function in `{shown}`. Consider returning the wrapped value directly."),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoRepeatedCallWrappers;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoRepeatedCallWrappers, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_function_every_call_of_which_is_wrapped_the_same() {
        // The documentation's three cases with functions; its class case cannot occur here.
        assert_eq!(
            places(
                "const getWords = text => text.toLowerCase().split(' ');\nconst firstWords = new Set(getWords(first));\nconst secondWords = new Set(getWords(second));"
            ),
            [(1, 7)]
        );
        assert_eq!(
            places(
                "const getEntries = object => Object.entries(object);\nconst firstMap = new Map(getEntries(first));\nconst secondMap = new Map(getEntries(second));"
            ),
            [(1, 7)]
        );
        let converted = "function readCount(object) { return object.count; }\nconst firstCount = Number(readCount(first));\nconst secondCount = Number(readCount(second));";
        assert_eq!(places(converted), [(1, 10)]);
        assert_eq!(
            messages(&lint(&NoRepeatedCallWrappers, "scripts/a.ts", converted)),
            [
                "Every call wraps the result of this function in `Number()`. Consider returning the wrapped value directly."
            ]
        );
    }

    #[test]
    fn leaves_unwrapped_mixed_single_or_escaped_calls() {
        for code in [
            "const getWords = text => text.split(' ');\nconst uniqueWords = new Set(getWords(first));\nconst words = getWords(second);",
            "const f = (a) => a;\nnew Set(f(1));\nnew Map(f(2));",
            "const f = (a) => a;\nnew Set(f(1));",
            "export const f = (a) => a;\nnew Set(f(1));\nnew Set(f(2));",
            "const f = async (a) => a;\nnew Set(f(1));\nnew Set(f(2));",
            "const f = (a) => a;\nnew Set(f(1));\nnew Set(f(2));\nconst g = f;",
            "const f = (a) => a;\nString(f(1));\nString(f(2), 3);",
            "const f = (a) => a;\nnew Set<number>(f(1));\nnew Set<string>(f(2));",
            "function f(a) { return a ? new Set(f(a - 1)) : a; }\nnew Set(f(1));\nnew Set(f(2));",
            "const f = (a) => a;\nParse(f(1));\nParse(f(2));",
            "const f = (a) => { const Set = 1; return a; };\nnew Set(f(1));\nnew Set(f(2));",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
