//! A sheet rule run on the cases of its test file, as `testRule` of `jest-preset-stylelint` 9.2.0
//! runs a rule's `__tests__/index` of Stylelint 17.16.0 and stylelint-scss 7.3.0
//! (REF-STYLELINT-SOURCE): each file, copied as it is into `tests/fixtures/scss/`, is read by Oxc,
//! each `testRule({ … })` call's object read from its literals and the file's top-level
//! constants. An `accept` case reports nothing; a `reject` case reports one warning, or those of
//! its `warnings`, in order, each with its message and, when given, its line, column, end line and
//! end column; with `fix: true`, a case's code fixed pass by pass, at most 10, gives its `fixed`,
//! or stays as it is when `unfixable`. A case is set apart, counted by its reason, when its test
//! does what this reading cannot: another syntax than SCSS (CSS-in-JS, Less, HTML, Sass's indented
//! syntax), a configuration the catalogue does not set, plugins, reference files or edit
//! information, a case skipped upstream, a value built at run time, or a code `PostCSS`'s CSS parser
//! reads and the SCSS reader refuses.

use crate::javascript::JsText;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use oxc_allocator::Allocator;
use oxc_ast::ast::{
    Argument, ArrayExpressionElement, BinaryOperator, BindingPattern, CallExpression, Expression,
    ForOfStatement, ForStatementLeft, ObjectPropertyKind, Statement, TaggedTemplateExpression,
    UnaryOperator,
};
use oxc_ast_visit::{Visit, walk};
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::{Context, SheetRule, message};
use crate::finding::{Fix, apply};
use crate::lines::Lines;
use crate::scss::parser::parse;

/// A value of a test file, as its literals write it.
#[derive(Clone, Debug, PartialEq)]
pub enum Js {
    /// `undefined`.
    Undefined,
    /// `null`.
    Null,
    /// A boolean.
    Bool(bool),
    /// A number.
    Number(f64),
    /// A string.
    Text(String),
    /// A regular expression, its pattern and its flags.
    Regex(String, String),
    /// An array.
    List(Vec<Self>),
    /// An object, its properties in order.
    Object(Vec<(String, Self)>),
    /// `messages.<id>(…args)` or `messages.<id>`.
    Message(String, Vec<Self>),
}

impl Js {
    /// An object's property, the last written of its name, as JavaScript reads it.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Self> {
        match self {
            Self::Object(properties) => properties
                .iter()
                .rev()
                .find(|(each, _)| each == key)
                .map(|(_, value)| value),
            _ => None,
        }
    }

    /// A string's text.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text(text) => Some(text),
            _ => None,
        }
    }

    /// A property's string, or each string of its array.
    #[must_use]
    pub fn texts(&self, key: &str) -> Option<Vec<String>> {
        match self.get(key)? {
            Self::Text(text) => Some(vec![text.clone()]),
            Self::List(items) => items
                .iter()
                .map(|item| item.text().map(str::to_owned))
                .collect(),
            _ => None,
        }
    }

    /// The value as JavaScript prints it in a template literal.
    #[must_use]
    pub fn display(&self) -> String {
        match self {
            Self::Undefined => "undefined".to_owned(),
            Self::Null => "null".to_owned(),
            Self::Bool(value) => value.to_string(),
            Self::Number(value) => format!("{value}"),
            Self::Text(text) => text.clone(),
            Self::Regex(pattern, flags) => format!("/{pattern}/{flags}"),
            Self::List(items) => items
                .iter()
                .map(Self::display)
                .collect::<Vec<_>>()
                .join(","),
            Self::Object(_) | Self::Message(..) => "[object Object]".to_owned(),
        }
    }
}

/// The names a file declares at its top, with their values.
type Env = Vec<(String, Js)>;

/// The value of a tagged template: `stripIndent` of `common-tags`, or `String.raw` without
/// expressions.
fn tagged_value(tagged: &TaggedTemplateExpression, source: &str, env: &Env) -> Option<Js> {
    if matches!(&tagged.tag, Expression::Identifier(name) if name.name == "stripIndent") {
        let mut text = String::new();
        for (index, quasi) in tagged.quasi.quasis.iter().enumerate() {
            text.push_str(quasi.value.cooked.as_ref()?.as_str());
            if let Some(inner) = tagged.quasi.expressions.get(index) {
                text.push_str(&eval(inner, source, env)?.display());
            }
        }
        return Some(Js::Text(strip_indent(&text)));
    }
    let Expression::StaticMemberExpression(member) = &tagged.tag else {
        return None;
    };
    let raw = matches!(&member.object, Expression::Identifier(name) if name.name == "String")
        && member.property.name == "raw";
    if !raw || !tagged.quasi.expressions.is_empty() {
        return None;
    }
    Some(Js::Text(
        tagged
            .quasi
            .quasis
            .iter()
            .map(|quasi| quasi.value.raw.as_str())
            .collect(),
    ))
}

/// The value an expression writes, `None` when it is built otherwise.
fn eval(expression: &Expression, source: &str, env: &Env) -> Option<Js> {
    match expression {
        Expression::StringLiteral(string) => Some(Js::Text(string.value.to_string())),
        Expression::NumericLiteral(number) => Some(Js::Number(number.value)),
        Expression::BooleanLiteral(boolean) => Some(Js::Bool(boolean.value)),
        Expression::NullLiteral(_) => Some(Js::Null),
        Expression::Identifier(name) if name.name == "undefined" => Some(Js::Undefined),
        Expression::Identifier(name) => env
            .iter()
            .rev()
            .find(|(each, _)| each == name.name.as_str())
            .map(|(_, value)| value.clone()),
        Expression::RegExpLiteral(regex) => {
            let span = regex.span;
            let raw = &source[span.start as usize..span.end as usize];
            let close = raw.rfind('/')?;
            Some(Js::Regex(
                raw[1..close].to_owned(),
                raw[close + 1..].to_owned(),
            ))
        }
        Expression::TemplateLiteral(template) => {
            let mut text = String::new();
            for (index, quasi) in template.quasis.iter().enumerate() {
                text.push_str(quasi.value.cooked.as_ref()?.as_str());
                if let Some(inner) = template.expressions.get(index) {
                    text.push_str(&eval(inner, source, env)?.display());
                }
            }
            Some(Js::Text(text))
        }
        Expression::TaggedTemplateExpression(tagged) => tagged_value(tagged, source, env),
        Expression::ArrayExpression(array) => {
            let mut items = Vec::new();
            for element in &array.elements {
                match element {
                    ArrayExpressionElement::SpreadElement(spread) => {
                        let Js::List(spread) = eval(&spread.argument, source, env)? else {
                            return None;
                        };
                        items.extend(spread);
                    }
                    ArrayExpressionElement::Elision(_) => items.push(Js::Undefined),
                    other => items.push(eval(other.as_expression()?, source, env)?),
                }
            }
            Some(Js::List(items))
        }
        Expression::ObjectExpression(object) => {
            let mut properties = Vec::new();
            for property in &object.properties {
                match property {
                    ObjectPropertyKind::ObjectProperty(property) => {
                        let key = if property.computed {
                            eval(property.key.as_expression()?, source, env)?.display()
                        } else {
                            property.key.static_name()?.to_string()
                        };
                        // An unread value is kept as `undefined`, read again by the case.
                        let value = eval(&property.value, source, env)
                            .unwrap_or(Js::Message("<unresolved>".to_owned(), Vec::new()));
                        properties.push((key, value));
                    }
                    ObjectPropertyKind::SpreadProperty(spread) => {
                        let Js::Object(spread) = eval(&spread.argument, source, env)? else {
                            return None;
                        };
                        properties.extend(spread);
                    }
                }
            }
            Some(Js::Object(properties))
        }
        Expression::BinaryExpression(binary) if binary.operator == BinaryOperator::Addition => {
            match (
                eval(&binary.left, source, env)?,
                eval(&binary.right, source, env)?,
            ) {
                (Js::Number(left), Js::Number(right)) => Some(Js::Number(left + right)),
                (left, right) => Some(Js::Text(format!("{}{}", left.display(), right.display()))),
            }
        }
        Expression::UnaryExpression(unary) if unary.operator == UnaryOperator::UnaryNegation => {
            match eval(&unary.argument, source, env)? {
                Js::Number(number) => Some(Js::Number(-number)),
                _ => None,
            }
        }
        Expression::ParenthesizedExpression(inner) => eval(&inner.expression, source, env),
        Expression::StaticMemberExpression(member) if is_messages(&member.object) => {
            Some(Js::Message(member.property.name.to_string(), Vec::new()))
        }
        Expression::CallExpression(call) => call_value(call, source, env),
        _ => None,
    }
}

/// Two values merged as `deepmerge` merges them for `mergeTestDescriptions`: objects key by key,
/// arrays one after the other, anything else the later.
fn deep_merge(earlier: Js, later: Js) -> Js {
    match (earlier, later) {
        (Js::Object(mut properties), Js::Object(added)) => {
            for (key, value) in added {
                match properties.iter().position(|(each, _)| *each == key) {
                    Some(at) => {
                        let previous = std::mem::replace(&mut properties[at].1, Js::Undefined);
                        properties[at].1 = deep_merge(previous, value);
                    }
                    None => properties.push((key, value)),
                }
            }
            Js::Object(properties)
        }
        (Js::List(mut items), Js::List(added)) => {
            items.extend(added);
            Js::List(items)
        }
        (_, later) => later,
    }
}

/// Whether an expression names the rule's messages: `messages` or `rule.messages`.
fn is_messages(expression: &Expression) -> bool {
    match expression {
        Expression::Identifier(name) => name.name == "messages",
        Expression::StaticMemberExpression(member) => member.property.name == "messages",
        _ => false,
    }
}

/// A text as `stripIndent` of `common-tags` gives it: the smallest indentation of its lines with
/// content taken off each line, then the whole trimmed.
fn strip_indent(text: &str) -> String {
    let indent = text
        .lines()
        .filter(|line| !line.js_trim().is_empty())
        .map(|line| line.len() - line.trim_start_matches([' ', '\t']).len())
        .min()
        .unwrap_or(0);
    text.split('\n')
        .map(|line| {
            let blank = line.len() - line.trim_start_matches([' ', '\t']).len();
            &line[blank.min(indent)..]
        })
        .collect::<Vec<_>>()
        .join("\n")
        .js_trim()
        .to_owned()
}

/// The value of a call: `messages.<id>(…)`, `[…].join(…)`, `'…'.repeat(n)`.
fn call_value(call: &CallExpression, source: &str, env: &Env) -> Option<Js> {
    // A helper of a test file giving its cases a description, `withSyntaxLabel(label, cases)`,
    // read as its cases.
    if matches!(&call.callee, Expression::Identifier(name) if name.name == "withSyntaxLabel") {
        return eval(call.arguments.get(1)?.as_expression()?, source, env);
    }
    if matches!(&call.callee, Expression::Identifier(name) if name.name == "mergeTestDescriptions")
    {
        let mut merged = Js::Object(Vec::new());
        for argument in &call.arguments {
            merged = deep_merge(merged, eval(argument.as_expression()?, source, env)?);
        }
        return Some(merged);
    }
    let Expression::StaticMemberExpression(member) = &call.callee else {
        return None;
    };
    let arguments: Option<Vec<Js>> = call
        .arguments
        .iter()
        .map(|argument| match argument {
            Argument::SpreadElement(_) => None,
            other => eval(other.as_expression()?, source, env),
        })
        .collect();
    let arguments = arguments?;
    if is_messages(&member.object) {
        return Some(Js::Message(member.property.name.to_string(), arguments));
    }
    let object = eval(&member.object, source, env)?;
    match (member.property.name.as_str(), object, arguments.as_slice()) {
        ("join", Js::List(items), [Js::Text(separator)]) => Some(Js::Text(
            items
                .iter()
                .map(Js::display)
                .collect::<Vec<_>>()
                .join(separator),
        )),
        ("repeat", Js::Text(text), [Js::Number(count)]) => {
            Some(Js::Text(text.repeat(format!("{count}").parse().ok()?)))
        }
        ("trim", Js::Text(text), []) => Some(Js::Text(text.js_trim().to_owned())),
        _ => None,
    }
}

/// A `testRule({ … })` call's object, read.
struct Schema {
    /// The object, its unread values marked.
    object: Js,
}

/// The `testRule` calls of a file.
struct Calls<'s> {
    /// The file's text.
    source: &'s str,
    /// Its top-level names.
    env: Env,
    /// Each call's object, `None` when it could not be read.
    schemas: Vec<Option<Schema>>,
}

impl<'a> Visit<'a> for Calls<'_> {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        if matches!(&call.callee, Expression::Identifier(name) if name.name == "testRule") {
            let object = call
                .arguments
                .first()
                .and_then(Argument::as_expression)
                .and_then(|argument| eval(argument, self.source, &self.env));
            self.schemas.push(object.map(|object| Schema { object }));
        }
        walk::walk_call_expression(self, call);
    }

    /// `for (const { a, ...rest } of list) { … }` over a list the file declares: its body read once
    /// for each item, the names bound to the item's properties.
    fn visit_for_of_statement(&mut self, statement: &ForOfStatement<'a>) {
        let items = match &statement.right {
            Expression::Identifier(name) => self
                .env
                .iter()
                .rev()
                .find(|(each, _)| each == name.name.as_str())
                .and_then(|(_, value)| match value {
                    Js::List(items) => Some(items.clone()),
                    _ => None,
                }),
            _ => None,
        };
        let pattern = match &statement.left {
            ForStatementLeft::VariableDeclaration(declaration) => declaration
                .declarations
                .first()
                .map(|declarator| &declarator.id),
            _ => None,
        };
        let (Some(items), Some(BindingPattern::ObjectPattern(names))) = (items, pattern) else {
            walk::walk_for_of_statement(self, statement);
            return;
        };
        for item in items {
            let saved = self.env.len();
            let mut taken = Vec::new();
            for property in &names.properties {
                if let Some(key) = property.key.static_name() {
                    let key = key.to_string();
                    let value = item.get(&key).cloned().unwrap_or(Js::Undefined);
                    let name = property
                        .value
                        .get_identifier_name()
                        .map_or_else(|| key.clone(), |name| name.to_string());
                    self.env.push((name, value));
                    taken.push(key);
                }
            }
            if let Some(rest) = &names.rest
                && let Some(name) = rest.argument.get_identifier_name()
                && let Js::Object(properties) = &item
            {
                let remaining = properties
                    .iter()
                    .filter(|(key, _)| !taken.contains(key))
                    .cloned()
                    .collect();
                self.env.push((name.to_string(), Js::Object(remaining)));
            }
            self.visit_statement(&statement.body);
            self.env.truncate(saved);
        }
    }
}

/// What the cases of a file gave.
#[derive(Debug, Default)]
pub struct Tally {
    /// The accept cases run.
    pub accepted: usize,
    /// The reject cases run.
    pub rejected: usize,
    /// The cases set apart, by reason.
    pub apart: BTreeMap<&'static str, usize>,
    /// Each case set apart: its code and why.
    pub apart_cases: Vec<(String, &'static str)>,
    /// Each case that failed, and why.
    pub failures: Vec<String>,
}

impl Tally {
    /// How many cases were set apart.
    #[must_use]
    pub fn apart_count(&self) -> usize {
        self.apart.values().sum()
    }

    /// Sets a case apart for a reason.
    fn set_apart(&mut self, code: &str, reason: &'static str) {
        *self.apart.entry(reason).or_default() += 1;
        self.apart_cases.push((code.to_owned(), reason));
    }
}

/// A rule built from a test's `config`, or why its configuration is not the catalogue's.
pub type Build<'b> = &'b dyn Fn(&Js) -> Result<Box<dyn SheetRule>, &'static str>;

/// A configuration's primary option: its array's first item, or itself.
#[must_use]
pub fn primary(config: &Js) -> &Js {
    match config {
        Js::List(items) => items.first().unwrap_or(&Js::Undefined),
        other => other,
    }
}

/// A configuration's secondary options: its array's second item.
#[must_use]
pub fn secondary(config: &Js) -> Option<&Js> {
    match config {
        Js::List(items) => items.get(1),
        _ => None,
    }
}

/// Why a whole schema is set apart, when it is.
fn schema_apart(schema: &Js) -> Option<&'static str> {
    if schema
        .get("skip")
        .is_some_and(|skip| *skip == Js::Bool(true))
    {
        return Some("skipped upstream");
    }
    for key in ["plugins", "referenceFiles", "languageOptions"] {
        if schema
            .get(key)
            .is_some_and(|value| *value != Js::Undefined && *value != Js::Bool(false))
        {
            return Some("plugins, reference files or language options");
        }
    }
    match schema.get("customSyntax") {
        None | Some(Js::Undefined) => None,
        Some(Js::Text(syntax)) if syntax == "postcss-scss" => None,
        Some(_) => Some("another syntax than SCSS"),
    }
}

/// The text of a file under `tests/fixtures/scss/`, empty when it cannot be read.
fn fixture(file: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/scss")
        .join(file);
    fs::read_to_string(&path).unwrap_or_default()
}

/// The constants a file's top level declares with a value written in literals, in order.
fn top_level(body: &[Statement], source: &str) -> Env {
    let mut env = Vec::new();
    for statement in body {
        let declarations = match statement {
            Statement::VariableDeclaration(variables) => Some(variables),
            _ => None,
        };
        for declarator in declarations.into_iter().flat_map(|each| &each.declarations) {
            if let (Some(name), Some(init)) =
                (declarator.id.get_identifier_name(), &declarator.init)
                && let Some(value) = eval(init, source, &env)
            {
                env.push((name.to_string(), value));
            }
        }
    }
    env
}

/// The constants a file under `tests/fixtures/scss/` declares at its top, with their values: the
/// data a package exports, read without running it.
#[must_use]
pub fn constants(file: &str) -> Vec<(String, Js)> {
    let source = fixture(file);
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &source, SourceType::mjs()).parse();
    top_level(&parsed.program.body, &source)
}

/// Runs a rule on every case of its test file, under `tests/fixtures/scss/`; the cases whose
/// code holds one of `apart`'s texts are set apart for its reason.
#[must_use]
pub fn run(file: &str, build: Build, apart: &[(&str, &'static str)]) -> Tally {
    let source = fixture(file);
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, &source, SourceType::mjs()).parse();
    let mut calls = Calls {
        source: &source,
        env: top_level(&parsed.program.body, &source),
        schemas: Vec::new(),
    };
    calls.visit_program(&parsed.program);
    let mut tally = Tally::default();
    if calls.schemas.is_empty() {
        tally
            .failures
            .push(format!("{file}: no testRule call read"));
    }
    for schema in calls.schemas {
        let Some(schema) = schema else {
            tally
                .failures
                .push(format!("{file}: a testRule call not read"));
            continue;
        };
        run_schema(&schema.object, build, apart, &mut tally);
    }
    tally
}

/// The cases of a schema, each `accept` then each `reject`.
fn cases(schema: &Js, key: &str) -> Vec<Js> {
    match schema.get(key) {
        Some(Js::List(items)) => items.clone(),
        _ => Vec::new(),
    }
}

/// Runs one `testRule` call's cases.
fn run_schema(schema: &Js, build: Build, apart: &[(&str, &'static str)], tally: &mut Tally) {
    let accept = cases(schema, "accept");
    let reject = cases(schema, "reject");
    let all: Vec<(bool, &Js)> = accept
        .iter()
        .map(|case| (true, case))
        .chain(reject.iter().map(|case| (false, case)))
        .collect();
    let set_all_apart = |tally: &mut Tally, reason: &'static str| {
        for (_, case) in &all {
            let code = case.get("code").map(Js::display).unwrap_or_default();
            tally.set_apart(&code, reason);
        }
    };
    if let Some(reason) = schema_apart(schema) {
        set_all_apart(tally, reason);
        return;
    }
    let config = schema.get("config").cloned().unwrap_or(Js::Undefined);
    let rule = match build(&config) {
        Ok(rule) => rule,
        Err(reason) => {
            set_all_apart(tally, reason);
            return;
        }
    };
    let css_syntax = schema
        .get("customSyntax")
        .is_none_or(|syntax| *syntax == Js::Undefined);
    let fix = schema.get("fix") == Some(&Js::Bool(true));
    for (accepted, case) in all {
        let code = case.get("code").and_then(Js::text).map(str::to_owned);
        let Some(code) = code else {
            tally.set_apart(&case.display(), "a value built at run time");
            continue;
        };
        if case.get("skip") == Some(&Js::Bool(true)) {
            tally.set_apart(&code, "skipped upstream");
            continue;
        }
        if let Some((_, reason)) = apart.iter().find(|(text, _)| code.contains(text)) {
            tally.set_apart(&code, reason);
            continue;
        }
        if code.contains("stylelint-disable") || code.contains("stylelint-enable") {
            tally.set_apart(
                &code,
                "a stylelint-disable comment, written lint-disable-next-line here",
            );
            continue;
        }
        let Some(reports) = lint(rule.as_ref(), &code) else {
            if css_syntax {
                tally.set_apart(
                    &code,
                    "read by PostCSS's CSS parser, refused by the SCSS reader",
                );
            } else {
                tally.failures.push(format!("{code:?}: not read"));
            }
            continue;
        };
        if accepted {
            tally.accepted += 1;
            if !reports.is_empty() {
                tally.failures.push(format!("accept {code:?}: {reports:?}"));
            }
            continue;
        }
        tally.rejected += 1;
        if let Some(failure) = check_reject(rule.as_ref(), &code, case, &reports) {
            tally.failures.push(failure);
            continue;
        }
        if fix && let Some(failure) = check_fix(rule.as_ref(), &code, case) {
            tally.failures.push(failure);
        }
    }
}

/// A finding as a test reads it: its message, first line and column, last line and column.
type Placed = (String, (usize, usize), (usize, usize));

/// A report placed, with its fix and the range of the node its fix belongs to.
#[derive(Debug)]
struct Found {
    /// Its message and place.
    placed: Placed,
    /// Its fix.
    fix: Option<Fix>,
    /// The node its fix rewrites, as `fix.node` gives it to Stylelint.
    scope: Option<(usize, usize)>,
}

/// The line, column and message of each report of a rule on a code, in order, for a test's own
/// cases; none when the code does not read.
#[must_use]
pub fn reports(rule: &dyn SheetRule, code: &str) -> Vec<(usize, usize, String)> {
    lint(rule, code)
        .unwrap_or_default()
        .into_iter()
        .map(|found| (found.placed.1.0, found.placed.1.1, found.placed.0))
        .collect()
}

/// The reports of a rule on a code, placed, in order; `None` when the code does not read.
fn lint(rule: &dyn SheetRule, code: &str) -> Option<Vec<Found>> {
    let sheet = parse(code).ok()?;
    let context = Context {
        path: "a.scss",
        text: code,
        lines: Lines::new(code),
        sheet: &sheet,
    };
    let mut reports = Vec::new();
    rule.check(&context, &mut reports);

    let lines = Lines::new(code);
    // An end past the text is that many columns past its last, as `rangeBy` gives the root's.
    let place = |offset: usize| {
        let (line, column) = lines.place(code, offset.min(code.len()));
        (line, column + offset.saturating_sub(code.len()))
    };
    Some(
        reports
            .into_iter()
            .map(|report| {
                let text = message(rule, report.key, &report.args);
                let start = place(report.start);
                let end = place(report.end);
                Found {
                    placed: (text, start, end),
                    scope: report.scope,
                    fix: report.fix,
                }
            })
            .collect(),
    )
}

/// The message a case expects, from `messages.<id>(…)` filled by the rule's template.
fn expected_message(rule: &dyn SheetRule, value: &Js) -> Option<String> {
    match value {
        Js::Message(key, args) => Some(message(
            rule,
            key,
            &args.iter().map(Js::display).collect::<Vec<_>>(),
        )),
        Js::Text(text) => Some(
            text.strip_suffix(&format!(" ({})", short_name(rule)))
                .unwrap_or(text)
                .to_owned(),
        ),
        _ => None,
    }
}

/// A rule's name as Stylelint writes it, without this linter's `stylelint/` prefix.
fn short_name(rule: &dyn SheetRule) -> &str {
    rule.name()
        .strip_prefix("stylelint/")
        .unwrap_or(rule.name())
}

/// A number a case expects.
fn number(case: &Js, key: &str) -> Option<usize> {
    match case.get(key) {
        Some(Js::Number(number)) => format!("{number}").parse().ok(),
        _ => None,
    }
}

/// Why a reject case's reports differ from its warnings, `None` when they match.
fn check_reject(rule: &dyn SheetRule, code: &str, case: &Js, reports: &[Found]) -> Option<String> {
    let warnings = match case.get("warnings") {
        Some(Js::List(warnings)) => warnings.clone(),
        _ => vec![case.clone()],
    };
    if warnings.len() != reports.len() {
        let found: Vec<&Placed> = reports.iter().map(|report| &report.placed).collect();
        return Some(format!(
            "reject {code:?}: {} warnings expected, found {found:?}",
            warnings.len()
        ));
    }
    // Stylelint records the edit of the first fix of each node, none for a fix whose node
    // overlaps an edit recorded (`computeEditInfo`).
    let mut edited: Vec<(usize, usize)> = Vec::new();
    for (warning, report) in warnings.iter().zip(reports) {
        let (text, start, end) = &report.placed;
        let edit = report.fix.as_ref().and_then(|fix| {
            let scope = report
                .scope
                .unwrap_or((fix.start as usize, fix.end as usize));
            if edited
                .iter()
                .any(|range| range.0 < scope.1 && scope.0 < range.1)
            {
                return None;
            }
            let info = edit_info(code, fix, scope);
            if let Some(Js::List(range)) = info.get("range")
                && let [Js::Number(from), Js::Number(to)] = range.as_slice()
            {
                edited.push((
                    format!("{from}").parse().unwrap_or(0),
                    format!("{to}").parse().unwrap_or(0),
                ));
            }
            Some(info)
        });
        if let Some(wanted) = warning.get("fix") {
            let found = edit.unwrap_or(Js::Undefined);
            if !same_edit(wanted, &found) {
                return Some(format!(
                    "reject {code:?}: edit {found:?}, expected {wanted:?}"
                ));
            }
        }
        let expected = warning
            .get("message")
            .and_then(|message| expected_message(rule, message));
        if expected.as_deref() != Some(text.as_str()) {
            return Some(format!(
                "reject {code:?}: message {text:?}, expected {expected:?}"
            ));
        }
        let places = [
            ("line", start.0),
            ("column", start.1),
            ("endLine", end.0),
            ("endColumn", end.1),
        ];
        for (key, found) in places {
            if let Some(wanted) = number(warning, key)
                && wanted != found
            {
                return Some(format!(
                    "reject {code:?}: {key} {found}, expected {wanted} ({text})"
                ));
            }
        }
    }
    None
}

/// A fix as Stylelint's edit information gives it, narrowed to the characters it changes as
/// `narrowFixRange` narrows it, at least one kept as context: `{ range: [start, end], text }`.
fn edit_info(code: &str, fix: &Fix, scope: (usize, usize)) -> Js {
    // The node's text before and after the fix, as Stylelint prints the node fixed.
    let (start, end) = (
        scope.0.min(fix.start as usize),
        scope.1.max(fix.end as usize),
    );
    let original: Vec<char> = code.get(start..end).unwrap_or_default().chars().collect();
    let replaced = format!(
        "{}{}{}",
        code.get(start..fix.start as usize).unwrap_or_default(),
        fix.text,
        code.get(fix.end as usize..end).unwrap_or_default()
    );
    let text: Vec<char> = replaced.chars().collect();
    let mut first = 0;
    while first < original.len() && first < text.len() && original[first] == text[first] {
        first += 1;
    }
    let mut last = 0;
    while last < original.len() - first
        && last < text.len() - first
        && original[original.len() - 1 - last] == text[text.len() - 1 - last]
    {
        last += 1;
    }
    let mut range = (start + first, end - last);
    let mut kept = (first, text.len() - last);
    if range.0 == range.1 {
        if range.0 > start {
            range.0 -= 1;
            kept.0 -= 1;
        } else if range.1 < end {
            range.1 += 1;
            kept.1 += 1;
        }
    }
    // Offsets in UTF-16 units, as JavaScript counts a string's indexes.
    let number = |value: usize| {
        let units = code
            .get(..value)
            .map_or(value, |before| before.encode_utf16().count());
        Js::Number(f64::from(u32::try_from(units).unwrap_or(0)))
    };
    Js::Object(vec![
        (
            "range".to_owned(),
            Js::List(vec![number(range.0), number(range.1)]),
        ),
        (
            "text".to_owned(),
            Js::Text(text[kept.0..kept.1].iter().collect()),
        ),
    ])
}

/// Whether an edit information found matches the one a case expects.
fn same_edit(wanted: &Js, found: &Js) -> bool {
    match (wanted, found) {
        (Js::Undefined, Js::Undefined) => true,
        // `toMatchObject`: only the keys the case gives.
        (Js::Object(properties), Js::Object(_)) => properties
            .iter()
            .all(|(key, value)| found.get(key) == Some(value)),
        _ => false,
    }
}

/// The code with the rule's fixes applied pass by pass, at most 10.
fn fixed(rule: &dyn SheetRule, code: &str) -> String {
    let mut text = code.to_owned();
    for _ in 0..crate::FIX_PASSES {
        let Some(reports) = lint(rule, &text) else {
            break;
        };
        let fixes: Vec<&Fix> = reports
            .iter()
            .filter_map(|report| report.fix.as_ref())
            .collect();
        if fixes.is_empty() {
            break;
        }
        let next = apply(&text, &fixes);
        if next == text {
            break;
        }
        text = next;
    }
    text
}

/// Why a reject case's fix differs from its `fixed`, `None` when it matches.
fn check_fix(rule: &dyn SheetRule, code: &str, case: &Js) -> Option<String> {
    let output = fixed(rule, code);
    let unfixable = case.get("unfixable") == Some(&Js::Bool(true));
    let wanted = case.get("fixed").and_then(Js::text);
    if unfixable {
        return (output != code).then(|| format!("unfixable {code:?} fixed into {output:?}"));
    }
    match wanted {
        Some(wanted) if wanted == output && output != code => None,
        // A code left as it is: a fix the rule refuses, told apart for a rule that refuses some.
        Some(wanted) if output == code => Some(format!("unfixed {code:?}, expected {wanted:?}")),
        Some(wanted) => Some(format!("fix {code:?}: {output:?}, expected {wanted:?}")),
        None => Some(format!("fix {code:?}: no fixed given")),
    }
}

/// The matchers of a secondary option's property: its strings and regular expressions; none when
/// absent.
///
/// # Errors
///
/// A value that is neither, or a regular expression Rust's engine cannot read.
pub fn matchers(secondary: Option<&Js>, key: &str) -> Result<Vec<super::Matcher>, &'static str> {
    let Some(value) = secondary.and_then(|secondary| secondary.get(key)) else {
        return Ok(Vec::new());
    };
    let items = match value {
        Js::List(items) => items.clone(),
        other => vec![other.clone()],
    };
    items
        .iter()
        .map(|item| match item {
            Js::Text(text) => Ok(super::Matcher::new(text)),
            Js::Regex(pattern, flags) => {
                super::Matcher::regex(pattern, flags).ok_or("a regular expression Rust cannot read")
            }
            _ => Err("an option value not read"),
        })
        .collect()
}

/// The pattern a configuration's primary option gives: a string, or a regular expression shown
/// as JavaScript prints it.
///
/// # Errors
///
/// Neither, or a regular expression Rust's engine cannot read.
pub fn pattern(config: &Js) -> Result<super::Pattern, &'static str> {
    match primary(config) {
        Js::Text(text) => Ok(super::Pattern::new(text)),
        Js::Regex(source, flags) => {
            let prefix = if flags.contains('i') { "(?i)" } else { "" };
            let regex = regex::Regex::new(&format!("{prefix}{source}"))
                .map_err(|_| "a regular expression Rust cannot read")?;
            Ok(super::Pattern {
                regex: Some(regex),
                text: format!("/{source}/{flags}"),
            })
        }
        _ => Err("an option value not read"),
    }
}

/// The options of an empty-line rule: its primary and its `ignore`, `except`, `ignoreAtRules`
/// and `ignoreComments`.
///
/// # Errors
///
/// A primary not a string, or a matcher not read.
pub fn empty_lines(config: &Js) -> Result<super::empty_lines::Lines, &'static str> {
    let Js::Text(primary) = primary(config) else {
        return Err("an option value not read");
    };
    let options = secondary(config);
    let words = |key: &str| options.and_then(|each| each.texts(key)).unwrap_or_default();
    Ok(super::empty_lines::Lines {
        primary: primary.clone(),
        ignore: words("ignore"),
        except: words("except"),
        ignore_at_rules: matchers(options, "ignoreAtRules")?,
        ignore_comments: matchers(options, "ignoreComments")?,
    })
}

#[cfg(test)]
mod tests {
    use super::{Js, eval};
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    /// The value of the expression a statement `x = …;` writes.
    fn value_of(source: &str) -> Option<Js> {
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, source, SourceType::mjs()).parse();
        match parsed.program.body.first() {
            Some(Statement::ExpressionStatement(statement)) => eval(
                &statement.expression,
                source,
                &vec![("a".to_owned(), Js::Text("x".to_owned()))],
            ),
            _ => None,
        }
    }

    #[test]
    fn reads_the_literals_a_test_file_writes() {
        assert_eq!(
            value_of("'a' + `b${a}` + 1;"),
            Some(Js::Text("abx1".to_owned()))
        );
        assert_eq!(
            value_of("/^a$/i;"),
            Some(Js::Regex("^a$".to_owned(), "i".to_owned()))
        );
        assert_eq!(
            value_of("messages.rejected('#fff', 2);"),
            Some(Js::Message(
                "rejected".to_owned(),
                vec![Js::Text("#fff".to_owned()), Js::Number(2.0)]
            ))
        );
        assert_eq!(
            value_of("['a', 'b'].join(', ');"),
            Some(Js::Text("a, b".to_owned()))
        );
        assert_eq!(
            value_of("'ab'.repeat(2);"),
            Some(Js::Text("abab".to_owned()))
        );
        assert_eq!(
            value_of("String.raw`a\\b`;"),
            Some(Js::Text("a\\b".to_owned()))
        );
        assert_eq!(
            value_of("[...[1], , -2];"),
            Some(Js::List(vec![
                Js::Number(1.0),
                Js::Undefined,
                Js::Number(-2.0)
            ]))
        );
        assert_eq!(
            value_of("({ ...{ b: 1 }, c: undefined });").map(|value| value.get("b").cloned()),
            Some(Some(Js::Number(1.0)))
        );
        assert_eq!(value_of("f();"), None);
        assert_eq!(
            Js::List(vec![Js::Null, Js::Bool(true)]).display(),
            "null,true"
        );
    }
}
