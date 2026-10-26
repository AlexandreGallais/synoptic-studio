//! `@angular-eslint/template/prefer-template-literal`: a template literal rather than a
//! concatenation with `+` of strings (`@angular-eslint/eslint-plugin-template` 22.6.0,
//! `src/rules/prefer-template-literal.ts` and `src/utils/literal-primitive.ts`): the outermost
//! `+` of a chain holding a string or a template literal is reported over itself and fixed into
//! one literal, quoted as its first string when all its parts are literals, in backticks
//! otherwise, its `${…}` dropped when already inside a template literal. A file named as an
//! inline template is left, its backticks being the component's.

use oxc_ast::AstKind;
use oxc_ast::ast::{BinaryExpression, BinaryOperator, Expression};
use oxc_ast_visit::Visit;
use oxc_span::GetSpan;
use oxc_str::JSStr;
use regex::Regex;

use super::r3::{merged, replace, slice};
use super::{Report, Template, TemplateRule, every_bound, placed};
use crate::template::expressions::Bound;
use crate::template::lexer::{Span, offset};

/// The rule.
pub struct PreferTemplateLiteral;

/// The name of a component's inline template, as the plugin's processor writes it.
const INLINE_TEMPLATE: &str = r"inline-template-[^/\\]+-\d+\.component\.html$";

/// A part of a concatenation: a literal's text, or an expression's span.
enum Part {
    /// A literal's value.
    Literal(String),
    /// An expression, its parentheses removed.
    Expression(Span),
}

/// An expression without its parentheses (`unwrapParenthesizedExpression`).
fn unwrap<'r, 'a>(expression: &'r Expression<'a>) -> &'r Expression<'a> {
    match expression {
        Expression::ParenthesizedExpression(inner) => unwrap(&inner.expression),
        other => other,
    }
}

/// The `+` an expression is, if it is one.
fn plus<'r, 'a>(expression: &'r Expression<'a>) -> Option<&'r BinaryExpression<'a>> {
    match expression {
        Expression::BinaryExpression(binary) if binary.operator == BinaryOperator::Addition => {
            Some(binary)
        }
        _ => None,
    }
}

/// Whether a chain of `+` holds a string or a template literal (`chainContainsString`).
fn contains_string(expression: &Expression) -> bool {
    let unwrapped = unwrap(expression);
    if matches!(
        unwrapped,
        Expression::StringLiteral(_) | Expression::TemplateLiteral(_)
    ) {
        return true;
    }
    plus(unwrapped).is_some_and(|binary| chain_contains_string(binary))
}

/// Whether a `+` holds a string or a template literal on either side.
fn chain_contains_string(binary: &BinaryExpression) -> bool {
    contains_string(&binary.left) || contains_string(&binary.right)
}

/// A literal's value as JavaScript's `String` writes it, `None` when not a literal.
fn literal_text(expression: &Expression) -> Option<String> {
    match expression {
        Expression::StringLiteral(string) => Some(string.value.as_str()?.to_owned()),
        Expression::NumericLiteral(number) => Some(format!("{}", number.value)),
        Expression::BooleanLiteral(boolean) => Some(boolean.value.to_string()),
        Expression::NullLiteral(_) => Some("null".to_owned()),
        Expression::Identifier(name) if name.name == "undefined" => Some("undefined".to_owned()),
        _ => None,
    }
}

/// The parts of a concatenation, left to right (`flattenBinaryConcat`).
fn flatten(expression: &Expression, bound: &Bound, parts: &mut Vec<Part>) {
    let unwrapped = unwrap(expression);
    if let Some(binary) = plus(unwrapped).filter(|binary| chain_contains_string(binary)) {
        flatten(&binary.left, bound, parts);
        flatten(&binary.right, bound, parts);
        return;
    }
    if let Expression::TemplateLiteral(template) = unwrapped {
        for (index, quasi) in template.quasis.iter().enumerate() {
            let text = quasi
                .value
                .cooked
                .and_then(JSStr::as_str)
                .map_or_else(|| quasi.value.raw.to_string(), str::to_owned);
            if !text.is_empty() {
                parts.push(Part::Literal(text));
            }
            if let Some(inner) = template.expressions.get(index) {
                parts.push(Part::Expression(placed(bound, inner.span())));
            }
        }
        return;
    }
    match literal_text(unwrapped) {
        Some(text) => parts.push(Part::Literal(text)),
        None => parts.push(Part::Expression(placed(bound, unwrapped.span()))),
    }
}

/// Finds the outermost `+` of each chain holding a string, with whether a template literal is
/// its parent.
struct Finder<'a> {
    /// The kinds of the nodes open, the innermost last.
    stack: Vec<AstKind<'a>>,
    /// The chains found: the `+` and whether inside a template literal.
    found: Vec<(&'a BinaryExpression<'a>, bool)>,
}

impl<'a> Visit<'a> for Finder<'a> {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        if let AstKind::BinaryExpression(binary) = kind
            && binary.operator == BinaryOperator::Addition
        {
            let parent = self.stack.last();
            let in_chain = matches!(parent, Some(AstKind::BinaryExpression(outer))
                if outer.operator == BinaryOperator::Addition);
            let in_template = matches!(parent, Some(AstKind::TemplateLiteral(_)));
            if !in_chain && chain_contains_string(binary) {
                self.found.push((binary, in_template));
            }
        }
        self.stack.push(kind);
    }

    fn leave_node(&mut self, _kind: AstKind<'a>) {
        self.stack.pop();
    }
}

/// The quote the fixed literal takes (`getQuote`): none inside a template literal, a backtick
/// around an expression, else the first quote of the concatenation, `'` when it has none.
fn quote_of(source: &str, in_template: bool, all_literals: bool) -> &'static str {
    if in_template {
        return "";
    }
    if !all_literals {
        return "`";
    }
    for character in source.chars() {
        match character {
            '\'' => return "'",
            '"' => return "\"",
            '`' => return "`",
            _ => {}
        }
    }
    "'"
}

/// The report of a chain of `+` and its fix.
fn report(text: &str, bound: &Bound, binary: &BinaryExpression, in_template: bool) -> Report {
    let span = placed(bound, binary.span);
    let mut parts = Vec::new();
    flatten(&binary.left, bound, &mut parts);
    flatten(&binary.right, bound, &mut parts);
    let all_literals = parts.iter().all(|part| matches!(part, Part::Literal(_)));
    let quote = quote_of(slice(text, span.start, span.end), in_template, all_literals);
    let effective = if quote.is_empty() { "`" } else { quote };
    let mut replacement = String::new();
    for part in &parts {
        match part {
            Part::Literal(value) => {
                replacement.push_str(&value.replace(effective, &format!("\\{effective}")));
            }
            Part::Expression(at) => {
                replacement.push_str("${");
                replacement.push_str(slice(text, at.start, at.end));
                replacement.push('}');
            }
        }
    }
    let mut fixes = Vec::new();
    if in_template {
        let before = text.get(..span.start as usize).unwrap_or_default();
        if let Some(open) = before.rfind("${") {
            fixes.push(replace(offset(open), span.start, ""));
        }
        let after = text.get(span.end as usize..).unwrap_or_default();
        if let Some(close) = after.find('}') {
            fixes.push(replace(span.end, span.end + offset(close + 1), ""));
        }
    } else {
        replacement = format!("{quote}{replacement}{quote}");
    }
    fixes.push(replace(span.start, span.end, &replacement));
    Report {
        fix: merged(text, fixes),
        ..Report::new(span, "preferTemplateLiteral")
    }
}

impl TemplateRule for PreferTemplateLiteral {
    fn name(&self) -> &'static str {
        "@angular-eslint/template/prefer-template-literal"
    }

    fn messages(&self) -> &'static [(&'static str, &'static str)] {
        &[(
            "preferTemplateLiteral",
            "Prefer using template literal instead of concatenating strings or expressions",
        )]
    }

    fn check(&self, template: &Template, reports: &mut Vec<Report>) {
        let inline =
            Regex::new(INLINE_TEMPLATE).is_ok_and(|pattern| pattern.is_match(template.path));
        if inline {
            return;
        }
        for written in template.expressions() {
            for bound in every_bound(&written.bound) {
                let Some(expression) = &bound.expression else {
                    continue;
                };
                let mut finder = Finder {
                    stack: Vec::new(),
                    found: Vec::new(),
                };
                finder.visit_expression(expression);
                for (binary, in_template) in finder.found {
                    reports.push(report(template.text, bound, binary, in_template));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferTemplateLiteral;
    use crate::template::rules::tester::run;

    #[test]
    fn f52_ac2_passes_the_plugin_s_cases() {
        let tally = run("prefer-template-literal.txt", &[], &|_| {
            Box::new(PreferTemplateLiteral)
        });

        assert_eq!(tally.failures, Vec::<String>::new());
        // The plugin's 15 valid and 83 invalid cases, all run.
        assert_eq!((tally.valid, tally.invalid, tally.suggestions), (15, 83, 0));
    }
}
