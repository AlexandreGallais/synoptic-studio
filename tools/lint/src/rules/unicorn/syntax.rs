//! What Unicorn's rules share, read from the syntax alone: method calls, TypeScript's wrappers
//! peeled off, global names, static numbers, the binding an identifier reads, and the kind of
//! collection a receiver is known to be (`eslint-plugin-unicorn`'s `utils/`, without type
//! information).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrowFunctionExpression, BindingPattern, CallExpression, Expression, FormalParameter,
    FormalParameters, Function, IdentifierReference, Statement, TSType, TSTypeName,
    TSTypeOperatorOperator, UnaryOperator, VariableDeclarationKind, VariableDeclarator,
};
use oxc_semantic::{AstNode, NodeId, SymbolId};
use oxc_span::{GetSpan, Span};

use crate::source::Source;

/// A call of a method named statically, `object.name(…)`.
#[derive(Clone, Copy)]
pub struct MethodCall<'b, 'a> {
    /// The call.
    pub call: &'b CallExpression<'a>,
    /// The receiver.
    pub object: &'b Expression<'a>,
    /// The method's name.
    pub name: &'a str,
    /// Where the name is written.
    pub property: Span,
    /// Whether the member is optional, `object?.name(…)`.
    pub optional_member: bool,
}

impl<'b, 'a> MethodCall<'b, 'a> {
    /// The arguments, when none is spread.
    #[must_use]
    pub fn arguments(&self) -> Option<Vec<&'b Expression<'a>>> {
        self.call
            .arguments
            .iter()
            .map(Argument::as_expression)
            .collect()
    }

    /// Whether it is a plain call, neither its member nor its call optional.
    #[must_use]
    pub fn is_plain(&self) -> bool {
        !self.optional_member && !self.call.optional
    }
}

/// The method call a call is, when its callee is a static member.
#[must_use]
pub fn method_call<'b, 'a>(call: &'b CallExpression<'a>) -> Option<MethodCall<'b, 'a>> {
    match &call.callee {
        Expression::StaticMemberExpression(member) => Some(MethodCall {
            call,
            object: &member.object,
            name: member.property.name.as_str(),
            property: member.property.span,
            optional_member: member.optional,
        }),
        _ => None,
    }
}

/// The method call an expression is, with a name among some.
#[must_use]
pub fn method_named<'b, 'a>(
    expression: &'b Expression<'a>,
    names: &[&str],
) -> Option<MethodCall<'b, 'a>> {
    match expression {
        Expression::CallExpression(call) => {
            method_call(call).filter(|method| names.contains(&method.name))
        }
        _ => None,
    }
}

/// An expression without the TypeScript wrappers that do nothing at run time: `as`,
/// `satisfies`, `!`, `<T>` and `f<T>`.
#[must_use]
pub fn unwrap<'b, 'a>(expression: &'b Expression<'a>) -> &'b Expression<'a> {
    match expression {
        Expression::TSAsExpression(wrapper) => unwrap(&wrapper.expression),
        Expression::TSSatisfiesExpression(wrapper) => unwrap(&wrapper.expression),
        Expression::TSNonNullExpression(wrapper) => unwrap(&wrapper.expression),
        Expression::TSTypeAssertion(wrapper) => unwrap(&wrapper.expression),
        Expression::TSInstantiationExpression(wrapper) => unwrap(&wrapper.expression),
        _ => expression,
    }
}

/// The text of a range of the file.
#[must_use]
pub fn text<'a>(source: &Source<'_, 'a>, span: Span) -> &'a str {
    source
        .text
        .get(span.start as usize..span.end as usize)
        .unwrap_or_default()
}

/// Whether a range of the file holds a comment.
#[must_use]
pub fn has_comment(source: &Source, span: Span) -> bool {
    source
        .program
        .comments
        .iter()
        .any(|comment| comment.span.start >= span.start && comment.span.end <= span.end)
}

/// The kind of the parent of a node.
#[must_use]
pub fn parent<'a>(source: &Source<'_, 'a>, id: NodeId) -> AstKind<'a> {
    source.semantic.nodes().parent_kind(id)
}

/// The kind of the first ancestor of a node that is not a TypeScript wrapper nor an optional
/// chain, with the id of the child it holds.
#[must_use]
pub fn outer_parent<'a>(source: &Source<'_, 'a>, id: NodeId) -> (AstKind<'a>, NodeId) {
    let nodes = source.semantic.nodes();
    let mut child = id;
    loop {
        let parent = nodes.parent_node(child);
        match parent.kind() {
            AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_)
            | AstKind::ChainExpression(_) => child = parent.id(),
            kind => return (kind, child),
        }
    }
}

/// Whether an expression is an identifier of a name that no binding of the file declares.
#[must_use]
pub fn is_global(source: &Source, expression: &Expression, name: &str) -> bool {
    match expression {
        Expression::Identifier(identifier) => {
            identifier.name == name && symbol_of(source, identifier).is_none()
        }
        _ => false,
    }
}

/// The symbol an identifier reads, when the file declares it.
#[must_use]
pub fn symbol_of(source: &Source, identifier: &IdentifierReference) -> Option<SymbolId> {
    let reference = identifier.reference_id.get()?;
    source
        .semantic
        .scoping()
        .get_reference(reference)
        .symbol_id()
}

/// The number an expression is written as: a numeric literal, signed or not, through
/// TypeScript's wrappers.
#[must_use]
pub fn static_number(expression: &Expression) -> Option<f64> {
    match unwrap(expression) {
        Expression::NumericLiteral(literal) => Some(literal.value),
        Expression::UnaryExpression(unary) => {
            let value = static_number(&unary.argument)?;
            match unary.operator {
                UnaryOperator::UnaryNegation => Some(-value),
                UnaryOperator::UnaryPlus => Some(value),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Where an identifier's binding is declared.
pub enum Binding<'a> {
    /// A variable, with its declaration's kind.
    Variable(&'a VariableDeclarator<'a>, VariableDeclarationKind),
    /// A parameter.
    Parameter(&'a FormalParameter<'a>),
}

/// The declaration of the binding an identifier reads, when it is a variable or a parameter
/// named directly (not destructured).
#[must_use]
pub fn binding_of<'a>(source: &Source<'_, 'a>, expression: &Expression) -> Option<Binding<'a>> {
    let Expression::Identifier(identifier) = expression else {
        return None;
    };
    let symbol = symbol_of(source, identifier)?;
    let scoping = source.semantic.scoping();
    if scoping.symbol_redeclarations(symbol).len() > 1 {
        return None;
    }
    let declaration = scoping.symbol_declaration(symbol);
    match source.semantic.nodes().kind(declaration) {
        AstKind::VariableDeclarator(declarator)
            if matches!(declarator.id, BindingPattern::BindingIdentifier(_)) =>
        {
            match source.semantic.nodes().parent_kind(declaration) {
                AstKind::VariableDeclaration(statement) => {
                    Some(Binding::Variable(declarator, statement.kind))
                }
                _ => None,
            }
        }
        AstKind::FormalParameter(parameter)
            if matches!(parameter.pattern, BindingPattern::BindingIdentifier(_)) =>
        {
            Some(Binding::Parameter(parameter))
        }
        _ => None,
    }
}

/// The value a `const` binding is initialized with.
#[must_use]
pub fn const_init<'a>(
    source: &Source<'_, 'a>,
    expression: &Expression,
) -> Option<&'a Expression<'a>> {
    match binding_of(source, expression)? {
        Binding::Variable(declarator, VariableDeclarationKind::Const) => declarator.init.as_ref(),
        _ => None,
    }
}

/// What a value is known to be, from its syntax, its binding or its annotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// An `Array` or a tuple.
    Array,
    /// A typed array: indexed, but not an `Array`.
    TypedArray,
    /// A `Map` or a `ReadonlyMap`.
    Map,
    /// A `Set` or a `ReadonlySet`.
    Set,
    /// A `WeakMap`.
    WeakMap,
    /// A `WeakSet`.
    WeakSet,
    /// Something else known: an object, a function, a string, an instance of another class.
    Other,
}

/// The typed arrays' names.
const TYPED_ARRAYS: [&str; 11] = [
    "Int8Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "Int16Array",
    "Uint16Array",
    "Int32Array",
    "Uint32Array",
    "Float32Array",
    "Float64Array",
    "BigInt64Array",
    "BigUint64Array",
];

/// The kind a type annotation names.
#[must_use]
pub fn kind_of_type(annotation: &TSType) -> Option<Kind> {
    match annotation {
        TSType::TSArrayType(_) | TSType::TSTupleType(_) => Some(Kind::Array),
        TSType::TSTypeOperatorType(operator)
            if operator.operator == TSTypeOperatorOperator::Readonly =>
        {
            kind_of_type(&operator.type_annotation)
        }
        TSType::TSParenthesizedType(inner) => kind_of_type(&inner.type_annotation),
        TSType::TSTypeReference(reference) => match &reference.type_name {
            TSTypeName::IdentifierReference(name) => match name.name.as_str() {
                "Array" | "ReadonlyArray" => Some(Kind::Array),
                "Map" | "ReadonlyMap" => Some(Kind::Map),
                "Set" | "ReadonlySet" => Some(Kind::Set),
                "WeakMap" => Some(Kind::WeakMap),
                "WeakSet" => Some(Kind::WeakSet),
                "CanvasRenderingContext2D" | "OffscreenCanvasRenderingContext2D" => {
                    Some(Kind::Other)
                }
                other if TYPED_ARRAYS.contains(&other) => Some(Kind::TypedArray),
                _ => None,
            },
            _ => None,
        },
        TSType::TSUnionType(union) => {
            let mut kinds = union.types.iter().map(kind_of_type);
            let first = kinds.next()??;
            kinds.all(|kind| kind == Some(first)).then_some(first)
        }
        TSType::TSStringKeyword(_)
        | TSType::TSNumberKeyword(_)
        | TSType::TSBooleanKeyword(_)
        | TSType::TSBigIntKeyword(_)
        | TSType::TSSymbolKeyword(_)
        | TSType::TSTypeLiteral(_)
        | TSType::TSFunctionType(_)
        | TSType::TSLiteralType(_)
        | TSType::TSTemplateLiteralType(_) => Some(Kind::Other),
        _ => None,
    }
}

/// The kind of a value, when its syntax, its `const` binding or its annotation tells it.
#[must_use]
pub fn kind_of(source: &Source, expression: &Expression) -> Option<Kind> {
    kind_at_depth(source, expression, 0)
}

/// `kind_of`, a binding followed at most a few times.
fn kind_at_depth(source: &Source, expression: &Expression, depth: u8) -> Option<Kind> {
    match expression {
        Expression::TSAsExpression(wrapper) => kind_of_type(&wrapper.type_annotation)
            .or_else(|| kind_at_depth(source, &wrapper.expression, depth)),
        Expression::TSSatisfiesExpression(wrapper) => kind_of_type(&wrapper.type_annotation)
            .or_else(|| kind_at_depth(source, &wrapper.expression, depth)),
        Expression::TSTypeAssertion(wrapper) => kind_of_type(&wrapper.type_annotation)
            .or_else(|| kind_at_depth(source, &wrapper.expression, depth)),
        Expression::TSNonNullExpression(wrapper) => {
            kind_at_depth(source, &wrapper.expression, depth)
        }
        Expression::ArrayExpression(_) => Some(Kind::Array),
        Expression::ConditionalExpression(conditional) => {
            let consequent = kind_at_depth(source, &conditional.consequent, depth)?;
            (kind_at_depth(source, &conditional.alternate, depth)? == consequent)
                .then_some(consequent)
        }
        Expression::ObjectExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::ClassExpression(_)
        | Expression::TemplateLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_) => Some(Kind::Other),
        Expression::NewExpression(new) => Some(match &new.callee {
            Expression::Identifier(callee) if symbol_of(source, callee).is_none() => {
                match callee.name.as_str() {
                    "Array" => Kind::Array,
                    "Map" => Kind::Map,
                    "Set" => Kind::Set,
                    "WeakMap" => Kind::WeakMap,
                    "WeakSet" => Kind::WeakSet,
                    name if TYPED_ARRAYS.contains(&name) => Kind::TypedArray,
                    _ => Kind::Other,
                }
            }
            _ => Kind::Other,
        }),
        Expression::CallExpression(call) => {
            if is_global(source, &call.callee, "Array") {
                return Some(Kind::Array);
            }
            let method = method_call(call)?;
            let array_maker = is_global(source, method.object, "Array")
                && matches!(method.name, "from" | "of")
                && method.is_plain();
            array_maker.then_some(Kind::Array)
        }
        Expression::Identifier(_) if depth < 4 => match binding_of(source, expression)? {
            Binding::Variable(declarator, kind) => {
                if let Some(annotation) = &declarator.type_annotation {
                    return kind_of_type(&annotation.type_annotation);
                }
                if kind != VariableDeclarationKind::Const {
                    return None;
                }
                kind_at_depth(source, declarator.init.as_ref()?, depth + 1)
            }
            Binding::Parameter(parameter) => parameter
                .type_annotation
                .as_ref()
                .and_then(|annotation| kind_of_type(&annotation.type_annotation)),
        },
        _ => None,
    }
}

/// Whether a receiver is known not to be an indexed collection (an `Array` or a typed array),
/// as Unicorn's `isKnownNonIndexedCollection`.
#[must_use]
pub fn is_known_non_indexed(source: &Source, expression: &Expression) -> bool {
    matches!(
        kind_of(source, expression),
        Some(Kind::Map | Kind::Set | Kind::WeakMap | Kind::WeakSet | Kind::Other)
    )
}

/// Whether an array rule skips a receiver known not to be an array, unless the receiver is
/// written as a literal, whose mismatch shows where it is called (`shouldSkipKnownNonArrayReceiver`).
#[must_use]
pub fn skips_non_array_receiver(source: &Source, expression: &Expression) -> bool {
    !matches!(
        expression,
        Expression::ArrayExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::StringLiteral(_)
            | Expression::NumericLiteral(_)
            | Expression::BooleanLiteral(_)
            | Expression::NullLiteral(_)
            | Expression::RegExpLiteral(_)
            | Expression::BigIntLiteral(_)
            | Expression::ObjectExpression(_)
            | Expression::TemplateLiteral(_)
    ) && is_known_non_indexed(source, expression)
}

/// A function written inline: its parameters, whether it is `async` or a generator or carries
/// a return type or type parameters, and its body's statements or expression.
pub struct Callback<'b, 'a> {
    /// Its parameters.
    pub params: &'b FormalParameters<'a>,
    /// Whether it is `async`, a generator, or typed by a return type or type parameters.
    pub special: bool,
    /// Its concise body.
    pub expression: Option<&'b Expression<'a>>,
    /// Its block's statements.
    pub statements: &'b [Statement<'a>],
}

/// The inline function an expression is: an arrow or a function expression.
#[must_use]
pub fn callback<'b, 'a>(expression: &'b Expression<'a>) -> Option<Callback<'b, 'a>> {
    match expression {
        Expression::ArrowFunctionExpression(arrow) => Some(arrow_callback(arrow)),
        Expression::FunctionExpression(function) => function_callback(function),
        _ => None,
    }
}

/// An arrow's parts.
fn arrow_callback<'b, 'a>(arrow: &'b ArrowFunctionExpression<'a>) -> Callback<'b, 'a> {
    Callback {
        params: &arrow.params,
        special: arrow.r#async || arrow.return_type.is_some() || arrow.type_parameters.is_some(),
        expression: arrow.get_expression(),
        statements: arrow
            .get_function_body()
            .map_or(&[], |body| body.statements.as_slice()),
    }
}

/// A function expression's parts.
fn function_callback<'b, 'a>(function: &'b Function<'a>) -> Option<Callback<'b, 'a>> {
    let body = function.body.as_ref()?;
    Some(Callback {
        params: &function.params,
        special: function.r#async
            || function.generator
            || function.return_type.is_some()
            || function.type_parameters.is_some(),
        expression: None,
        statements: body.statements.as_slice(),
    })
}

impl<'b, 'a> Callback<'b, 'a> {
    /// The expression it returns: its concise body, or the argument of its only statement when
    /// that is a `return` (`getFunctionReturnExpression`).
    #[must_use]
    pub fn returned(&self) -> Option<&'b Expression<'a>> {
        if self.expression.is_some() {
            return self.expression;
        }
        match self.statements {
            [Statement::ReturnStatement(statement)] => statement.argument.as_ref(),
            _ => None,
        }
    }

    /// The names of its parameters, when each is a plain identifier and none is rest.
    #[must_use]
    pub fn parameter_names(&self) -> Option<Vec<&'a str>> {
        if self.params.rest.is_some() {
            return None;
        }
        self.params
            .items
            .iter()
            .map(|parameter| match &parameter.pattern {
                BindingPattern::BindingIdentifier(identifier)
                    if parameter.initializer.is_none() =>
                {
                    Some(identifier.name.as_str())
                }
                _ => None,
            })
            .collect()
    }
}

/// Whether two expressions read the same reference: the same names along the same static
/// members (`isSameReference`).
#[must_use]
pub fn same_reference(left: &Expression, right: &Expression) -> bool {
    let (left, right) = (unwrap(left), unwrap(right));
    match (left, right) {
        (Expression::Identifier(a), Expression::Identifier(b)) => a.name == b.name,
        (Expression::ThisExpression(_), Expression::ThisExpression(_))
        | (Expression::Super(_), Expression::Super(_))
        | (Expression::NullLiteral(_), Expression::NullLiteral(_)) => true,
        (Expression::StringLiteral(a), Expression::StringLiteral(b)) => a.value == b.value,
        (Expression::NumericLiteral(a), Expression::NumericLiteral(b)) => {
            a.value.total_cmp(&b.value).is_eq()
        }
        (Expression::BooleanLiteral(a), Expression::BooleanLiteral(b)) => a.value == b.value,
        _ => match (static_member(left), static_member(right)) {
            (Some((a_object, a_name)), Some((b_object, b_name))) => {
                a_name == b_name && same_reference(a_object, b_object)
            }
            _ => match (left, right) {
                (
                    Expression::ComputedMemberExpression(a),
                    Expression::ComputedMemberExpression(b),
                ) => {
                    same_reference(&a.object, &b.object)
                        && same_reference(&a.expression, &b.expression)
                }
                _ => false,
            },
        },
    }
}

/// The object and the name of a member read statically: `a.b`, `a["b"]`, ``a[`b`]``, `a[0]`.
#[must_use]
pub fn static_member<'b, 'a>(
    expression: &'b Expression<'a>,
) -> Option<(&'b Expression<'a>, String)> {
    match expression {
        Expression::StaticMemberExpression(member) => {
            Some((&member.object, member.property.name.to_string()))
        }
        Expression::ComputedMemberExpression(member) => match &member.expression {
            Expression::StringLiteral(literal) => Some((&member.object, literal.value.to_string())),
            Expression::NumericLiteral(literal) => {
                Some((&member.object, literal.value.to_string()))
            }
            Expression::TemplateLiteral(template) if template.expressions.is_empty() => Some((
                &member.object,
                template.quasis.first()?.value.cooked?.to_string(),
            )),
            _ => None,
        },
        _ => None,
    }
}

/// Whether an expression needs parentheses to stand as the operand of a unary operator.
#[must_use]
pub fn needs_parentheses_as_operand(expression: &Expression) -> bool {
    matches!(
        expression,
        Expression::BinaryExpression(_)
            | Expression::LogicalExpression(_)
            | Expression::ConditionalExpression(_)
            | Expression::AssignmentExpression(_)
            | Expression::SequenceExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::YieldExpression(_)
            | Expression::TSAsExpression(_)
            | Expression::TSSatisfiesExpression(_)
            | Expression::UpdateExpression(_)
            | Expression::TSNonNullExpression(_)
            | Expression::TSTypeAssertion(_)
    )
}

/// The keywords after which `(` opens a group rather than a call's or a statement's list.
const GROUPING_KEYWORDS: [&str; 12] = [
    "return", "typeof", "void", "delete", "await", "yield", "in", "of", "case", "throw", "new",
    "else",
];

/// Whether the text around a range holds parentheses that group it: `(` before, `)` after, the
/// `(` not opening a call's arguments nor the head of `if`, `while`, `for`, `switch` or `catch`.
#[must_use]
pub fn is_parenthesized(source: &Source, span: Span) -> bool {
    let all = source.text;
    let open = all
        .get(..span.start as usize)
        .unwrap_or_default()
        .trim_end();
    let rest = all.get(span.end as usize..).unwrap_or_default();
    if !open.ends_with('(') || !rest.trim_start().starts_with(')') {
        return false;
    }
    let open_at = open.len() - 1;
    let close_at = span.end as usize + (rest.len() - rest.trim_start().len());
    let outside = all[..open_at].trim_end();
    let word_start = outside
        .rfind(|character: char| {
            !(character.is_alphanumeric() || character == '_' || character == '$')
        })
        .map_or(0, |at| at + 1);
    let syntactic = match outside.chars().last() {
        Some(')' | ']' | '>') => true,
        Some(character) if character.is_alphanumeric() || character == '_' || character == '$' => {
            !GROUPING_KEYWORDS.contains(&&outside[word_start..])
        }
        _ => false,
    };
    !syntactic
        || is_parenthesized(
            source,
            Span::new(
                u32::try_from(open_at).unwrap_or(span.start),
                u32::try_from(close_at + 1).unwrap_or(span.end),
            ),
        )
}

/// The text of an expression with the parentheses written around it, when it has some.
#[must_use]
pub fn parenthesized_text(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    if is_parenthesized(source, expression.span()) {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

/// The text of an operand of a logical operator, parenthesized when the operator would bind it
/// otherwise; written parentheses are kept when asked (`getLogicalExpressionChildText`): a
/// call's own parentheses around its only argument are not one's.
#[must_use]
pub fn logical_operand_text(
    source: &Source,
    expression: &Expression,
    operator: oxc_ast::ast::LogicalOperator,
    keep_written: bool,
) -> String {
    if keep_written && is_parenthesized(source, expression.span()) {
        return parenthesized_text(source, expression);
    }
    let needs = match expression {
        Expression::LogicalExpression(logical) => logical.operator != operator,
        Expression::AwaitExpression(_)
        | Expression::BinaryExpression(_)
        | Expression::ConditionalExpression(_)
        | Expression::AssignmentExpression(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::YieldExpression(_)
        | Expression::SequenceExpression(_) => true,
        _ => false,
    };
    let inner = text(source, expression.span());
    if needs {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

/// A value an expression always has, read without running it.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// A string.
    Text(String),
    /// A number.
    Number(f64),
    /// A boolean.
    Boolean(bool),
    /// `null`.
    Null,
    /// `undefined`.
    Undefined,
    /// A `BigInt`, by its digits.
    BigInt(String),
}

impl Value {
    /// Whether two values are equal as `SameValueZero` compares them.
    #[must_use]
    pub fn same_value_zero(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => {
                (a.is_nan() && b.is_nan()) || (a - b).abs() == 0.0
            }
            _ => self == other,
        }
    }
}

/// The value an expression always has: a literal, a template without expressions, a signed
/// number, `undefined`, a `const` bound to one, a sum or a concatenation of them.
#[must_use]
pub fn static_value(source: &Source, expression: &Expression) -> Option<Value> {
    value_at_depth(source, expression, 0)
}

/// `static_value`, a binding followed at most a few times.
fn value_at_depth(source: &Source, expression: &Expression, depth: u8) -> Option<Value> {
    match unwrap(expression) {
        Expression::StringLiteral(literal) => Some(Value::Text(literal.value.to_string())),
        Expression::NumericLiteral(literal) => Some(Value::Number(literal.value)),
        Expression::BooleanLiteral(literal) => Some(Value::Boolean(literal.value)),
        Expression::NullLiteral(_) => Some(Value::Null),
        Expression::BigIntLiteral(literal) => Some(Value::BigInt(literal.value.to_string())),
        Expression::TemplateLiteral(template) if template.expressions.is_empty() => Some(
            Value::Text(template.quasis.first()?.value.cooked?.to_string()),
        ),
        Expression::Identifier(identifier)
            if matches!(identifier.name.as_str(), "undefined" | "NaN" | "Infinity") =>
        {
            symbol_of(source, identifier)
                .is_none()
                .then(|| match identifier.name.as_str() {
                    "NaN" => Value::Number(f64::NAN),
                    "Infinity" => Value::Number(f64::INFINITY),
                    _ => Value::Undefined,
                })
        }
        Expression::Identifier(_) if depth < 4 => {
            value_at_depth(source, const_init(source, unwrap(expression))?, depth + 1)
        }
        Expression::UnaryExpression(unary) => {
            let value = value_at_depth(source, &unary.argument, depth)?;
            match (unary.operator, value) {
                (UnaryOperator::UnaryNegation, Value::Number(number)) => {
                    Some(Value::Number(-number))
                }
                (UnaryOperator::UnaryPlus, Value::Number(number)) => Some(Value::Number(number)),
                (UnaryOperator::LogicalNot, Value::Boolean(boolean)) => {
                    Some(Value::Boolean(!boolean))
                }
                (UnaryOperator::Void, _) => Some(Value::Undefined),
                _ => None,
            }
        }
        Expression::BinaryExpression(binary) => {
            let left = value_at_depth(source, &binary.left, depth)?;
            let right = value_at_depth(source, &binary.right, depth)?;
            match (binary.operator, left, right) {
                (oxc_ast::ast::BinaryOperator::Addition, Value::Number(a), Value::Number(b)) => {
                    Some(Value::Number(a + b))
                }
                (oxc_ast::ast::BinaryOperator::Addition, Value::Text(a), Value::Text(b)) => {
                    Some(Value::Text(a + &b))
                }
                (oxc_ast::ast::BinaryOperator::Subtraction, Value::Number(a), Value::Number(b)) => {
                    Some(Value::Number(a - b))
                }
                (
                    oxc_ast::ast::BinaryOperator::Multiplication,
                    Value::Number(a),
                    Value::Number(b),
                ) => Some(Value::Number(a * b)),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The text of an expression negated with `!`, parenthesized when needed
/// (`getNegatedExpressionText`).
#[must_use]
pub fn negated_text(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    if needs_parentheses_as_operand(expression) {
        format!("!({inner})")
    } else {
        format!("!{inner}")
    }
}

/// Whether a node is the whole of a statement that discards its value.
#[must_use]
pub fn is_discarded(source: &Source, node: &AstNode) -> bool {
    matches!(
        outer_parent(source, node.id()).0,
        AstKind::ExpressionStatement(_)
    )
}

/// Whether an expression is a path of static members from a name or `this`: `a.b.c`.
#[must_use]
pub fn is_path(expression: &Expression, path: &str) -> bool {
    let (rest, last) = match path.rsplit_once('.') {
        Some((rest, last)) => (Some(rest), last),
        None => (None, path),
    };
    match (expression, rest) {
        (Expression::StaticMemberExpression(member), Some(rest)) => {
            member.property.name == last && is_path(&member.object, rest)
        }
        (Expression::Identifier(identifier), None) => identifier.name == last,
        (Expression::ThisExpression(_), None) => last == "this",
        _ => false,
    }
}

/// Whether an expression is `object.length`, `.length` read without `?.`.
#[must_use]
pub fn is_length_of(expression: &Expression, object: &Expression) -> bool {
    matches!(unwrap(expression), Expression::StaticMemberExpression(member)
        if !member.optional && member.property.name == "length" && same_reference(&member.object, object))
}

/// Whether an expression is `object.length - 1`.
#[must_use]
pub fn is_length_minus_one_of(expression: &Expression, object: &Expression) -> bool {
    matches!(unwrap(expression), Expression::BinaryExpression(binary)
        if binary.operator == oxc_ast::ast::BinaryOperator::Subtraction
            && static_number(&binary.right) == Some(1.0)
            && is_length_of(&binary.left, object))
}

/// Whether the value of a node is thrown away: it is a statement, through `?.`, or an element of
/// a sequence whose value is not the one kept (`isValueNotUsable`).
#[must_use]
pub fn is_value_unused(source: &Source, id: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = id;
    let mut parent = nodes.parent_node(child);
    if matches!(parent.kind(), AstKind::ChainExpression(_)) {
        child = parent.id();
        parent = nodes.parent_node(child);
    }
    match parent.kind() {
        AstKind::ExpressionStatement(_) => true,
        AstKind::SequenceExpression(sequence) => {
            sequence.expressions.last().map(Expression::node_id) != Some(child)
                || is_value_unused(source, parent.id())
        }
        _ => false,
    }
}

/// Whether evaluating an expression may change something: a call, an assignment, an update,
/// `new`, `await`, `yield`, `delete`, or anything this check does not read.
#[must_use]
pub fn has_side_effect(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(_)
        | Expression::ThisExpression(_)
        | Expression::Super(_)
        | Expression::NumericLiteral(_)
        | Expression::StringLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::FunctionExpression(_)
        | Expression::ArrowFunctionExpression(_) => false,
        Expression::StaticMemberExpression(member) => has_side_effect(&member.object),
        Expression::ComputedMemberExpression(member) => {
            has_side_effect(&member.object) || has_side_effect(&member.expression)
        }
        Expression::TemplateLiteral(template) => template.expressions.iter().any(has_side_effect),
        Expression::UnaryExpression(unary) => {
            unary.operator == UnaryOperator::Delete || has_side_effect(&unary.argument)
        }
        Expression::BinaryExpression(binary) => {
            has_side_effect(&binary.left) || has_side_effect(&binary.right)
        }
        Expression::LogicalExpression(logical) => {
            has_side_effect(&logical.left) || has_side_effect(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            has_side_effect(&conditional.test)
                || has_side_effect(&conditional.consequent)
                || has_side_effect(&conditional.alternate)
        }
        Expression::ArrayExpression(array) => array
            .elements
            .iter()
            .any(|element| element.as_expression().is_none_or(has_side_effect)),
        Expression::TSAsExpression(_)
        | Expression::TSSatisfiesExpression(_)
        | Expression::TSNonNullExpression(_)
        | Expression::TSTypeAssertion(_)
        | Expression::TSInstantiationExpression(_) => has_side_effect(unwrap(expression)),
        _ => true,
    }
}

/// The range a removed statement takes: its whole line when it stands alone on it.
#[must_use]
pub fn statement_removal(source: &Source, span: Span) -> Span {
    let before = source.text.get(..span.start as usize).unwrap_or_default();
    let line_start = before.rfind('\n').map_or(0, |at| at + 1);
    let after = source.text.get(span.end as usize..).unwrap_or_default();
    let line_end = after.find('\n').map_or(after.len(), |at| at + 1);
    let alone = before[line_start..].trim().is_empty() && after[..line_end].trim().is_empty();
    if alone {
        Span::new(
            u32::try_from(line_start).unwrap_or(span.start),
            span.end + u32::try_from(line_end).unwrap_or(0),
        )
    } else {
        span
    }
}

/// The text of an expression as the object of a member, parenthesized unless it binds tighter
/// than a member access.
#[must_use]
pub fn object_text(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    let tight = matches!(
        expression,
        Expression::Identifier(_)
            | Expression::ThisExpression(_)
            | Expression::Super(_)
            | Expression::StaticMemberExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::PrivateFieldExpression(_)
            | Expression::CallExpression(_)
            | Expression::ArrayExpression(_)
            | Expression::StringLiteral(_)
            | Expression::TemplateLiteral(_)
            | Expression::TSNonNullExpression(_)
    ) || (matches!(expression, Expression::NewExpression(_)) && inner.ends_with(')'));
    if tight {
        inner.to_owned()
    } else {
        format!("({inner})")
    }
}

/// Whether an expression reads through `?.` anywhere along its objects and callees.
#[must_use]
pub fn has_optional_chain(expression: &Expression) -> bool {
    match unwrap(expression) {
        Expression::ChainExpression(_) => true,
        Expression::StaticMemberExpression(member) => {
            member.optional || has_optional_chain(&member.object)
        }
        Expression::ComputedMemberExpression(member) => {
            member.optional
                || has_optional_chain(&member.object)
                || has_optional_chain(&member.expression)
        }
        Expression::CallExpression(call) => call.optional || has_optional_chain(&call.callee),
        _ => false,
    }
}

/// The nodes under a node, in source order: the ones its subtree holds.
pub fn descendants<'s, 'a>(
    source: &'s Source<'_, 'a>,
    id: NodeId,
) -> impl Iterator<Item = &'s AstNode<'a>> + 's {
    let nodes = source.semantic.nodes();
    (id.index() + 1..nodes.len())
        .map(move |index| nodes.get_node(NodeId::new(index)))
        .take_while(move |node| nodes.ancestor_ids(node.id()).any(|ancestor| ancestor == id))
}

/// The nodes from a node up to an ancestor, both included.
pub fn path_up<'s, 'a>(
    source: &'s Source<'_, 'a>,
    from: NodeId,
    to: NodeId,
) -> impl Iterator<Item = &'s AstNode<'a>> + 's {
    let nodes = source.semantic.nodes();
    std::iter::once(nodes.get_node(from)).chain(
        nodes
            .ancestors(from)
            .take_while(move |node| node.id() != to)
            .chain(std::iter::once(nodes.get_node(to)))
            .filter(move |_| from != to),
    )
}

/// Whether a node starts a function or a class, whose body runs elsewhere.
#[must_use]
pub fn is_function_or_class(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::Function(_) | AstKind::ArrowFunctionExpression(_) | AstKind::Class(_)
    )
}

/// The identifier a reference starts from: `a` in `a.b[c]`.
#[must_use]
pub fn root_identifier<'b, 'a>(
    expression: &'b Expression<'a>,
) -> Option<&'b IdentifierReference<'a>> {
    match unwrap(expression) {
        Expression::Identifier(identifier) => Some(identifier),
        Expression::StaticMemberExpression(member) => root_identifier(&member.object),
        Expression::ComputedMemberExpression(member) => root_identifier(&member.object),
        _ => None,
    }
}

/// Whether two identifiers read the same binding; two undeclared ones, the same name.
#[must_use]
pub fn same_binding(
    source: &Source,
    left: &IdentifierReference,
    right: &IdentifierReference,
) -> bool {
    match (symbol_of(source, left), symbol_of(source, right)) {
        (None, None) => left.name == right.name,
        (left, right) => left == right,
    }
}

/// Whether two references read the same thing: the same path from the same binding.
#[must_use]
pub fn same_reference_binding(source: &Source, left: &Expression, right: &Expression) -> bool {
    if !same_reference(left, right) {
        return false;
    }
    match (root_identifier(left), root_identifier(right)) {
        (Some(left), Some(right)) => same_binding(source, left, right),
        _ => true,
    }
}

/// Whether an expression is a reference: a name, a member, `this` or `super`.
#[must_use]
pub fn is_reference(expression: &Expression) -> bool {
    matches!(
        unwrap(expression),
        Expression::Identifier(_)
            | Expression::StaticMemberExpression(_)
            | Expression::ComputedMemberExpression(_)
            | Expression::PrivateFieldExpression(_)
            | Expression::ThisExpression(_)
            | Expression::Super(_)
    )
}

/// Whether an expression calls something, outside the functions it holds.
#[must_use]
pub fn has_call(expression: &Expression) -> bool {
    match expression {
        Expression::CallExpression(_) => true,
        Expression::StaticMemberExpression(member) => has_call(&member.object),
        Expression::ComputedMemberExpression(member) => {
            has_call(&member.object) || has_call(&member.expression)
        }
        Expression::TemplateLiteral(template) => template.expressions.iter().any(has_call),
        Expression::UnaryExpression(unary) => has_call(&unary.argument),
        Expression::AwaitExpression(unary) => has_call(&unary.argument),
        Expression::BinaryExpression(binary) => has_call(&binary.left) || has_call(&binary.right),
        Expression::LogicalExpression(logical) => {
            has_call(&logical.left) || has_call(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            has_call(&conditional.test)
                || has_call(&conditional.consequent)
                || has_call(&conditional.alternate)
        }
        Expression::NewExpression(new) => {
            has_call(&new.callee)
                || new
                    .arguments
                    .iter()
                    .any(|argument| argument.as_expression().is_some_and(has_call))
        }
        Expression::ArrayExpression(array) => array
            .elements
            .iter()
            .any(|element| element.as_expression().is_some_and(has_call)),
        Expression::TSAsExpression(_)
        | Expression::TSSatisfiesExpression(_)
        | Expression::TSNonNullExpression(_)
        | Expression::TSTypeAssertion(_)
        | Expression::TSInstantiationExpression(_) => has_call(unwrap(expression)),
        _ => false,
    }
}

/// The statement a statement position holds: itself, or the only statement of its block.
#[must_use]
pub fn single_statement<'b, 'a>(statement: &'b Statement<'a>) -> Option<&'b Statement<'a>> {
    match statement {
        Statement::BlockStatement(block) => match block.body.as_slice() {
            [only] => Some(only),
            _ => None,
        },
        other => Some(other),
    }
}

/// The text of an expression as the concise body of an arrow, parenthesized when it would start
/// with `{` or is a sequence or an assignment (`getConciseArrowBodyText`).
#[must_use]
pub fn concise_body_text(source: &Source, expression: &Expression) -> String {
    let inner = text(source, expression.span());
    let needs = inner.starts_with('{')
        || matches!(
            expression,
            Expression::SequenceExpression(_) | Expression::AssignmentExpression(_)
        );
    if needs {
        format!("({inner})")
    } else {
        inner.to_owned()
    }
}

/// Whether a node, outside the functions it holds, waits: `await`, `yield` or `for await`.
#[must_use]
pub fn has_suspension(source: &Source, id: NodeId) -> bool {
    std::iter::once(source.semantic.nodes().get_node(id))
        .chain(descendants(source, id))
        .filter(|node| {
            !path_up(source, node.id(), id)
                .any(|each| each.id() != node.id() && is_function_or_class(&each.kind()))
        })
        .any(|node| match node.kind() {
            AstKind::AwaitExpression(_) | AstKind::YieldExpression(_) => true,
            AstKind::ForOfStatement(statement) => statement.r#await,
            _ => false,
        })
}

/// Whether a binding is read or written inside a range.
#[must_use]
pub fn is_referenced_in(source: &Source, symbol: SymbolId, span: Span) -> bool {
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    scoping
        .get_resolved_reference_ids(symbol)
        .iter()
        .any(|reference| {
            let at = nodes
                .kind(scoping.get_reference(*reference).node_id())
                .span();
            at.start >= span.start && at.end <= span.end
        })
}

/// The statement after a node in its list of statements; an exported declaration counts as its
/// `export` statement.
#[must_use]
pub fn next_statement<'a>(source: &Source<'_, 'a>, id: NodeId) -> Option<&'a Statement<'a>> {
    let nodes = source.semantic.nodes();
    let mut statement = id;
    if matches!(nodes.parent_kind(statement), AstKind::ExportDeclaration(_)) {
        statement = nodes.parent_id(statement);
    }
    let list: &'a [Statement<'a>] = match nodes.parent_kind(statement) {
        AstKind::Program(program) => &program.body,
        AstKind::BlockStatement(block) => &block.body,
        AstKind::FunctionBody(body) => &body.statements,
        AstKind::SwitchCase(case) => &case.consequent,
        AstKind::StaticBlock(block) => &block.body,
        _ => return None,
    };
    let index = list.iter().position(|each| each.node_id() == statement)?;
    list.get(index + 1)
}

/// Whether no binding named so hides the global at a node.
#[must_use]
pub fn is_global_name_free(source: &Source, node: &AstNode, name: &str) -> bool {
    source
        .semantic
        .scoping()
        .find_binding(node.scope_id(), oxc_ast::ast::Ident::from(name))
        .is_none()
}

/// Whether a node's value is only read for its truth: under `!`, in `Boolean(…)`, or as the
/// test of an `if`, a loop or a ternary, through `&&` and `||` (`isBooleanContext`).
#[must_use]
pub fn is_boolean_context(source: &Source, id: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = id;
    loop {
        let parent = nodes.parent_node(child);
        let decided = match parent.kind() {
            AstKind::UnaryExpression(unary) => Some(unary.operator == UnaryOperator::LogicalNot),
            AstKind::CallExpression(call) => Some(
                is_global(source, &call.callee, "Boolean")
                    && !call.optional
                    && call.arguments.len() == 1
                    && call.arguments[0]
                        .as_expression()
                        .is_some_and(|argument| argument.node_id() == child),
            ),
            AstKind::IfStatement(statement) => Some(statement.test.node_id() == child),
            AstKind::ConditionalExpression(conditional) => {
                Some(conditional.test.node_id() == child)
            }
            AstKind::WhileStatement(statement) => Some(statement.test.node_id() == child),
            AstKind::DoWhileStatement(statement) => Some(statement.test.node_id() == child),
            AstKind::ForStatement(statement) => Some(
                statement
                    .test
                    .as_ref()
                    .is_some_and(|test| test.node_id() == child),
            ),
            AstKind::LogicalExpression(logical)
                if logical.operator != oxc_ast::ast::LogicalOperator::Coalesce =>
            {
                None
            }
            _ => Some(false),
        };
        match decided {
            Some(decided) => return decided,
            None => child = parent.id(),
        }
    }
}

/// What a type name the file declares stands for.
pub enum TypeDefinition<'a> {
    /// A type alias without parameters, and what it aliases.
    Alias(&'a TSType<'a>),
    /// An interface or a class: an object.
    Object,
    /// Anything else, a generic alias among them.
    Other,
}

/// The declaration a type name refers to from a scope, when the file declares it.
#[must_use]
pub fn type_definition<'a>(
    source: &Source<'_, 'a>,
    scope: oxc_semantic::ScopeId,
    name: &str,
) -> Option<TypeDefinition<'a>> {
    let symbol = source
        .semantic
        .scoping()
        .find_binding(scope, oxc_ast::ast::Ident::from(name))?;
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    Some(match source.semantic.nodes().kind(declaration) {
        AstKind::TSTypeAliasDeclaration(alias) if alias.type_parameters.is_none() => {
            TypeDefinition::Alias(&alias.type_annotation)
        }
        AstKind::TSInterfaceDeclaration(_) | AstKind::Class(_) => TypeDefinition::Object,
        _ => TypeDefinition::Other,
    })
}
