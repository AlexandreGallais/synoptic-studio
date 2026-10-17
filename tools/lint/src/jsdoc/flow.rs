//! What a function's body does, as `eslint-plugin-jsdoc` 65.3.1 reads it from the tree alone:
//! whether it throws (`hasThrowValue`), rejects (`hasRejectValue` of `requireRejects.js`),
//! returns a value (`utils/hasReturnValue.js`) or yields one (`hasNonFunctionYield`). Each walks
//! only the statements and expressions its source names, as the plugin does.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrayExpressionElement, ArrowFunctionBody, ArrowFunctionExpression, BindingPattern,
    ChainElement, Expression, FormalParameters, Function, MemberExpression, ObjectPropertyKind,
    PropertyKey, Statement, TSType, TSTypeAnnotation,
};

/// A function of the tree, by its kind.
#[derive(Clone, Copy)]
pub enum FunctionNode<'s, 'a> {
    /// A function declaration, expression or declared function.
    Function(&'s Function<'a>),
    /// An arrow function.
    Arrow(&'s ArrowFunctionExpression<'a>),
}

impl<'s, 'a> FunctionNode<'s, 'a> {
    /// Whether it is `async`.
    #[must_use]
    pub fn is_async(self) -> bool {
        match self {
            Self::Function(function) => function.r#async,
            Self::Arrow(arrow) => arrow.r#async,
        }
    }

    /// Whether it is a generator.
    #[must_use]
    pub fn is_generator(self) -> bool {
        matches!(self, Self::Function(function) if function.generator)
    }

    /// Its body's statements, when it has a block body.
    #[must_use]
    pub fn statements(self) -> Option<&'s [Statement<'a>]> {
        match self {
            Self::Function(function) => function
                .body
                .as_deref()
                .map(|body| body.statements.as_slice()),
            Self::Arrow(arrow) => match &arrow.body {
                ArrowFunctionBody::FunctionBody(body) => Some(body.statements.as_slice()),
                _ => None,
            },
        }
    }

    /// Its body's expression, for an arrow function written without a block.
    #[must_use]
    pub fn expression(self) -> Option<&'s Expression<'a>> {
        match self {
            Self::Arrow(arrow) => arrow.body.as_expression(),
            Self::Function(_) => None,
        }
    }

    /// Its parameters.
    #[must_use]
    pub fn params(self) -> &'s FormalParameters<'a> {
        match self {
            Self::Function(function) => &function.params,
            Self::Arrow(arrow) => &arrow.params,
        }
    }

    /// Its written return type.
    #[must_use]
    pub fn return_type(self) -> Option<&'s TSTypeAnnotation<'a>> {
        match self {
            Self::Function(function) => function.return_type.as_deref(),
            Self::Arrow(arrow) => arrow.return_type.as_deref(),
        }
    }

    /// Whether it is a declared function, without body.
    #[must_use]
    pub fn is_declared(self) -> bool {
        matches!(self, Self::Function(function) if function.body.is_none())
    }
}

/// The function an expression is, if any.
fn function_of<'s, 'a>(expression: &'s Expression<'a>) -> Option<FunctionNode<'s, 'a>> {
    match expression {
        Expression::FunctionExpression(function) => Some(FunctionNode::Function(function)),
        Expression::ArrowFunctionExpression(arrow) => Some(FunctionNode::Arrow(arrow)),
        _ => None,
    }
}

/// Whether a statement throws, a function declared inside left out (`hasThrowValue`).
fn statement_throws(statement: &Statement) -> bool {
    match statement {
        Statement::BlockStatement(block) => block_throws(&block.body),
        Statement::DoWhileStatement(each) => statement_throws(&each.body),
        Statement::ForInStatement(each) => statement_throws(&each.body),
        Statement::ForOfStatement(each) => statement_throws(&each.body),
        Statement::ForStatement(each) => statement_throws(&each.body),
        Statement::LabeledStatement(each) => statement_throws(&each.body),
        Statement::WhileStatement(each) => statement_throws(&each.body),
        Statement::WithStatement(each) => statement_throws(&each.body),
        Statement::IfStatement(each) => {
            statement_throws(&each.consequent)
                || each.alternate.as_ref().is_some_and(statement_throws)
        }
        Statement::SwitchStatement(each) => each
            .cases
            .iter()
            .any(|case| case.consequent.iter().any(statement_throws)),
        Statement::ThrowStatement(_) => true,
        Statement::TryStatement(each) => {
            each.handler
                .as_ref()
                .is_some_and(|handler| block_throws(&handler.body.body))
                || each
                    .finalizer
                    .as_ref()
                    .is_some_and(|finalizer| block_throws(&finalizer.body))
        }
        _ => false,
    }
}

/// Whether a block's statements throw.
fn block_throws(statements: &[Statement]) -> bool {
    statements.iter().any(|statement| {
        !matches!(statement, Statement::FunctionDeclaration(_)) && statement_throws(statement)
    })
}

/// Whether a function throws: not `async`, and a `throw` in its body outside a `try` block and
/// any inner function (`hasThrowValue`).
#[must_use]
pub fn throws(function: FunctionNode) -> bool {
    !function.is_async() && function.statements().is_some_and(block_throws)
}

/// Whether an expression calls `Promise.reject()` or `reject()`, or builds a promise whose
/// executor rejects (`hasRejectValue` on expressions).
fn expression_rejects(expression: &Expression, inner: bool, is_async: Option<bool>) -> bool {
    match expression {
        Expression::CallExpression(call) => match &call.callee {
            Expression::StaticMemberExpression(member) => {
                matches!(&member.object, Expression::Identifier(object) if object.name == "Promise")
                    && member.property.name == "reject"
            }
            Expression::Identifier(callee) => callee.name == "reject",
            _ => false,
        },
        Expression::NewExpression(new) => {
            matches!(&new.callee, Expression::Identifier(callee) if callee.name == "Promise")
                && new
                    .arguments
                    .first()
                    .and_then(Argument::as_expression)
                    .and_then(function_of)
                    .is_some_and(|executor| body_rejects(executor, false, Some(false)))
        }
        _ => function_of(expression)
            .is_some_and(|function| function_rejects(function, inner, is_async)),
    }
}

/// Whether a function's body rejects.
fn body_rejects(function: FunctionNode, inner: bool, is_async: Option<bool>) -> bool {
    match (function.statements(), function.expression()) {
        (Some(statements), _) => statements
            .iter()
            .any(|statement| statement_rejects(statement, inner, is_async)),
        (None, Some(expression)) => expression_rejects(expression, inner, is_async),
        (None, None) => false,
    }
}

/// Whether a function rejects: from the top, its body; inside it, its body as part of the
/// caller's (`hasRejectValue` on functions).
fn function_rejects(function: FunctionNode, inner: bool, is_async: Option<bool>) -> bool {
    if inner {
        let inner_async = function.is_async() || is_async == Some(true);
        body_rejects(function, false, Some(inner_async))
    } else {
        body_rejects(function, true, Some(function.is_async()))
    }
}

/// Whether a statement rejects (`hasRejectValue` on statements).
fn statement_rejects(statement: &Statement, inner: bool, is_async: Option<bool>) -> bool {
    let each = |statement: &Statement| statement_rejects(statement, inner, is_async);
    match statement {
        Statement::FunctionDeclaration(function) => {
            function_rejects(FunctionNode::Function(function), inner, is_async)
        }
        Statement::BlockStatement(block) => block.body.iter().any(each),
        Statement::DoWhileStatement(loop_) => each(&loop_.body),
        Statement::ForInStatement(loop_) => each(&loop_.body),
        Statement::ForOfStatement(loop_) => each(&loop_.body),
        Statement::ForStatement(loop_) => each(&loop_.body),
        Statement::LabeledStatement(labeled) => each(&labeled.body),
        Statement::WhileStatement(loop_) => each(&loop_.body),
        Statement::WithStatement(with) => each(&with.body),
        Statement::ExpressionStatement(statement) => {
            expression_rejects(&statement.expression, inner, is_async)
        }
        Statement::IfStatement(branch) => {
            each(&branch.consequent) || branch.alternate.as_ref().is_some_and(each)
        }
        Statement::ReturnStatement(statement) => statement
            .argument
            .as_ref()
            .is_some_and(|argument| expression_rejects(argument, inner, is_async)),
        Statement::SwitchStatement(switch) => switch
            .cases
            .iter()
            .any(|case| case.consequent.iter().any(each)),
        Statement::ThrowStatement(_) => is_async == Some(true),
        Statement::TryStatement(attempt) => {
            attempt
                .handler
                .as_ref()
                .is_some_and(|handler| handler.body.body.iter().any(each))
                || attempt
                    .finalizer
                    .as_ref()
                    .is_some_and(|finalizer| finalizer.body.iter().any(each))
        }
        _ => false,
    }
}

/// Whether a function may reject (`hasRejectValue` from the function).
#[must_use]
pub fn rejects(function: FunctionNode) -> bool {
    function_rejects(function, false, None)
}

/// An empty `return` met where the plugin throws `Null return`.
pub struct NullReturn;

/// What a `new Promise(…)` returned counts as.
pub type PromiseFilter<'f> = &'f dyn Fn(&Expression) -> bool;

/// Whether an expression is `new Promise(…)` (`isNewPromiseExpression`).
fn is_new_promise(expression: &Expression) -> bool {
    matches!(expression, Expression::NewExpression(new) if matches!(&new.callee, Expression::Identifier(callee) if callee.name == "Promise"))
}

/// Whether an expression is `new Promise<void>(…)` (`isVoidPromise`).
fn is_void_promise(expression: &Expression) -> bool {
    matches!(expression, Expression::NewExpression(new)
        if new.type_arguments.as_ref().and_then(|arguments| arguments.params.first()).is_some_and(|first| matches!(first, TSType::TSVoidKeyword(_))))
}

/// Whether a written return type is a type other than `never`, `undefined` or `void`.
fn returns_typed(annotation: Option<&TSTypeAnnotation>) -> bool {
    annotation.is_some_and(|annotation| {
        !matches!(
            annotation.type_annotation,
            TSType::TSNeverKeyword(_) | TSType::TSUndefinedKeyword(_) | TSType::TSVoidKeyword(_)
        )
    })
}

/// Whether a statement returns a value; an empty `return` an error when asked
/// (`hasReturnValue`).
fn statement_returns(
    statement: &Statement,
    null_fails: bool,
    filter: Option<PromiseFilter>,
) -> Result<bool, NullReturn> {
    let each = |statement: &Statement| statement_returns(statement, null_fails, filter);
    Ok(match statement {
        Statement::BlockStatement(block) => block_returns(&block.body, null_fails, filter)?,
        Statement::DoWhileStatement(loop_) => each(&loop_.body)?,
        Statement::ForInStatement(loop_) => each(&loop_.body)?,
        Statement::ForOfStatement(loop_) => each(&loop_.body)?,
        Statement::ForStatement(loop_) => each(&loop_.body)?,
        Statement::LabeledStatement(labeled) => each(&labeled.body)?,
        Statement::WhileStatement(loop_) => each(&loop_.body)?,
        Statement::WithStatement(with) => each(&with.body)?,
        Statement::IfStatement(branch) => {
            each(&branch.consequent)?
                || branch
                    .alternate
                    .as_ref()
                    .map(each)
                    .transpose()?
                    .unwrap_or(false)
        }
        Statement::ReturnStatement(statement) => match &statement.argument {
            None if null_fails => return Err(NullReturn),
            None => false,
            Some(argument) => match filter {
                Some(filter) if is_new_promise(argument) => filter(argument),
                _ => true,
            },
        },
        Statement::SwitchStatement(switch) => {
            let mut found = false;
            for case in &switch.cases {
                for statement in &case.consequent {
                    if each(statement)? {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            found
        }
        Statement::TryStatement(attempt) => {
            block_returns(&attempt.block.body, null_fails, filter)?
                || attempt
                    .handler
                    .as_ref()
                    .map(|handler| block_returns(&handler.body.body, null_fails, filter))
                    .transpose()?
                    .unwrap_or(false)
                || attempt
                    .finalizer
                    .as_ref()
                    .map(|finalizer| block_returns(&finalizer.body, null_fails, filter))
                    .transpose()?
                    .unwrap_or(false)
        }
        _ => false,
    })
}

/// Whether a block's statements return a value, function declarations left out.
fn block_returns(
    statements: &[Statement],
    null_fails: bool,
    filter: Option<PromiseFilter>,
) -> Result<bool, NullReturn> {
    for statement in statements {
        if !matches!(statement, Statement::FunctionDeclaration(_))
            && statement_returns(statement, null_fails, filter)?
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether a function returns a value: an arrow function's expression (unless a
/// `new Promise<void>`), a `return` with a value in its body, or a declared function's
/// written type (`hasReturnValue` on functions).
///
/// # Errors
///
/// An empty `return`, when asked.
pub fn returns(
    function: FunctionNode,
    null_fails: bool,
    filter: Option<PromiseFilter>,
) -> Result<bool, NullReturn> {
    if function.is_declared() {
        return Ok(returns_typed(function.return_type()));
    }
    if let Some(expression) = function.expression()
        && (!is_new_promise(expression) || !is_void_promise(expression))
    {
        return Ok(true);
    }
    match function.statements() {
        Some(statements) => block_returns(statements, null_fails, filter),
        None => Ok(false),
    }
}

/// Whether every branch of a statement returns a value or throws (`allBrancheshaveReturnValues`).
fn statement_always_returns(statement: &Statement, filter: Option<PromiseFilter>) -> bool {
    match statement {
        Statement::BlockStatement(block) => block
            .body
            .last()
            .is_some_and(|last| statement_always_returns(last, filter)),
        Statement::DoWhileStatement(loop_) => {
            if matches!(&loop_.test, Expression::BooleanLiteral(literal) if literal.value) {
                statement_returns(&loop_.body, false, filter).unwrap_or(false)
            } else {
                statement_always_returns(&loop_.body, filter)
            }
        }
        Statement::WhileStatement(loop_) => {
            if matches!(&loop_.test, Expression::BooleanLiteral(literal) if literal.value) {
                statement_returns(&loop_.body, false, filter).unwrap_or(false)
            } else {
                statement_always_returns(&loop_.body, filter)
            }
        }
        Statement::ForStatement(loop_) => {
            if loop_.test.is_none() {
                statement_returns(&loop_.body, false, filter).unwrap_or(false)
            } else {
                statement_always_returns(&loop_.body, filter)
            }
        }
        Statement::ForInStatement(loop_) => statement_always_returns(&loop_.body, filter),
        Statement::ForOfStatement(loop_) => statement_always_returns(&loop_.body, filter),
        Statement::LabeledStatement(labeled) => statement_always_returns(&labeled.body, filter),
        Statement::WithStatement(with) => statement_always_returns(&with.body, filter),
        Statement::IfStatement(branch) => {
            statement_always_returns(&branch.consequent, filter)
                && branch
                    .alternate
                    .as_ref()
                    .is_some_and(|alternate| statement_always_returns(alternate, filter))
        }
        Statement::ReturnStatement(statement) => match &statement.argument {
            None => false,
            Some(argument) => match filter {
                Some(filter) if is_new_promise(argument) => filter(argument),
                _ => true,
            },
        },
        Statement::SwitchStatement(switch) => switch.cases.iter().all(|case| {
            !case.consequent.iter().any(|statement| {
                matches!(statement, Statement::BreakStatement(_))
                    || matches!(statement, Statement::ReturnStatement(statement) if statement.argument.is_none())
            })
        }),
        Statement::ThrowStatement(_) => true,
        Statement::TryStatement(attempt) => {
            let finalizer_returns = attempt.finalizer.as_ref().is_some_and(|finalizer| {
                finalizer
                    .body
                    .last()
                    .is_some_and(|last| statement_always_returns(last, filter))
            });
            let block = attempt
                .block
                .body
                .last()
                .is_some_and(|last| statement_always_returns(last, filter));
            let handler = attempt.handler.as_ref().is_none_or(|handler| {
                handler
                    .body
                    .body
                    .last()
                    .is_some_and(|last| statement_always_returns(last, filter))
            });
            let finalizer_no_empty = attempt
                .finalizer
                .as_ref()
                .is_none_or(|finalizer| block_returns(&finalizer.body, true, filter).is_ok());
            finalizer_returns || (block && handler && finalizer_no_empty)
        }
        _ => false,
    }
}

/// Whether every branch of a function returns a value (`allBrancheshaveReturnValues` on
/// functions).
#[must_use]
pub fn always_returns(function: FunctionNode, filter: Option<PromiseFilter>) -> bool {
    if function.is_declared() {
        return returns_typed(function.return_type());
    }
    if let Some(expression) = function.expression() {
        return !is_new_promise(expression) || !is_void_promise(expression);
    }
    let statements = function.statements().unwrap_or_default();
    statements
        .last()
        .is_some_and(|last| statement_always_returns(last, filter))
        || statements
            .iter()
            .any(|statement| matches!(statement, Statement::ReturnStatement(_)))
}

/// Whether a function returns a value, or a promise whose executor resolves with one
/// (`hasValueOrExecutorHasNonEmptyResolveValue`), on every branch when asked.
#[must_use]
pub fn returns_value_or_resolves(
    function: FunctionNode,
    any_promise: bool,
    all_branches: bool,
) -> bool {
    let filter = |promise: &Expression| {
        if any_promise {
            return true;
        }
        if is_void_promise(promise) {
            return false;
        }
        let Expression::NewExpression(new) = promise else {
            return false;
        };
        let Some(executor) = new
            .arguments
            .first()
            .and_then(Argument::as_expression)
            .and_then(function_of)
        else {
            return false;
        };
        let Some(first) = executor.params().items.first() else {
            return false;
        };
        let BindingPattern::BindingIdentifier(resolver) = &first.pattern else {
            return false;
        };
        resolves_function_body(executor, &resolver.name)
    };
    if all_branches {
        match returns(function, true, Some(&filter)) {
            Ok(returned) => returned && always_returns(function, Some(&filter)),
            Err(NullReturn) => false,
        }
    } else {
        returns(function, false, Some(&filter)).unwrap_or(false)
    }
}

/// Whether a function's body calls a resolver with a value or passes it on.
fn resolves_function_body(function: FunctionNode, resolver: &str) -> bool {
    match (function.statements(), function.expression()) {
        (Some(statements), _) => statements
            .iter()
            .any(|statement| statement_resolves(statement, resolver)),
        (None, Some(expression)) => expression_resolves(expression, resolver),
        (None, None) => false,
    }
}

/// Whether a function shadows a resolver with its first parameter, or calls it in its body.
fn function_resolves(function: FunctionNode, resolver: &str) -> bool {
    let shadows = function.params().items.first().is_some_and(|first| {
        matches!(&first.pattern, BindingPattern::BindingIdentifier(id) if id.name == resolver)
    });
    !shadows && resolves_function_body(function, resolver)
}

/// Whether a key or value of an object reads a resolver call.
fn key_resolves(key: &PropertyKey, computed: bool, resolver: &str) -> bool {
    computed
        && key
            .as_expression()
            .is_some_and(|key| expression_resolves(key, resolver))
}

/// Whether an expression calls a resolver with a value or passes it on
/// (`hasNonEmptyResolverCall` on expressions).
fn expression_resolves(expression: &Expression, resolver: &str) -> bool {
    let each = |expression: &Expression| expression_resolves(expression, resolver);
    match expression {
        Expression::ArrayExpression(array) => array.elements.iter().any(|element| match element {
            ArrayExpressionElement::SpreadElement(spread) => each(&spread.argument),
            ArrayExpressionElement::Elision(_) => false,
            other => other.as_expression().is_some_and(each),
        }),
        Expression::FunctionExpression(function) => {
            function_resolves(FunctionNode::Function(function), resolver)
        }
        Expression::ArrowFunctionExpression(arrow) => {
            function_resolves(FunctionNode::Arrow(arrow), resolver)
        }
        Expression::AssignmentExpression(assignment) => {
            assignment
                .left
                .as_member_expression()
                .is_some_and(|member| member_resolves(member, resolver))
                || each(&assignment.right)
        }
        Expression::BinaryExpression(binary) => each(&binary.left) || each(&binary.right),
        Expression::LogicalExpression(logical) => each(&logical.left) || each(&logical.right),
        Expression::AwaitExpression(await_) => each(&await_.argument),
        Expression::UnaryExpression(unary) => each(&unary.argument),
        Expression::YieldExpression(yield_) => yield_.argument.as_ref().is_some_and(each),
        Expression::CallExpression(call) => {
            let named =
                matches!(&call.callee, Expression::Identifier(callee) if callee.name == resolver);
            (named && !call.arguments.is_empty())
                || call.arguments.iter().any(|argument| match argument {
                    Argument::SpreadElement(spread) => each(&spread.argument),
                    other => other.as_expression().is_some_and(|argument| {
                        matches!(argument, Expression::Identifier(id) if id.name == resolver)
                            || each(argument)
                    }),
                })
        }
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(call) => {
                let named = matches!(&call.callee, Expression::Identifier(callee) if callee.name == resolver);
                (named && !call.arguments.is_empty())
                    || call.arguments.iter().any(|argument| {
                        argument.as_expression().is_some_and(|argument| {
                            matches!(argument, Expression::Identifier(id) if id.name == resolver)
                                || each(argument)
                        })
                    })
            }
            other => other
                .as_member_expression()
                .is_some_and(|member| member_resolves(member, resolver)),
        },
        Expression::ConditionalExpression(conditional) => {
            each(&conditional.test) || each(&conditional.consequent) || each(&conditional.alternate)
        }
        Expression::ImportExpression(import) => each(&import.source),
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => expression
            .as_member_expression()
            .is_some_and(|member| member_resolves(member, resolver)),
        Expression::ObjectExpression(object) => {
            object.properties.iter().any(|property| match property {
                ObjectPropertyKind::ObjectProperty(property) => {
                    key_resolves(&property.key, property.computed, resolver)
                        || each(&property.value)
                }
                ObjectPropertyKind::SpreadProperty(spread) => each(&spread.argument),
            })
        }
        Expression::SequenceExpression(sequence) => sequence.expressions.iter().any(each),
        Expression::TemplateLiteral(template) => template.expressions.iter().any(each),
        Expression::TaggedTemplateExpression(tagged) => tagged.quasi.expressions.iter().any(each),
        _ => false,
    }
}

/// Whether a member expression's object or property reads a resolver call.
fn member_resolves(member: &MemberExpression, resolver: &str) -> bool {
    expression_resolves(member.object(), resolver)
        || matches!(member, MemberExpression::ComputedMemberExpression(computed) if expression_resolves(&computed.expression, resolver))
}

/// Whether a statement calls a resolver with a value or passes it on
/// (`hasNonEmptyResolverCall` on statements).
fn statement_resolves(statement: &Statement, resolver: &str) -> bool {
    let each = |statement: &Statement| statement_resolves(statement, resolver);
    match statement {
        Statement::FunctionDeclaration(function) => {
            function_resolves(FunctionNode::Function(function), resolver)
        }
        Statement::BlockStatement(block) => block.body.iter().any(each),
        Statement::ExpressionStatement(statement) => {
            expression_resolves(&statement.expression, resolver)
        }
        Statement::IfStatement(branch) => {
            expression_resolves(&branch.test, resolver)
                || each(&branch.consequent)
                || branch.alternate.as_ref().is_some_and(each)
        }
        Statement::DoWhileStatement(loop_) => each(&loop_.body),
        Statement::ForInStatement(loop_) => each(&loop_.body),
        Statement::ForOfStatement(loop_) => each(&loop_.body),
        Statement::ForStatement(loop_) => each(&loop_.body),
        Statement::LabeledStatement(labeled) => each(&labeled.body),
        Statement::WhileStatement(loop_) => each(&loop_.body),
        Statement::WithStatement(with) => each(&with.body),
        Statement::ReturnStatement(statement) => statement
            .argument
            .as_ref()
            .is_some_and(|argument| expression_resolves(argument, resolver)),
        Statement::SwitchStatement(switch) => switch
            .cases
            .iter()
            .any(|case| case.consequent.iter().any(each)),
        Statement::TryStatement(attempt) => {
            attempt.block.body.iter().any(each)
                || attempt
                    .handler
                    .as_ref()
                    .is_some_and(|handler| handler.body.body.iter().any(each))
                || attempt
                    .finalizer
                    .as_ref()
                    .is_some_and(|finalizer| finalizer.body.iter().any(each))
        }
        Statement::VariableDeclaration(declaration) => {
            declaration.declarations.iter().any(|declarator| {
                declarator
                    .init
                    .as_ref()
                    .is_some_and(|init| expression_resolves(init, resolver))
            })
        }
        _ => false,
    }
}

/// Whether an expression yields a value outside any inner function (`hasNonFunctionYield`).
fn expression_yields(expression: &Expression) -> bool {
    let each = |expression: &Expression| expression_yields(expression);
    match expression {
        Expression::ArrayExpression(array) => array.elements.iter().any(|element| match element {
            ArrayExpressionElement::SpreadElement(spread) => each(&spread.argument),
            ArrayExpressionElement::Elision(_) => false,
            other => other.as_expression().is_some_and(each),
        }),
        Expression::AssignmentExpression(assignment) => {
            assignment.left.as_member_expression().is_some_and(|member| {
                each(member.object())
                    || matches!(member, MemberExpression::ComputedMemberExpression(computed) if each(&computed.expression))
            }) || each(&assignment.right)
        }
        Expression::BinaryExpression(binary) => each(&binary.left) || each(&binary.right),
        Expression::LogicalExpression(logical) => each(&logical.left) || each(&logical.right),
        Expression::CallExpression(call) => call.arguments.iter().any(|argument| match argument {
            Argument::SpreadElement(spread) => each(&spread.argument),
            other => other.as_expression().is_some_and(each),
        }),
        Expression::ChainExpression(chain) => match &chain.expression {
            ChainElement::CallExpression(call) => call
                .arguments
                .iter()
                .any(|argument| argument.as_expression().is_some_and(each)),
            other => other.as_member_expression().is_some_and(|member| {
                each(member.object())
                    || matches!(member, MemberExpression::ComputedMemberExpression(computed) if each(&computed.expression))
            }),
        },
        Expression::ConditionalExpression(conditional) => {
            each(&conditional.test) || each(&conditional.consequent) || each(&conditional.alternate)
        }
        Expression::ImportExpression(import) => each(&import.source),
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => expression.as_member_expression().is_some_and(|member| {
            each(member.object())
                || matches!(member, MemberExpression::ComputedMemberExpression(computed) if each(&computed.expression))
        }),
        Expression::ObjectExpression(object) => object.properties.iter().any(|property| match property {
            ObjectPropertyKind::ObjectProperty(property) => {
                (property.computed && property.key.as_expression().is_some_and(each)) || each(&property.value)
            }
            ObjectPropertyKind::SpreadProperty(spread) => each(&spread.argument),
        }),
        Expression::SequenceExpression(sequence) => sequence.expressions.iter().any(each),
        Expression::TemplateLiteral(template) => template.expressions.iter().any(each),
        Expression::UnaryExpression(unary) => each(&unary.argument),
        Expression::TaggedTemplateExpression(tagged) => tagged.quasi.expressions.iter().any(each),
        Expression::YieldExpression(yield_) => yield_.argument.is_some(),
        _ => false,
    }
}

/// Whether a statement yields a value outside any inner function.
fn statement_yields(statement: &Statement) -> bool {
    match statement {
        Statement::BlockStatement(block) => block_yields(&block.body),
        Statement::ExpressionStatement(statement) => expression_yields(&statement.expression),
        Statement::IfStatement(branch) => {
            expression_yields(&branch.test)
                || statement_yields(&branch.consequent)
                || branch.alternate.as_ref().is_some_and(statement_yields)
        }
        Statement::DoWhileStatement(loop_) => statement_yields(&loop_.body),
        Statement::ForInStatement(loop_) => statement_yields(&loop_.body),
        Statement::ForOfStatement(loop_) => statement_yields(&loop_.body),
        Statement::ForStatement(loop_) => statement_yields(&loop_.body),
        Statement::LabeledStatement(labeled) => statement_yields(&labeled.body),
        Statement::WhileStatement(loop_) => statement_yields(&loop_.body),
        Statement::WithStatement(with) => statement_yields(&with.body),
        Statement::ReturnStatement(statement) => {
            statement.argument.as_ref().is_some_and(expression_yields)
        }
        Statement::SwitchStatement(switch) => switch
            .cases
            .iter()
            .any(|case| case.consequent.iter().any(statement_yields)),
        Statement::TryStatement(attempt) => {
            block_yields(&attempt.block.body)
                || attempt
                    .handler
                    .as_ref()
                    .is_some_and(|handler| block_yields(&handler.body.body))
                || attempt
                    .finalizer
                    .as_ref()
                    .is_some_and(|finalizer| block_yields(&finalizer.body))
        }
        Statement::VariableDeclaration(declaration) => declaration
            .declarations
            .iter()
            .any(|declarator| declarator.init.as_ref().is_some_and(expression_yields)),
        _ => false,
    }
}

/// Whether a block's statements yield, inner functions left out.
fn block_yields(statements: &[Statement]) -> bool {
    statements.iter().any(statement_yields)
}

/// Whether a generator yields a value (`hasYieldValue`).
#[must_use]
pub fn yields(function: FunctionNode) -> bool {
    function.is_generator() && function.statements().is_some_and(block_yields)
}

/// The function a node of the default contexts is.
#[must_use]
pub fn function_node<'s, 'a>(kind: &AstKind<'a>) -> Option<FunctionNode<'s, 'a>>
where
    'a: 's,
{
    match *kind {
        AstKind::Function(function) => Some(FunctionNode::Function(function)),
        AstKind::ArrowFunctionExpression(arrow) => Some(FunctionNode::Arrow(arrow)),
        _ => None,
    }
}

/// The parts of a type at its top level, split at the `|` outside brackets.
fn union_parts(written: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth: usize = 0;
    let mut start = 0;
    for (at, character) in written.char_indices() {
        match character {
            '<' | '(' | '{' | '[' => depth += 1,
            '>' | ')' | '}' | ']' => depth = depth.saturating_sub(1),
            '|' if depth == 0 => {
                parts.push(&written[start..at]);
                start = at + 1;
            }
            _ => {}
        }
    }
    parts.push(&written[start..]);
    parts
}

/// Whether a tag's type may be `undefined`: none written, `undefined`, `void`, `*`, `any`, or a
/// union holding `undefined` or `void` (`mayBeUndefinedTypeTag`, an empty type read as present).
#[must_use]
pub fn may_be_undefined(written: &str) -> bool {
    let kept = written.trim();
    if matches!(kept, "undefined" | "void" | "*" | "any") {
        return true;
    }
    let parts = union_parts(kept);
    parts.len() > 1
        && parts
            .iter()
            .any(|part| matches!(part.trim(), "undefined" | "void"))
}

/// Whether a return type may be `undefined` at its top level, through a parenthesis, a union or
/// a `@typedef` of the file whose type may be (`mayReturnUndefined` of `requireReturnsCheck.js`).
#[must_use]
pub fn return_may_be_undefined(written: &str, typedefs: &[(String, String)]) -> bool {
    if may_be_undefined(written) {
        return true;
    }
    let kept = written.trim();
    if kept.is_empty() {
        return false;
    }
    let inner = kept
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
        .unwrap_or(kept);
    union_parts(inner).iter().any(|part| {
        let part = part.trim();
        part == "undefined"
            || part == "void"
            || typedefs
                .iter()
                .any(|(name, written)| name == part && may_be_undefined(written))
    })
}
