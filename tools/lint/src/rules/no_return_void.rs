//! `functional/no-return-void`: a function of the functional core returns a value — neither
//! `void`, `undefined` nor `null` (ADR-0014, `REF-FUNCTIONAL-PLUGIN`). A written return type is
//! read as the plugin read it; without one, the plugin asked the compiler for the inferred type,
//! which Oxc has not: the body is read instead — a block that returns no value, or an arrow
//! whose expression is `undefined`, `void …` or `null`, returns nothing. A call's result is not
//! followed (a known loss, beside `typescript/explicit-function-return-type`, which asks for the
//! written type).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrowFunctionBody, ArrowFunctionExpression, Expression, Function, FunctionBody, FunctionType,
    Statement, TSTypeAnnotation, UnaryOperator,
};
use oxc_ast_visit::Visit;
use oxc_ast_visit::walk::walk_function_body;
use oxc_semantic::{AstNode, ScopeFlags};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoReturnVoid;

/// The rule's message.
const MESSAGE: &str = "Function must return a value.";

/// Whether a written return type is `void`, `undefined` or `null`.
fn is_nothing(annotation: &TSTypeAnnotation) -> bool {
    use oxc_ast::ast::TSType;
    matches!(
        annotation.type_annotation,
        TSType::TSVoidKeyword(_) | TSType::TSUndefinedKeyword(_) | TSType::TSNullKeyword(_)
    )
}

/// The walk that looks for a `return` with a value, nested functions left out.
#[derive(Default)]
struct Returns {
    with_value: bool,
}

impl<'a> Visit<'a> for Returns {
    fn visit_return_statement(&mut self, statement: &oxc_ast::ast::ReturnStatement<'a>) {
        self.with_value |= statement.argument.is_some();
    }

    fn visit_function(&mut self, _function: &Function<'a>, _flags: ScopeFlags) {}

    fn visit_arrow_function_expression(&mut self, _arrow: &ArrowFunctionExpression<'a>) {}
}

/// Whether a block returns nothing: no `return` with a value, and an end the code reaches —
/// a block ending on `throw` returns `never`, as the compiler infers for an expression.
fn block_returns_nothing(body: &FunctionBody, ends_reachable_matters: bool) -> bool {
    let mut returns = Returns::default();
    walk_function_body(&mut returns, body);
    let ends_on_throw = matches!(body.statements.last(), Some(Statement::ThrowStatement(_)));

    let never = ends_reachable_matters && ends_on_throw;

    !(returns.with_value || never)
}

/// Whether an expression is `undefined`, `void …` or `null`.
fn is_nothing_expression(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(identifier) => identifier.name == "undefined",
        Expression::UnaryExpression(unary) => unary.operator == UnaryOperator::Void,
        Expression::NullLiteral(_) => true,
        _ => false,
    }
}

/// Where a function that returns nothing is reported: its written type, or the function.
fn finding_at(node: &AstNode) -> Option<u32> {
    match node.kind() {
        AstKind::Function(function) => function_finding(function),
        AstKind::ArrowFunctionExpression(arrow) => match &arrow.return_type {
            Some(annotation) => is_nothing(annotation).then_some(annotation.span.start),
            None if arrow.r#async => None,
            None => match &arrow.body {
                ArrowFunctionBody::FunctionBody(body) => {
                    block_returns_nothing(body, true).then_some(arrow.span.start)
                }
                expression => expression
                    .as_expression()
                    .filter(|expression| is_nothing_expression(expression))
                    .map(|_| arrow.span.start),
            },
        },
        AstKind::TSFunctionType(function) => {
            is_nothing(&function.return_type).then_some(function.return_type.span.start)
        }
        AstKind::TSCallSignatureDeclaration(signature) => written(signature.return_type.as_deref()),
        AstKind::TSConstructSignatureDeclaration(signature) => {
            written(signature.return_type.as_deref())
        }
        AstKind::TSMethodSignature(signature) => written(signature.return_type.as_deref()),
        _ => None,
    }
}

/// The place of a written return type of nothing.
fn written(annotation: Option<&TSTypeAnnotation>) -> Option<u32> {
    annotation
        .filter(|annotation| is_nothing(annotation))
        .map(|annotation| annotation.span.start)
}

/// Where a function declaration or expression that returns nothing is reported.
fn function_finding(function: &Function) -> Option<u32> {
    if let Some(annotation) = &function.return_type {
        return written(Some(annotation));
    }
    let inferred = matches!(
        function.r#type,
        FunctionType::FunctionDeclaration | FunctionType::FunctionExpression
    ) && !function.r#async
        && !function.generator;
    let is_expression = function.r#type == FunctionType::FunctionExpression;

    function
        .body
        .as_ref()
        .filter(|body| inferred && block_returns_nothing(body, is_expression))
        .map(|_| function.span.start)
}

impl Rule for NoReturnVoid {
    fn name(&self) -> &'static str {
        "functional/no-return-void"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let Some(at) = finding_at(node) {
            findings.push(source.finding(self.name(), at, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoReturnVoid;
    use crate::testing::lint;

    /// The columns of the findings on one line of code.
    fn columns(code: &str) -> Vec<usize> {
        lint(&NoReturnVoid, "src/math/a.ts", code)
            .iter()
            .map(|finding| finding.column)
            .collect()
    }

    #[test]
    fn reports_a_written_return_type_of_nothing() {
        // [F19.AC3] `functional/no-return-void`, a written return type of nothing.
        // The plugin's documentation: `function updateText(): void {}`; the finding on `: void`.
        assert_eq!(columns("function updateText(): void {}"), [22]);
        assert_eq!(
            columns("function f(): undefined { return undefined; }"),
            [13]
        );
        assert_eq!(columns("const f = (): null => null;"), [13]);
    }

    #[test]
    fn reports_signatures_and_function_types_of_nothing() {
        assert_eq!(columns("type F = (a: number) => void;"), [22]);
        assert_eq!(
            columns("type T = { m(): void; (): void; new (): void };"),
            [15, 25, 39]
        );
        assert_eq!(columns("declare function f(): void;"), [21]);
    }

    #[test]
    fn reports_a_body_that_returns_nothing() {
        assert_eq!(columns("function f(a: number) { a; }"), [1]);
        assert_eq!(columns("const f = function () { return; };"), [11]);
        assert_eq!(columns("const f = () => {};"), [11]);
        assert_eq!(columns("const f = () => undefined;"), [11]);
        assert_eq!(columns("const f = () => void 0;"), [11]);
        // The inner arrow's `return` is its own: the outer function returns nothing.
        assert_eq!(
            columns("function f() { const g = () => { return 1; }; g; }"),
            [1]
        );
    }

    #[test]
    fn accepts_a_function_that_returns_a_value() {
        for code in [
            "function updateText(value: string): string { return value; }",
            "async function f(): Promise<void> {}",
            "async function f() {}",
            "function* f() {}",
            "const f = async () => {};",
            "const f = (a: number) => a;",
            "const f = (a: () => number) => a();",
            "function f(a: boolean) { if (a) { return 1; } return 2; }",
            "const f = () => { throw new Error(); };",
            "type F = () => number;",
            "type T = { m(): string; n(); };",
        ] {
            assert_eq!(columns(code), Vec::<usize>::new(), "{code}");
        }
    }
}
