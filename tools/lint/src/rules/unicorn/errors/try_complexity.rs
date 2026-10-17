//! `unicorn/try-complexity`: a `try` block's cyclomatic complexity, counted as `ESLint`'s
//! `complexity` counts it — one, plus each `if`, loop, conditional expression, logical
//! expression, logical assignment, default value, `case` with a test, `catch` and optional call
//! or member read — may not exceed one, functions inside it left out (`eslint-plugin-unicorn`
//! 78.0.0, `docs/rules/try-complexity.md`, `max: 1`). A nested `try` adds to every enclosing
//! one. No fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{AssignmentOperator, BindingPattern, Expression, Statement};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct TryComplexity;

/// The highest complexity allowed.
const MAXIMUM: usize = 1;

/// The count of a block's decision points.
#[derive(Default)]
struct Decisions {
    count: usize,
}

impl<'a> Visit<'a> for Decisions {
    fn visit_statement(&mut self, statement: &Statement<'a>) {
        self.count += usize::from(matches!(
            statement,
            Statement::IfStatement(_)
                | Statement::ForStatement(_)
                | Statement::ForInStatement(_)
                | Statement::ForOfStatement(_)
                | Statement::WhileStatement(_)
                | Statement::DoWhileStatement(_)
        ));
        walk::walk_statement(self, statement);
    }

    fn visit_expression(&mut self, expression: &Expression<'a>) {
        self.count += usize::from(match expression {
            Expression::ConditionalExpression(_) | Expression::LogicalExpression(_) => true,
            Expression::AssignmentExpression(assignment) => matches!(
                assignment.operator,
                AssignmentOperator::LogicalAnd
                    | AssignmentOperator::LogicalOr
                    | AssignmentOperator::LogicalNullish
            ),
            Expression::CallExpression(call) => call.optional,
            Expression::StaticMemberExpression(member) => member.optional,
            Expression::ComputedMemberExpression(member) => member.optional,
            Expression::PrivateFieldExpression(member) => member.optional,
            _ => false,
        });
        walk::walk_expression(self, expression);
    }

    fn visit_chain_element(&mut self, element: &oxc_ast::ast::ChainElement<'a>) {
        self.count += usize::from(match element {
            oxc_ast::ast::ChainElement::CallExpression(call) => call.optional,
            oxc_ast::ast::ChainElement::StaticMemberExpression(member) => member.optional,
            oxc_ast::ast::ChainElement::ComputedMemberExpression(member) => member.optional,
            oxc_ast::ast::ChainElement::PrivateFieldExpression(member) => member.optional,
            oxc_ast::ast::ChainElement::TSNonNullExpression(_) => false,
        });
        walk::walk_chain_element(self, element);
    }

    fn visit_switch_case(&mut self, case: &oxc_ast::ast::SwitchCase<'a>) {
        self.count += usize::from(case.test.is_some());
        walk::walk_switch_case(self, case);
    }

    fn visit_catch_clause(&mut self, clause: &oxc_ast::ast::CatchClause<'a>) {
        self.count += 1;
        walk::walk_catch_clause(self, clause);
    }

    fn visit_binding_pattern(&mut self, pattern: &BindingPattern<'a>) {
        self.count += usize::from(matches!(pattern, BindingPattern::AssignmentPattern(_)));
        walk::walk_binding_pattern(self, pattern);
    }

    fn visit_formal_parameter(&mut self, parameter: &oxc_ast::ast::FormalParameter<'a>) {
        self.count += usize::from(parameter.initializer.is_some());
        walk::walk_formal_parameter(self, parameter);
    }

    fn visit_assignment_target_with_default(
        &mut self,
        target: &oxc_ast::ast::AssignmentTargetWithDefault<'a>,
    ) {
        self.count += 1;
        walk::walk_assignment_target_with_default(self, target);
    }

    fn visit_assignment_target_property_identifier(
        &mut self,
        target: &oxc_ast::ast::AssignmentTargetPropertyIdentifier<'a>,
    ) {
        self.count += usize::from(target.init.is_some());
        walk::walk_assignment_target_property_identifier(self, target);
    }

    fn visit_function(
        &mut self,
        _function: &oxc_ast::ast::Function<'a>,
        _flags: oxc_semantic::ScopeFlags,
    ) {
    }

    fn visit_arrow_function_expression(
        &mut self,
        _arrow: &oxc_ast::ast::ArrowFunctionExpression<'a>,
    ) {
    }

    fn visit_class(&mut self, _class: &oxc_ast::ast::Class<'a>) {}
}

impl Rule for TryComplexity {
    fn name(&self) -> &'static str {
        "unicorn/try-complexity"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TryStatement(attempt) = node.kind() else {
            return;
        };
        let mut decisions = Decisions::default();
        decisions.visit_block_statement(&attempt.block);
        let complexity = 1 + decisions.count;
        if complexity > MAXIMUM {
            findings.push(source.finding(
                self.name(),
                attempt.span.start,
                format!(
                    "Try block has a complexity of {complexity}. Maximum allowed is {MAXIMUM}."
                ),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TryComplexity;
    use crate::testing::{lint, messages};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&TryComplexity, "scripts/a.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_try_block_with_a_decision() {
        // The documentation's two cases.
        assert_eq!(
            places(
                "try {\n\tif (condition) {\n\t\tdoSomething();\n\t}\n} catch (error) {\n\thandleError(error);\n}"
            ),
            [(1, 1)]
        );
        let ternary = "try {\n\tconst value = condition ? a : b;\n} catch (error) {\n\thandleError(error);\n}";
        assert_eq!(
            messages(&lint(&TryComplexity, "scripts/a.ts", ternary)),
            ["Try block has a complexity of 2. Maximum allowed is 1."]
        );
        // Each decision point, and a nested `try` counted in both.
        for code in [
            "try { a && b; } catch {}",
            "try { a ??= b; } catch {}",
            "try { a?.b; } catch {}",
            "try { a?.(); } catch {}",
            "try { for (const b of c) {} } catch {}",
            "try { while (a) {} } catch {}",
            "try { switch (a) { case 1: break; default: } } catch {}",
            "try { const { a = 1 } = b; } catch {}",
            "try { const [a = 1] = b; } catch {}",
            "try { ({ a = 1 } = b); } catch {}",
        ] {
            assert_eq!(places(code), [(1, 1)], "{code}");
        }
        assert_eq!(
            places("try {\n  try { a(); } catch { b(); }\n} finally {}"),
            [(1, 1)]
        );
    }

    #[test]
    fn leaves_straight_blocks_and_functions_inside() {
        for code in [
            "try {\n\tdoSomething();\n\tdoSomethingElse();\n} catch (error) {\n\thandleError(error);\n}",
            "const value = condition ? a : b;\ntry {\n\tdoSomething(value);\n} catch (error) {\n\thandleError(error);\n}",
            "try { const f = () => a && b; function g(x = 1) { if (x) {} } } catch {}",
            "try { a(); } catch { if (b) {} } finally { c && d; }",
            "try { a = b; a.b(); switch (a) { default: } } catch {}",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
