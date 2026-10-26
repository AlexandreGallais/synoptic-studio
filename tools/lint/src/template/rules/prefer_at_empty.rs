//! `@angular-eslint/template/prefer-at-empty`: an `@if` testing that a collection is empty,
//! next to a `@for` over it or around one, is the `@for`'s `@empty`
//! (`@angular-eslint/eslint-plugin-template` 22.6.0, `src/rules/prefer-at-empty.ts`): an "if
//! empty" right before or after the `@for`, or before an "if not empty" holding only the `@for`;
//! an "if empty" whose `@else` holds only the `@for`; an "if not empty" holding only the `@for`,
//! with an `@else`. A collection is empty when its `length` is zero or negated, not empty when
//! its `length` is read or above zero; the collections compare as Angular's trees. Reported over
//! the `@if` that goes, and fixed by moving its content into the `@empty`.

use oxc_allocator::Allocator;

use super::r3::{
    Ng, R3, equivalent, is_zero, length_receiver, merged, of_bound, replace, siblings, slice,
};
use super::{Report, Template, TemplateRule};
use crate::finding::Fix;
use crate::template::bindings::block_parameters;
use crate::template::lexer::Span;
use crate::template::tree::Block;

/// The rule.
pub struct PreferAtEmpty;

/// What a `@for` or an `@if` tests (`NodeInfo`).
enum Info<'t> {
    /// A `@for`, its `@empty`, its collection.
    For(For<'t>),
    /// An `@if` testing a collection, its kind, its branches.
    If(IfKind, Vec<&'t Block>, Ng),
}

/// A `@for` and what it loops over.
struct For<'t> {
    /// Its block.
    main: &'t Block,
    /// Its `@empty`.
    empty: Option<&'t Block>,
    /// Its collection.
    collection: Ng,
}

impl For<'_> {
    /// Its whole span, its `@empty` included (`sourceSpan`).
    fn span(&self) -> Span {
        let end = self
            .empty
            .and_then(|empty| empty.end_span)
            .or(self.main.end_span)
            .map_or(self.main.span.end, |end| end.end);
        Span::new(self.main.span.start as usize, end as usize)
    }
}

/// The kinds of `@if` the rule reads.
#[derive(Clone, Copy, PartialEq, Eq)]
enum IfKind {
    /// `@if (c.length === 0) {}`.
    Empty,
    /// `@if (c.length === 0) {} @else {}`.
    EmptyElse,
    /// `@if (c.length > 0) {}`.
    NotEmpty,
    /// `@if (c.length > 0) {} @else …`.
    NotEmptyElse,
}

/// A `@for` read.
fn for_of<'t>(allocator: &Allocator, main: &'t Block, empty: Option<&'t Block>) -> Option<For<'t>> {
    let parameters = block_parameters(allocator, main).ok()?;
    Some(For {
        main,
        empty,
        collection: of_bound(parameters.expression.as_ref()?),
    })
}

/// The collection a condition tests not empty (`getNotEmptyTestCollection`).
fn not_empty_collection(condition: &Ng) -> Option<&Ng> {
    if let Some(receiver) = length_receiver(condition) {
        return Some(receiver);
    }
    let Ng::Binary(operator, left, right) = condition else {
        return None;
    };
    let operator = operator.as_str();
    if let Some(receiver) = length_receiver(left) {
        return (matches!(operator, "!==" | ">" | "!=") && is_zero(right)).then_some(receiver);
    }
    if is_zero(left) && matches!(operator, "!==" | "<" | "!=") {
        return length_receiver(right);
    }
    None
}

/// The collection a condition tests empty (`getEmptyTestCollection`).
fn empty_collection(condition: &Ng) -> Option<&Ng> {
    match condition {
        Ng::Not(inner) => length_receiver(inner),
        Ng::Binary(operator, left, right) if matches!(operator.as_str(), "===" | "==") => {
            if let Some(receiver) = length_receiver(left) {
                return is_zero(right).then_some(receiver);
            }
            if is_zero(left) {
                return length_receiver(right);
            }
            None
        }
        _ => None,
    }
}

/// What a node of Angular's tree tests (`getNodeInfo`).
fn info_of<'t>(allocator: &Allocator, node: &R3<'t>) -> Option<Info<'t>> {
    match node {
        R3::For(main, empty) => for_of(allocator, main, *empty).map(Info::For),
        R3::If(branches) => {
            let first = *branches.first()?;
            let parameters = block_parameters(allocator, first).ok()?;
            let condition = of_bound(parameters.expression.as_ref()?);
            if let Some(collection) = not_empty_collection(&condition) {
                let kind = if branches.len() == 1 {
                    IfKind::NotEmpty
                } else {
                    IfKind::NotEmptyElse
                };
                return Some(Info::If(kind, branches.clone(), collection.clone()));
            }
            let collection = empty_collection(&condition)?.clone();
            match branches.as_slice() {
                [_] => Some(Info::If(IfKind::Empty, branches.clone(), collection)),
                [_, second] if second.name == "else" => {
                    Some(Info::If(IfKind::EmptyElse, branches.clone(), collection))
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// The one `@for` a branch holds, whitespace aside (`getOnlyForBlock`).
fn only_for<'t>(allocator: &Allocator, branch: &'t Block) -> Option<For<'t>> {
    let mut found = None;
    for node in siblings(&branch.children) {
        match node {
            R3::For(main, empty) => {
                if found.is_some() {
                    return None;
                }
                found = Some((main, empty));
            }
            R3::Text(text) if text.value.trim().is_empty() => {}
            _ => return None,
        }
    }
    let (main, empty) = found?;
    for_of(allocator, main, empty)
}

/// An `@if`'s spans: its whole, from its first branch to its last; its start, its first
/// branch's; its end, its last branch's `}`.
fn if_spans(branches: &[&Block]) -> (Span, Span, Option<Span>) {
    let first = branches[0];
    let last = branches[branches.len() - 1];
    (
        Span::new(first.span.start as usize, last.span.end as usize),
        first.start_span,
        last.end_span,
    )
}

/// The rule's checks of one node against the one before it, its report pushed.
struct Checker<'r> {
    /// The template's text.
    text: &'r str,
    /// The arena.
    allocator: &'r Allocator,
    /// The reports.
    reports: &'r mut Vec<Report>,
}

impl Checker<'_> {
    /// Pushes a report.
    fn report(&mut self, at: Span, fix: Option<Fix>) {
        self.reports.push(Report {
            fix,
            ..Report::new(at, "preferAtEmpty")
        });
    }

    /// A `@for` after an "if empty" of its collection (`checkFor`).
    fn check_for(&mut self, current: &For, previous: Option<&Info>) {
        let Some(Info::If(IfKind::Empty, branches, collection)) = previous else {
            return;
        };
        if !equivalent(&current.collection, collection) {
            return;
        }
        let branch = branches[0];
        let (whole, _, _) = if_spans(branches);
        let fix = branch.end_span.and_then(|branch_end| {
            let mut fixes = vec![replace(whole.start, whole.end, "")];
            if let Some(empty) = current.empty {
                let contents = slice(self.text, branch.start_span.end, branch_end.end - 1);
                let at = empty.start_span.end;
                fixes.push(replace(at, at, contents));
            } else {
                let contents = slice(self.text, branch.start_span.end, branch_end.end);
                let at = current.span().end;
                fixes.push(replace(at, at, &format!(" @empty {{{contents}")));
            }
            merged(self.text, fixes)
        });
        self.report(branch.name_span, fix);
    }

    /// An "if empty" after a `@for`, or after an "if not empty" holding only the `@for`
    /// (`checkIfEmpty`).
    fn check_if_empty(&mut self, branches: &[&Block], collection: &Ng, previous: Option<&Info>) {
        let (whole, start, _) = if_spans(branches);
        match previous {
            Some(Info::For(before)) if equivalent(collection, &before.collection) => {
                let fixes = match before.empty.and_then(|empty| empty.end_span) {
                    Some(end) => vec![
                        replace(end.start, end.end, ""),
                        replace(start.start, start.end, ""),
                    ],
                    None => vec![replace(start.start, start.end, "@empty {")],
                };
                let fix = merged(self.text, fixes);
                self.report(branches[0].name_span, fix);
            }
            Some(Info::If(IfKind::NotEmpty, before, before_collection))
                if equivalent(collection, before_collection) =>
            {
                let Some(inner) = only_for(self.allocator, before[0]) else {
                    return;
                };
                if !equivalent(collection, &inner.collection) {
                    return;
                }
                let (_, before_start, before_end) = if_spans(before);
                let fix = before_end.and_then(|before_end| {
                    let contents = slice(self.text, start.end, whole.end);
                    let at = inner.span().end;
                    merged(
                        self.text,
                        vec![
                            replace(before_start.start, before_start.end, ""),
                            replace(before_end.start, before_end.end, ""),
                            replace(at, at, &format!(" @empty {{{contents}")),
                            replace(whole.start, whole.end, ""),
                        ],
                    )
                });
                self.report(before[0].name_span, fix);
            }
            _ => {}
        }
    }

    /// An "if empty" whose `@else` holds only the `@for` (`checkIfEmptyElse`).
    fn check_if_empty_else(&mut self, branches: &[&Block], collection: &Ng) {
        let Some(inner) = only_for(self.allocator, branches[1]) else {
            return;
        };
        if !equivalent(collection, &inner.collection) {
            return;
        }
        let (whole, start, end) = if_spans(branches);
        let fix = branches[0].end_span.and_then(|if_branch_end| {
            let mut fixes = vec![replace(whole.start, branches[1].start_span.end, "")];
            let empty = slice(self.text, start.end, if_branch_end.start);
            let existing = inner
                .empty
                .and_then(|block| block.end_span.map(|end| (block, end)));
            if let Some((block, block_end)) = existing {
                fixes.push(replace(block.start_span.end, block_end.start, empty));
                if let Some(end) = end {
                    fixes.push(replace(end.start, end.end, ""));
                }
            } else {
                let at = inner.span().end;
                fixes.push(replace(at, at, &format!(" @empty {{{empty}")));
            }
            merged(self.text, fixes)
        });
        self.report(branches[0].name_span, fix);
    }

    /// An "if not empty" holding only the `@for`, after an "if empty" (`checkIfNotEmpty`).
    fn check_if_not_empty(
        &mut self,
        branches: &[&Block],
        collection: &Ng,
        previous: Option<&Info>,
    ) {
        let Some(Info::If(IfKind::Empty, before, before_collection)) = previous else {
            return;
        };
        if !equivalent(collection, before_collection) {
            return;
        }
        let Some(inner) = only_for(self.allocator, branches[0]) else {
            return;
        };
        if !equivalent(collection, &inner.collection) {
            return;
        }
        let (before_whole, before_start, _) = if_spans(before);
        let (_, start, _) = if_spans(branches);
        let contents = slice(self.text, before_start.end, before_whole.end - 1);
        let at = inner.span().end;
        let fix = merged(
            self.text,
            vec![
                replace(before_whole.start, before_whole.end, ""),
                replace(start.start, start.end, ""),
                replace(at, at, &format!(" @empty {{{contents}")),
            ],
        );
        self.report(branches[0].name_span, fix);
    }

    /// An "if not empty" holding only the `@for`, with an `@else` (`checkIfNotEmptyElse`).
    fn check_if_not_empty_else(&mut self, branches: &[&Block], collection: &Ng) {
        let Some(inner) = only_for(self.allocator, branches[0]) else {
            return;
        };
        if !equivalent(collection, &inner.collection) {
            return;
        }
        let (_, start, if_end) = if_spans(branches);
        let fix = branches[0]
            .end_span
            .zip(if_end)
            .and_then(|(if_branch_end, if_end)| {
                let mut fixes = Vec::new();
                if let Some(empty) = inner.empty {
                    fixes.push(replace(empty.span.start, empty.span.end, ""));
                }
                fixes.push(replace(start.start, start.end, ""));
                let else_branch = branches[1];
                if else_branch.name == "else if" {
                    fixes.push(replace(
                        if_branch_end.end - 1,
                        else_branch.name_span.end,
                        "@empty { @if ",
                    ));
                    fixes.push(replace(if_end.end, if_end.end, "}"));
                } else {
                    fixes.push(replace(
                        if_branch_end.end - 1,
                        else_branch.start_span.end,
                        "@empty {",
                    ));
                }
                merged(self.text, fixes)
            });
        self.report(branches[0].name_span, fix);
    }
}

impl TemplateRule for PreferAtEmpty {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-at-empty"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferAtEmpty",
            "Prefer using `@for (...) {...} @empty {...}`.",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let mut checker = Checker {
            text: template.text,
            allocator: template.allocator,
            reports,
        };
        for list in template.lists() {
            let mut previous: Option<Info> = None;
            for node in siblings(list) {
                let current = info_of(template.allocator, &node);
                match &current {
                    Some(Info::For(each)) => checker.check_for(each, previous.as_ref()),
                    Some(Info::If(kind, branches, collection)) => match kind {
                        IfKind::Empty => {
                            checker.check_if_empty(branches, collection, previous.as_ref());
                        }
                        IfKind::EmptyElse => checker.check_if_empty_else(branches, collection),
                        IfKind::NotEmpty => {
                            checker.check_if_not_empty(branches, collection, previous.as_ref());
                        }
                        IfKind::NotEmptyElse => {
                            checker.check_if_not_empty_else(branches, collection);
                        }
                    },
                    None => {}
                }
                previous = current;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferAtEmpty;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-at-empty.txt", &[], &|_| Box::new(PreferAtEmpty));

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 13 valid and 35 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (13, 35, 0));
    }
}
