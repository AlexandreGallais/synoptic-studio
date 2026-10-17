//! `unicorn/no-late-event-control`: `preventDefault()`, `stopPropagation()` and
//! `stopImmediatePropagation()` do nothing once the handler has suspended, inside a function it
//! defers, or in a generator handler, whose body does not run during dispatch
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-late-event-control.md`). The event is a
//! handler's parameter named `e`, `event`, `evt` or `…Event`, or annotated with an event type,
//! unless its annotation says it is no event.

use oxc_ast::AstKind;
use oxc_ast::ast::Expression;
use oxc_semantic::AstNode;

use super::events::{enclosing_function, event_class, handler_of, is_event_name, runs_late};
use super::nodes::{member_object, static_property_name};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::annotations::Class;
use crate::rules::unicorn::syntax::unwrap;
use crate::source::Source;

/// The rule.
pub struct NoLateEventControl;

impl Rule for NoLateEventControl {
    fn name(&self) -> &'static str {
        "unicorn/no-late-event-control"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(method) = static_property_name(&call.callee).filter(|name| {
            matches!(
                name.as_str(),
                "preventDefault" | "stopImmediatePropagation" | "stopPropagation"
            )
        }) else {
            return;
        };
        let Some(Expression::Identifier(event)) = member_object(&call.callee).map(unwrap) else {
            return;
        };
        let Some((handler, symbol)) = handler_of(source, event) else {
            return;
        };
        let class = event_class(source, symbol);
        let declared = source.semantic.scoping().symbol_name(symbol);
        if matches!(class, Class::NonTarget | Class::Nullish)
            || !(is_event_name(declared) || class == Class::Target)
        {
            return;
        }
        let nodes = source.semantic.nodes();
        let generator =
            matches!(nodes.kind(handler), AstKind::Function(function) if function.generator);
        let nested = enclosing_function(source, node.id()) != Some(handler);
        if !nested && !generator && !runs_late(source, node.id(), handler, call.span.end) {
            return;
        }
        let name = &event.name;
        let message = if nested {
            format!(
                "`{name}.{method}()` may run after synchronous event dispatch has finished. Call it in the outer handler before deferring work."
            )
        } else if generator {
            format!(
                "`{name}.{method}()` is not called during event dispatch inside a generator function. Use a normal function handler instead."
            )
        } else {
            format!(
                "`{name}.{method}()` has no effect after the handler suspends. Call it before `await` or `yield` while the event is still being dispatched."
            )
        };
        findings.push(source.finding(self.name(), call.span.start, message));
    }
}

#[cfg(test)]
mod tests {
    use super::NoLateEventControl;
    use crate::testing::lint;

    /// The places of the findings, with the first word of their reason.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&NoLateEventControl, "playground/a/b.ts", code)
            .into_iter()
            .map(|each| {
                let reason = ["after the handler", "generator", "may run after"]
                    .into_iter()
                    .find(|reason| each.message.contains(reason))
                    .unwrap_or_default();
                (each.line, each.column, reason.to_owned())
            })
            .collect()
    }

    #[test]
    fn reports_late_calls() {
        // The documentation's two cases.
        assert_eq!(
            check(
                "element.addEventListener('click', async event => {\n\tawait save();\n\tevent.preventDefault();\n});"
            ),
            [(3, 2, "after the handler".to_owned())]
        );
        assert_eq!(
            check(
                "element.addEventListener('click', event => {\n\tsetTimeout(() => {\n\t\tevent.stopPropagation();\n\t});\n});"
            ),
            [(3, 3, "may run after".to_owned())]
        );
        // A generator handler, a typed name, a computed method, an argument that awaits.
        assert_eq!(
            check(
                "function* g(e) { e.stopImmediatePropagation(); }\nasync function f(x: KeyboardEvent) { x['preventDefault'](await y); }"
            ),
            [
                (1, 18, "generator".to_owned()),
                (2, 38, "after the handler".to_owned())
            ]
        );
    }

    #[test]
    fn leaves_calls_during_dispatch() {
        for code in [
            "element.addEventListener('click', async event => {\n\tevent.preventDefault();\n\tawait save();\n});",
            "element.addEventListener('click', event => {\n\tevent.stopPropagation();\n\tsetTimeout(() => {\n\t\tupdate();\n\t});\n});",
            "async function f(item) { await g(); item.preventDefault(); }",
            "async function f(event: string) { await g(); event.preventDefault(); }",
            "async function f(event) { await g(); event.preventDefault; }",
            "async function f(event) { await g(); event.focus(); }",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
