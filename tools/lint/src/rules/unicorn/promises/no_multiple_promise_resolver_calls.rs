//! `unicorn/no-multiple-promise-resolver-calls`: a call to a `new Promise()` executor's `resolve`
//! or `reject` that a call to either may already have run before, on the same path
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-multiple-promise-resolver-calls.md`). The
//! plugin asks `ESLint`'s code paths; this follows the tree of each function holding a call — the
//! executor and the functions in it, each a path of its own — with two marks per path, "may not
//! have been called" and "may have been called", merged where branches meet: `if`, `switch`,
//! `&&`, `||`, `??`, `?:`, `?.`, loops run again until nothing changes, `try` whose `catch`
//! receives the marks of every call, `new`, member read, `throw`, `yield` or awaited name before
//! it, and `return`, `throw`, `break`, `continue` and `process.exit()` ending a path. A body under
//! a literal false test never runs. The resolvers are the executor's first two parameters,
//! plain names never reassigned. No fix.

use std::collections::BTreeSet;

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrowFunctionExpression, AwaitExpression, BindingPattern, CallExpression, Class,
    ComputedMemberExpression, DoWhileStatement, Expression, ForStatement, ForStatementInit,
    Function, IfStatement, LabeledStatement, NewExpression, PrivateFieldExpression, Statement,
    StaticMemberExpression, SwitchStatement, TryStatement, VariableDeclaration, YieldExpression,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::{AstNode, ScopeFlags, SymbolId};
use oxc_span::GetSpan;

use super::calls::{Body, promise_executor};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{Value, static_value, symbol_of, unwrap};
use crate::source::Source;

/// The rule.
pub struct NoMultiplePromiseResolverCalls;

/// The rule's message.
const MESSAGE: &str =
    "Promise executor resolver functions may be called multiple times on the same execution path.";

/// The marks of the paths reaching a point; neither when none does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct State {
    /// A path where no resolver was called reaches it.
    uncalled: bool,
    /// A path where a resolver was called reaches it.
    called: bool,
}

impl State {
    /// No path.
    const DEAD: Self = Self {
        uncalled: false,
        called: false,
    };

    /// The start of a function.
    const START: Self = Self {
        uncalled: true,
        called: false,
    };

    /// The paths of both.
    fn merge(self, other: Self) -> Self {
        Self {
            uncalled: self.uncalled || other.uncalled,
            called: self.called || other.called,
        }
    }
}

/// A statement `break` or `continue` may leave.
struct Frame<'a> {
    /// Its label.
    label: Option<&'a str>,
    /// Whether it is a loop, which an unlabeled `continue` goes back to.
    is_loop: bool,
    /// Whether an unlabeled `break` leaves it: a loop or a `switch`.
    breakable: bool,
    /// The paths leaving it by `break`.
    breaks: State,
    /// The paths going back to its start by `continue`.
    continues: State,
}

impl<'a> Frame<'a> {
    /// A statement no path has left yet.
    const fn new(label: Option<&'a str>, is_loop: bool, breakable: bool) -> Self {
        Self {
            label,
            is_loop,
            breakable,
            breaks: State::DEAD,
            continues: State::DEAD,
        }
    }
}

/// The walk of one function's body.
struct Paths<'s, 'b, 'a> {
    /// The file.
    source: &'s Source<'b, 'a>,
    /// The executor's resolvers.
    resolvers: &'s [SymbolId],
    /// The paths reaching the current point.
    state: State,
    /// The paths each enclosing `catch` receives, the innermost last.
    catches: Vec<State>,
    /// The statements `break` and `continue` may leave, the innermost last.
    frames: Vec<Frame<'a>>,
    /// The starts of the calls reported.
    reports: BTreeSet<u32>,
}

/// Whether an expression is a literal whose value is false.
fn is_false_literal(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::BooleanLiteral(literal) => !literal.value,
        Expression::NumericLiteral(literal) => literal.value == 0.0 || literal.value.is_nan(),
        Expression::StringLiteral(literal) => literal.value.is_empty(),
        Expression::NullLiteral(_) => true,
        _ => false,
    }
}

/// Whether an expression is a literal whose value is true.
fn is_true_literal(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::BooleanLiteral(literal) => literal.value,
        Expression::NumericLiteral(literal) => literal.value != 0.0 && !literal.value.is_nan(),
        Expression::StringLiteral(literal) => !literal.value.is_empty(),
        Expression::RegExpLiteral(_) => true,
        _ => false,
    }
}

/// Whether a statement is only `return;`.
fn is_bare_return(statement: &Statement) -> bool {
    match statement {
        Statement::ReturnStatement(stop) => stop.argument.is_none(),
        Statement::BlockStatement(block) => {
            matches!(block.body.as_slice(), [only] if is_bare_return(only))
        }
        _ => false,
    }
}

impl<'a> Paths<'_, '_, 'a> {
    /// The paths that may throw here go to the innermost `catch`.
    fn throws(&mut self) {
        if let Some(caught) = self.catches.last_mut() {
            *caught = caught.merge(self.state);
        }
    }

    /// Whether a call is to a resolver.
    fn is_resolver_call(&self, call: &CallExpression) -> bool {
        matches!(unwrap(&call.callee), Expression::Identifier(name)
            if symbol_of(self.source, name).is_some_and(|symbol| self.resolvers.contains(&symbol)))
    }

    /// The statements in order.
    fn statements(&mut self, statements: &[Statement<'a>]) {
        for statement in statements {
            self.statement(statement, None);
        }
    }

    /// The paths through a loop's body run again until they change no more, then leaving by
    /// its test or a `break`.
    fn repeat(
        &mut self,
        label: Option<&'a str>,
        body: &Statement<'a>,
        mut test: impl FnMut(&mut Self),
        exits_by_test: bool,
    ) {
        let entry = self.state;
        let mut head = entry;
        let mut leaving = State::DEAD;
        loop {
            self.state = head;
            test(self);
            if exits_by_test {
                leaving = leaving.merge(self.state);
            }
            self.frames.push(Frame::new(label, true, true));
            self.statement(body, None);
            let frame = self.frames.pop().unwrap_or(Frame::new(label, true, true));
            leaving = leaving.merge(frame.breaks);
            let next = entry.merge(self.state).merge(frame.continues);
            if next == head {
                break;
            }
            head = next;
        }
        self.state = leaving;
    }

    /// An `if`: both branches from its test, a literal false test skipping the first, a literal
    /// true test before a bare `return` ending the path.
    fn choice(&mut self, choice: &IfStatement<'a>) {
        self.visit_expression(&choice.test);
        let before = self.state;
        if is_false_literal(&choice.test) {
            if let Some(alternate) = &choice.alternate {
                self.statement(alternate, None);
            }
            return;
        }
        self.statement(&choice.consequent, None);
        let after_consequent = self.state;
        self.state = before;
        if let Some(alternate) = &choice.alternate {
            self.statement(alternate, None);
        }
        self.state = self.state.merge(after_consequent);
        if is_true_literal(&choice.test) && is_bare_return(&choice.consequent) {
            self.state = State::DEAD;
        }
    }

    /// A `do … while`: its body once, then again while its test is not literally false.
    fn do_while(&mut self, each: &DoWhileStatement<'a>, label: Option<&'a str>) {
        self.frames.push(Frame::new(label, true, true));
        let entry = self.state;
        let mut head = entry;
        let looping = !is_false_literal(&each.test);
        let mut after_test;
        loop {
            self.state = head;
            self.statement(&each.body, None);
            if let Some(frame) = self.frames.last_mut() {
                self.state = self.state.merge(frame.continues);
                frame.continues = State::DEAD;
            }
            self.visit_expression(&each.test);
            after_test = self.state;
            let next = entry.merge(after_test);
            if !looping || next == head {
                break;
            }
            head = next;
        }
        let breaks = self.frames.pop().map_or(State::DEAD, |frame| frame.breaks);
        self.state = if is_true_literal(&each.test) {
            breaks
        } else {
            after_test.merge(breaks)
        };
    }

    /// A `for`: its start, then its test, body and update repeated; no body under a literal false
    /// test.
    fn for_loop(&mut self, each: &ForStatement<'a>, label: Option<&'a str>) {
        match &each.init {
            Some(ForStatementInit::VariableDeclaration(declaration)) => {
                self.declarations(declaration);
            }
            Some(init) => {
                if let Some(init) = init.as_expression() {
                    self.visit_expression(init);
                }
            }
            None => {}
        }
        if let Some(test) = each.test.as_ref().filter(|test| is_false_literal(test)) {
            self.visit_expression(test);
            return;
        }
        let endless = each.test.as_ref().is_none_or(is_true_literal);
        let update = each.update.as_ref();
        let test = each.test.as_ref();
        let mut first = true;
        self.repeat(
            label,
            &each.body,
            |paths| {
                if !first && let Some(update) = update {
                    paths.visit_expression(update);
                }
                first = false;
                if let Some(test) = test {
                    paths.visit_expression(test);
                }
            },
            !endless,
        );
    }

    /// A `switch`: each case from the tests before it and the case falling into it.
    fn switch(&mut self, choice: &SwitchStatement<'a>, label: Option<&'a str>) {
        self.visit_expression(&choice.discriminant);
        self.frames.push(Frame::new(label, false, true));
        let mut falling = State::DEAD;
        let mut tested = self.state;
        for case in &choice.cases {
            self.state = tested;
            if let Some(test) = &case.test {
                self.visit_expression(test);
                tested = self.state;
            }
            self.state = self.state.merge(falling);
            self.statements(&case.consequent);
            falling = self.state;
        }
        let breaks = self.frames.pop().map_or(State::DEAD, |frame| frame.breaks);
        let unmatched = if choice.cases.iter().any(|case| case.test.is_none()) {
            State::DEAD
        } else {
            tested
        };
        self.state = falling.merge(breaks).merge(unmatched);
    }

    /// A `try`: its block, its `catch` from every path that may throw in the block, then its
    /// `finally`.
    fn attempt(&mut self, attempt: &TryStatement<'a>) {
        if attempt.handler.is_some() {
            self.catches.push(State::DEAD);
        }
        self.statements(&attempt.block.body);
        let mut after = self.state;
        if let Some(handler) = &attempt.handler {
            self.state = self.catches.pop().unwrap_or(State::DEAD);
            self.statements(&handler.body.body);
            after = after.merge(self.state);
        }
        self.state = after;
        if let Some(finalizer) = &attempt.finalizer {
            self.statements(&finalizer.body);
        }
    }

    /// A labeled statement: a loop or a `switch` takes the label, another statement is left by
    /// a `break` to it.
    fn labeled(&mut self, labeled: &LabeledStatement<'a>) {
        let name = labeled.label.name.as_str();
        if matches!(
            labeled.body,
            Statement::ForStatement(_)
                | Statement::ForInStatement(_)
                | Statement::ForOfStatement(_)
                | Statement::WhileStatement(_)
                | Statement::DoWhileStatement(_)
                | Statement::SwitchStatement(_)
        ) {
            self.statement(&labeled.body, Some(name));
            return;
        }
        self.frames.push(Frame::new(Some(name), false, false));
        self.statement(&labeled.body, None);
        let breaks = self.frames.pop().map_or(State::DEAD, |frame| frame.breaks);
        self.state = self.state.merge(breaks);
    }

    /// A `break` or a `continue`: the paths go to the statement it leaves, and end here.
    fn leave(&mut self, wanted: Option<&str>, back: bool) {
        let state = self.state;
        let frame = self.frames.iter_mut().rev().find(|frame| {
            if back {
                frame.is_loop && wanted.is_none_or(|name| frame.label == Some(name))
            } else {
                wanted.map_or(frame.breakable, |name| frame.label == Some(name))
            }
        });
        if let Some(frame) = frame {
            if back {
                frame.continues = frame.continues.merge(state);
            } else {
                frame.breaks = frame.breaks.merge(state);
            }
        }
        self.state = State::DEAD;
    }

    /// The values a declaration sets.
    fn declarations(&mut self, declaration: &VariableDeclaration<'a>) {
        for declarator in &declaration.declarations {
            if let Some(init) = &declarator.init {
                self.visit_expression(init);
            }
        }
    }

    /// A statement, with the label written before it.
    fn statement(&mut self, statement: &Statement<'a>, label: Option<&'a str>) {
        if self.state == State::DEAD {
            return;
        }
        match statement {
            Statement::ExpressionStatement(statement) => {
                self.visit_expression(&statement.expression);
            }
            Statement::VariableDeclaration(declaration) => self.declarations(declaration),
            Statement::BlockStatement(block) => self.statements(&block.body),
            Statement::ReturnStatement(stop) => {
                if let Some(argument) = &stop.argument {
                    self.visit_expression(argument);
                }
                self.state = State::DEAD;
            }
            Statement::ThrowStatement(stop) => {
                self.visit_expression(&stop.argument);
                self.throws();
                self.state = State::DEAD;
            }
            Statement::IfStatement(choice) => self.choice(choice),
            Statement::WhileStatement(each) => {
                if is_false_literal(&each.test) {
                    self.visit_expression(&each.test);
                    return;
                }
                let endless = is_true_literal(&each.test);
                let test = |paths: &mut Self| paths.visit_expression(&each.test);
                self.repeat(label, &each.body, test, !endless);
            }
            Statement::DoWhileStatement(each) => self.do_while(each, label),
            Statement::ForStatement(each) => self.for_loop(each, label),
            Statement::ForOfStatement(each) => {
                self.visit_expression(&each.right);
                let empty = matches!(static_value(self.source, &each.right), Some(Value::Text(text)) if text.is_empty())
                    || matches!(unwrap(&each.right), Expression::ArrayExpression(array) if array.elements.is_empty());
                if !empty {
                    self.repeat(label, &each.body, |_| {}, true);
                }
            }
            Statement::ForInStatement(each) => {
                self.visit_expression(&each.right);
                let empty = matches!(unwrap(&each.right), Expression::ObjectExpression(object) if object.properties.is_empty());
                if !empty {
                    self.repeat(label, &each.body, |_| {}, true);
                }
            }
            Statement::SwitchStatement(choice) => self.switch(choice, label),
            Statement::TryStatement(attempt) => self.attempt(attempt),
            Statement::LabeledStatement(labeled) => self.labeled(labeled),
            Statement::BreakStatement(stop) => {
                self.leave(stop.label.as_ref().map(|each| each.name.as_str()), false);
            }
            Statement::ContinueStatement(stop) => {
                self.leave(stop.label.as_ref().map(|each| each.name.as_str()), true);
            }
            _ => {}
        }
    }
}

impl<'a> Visit<'a> for Paths<'_, '_, 'a> {
    fn visit_expression(&mut self, expression: &Expression<'a>) {
        match expression {
            Expression::LogicalExpression(logical) => {
                self.visit_expression(&logical.left);
                let short = self.state;
                self.visit_expression(&logical.right);
                self.state = self.state.merge(short);
            }
            Expression::ConditionalExpression(conditional) => {
                self.visit_expression(&conditional.test);
                let tested = self.state;
                self.visit_expression(&conditional.consequent);
                let consequent = self.state;
                self.state = tested;
                self.visit_expression(&conditional.alternate);
                self.state = self.state.merge(consequent);
            }
            Expression::ChainExpression(_) => {
                let short = self.state;
                walk::walk_expression(self, expression);
                self.state = self.state.merge(short);
            }
            Expression::ImportExpression(import) => {
                walk::walk_import_expression(self, import);
            }
            _ => walk::walk_expression(self, expression),
        }
    }

    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        walk::walk_call_expression(self, call);
        if self.is_resolver_call(call) {
            if self.state.called {
                self.reports.insert(call.span.start);
            }
            self.state = State {
                uncalled: false,
                called: self.state.uncalled || self.state.called,
            };
            return;
        }
        self.throws();
        let exits = matches!(&call.callee, Expression::StaticMemberExpression(member)
            if !member.optional && member.property.name == "exit" && member.object.is_specific_id("process"));
        if !call.optional && exits {
            self.state = State::DEAD;
        }
    }

    fn visit_new_expression(&mut self, new: &NewExpression<'a>) {
        walk::walk_new_expression(self, new);
        self.throws();
    }

    fn visit_static_member_expression(&mut self, member: &StaticMemberExpression<'a>) {
        walk::walk_static_member_expression(self, member);
        self.throws();
    }

    fn visit_computed_member_expression(&mut self, member: &ComputedMemberExpression<'a>) {
        walk::walk_computed_member_expression(self, member);
        self.throws();
    }

    fn visit_private_field_expression(&mut self, member: &PrivateFieldExpression<'a>) {
        walk::walk_private_field_expression(self, member);
        self.throws();
    }

    fn visit_await_expression(&mut self, wait: &AwaitExpression<'a>) {
        walk::walk_await_expression(self, wait);
        if matches!(
            unwrap(&wait.argument),
            Expression::Identifier(_) | Expression::ImportExpression(_)
        ) {
            self.throws();
        }
    }

    fn visit_yield_expression(&mut self, value: &YieldExpression<'a>) {
        walk::walk_yield_expression(self, value);
        self.throws();
    }

    fn visit_function(&mut self, _: &Function<'a>, _: ScopeFlags) {}

    fn visit_arrow_function_expression(&mut self, _: &ArrowFunctionExpression<'a>) {}

    fn visit_class(&mut self, _: &Class<'a>) {}
}

/// The resolvers of an executor: its first two parameters, plain names never reassigned.
fn resolvers(source: &Source, params: &oxc_ast::ast::FormalParameters) -> Vec<SymbolId> {
    let scoping = source.semantic.scoping();
    params
        .items
        .iter()
        .take(2)
        .filter_map(|parameter| match &parameter.pattern {
            BindingPattern::BindingIdentifier(name) => name.symbol_id.get(),
            _ => None,
        })
        .filter(|symbol| {
            !scoping
                .get_resolved_references(*symbol)
                .any(oxc_semantic::Reference::is_write)
        })
        .collect()
}

/// The parameters and the body of a function or an arrow that is not a generator.
fn executor_parts<'s, 'a>(
    expression: &'s Expression<'a>,
) -> Option<(&'s oxc_ast::ast::FormalParameters<'a>, Body<'s, 'a>)> {
    match unwrap(expression) {
        Expression::ArrowFunctionExpression(arrow) => {
            Some((&arrow.params, Body::of_arrow(&arrow.body)?))
        }
        Expression::FunctionExpression(function) if !function.generator => {
            Some((&function.params, Body::Block(function.body.as_ref()?)))
        }
        _ => None,
    }
}

impl Rule for NoMultiplePromiseResolverCalls {
    fn name(&self) -> &'static str {
        "unicorn/no-multiple-promise-resolver-calls"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::NewExpression(new) = node.kind() else {
            return;
        };
        let Some(executor) = promise_executor(source, new) else {
            return;
        };
        let Some((params, body)) = executor_parts(executor) else {
            return;
        };
        let resolvers = resolvers(source, params);
        if resolvers.is_empty() {
            return;
        }
        let span = unwrap(executor).span();
        let inner = source.semantic.nodes().iter().filter_map(|each| {
            let within =
                each.kind().span().start > span.start && each.kind().span().end <= span.end;
            match each.kind() {
                AstKind::Function(function) if within => function.body.as_deref().map(Body::Block),
                AstKind::ArrowFunctionExpression(arrow) if within => Body::of_arrow(&arrow.body),
                _ => None,
            }
        });
        let mut reports = BTreeSet::new();
        for each in std::iter::once(body).chain(inner) {
            let mut paths = Paths {
                source,
                resolvers: &resolvers,
                state: State::START,
                catches: Vec::new(),
                frames: Vec::new(),
                reports: BTreeSet::new(),
            };
            match each {
                Body::Block(block) => paths.statements(&block.statements),
                Body::Expression(expression) => paths.visit_expression(expression),
            }
            reports.extend(paths.reports);
        }
        findings.extend(
            reports
                .into_iter()
                .map(|at| source.finding(self.name(), at, MESSAGE.to_owned())),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::NoMultiplePromiseResolverCalls;
    use crate::testing::lint;

    /// The lines of the findings.
    fn lines(code: &str) -> Vec<usize> {
        lint(&NoMultiplePromiseResolverCalls, "playground/a.ts", code)
            .iter()
            .map(|each| each.line)
            .collect()
    }

    #[test]
    fn reports_the_documentation_s_cases() {
        // A missing `return`, then the fixed and exclusive forms, then the two `try` cases.
        assert_eq!(
            lines(
                "new Promise((resolve, reject) => {\n\tif (error) {\n\t\treject(error);\n\t}\n\n\tresolve(value);\n});"
            ),
            [6]
        );
        for code in [
            "new Promise((resolve, reject) => {\n\tif (error) {\n\t\treject(error);\n\t\treturn;\n\t}\n\n\tresolve(value);\n});",
            "new Promise((resolve, reject) => {\n\tif (error) {\n\t\treject(error);\n\t} else {\n\t\tresolve(value);\n\t}\n});",
            "new Promise((resolve, reject) => {\n\ttry {\n\t\tresolve(mayThrow());\n\t} catch (error) {\n\t\treject(error);\n\t}\n});",
        ] {
            assert_eq!(lines(code), Vec::<usize>::new(), "{code}");
        }
        assert_eq!(
            lines(
                "new Promise((resolve, reject) => {\n\ttry {\n\t\tresolve(value);\n\t\tmayThrow();\n\t} catch (error) {\n\t\treject(error);\n\t}\n});"
            ),
            [6]
        );
    }

    #[test]
    fn follows_loops_branches_and_ends_of_paths() {
        let reported = [
            "new Promise(resolve => { for (const a of b) { resolve(a); } });",
            "new Promise(resolve => { while (a) { resolve(); } });",
            "new Promise(resolve => { do { resolve(); } while (a); });",
            "new Promise(r => { a && r(); r(); });",
            "new Promise(r => { a ? r() : f(); r(); });",
            "new Promise(r => { switch (a) { case 1: r(); case 2: r(); } });",
            "new Promise(r => { r(); setTimeout(() => {}); r(); });",
            "new Promise(r => { function f() { r(); r(); } });",
            "new Promise(r => { a?.(r()); r(); });",
            "new Promise(function (resolve, reject) { resolve(); reject(); });",
            "new Promise((r, j) => { try { r(); } finally { j(); } });",
            "new Promise(r => { label: { if (a) break label; r(); } r(); });",
            "new Promise((r, j) => { r() && process.exit(1); j(); });",
            "new Promise((r, j) => { r(); a?.b(process.exit(1)); j(); });",
            "new Promise((r, j) => { a ? process.exit(1) : r(); j(); });",
        ];
        for code in reported {
            assert_eq!(lines(code), [1], "{code}");
        }
        for code in [
            "new Promise(r => { if (a) { r(); return; } r(); });",
            "new Promise(r => { if (a) { r(); throw e; } r(); });",
            "new Promise(r => { if (a) { r(); process.exit(1); } r(); });",
            "new Promise(r => { while (true) { r(); break; } });",
            "new Promise(r => { for (const a of []) { r(); } r(); });",
            "new Promise(r => { while (false) { r(); } r(); });",
            "new Promise(r => { if (false) { r(); } r(); });",
            "new Promise(r => { switch (a) { case 1: r(); break; default: r(); } });",
            "new Promise(r => { r(); setTimeout(() => r()); });",
            "new Promise(r => { do { r(); } while (false); });",
            "new Promise(r => { for (;;) { r(); return; } });",
            "new Promise(r => { if (true) return; r(); r(); });",
            "new Promise((r, j) => { r(); j; });",
            "new Promise(r => { r = f; r(); r(); });",
            "new Promise(({ r }) => { r(); r(); });",
            "new Promise(function* (r) { r(); r(); });",
            "new Other(r => { r(); r(); });",
            "new Promise(r => { label: for (const a of b) { for (const c of d) { r(); break label; } } });",
            "new Promise(r => { for (const a of b) { if (a) { r(); return; } } r(); });",
        ] {
            assert_eq!(lines(code), Vec::<usize>::new(), "{code}");
        }
    }

    #[test]
    fn reports_once_per_call_and_separates_executors() {
        assert_eq!(
            lines("new Promise(r => {\n\tr();\n\tnew Promise(s => { s(); });\n\tr();\n\tr();\n});"),
            [4, 5]
        );
        assert_eq!(
            lines(
                "new Promise(async (r, j) => {\n\ttry {\n\t\tr();\n\t\tawait x;\n\t} catch {\n\t\tj();\n\t}\n});"
            ),
            [6]
        );
    }
}
