//! `unicorn/require-passive-events`: a listener of `touchstart`, `touchmove`, `touchenter`,
//! `touchend`, `touchleave`, `wheel` or `mousewheel` that never cancels its event is registered
//! with `{passive: true}`, so that the browser scrolls without waiting for it
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/require-passive-events.md`). Only inline
//! listeners that read their event's properties alone, `preventDefault` aside. Autofixed.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, BindingPattern, Expression, ObjectExpression, ObjectPropertyKind};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use super::nodes::{plain_method_call_at_least, static_string};
use crate::expressions::{has_comment_in, is_written, line_indent};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequirePassiveEvents;

/// The events whose listeners should be passive.
const PASSIVE_EVENTS: [&str; 7] = [
    "touchstart",
    "touchmove",
    "touchenter",
    "touchend",
    "touchleave",
    "wheel",
    "mousewheel",
];

/// Whether a `function` reads `arguments` outside its nested functions.
fn reads_arguments(source: &Source, function: NodeId, span: Span) -> bool {
    source.semantic.nodes().iter().any(|node| {
        matches!(node.kind(), AstKind::IdentifierReference(name)
            if name.name == "arguments" && name.span.start >= span.start && name.span.end <= span.end)
            && source
                .semantic
                .nodes()
                .ancestors(node.id())
                .find(|ancestor| matches!(ancestor.kind(), AstKind::Function(_)))
                .is_some_and(|ancestor| ancestor.id() == function)
    })
}

/// Whether a reference to the event reads one of its properties, `preventDefault` aside, its
/// whole member chain left unwritten.
fn is_safe_read(source: &Source, reference: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(reference);
    let object_span = nodes.kind(reference).span();
    let name = match parent.kind() {
        AstKind::StaticMemberExpression(member) if member.object.span() == object_span => {
            Some(member.property.name.to_string())
        }
        AstKind::ComputedMemberExpression(member) if member.object.span() == object_span => {
            match &member.expression {
                Expression::NumericLiteral(number) => Some(number.value.to_string()),
                key => static_string(key),
            }
        }
        _ => None,
    };
    if name.is_none_or(|name| name == "preventDefault") {
        return false;
    }
    let mut outer = parent;
    loop {
        let next = nodes.parent_node(outer.id());
        let inner = outer.kind().span();
        let is_object = match next.kind() {
            AstKind::StaticMemberExpression(member) => member.object.span() == inner,
            AstKind::ComputedMemberExpression(member) => member.object.span() == inner,
            _ => false,
        };
        if !is_object {
            break;
        }
        outer = next;
    }
    !is_written(source, outer)
}

/// Whether a listener's event is only read for its properties.
fn is_event_safe(source: &Source, listener: &Expression) -> bool {
    let parameters = match listener {
        Expression::FunctionExpression(function) => {
            let Some(body) = &function.body else {
                return false;
            };
            if reads_arguments(source, function.node_id.get(), body.span) {
                return false;
            }
            &function.params
        }
        Expression::ArrowFunctionExpression(arrow) => &arrow.params,
        _ => return false,
    };
    let Some(first) = parameters.items.first() else {
        return parameters.rest.is_none();
    };
    let BindingPattern::BindingIdentifier(event) = &first.pattern else {
        return false;
    };
    if first.initializer.is_some() {
        return false;
    }
    let Some(symbol) = event.symbol_id.get() else {
        return false;
    };
    let scoping = source.semantic.scoping();
    scoping
        .get_resolved_reference_ids(symbol)
        .iter()
        .all(|reference| is_safe_read(source, scoping.get_reference(*reference).node_id()))
}

/// The fix of an options' object without `passive`.
fn add_passive(source: &Source, options: &ObjectExpression) -> Option<Fix> {
    let Some(last) = options.properties.last() else {
        return Some(Fix {
            start: options.span.start,
            end: options.span.end,
            text: "{passive: true}".to_owned(),
        });
    };
    let last = last.span();
    if has_comment_in(source, Span::new(last.end, options.span.end)) {
        return None;
    }
    let single_line = source.line(options.span.start) == source.line(options.span.end);
    if single_line {
        return Some(Fix {
            start: last.end,
            end: last.end,
            text: ", passive: true".to_owned(),
        });
    }
    let indent = line_indent(source, last.start);
    let after = &source.text[last.end as usize..];
    let blanks = after.len() - after.trim_start().len();
    if after.trim_start().starts_with(',') {
        let comma = last.end + u32::try_from(blanks).unwrap_or(0) + 1;
        Some(Fix {
            start: comma,
            end: comma,
            text: format!("\n{indent}passive: true,"),
        })
    } else {
        Some(Fix {
            start: last.end,
            end: last.end,
            text: format!(",\n{indent}passive: true"),
        })
    }
}

/// A problem of a listener's options.
enum Problem {
    /// Fixed so.
    Fixed(Fix),
    /// Reported only.
    Reported,
}

/// The problem of a listener's options, when they are not passive.
fn options_fix(
    source: &Source,
    listener: &Expression,
    options: Option<&Expression>,
) -> Option<Problem> {
    let Some(options) = options else {
        let end = listener.span().end;
        return Some(Problem::Fixed(Fix {
            start: end,
            end,
            text: ", {passive: true}".to_owned(),
        }));
    };
    match options {
        Expression::BooleanLiteral(capture) => Some(Problem::Fixed(Fix {
            start: capture.span.start,
            end: capture.span.end,
            text: if capture.value {
                "{capture: true, passive: true}".to_owned()
            } else {
                "{passive: true}".to_owned()
            },
        })),
        Expression::NullLiteral(_) => Some(Problem::Reported),
        Expression::Identifier(name) if name.name == "undefined" => Some(Problem::Reported),
        Expression::ObjectExpression(object) => {
            let mut passive = None;
            for property in &object.properties {
                let ObjectPropertyKind::ObjectProperty(property) = property else {
                    return None;
                };
                if property.computed {
                    return None;
                }
                if property.key.is_specific_static_name("passive") {
                    passive = Some(&property.value);
                }
            }
            match passive {
                None => Some(add_passive(source, object).map_or(Problem::Reported, Problem::Fixed)),
                Some(Expression::BooleanLiteral(value)) if !value.value => {
                    Some(Problem::Fixed(Fix {
                        start: value.span.start,
                        end: value.span.end,
                        text: "true".to_owned(),
                    }))
                }
                Some(_) => None,
            }
        }
        _ => None,
    }
}

impl Rule for RequirePassiveEvents {
    fn name(&self) -> &'static str {
        "unicorn/require-passive-events"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        if plain_method_call_at_least(call, &["addEventListener"], 2).is_none()
            || call.arguments.len() > 3
            || call.arguments.iter().any(Argument::is_spread)
        {
            return;
        }
        let Some(Expression::StringLiteral(event)) = call.arguments[0].as_expression() else {
            return;
        };
        let Some(listener) = call.arguments[1].as_expression() else {
            return;
        };
        if !event
            .value
            .as_str()
            .is_some_and(|name| PASSIVE_EVENTS.contains(&name))
            || !is_event_safe(source, listener)
        {
            return;
        }
        let options = call.arguments.get(2).and_then(Argument::as_expression);
        let Some(fix) = options_fix(source, listener, options) else {
            return;
        };
        let message = "Use `{passive: true}` for this high-frequency event listener.".to_owned();
        findings.push(match fix {
            Problem::Fixed(fix) => source.fixed(self.name(), event.span.start, message, fix),
            Problem::Reported => source.finding(self.name(), event.span.start, message),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::RequirePassiveEvents;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&RequirePassiveEvents, "playground/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn makes_high_frequency_listeners_passive() {
        // The documentation's case, then the forms of the options; the fixed text is ESLint's.
        assert_eq!(
            check("window.addEventListener('wheel', () => {});\nel.addEventListener('touchstart', e => use(e.touches), true);\nel.addEventListener('touchmove', e => use(e['touches']), false);\nel.addEventListener('touchend', e => e.x, null);\nel.addEventListener('wheel', e => { e.deltaY; }, {once: true});\nel.addEventListener('wheel', e => { e.deltaY; }, {});\nel.addEventListener('wheel', e => { e.deltaY; }, {\n\tonce: true,\n});\nel.addEventListener('wheel', e => { e.deltaY; }, {\n\tonce: true\n});\nel.addEventListener('wheel', e => { e.deltaY; }, {passive: false});\nel.addEventListener('wheel', e => { e.deltaY; }, {once: true /* c */});"),
            (
                vec![(1, 25), (2, 21), (3, 21), (4, 21), (5, 21), (6, 21), (7, 21), (10, 21), (13, 21), (14, 21)],
                "window.addEventListener('wheel', () => {}, {passive: true});\nel.addEventListener('touchstart', e => use(e.touches), {capture: true, passive: true});\nel.addEventListener('touchmove', e => use(e['touches']), {passive: true});\nel.addEventListener('touchend', e => e.x, null);\nel.addEventListener('wheel', e => { e.deltaY; }, {once: true, passive: true});\nel.addEventListener('wheel', e => { e.deltaY; }, {passive: true});\nel.addEventListener('wheel', e => { e.deltaY; }, {\n\tonce: true,\n\tpassive: true,\n});\nel.addEventListener('wheel', e => { e.deltaY; }, {\n\tonce: true,\n\tpassive: true\n});\nel.addEventListener('wheel', e => { e.deltaY; }, {passive: true});\nel.addEventListener('wheel', e => { e.deltaY; }, {once: true /* c */});"
                    .to_owned()
            )
        );
    }

    #[test]
    fn leaves_cancelling_or_unknown_listeners() {
        for code in [
            "window.addEventListener('wheel', () => {}, {passive: true});",
            "window.addEventListener('wheel', event => {\n\tevent.preventDefault();\n});",
            "window.addEventListener('click', () => {});",
            "el.addEventListener('wheel', e => { e.deltaY; }, {passive: x});",
            "el.addEventListener('wheel', e => { e.deltaY; }, {...o});",
            "el.addEventListener('wheel', e => { e.deltaY; }, opts);",
            "el.addEventListener('wheel', e => { use(e); });",
            "el.addEventListener('wheel', e => { e.a.b = 1; });",
            "el.addEventListener('wheel', function () { use(arguments); });",
            "el.addEventListener('wheel', ({ deltaY }) => deltaY);",
            "el.addEventListener('wheel', handler);",
            "el?.addEventListener('wheel', () => {});",
            "el.addEventListener(`wheel`, () => {});",
        ] {
            assert_eq!(check(code).0, Vec::new(), "{code}");
        }
        assert_eq!(
            check("el.addEventListener('wheel', function () { const f = function () { arguments; }; });").0,
            [(1, 21)]
        );
    }
}
