//! What Unicorn's rules on control flow, functions, errors and numbers read of a file, from the
//! tree alone (EN-079): the text and the tokens of a node, comments, the variable a name binds,
//! the statement list holding a statement, and whether a value is a boolean, as
//! `eslint-plugin-unicorn` 78.0.0 tells it without type information (`utils/is-boolean.js`,
//! `utils/boolean.js`).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, Expression, FormalParameter, IdentifierReference, LogicalOperator, Statement,
    TSTypeAnnotation, UnaryOperator, VariableDeclarationKind,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::{AstNode, NodeId, SymbolId};
use oxc_span::{GetSpan, Span};

use crate::source::Source;
use crate::written::{is_boolean_expression, is_boolean_type};

/// The text of a range of the file.
#[must_use]
pub fn text<'a>(source: &Source<'_, 'a>, span: Span) -> &'a str {
    source
        .text
        .get(span.start as usize..span.end as usize)
        .unwrap_or_default()
}

/// The texts of the tokens of a range, a last `;` left out.
fn token_texts<'a>(source: &Source<'_, 'a>, span: Span) -> Vec<&'a str> {
    let first = source
        .tokens
        .partition_point(|token| token.start() < span.start);
    let mut texts: Vec<&'a str> = source.tokens[first..]
        .iter()
        .take_while(|token| token.end() <= span.end)
        .map(|token| text(source, token.span()))
        .collect();
    if texts.last() == Some(&";") {
        texts.pop();
    }
    texts
}

/// Whether two ranges hold the same tokens, comments, layout and a last `;` left out; two
/// ranges without a token are not the same (`utils/is-same-statement.js`). The rule asking
/// needs the file's tokens.
#[must_use]
pub fn same_tokens(source: &Source, left: Span, right: Span) -> bool {
    let left = token_texts(source, left);

    !left.is_empty() && left == token_texts(source, right)
}

/// An offset of the file from a length, as Oxc counts them.
#[must_use]
pub fn offset(length: usize) -> u32 {
    u32::try_from(length).unwrap_or(u32::MAX)
}

/// Whether a range is written between parentheses, blanks aside.
#[must_use]
pub fn is_parenthesized(source: &Source, span: Span) -> bool {
    let before = source.text[..span.start as usize].trim_end();
    let after = source.text[span.end as usize..].trim_start();
    before.ends_with('(') && after.starts_with(')')
}

/// Whether a comment lies in a range.
#[must_use]
pub fn has_comment_in(source: &Source, span: Span) -> bool {
    source
        .program
        .comments
        .iter()
        .any(|comment| comment.span.start >= span.start && comment.span.end <= span.end)
}

/// The variable a name refers to, when declared in the file.
#[must_use]
pub fn symbol(source: &Source, name: &IdentifierReference) -> Option<SymbolId> {
    let reference = name.reference_id.get()?;
    source
        .semantic
        .scoping()
        .get_reference(reference)
        .symbol_id()
}

/// Whether a name refers to a global: declared nowhere in the file.
#[must_use]
pub fn is_global(source: &Source, name: &IdentifierReference) -> bool {
    symbol(source, name).is_none()
}

/// Whether an expression is a call of a global function by its name, with one argument that is
/// not spread, not optional (`isGlobalBooleanCall`).
#[must_use]
pub fn is_global_call(source: &Source, expression: &Expression, name: &str) -> bool {
    matches!(expression, Expression::CallExpression(call)
        if !call.optional
            && call.arguments.len() == 1
            && !call.arguments[0].is_spread()
            && matches!(&call.callee, Expression::Identifier(callee)
                if callee.name == name && is_global(source, callee)))
}

/// The value a `const` binds by its name alone, `const name = value`.
#[must_use]
pub fn const_value<'s, 'a>(
    source: &Source<'s, 'a>,
    symbol: SymbolId,
) -> Option<&'s Expression<'a>> {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let AstKind::VariableDeclarator(declarator) = nodes.kind(declaration) else {
        return None;
    };
    let is_const = matches!(nodes.parent_kind(declaration),
        AstKind::VariableDeclaration(declaration) if declaration.kind == VariableDeclarationKind::Const);
    if !is_const || !matches!(declarator.id, BindingPattern::BindingIdentifier(_)) {
        return None;
    }
    declarator.init.as_ref()
}

/// The written type of the variable a name binds: a variable's or a parameter's annotation.
#[must_use]
pub fn written_type<'s, 'a>(
    source: &Source<'s, 'a>,
    symbol: SymbolId,
) -> Option<&'s TSTypeAnnotation<'a>> {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    match nodes.kind(declaration) {
        AstKind::VariableDeclarator(declarator) => declarator.type_annotation.as_deref(),
        AstKind::FormalParameter(parameter) => parameter_type(parameter),
        _ => None,
    }
}

/// A parameter's written type, when it is not optional.
fn parameter_type<'s, 'a>(parameter: &'s FormalParameter<'a>) -> Option<&'s TSTypeAnnotation<'a>> {
    if parameter.optional {
        return None;
    }
    parameter.type_annotation.as_deref()
}

/// The statements of a node that holds a list of them.
#[must_use]
pub fn statements(kind: AstKind<'_>) -> Option<&[Statement<'_>]> {
    match kind {
        AstKind::Program(program) => Some(&program.body),
        AstKind::BlockStatement(block) => Some(&block.body),
        AstKind::FunctionBody(body) => Some(&body.statements),
        AstKind::StaticBlock(block) => Some(&block.body),
        AstKind::SwitchCase(case) => Some(&case.consequent),
        _ => None,
    }
}

/// The statement list holding a statement, and its place in it.
#[must_use]
pub fn siblings<'a>(
    source: &Source<'_, 'a>,
    node: &AstNode<'a>,
) -> Option<(&'a [Statement<'a>], usize)> {
    let list = statements(source.semantic.nodes().parent_kind(node.id()))?;
    let span = node.kind().span();
    let index = list.iter().position(|each| each.span() == span)?;

    Some((list, index))
}

/// The kind of a node's parent.
#[must_use]
pub fn parent<'a>(source: &Source<'_, 'a>, id: NodeId) -> AstKind<'a> {
    source.semantic.nodes().parent_kind(id)
}

/// The global methods that answer a boolean, by their object.
const BOOLEAN_STATIC_METHODS: [(&str, &[&str]); 8] = [
    ("Array", &["isArray"]),
    ("ArrayBuffer", &["isView"]),
    ("Atomics", &["isLockFree"]),
    ("Error", &["isError"]),
    (
        "Number",
        &["isFinite", "isInteger", "isNaN", "isSafeInteger"],
    ),
    (
        "Object",
        &["hasOwn", "is", "isExtensible", "isFrozen", "isSealed"],
    ),
    ("Reflect", &["deleteProperty", "has"]),
    ("URL", &["canParse"]),
];

/// Whether a value is a boolean, as the tree tells it: a literal, a negation, a comparison, a
/// logical or conditional expression of booleans, `Boolean(…)`, a global method answering a
/// boolean, a variable written `boolean` or a `const` holding a boolean, a local function
/// written to return one, or a value asserted to `boolean` (`utils/is-boolean.js`).
#[must_use]
pub fn is_boolean(source: &Source, expression: &Expression) -> bool {
    is_boolean_within(source, expression, 0)
}

/// `is_boolean`, a chain of variables followed at most a few steps.
fn is_boolean_within(source: &Source, expression: &Expression, depth: usize) -> bool {
    if depth > 8 {
        return false;
    }
    let next = |each: &Expression| is_boolean_within(source, each, depth + 1);
    match expression {
        Expression::BooleanLiteral(_)
        | Expression::BinaryExpression(_)
        | Expression::UnaryExpression(_) => is_boolean_expression(expression),
        Expression::LogicalExpression(logical) => {
            logical.operator != LogicalOperator::Coalesce
                && next(&logical.left)
                && next(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            next(&conditional.consequent) && next(&conditional.alternate)
        }
        Expression::ParenthesizedExpression(inner) => next(&inner.expression),
        Expression::TSNonNullExpression(inner) => next(&inner.expression),
        Expression::AwaitExpression(inner) => next(&inner.argument),
        Expression::SequenceExpression(sequence) => sequence.expressions.last().is_some_and(next),
        Expression::AssignmentExpression(assignment) => {
            assignment.operator.is_assign() && next(&assignment.right)
        }
        Expression::TSAsExpression(assertion) => {
            is_boolean_type(&assertion.type_annotation) || next(&assertion.expression)
        }
        Expression::TSSatisfiesExpression(assertion) => {
            is_boolean_type(&assertion.type_annotation) || next(&assertion.expression)
        }
        Expression::Identifier(name) => {
            symbol(source, name).is_some_and(|symbol| match written_type(source, symbol) {
                Some(written) => is_boolean_type(&written.type_annotation),
                None => const_value(source, symbol).is_some_and(next),
            })
        }
        Expression::CallExpression(call) => {
            is_global_call(source, expression, "Boolean")
                || is_boolean_static_call(source, &call.callee)
                || matches!(&call.callee, Expression::Identifier(callee)
                    if (callee.name == "isFinite" || callee.name == "isNaN") && is_global(source, callee))
                || matches!(&call.callee, Expression::Identifier(callee)
                    if returns_boolean(source, callee, depth))
        }
        _ => false,
    }
}

/// Whether a callee is a global method answering a boolean, `Number.isInteger`.
fn is_boolean_static_call(source: &Source, callee: &Expression) -> bool {
    let Some(member) = callee.get_member_expr() else {
        return false;
    };
    let (Expression::Identifier(object), Some(property)) =
        (member.object(), member.static_property_name())
    else {
        return false;
    };
    is_global(source, object)
        && BOOLEAN_STATIC_METHODS.iter().any(|(name, methods)| {
            *name == object.name.as_str() && methods.iter().any(|method| property == *method)
        })
}

/// Whether a local function, by its name, is written to return a boolean or returns one.
fn returns_boolean(source: &Source, callee: &IdentifierReference, depth: usize) -> bool {
    let Some(symbol) = symbol(source, callee) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let function_returns = |returns: Option<&TSTypeAnnotation>, body: Option<&Expression>| {
        returns.map_or_else(
            || body.is_some_and(|each| is_boolean_within(source, each, depth + 1)),
            |written| is_boolean_type(&written.type_annotation),
        )
    };
    match nodes.kind(declaration) {
        AstKind::Function(function) => {
            !function.r#async
                && !function.generator
                && function_returns(function.return_type.as_deref(), None)
        }
        AstKind::VariableDeclarator(_) => match const_value(source, symbol) {
            Some(Expression::ArrowFunctionExpression(arrow)) => {
                !arrow.r#async
                    && function_returns(arrow.return_type.as_deref(), arrow.get_expression())
            }
            Some(Expression::FunctionExpression(function)) => {
                !function.r#async
                    && !function.generator
                    && function_returns(function.return_type.as_deref(), None)
            }
            _ => false,
        },
        _ => false,
    }
}

/// Whether a value's result is used only for its truth: negated, given to `Boolean()`, or the
/// test of an `if`, a loop or a conditional expression, through the `&&` and `||` around it
/// (`isBooleanContext`).
#[must_use]
pub fn is_boolean_context(source: &Source, id: NodeId) -> bool {
    read_for_truth(source, id, true)
}

/// Whether a value is the test of an `if`, a loop or a conditional expression, through the `&&`
/// and `||` around it (`isControlFlowTest`).
#[must_use]
pub fn is_control_flow_test(source: &Source, id: NodeId) -> bool {
    read_for_truth(source, id, false)
}

/// Whether a value is read for its truth: as a test, or also negated or given to `Boolean()`.
fn read_for_truth(source: &Source, id: NodeId, converted: bool) -> bool {
    let nodes = source.semantic.nodes();
    let itself_converts = match nodes.kind(id) {
        AstKind::UnaryExpression(unary) => unary.operator == UnaryOperator::LogicalNot,
        AstKind::CallExpression(call) => {
            !call.optional
                && call.arguments.len() == 1
                && matches!(&call.callee, Expression::Identifier(callee)
                    if callee.name == "Boolean" && is_global(source, callee))
        }
        _ => false,
    };
    if converted && itself_converts {
        return true;
    }
    let mut current = id;
    loop {
        let span = nodes.kind(current).span();
        let parent = nodes.parent_id(current);
        let tested = |test: &Expression| test.span() == span;
        match nodes.kind(parent) {
            AstKind::UnaryExpression(unary) if unary.operator == UnaryOperator::LogicalNot => {
                return converted;
            }
            AstKind::CallExpression(call) => {
                return converted
                    && call.arguments.len() == 1
                    && call.arguments[0].span() == span
                    && matches!(&call.callee, Expression::Identifier(callee)
                        if callee.name == "Boolean" && is_global(source, callee))
                    && !call.optional;
            }
            AstKind::IfStatement(statement) => return tested(&statement.test),
            AstKind::WhileStatement(statement) => return tested(&statement.test),
            AstKind::DoWhileStatement(statement) => return tested(&statement.test),
            AstKind::ConditionalExpression(conditional) => return tested(&conditional.test),
            AstKind::ForStatement(statement) => return statement.test.as_ref().is_some_and(tested),
            AstKind::LogicalExpression(logical)
                if logical.operator != LogicalOperator::Coalesce =>
            {
                current = parent;
            }
            _ => return false,
        }
        if current == NodeId::ROOT {
            return false;
        }
    }
}

/// What a walk over a node finds.
#[derive(Default)]
struct Found<'a> {
    /// The names it reads.
    names: Vec<&'a str>,
    /// Whether it calls, assigns, deletes or creates anything.
    side_effect: bool,
    /// Whether it awaits or yields.
    suspends: bool,
}

impl<'a> Visit<'a> for Found<'a> {
    fn visit_identifier_reference(&mut self, name: &IdentifierReference<'a>) {
        self.names.push(name.name.as_str());
    }

    fn visit_expression(&mut self, expression: &Expression<'a>) {
        match expression {
            Expression::CallExpression(_)
            | Expression::NewExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::UpdateExpression(_)
            | Expression::TaggedTemplateExpression(_)
            | Expression::ImportExpression(_) => self.side_effect = true,
            Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::Delete => {
                self.side_effect = true;
            }
            Expression::AwaitExpression(_) | Expression::YieldExpression(_) => {
                self.side_effect = true;
                self.suspends = true;
            }
            _ => {}
        }
        walk::walk_expression(self, expression);
    }
}

/// What a walk over an expression finds.
fn found<'a>(expression: &Expression<'a>) -> Found<'a> {
    let mut found = Found::default();
    found.visit_expression(expression);
    found
}

/// The names an expression reads.
#[must_use]
pub fn read_names<'a>(expression: &Expression<'a>) -> Vec<&'a str> {
    found(expression).names
}

/// The names a statement reads.
#[must_use]
pub fn statement_names<'a>(statement: &Statement<'a>) -> Vec<&'a str> {
    let mut found = Found::default();
    found.visit_statement(statement);
    found.names
}

/// Whether an expression may change anything: a call, an assignment, an update, a `delete`, a
/// `new`, an `await` or a `yield` (`hasSideEffect` of `@eslint-community/eslint-utils`, getters
/// left out).
#[must_use]
pub fn has_side_effect(expression: &Expression) -> bool {
    found(expression).side_effect
}

/// Whether an expression awaits or yields.
#[must_use]
pub fn suspends(expression: &Expression) -> bool {
    found(expression).suspends
}

/// Whether an expression may change anything or run a getter: `has_side_effect`, member reads
/// and comparisons or arithmetic that may convert their operands included
/// (`considerGetters` and `considerImplicitTypeConversion`).
#[must_use]
pub fn may_run_code(expression: &Expression) -> bool {
    /// The search for a member read or a converting operator.
    #[derive(Default)]
    struct Search {
        found: bool,
    }
    impl<'a> Visit<'a> for Search {
        fn visit_expression(&mut self, expression: &Expression<'a>) {
            match expression {
                Expression::StaticMemberExpression(_)
                | Expression::ComputedMemberExpression(_)
                | Expression::PrivateFieldExpression(_) => self.found = true,
                Expression::BinaryExpression(binary) => {
                    let strict = matches!(
                        binary.operator,
                        oxc_ast::ast::BinaryOperator::StrictEquality
                            | oxc_ast::ast::BinaryOperator::StrictInequality
                    );
                    let literals = binary.left.is_literal() && binary.right.is_literal();
                    self.found |= !strict && !literals;
                }
                _ => {}
            }
            walk::walk_expression(self, expression);
        }
    }
    let mut search = Search::default();
    search.visit_expression(expression);
    search.found || has_side_effect(expression)
}

/// Whether two expressions are the same reference: the same name, `this`, `super`, literal, or
/// member read with the same key from the same object, TypeScript's wrappers left out
/// (`utils/is-same-reference.js`).
#[must_use]
pub fn same_reference(left: &Expression, right: &Expression) -> bool {
    let (left, right) = (unwrapped(left), unwrapped(right));
    match (left, right) {
        (Expression::Identifier(a), Expression::Identifier(b)) => a.name == b.name,
        (Expression::ThisExpression(_), Expression::ThisExpression(_))
        | (Expression::Super(_), Expression::Super(_))
        | (Expression::NullLiteral(_), Expression::NullLiteral(_)) => true,
        (Expression::StringLiteral(a), Expression::StringLiteral(b)) => a.value == b.value,
        (Expression::NumericLiteral(a), Expression::NumericLiteral(b)) => {
            a.value.to_bits() == b.value.to_bits()
        }
        (Expression::BooleanLiteral(a), Expression::BooleanLiteral(b)) => a.value == b.value,
        (Expression::StaticMemberExpression(a), Expression::StaticMemberExpression(b)) => {
            a.property.name == b.property.name && same_reference(&a.object, &b.object)
        }
        (Expression::ComputedMemberExpression(a), Expression::ComputedMemberExpression(b)) => {
            same_reference(&a.expression, &b.expression) && same_reference(&a.object, &b.object)
        }
        (Expression::StaticMemberExpression(a), Expression::ComputedMemberExpression(b))
        | (Expression::ComputedMemberExpression(b), Expression::StaticMemberExpression(a)) => {
            matches!(&b.expression, Expression::StringLiteral(key) if key.value == a.property.name)
                && same_reference(&a.object, &b.object)
        }
        _ => false,
    }
}

/// An expression without its TypeScript wrappers, `as`, `satisfies`, `<T>` and `!`, nor parentheses.
#[must_use]
pub fn unwrapped<'s, 'a>(expression: &'s Expression<'a>) -> &'s Expression<'a> {
    match expression {
        Expression::ParenthesizedExpression(inner) => unwrapped(&inner.expression),
        Expression::TSAsExpression(inner) => unwrapped(&inner.expression),
        Expression::TSSatisfiesExpression(inner) => unwrapped(&inner.expression),
        Expression::TSNonNullExpression(inner) => unwrapped(&inner.expression),
        Expression::TSTypeAssertion(inner) => unwrapped(&inner.expression),
        other => other,
    }
}

/// Whether a node is written rather than read: assigned, updated, deleted, destructured into or
/// a loop's target (`utils/is-left-hand-side.js`).
#[must_use]
pub fn is_written(source: &Source, node: &AstNode) -> bool {
    let span = node.kind().span();
    match source.semantic.nodes().parent_kind(node.id()) {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == span,
        AstKind::ForOfStatement(statement) => statement.left.span() == span,
        AstKind::ForInStatement(statement) => statement.left.span() == span,
        AstKind::AssignmentTargetWithDefault(target) => target.binding.span() == span,
        AstKind::UpdateExpression(_)
        | AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetPropertyProperty(_)
        | AstKind::AssignmentTargetRest(_) => true,
        AstKind::UnaryExpression(unary) => unary.operator == UnaryOperator::Delete,
        _ => false,
    }
}

/// A range's text with edits applied, each a range of the file and its new text; the edits
/// lie inside the range and do not overlap.
#[must_use]
pub fn edited(source: &Source, range: Span, edits: &mut [(u32, u32, String)]) -> String {
    edits.sort_by_key(|(start, end, _)| (*start, *end));
    let mut result = String::new();
    let mut at = range.start;
    for (start, end, replacement) in edits.iter() {
        result.push_str(text(source, Span::new(at, *start)));
        result.push_str(replacement);
        at = *end;
    }
    result.push_str(text(source, Span::new(at, range.end)));
    result
}

/// The blanks that open the line of an offset.
#[must_use]
pub fn line_indent<'a>(source: &Source<'_, 'a>, offset: u32) -> &'a str {
    let before = &source.text[..offset as usize];
    let start = before.rfind('\n').map_or(0, |at| at + 1);
    let line = &source.text[start..];
    let blanks = line.len() - line.trim_start_matches([' ', '\t']).len();
    &line[..blanks]
}

/// A text whose lines start with one indent moved to start with another
/// (`utils/reindent-text.js`).
#[must_use]
pub fn reindent(text: &str, from: &str, to: &str) -> String {
    text.split('\n')
        .enumerate()
        .map(|(index, line)| match line.strip_prefix(from) {
            Some(rest) if index > 0 || !from.is_empty() => format!("{to}{rest}"),
            _ => line.to_owned(),
        })
        .collect::<Vec<String>>()
        .join("\n")
}

/// Whether an expression needs parentheses as the operand of a unary operator: anything but a
/// name, a literal, a member access, a call or another unary expression.
#[must_use]
pub fn needs_parentheses_as_operand(expression: &Expression) -> bool {
    !matches!(
        expression,
        Expression::Identifier(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::StringLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::ThisExpression(_)
            | Expression::ArrayExpression(_)
            | Expression::ObjectExpression(_)
            | Expression::CallExpression(_)
            | Expression::NewExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::StaticMemberExpression(_)
            | Expression::PrivateFieldExpression(_)
            | Expression::ChainExpression(_)
            | Expression::UnaryExpression(_)
            | Expression::ParenthesizedExpression(_)
            | Expression::TSNonNullExpression(_)
    )
}

/// The text of an expression negated with `!`, parenthesized when needed; `!a` gives `!!a`
/// (`utils/get-negated-expression-text.js`).
#[must_use]
pub fn negated_text(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    if needs_parentheses_as_operand(expression) {
        format!("!({inner})")
    } else {
        format!("!{inner}")
    }
}

#[cfg(test)]
mod tests {
    use oxc_ast::AstKind;
    use oxc_semantic::AstNode;
    use oxc_span::GetSpan;

    use super::{is_boolean, is_boolean_context, same_tokens};
    use crate::finding::Finding;
    use crate::rules::Rule;
    use crate::source::Source;
    use crate::testing::lint;

    /// Reports each value given to `probe(…)` that is a boolean, and each `probe` read for its
    /// truth.
    struct Probe;

    impl Rule for Probe {
        fn name(&self) -> &'static str {
            "probe/boolean"
        }

        fn needs_tokens(&self) -> bool {
            true
        }

        fn checks_nodes(&self) -> bool {
            true
        }

        fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
            match node.kind() {
                AstKind::CallExpression(call) if call.callee.is_specific_id("probe") => {
                    if let Some(argument) = call.arguments[0].as_expression()
                        && is_boolean(source, argument)
                    {
                        findings.push(source.finding(
                            self.name(),
                            call.span.start,
                            "boolean".into(),
                        ));
                    }
                }
                AstKind::IdentifierReference(name) if name.name == "context" => {
                    if is_boolean_context(source, node.id()) {
                        findings.push(source.finding(
                            self.name(),
                            name.span.start,
                            "context".into(),
                        ));
                    }
                }
                AstKind::IfStatement(statement) => {
                    if let Some(alternate) = &statement.alternate
                        && same_tokens(source, statement.consequent.span(), alternate.span())
                    {
                        findings.push(source.finding(
                            self.name(),
                            statement.span.start,
                            "same".into(),
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    /// The lines of the findings with a message.
    fn lines(code: &str, message: &str) -> Vec<usize> {
        lint(&Probe, "scripts/a.ts", code)
            .iter()
            .filter(|finding| finding.message == message)
            .map(|finding| finding.line)
            .collect()
    }

    #[test]
    fn tells_a_boolean_from_the_tree() {
        let code = "const yes = a > 1;\nconst no = a + 1;\nfunction f(flag: boolean, n: number, other?: boolean): boolean { return true; }\nconst g = (x: number) => x > 1;\nconst h = async () => true;\nfunction k() { return true; }\nprobe(yes);\nprobe(no);\nprobe(Number.isInteger(a));\nprobe(Number.parseInt(a));\nprobe(f(true, true, 1));\nprobe(g(1));\nprobe(h());\nprobe(k());\nprobe(a ? true : !b);\nprobe(a ?? true);\nprobe((a, b > 1));\nprobe(a as boolean);\nprobe(a! satisfies unknown);\nprobe(isNaN(a));\nprobe(Boolean(a));\nprobe(x = true);\nfunction m(flag: boolean, other?: boolean) { probe(flag); probe(other); }\nprobe(yes && Array.isArray(a));\nprobe(Foo.isArray(a));\nprobe(a.b.c(1));";

        assert_eq!(
            lines(code, "boolean"),
            [7, 9, 11, 12, 15, 17, 18, 20, 21, 22, 23, 24]
        );
    }

    #[test]
    fn tells_a_value_read_for_its_truth() {
        let code = "if (context && a) {}\nconst b = context || c;\nwhile (!context) {}\nconst d = Boolean(context);\nconst e = context ? 1 : 2;\nfor (; context;) {}\ndo {} while (context);\nconst f = g(context);\nconst h = context ?? i;";

        assert_eq!(lines(code, "context"), [1, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn compares_tokens_without_layout_comments_or_last_semicolon() {
        let code =
            "if (a) b( 1 /* x */ ); else b(1)\nif (a) { b(1) } else { b(2) }\nif (a) {} else {}";

        assert_eq!(lines(code, "same"), [1, 3]);
    }
}
