//! `@angular-eslint/template/conditional-complexity`: no condition above `maxComplexity`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/conditional-complexity.ts`):
//! each binary operation and each conditional of a bound attribute's value, an interpolation,
//! an `@if`'s or `@else if`'s condition, a `@switch`'s value or a `@let`'s value counts 1 with
//! those of its operands, parentheses and its outer pipe looked through; above the limit, it is
//! reported over the expression. The project runs the plugin's default limit, 5.

use super::r3::{Literal, Ng, of_bound, unwrapped};
use super::{Report, Template, TemplateRule, bound3, pipe_span, template_attrs, trimmed};
use crate::template::bindings::{
    AttributeKind, Value, block_parameters, interpolations, read_attribute,
};
use crate::template::expressions::{Bound, read};
use crate::template::lexer::Span;
use crate::template::tree::Node;

/// The rule and its option.
pub struct ConditionalComplexity {
    /// The highest complexity allowed.
    pub max_complexity: u32,
}

impl ConditionalComplexity {
    /// The plugin's default, which the project runs.
    pub const PROJECT: Self = Self { max_complexity: 5 };
}

/// An expression with its parentheses and its outer pipe looked through
/// (`extractPossibleBinaryOrConditionalFrom`).
fn extract(node: &Ng) -> &Ng {
    match unwrapped(node) {
        Ng::Pipe(_, inner, _) => inner,
        other => other,
    }
}

/// The two operands of a binary operation, Angular's unary `-x` and `+x` being `0 - x`.
fn operands(node: &Ng) -> Option<(Ng, Ng)> {
    match node {
        Ng::Binary(_, left, right) => Some(((**left).clone(), (**right).clone())),
        Ng::Unary(_, inner) => Some((Ng::Literal(Literal::Number(0.0)), (**inner).clone())),
        _ => None,
    }
}

/// Whether an expression, parentheses aside, is a binary operation or a conditional.
fn counts(node: &Ng) -> bool {
    matches!(
        unwrapped(node),
        Ng::Binary(..) | Ng::Unary(..) | Ng::Conditional(..)
    )
}

/// The complexity of an expression (`getTotalComplexity`).
fn total(node: &Ng) -> u32 {
    let node = extract(node);
    if !counts(node) {
        return 0;
    }
    let mut sum = 1;
    if let Some((left, right)) = operands(node) {
        if counts(&left) {
            sum += total(&left);
        }
        if counts(&right) {
            sum += total(&right);
        }
    }
    if let Ng::Conditional(condition, yes, no) = node {
        sum += total(condition) + total(yes) + total(no);
    }
    sum
}

impl ConditionalComplexity {
    /// Reports an expression above the limit.
    fn report(&self, complexity: u32, span: Span, reports: &mut Vec<Report>) {
        if complexity > self.max_complexity {
            reports.push(Report {
                data: vec![
                    ("maxComplexity", self.max_complexity.to_string()),
                    ("totalComplexity", complexity.to_string()),
                ],
                ..Report::new(span, "conditionalComplexity")
            });
        }
    }

    /// A control flow's expression (`checkControlFlowExpression`), reported over itself.
    fn check_control_flow(&self, text: &str, bound: &Bound, reports: &mut Vec<Report>) {
        let complexity = total(extract(&of_bound(bound)));
        self.report(complexity, pipe_span(text, bound, false), reports);
    }

    /// The interpolations of a text or a value, each reported over itself.
    fn check_interpolations(&self, text: &str, bounds: &[Bound], reports: &mut Vec<Report>) {
        for bound in bounds {
            self.report(
                total(&of_bound(bound)),
                pipe_span(text, bound, false),
                reports,
            );
        }
    }
}

impl TemplateRule for ConditionalComplexity {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/conditional-complexity"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "conditionalComplexity",
            "The conditional complexity {{totalComplexity}} exceeds the defined limit {{maxComplexity}}",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let text = template.text;
        let allocator = template.allocator;
        template.walk(&mut |node, _| match node {
            Node::Element(element) => {
                for each in bound3(element) {
                    if each.role == super::Role::Output {
                        continue;
                    }
                    let Ok(read) = read_attribute(allocator, &element.name, each.attribute) else {
                        continue;
                    };
                    match (each.kind, read.value) {
                        (AttributeKind::Plain, Value::Interpolations(bounds)) => {
                            self.check_interpolations(text, &bounds, reports);
                        }
                        (_, Value::Binding(bound)) if bound.expression.is_some() => {
                            let span = each.attribute.value_span.unwrap_or(bound.span);
                            self.report(total(&of_bound(&bound)), span, reports);
                        }
                        _ => {}
                    }
                }
                for each in template_attrs(allocator, element) {
                    if let Some(bound) = &each.value {
                        let span = trimmed(text, bound.span);
                        self.report(total(&of_bound(bound)), span, reports);
                    }
                }
            }
            Node::Text(each) => {
                let found = interpolations(allocator, &each.tokens).unwrap_or_default();
                self.check_interpolations(text, &found, reports);
            }
            Node::Block(block) if matches!(block.name.as_str(), "if" | "else if" | "switch") => {
                if let Ok(parameters) = block_parameters(allocator, block)
                    && let Some(bound) = &parameters.expression
                {
                    self.check_control_flow(text, bound, reports);
                }
            }
            Node::LetDeclaration(declaration) => {
                if let Ok(bound) = read(allocator, &declaration.value, declaration.value_span.start)
                {
                    self.check_control_flow(text, &bound, reports);
                }
            }
            _ => {}
        });
    }
}

#[cfg(test)]
mod tests {
    use super::ConditionalComplexity;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("conditional-complexity.txt", &[], &|options| {
            Box::new(ConditionalComplexity {
                max_complexity: options
                    .and_then(|each| each.number("maxComplexity"))
                    .map_or(ConditionalComplexity::PROJECT.max_complexity, |each| {
                        format!("{each}").parse().unwrap_or(u32::MAX)
                    }),
            })
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 11 valid and 12 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (11, 12, 0));
    }

    #[test]
    fn f52_ac2_reports_a_condition_above_the_project_s_limit_of_5() {
        let found = |text: &str| {
            crate::template::lint_with("a.component.html", text, &[&ConditionalComplexity::PROJECT])
                .len()
        };

        // Each `&&` counts 1: five make 5, at the limit; six make 6, above it.
        assert_eq!(found("{{ a && b && c && d && e && f }}"), 0);
        assert_eq!(found("{{ a && b && c && d && e && f && g }}"), 1);
    }
}
