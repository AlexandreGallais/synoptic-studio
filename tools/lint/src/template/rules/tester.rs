//! A template rule run on the cases of its file of the plugin's tests, as `RuleTester` of
//! `@typescript-eslint/rule-tester` 8.71.0 runs them for `@angular-eslint/eslint-plugin-template`
//! 22.6.0 (REF-ANGULAR-ESLINT-TEMPLATE-TESTS, REF-TSESLINT-RULE-TESTER): a valid case reports
//! nothing; an invalid case reports its errors in order, each with its message's id, its message
//! when the case gives its data, and its first and last line and column when given; its template
//! is fixed pass by pass, at most 10, until a pass changes nothing, each pass giving the next
//! output, and a case without output is fixed by no pass. The suggestions are counted, never
//! checked: the linter carries no suggestion. A case this reading cannot build fails.

use std::fs;
use std::path::Path;

use super::super::corpus::{Case, Value, read_rule_cases};
use super::super::tree::parse;
use super::{Report, TemplateRule, interpolate, message, reports};
use crate::finding::apply;
use crate::lines::Lines;

/// What the cases of a file gave.
#[derive(Debug, Default)]
pub struct Tally {
    /// The valid cases run.
    pub valid: usize,
    /// The invalid cases run.
    pub invalid: usize,
    /// The suggestions the invalid cases expect, not checked.
    pub suggestions: usize,
    /// The cases whose `filename` is a TypeScript file, not run.
    pub typescript: usize,
    /// Each case that failed, and why.
    pub failures: Vec<String>,
}

/// A rule built from a case's first option, `None` when it gives none.
pub type Build<'b> = &'b dyn Fn(Option<&Value>) -> Box<dyn TemplateRule>;

/// Runs a rule on every case of its file of the plugin's tests, the names the file imports given
/// their values.
#[must_use]
pub fn run(file: &str, constants: &[(&str, &str)], build: Build) -> Tally {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/templates/angular-eslint")
        .join(file);
    let text = fs::read_to_string(&path).unwrap_or_default();
    let corpus = read_rule_cases(&text, constants);
    let mut tally = Tally {
        typescript: corpus.typescript,
        ..Tally::default()
    };
    if corpus.cases.is_empty() || corpus.dynamic + corpus.other > 0 {
        tally.failures.push(format!(
            "{file}: {} cases, {} dynamic, {} other",
            corpus.cases.len(),
            corpus.dynamic,
            corpus.other
        ));
    }
    for case in &corpus.cases {
        if case.valid {
            tally.valid += 1;
        } else {
            tally.invalid += 1;
            tally.suggestions += case.suggestions;
        }
        let rule = build(case.options.as_ref());
        if let Err(why) = check_case(rule.as_ref(), case) {
            tally
                .failures
                .push(format!("{} {}: {why}\n{}", file, case.origin, case.code));
        }
    }
    tally
}

/// The reports of a rule on a template, or why it could not be read.
fn lint(rule: &dyn TemplateRule, text: &str) -> Result<Vec<Report>, String> {
    let parsed = parse(text);
    let errors = super::super::bindings::check(&parsed);
    if let Some(error) = errors.first() {
        return Err(format!("unreadable: {}", error.message));
    }
    Ok(reports(rule, text, &parsed.nodes))
}

/// A report as the case reads it: its message's id, its message, its places.
fn describe(rule: &dyn TemplateRule, text: &str, report: &Report) -> String {
    let lines = Lines::new(text);
    let (line, column) = lines.place(text, report.span.start as usize);
    let (end_line, end_column) = lines.place(text, report.span.end as usize);
    format!(
        "{} {line}:{column}-{end_line}:{end_column} {}",
        report.message_id,
        message(rule, report)
    )
}

/// Whether a case passes, or why not.
fn check_case(rule: &dyn TemplateRule, case: &Case) -> Result<(), String> {
    if !case.unresolved.is_empty() {
        return Err(format!("unresolved {:?}", case.unresolved));
    }
    let found = lint(rule, &case.code)?;
    let described: Vec<String> = found
        .iter()
        .map(|report| describe(rule, &case.code, report))
        .collect();
    if found.len() != case.errors.len() {
        return Err(format!(
            "{} report(s), {} expected: {described:?}",
            found.len(),
            case.errors.len()
        ));
    }
    let lines = Lines::new(&case.code);
    for (index, (report, expected)) in found.iter().zip(&case.errors).enumerate() {
        let (line, column) = lines.place(&case.code, report.span.start as usize);
        let (end_line, end_column) = lines.place(&case.code, report.span.end as usize);
        let places = [
            (expected.line, line),
            (expected.column, column),
            (expected.end_line, end_line),
            (expected.end_column, end_column),
        ];
        let misplaced = places
            .iter()
            .any(|(wanted, got)| wanted.is_some_and(|wanted| wanted != *got));
        let wanted_message = expected.data.as_ref().map(|data| {
            let template = rule
                .messages()
                .iter()
                .find(|(id, _)| *id == expected.message_id)
                .map_or("", |(_, text)| text);
            interpolate(template, &data_of(data))
        });
        if report.message_id != expected.message_id
            || misplaced
            || wanted_message.is_some_and(|wanted| wanted != message(rule, report))
        {
            return Err(format!(
                "error {index}: {} expected {expected:?}",
                described.get(index).map_or("", String::as_str)
            ));
        }
    }
    let mut text = case.code.clone();
    let mut outputs = Vec::new();
    // `RuleTester` fixes pass by pass until a pass changes nothing, at most 10 passes; a
    // string `output` is the one pass expected, an array each pass, none no pass at all.
    for _ in 0..crate::FIX_PASSES {
        let fixes: Vec<_> = lint(rule, &text)?
            .into_iter()
            .filter_map(|report| report.fix)
            .collect();
        let pass = apply(&text, &fixes.iter().collect::<Vec<_>>());
        if pass == text {
            break;
        }
        text = pass;
        outputs.push(text.clone());
    }
    if outputs != case.output {
        return Err(format!(
            "outputs:\n{}\nexpected:\n{}",
            outputs.join("\n----\n"),
            case.output.join("\n----\n")
        ));
    }
    Ok(())
}

/// A case's data as the message's values.
fn data_of(data: &Value) -> Vec<(&str, String)> {
    match data {
        Value::Object(properties) => properties
            .iter()
            .map(|(name, value)| (name.as_str(), value.display()))
            .collect(),
        _ => Vec::new(),
    }
}
