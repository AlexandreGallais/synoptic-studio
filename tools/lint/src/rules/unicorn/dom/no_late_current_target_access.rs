//! `unicorn/no-late-current-target-access`: `event.currentTarget` is `null` once the handler
//! has suspended, or inside a function it defers, so it is read before (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/no-late-current-target-access.md`). The event is a handler's parameter
//! named `e`, `event`, `evt` or `…Event`, unless its annotation says it is no event.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::events::{enclosing_function, event_class, handler_of, is_event_name, runs_late};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::annotations::Class;
use crate::source::Source;

/// The rule.
pub struct NoLateCurrentTargetAccess;

impl Rule for NoLateCurrentTargetAccess {
    fn name(&self) -> &'static str {
        "unicorn/no-late-current-target-access"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::StaticMemberExpression(member) = node.kind() else {
            return;
        };
        let Expression::Identifier(event) = &member.object else {
            return;
        };
        if member.property.name != "currentTarget" || !is_event_name(&event.name) {
            return;
        }
        let Some((handler, symbol)) = handler_of(source, event) else {
            return;
        };
        if matches!(
            event_class(source, symbol),
            Class::NonTarget | Class::Nullish
        ) {
            return;
        }
        let nested = enclosing_function(source, node.id()) != Some(handler);
        if !nested && !runs_late(source, node.id(), handler, member.span.start) {
            return;
        }
        let name = &event.name;
        let message = if nested {
            format!(
                "`{name}.currentTarget` is `null` inside this nested function. It is only set during synchronous event dispatch; save it to a variable in the outer function."
            )
        } else {
            format!(
                "`{name}.currentTarget` is `null` after the handler suspends. It is only set during synchronous event dispatch; save it to a variable beforehand."
            )
        };
        findings.push(source.finding(self.name(), member.span.start, message));
    }
}

#[cfg(test)]
mod tests {
    use super::NoLateCurrentTargetAccess;
    use crate::testing::lint;

    /// The places of the findings, and whether each says a nested function.
    fn check(code: &str) -> Vec<(usize, usize, bool)> {
        lint(&NoLateCurrentTargetAccess, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message.contains("nested")))
            .collect()
    }

    #[test]
    fn reports_reads_after_a_suspension_or_in_a_nested_function() {
        // The documentation's two cases.
        assert_eq!(
            check(
                "element.addEventListener('click', event => {\n\tsetTimeout(() => {\n\t\tevent.currentTarget.click();\n\t});\n});"
            ),
            [(3, 3, true)]
        );
        assert_eq!(
            check(
                "element.addEventListener('click', async event => {\n\tevent.currentTarget.disabled = true;\n\tawait somePromise;\n\tevent.currentTarget.disabled = false;\n});"
            ),
            [(4, 2, false)]
        );
        // A loop holding a suspension, a `for await`, a generator's `yield`, a typed name.
        assert_eq!(
            check(
                "async function f(e) {\n  while (x) {\n    e.currentTarget;\n    await y;\n  }\n  for await (const a of b) { e.currentTarget; }\n}\nfunction* g(clickEvent: MouseEvent) { yield 1; clickEvent.currentTarget; }"
            ),
            [(3, 5, false), (6, 30, false), (8, 48, false)]
        );
        // The iterable is awaited before the body runs.
        assert_eq!(
            check("async function f(event) { for (const a of await b) { event.currentTarget; } }"),
            [(1, 54, false)]
        );
    }

    #[test]
    fn leaves_reads_during_dispatch_and_other_values() {
        for code in [
            "element.addEventListener('click', event => {\n\tconst {currentTarget} = event;\n\tsetTimeout(() => {\n\t\tcurrentTarget.click();\n\t});\n});",
            "element.addEventListener('click', async event => {\n\tconst {currentTarget} = event;\n\tcurrentTarget.disabled = true;\n\tawait somePromise;\n\tcurrentTarget.disabled = false;\n});",
            "async function f(event) { await g(event.currentTarget); }",
            "async function f(item) { await g(); item.currentTarget; }",
            "async function f(event: number) { await g(); event.currentTarget; }",
            "const event = x; setTimeout(() => event.currentTarget);",
            "function f(event) { event.currentTarget; }",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
