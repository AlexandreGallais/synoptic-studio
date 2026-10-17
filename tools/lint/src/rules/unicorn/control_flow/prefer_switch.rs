//! `unicorn/prefer-switch`: an `if`/`else if` chain of three or more tests comparing the same
//! value with `===`, joined by `||` inside a test, becomes a `switch` (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/prefer-switch.md`, `minimumCases: 3`, `emptyDefaultCase:
//! 'no-default-comment'`). Reported from the first `if` to its branch. Autofixed as the plugin
//! writes it when the value is read without running code, no branch holds a `break` the `switch`
//! would catch, no comment would be lost and no case repeats; a member read is only reported, as
//! its getter could answer differently each time.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryExpression, BinaryOperator, Expression, IfStatement, LogicalOperator, Statement,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::expressions::{
    edited, has_comment_in, has_side_effect, line_indent, parent, same_reference, text,
};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferSwitch;

/// The rule's message.
const MESSAGE: &str = "Use `switch` instead of multiple `else-if`.";

/// The fewest cases reported.
const MINIMUM_CASES: usize = 3;

/// Whether an operand is a constant: a literal or `undefined`.
fn is_constant(expression: &Expression) -> bool {
    expression.is_literal()
        || matches!(expression, Expression::Identifier(name) if name.name == "undefined")
}

/// The `===` comparisons of a test joined by `||`, or none when another operand stands there.
fn equalities<'s, 'a>(test: &'s Expression<'a>, into: &mut Vec<&'s BinaryExpression<'a>>) -> bool {
    match test {
        Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::Or => {
            equalities(&logical.left, into) && equalities(&logical.right, into)
        }
        Expression::BinaryExpression(binary)
            if binary.operator == BinaryOperator::StrictEquality =>
        {
            into.push(binary);
            true
        }
        _ => false,
    }
}

/// The links of a chain comparing one value, and that value.
struct Chain<'s, 'a> {
    links: Vec<(&'s IfStatement<'a>, Vec<&'s BinaryExpression<'a>>)>,
    discriminant: Option<&'s Expression<'a>>,
}

/// The chain starting at an `if`, as long as its tests compare a common value.
fn chain<'s, 'a>(first: &'s IfStatement<'a>) -> Chain<'s, 'a> {
    let mut links = Vec::new();
    let mut candidates: Option<Vec<&Expression>> = None;
    let mut current = Some(first);
    while let Some(link) = current {
        let mut comparisons = Vec::new();
        if !equalities(&link.test, &mut comparisons) {
            break;
        }
        let mut kept: Vec<&Expression> = candidates.clone().unwrap_or_else(|| {
            [&comparisons[0].left, &comparisons[0].right]
                .into_iter()
                .filter(|each| !is_constant(each))
                .collect()
        });
        for comparison in &comparisons {
            kept.retain(|each| {
                same_reference(each, &comparison.left) || same_reference(each, &comparison.right)
            });
        }
        if kept.is_empty() {
            break;
        }
        candidates = Some(kept);
        links.push((link, comparisons));
        current = match &link.alternate {
            Some(Statement::IfStatement(next)) => Some(next),
            _ => None,
        };
    }
    Chain {
        discriminant: candidates.and_then(|each| each.first().copied()),
        links,
    }
}

/// The search of a branch for a `break` the `switch` would catch.
#[derive(Default)]
struct Breaks {
    depth: usize,
    found: bool,
}

impl<'a> Visit<'a> for Breaks {
    fn visit_break_statement(&mut self, stop: &oxc_ast::ast::BreakStatement<'a>) {
        self.found |= stop.label.is_none() && self.depth == 0;
    }

    fn visit_statement(&mut self, statement: &Statement<'a>) {
        let nested = matches!(
            statement,
            Statement::SwitchStatement(_)
                | Statement::WhileStatement(_)
                | Statement::DoWhileStatement(_)
                | Statement::ForStatement(_)
                | Statement::ForInStatement(_)
                | Statement::ForOfStatement(_)
        );
        self.depth += usize::from(nested);
        walk::walk_statement(self, statement);
        self.depth -= usize::from(nested);
    }
}

/// Whether a branch's end can fall through to the next case, so needs a `break`.
fn needs_break(statement: &Statement) -> bool {
    match statement {
        Statement::ReturnStatement(_) | Statement::ThrowStatement(_) => false,
        Statement::IfStatement(choice) => {
            choice.alternate.as_ref().is_none_or(needs_break) || needs_break(&choice.consequent)
        }
        Statement::BlockStatement(block) => block
            .body
            .iter()
            .rev()
            .find(|each| {
                !matches!(
                    each,
                    Statement::FunctionDeclaration(_) | Statement::EmptyStatement(_)
                )
            })
            .is_none_or(needs_break),
        _ => true,
    }
}

/// The key of a case's value, to find a repeated one.
fn case_key(source: &Source, value: &Expression) -> String {
    match value {
        Expression::StringLiteral(each) => format!("value:string:{}", each.value),
        Expression::NumericLiteral(each) => format!("value:number:{}", each.value),
        Expression::BooleanLiteral(each) => format!("value:boolean:{}", each.value),
        Expression::NullLiteral(_) => "value:object:null".to_owned(),
        other => format!("text:{}", text(source, other.span())),
    }
}

impl PreferSwitch {
    /// The fix of a chain, when it keeps every comment and no case repeats.
    fn fix(source: &Source, found: &Chain, discriminant: &Expression) -> Option<Fix> {
        let first = found.links[0].0;
        let indent = line_indent(source, first.span.start);
        let mut keys = Vec::new();
        let mut edits: Vec<(u32, u32, String)> = Vec::new();
        for (link, comparisons) in &found.links {
            let consequent = link.consequent.span();
            if has_comment_in(source, Span::new(link.span.start, consequent.start))
                || link.alternate.as_ref().is_some_and(|alternate| {
                    has_comment_in(source, Span::new(consequent.end, alternate.span().start))
                })
            {
                return None;
            }
            edits.push((link.span.start, consequent.start, String::new()));
            if let Some(alternate) = &link.alternate {
                edits.push((consequent.end, alternate.span().start, String::new()));
            }
            for comparison in comparisons {
                let value = if same_reference(&comparison.left, discriminant) {
                    &comparison.right
                } else {
                    &comparison.left
                };
                let key = case_key(source, value);
                if keys.contains(&key) {
                    return None;
                }
                keys.push(key);
                edits.push((
                    consequent.start,
                    consequent.start,
                    format!("\n{indent}case {}: ", text(source, value.span())),
                ));
            }
            if needs_break(&link.consequent) {
                match &link.consequent {
                    Statement::BlockStatement(block) => {
                        edits.push((
                            block.span.end - 1,
                            block.span.end - 1,
                            format!("\n{indent}break;\n{indent}"),
                        ));
                    }
                    other => {
                        edits.push((
                            other.span().start,
                            other.span().start,
                            format!("{{\n{indent}"),
                        ));
                        edits.push((
                            other.span().end,
                            other.span().end,
                            format!("\n{indent}break;\n{indent}}}"),
                        ));
                    }
                }
            }
        }
        let last = found.links[found.links.len() - 1].0;
        let ending = match &last.alternate {
            Some(alternate) => {
                edits.push((
                    alternate.span().start,
                    alternate.span().start,
                    format!("\n{indent}default: "),
                ));
                format!("\n{indent}}}")
            }
            None => format!("\n{indent}// No default\n{indent}}}"),
        };
        let body = edited(source, first.span, &mut edits);
        Some(Fix {
            start: first.span.start,
            end: first.span.end,
            text: format!(
                "switch ({}) {{{body}{ending}",
                text(source, discriminant.span())
            ),
        })
    }
}

impl Rule for PreferSwitch {
    fn name(&self) -> &'static str {
        "unicorn/prefer-switch"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::IfStatement(first) = node.kind() else {
            return;
        };
        if let AstKind::IfStatement(outer) = parent(source, node.id())
            && outer
                .alternate
                .as_ref()
                .is_some_and(|each| each.span() == first.span)
            && chain(outer).links.len() >= MINIMUM_CASES
        {
            return;
        }
        let found = chain(first);
        let Some(discriminant) = found.discriminant else {
            return;
        };
        if found.links.len() < MINIMUM_CASES {
            return;
        }
        let breaks = found.links.iter().any(|(link, _)| {
            let mut search = Breaks::default();
            search.visit_statement(&link.consequent);
            search.found
        });
        let getter = matches!(
            discriminant,
            Expression::StaticMemberExpression(_)
                | Expression::ComputedMemberExpression(_)
                | Expression::PrivateFieldExpression(_)
        );
        let fix = (!has_side_effect(discriminant) && !breaks && !getter)
            .then(|| Self::fix(source, &found, discriminant))
            .flatten();
        findings.push(match fix {
            Some(fix) => source.fixed(self.name(), first.span.start, MESSAGE.to_owned(), fix),
            None => source.finding(self.name(), first.span.start, MESSAGE.to_owned()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSwitch;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferSwitch, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn turns_a_chain_of_three_comparisons_into_a_switch() {
        // The documentation's case, written as the plugin's fix writes it.
        assert_eq!(
            check("if (foo === 1) {\n\ta();\n} else if (foo === 2) {\n\tb();\n} else if (foo === 3) {\n\tc();\n} else {\n\td();\n}"),
            (
                vec![(1, 1)],
                "switch (foo) {\ncase 1: {\n\ta();\n\nbreak;\n}\ncase 2: {\n\tb();\n\nbreak;\n}\ncase 3: {\n\tc();\n\nbreak;\n}\ndefault: {\n\td();\n}\n}".to_owned()
            )
        );
        // `||` in a test makes one case of two labels; a returning branch needs no `break`.
        assert_eq!(
            check("function f() {\n  if (a === 1 || a === 2) { return 1; } else if (3 === a) { g(); } else if (a === 4) h();\n}").1,
            "function f() {\n  switch (a) {\n  case 1: \n  case 2: { return 1; }\n  case 3: { g(); \n  break;\n  }\n  case 4: {\n  h();\n  break;\n  }\n  // No default\n  }\n}"
        );
    }

    #[test]
    fn leaves_short_or_mixed_chains() {
        for code in [
            "if (foo === 1) {} else if (foo === 2) {}",
            "if (foo === 1) {} else if (bar === 2) {} else if (foo === 3) {}",
            "if (foo === 1) {} else if (foo > 2) {} else if (foo === 3) {}",
            "if (foo == 1) {} else if (foo == 2) {} else if (foo == 3) {}",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        // The chain is reported once, from its first `if`.
        assert_eq!(
            check("if (a === 1) {} else if (a === 2) {} else if (a === 3) {} else if (a === 4) {}")
                .0,
            [(1, 1)]
        );
    }

    #[test]
    fn reports_without_fix_on_members_breaks_comments_or_repeats() {
        for code in [
            "if (foo.bar === 1) {} else if (foo.bar === 2) {} else if (foo.bar === 3) {}",
            "for (;;) { if (a === 1) { break; } else if (a === 2) {} else if (a === 3) {} }",
            "if (a === 1) {} /* why */ else if (a === 2) {} else if (a === 3) {}",
            "if (a === 1) {} else if (a === 1) {} else if (a === 3) {}",
        ] {
            let (places, result) = check(code);
            assert_eq!(places.len(), 1, "{code}");
            assert_eq!(result, code);
        }
    }
}
