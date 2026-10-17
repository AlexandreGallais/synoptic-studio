//! `require-atomic-updates`, properties included (`allowProperties: false`): in an `async`
//! function or a generator, an assignment is reported when its target — a variable, or a
//! property through a variable — was read before an `await` or a `yield` that comes before the
//! assignment, unless read again since: the value may have changed while the function was paused
//! (`REF-ESLINT-REQUIRE-ATOMIC-UPDATES`). A local variable no closure reaches is safe, unless it
//! is a parameter written through a property. `ESLint` follows the code paths; this rule walks the
//! function in order, a branch's reads joined after it, the code after `return` or `throw` left
//! out, and a loop's body read once.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrowFunctionExpression, AssignmentExpression, AssignmentOperator, AssignmentTarget,
    AwaitExpression, ConditionalExpression, Expression, ForInStatement, ForOfStatement,
    ForStatement, Function, IdentifierReference, IfStatement, LogicalExpression, ReturnStatement,
    SwitchStatement, ThrowStatement, TryStatement, WhileStatement, YieldExpression,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::{ScopeFlags, ScopeId, SymbolId};
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct RequireAtomicUpdates;

/// What a paused function has read: the variables read since the last pause, and those read
/// before it and not since.
#[derive(Clone, Debug, Default)]
struct Reads {
    fresh: Vec<SymbolId>,
    outdated: Vec<SymbolId>,
}

/// Two branches' reads joined; an unreachable branch adds nothing.
fn join(left: Option<Reads>, right: Option<Reads>) -> Option<Reads> {
    match (left, right) {
        (Some(mut left), Some(right)) => {
            for symbol in right.fresh {
                if !left.fresh.contains(&symbol) {
                    left.fresh.push(symbol);
                }
            }
            for symbol in right.outdated {
                if !left.outdated.contains(&symbol) {
                    left.outdated.push(symbol);
                }
            }
            Some(left)
        }
        (left, right) => left.or(right),
    }
}

/// A function being walked: whether it can pause, and its reads, `None` past a `return`.
struct Frame {
    pauses: bool,
    reads: Option<Reads>,
}

/// The walk over a file.
struct Walk<'s, 'a> {
    source: &'s Source<'s, 'a>,
    frames: Vec<Frame>,
    /// The base of an assignment's target written with `=`, which `ESLint` does not count as read.
    unread: Option<Span>,
    findings: Vec<Finding>,
}

impl Walk<'_, '_> {
    /// The reads of the current function, when it can pause and the code is reachable.
    fn reads(&mut self) -> Option<&mut Reads> {
        self.frames
            .last_mut()
            .filter(|frame| frame.pauses)
            .and_then(|frame| frame.reads.as_mut())
    }

    /// The reads, kept before a branch.
    fn snapshot(&self) -> Option<Reads> {
        self.frames.last().and_then(|frame| frame.reads.clone())
    }

    /// The reads set back, after a branch.
    fn restore(&mut self, reads: Option<Reads>) {
        if let Some(frame) = self.frames.last_mut() {
            frame.reads = reads;
        }
    }

    /// The variable a name resolves to.
    fn symbol(&self, identifier: &IdentifierReference) -> Option<SymbolId> {
        let reference = identifier.reference_id.get()?;
        self.source
            .semantic
            .scoping()
            .get_reference(reference)
            .symbol_id()
    }

    /// The function scope holding a scope: itself, or the nearest function above, or the module.
    fn function_scope(&self, scope: ScopeId) -> ScopeId {
        let scoping = self.source.semantic.scoping();
        let mut current = scope;
        while !scoping
            .scope_flags(current)
            .intersects(ScopeFlags::Function | ScopeFlags::Top)
        {
            match scoping.scope_parent_id(current) {
                Some(parent) => current = parent,
                None => break,
            }
        }
        current
    }

    /// Whether a variable is a parameter, destructured or not.
    fn is_parameter(&self, symbol: SymbolId) -> bool {
        let nodes = self.source.semantic.nodes();
        let declaration = self.source.semantic.scoping().symbol_declaration(symbol);
        std::iter::once(nodes.get_node(declaration))
            .chain(nodes.ancestors(declaration))
            .map(oxc_semantic::AstNode::kind)
            .take_while(|kind| {
                !matches!(
                    kind,
                    AstKind::Function(_)
                        | AstKind::ArrowFunctionExpression(_)
                        | AstKind::Program(_)
                )
            })
            .any(|kind| {
                matches!(
                    kind,
                    AstKind::FormalParameter(_) | AstKind::FormalParameters(_)
                )
            })
    }

    /// Whether a variable is local to its function and read by no closure: then no other code
    /// can change it during a pause. A parameter written through a property is not.
    fn is_local(&self, symbol: SymbolId, through_property: bool) -> bool {
        if through_property && self.is_parameter(symbol) {
            return false;
        }
        let scoping = self.source.semantic.scoping();
        let nodes = self.source.semantic.nodes();
        let own = self.function_scope(scoping.symbol_scope_id(symbol));
        scoping
            .get_resolved_reference_ids(symbol)
            .iter()
            .map(|reference| scoping.get_reference(*reference).node_id())
            .all(|node| self.function_scope(nodes.get_node(node).scope_id()) == own)
    }

    /// Walks a function's body as a frame of its own.
    fn within(&mut self, pauses: bool, walk_body: impl FnOnce(&mut Self)) {
        self.frames.push(Frame {
            pauses,
            reads: Some(Reads::default()),
        });
        walk_body(self);
        self.frames.pop();
    }

    /// The reads made stale by a pause.
    fn pause(&mut self) {
        if let Some(reads) = self.reads() {
            let fresh = std::mem::take(&mut reads.fresh);
            for symbol in fresh {
                if !reads.outdated.contains(&symbol) {
                    reads.outdated.push(symbol);
                }
            }
        }
    }
}

/// The variable an assignment writes, or writes a property of, and whether through a property.
fn written<'b, 'a>(
    target: &'b AssignmentTarget<'a>,
) -> Option<(&'b IdentifierReference<'a>, bool)> {
    if let AssignmentTarget::AssignmentTargetIdentifier(identifier) = target {
        return Some((identifier, false));
    }
    let mut object = target.as_member_expression()?.object();
    loop {
        match object {
            Expression::Identifier(identifier) => return Some((identifier, true)),
            other => object = other.as_member_expression()?.object(),
        }
    }
}

impl<'a> Visit<'a> for Walk<'_, 'a> {
    fn visit_function(&mut self, function: &Function<'a>, flags: ScopeFlags) {
        self.within(function.r#async || function.generator, |walk| {
            walk::walk_function(walk, function, flags);
        });
    }

    fn visit_arrow_function_expression(&mut self, arrow: &ArrowFunctionExpression<'a>) {
        self.within(arrow.r#async, |walk| {
            walk::walk_arrow_function_expression(walk, arrow);
        });
    }

    fn visit_identifier_reference(&mut self, identifier: &IdentifierReference<'a>) {
        let Some(symbol) = self.symbol(identifier) else {
            return;
        };
        let is_read = identifier.reference_id.get().is_some_and(|reference| {
            self.source
                .semantic
                .scoping()
                .get_reference(reference)
                .is_read()
        });
        if is_read
            && self.unread != Some(identifier.span)
            && let Some(reads) = self.reads()
        {
            reads.outdated.retain(|each| *each != symbol);
            if !reads.fresh.contains(&symbol) {
                reads.fresh.push(symbol);
            }
        }
    }

    fn visit_assignment_expression(&mut self, assignment: &AssignmentExpression<'a>) {
        let target = written(&assignment.left).and_then(|(identifier, through)| {
            Some((identifier, through, self.symbol(identifier)?))
        });
        let watched = target.filter(|(_, through, symbol)| {
            self.reads().is_some() && !self.is_local(*symbol, *through)
        });
        if assignment.operator == AssignmentOperator::Assign {
            self.unread = target.map(|(identifier, _, _)| identifier.span);
        }
        self.visit_assignment_target(&assignment.left);
        self.unread = None;
        self.visit_expression(&assignment.right);

        let Some((identifier, through, symbol)) = watched else {
            return;
        };
        if !self
            .reads()
            .is_some_and(|reads| reads.outdated.contains(&symbol))
        {
            return;
        }
        let name = identifier.name.as_str();
        let message = if through {
            let left = assignment.left.span();
            format!(
                "Possible race condition: `{}` might be assigned based on an outdated state of `{name}`.",
                &self.source.text[left.start as usize..left.end as usize]
            )
        } else {
            format!(
                "Possible race condition: `{name}` might be reassigned based on an outdated value of `{name}`."
            )
        };
        let finding = self
            .source
            .finding("require-atomic-updates", assignment.span.start, message);
        self.findings.push(finding);
    }

    fn visit_await_expression(&mut self, expression: &AwaitExpression<'a>) {
        walk::walk_await_expression(self, expression);
        self.pause();
    }

    fn visit_yield_expression(&mut self, expression: &YieldExpression<'a>) {
        walk::walk_yield_expression(self, expression);
        self.pause();
    }

    fn visit_if_statement(&mut self, statement: &IfStatement<'a>) {
        self.visit_expression(&statement.test);
        let before = self.snapshot();
        self.visit_statement(&statement.consequent);
        let consequent = self.snapshot();
        self.restore(before);
        if let Some(alternate) = &statement.alternate {
            self.visit_statement(alternate);
        }
        let alternate = self.snapshot();
        self.restore(join(consequent, alternate));
    }

    fn visit_conditional_expression(&mut self, expression: &ConditionalExpression<'a>) {
        self.visit_expression(&expression.test);
        let before = self.snapshot();
        self.visit_expression(&expression.consequent);
        let consequent = self.snapshot();
        self.restore(before);
        self.visit_expression(&expression.alternate);
        let alternate = self.snapshot();
        self.restore(join(consequent, alternate));
    }

    fn visit_logical_expression(&mut self, expression: &LogicalExpression<'a>) {
        self.visit_expression(&expression.left);
        let before = self.snapshot();
        self.visit_expression(&expression.right);
        let after = self.snapshot();
        self.restore(join(before, after));
    }

    fn visit_while_statement(&mut self, statement: &WhileStatement<'a>) {
        self.visit_expression(&statement.test);
        let before = self.snapshot();
        self.visit_statement(&statement.body);
        self.restore(before);
    }

    fn visit_for_statement(&mut self, statement: &ForStatement<'a>) {
        if let Some(init) = &statement.init {
            self.visit_for_statement_init(init);
        }
        if let Some(test) = &statement.test {
            self.visit_expression(test);
        }
        let before = self.snapshot();
        self.visit_statement(&statement.body);
        if let Some(update) = &statement.update {
            self.visit_expression(update);
        }
        self.restore(before);
    }

    fn visit_for_of_statement(&mut self, statement: &ForOfStatement<'a>) {
        self.visit_expression(&statement.right);
        let before = self.snapshot();
        self.visit_for_statement_left(&statement.left);
        self.visit_statement(&statement.body);
        self.restore(before);
    }

    fn visit_for_in_statement(&mut self, statement: &ForInStatement<'a>) {
        self.visit_expression(&statement.right);
        let before = self.snapshot();
        self.visit_for_statement_left(&statement.left);
        self.visit_statement(&statement.body);
        self.restore(before);
    }

    fn visit_switch_statement(&mut self, statement: &SwitchStatement<'a>) {
        self.visit_expression(&statement.discriminant);
        let before = self.snapshot();
        let mut after = before.clone();
        for case in &statement.cases {
            let entry = join(before.clone(), self.snapshot());
            self.restore(entry);
            self.visit_switch_case(case);
            after = join(after, self.snapshot());
        }
        self.restore(after);
    }

    fn visit_try_statement(&mut self, statement: &TryStatement<'a>) {
        let before = self.snapshot();
        self.visit_block_statement(&statement.block);
        let tried = self.snapshot();
        if let Some(handler) = &statement.handler {
            self.restore(join(before, tried.clone()));
            self.visit_catch_clause(handler);
            let caught = self.snapshot();
            self.restore(join(tried, caught));
        }
        if let Some(finalizer) = &statement.finalizer {
            self.visit_block_statement(finalizer);
        }
    }

    fn visit_return_statement(&mut self, statement: &ReturnStatement<'a>) {
        walk::walk_return_statement(self, statement);
        self.restore(None);
    }

    fn visit_throw_statement(&mut self, statement: &ThrowStatement<'a>) {
        walk::walk_throw_statement(self, statement);
        self.restore(None);
    }
}

impl Rule for RequireAtomicUpdates {
    fn name(&self) -> &'static str {
        "require-atomic-updates"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut walk = Walk {
            source,
            frames: vec![Frame {
                pauses: false,
                reads: Some(Reads::default()),
            }],
            unread: None,
            findings: Vec::new(),
        };
        walk.visit_program(source.program);
        findings.append(&mut walk.findings);
    }
}

#[cfg(test)]
mod tests {
    use super::RequireAtomicUpdates;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code, after `let result;` and `declare` helpers.
    fn found(code: &str) -> Vec<String> {
        let prelude = "let result: number;\ndeclare const something: Promise<number>;\ndeclare function doSomething(a: number): number;\ndeclare function initialize(): Promise<number>;\ndeclare function getSomething(): Promise<number>;\n";
        messages(&lint(
            &RequireAtomicUpdates,
            "src/a.ts",
            &format!("{prelude}{code}"),
        ))
    }

    /// The message on a variable.
    fn reassigned(name: &str) -> String {
        format!(
            "Possible race condition: `{name}` might be reassigned based on an outdated value of `{name}`."
        )
    }

    #[test]
    fn reports_the_incorrect_cases_of_the_documentation() {
        // [F19.AC3] ESLint's documented incorrect cases, each reported.
        for code in [
            "async function foo() { result += await something; }",
            "async function bar() { result = result + await something; }",
            "async function baz() { result = result + doSomething(await something); }",
            "async function qux() { if (!result) { result = await initialize(); } }",
            "function* generator() { result += yield; }",
        ] {
            assert_eq!(found(code), [reassigned("result")], "{code}");
        }
    }

    #[test]
    fn accepts_the_correct_cases_of_the_documentation() {
        for code in [
            "async function foobar() { result = await something + result; }",
            "async function baz() { const tmp = doSomething(await something); result += tmp; }",
            "async function qux() { if (!result) { const tmp = await initialize(); if (!result) { result = tmp; } } }",
            "async function quux() { let localVariable = 0; localVariable += await something; }",
            "function* generator() { result = (yield) + result; }",
        ] {
            assert_eq!(found(code), NONE, "{code}");
        }
    }

    #[test]
    fn reports_a_property_written_through_a_parameter_read_before_a_pause() {
        // The documentation's case for `allowProperties: false`.
        assert_eq!(
            found(
                "async function foo(obj: { done: boolean; something: number }) { if (!obj.done) { obj.something = await getSomething(); } }"
            ),
            [
                "Possible race condition: `obj.something` might be assigned based on an outdated state of `obj`."
            ]
        );
    }

    #[test]
    fn follows_the_path_past_a_branch_that_returns() {
        // The test reads `o` on the path that goes on: still read before the pause.
        assert_eq!(
            found("async function f(o: { a: number }) { if (o.a) { return; } o.a = await getSomething(); }").len(),
            1
        );
        // Read only on the path that returns: nothing read reaches the pause.
        assert_eq!(
            found(
                "async function f(o: { a: number }, b: boolean) { if (b) { o.a; return; } o.a = await getSomething(); }"
            ),
            NONE
        );
    }

    #[test]
    fn accepts_a_property_of_a_local_object_or_one_not_read_before() {
        for code in [
            "async function f() { const o = { a: 1 }; if (o.a) { o.a = await getSomething(); } }",
            "async function f(o: { a: number }) { o.a = await getSomething(); }",
            "function f(o: { a: number }) { if (o.a) { o.a = 1; } }",
            "async function f(o: { a: number }) { await something; o.a = o.a + 1; }",
        ] {
            assert_eq!(found(code), NONE, "{code}");
        }
    }

    #[test]
    fn joins_branches_and_reads_loops_once() {
        assert_eq!(
            found("async function f(a: boolean) { a ? result : 0; result = await something; }")
                .len(),
            1
        );
        assert_eq!(
            found(
                "async function f(a: boolean) { a && result; await something; result = 1 + result; }"
            ),
            NONE
        );
        assert_eq!(
            found("async function f(a: number) { switch (a) { case 1: result; break; default: } result = await something; }").len(),
            1
        );
        assert_eq!(
            found("async function f() { try { result; } catch { } finally { } result = await something; }").len(),
            1
        );
        assert_eq!(
            found(
                "async function f() { for (const x of [result]) { x; } result = await something; }"
            )
            .len(),
            1
        );
        assert_eq!(
            found(
                "async function f() { while (result) { await something; } for (;;) { } for (const k in {}) { } result = 1; }"
            ),
            NONE
        );
    }

    #[test]
    fn leaves_the_module_s_own_awaits() {
        assert_eq!(
            found("function reset() { result = 0; }\nresult += await something;"),
            NONE
        );
    }

    #[test]
    fn walks_nested_functions_as_their_own() {
        assert_eq!(
            found(
                "async function f() { result; const g = () => { result = 1; }; await something; g(); }"
            ),
            NONE
        );
        assert_eq!(
            found("function f() { result; return async () => { result += await something; }; }"),
            [reassigned("result")]
        );
    }
}
