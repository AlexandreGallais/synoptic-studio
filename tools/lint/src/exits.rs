//! Whether a statement always leaves the code that follows it: a `return`, a `throw`, a `break`
//! or a `continue` out of it, a call to the global `process.exit()`, or statements whose every
//! path ends so. `eslint-plugin-unicorn` 78.0.0 asks `ESLint`'s code paths (`is-branch-exit.js`);
//! this reads the tree: a block leaves when one of its statements does, an `if` when both of its
//! branches do, a `switch` with a `default` when every case leaves and none breaks out of it, a
//! `try` when its block and its handler leave or its `finally` does, and an endless loop when
//! nothing breaks out of it (EN-079).

use oxc_ast::ast::{Expression, Statement};
use oxc_ast_visit::{Visit, walk};

/// What a `break` or a `continue` would leave, from where it is written.
#[derive(Clone, Copy, Default)]
struct Within {
    /// Inside a loop or a `switch` of the statement: an unlabeled `break` stays inside.
    breakable: bool,
    /// Inside a loop of the statement: an unlabeled `continue` stays inside.
    looping: bool,
}

/// Whether a statement always leaves the code that follows it.
#[must_use]
pub fn always_exits(statement: &Statement) -> bool {
    exits(statement, Within::default())
}

/// Whether a call is `process.exit(…)`; a local named `process` is not told apart.
#[must_use]
pub fn is_process_exit(expression: &Expression) -> bool {
    let Expression::CallExpression(call) = expression.get_inner_expression() else {
        return false;
    };
    let Some(member) = call.callee.get_member_expr() else {
        return false;
    };
    !call.optional
        && !member.optional()
        && member
            .static_property_name()
            .is_some_and(|name| name == "exit")
        && member.object().is_specific_id("process")
}

/// Whether a statement leaves, from where it is written.
fn exits(statement: &Statement, within: Within) -> bool {
    match statement {
        Statement::ReturnStatement(_) | Statement::ThrowStatement(_) => true,
        Statement::BreakStatement(stop) => stop.label.is_some() || !within.breakable,
        Statement::ContinueStatement(stop) => stop.label.is_some() || !within.looping,
        Statement::ExpressionStatement(expression) => is_process_exit(&expression.expression),
        Statement::BlockStatement(block) => block.body.iter().any(|each| exits(each, within)),
        Statement::IfStatement(choice) => {
            choice
                .alternate
                .as_ref()
                .is_some_and(|alternate| exits(alternate, within))
                && exits(&choice.consequent, within)
        }
        Statement::TryStatement(attempt) => {
            let finally = attempt
                .finalizer
                .as_ref()
                .is_some_and(|block| block.body.iter().any(|each| exits(each, within)));
            let block = attempt.block.body.iter().any(|each| exits(each, within));
            let handled = attempt
                .handler
                .as_ref()
                .is_none_or(|handler| handler.body.body.iter().any(|each| exits(each, within)));
            finally || (block && handled)
        }
        Statement::SwitchStatement(switch) => {
            let inside = Within {
                breakable: true,
                ..within
            };
            switch.cases.iter().any(|case| case.test.is_none())
                && switch
                    .cases
                    .last()
                    .is_some_and(|last| last.consequent.iter().any(|each| exits(each, inside)))
                && switch.cases.iter().all(|case| {
                    case.consequent.is_empty()
                        || case.consequent.iter().any(|each| exits(each, inside))
                })
                && !breaks_out(statement)
        }
        Statement::WhileStatement(_) | Statement::ForStatement(_) => {
            is_endless(statement) && !breaks_out(statement)
        }
        Statement::LabeledStatement(labeled) => exits(&labeled.body, within),
        _ => false,
    }
}

/// Whether a loop's test is missing or `true`.
fn is_endless(statement: &Statement) -> bool {
    let test = match statement {
        Statement::WhileStatement(each) => Some(&each.test),
        Statement::ForStatement(each) => each.test.as_ref(),
        _ => return false,
    };
    test.is_none_or(|test| matches!(test.get_inner_expression(), Expression::BooleanLiteral(literal) if literal.value))
}

/// Whether an unlabeled `break` leaves a loop or a `switch`, functions and nested loops and
/// `switch`es left out.
fn breaks_out(statement: &Statement) -> bool {
    let mut search = BreakSearch {
        depth: 0,
        found: false,
    };
    match statement {
        Statement::SwitchStatement(switch) => {
            for case in &switch.cases {
                for each in &case.consequent {
                    search.visit_statement(each);
                }
            }
        }
        Statement::WhileStatement(each) => search.visit_statement(&each.body),
        Statement::ForStatement(each) => search.visit_statement(&each.body),
        _ => {}
    }
    search.found
}

/// The search for a `break` of the statement searched.
struct BreakSearch {
    /// How many loops or `switch`es deep the walk is, inside the statement.
    depth: usize,
    /// Whether one was found.
    found: bool,
}

impl<'a> Visit<'a> for BreakSearch {
    fn visit_break_statement(&mut self, stop: &oxc_ast::ast::BreakStatement<'a>) {
        if stop.label.is_none() && self.depth == 0 {
            self.found = true;
        }
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
        if nested {
            self.depth += 1;
        }
        walk::walk_statement(self, statement);
        if nested {
            self.depth -= 1;
        }
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
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    use super::always_exits;

    /// Whether the first statement of a piece of code always exits.
    fn first_exits(code: &str) -> bool {
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, code, SourceType::ts()).parse();

        parsed.program.body.first().is_some_and(always_exits)
    }

    #[test]
    fn reads_the_statements_that_leave() {
        for code in [
            "return;",
            "throw new Error();",
            "{ a(); return 1; }",
            "if (a) { return; } else { throw b; }",
            "process.exit(1);",
            "try { return; } catch { return; }",
            "try { a(); } finally { return; }",
            "switch (a) { case 1: return 1; case 2: case 3: throw b; default: return 2; }",
            "while (true) { a(); }",
            "for (;;) { for (;;) { break; } }",
            "x: { break x; }",
            "break;",
            "continue;",
        ] {
            assert!(first_exits(code), "{code}");
        }
    }

    #[test]
    fn reads_the_statements_that_may_go_on() {
        for code in [
            "a();",
            "if (a) { return; }",
            "if (a) { return; } else { b(); }",
            "try { return; } catch { b(); }",
            "switch (a) { case 1: return 1; }",
            "switch (a) { case 1: if (b) { break; } return 1; default: return 2; }",
            "switch (a) { case 1: b(); default: return 2; case 2: }",
            "while (true) { if (a) { break; } }",
            "while (a) { return; }",
            "for (const b of a) { break; }",
            "do { return; } while (a);",
            "other.exit(1);",
            "process.exit?.(1);",
        ] {
            assert!(!first_exits(code), "{code}");
        }
        assert!(!first_exits(""));
    }

    #[test]
    fn leaves_a_break_or_continue_to_a_loop_inside() {
        assert!(!first_exits(
            "switch (a) { case 1: while (b) { continue; } default: { for (;;) { break; } } }"
        ));
        assert!(first_exits(
            "while (true) { const f = () => { for (;;) { break; } }; }"
        ));
    }
}
