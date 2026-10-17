//! `unicorn/prefer-aggregate-error`: `throw new Error(…)` guarded by a check that a collection
//! of errors is not empty — a name ending in `errors`, `errorList`, `errorArray` or
//! `errorCollection`, written as an array of built-in errors — throws an `AggregateError` of that
//! collection instead (`eslint-plugin-unicorn` 78.0.0, `docs/rules/prefer-aggregate-error.md`).
//! Autofixed unless a comment is inside the thrown error.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryOperator, Expression, LogicalOperator, Statement, TSTupleElement, TSType,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::no_error_property_assignment::BUILTIN_ERRORS;
use crate::expressions::{has_comment_in, is_global, symbol, text, unwrapped, written_type};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferAggregateError;

/// Whether a name says it holds errors.
fn is_collection_name(name: &str) -> bool {
    ["errors", "Errors"].iter().any(|end| name.ends_with(end))
        || ["error", "Error"].iter().any(|stem| {
            ["List", "Array", "Collection"]
                .iter()
                .any(|end| name.ends_with(&format!("{stem}{end}")))
        })
}

/// Whether a written type is a built-in error.
fn is_error_type(written: &TSType) -> bool {
    match written {
        TSType::TSTypeReference(reference) => reference
            .type_name
            .get_identifier_reference()
            .is_some_and(|name| BUILTIN_ERRORS.contains(&name.name.as_str())),
        TSType::TSUnionType(union) => union.types.iter().all(is_error_type),
        TSType::TSIntersectionType(intersection) => intersection.types.iter().any(is_error_type),
        TSType::TSTypeOperatorType(operator) => is_error_type(&operator.type_annotation),
        _ => false,
    }
}

/// Whether a written type is an array of built-in errors.
fn is_error_array_type(written: &TSType) -> bool {
    match written {
        TSType::TSArrayType(array) => is_error_type(&array.element_type),
        TSType::TSTypeOperatorType(operator) => is_error_array_type(&operator.type_annotation),
        TSType::TSTupleType(tuple) => {
            !tuple.element_types.is_empty()
                && tuple.element_types.iter().all(|element| match element {
                    TSTupleElement::TSRestType(rest) => is_error_array_type(&rest.type_annotation),
                    other => other.as_ts_type().is_some_and(is_error_type),
                })
        }
        TSType::TSTypeReference(reference) => {
            let name = reference
                .type_name
                .get_identifier_reference()
                .map(|each| each.name.as_str());
            matches!(name, Some("Array" | "ReadonlyArray"))
                && reference.type_arguments.as_ref().is_some_and(|arguments| {
                    matches!(arguments.params.as_slice(), [only] if is_error_type(only))
                })
        }
        TSType::TSUnionType(union) => union.types.iter().all(is_error_array_type),
        TSType::TSIntersectionType(intersection) => {
            intersection.types.iter().any(is_error_array_type)
        }
        _ => false,
    }
}

/// The collection a `.length` read checks, when it is a written array of errors.
fn collection<'s, 'a>(
    source: &Source,
    expression: &'s Expression<'a>,
) -> Option<&'s Expression<'a>> {
    let Expression::StaticMemberExpression(member) = unwrapped(expression) else {
        return None;
    };
    let object = unwrapped(&member.object);
    let Expression::Identifier(name) = object else {
        return None;
    };
    let written = symbol(source, name)
        .filter(|found| {
            source
                .semantic
                .scoping()
                .symbol_redeclarations(*found)
                .len()
                <= 1
        })
        .and_then(|found| written_type(source, found));
    let evidence = written.is_some_and(|each| is_error_array_type(&each.type_annotation));
    (member.property.name == "length"
        && !member.optional
        && is_collection_name(&name.name)
        && evidence)
        .then_some(object)
}

/// The number a literal writes.
fn number(expression: &Expression) -> Option<f64> {
    match unwrapped(expression) {
        Expression::NumericLiteral(each) => Some(each.value),
        _ => None,
    }
}

/// The collection a test checks is not empty: `errors.length`, `errors.length > 0`,
/// `errors.length !== 0`, `0 < errors.length`.
fn non_empty<'s, 'a>(source: &Source, test: &'s Expression<'a>) -> Option<&'s Expression<'a>> {
    let test = unwrapped(test);
    if let Some(found) = collection(source, test) {
        return Some(found);
    }
    let Expression::BinaryExpression(binary) = test else {
        return None;
    };
    let positive = |operator: BinaryOperator, value: &Expression, left: bool| {
        let zero = number(value) == Some(0.0);
        let value = number(value);
        matches!(
            operator,
            BinaryOperator::StrictInequality | BinaryOperator::Inequality
        ) && zero
            || value.is_some_and(|value| match (operator, left) {
                (BinaryOperator::GreaterThan, true) | (BinaryOperator::LessThan, false) => {
                    value >= 0.0
                }
                (BinaryOperator::GreaterEqualThan, true)
                | (BinaryOperator::LessEqualThan, false) => value >= 1.0,
                _ => false,
            })
    };
    if let Some(found) = collection(source, &binary.left)
        && positive(binary.operator, &binary.right, true)
    {
        return Some(found);
    }
    collection(source, &binary.right).filter(|_| positive(binary.operator, &binary.left, false))
}

/// The `&&` operands of a test.
fn conjuncts<'s, 'a>(test: &'s Expression<'a>, into: &mut Vec<&'s Expression<'a>>) {
    match unwrapped(test) {
        Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::And => {
            conjuncts(&logical.left, into);
            conjuncts(&logical.right, into);
        }
        other => into.push(other),
    }
}

impl Rule for PreferAggregateError {
    fn name(&self) -> &'static str {
        "unicorn/prefer-aggregate-error"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::IfStatement(guard) = node.kind() else {
            return;
        };
        let only = match &guard.consequent {
            Statement::BlockStatement(block) => match block.body.as_slice() {
                [only] => only,
                _ => return,
            },
            other => other,
        };
        let Statement::ThrowStatement(thrown) = only else {
            return;
        };
        let Expression::NewExpression(error) = &thrown.argument else {
            return;
        };
        let generic = matches!(&error.callee, Expression::Identifier(callee) if callee.name == "Error" && is_global(source, callee));
        if !generic
            || error.arguments.len() > 2
            || source
                .semantic
                .scoping()
                .symbol_names()
                .any(|name| name == "AggregateError")
        {
            return;
        }
        let mut parts = Vec::new();
        conjuncts(&guard.test, &mut parts);
        let found: Vec<&Expression> = parts
            .iter()
            .filter_map(|each| non_empty(source, each))
            .collect();
        let [errors] = found.as_slice() else {
            return;
        };
        let message = "Use `AggregateError` when throwing collected errors.".to_owned();
        if has_comment_in(source, error.span) {
            findings.push(source.finding(self.name(), error.callee.span().start, message));
            return;
        }
        let arguments: Vec<&str> = std::iter::once(text(source, errors.span()))
            .chain(error.arguments.iter().map(|each| text(source, each.span())))
            .collect();
        let fix = Fix {
            start: error.span.start,
            end: error.span.end,
            text: format!("new AggregateError({})", arguments.join(", ")),
        };
        findings.push(source.fixed(self.name(), error.callee.span().start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::PreferAggregateError;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferAggregateError, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn throws_an_aggregate_of_collected_errors() {
        // The documentation's two cases.
        assert_eq!(
            check("const errors: Error[] = [new Error('Email is required.')];\nif (errors.length > 0) {\n\tthrow new Error('Validation failed.');\n}"),
            (
                vec![(3, 12)],
                "const errors: Error[] = [new Error('Email is required.')];\nif (errors.length > 0) {\n\tthrow new AggregateError(errors, 'Validation failed.');\n}".to_owned()
            )
        );
        assert_eq!(
            check("const validationErrors: TypeError[] = [new TypeError('Invalid email.')];\nif (validationErrors.length !== 0) {\n\tthrow new Error('Validation failed.', {cause});\n}").1,
            "const validationErrors: TypeError[] = [new TypeError('Invalid email.')];\nif (validationErrors.length !== 0) {\n\tthrow new AggregateError(validationErrors, 'Validation failed.', {cause});\n}"
        );
        // Other checks and written types.
        for code in [
            "function f(errorList: readonly Error[]) { if (errorList.length) throw new Error('x'); }",
            "function f(errors: Array<RangeError>) { if (a && 0 < errors.length) { throw new Error(); } }",
            "function f(errors: [Error, ...TypeError[]]) { if (errors.length >= 1) { throw new Error(); } }",
        ] {
            assert_eq!(check(code).0.len(), 1, "{code}");
        }
    }

    #[test]
    fn leaves_unknown_collections_and_other_throws() {
        for code in [
            "const errors = [new Error('x')];\nif (errors.length > 0) { throw new Error('y'); }",
            "const errors: Error[] = [];\nconst errors: Error[] = [];\nif (errors.length) { throw new Error('y'); }",
            "const errors: Error[] = [];\nif (errors.length >= 0) { throw new Error('y'); }",
            "const items: Error[] = [];\nif (items.length > 0) { throw new Error('y'); }",
            "const errors: string[] = [];\nif (errors.length > 0) { throw new Error('y'); }",
            "const errors: Error[] = [];\nif (errors.length > 0) { throw new TypeError('y'); }",
            "const errors: Error[] = [];\nif (errors.length > 0) { g(); throw new Error('y'); }",
            "const errors: Error[] = [];\nif (errors.length < 1) { throw new Error('y'); }",
            "const errors: Error[] = [];\nif (errors.length === 0) { throw new Error('y'); }",
            "const errors: Error[] = [];\nconst AggregateError = f;\nif (errors.length) { throw new Error('y'); }",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented =
            "const errors: Error[] = [];\nif (errors.length) { throw new Error(/* why */ 'y'); }";
        assert_eq!(check(commented), (vec![(2, 32)], commented.to_owned()));
    }
}
