//! `@angular-eslint/template/prefer-at-else`: an `@if` right after an `@if` of the opposite
//! condition is its `@else` (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-at-else.ts`): the two `@if`s, each of one branch or with an `@else`, are
//! siblings of Angular's tree, nothing but whitespace and comments between; their conditions
//! are opposite when one negates the other (`a`, `!a`), their operators are opposite on
//! equivalent operands (`a > b`, `a <= b`), or one tests a `length` against zero and the other
//! its opposite. Reported over the second `@if`'s name and fixed into an `@else`, its own `@else`
//! moved into the first `@if`; not fixed when the second has an alias, or when merging would
//! give a branch several roots that content projection needs alone.

use oxc_allocator::Allocator;

use super::r3::{
    Ng, R3, equivalent, is_zero, length_receiver, merged, of_bound, replace, siblings, slice,
};
use super::{Report, Template, TemplateRule};
use crate::finding::Fix;
use crate::template::bindings::block_parameters;
use crate::template::tags::split_ns_name;
use crate::template::tree::Block;

/// The rule.
pub struct PreferAtElse;

/// What an `@if` tests: its operands and its operator (`getIfNodeInfo`).
struct IfInfo<'t> {
    /// Its branches: the `@if`, then its `@else` when it has one.
    branches: Vec<&'t Block>,
    /// The left operand, or the whole condition.
    lhs: Ng,
    /// The right operand of a comparison.
    rhs: Option<Ng>,
    /// The operator: `''` for the condition alone, `!` for its negation.
    operator: String,
    /// Whether the condition names an alias (`as x`).
    aliased: bool,
}

/// The operators each one is the opposite of (`OPPOSITE_OPERATORS`).
const OPPOSITES: [(&str, &str); 10] = [
    ("", "!"),
    ("!", ""),
    ("<", ">="),
    (">", "<="),
    ("<=", ">"),
    (">=", "<"),
    ("==", "!="),
    ("!=", "=="),
    ("===", "!=="),
    ("!==", "==="),
];

/// What an `@if` of one branch, or with a plain `@else`, tests.
fn if_info<'t>(allocator: &Allocator, branches: &[&'t Block]) -> Option<IfInfo<'t>> {
    let first = *branches.first()?;
    let plain_else = match branches.get(1) {
        None => true,
        Some(second) => branches.len() == 2 && second.name == "else",
    };
    if !plain_else {
        return None;
    }
    let parameters = block_parameters(allocator, first).ok()?;
    let condition = of_bound(parameters.expression.as_ref()?);
    let (lhs, rhs, operator) = match condition {
        Ng::Binary(operator, left, right) => (*left, Some(*right), operator),
        Ng::Not(inner) => (*inner, None, "!".to_owned()),
        other => (other, None, String::new()),
    };
    Some(IfInfo {
        branches: branches.to_vec(),
        lhs,
        rhs,
        operator,
        aliased: parameters.alias.is_some(),
    })
}

/// Whether an `@if` tests that a `length` is zero (`isEmptyLength`).
fn is_empty_length(info: &IfInfo) -> bool {
    let Some(rhs) = &info.rhs else {
        return false;
    };
    (info.operator == "==" || info.operator == "===")
        && ((length_receiver(&info.lhs).is_some() && is_zero(rhs))
            || (is_zero(&info.lhs) && length_receiver(rhs).is_some()))
}

/// Whether an `@if` tests that a `length` is above zero (`isNonEmptyLength`).
fn is_non_empty_length(info: &IfInfo) -> bool {
    let Some(rhs) = &info.rhs else {
        return false;
    };
    (length_receiver(&info.lhs).is_some() && info.operator == ">" && is_zero(rhs))
        || (is_zero(&info.lhs) && info.operator == "<" && length_receiver(rhs).is_some())
}

/// Whether two `@if`s test opposite conditions (`canCombine`).
fn can_combine(previous: &IfInfo, current: &IfInfo) -> bool {
    let opposite = OPPOSITES
        .iter()
        .any(|(one, other)| *one == previous.operator && *other == current.operator);
    if opposite && equivalent(&previous.lhs, &current.lhs) {
        match (&previous.rhs, &current.rhs) {
            (None, None) => return true,
            (Some(a), Some(b)) if equivalent(a, b) => return true,
            _ => {}
        }
    }
    (is_empty_length(previous) && is_non_empty_length(current))
        || (is_non_empty_length(previous) && is_empty_length(current))
}

/// Whether nodes merged into one branch would have several roots, one of them an element or a
/// template (`wouldPreventContentProjection`).
fn would_prevent_content_projection(nodes: &[&Block]) -> bool {
    let mut roots = 0;
    let mut projectable = false;
    for block in nodes {
        for node in siblings(&block.children) {
            match node {
                R3::Text(text) if text.value.trim().is_empty() => continue,
                R3::Element(element) => {
                    projectable |= split_ns_name(&element.name).1 != "ng-content";
                }
                _ => {}
            }
            roots += 1;
        }
    }
    roots > 1 && projectable
}

/// The fix turning the second `@if` into the first's `@else` (`getFix`).
fn fix(text: &str, previous: &IfInfo, current: &IfInfo) -> Option<Fix> {
    let previous_if = previous.branches[0];
    let current_if = current.branches[0];
    let previous_else = previous.branches.get(1).copied();
    let current_else = current.branches.get(1).copied();
    if current.aliased {
        return None;
    }
    let blocked = previous_else
        .is_some_and(|block| would_prevent_content_projection(&[block, current_if]))
        || current_else
            .is_some_and(|block| would_prevent_content_projection(&[previous_if, block]));
    if blocked {
        return None;
    }
    let mut fixes = Vec::new();
    match previous_else {
        None => fixes.push(replace(
            current_if.span.start,
            current_if.start_span.end,
            "@else {",
        )),
        Some(previous_else) => {
            let contents = slice(text, current_if.start_span.end, current_if.span.end - 1);
            let at = previous_else.span.end - 1;
            fixes.push(replace(at, at, contents));
            fixes.push(replace(current_if.span.start, current_if.span.end, ""));
        }
    }
    if let (Some(current_else), Some(end)) = (current_else, current_if.end_span) {
        let contents = slice(text, current_else.start_span.end, current_else.span.end - 1);
        let at = previous_if.span.end - 1;
        fixes.push(replace(at, at, contents));
        fixes.push(replace(end.end, current_else.span.end, ""));
    }
    merged(text, fixes)
}

impl TemplateRule for PreferAtElse {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-at-else"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferAtElse",
            "Prefer using `@else` instead of a second `@if` clause.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        for list in template.lists() {
            let mut previous: Option<IfInfo> = None;
            for node in siblings(list) {
                let current = match &node {
                    R3::If(branches) => if_info(template.allocator, branches),
                    _ => None,
                };
                if let (Some(before), Some(now)) = (&previous, &current)
                    && can_combine(before, now)
                {
                    reports.push(Report {
                        fix: fix(template.text, before, now),
                        ..Report::new(now.branches[0].name_span, "preferAtElse")
                    });
                }
                previous = current;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferAtElse;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-at-else.txt", &[], &|_| Box::new(PreferAtElse));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 15 valid and 31 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (15, 31, 0));
    }
}
