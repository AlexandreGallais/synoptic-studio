//! `unicorn/no-prevent-default-in-passive-listener`: `preventDefault()` does nothing in a
//! listener registered with `{passive: true}` (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-prevent-default-in-passive-listener.md`). Only a listener written in the
//! `addEventListener(…)` call, its event its first parameter never reassigned, the call made
//! before the listener suspends. Reported only: `ESLint` only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, Expression};
use oxc_semantic::AstNode;

use super::events::{enclosing_function, option, registration, runs_late};
use super::nodes::{member_object, static_property_name};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{symbol_of, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoPreventDefaultInPassiveListener;

impl Rule for NoPreventDefaultInPassiveListener {
    fn name(&self) -> &'static str {
        "unicorn/no-prevent-default-in-passive-listener"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let callee = unwrap(&call.callee);
        if static_property_name(callee).as_deref() != Some("preventDefault") {
            return;
        }
        let Some(Expression::Identifier(event)) = member_object(callee).map(unwrap) else {
            return;
        };
        let Some(listener) = enclosing_function(source, node.id()) else {
            return;
        };
        let nodes = source.semantic.nodes();
        let (parameters, generator) = match nodes.kind(listener) {
            AstKind::Function(function) => (&function.params, function.generator),
            AstKind::ArrowFunctionExpression(arrow) => (&arrow.params, false),
            _ => return,
        };
        let Some(first) = parameters
            .items
            .first()
            .filter(|first| first.initializer.is_none())
        else {
            return;
        };
        let BindingPattern::BindingIdentifier(parameter) = &first.pattern else {
            return;
        };
        if generator {
            return;
        }
        let Some((_, options)) = registration(source, listener) else {
            return;
        };
        let passive = option(options, "passive")
            .flatten()
            .map(unwrap)
            .is_some_and(
                |value| matches!(value, Expression::BooleanLiteral(literal) if literal.value),
            );
        let symbol = symbol_of(source, event);
        if !passive || symbol.is_none() || symbol != parameter.symbol_id.get() {
            return;
        }
        let scoping = source.semantic.scoping();
        let written = symbol.is_some_and(|symbol| {
            scoping
                .get_resolved_reference_ids(symbol)
                .iter()
                .any(|reference| scoping.get_reference(*reference).is_write())
        });
        if written || runs_late(source, node.id(), listener, call.span.end) {
            return;
        }
        findings.push(source.finding(
            self.name(),
            call.span.start,
            "`preventDefault()` has no effect in a passive event listener.".to_owned(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoPreventDefaultInPassiveListener;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(
            &NoPreventDefaultInPassiveListener,
            "playground/a/b.ts",
            code,
        )
        .into_iter()
        .map(|each| (each.line, each.column))
        .collect()
    }

    #[test]
    fn reports_prevent_default_in_passive_listeners() {
        // The documentation's two cases.
        assert_eq!(
            check(
                "element.addEventListener('wheel', event => {\n\tevent.preventDefault();\n}, {passive: true});"
            ),
            [(2, 2)]
        );
        assert_eq!(
            check(
                "element.addEventListener('click', async event => {\n\tevent.preventDefault();\n\tawait save();\n}, {passive: true});"
            ),
            [(2, 2)]
        );
        // The last `passive` wins; wrappers and a function expression.
        assert_eq!(
            check(
                "addEventListener('x', function (e: Event) { (e as Event)['preventDefault'](); } as any, {passive: false, passive: true as const});"
            ),
            [(1, 45)]
        );
    }

    #[test]
    fn leaves_cancelable_listeners_and_unknown_cases() {
        for code in [
            "element.addEventListener('wheel', event => {\n\tevent.preventDefault();\n}, {passive: false});",
            "element.addEventListener('wheel', event => {\n\tconsole.log(event.deltaY);\n}, {passive: true});",
            "element.addEventListener('wheel', event => { event.preventDefault(); }, {passive: true, passive: false});",
            "element.addEventListener('wheel', event => { event.preventDefault(); }, {...o, passive: true});",
            "element.addEventListener('wheel', event => { event.preventDefault(); }, options);",
            "element.addEventListener('wheel', event => { event.preventDefault(); });",
            "element.addEventListener('wheel', async event => { await x; event.preventDefault(); }, {passive: true});",
            "element.addEventListener('wheel', event => { setTimeout(() => event.preventDefault()); }, {passive: true});",
            "element.addEventListener('wheel', ({preventDefault}) => { preventDefault(); }, {passive: true});",
            "element.addEventListener('wheel', (a, event) => { event.preventDefault(); }, {passive: true});",
            "element.addEventListener('wheel', event => { event = other; event.preventDefault(); }, {passive: true});",
            "element.on('wheel', event => { event.preventDefault(); }, {passive: true});",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
