//! `unicorn/no-unused-builtin-method-return`: the result of an array method that computes a
//! value, of a `Set` operation or of a `Temporal` update is used, or discarded with `void`
//! (`eslint-plugin-unicorn` 78.0.0). The receiver is read from its syntax, its binding or its
//! annotation; an unknown one counts as an array unless it is plainly something else.

use oxc_ast::AstKind;
use oxc_ast::ast::{BindingPattern, CallExpression, Expression, TSType, TSTypeName};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{
    Kind, is_global, is_lazy_helper, kind_of, kind_of_type, symbol_of,
};
use crate::source::Source;

/// The rule.
pub struct NoUnusedBuiltinMethodReturn;

/// The array methods whose result is the point of the call.
const ARRAY_METHODS: [&str; 24] = [
    "at",
    "concat",
    "entries",
    "every",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "flat",
    "flatMap",
    "includes",
    "indexOf",
    "join",
    "keys",
    "lastIndexOf",
    "map",
    "some",
    "slice",
    "toReversed",
    "toSorted",
    "toSpliced",
    "values",
    "with",
];

/// The `Set` methods that only compute.
const SET_METHODS: [&str; 8] = [
    "has",
    "union",
    "intersection",
    "difference",
    "symmetricDifference",
    "isSubsetOf",
    "isSupersetOf",
    "isDisjointFrom",
];

/// The `Temporal` types, each with the methods that return a new value.
const TEMPORAL_TYPES: [(&str, &[&str]); 8] = [
    ("Instant", &["add", "subtract", "round"]),
    ("ZonedDateTime", &["add", "subtract", "with", "round"]),
    ("PlainDate", &["add", "subtract", "with"]),
    ("PlainTime", &["add", "subtract", "with", "round"]),
    ("PlainDateTime", &["add", "subtract", "with", "round"]),
    ("PlainYearMonth", &["add", "subtract", "with"]),
    ("PlainMonthDay", &["with"]),
    ("Duration", &["add", "subtract", "with", "round"]),
];

/// The global functions whose call is no array.
const NON_ARRAY_FACTORIES: [&str; 6] =
    ["BigInt", "Boolean", "Number", "RegExp", "String", "Symbol"];

/// Whether a method is one of a `Temporal` type's.
fn is_temporal_method(method: &str) -> bool {
    TEMPORAL_TYPES
        .iter()
        .any(|(_, methods)| methods.contains(&method))
}

/// Whether a `Temporal` type name has a method.
fn temporal_has(type_name: &str, method: &str) -> bool {
    TEMPORAL_TYPES
        .iter()
        .any(|(name, methods)| *name == type_name && methods.contains(&method))
}

/// What a receiver resolves to.
enum Receiver<'b, 'a> {
    /// An expression to judge.
    Expression(&'b Expression<'a>),
    /// Unknowable: skipped.
    Uncertain,
}

/// The name of the method a call calls, written statically.
fn method_name<'b, 'a>(
    call: &'b CallExpression<'a>,
) -> Option<(&'b str, Span, &'b Expression<'a>)> {
    match &call.callee {
        Expression::StaticMemberExpression(member) => Some((
            member.property.name.as_str(),
            member.property.span,
            &member.object,
        )),
        Expression::ComputedMemberExpression(member) => match &member.expression {
            Expression::StringLiteral(literal) => {
                Some((literal.value.as_str(), literal.span, &member.object))
            }
            _ => None,
        },
        _ => None,
    }
}

/// Whether a call's value is thrown away: a statement, a sequence's element whose value is not
/// kept, or a `for` head's start or update, through `?.`, `await` and TypeScript's wrappers.
fn is_discarded(source: &Source, id: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = id;
    loop {
        let parent = nodes.parent_node(child);
        let span = nodes.kind(child).span();
        match parent.kind() {
            AstKind::ExpressionStatement(_) => return true,
            AstKind::SequenceExpression(sequence) => {
                let last = sequence.expressions.last().map(GetSpan::span);
                if last != Some(span) {
                    return true;
                }
                child = parent.id();
            }
            AstKind::ForStatement(statement) => {
                let init = statement
                    .init
                    .as_ref()
                    .and_then(|init| init.as_expression())
                    .map(GetSpan::span);
                return init == Some(span)
                    || statement.update.as_ref().map(GetSpan::span) == Some(span);
            }
            AstKind::ChainExpression(_)
            | AstKind::AwaitExpression(_)
            | AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_)
            | AstKind::TSInstantiationExpression(_) => child = parent.id(),
            _ => return false,
        }
    }
}

/// The `Temporal` type an annotation names, `Temporal.Name`.
fn temporal_type_name<'a>(annotation: &TSType<'a>) -> Option<&'a str> {
    let TSType::TSTypeReference(reference) = annotation else {
        return None;
    };
    let TSTypeName::QualifiedName(name) = &reference.type_name else {
        return None;
    };
    matches!(&name.left, TSTypeName::IdentifierReference(left) if left.name == "Temporal")
        .then_some(name.right.name.as_str())
}

/// Whether an annotation fits a method's receiver: an array, a `Set`, or a `Temporal` type
/// having the method; a union, only when every member does.
fn is_supported_type(annotation: &TSType, method: &str) -> bool {
    if let TSType::TSUnionType(union) = annotation {
        return union
            .types
            .iter()
            .all(|each| is_supported_type(each, method));
    }
    if SET_METHODS.contains(&method) {
        return kind_of_type(annotation) == Some(Kind::Set);
    }
    if method == "with" && kind_of_type(annotation) == Some(Kind::Array) {
        return true;
    }
    if is_temporal_method(method) {
        return temporal_type_name(annotation).is_some_and(|name| temporal_has(name, method));
    }
    kind_of_type(annotation) == Some(Kind::Array)
}

/// Whether an expression is known to fit a method's receiver.
fn is_supported(source: &Source, expression: &Expression, method: &str) -> bool {
    if SET_METHODS.contains(&method) {
        return kind_of(source, expression) == Some(Kind::Set);
    }
    if method == "with" && kind_of(source, expression) == Some(Kind::Array) {
        return true;
    }
    if is_temporal_method(method) {
        return is_temporal(source, expression, method, 0);
    }
    kind_of(source, expression) == Some(Kind::Array)
}

/// Whether an expression is a `Temporal` value of a type having the method: built by `new` or
/// `.from()`, or bound or annotated so.
fn is_temporal(source: &Source, expression: &Expression, method: &str, depth: u8) -> bool {
    if depth > 4 {
        return false;
    }
    let constructor = match expression {
        Expression::NewExpression(new) => Some(&new.callee),
        Expression::CallExpression(call) if !call.optional => match &call.callee {
            Expression::StaticMemberExpression(member)
                if !member.optional && member.property.name == "from" =>
            {
                Some(&member.object)
            }
            _ => None,
        },
        Expression::TSAsExpression(wrapper) => {
            return temporal_type_name(&wrapper.type_annotation)
                .is_some_and(|name| temporal_has(name, method));
        }
        Expression::Identifier(identifier) => {
            let Some(symbol) = symbol_of(source, identifier) else {
                return false;
            };
            let nodes = source.semantic.nodes();
            return match nodes.kind(source.semantic.scoping().symbol_declaration(symbol)) {
                AstKind::VariableDeclarator(declarator) => {
                    if let Some(annotation) = &declarator.type_annotation {
                        return temporal_type_name(&annotation.type_annotation)
                            .is_some_and(|name| temporal_has(name, method));
                    }
                    declarator
                        .init
                        .as_ref()
                        .is_some_and(|init| is_temporal(source, init, method, depth + 1))
                }
                AstKind::FormalParameter(parameter) => parameter
                    .type_annotation
                    .as_ref()
                    .is_some_and(|annotation| {
                        temporal_type_name(&annotation.type_annotation)
                            .is_some_and(|name| temporal_has(name, method))
                    }),
                _ => false,
            };
        }
        _ => None,
    };
    constructor.is_some_and(|constructor| {
        matches!(constructor, Expression::StaticMemberExpression(member)
            if !member.optional
                && matches!(&member.object, Expression::Identifier(object) if object.name == "Temporal")
                && temporal_has(member.property.name.as_str(), method))
    })
}

/// The receiver a name stands for: itself when undeclared or a plain parameter, its
/// initializer when declared once and written nowhere before the call (`resolveReceiver`).
fn resolve<'a>(
    source: &Source<'_, 'a>,
    expression: &'a Expression<'a>,
    method: &str,
    call_start: u32,
    depth: u8,
) -> Receiver<'a, 'a> {
    if depth > 8 {
        return Receiver::Expression(expression);
    }
    match expression {
        Expression::Identifier(identifier) => {
            let Some(symbol) = symbol_of(source, identifier) else {
                return Receiver::Expression(expression);
            };
            let scoping = source.semantic.scoping();
            if scoping.symbol_redeclarations(symbol).len() > 1 {
                return Receiver::Uncertain;
            }
            let nodes = source.semantic.nodes();
            let written_before = scoping.get_resolved_reference_ids(symbol).iter().any(|id| {
                let reference = scoping.get_reference(*id);
                reference.is_write() && nodes.kind(reference.node_id()).span().start < call_start
            });
            if written_before {
                return Receiver::Uncertain;
            }
            let declaration = scoping.symbol_declaration(symbol);
            match nodes.kind(declaration) {
                AstKind::FormalParameter(parameter)
                    if matches!(parameter.pattern, BindingPattern::BindingIdentifier(_))
                        && parameter.initializer.is_none() =>
                {
                    let unsupported = parameter.optional
                        || parameter
                            .type_annotation
                            .as_ref()
                            .is_some_and(|annotation| {
                                !is_supported_type(&annotation.type_annotation, method)
                            });
                    if unsupported {
                        Receiver::Uncertain
                    } else {
                        Receiver::Expression(expression)
                    }
                }
                AstKind::VariableDeclarator(declarator)
                    if matches!(declarator.id, BindingPattern::BindingIdentifier(_)) =>
                {
                    if let Some(annotation) = &declarator.type_annotation {
                        return if is_supported_type(&annotation.type_annotation, method) {
                            Receiver::Expression(expression)
                        } else {
                            Receiver::Uncertain
                        };
                    }
                    match &declarator.init {
                        Some(init) => resolve(source, init, method, call_start, depth + 1),
                        None => Receiver::Uncertain,
                    }
                }
                _ => Receiver::Uncertain,
            }
        }
        Expression::ChainExpression(chain) => match &chain.expression {
            oxc_ast::ast::ChainElement::CallExpression(_) => Receiver::Expression(expression),
            oxc_ast::ast::ChainElement::TSNonNullExpression(wrapper) => {
                resolve(source, &wrapper.expression, method, call_start, depth + 1)
            }
            _ => Receiver::Uncertain,
        },
        Expression::TSNonNullExpression(wrapper) => {
            resolve(source, &wrapper.expression, method, call_start, depth + 1)
        }
        Expression::TSSatisfiesExpression(wrapper) => {
            resolve(source, &wrapper.expression, method, call_start, depth + 1)
        }
        Expression::StaticMemberExpression(_)
        | Expression::ComputedMemberExpression(_)
        | Expression::PrivateFieldExpression(_) => Receiver::Uncertain,
        Expression::TSAsExpression(wrapper) => {
            if is_supported_type(&wrapper.type_annotation, method) {
                Receiver::Expression(expression)
            } else {
                Receiver::Uncertain
            }
        }
        Expression::TSTypeAssertion(wrapper) => {
            if is_supported_type(&wrapper.type_annotation, method) {
                Receiver::Expression(expression)
            } else {
                Receiver::Uncertain
            }
        }
        _ => Receiver::Expression(expression),
    }
}

/// Whether a receiver is plainly no array: `undefined`, an object, a literal, an operation, a
/// template, a function, a class, a `new` other than `Array`'s, a call of `String()` and its
/// kind, or a name in `PascalCase` not known to be an array.
fn is_obviously_non_array(source: &Source, expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(identifier) => {
            (identifier.name == "undefined" && symbol_of(source, identifier).is_none())
                || (identifier
                    .name
                    .chars()
                    .next()
                    .is_some_and(char::is_uppercase)
                    && kind_of(source, expression) != Some(Kind::Array))
        }
        Expression::ObjectExpression(_)
        | Expression::StringLiteral(_)
        | Expression::NumericLiteral(_)
        | Expression::BooleanLiteral(_)
        | Expression::NullLiteral(_)
        | Expression::BigIntLiteral(_)
        | Expression::RegExpLiteral(_)
        | Expression::BinaryExpression(_)
        | Expression::TemplateLiteral(_)
        | Expression::ArrowFunctionExpression(_)
        | Expression::FunctionExpression(_)
        | Expression::ClassExpression(_) => true,
        Expression::NewExpression(new) => !is_global(source, &new.callee, "Array"),
        Expression::CallExpression(call) => NON_ARRAY_FACTORIES
            .iter()
            .any(|name| is_global(source, &call.callee, name)),
        _ => false,
    }
}

/// Whether a receiver is `expect(…)` or `expect.element(…)`, `.poll(…)`, `.soft(…)`.
fn is_expect_call(expression: &Expression) -> bool {
    let Expression::CallExpression(call) = expression else {
        return false;
    };
    match &call.callee {
        Expression::Identifier(callee) => callee.name == "expect",
        Expression::StaticMemberExpression(member) => {
            matches!(&member.object, Expression::Identifier(object) if object.name == "expect")
                && matches!(member.property.name.as_str(), "element" | "poll" | "soft")
        }
        _ => false,
    }
}

/// Whether the rule leaves a receiver alone.
fn skips_receiver(source: &Source, object: &Expression, method: &str, call_start: u32) -> bool {
    let requires_known = SET_METHODS.contains(&method) || is_temporal_method(method);
    let Receiver::Expression(resolved) = resolve(source, object, method, call_start, 0) else {
        return true;
    };
    if requires_known {
        return !is_supported(source, resolved, method);
    }
    if is_expect_call(resolved) {
        return true;
    }
    if method == "values" {
        return kind_of(source, resolved) != Some(Kind::Array);
    }
    is_obviously_non_array(source, resolved)
}

impl Rule for NoUnusedBuiltinMethodReturn {
    fn name(&self) -> &'static str {
        "unicorn/no-unused-builtin-method-return"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some((method, property, object)) = method_name(call) else {
            return;
        };
        let tracked = ARRAY_METHODS.contains(&method)
            || SET_METHODS.contains(&method)
            || is_temporal_method(method);
        if !tracked
            || !is_discarded(source, node.id())
            || is_lazy_helper(source, call, 0)
            || skips_receiver(source, object, method, call.span.start)
        {
            return;
        }
        findings.push(source.finding(
            self.name(),
            property.start,
            format!("Do not ignore the return value of `.{method}(…)`."),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnusedBuiltinMethodReturn;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnusedBuiltinMethodReturn, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_discarded_results() {
        assert_eq!(
            check("array.map(element => transform(element));"),
            ["Do not ignore the return value of `.map(…)`."]
        );
        assert_eq!(
            check("array.find(element => element.id === targetId);"),
            ["Do not ignore the return value of `.find(…)`."]
        );
        assert_eq!(
            check("const set = new Set([1, 2]);\nset.union(other);"),
            ["Do not ignore the return value of `.union(…)`."]
        );
        assert_eq!(
            check("const date = Temporal.PlainDate.from('2026-09-08');\ndate.add({days: 1});"),
            ["Do not ignore the return value of `.add(…)`."]
        );
        assert_eq!(
            check("async function f() { await list?.filter(Boolean); }"),
            ["Do not ignore the return value of `.filter(…)`."]
        );
        assert_eq!(
            check("for (list.map(f); ; list.slice()) {}"),
            [
                "Do not ignore the return value of `.map(…)`.",
                "Do not ignore the return value of `.slice(…)`."
            ]
        );
        assert_eq!(
            check("(list.includes(a), next());"),
            ["Do not ignore the return value of `.includes(…)`."]
        );
        assert_eq!(
            check("function f(items: readonly number[]) { items.values(); }"),
            ["Do not ignore the return value of `.values(…)`."]
        );
    }

    #[test]
    fn accepts_used_or_void_results_and_unknown_receivers() {
        for code in [
            "const transformed = array.map(element => transform(element));",
            "const match = array.find(element => element.id === targetId);",
            "void array.map(element => transform(element));",
            "const combined = set.union(other);",
            "const tomorrow = date.add({days: 1});",
            "array.forEach(f);",
            "array.sort();",
            "set.add(1);",
            "other.union(x);",
            "date.add({days: 1});",
            "wrapper.items.map(f);",
            "'text'.includes('t');",
            "Foo.map(f);",
            "new Thing().map(f);",
            "String(x).includes('t');",
            "expect(list).includes(1);",
            "map.values();",
            "ok && list.map(f);",
            "function f(list: Set<number>) { list.map(g); }",
            "let list = []; list = other; list.map(f);",
            "const it = list.values().map(f); it.filter(g);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
