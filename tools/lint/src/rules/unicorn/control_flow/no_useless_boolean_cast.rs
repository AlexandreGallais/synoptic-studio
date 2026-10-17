//! `unicorn/no-useless-boolean-cast`: `Boolean(…)` returned by the callback of an array's
//! predicate method — `every`, `filter`, `find`, `findIndex`, `findLast`, `findLastIndex`,
//! `some` — which reads the value for its truth anyway (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/no-useless-boolean-cast.md`). A cast of an optional chain stays: it turns
//! `undefined` into `false`; so does a callback that is async, a generator, has a written return
//! type, or casts its own first parameter. Autofixed when no comment is inside the cast.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrowFunctionExpression, Expression, FormalParameters, FunctionBody, LogicalOperator,
    Statement,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::{has_comment_in, is_global_call, text};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUselessBooleanCast;

/// The array methods that read their callback's result for its truth.
const PREDICATE_METHODS: [&str; 7] = [
    "every",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "some",
];

/// The value a block body returns by a lone `return`.
fn returned<'s, 'a>(body: &'s FunctionBody<'a>) -> Option<&'s Expression<'a>> {
    match body.statements.as_slice() {
        [Statement::ReturnStatement(statement)] => statement.argument.as_ref(),
        _ => None,
    }
}

/// Whether a value ends in an optional chain, through `&&`, `||`, `??`, a conditional or a
/// sequence.
fn ends_in_optional_chain(expression: &Expression) -> bool {
    match expression.get_inner_expression() {
        Expression::ChainExpression(_) => true,
        Expression::LogicalExpression(logical) => {
            (logical.operator == LogicalOperator::And && ends_in_optional_chain(&logical.left))
                || ends_in_optional_chain(&logical.right)
        }
        Expression::ConditionalExpression(conditional) => {
            ends_in_optional_chain(&conditional.consequent)
                || ends_in_optional_chain(&conditional.alternate)
        }
        Expression::SequenceExpression(sequence) => sequence
            .expressions
            .last()
            .is_some_and(ends_in_optional_chain),
        _ => false,
    }
}

/// A callback's parts: its parameters, its returned value, whether its body is that value.
struct Callback<'s, 'a> {
    parameters: &'s FormalParameters<'a>,
    value: &'s Expression<'a>,
    concise: bool,
}

/// The parts of a callback the rule reads: neither async, nor a generator, nor typed.
fn callback<'s, 'a>(argument: &'s Argument<'a>) -> Option<Callback<'s, 'a>> {
    match argument {
        Argument::ArrowFunctionExpression(arrow) => arrow_callback(arrow),
        Argument::FunctionExpression(function) => {
            if function.r#async || function.generator || function.return_type.is_some() {
                return None;
            }
            Some(Callback {
                parameters: &function.params,
                value: returned(function.body.as_ref()?)?,
                concise: false,
            })
        }
        _ => None,
    }
}

/// The parts of an arrow callback.
fn arrow_callback<'s, 'a>(arrow: &'s ArrowFunctionExpression<'a>) -> Option<Callback<'s, 'a>> {
    if arrow.r#async || arrow.return_type.is_some() {
        return None;
    }
    Some(Callback {
        parameters: &arrow.params,
        value: match arrow.get_expression() {
            Some(value) => value,
            None => returned(arrow.get_function_body()?)?,
        },
        concise: arrow.is_expression(),
    })
}

impl Rule for NoUselessBooleanCast {
    fn name(&self) -> &'static str {
        "unicorn/no-useless-boolean-cast"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Expression::StaticMemberExpression(member) = &call.callee else {
            return;
        };
        let method = member.property.name.as_str();
        if call.optional
            || member.optional
            || !PREDICATE_METHODS.contains(&method)
            || !(1..=2).contains(&call.arguments.len())
            || call.arguments.iter().any(Argument::is_spread)
        {
            return;
        }
        let Some(callback) = callback(&call.arguments[0]) else {
            return;
        };
        if !is_global_call(source, callback.value, "Boolean") {
            return;
        }
        let Expression::CallExpression(cast) = callback.value else {
            return;
        };
        let Some(argument) = cast.arguments[0].as_expression() else {
            return;
        };
        let casts_first_parameter = matches!(
            (callback.parameters.items.first().map(|first| &first.pattern), argument),
            (Some(oxc_ast::ast::BindingPattern::BindingIdentifier(first)), Expression::Identifier(name))
                if first.name == name.name
        );
        if casts_first_parameter || ends_in_optional_chain(argument) {
            return;
        }
        let message = format!("`Boolean()` is unnecessary in `Array#{method}()` callbacks.");
        if has_comment_in(source, cast.span) {
            findings.push(source.finding(self.name(), cast.span.start, message));
            return;
        }
        let inner = text(source, argument.span());
        let wrapped = callback.concise
            && (inner.starts_with('{')
                || matches!(
                    argument,
                    Expression::SequenceExpression(_) | Expression::AssignmentExpression(_)
                ));
        let fix = Fix {
            start: cast.span.start,
            end: cast.span.end,
            text: if wrapped {
                format!("({inner})")
            } else {
                inner.to_owned()
            },
        };
        findings.push(source.fixed(self.name(), cast.span.start, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUselessBooleanCast;
    use crate::testing::{fixed, lint, messages};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&NoUselessBooleanCast, "scripts/a.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn removes_the_cast_in_a_predicate_callback() {
        // The documentation's three cases.
        let code = "const enabledRecords = records.filter(record => Boolean(record.enabled));\nconst hasEnabledRecord = records.some(record => Boolean(record.enabled));\nconst allRecordsAreEnabled = records.every(record => Boolean(record.enabled));";
        assert_eq!(
            check(code),
            (
                vec![(1, 49), (2, 49), (3, 54)],
                "const enabledRecords = records.filter(record => record.enabled);\nconst hasEnabledRecord = records.some(record => record.enabled);\nconst allRecordsAreEnabled = records.every(record => record.enabled);".to_owned()
            )
        );
        assert_eq!(
            messages(&lint(&NoUselessBooleanCast, "scripts/a.ts", code))[0],
            "`Boolean()` is unnecessary in `Array#filter()` callbacks."
        );
        // A function expression with a lone return; an object literal kept as a concise body.
        assert_eq!(
            check("a.find(function (x) { return Boolean(x.y); });\na.some((x) => Boolean({ x }));")
                .1,
            "a.find(function (x) { return x.y; });\na.some((x) => ({ x }));"
        );
    }

    #[test]
    fn keeps_casts_that_change_the_value_or_are_not_predicates() {
        for code in [
            "const toBoolean = value => Boolean(value);",
            "const hasMatch = records.some(record => Boolean(record.get('name')?.includes('x')));",
            "records.map(record => Boolean(record.enabled));",
            "records.filter(record => Boolean(record));",
            "records.filter(async record => Boolean(record.a));",
            "records.filter((record): boolean => Boolean(record.a));",
            "records.filter(record => { g(); return Boolean(record.a); });",
            "records?.filter(record => Boolean(record.a));",
            "records.filter(record => Boolean(record.a, 1));",
            "records.filter(record => Boolean(record.a ?? record.b?.c));",
            "const Boolean = f; records.filter(record => Boolean(record.a));",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
        let commented = "a.some((x) => Boolean(/* why */ x.y));";
        assert_eq!(check(commented), (vec![(1, 15)], commented.to_owned()));
    }
}
