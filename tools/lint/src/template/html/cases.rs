//! The cases of html-validate's tests a rule passes: each `it` of the rule's test file
//! (`src/rules/<rule>.spec.ts` of `html-validate` 11.17.0, REF-HTML-VALIDATE-SOURCE), copied into
//! `tests/fixtures/templates/html-validate/` and read by Oxc, run as its test runs it. A case
//! validates a markup (`validateString`) with the rule configured by `new HtmlValidate({ rules })`,
//! in its `it` or the `beforeAll` of a `describe` around it, and expects the report valid
//! (`toBeValid`), invalid (`toBeInvalid`), equal to a code frame (`toMatchInlineCodeframe`: each
//! error's message, line, column and markers, as `codeFrameColumns` of `src/utils/` draws them) or
//! holding exactly the errors given (`toHaveError`, `toHaveErrors`, `objectContaining` each). The
//! rows of `it.each` are built, each a case. A case is set apart, counted by its reason, when its
//! test does what a template linter cannot: validating a file of the repository, a markup through
//! a transform, with other rules, a preset or element metadata of its own; reading the rule's
//! documentation, the report's messages or their context; an external snapshot; or a markup
//! Angular's parser refuses (`<div/>`, an end tag on a void element), which never reaches the
//! rules.

use crate::javascript::JsText;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use oxc_allocator::Allocator;
use oxc_ast::ast::{
    Argument, ArrayExpressionElement, ArrowFunctionBody, BindingPattern, CallExpression,
    Expression, FormalParameters, ObjectExpression, ObjectPropertyKind, Statement,
    TaggedTemplateExpression,
};
use oxc_parser::Parser;
use oxc_span::SourceType;
use regex::Regex;

use super::elements::meta_for;
use super::view::Doc;
use crate::finding::apply;
use crate::lines::Lines;
use crate::template::bindings::check as read_errors;
use crate::template::corpus::{Value, literal, value};
use crate::template::rules::{TemplateRule, message, reports};
use crate::template::tree::parse;

/// An error a code frame or a `toHaveError` expects.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Expected {
    /// The message, its last period dropped for a code frame.
    message: String,
    /// The line, from 1.
    line: Option<usize>,
    /// The column, from 1.
    column: Option<usize>,
    /// The markers under the first line; for the element files, the size.
    markers: Option<usize>,
    /// The offset in UTF-16 units, for the element files.
    offset: Option<usize>,
}

/// What a case expects of a report.
#[derive(Clone, Debug)]
enum Check {
    /// No error.
    Valid,
    /// An error at least.
    Invalid,
    /// These errors, in order, from a code frame.
    Frame(Vec<Expected>),
    /// These errors, in order, by `objectContaining`.
    Errors(Vec<Expected>),
    /// A helper of the rule called on the first element of a tag gives this.
    Helper(String, String, bool),
}

/// A helper of a rule a test calls on an element (`isSimpleTable`), by its name.
pub type Helpers<'h> = &'h [(&'h str, &'h dyn Fn(&Doc, usize) -> bool)];

/// How a test configures the rule: its options, or why the case is set apart.
type Config = Result<Options, &'static str>;

/// A rule's configuration: its options, and the elements the test defines itself.
#[derive(Clone, Debug)]
struct Options {
    /// The rule's options.
    value: Option<Value>,
    /// The names of the elements given metadata of their own, none of HTML's.
    custom: Vec<String>,
}

/// The names of the elements a configuration's `elements` defines beside `"html5"`, or why the
/// configuration is set apart: metadata changing an HTML element, or not read.
fn custom_elements(elements: &Expression) -> Result<Vec<String>, &'static str> {
    let Expression::ArrayExpression(array) = elements else {
        return Err("element metadata of its own");
    };
    let mut custom = Vec::new();
    for element in &array.elements {
        match element {
            ArrayExpressionElement::StringLiteral(name) if name.value == "html5" => {}
            ArrayExpressionElement::ObjectExpression(object) => {
                for property in &object.properties {
                    let name = match property {
                        ObjectPropertyKind::ObjectProperty(property) => property.key.static_name(),
                        ObjectPropertyKind::SpreadProperty(_) => None,
                    };
                    match name {
                        Some(name)
                            if meta_for(&name.to_lowercase()).is_none() && !name.contains('*') =>
                        {
                            custom.push(name.to_lowercase());
                        }
                        _ => return Err("element metadata of its own"),
                    }
                }
            }
            _ => return Err("element metadata of its own"),
        }
    }
    Ok(custom)
}

/// A case: its title, the markups it validates and what it expects of each report.
#[derive(Debug)]
struct Case {
    /// Its title.
    title: String,
    /// Each check: the markup and the configuration validated, and the check.
    checks: Vec<Checked>,
}

/// What the cases of a test file gave.
#[derive(Debug, Default)]
pub struct Tally {
    /// The cases run whose reports must be valid.
    pub valid: usize,
    /// The other cases run.
    pub invalid: usize,
    /// The cases set apart, by reason.
    pub apart: BTreeMap<&'static str, usize>,
    /// Each case set apart: its title and why.
    pub apart_cases: Vec<(String, &'static str)>,
    /// Each case that failed, and why.
    pub failures: Vec<String>,
}

impl Tally {
    /// Sets a case apart.
    fn set_apart(&mut self, title: String, reason: &'static str) {
        *self.apart.entry(reason).or_default() += 1;
        self.apart_cases.push((title, reason));
    }

    /// Runs a case's checks and counts it, or sets it apart when Angular's parser refuses one
    /// of its markups.
    fn run_case(&mut self, file: &str, case: Case, build: Build, helpers: Helpers) {
        let outcomes: Vec<_> = case
            .checks
            .iter()
            .map(|(markup, options, check)| {
                let rule = build(options.as_ref());
                run_check(rule.as_ref(), markup, check, helpers).map_err(|why| (markup, why))
            })
            .collect();
        if outcomes.iter().any(|each| matches!(each, Err((_, None)))) {
            self.set_apart(case.title, "a markup Angular's parser refuses");
            return;
        }
        if case
            .checks
            .iter()
            .all(|(_, _, check)| check_is_empty(check))
        {
            self.valid += 1;
        } else {
            self.invalid += 1;
        }
        for (markup, why) in outcomes.into_iter().filter_map(Result::err) {
            self.failures.push(format!(
                "{file} {:?}:\n{markup}\n{}",
                case.title,
                why.unwrap_or_default()
            ));
        }
    }

    /// The cases set apart, all reasons together.
    #[must_use]
    pub fn apart_count(&self) -> usize {
        self.apart.values().sum()
    }
}

/// The names in scope: the strings and lists a test declares, and a row's values.
type Scope = Vec<(String, Value)>;

/// The scope's strings, as the template literals read them.
fn strings(scope: &Scope) -> Vec<(String, String)> {
    scope
        .iter()
        .filter_map(|(name, each)| match each {
            Value::Text(text) => Some((name.clone(), text.clone())),
            Value::Bool(_) | Value::Number(_) => Some((name.clone(), each.display())),
            _ => None,
        })
        .collect()
}

/// A text of the scope: a literal built, or a name bound to a string.
fn text_of(expression: &Expression, scope: &Scope) -> Result<String, &'static str> {
    if let Expression::Identifier(name) = expression
        && let Some((_, Value::Text(text))) =
            scope.iter().find(|(each, _)| each == name.name.as_str())
    {
        return Ok(text.clone());
    }
    match literal(expression, &strings(scope)) {
        Some(Ok(text)) => Ok(text),
        Some(Err(())) => Err("a markup built at run time"),
        None => Err("a markup not written as a literal"),
    }
}

/// A rule's configuration read from `new HtmlValidate({ … })`.
fn config_of(call: &Expression, rule: &str, scope: &Scope) -> Option<Config> {
    let Expression::NewExpression(new) = call else {
        return None;
    };
    if !matches!(&new.callee, Expression::Identifier(name) if name.name == "HtmlValidate") {
        return None;
    }
    let Some(Argument::ObjectExpression(object)) = new.arguments.first() else {
        return Some(Err("the default configuration, a preset"));
    };
    Some(config_object(object, rule, scope))
}

/// See `config_of`.
fn config_object(object: &ObjectExpression, rule: &str, scope: &Scope) -> Config {
    let mut options = Err("the rule not configured");
    let mut custom = Vec::new();
    for property in &object.properties {
        let ObjectPropertyKind::ObjectProperty(property) = property else {
            return Err("a configuration spread");
        };
        let key = property
            .key
            .static_name()
            .map(|name| name.to_string())
            .unwrap_or_default();
        match key.as_str() {
            "root" => {}
            "elements" => custom = custom_elements(&property.value)?,
            "rules" => {
                let Expression::ObjectExpression(rules) = &property.value else {
                    return Err("rules not written as an object");
                };
                for each in &rules.properties {
                    let ObjectPropertyKind::ObjectProperty(each) = each else {
                        return Err("rules not written as an object");
                    };
                    let name = each
                        .key
                        .static_name()
                        .map(|name| name.to_string())
                        .unwrap_or_default();
                    let setting = value(&each.value, &strings(scope));
                    let (severity, given) = match &setting {
                        Some(Value::Text(severity)) => (severity.clone(), None),
                        Some(Value::List(list)) => match list.as_slice() {
                            [Value::Text(severity)] => (severity.clone(), None),
                            [Value::Text(severity), given] => {
                                (severity.clone(), Some(given.clone()))
                            }
                            _ => return Err("rules not written as an object"),
                        },
                        _ => return Err("rules not written as an object"),
                    };
                    if name == rule {
                        options = if severity == "off" {
                            Err("the rule off")
                        } else {
                            Ok(given)
                        };
                    } else if severity != "off" {
                        return Err("other rules on");
                    }
                }
            }
            "extends" => return Err("a preset"),
            _ => return Err("other configuration"),
        }
    }
    options.map(|value| Options { value, custom })
}

/// The statements of a function's body.
fn body<'s, 'a>(expression: &'s Expression<'a>) -> Option<&'s [Statement<'a>]> {
    match expression {
        Expression::ArrowFunctionExpression(arrow) => match &arrow.body {
            ArrowFunctionBody::FunctionBody(body) => Some(body.statements.as_slice()),
            _ => None,
        },
        Expression::FunctionExpression(function) => function
            .body
            .as_ref()
            .map(|body| body.statements.as_slice()),
        _ => None,
    }
}

/// An argument as an expression.
fn argument<'s, 'a>(call: &'s CallExpression<'a>, index: usize) -> Option<&'s Expression<'a>> {
    call.arguments.get(index).and_then(Argument::as_expression)
}

/// The name of a call's callee: `it`, `it.each`, `expect`…
fn callee_name(expression: &Expression) -> String {
    match expression {
        Expression::Identifier(name) => name.name.to_string(),
        Expression::StaticMemberExpression(member) => {
            format!("{}.{}", callee_name(&member.object), member.property.name)
        }
        _ => String::new(),
    }
}

/// An expression without its `await`.
fn awaited<'s, 'a>(expression: &'s Expression<'a>) -> &'s Expression<'a> {
    match expression {
        Expression::AwaitExpression(inner) => &inner.argument,
        other => other,
    }
}

/// The expected errors of an inline code frame (`codeFrameColumns`).
fn frame(text: &str) -> Vec<Expected> {
    let mut found: Vec<Expected> = Vec::new();
    let mut pending: Option<Expected> = None;
    for line in text.lines() {
        let trimmed = line.js_trim_start().trim_start_matches('"');
        if let Some(rest) = trimmed
            .strip_prefix("error: ")
            .or_else(|| trimmed.strip_prefix("warning: "))
        {
            found.extend(pending.take());
            let message = rest.rsplit_once(" (").map_or(rest, |(message, _)| message);
            pending = Some(Expected {
                message: message.to_owned(),
                line: None,
                column: None,
                markers: None,
                offset: None,
            });
            continue;
        }
        let Some(expected) = pending.as_mut() else {
            continue;
        };
        let Some((gutter, after)) = line.split_once('|') else {
            continue;
        };
        if let Some(number) = gutter.js_trim_start().strip_prefix('>') {
            if expected.line.is_none() {
                expected.line = number.js_trim().parse().ok();
            }
        } else if gutter.js_trim().is_empty() && expected.column.is_none() && after.contains('^') {
            let marks = after.strip_prefix(' ').unwrap_or(after);
            let spacing = marks.find('^').unwrap_or(0);
            expected.column = Some(marks[..spacing].chars().count() + 1);
            expected.markers = Some(
                marks[spacing..]
                    .chars()
                    .take_while(|each| *each == '^')
                    .count(),
            );
        }
    }
    found.extend(pending);
    found
}

/// An error `toHaveError` expects, from an object or a rule and a message; `Err` when it
/// compares what the linter does not report (a context, a selector).
fn expected_error(
    rule: &str,
    call: &CallExpression,
    scope: &Scope,
) -> Result<Expected, &'static str> {
    let first = argument(call, 0).ok_or("an assertion not read")?;
    let found = match (value(first, &strings(scope)), argument(call, 1)) {
        (Some(Value::Text(id)), Some(second)) => {
            if call.arguments.len() > 2 {
                return Err("compares the message's context");
            }
            let text = text_of(second, scope).map_err(|_| "an assertion not read")?;
            (
                id,
                Value::Object(vec![("message".to_owned(), Value::Text(text))]),
            )
        }
        (Some(object @ Value::Object(_)), None) => {
            (object.text("ruleId").unwrap_or(rule).to_owned(), object)
        }
        _ => return Err("an assertion not read"),
    };
    expected_of(rule, &found.0, &found.1)
}

/// An error from an object `objectContaining` compares.
fn expected_of(rule: &str, id: &str, object: &Value) -> Result<Expected, &'static str> {
    if id != rule {
        return Err("expects another rule's error");
    }
    let Value::Object(properties) = object else {
        return Err("an assertion not read");
    };
    let mut expected = Expected {
        message: String::new(),
        line: None,
        column: None,
        markers: None,
        offset: None,
    };
    for (key, each) in properties {
        match (key.as_str(), each) {
            ("ruleId", _) => {}
            ("message", Value::Text(text)) => expected.message.clone_from(text),
            ("line", Value::Number(number)) => expected.line = format!("{number}").parse().ok(),
            ("column", Value::Number(number)) => expected.column = format!("{number}").parse().ok(),
            _ => return Err("compares the message's context"),
        }
    }
    Ok(expected)
}

/// Whether a test's hooks are `{ processAttribute }`, the mock transform of
/// `src/transform/mocks/attribute.ts`: `x="{{ y }}"` dynamic, as Angular reads it, and
/// `dynamic-x="y"` an alias of `x` bound to `y`.
fn is_attribute_mock(hooks: &ObjectExpression) -> bool {
    matches!(hooks.properties.as_slice(), [ObjectPropertyKind::ObjectProperty(property)]
        if property.key.static_name().is_some_and(|name| name == "processAttribute"))
}

/// A markup the mock transform reads, written as Angular binds it: `dynamic-x="y"` becomes
/// `[x]="y"`.
fn bound(markup: &str) -> String {
    Regex::new(r"dynamic-([\w-]+)=").map_or_else(
        |_| markup.to_owned(),
        |regex| regex.replace_all(markup, "[$1]=").into_owned(),
    )
}

/// A check on a markup whose bindings moved its columns: the errors' messages and lines kept,
/// their columns and markers not compared.
fn loose(check: Check) -> Check {
    let unplaced = |expected: Vec<Expected>| {
        expected
            .into_iter()
            .map(|each| Expected {
                column: None,
                markers: None,
                ..each
            })
            .collect()
    };
    match check {
        Check::Frame(expected) => Check::Frame(unplaced(expected)),
        Check::Errors(expected) => Check::Errors(unplaced(expected)),
        other => other,
    }
}

/// A check of a case: the markup and the options validated, and what is expected.
type Checked = (String, Option<Value>, Check);

/// A report a case validates: its markup, its options and whether through the mock transform;
/// or why the case is set apart.
type Run = Result<(String, Option<Value>, bool), &'static str>;

/// What an `it` has read so far.
struct It<'r> {
    /// The rule's name.
    rule: &'r str,
    /// The names in scope.
    locals: Scope,
    /// The rule's configuration.
    config: Config,
    /// The reports validated, by the name they are held in.
    runs: Vec<(String, Run)>,
    /// The documents parsed (`parser.parseHtml(markup)`), by the name they are held in.
    documents: Vec<(String, String)>,
    /// The results of a helper called on a document's element: their name, the helper, the
    /// document and the tag.
    results: Vec<(String, String, String, String)>,
    /// The checks read.
    checks: Vec<Checked>,
}

impl It<'_> {
    /// Reads a declaration: a configuration, a validation, or a text.
    fn declare(&mut self, name: &str, init: &Expression) -> Result<(), &'static str> {
        if let Some(found) = config_of(init, self.rule, &self.locals) {
            self.config = found;
            return Ok(());
        }
        if let Expression::CallExpression(call) = awaited(init) {
            match callee_name(&call.callee).rsplit('.').next() {
                Some("validateString") => {
                    let run = self.validation(call)?;
                    self.runs.push((name.to_owned(), run));
                    return Ok(());
                }
                Some("validateFile") => return Err("a file of the repository"),
                Some("getRuleDocumentation") => return Err("the rule's documentation"),
                Some("autofix" | "validateSource") => return Err("a source object or an autofix"),
                Some("parseHtml") => {
                    let markup = argument(call, 0)
                        .ok_or("a markup not written as a literal")
                        .and_then(|markup| text_of(markup, &self.locals))?;
                    self.documents.push((name.to_owned(), markup));
                    return Ok(());
                }
                _ => {}
            }
            if let Some(result) = self.helper_call(call) {
                self.results
                    .push((name.to_owned(), result.0, result.1, result.2));
                return Ok(());
            }
        }
        let text = text_of(init, &self.locals).map_err(|_| "reads the report's messages")?;
        self.locals.push((name.to_owned(), Value::Text(text)));
        Ok(())
    }

    /// A helper called on a document's element (`isSimpleTable(document.querySelector("table")!)`):
    /// the helper, the document and the tag.
    fn helper_call(&self, call: &CallExpression) -> Option<(String, String, String)> {
        let Expression::Identifier(helper) = &call.callee else {
            return None;
        };
        let Some(Expression::TSNonNullExpression(found)) = argument(call, 0) else {
            return None;
        };
        let Expression::CallExpression(query) = &found.expression else {
            return None;
        };
        let Expression::StaticMemberExpression(member) = &query.callee else {
            return None;
        };
        let Expression::Identifier(document) = &member.object else {
            return None;
        };
        let tag = argument(query, 0).and_then(|tag| text_of(tag, &self.locals).ok())?;
        let known = self
            .documents
            .iter()
            .any(|(name, _)| name == document.name.as_str());
        (member.property.name == "querySelector" && known)
            .then(|| (helper.name.to_string(), document.name.to_string(), tag))
    }

    /// A `validateString` read: its markup, rewritten as Angular binds it when the mock
    /// transform reads it.
    fn validation(&self, call: &CallExpression) -> Result<Run, &'static str> {
        let transformed = match call.arguments.get(1) {
            None => false,
            Some(Argument::ObjectExpression(hooks)) if is_attribute_mock(hooks) => true,
            Some(_) => return Err("a markup through a transform"),
        };
        let markup = argument(call, 0)
            .ok_or("a markup not written as a literal")
            .and_then(|markup| text_of(markup, &self.locals))
            .map(|markup| if transformed { bound(&markup) } else { markup });
        Ok(markup.and_then(|markup| {
            let options = self.config.clone()?;
            let lower = markup.to_lowercase();
            if options
                .custom
                .iter()
                .any(|name| lower.contains(&format!("<{name}")))
            {
                return Err("element metadata of its own");
            }
            Ok((markup, options.value, transformed))
        }))
    }

    /// Reads a statement of expression: a configuration assigned, `expect.assertions`, or an
    /// assertion on a report.
    fn expression(&mut self, expression: &Expression) -> Result<(), &'static str> {
        if let Expression::AssignmentExpression(assignment) = expression
            && let Some(found) = config_of(&assignment.right, self.rule, &self.locals)
        {
            self.config = found;
            return Ok(());
        }
        let Expression::CallExpression(call) = awaited(expression) else {
            return Err("other statements");
        };
        let Expression::StaticMemberExpression(member) = &call.callee else {
            return Err("other statements");
        };
        let Expression::CallExpression(subject) = &member.object else {
            if callee_name(&call.callee) == "expect.assertions" {
                return Ok(());
            }
            return Err("other statements");
        };
        if callee_name(&subject.callee) != "expect" {
            return Err("other statements");
        }
        let Some(Expression::Identifier(target)) = argument(subject, 0) else {
            return Err("asserts other than the report");
        };
        if let Some((_, helper, document, tag)) = self
            .results
            .iter()
            .find(|(name, ..)| name == target.name.as_str())
        {
            let Some(Expression::BooleanLiteral(expected)) = argument(call, 0) else {
                return Err("an assertion not read");
            };
            let markup = self
                .documents
                .iter()
                .find(|(name, _)| name == document)
                .map(|(_, markup)| markup.clone())
                .unwrap_or_default();
            let check = Check::Helper(helper.clone(), tag.clone(), expected.value);
            self.checks.push((markup, None, check));
            return Ok(());
        }
        let Some((_, run)) = self
            .runs
            .iter()
            .find(|(name, _)| name == target.name.as_str())
        else {
            return Err("asserts other than the report");
        };
        let (markup, options, transformed) = run.clone()?;
        let check = self.check(member.property.name.as_str(), call)?;
        self.checks.push((
            markup,
            options,
            if transformed { loose(check) } else { check },
        ));
        Ok(())
    }

    /// The check of a matcher.
    fn check(&self, matcher: &str, call: &CallExpression) -> Result<Check, &'static str> {
        match matcher {
            "toBeValid" => Ok(Check::Valid),
            "toBeInvalid" => Ok(Check::Invalid),
            "toMatchInlineCodeframe" => {
                let text = argument(call, 0)
                    .ok_or("an assertion not read")
                    .and_then(|text| {
                        text_of(text, &self.locals).map_err(|_| "an assertion not read")
                    })?;
                Ok(Check::Frame(frame(&text)))
            }
            "toHaveError" => Ok(Check::Errors(vec![expected_error(
                self.rule,
                call,
                &self.locals,
            )?])),
            "toHaveErrors" => {
                let Some(Expression::ArrayExpression(array)) = argument(call, 0) else {
                    return Err("an assertion not read");
                };
                let errors = array
                    .elements
                    .iter()
                    .map(|element| {
                        let found = element
                            .as_expression()
                            .and_then(|each| value(each, &strings(&self.locals)))
                            .ok_or("an assertion not read")?;
                        match &found {
                            Value::List(pair) => match pair.as_slice() {
                                [Value::Text(id), Value::Text(text)] => expected_of(
                                    self.rule,
                                    id,
                                    &Value::Object(vec![(
                                        "message".to_owned(),
                                        Value::Text(text.clone()),
                                    )]),
                                ),
                                _ => Err("an assertion not read"),
                            },
                            object => expected_of(
                                self.rule,
                                object.text("ruleId").unwrap_or(self.rule),
                                object,
                            ),
                        }
                    })
                    .collect::<Result<_, _>>()?;
                Ok(Check::Errors(errors))
            }
            "toMatchSnapshot" | "toMatchCodeframe" => Err("an external snapshot"),
            _ => Err("asserts other than the report"),
        }
    }
}

/// Reads the statements of an `it`, its checks or why it is set apart.
fn read_it(
    rule: &str,
    statements: &[Statement],
    scope: &Scope,
    config: &Config,
) -> Result<Vec<Checked>, &'static str> {
    let mut it = It {
        rule,
        locals: scope.clone(),
        config: config.clone(),
        runs: Vec::new(),
        documents: Vec::new(),
        results: Vec::new(),
        checks: Vec::new(),
    };
    for statement in statements {
        match statement {
            Statement::VariableDeclaration(variables) => {
                for declarator in &variables.declarations {
                    let Some(init) = &declarator.init else {
                        continue;
                    };
                    let Some(name) = declarator.id.get_identifier_name() else {
                        return Err("reads the report's messages");
                    };
                    it.declare(&name, init)?;
                }
            }
            Statement::ExpressionStatement(expression) => it.expression(&expression.expression)?,
            _ => return Err("other statements"),
        }
    }
    if it.checks.is_empty() {
        return Err("asserts nothing of a report");
    }
    Ok(it.checks)
}

/// The rows of an `it.each`: each its values bound to the test function's parameters, and its
/// title.
fn rows(
    table: &Expression,
    parameters: &FormalParameters,
    title: &str,
    scope: &Scope,
) -> Option<Vec<(Scope, String)>> {
    let list = match table {
        Expression::Identifier(name) => {
            match scope.iter().find(|(each, _)| each == name.name.as_str()) {
                Some((_, Value::List(list))) => list.clone(),
                _ => return None,
            }
        }
        other => match value(other, &strings(scope))? {
            Value::List(list) => list,
            _ => return None,
        },
    };
    list.into_iter()
        .map(|row| bind(parameters, &row, title, scope))
        .collect()
}

/// A row bound to the test function's parameters, and the title it gives.
fn bind(
    parameters: &FormalParameters,
    row: &Value,
    title: &str,
    scope: &Scope,
) -> Option<(Scope, String)> {
    let mut bound = scope.clone();
    let patterns: Vec<&BindingPattern> =
        parameters.items.iter().map(|item| &item.pattern).collect();
    let display = row.display();
    match (patterns.as_slice(), row) {
        ([single], Value::Object(properties)) if single.get_identifier_name().is_none() => {
            bound.extend(properties.iter().cloned());
        }
        ([single], _) => bound.push((single.get_identifier_name()?.to_string(), row.clone())),
        (_, Value::List(values)) => {
            for (pattern, each) in patterns.iter().zip(values) {
                bound.push((pattern.get_identifier_name()?.to_string(), each.clone()));
            }
        }
        _ => return None,
    }
    let mut titled = title.replacen("%s", &display, 1);
    if let Value::Object(properties) = row {
        for (key, each) in properties {
            titled = titled.replace(&format!("${key}"), &each.display());
        }
    }
    Some((bound, titled))
}

/// The rows of a table tagged `it.each`, a header of names then a cell `${…}` each, each row an
/// object of its columns.
fn table_rows(tagged: &TaggedTemplateExpression, scope: &Scope) -> Option<Vec<Value>> {
    let header = tagged.quasi.quasis.first()?.value.raw.as_str();
    let columns: Vec<String> = header
        .split('|')
        .map(|each| each.js_trim().to_owned())
        .filter(|each| !each.is_empty())
        .collect();
    if columns.is_empty() {
        return None;
    }
    let cells: Vec<Value> = tagged
        .quasi
        .expressions
        .iter()
        .map(|each| value(each, &strings(scope)))
        .collect::<Option<_>>()?;
    Some(
        cells
            .chunks(columns.len())
            .map(|row| Value::Object(columns.iter().cloned().zip(row.iter().cloned()).collect()))
            .collect(),
    )
}

/// The file's cases, and those set apart, read from a list of statements.
fn read_block(
    rule: &str,
    statements: &[Statement],
    scope: &mut Scope,
    config: &mut Config,
    cases: &mut Vec<Case>,
    apart: &mut Vec<(String, &'static str)>,
) {
    for statement in statements {
        if let Statement::VariableDeclaration(variables) = statement {
            for declarator in &variables.declarations {
                let (Some(name), Some(init)) =
                    (declarator.id.get_identifier_name(), &declarator.init)
                else {
                    continue;
                };
                if let Some(found) = config_of(init, rule, scope) {
                    *config = found;
                } else if let Some(found) = value(init, &strings(scope)) {
                    scope.push((name.to_string(), found));
                }
            }
            continue;
        }
        let Statement::ExpressionStatement(expression) = statement else {
            continue;
        };
        let Expression::CallExpression(call) = &expression.expression else {
            continue;
        };
        let name = callee_name(&call.callee);
        match name.as_str() {
            "describe" => {
                if let Some(statements) = argument(call, 1).and_then(body) {
                    let (mut inner_scope, mut inner_config) = (scope.clone(), config.clone());
                    read_block(
                        rule,
                        statements,
                        &mut inner_scope,
                        &mut inner_config,
                        cases,
                        apart,
                    );
                }
            }
            "beforeAll" | "beforeEach" => {
                for each in argument(call, 0).and_then(body).unwrap_or_default() {
                    if let Statement::ExpressionStatement(inner) = each
                        && let Expression::AssignmentExpression(assignment) = &inner.expression
                        && let Some(found) = config_of(&assignment.right, rule, scope)
                    {
                        *config = found;
                    }
                }
            }
            "it" => {
                let title = argument(call, 0)
                    .and_then(|title| text_of(title, scope).ok())
                    .unwrap_or_default();
                add_case(rule, call, title, scope, config, cases, apart);
            }
            _ => add_each(rule, call, scope, config, cases, apart),
        }
    }
}

/// Adds the case of an `it`, or sets it apart.
fn add_case(
    rule: &str,
    call: &CallExpression,
    title: String,
    scope: &Scope,
    config: &Config,
    cases: &mut Vec<Case>,
    apart: &mut Vec<(String, &'static str)>,
) {
    let statements = argument(call, 1).and_then(body).unwrap_or_default();
    match read_it(rule, statements, scope, config) {
        Ok(checks) => cases.push(Case { title, checks }),
        Err(reason) => apart.push((title, reason)),
    }
}

/// Adds the cases of an `it.each(…)(title, fn)`, or of a table tagged `it.each` and called with
/// a title and a function.
fn add_each(
    rule: &str,
    call: &CallExpression,
    scope: &Scope,
    config: &Config,
    cases: &mut Vec<Case>,
    apart: &mut Vec<(String, &'static str)>,
) {
    let title = argument(call, 0)
        .and_then(|title| text_of(title, scope).ok())
        .unwrap_or_default();
    let Some(function @ Expression::ArrowFunctionExpression(arrow)) = argument(call, 1) else {
        return;
    };
    let statements = body(function).unwrap_or_default();
    let rows = match &call.callee {
        Expression::CallExpression(each) if callee_name(&each.callee) == "it.each" => {
            argument(each, 0).and_then(|table| rows(table, &arrow.params, &title, scope))
        }
        Expression::TaggedTemplateExpression(tagged) if callee_name(&tagged.tag) == "it.each" => {
            table_rows(tagged, scope).and_then(|table| {
                table
                    .into_iter()
                    .map(|row| bind(&arrow.params, &row, &title, scope))
                    .collect()
            })
        }
        _ => return,
    };
    let Some(rows) = rows else {
        apart.push((title, "rows not read"));
        return;
    };
    for (bound, named) in rows {
        match read_it(rule, statements, &bound, config) {
            Ok(checks) => cases.push(Case {
                title: named,
                checks,
            }),
            Err(reason) => apart.push((named, reason)),
        }
    }
}

/// An error as a frame or `objectContaining` compares it, from a report.
fn actual(
    rule: &dyn TemplateRule,
    text: &str,
    report: &crate::template::rules::Report,
) -> Expected {
    let lines = Lines::new(text);
    let start = report.span.start as usize;
    let (line, column) = lines.place(text, start);
    let end = report.span.end as usize;
    let line_end = text[start..]
        .find(['\n', '\r'])
        .map_or(text.len(), |at| start + at);
    let marked = text.get(start..end.min(line_end)).unwrap_or_default();
    let markers = marked.encode_utf16().count().max(1);
    Expected {
        message: message(rule, report),
        line: Some(line),
        column: Some(column),
        markers: Some(markers),
        offset: None,
    }
}

/// Whether an error matches an expected one: its message, and its places when given.
fn same(found: &Expected, expected: &Expected, frame: bool) -> bool {
    let wanted = if frame {
        strip_period(&found.message)
    } else {
        found.message.clone()
    };
    wanted == expected.message
        && [
            (expected.line, found.line),
            (expected.column, found.column),
            (expected.markers, found.markers),
        ]
        .iter()
        .all(|(wanted, got)| wanted.is_none() || wanted == got)
}

/// A message as a code frame writes it: a last period after a non-space dropped.
fn strip_period(message: &str) -> String {
    match message.strip_suffix('.') {
        Some(rest) if !rest.ends_with(' ') && !rest.is_empty() => rest.to_owned(),
        _ => message.to_owned(),
    }
}

/// Runs a check, or says why it failed; `Err(None)` when Angular's parser refuses the markup.
fn run_check(
    rule: &dyn TemplateRule,
    markup: &str,
    check: &Check,
    helpers: Helpers,
) -> Result<(), Option<String>> {
    let parsed = parse(markup);
    if !read_errors(&parsed).is_empty() {
        return Err(None);
    }
    if let Check::Helper(name, tag, expected) = check {
        let doc = Doc::new(markup, &parsed.nodes);
        let element = (0..doc.elements.len()).find(|index| doc.el(*index).tag == *tag);
        let helper = helpers.iter().find(|(each, _)| each == name);
        return match (helper, element) {
            (Some((_, helper)), Some(element)) if helper(&doc, element) == *expected => Ok(()),
            _ => Err(Some(format!("{name}({tag}) is not {expected}"))),
        };
    }
    let found: Vec<Expected> = reports(rule, "a.component.html", markup, &parsed.nodes)
        .iter()
        .map(|report| actual(rule, markup, report))
        .collect();
    let ok = match check {
        Check::Valid => found.is_empty(),
        Check::Invalid => !found.is_empty(),
        Check::Helper(..) => false,
        Check::Frame(expected) | Check::Errors(expected) => {
            let frame = matches!(check, Check::Frame(_));
            found.len() == expected.len()
                && found
                    .iter()
                    .zip(expected)
                    .all(|(found, expected)| same(found, expected, frame))
        }
    };
    if ok {
        Ok(())
    } else {
        Err(Some(format!("{check:?}\nfound {found:?}")))
    }
}

/// The text after a rule's fixes, pass by pass (at most 10), as html-validate's tests read an
/// autofix; for the fix cases of a rule's own tests.
#[must_use]
pub fn fixed(rule: &dyn TemplateRule, markup: &str) -> String {
    let mut text = markup.to_owned();
    for _ in 0..crate::FIX_PASSES {
        let parsed = parse(&text);
        let fixes: Vec<_> = reports(rule, "a.component.html", &text, &parsed.nodes)
            .into_iter()
            .filter_map(|report| report.fix)
            .collect();
        let pass = apply(&text, &fixes.iter().collect::<Vec<_>>());
        if pass == text {
            break;
        }
        text = pass;
    }
    text
}

/// The messages of a rule's reports on a markup, each after its line and column, for a rule's
/// own cases.
#[must_use]
pub fn messages(rule: &dyn TemplateRule, markup: &str) -> Vec<String> {
    let parsed = parse(markup);
    reports(rule, "a.component.html", markup, &parsed.nodes)
        .iter()
        .map(|report| {
            let found = actual(rule, markup, report);
            format!(
                "{}:{} {}",
                found.line.unwrap_or_default(),
                found.column.unwrap_or_default(),
                found.message
            )
        })
        .collect()
}

/// A rule built from a case's options, `None` for none.
pub type Build<'b> = &'b dyn Fn(Option<&Value>) -> Box<dyn TemplateRule>;

/// Runs a rule on every case of its test file.
#[must_use]
pub fn run(file: &str, rule: &str, build: Build) -> Tally {
    run_except(file, rule, &[], build)
}

/// Runs a rule on every case of its test file but those its known divergences set apart.
#[must_use]
pub fn run_except(file: &str, rule: &str, known: &[(&str, &'static str)], build: Build) -> Tally {
    run_with(file, rule, known, &[], build)
}

/// Runs a rule on every case of its test file but those its known divergences set apart (each
/// case's title, and why html-validate and Angular read its markup apart), the helpers its
/// tests call given.
#[must_use]
pub fn run_with(
    file: &str,
    rule: &str,
    known: &[(&str, &'static str)],
    helpers: Helpers,
    build: Build,
) -> Tally {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/templates/html-validate")
        .join(file);
    let mut tally = Tally::default();
    let Ok(text) = fs::read_to_string(&path) else {
        tally.failures.push(format!("{file}: cannot be read"));
        return tally;
    };
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &text, SourceType::ts()).parse();
    let (mut cases, mut apart) = (Vec::new(), Vec::new());
    read_block(
        rule,
        &parsed.program.body,
        &mut Vec::new(),
        &mut Err("the rule not configured"),
        &mut cases,
        &mut apart,
    );
    if cases.is_empty() && apart.is_empty() {
        tally.failures.push(format!("{file}: no case read"));
    }
    for (title, reason) in apart {
        tally.set_apart(title, reason);
    }
    for case in cases {
        let divergence = known
            .iter()
            .find(|(title, _)| *title == case.title)
            .map(|(_, reason)| *reason);
        if let Some(reason) = divergence.or_else(|| {
            case.checks
                .iter()
                .find_map(|(markup, _, _)| not_a_template(markup))
        }) {
            tally.set_apart(case.title, reason);
            continue;
        }
        tally.run_case(file, case, build, helpers);
    }
    tally
}

/// An error of the element files' snapshot: its rule, and the error as compared.
type Snapped = (String, Expected);

/// A string of the snapshot unescaped: the template literal's escapes, then pretty-format's.
fn unescaped(text: &str) -> String {
    let literal = text
        .replace("\\`", "`")
        .replace("\\${", "${")
        .replace("\\\\", "\\");
    let mut found = String::new();
    let mut chars = literal.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            found.extend(chars.next());
        } else {
            found.push(character);
        }
    }
    found
}

/// The errors of each element file in the snapshot of html-validate's element tests
/// (`src/elements/__snapshots__/html5.spec.ts.snap`): by file name, each error's rule, message,
/// line, column and size.
fn snapshot(text: &str) -> Vec<(String, Vec<Snapped>)> {
    let mut found: Vec<(String, Vec<Snapped>)> = Vec::new();
    let mut current: Option<Snapped> = None;
    for line in text.lines() {
        if let Some(name) = line.strip_prefix("exports[`") {
            let name = name.split('`').next().unwrap_or_default();
            let words: Vec<&str> = name.split(' ').collect();
            let slug = match words.as_slice() {
                ["HTML", "elements", tag, variant, "markup", _] => format!(
                    "{}-{variant}",
                    tag.trim_matches(['<', '>']).replace(':', "_")
                ),
                [
                    "HTML",
                    "elements",
                    "global",
                    "attributes",
                    variant,
                    "markup",
                    _,
                ] => {
                    format!("global-attributes-{variant}")
                }
                _ => continue,
            };
            found.push((slug, Vec::new()));
            continue;
        }
        if line == "      {" {
            current = Some((
                String::new(),
                Expected {
                    message: String::new(),
                    line: None,
                    column: None,
                    markers: None,
                    offset: None,
                },
            ));
            continue;
        }
        if line == "      }," || line == "      }" {
            if let (Some(each), Some((_, list))) = (current.take(), found.last_mut()) {
                list.push(each);
            }
            continue;
        }
        let (Some((rule, expected)), Some(field)) =
            (current.as_mut(), line.strip_prefix("        \""))
        else {
            continue;
        };
        let Some((key, value)) = field.split_once("\": ") else {
            continue;
        };
        let value = value.strip_suffix(',').unwrap_or(value);
        let number = || value.parse::<usize>().ok();
        let string = || {
            unescaped(
                value
                    .strip_prefix('"')
                    .and_then(|inner| inner.strip_suffix('"'))
                    .unwrap_or(value),
            )
        };
        match key {
            "ruleId" => *rule = string(),
            "message" => expected.message = string(),
            // html-validate places a raw character of a text on the text's first line
            // (`sliceLocation` without the text), its offset right: the offset is compared.
            "offset" => expected.offset = number(),
            "size" => expected.markers = number(),
            _ => {}
        }
    }
    found
}

/// What the element files gave.
#[derive(Debug, Default)]
pub struct Elements {
    /// The files run.
    pub files: usize,
    /// The errors of the rules run the snapshot holds, each compared.
    pub errors: usize,
    /// The files set apart, by name, and why.
    pub refused: Vec<(String, &'static str)>,
    /// Each file whose errors differ, and how.
    pub failures: Vec<String>,
}

/// Runs rules on every element file of html-validate's element tests (`test-files/elements/`,
/// copied into `tests/fixtures/templates/html-validate/elements/`), each against the errors the
/// snapshot holds for its name: their message, line, column and size, in order. The tests run
/// the preset `html-validate:recommended` with some rules off (`src/elements/html5.spec.ts`);
/// only the rules given are compared, under their names. A file holding a whole document
/// (`<html>`, a doctype) is a template no more than one Angular's parser refuses.
#[must_use]
pub fn run_elements(
    rules: &[(&str, &dyn TemplateRule)],
    known: &[(&str, &'static str)],
) -> Elements {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/templates/html-validate/elements");
    let mut tally = Elements::default();
    let Ok(text) = fs::read_to_string(folder.join("html5.spec.snap.txt")) else {
        tally
            .failures
            .push("the snapshot cannot be read".to_owned());
        return tally;
    };
    for (slug, snapped) in snapshot(&text) {
        let Ok(markup) = fs::read_to_string(folder.join(format!("{slug}.txt"))) else {
            tally.failures.push(format!("{slug}: cannot be read"));
            continue;
        };
        let parsed = parse(&markup);
        let reason = if read_errors(&parsed).is_empty() {
            not_a_template(&markup).or_else(|| {
                known
                    .iter()
                    .find(|(file, _)| *file == slug)
                    .map(|(_, reason)| *reason)
            })
        } else {
            Some("a markup Angular's parser refuses")
        };
        if let Some(reason) = reason {
            tally.refused.push((slug, reason));
            continue;
        }
        tally.files += 1;
        let mut found: Vec<(usize, String, Expected)> = Vec::new();
        for (name, rule) in rules {
            for report in reports(*rule, "a.component.html", &markup, &parsed.nodes) {
                let start = report.span.start as usize;
                let error = Expected {
                    message: message(*rule, &report),
                    line: None,
                    column: None,
                    markers: Some(
                        markup[start..report.span.end as usize]
                            .encode_utf16()
                            .count(),
                    ),
                    offset: Some(markup[..start].encode_utf16().count()),
                };
                found.push((report.span.start as usize, (*name).to_owned(), error));
            }
        }
        // Errors at one place come in the order html-validate's events fire; both lists are
        // compared sorted by place, rule and message.
        found.sort_by(|(_, rule, error), (_, other_rule, other)| {
            (error.offset, rule, &error.message).cmp(&(other.offset, other_rule, &other.message))
        });
        let mut expected: Vec<&Snapped> = snapped
            .iter()
            .filter(|(rule, _)| rules.iter().any(|(name, _)| name == rule))
            .collect();
        expected.sort_by(|(rule, error), (other_rule, other)| {
            (error.offset, rule, &error.message).cmp(&(other.offset, other_rule, &other.message))
        });
        tally.errors += expected.len();
        let differs = found.len() != expected.len()
            || found
                .iter()
                .zip(&expected)
                .any(|((_, rule, error), (wanted, snapped))| rule != wanted || error != snapped);
        if differs {
            let found: Vec<String> = found
                .iter()
                .map(|(_, rule, error)| described(rule, error))
                .collect();
            let expected: Vec<String> = expected
                .iter()
                .map(|(rule, error)| described(rule, error))
                .collect();
            tally.failures.push(format!(
                "{slug}\nextra:\n{}\nmissing:\n{}",
                lacking(&found, &expected),
                lacking(&expected, &found)
            ));
        }
    }
    tally
}

/// Why a markup is no template a rule of the project meets, when it is not: a whole document,
/// whose `<head>` and `<body>` html-validate's parser opens itself and Angular's does not; or a
/// markup holding html-validate's own disable comments.
fn not_a_template(markup: &str) -> Option<&'static str> {
    if markup.contains("[html-validate-") {
        return Some("html-validate's own disable comments");
    }
    Regex::new(r"(?i)<(html|head|body)[\s>/]|<!doctype")
        .is_ok_and(|regex| regex.is_match(markup))
        .then_some("a whole document, never a template")
}

/// An error of an element file, described for a failure.
fn described(rule: &str, error: &Expected) -> String {
    format!(
        "{rule} {:?}+{:?} {}",
        error.offset, error.markers, error.message
    )
}

/// The lines of one list another lacks, joined.
fn lacking(one: &[String], other: &[String]) -> String {
    one.iter()
        .filter(|each| !other.contains(each))
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether a check expects no error.
fn check_is_empty(check: &Check) -> bool {
    match check {
        Check::Valid => true,
        Check::Frame(expected) | Check::Errors(expected) => expected.is_empty(),
        Check::Invalid | Check::Helper(..) => false,
    }
}

/// The markups a file's cases set apart because Angular's parser refuses them, for a review of
/// the reader: each case's title.
#[must_use]
pub fn refused(file: &str, rule: &str) -> Vec<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/templates/html-validate")
        .join(file);
    let text = fs::read_to_string(&path).unwrap_or_default();
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &text, SourceType::ts()).parse();
    let (mut cases, mut apart) = (Vec::new(), Vec::new());
    read_block(
        rule,
        &parsed.program.body,
        &mut Vec::new(),
        &mut Err(""),
        &mut cases,
        &mut apart,
    );
    cases
        .iter()
        .filter(|case| {
            case.checks
                .iter()
                .any(|(markup, _, _)| !read_errors(&parse(markup)).is_empty())
        })
        .map(|case| case.title.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Expected, frame, strip_period};

    #[test]
    fn reads_a_code_frame() {
        // Our own case: two errors, the second on a line of tabs, as `codeFrameColumns` draws them.
        let text = "\n\"error: A. (r)\n> 1 |  <p a>\n    |     ^\nSelector: p\n\nerror: B (r)\n  1 |\n> 2 | \t<i>\n    | \t ^^\nSelector: i\"\n";
        assert_eq!(
            frame(text),
            [
                Expected {
                    message: "A.".to_owned(),
                    line: Some(1),
                    column: Some(5),
                    markers: Some(1),
                    offset: None,
                },
                Expected {
                    message: "B".to_owned(),
                    line: Some(2),
                    column: Some(3),
                    markers: Some(2),
                    offset: None,
                },
            ]
        );
        assert_eq!(strip_period("A."), "A");
        assert_eq!(strip_period("A ."), "A .");
    }
}
