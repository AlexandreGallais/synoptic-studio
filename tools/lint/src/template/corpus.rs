//! The corpus the reader is held to (F52.AC1): every template of Angular's control-flow guide
//! (`adev/src/content/guide/templates/control-flow.md` of Angular 22.2.3, REF-ANGULAR-CONTROL-FLOW),
//! and every template of the template rules' tests of `@angular-eslint/eslint-plugin-template`
//! 22.6.0, whose 42 `tests/rules/*/cases.ts` files are copied as they are into
//! `tests/fixtures/templates/angular-eslint/` (REF-ANGULAR-ESLINT-TEMPLATE-TESTS) and read here by
//! Oxc: each string of `valid`, each `code`, each `annotatedSource` cleaned as
//! `convertAnnotatedSourceToFailureCase` of `@angular-eslint/test-utils` 22.6.0 cleans it (its
//! `parseInvalidSource`: the other messages' marks made spaces, the last message's removed), each
//! `output`, `annotatedOutput`, `annotatedOutputs` and suggestion's `output`. The cases a file
//! builds by `.map(…)` or `.flatMap(…)` over a list written there, declared at its top or itself
//! mapped, are built here the same way, each `${name}` given its value, as are
//! `${text.repeat(n)}` and `${text.slice(n)}`, `n` a number or a sum of `name.length`s; a list
//! wrapped in a `.map(…)` that gives each case a default is read as its array. A case whose
//! `filename` is a TypeScript file holds a component, not a template, and is counted apart, as is
//! any other `${…}` and any other element, which only running the test file would build: none
//! is, at this version.

use std::fs;
use std::path::Path;

use oxc_allocator::Allocator;
use oxc_ast::ast::{
    ArrayExpression, ArrayExpressionElement, BindingPattern, Declaration, Expression,
    ObjectExpression, ObjectPropertyKind, Statement,
};
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::bindings::check;
use super::tree::parse;

/// What a test file gives: its templates and the cases left apart.
#[derive(Debug, Default, PartialEq, Eq)]
struct Corpus {
    /// Each template, with where it comes from.
    templates: Vec<(String, String)>,
    /// The cases whose `filename` is a TypeScript file.
    typescript: usize,
    /// The template literals with a `${…}` not built here.
    dynamic: usize,
    /// The array elements neither a string, an object, a call of the helper nor a mapped array.
    other: usize,
}

/// The values of the names a mapped case reads.
type Values = [(String, String)];

/// A string literal's or a template literal's value, its `${name}` and `${'text'.repeat(n)}`
/// built; `Err(())` for any other `${…}`, `None` for anything else.
fn literal(expression: &Expression, values: &Values) -> Option<Result<String, ()>> {
    match expression {
        Expression::StringLiteral(string) => Some(Ok(string.value.to_string())),
        Expression::TemplateLiteral(template) => {
            let mut text = String::new();
            for (index, quasi) in template.quasis.iter().enumerate() {
                text.push_str(
                    quasi
                        .value
                        .cooked
                        .as_ref()
                        .map_or("", |cooked| cooked.as_str()),
                );
                if let Some(inner) = template.expressions.get(index) {
                    match substitution(inner, values) {
                        Some(value) => text.push_str(&value),
                        None => return Some(Err(())),
                    }
                }
            }
            Some(Ok(text))
        }
        Expression::TSAsExpression(cast) => literal(&cast.expression, values),
        _ => None,
    }
}

/// The value of a `${…}`: a name given a value, `text.repeat(n)` or `text.slice(n)`.
fn substitution(expression: &Expression, values: &Values) -> Option<String> {
    match expression {
        Expression::Identifier(name) => values
            .iter()
            .find(|(each, _)| each == name.name.as_str())
            .map(|(_, value)| value.clone()),
        Expression::CallExpression(call) => {
            let Expression::StaticMemberExpression(member) = &call.callee else {
                return None;
            };
            let text = text(&member.object, values)?;
            let count = call
                .arguments
                .first()
                .and_then(|argument| argument.as_expression())
                .and_then(|count| number(count, values))?;
            match member.property.name.as_str() {
                "repeat" => Some(text.repeat(count)),
                "slice" => Some(text.chars().skip(count).collect()),
                _ => None,
            }
        }
        _ => None,
    }
}

/// The value of a count: a number, a `name.length`, a sum or a difference of them.
fn number(expression: &Expression, values: &Values) -> Option<usize> {
    match expression {
        Expression::NumericLiteral(number) => format!("{}", number.value).parse().ok(),
        Expression::StaticMemberExpression(member) if member.property.name == "length" => {
            substitution(&member.object, values).map(|text| text.chars().count())
        }
        Expression::BinaryExpression(binary) => {
            let (left, right) = (
                number(&binary.left, values)?,
                number(&binary.right, values)?,
            );
            match binary.operator.as_str() {
                "+" => Some(left + right),
                "-" => left.checked_sub(right),
                _ => None,
            }
        }
        _ => None,
    }
}

/// An object's property by its name.
fn property<'o, 'a>(object: &'o ObjectExpression<'a>, name: &str) -> Option<&'o Expression<'a>> {
    object.properties.iter().find_map(|each| match each {
        ObjectPropertyKind::ObjectProperty(property)
            if property.key.static_name().is_some_and(|key| key == name) =>
        {
            Some(&property.value)
        }
        _ => None,
    })
}

/// The objects of an array property.
fn objects<'o, 'a>(object: &'o ObjectExpression<'a>, name: &str) -> Vec<&'o ObjectExpression<'a>> {
    let Some(Expression::ArrayExpression(array)) = property(object, name) else {
        return Vec::new();
    };
    array
        .elements
        .iter()
        .filter_map(|element| match element.as_expression() {
            Some(Expression::ObjectExpression(each)) => Some(&**each),
            _ => None,
        })
        .collect()
}

/// A case's suggestions: its own, its messages' and its errors'.
fn suggestions<'o, 'a>(object: &'o ObjectExpression<'a>) -> Vec<&'o ObjectExpression<'a>> {
    let mut found = objects(object, "suggestions");
    for holder in objects(object, "messages")
        .into_iter()
        .chain(objects(object, "errors"))
    {
        found.extend(objects(holder, "suggestions"));
    }
    found
}

/// An annotated source cleaned for its last mark, the others made spaces (`parseInvalidSource`).
fn cleaned(source: &str, marks: &[char]) -> String {
    let Some((last, others)) = marks.split_last() else {
        return source.to_owned();
    };
    source
        .chars()
        .filter(|each| each != last)
        .map(|each| if others.contains(&each) { ' ' } else { each })
        .collect()
}

/// The marks of a failure case's messages: `~` for one `messageId`, else each `char`.
fn marks(object: &ObjectExpression) -> Vec<char> {
    if property(object, "messageId").is_some() {
        return vec!['~'];
    }
    objects(object, "messages")
        .into_iter()
        .filter_map(|message| property(message, "char"))
        .filter_map(|mark| literal(mark, &[]).and_then(Result::ok))
        .filter_map(|mark| mark.chars().next())
        .collect()
}

/// Adds the templates of a case's object to the corpus.
fn add_case(
    corpus: &mut Corpus,
    origin: &str,
    object: &ObjectExpression,
    annotated: bool,
    values: &Values,
) {
    if property(object, "filename")
        .and_then(|filename| literal(filename, values))
        .and_then(Result::ok)
        .is_some_and(|filename| {
            Path::new(&filename)
                .extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("ts"))
        })
    {
        corpus.typescript += 1;
        return;
    }
    let mut add = |name: &str, value: Option<&Expression>, clean: &dyn Fn(&str) -> String| {
        let Some(value) = value else {
            return;
        };
        let each_value: Vec<&Expression> = match value {
            Expression::ArrayExpression(array) => array
                .elements
                .iter()
                .filter_map(ArrayExpressionElement::as_expression)
                .collect(),
            single => vec![single],
        };
        for each in each_value {
            match literal(each, values) {
                Some(Ok(text)) => corpus
                    .templates
                    .push((format!("{origin} {name}"), clean(&text))),
                Some(Err(())) => corpus.dynamic += 1,
                None => {}
            }
        }
    };
    if annotated {
        let marks = marks(object);
        add(
            "annotatedSource",
            property(object, "annotatedSource"),
            &|text| cleaned(text, &marks),
        );
        add(
            "annotatedOutput",
            property(object, "annotatedOutput"),
            &|text| cleaned(text, &['~']),
        );
        add(
            "annotatedOutputs",
            property(object, "annotatedOutputs"),
            &|text| cleaned(text, &['~']),
        );
    } else {
        add("code", property(object, "code"), &str::to_owned);
        add("output", property(object, "output"), &str::to_owned);
    }
    for suggestion in suggestions(object) {
        add("suggestion", property(suggestion, "output"), &str::to_owned);
    }
}

/// Adds the templates of an element of a `valid` or `invalid` array to the corpus.
fn add_element(corpus: &mut Corpus, origin: &str, expression: &Expression, values: &Values) {
    match expression {
        Expression::ParenthesizedExpression(inner) => {
            add_element(corpus, origin, &inner.expression, values);
        }
        Expression::ObjectExpression(object) => add_case(corpus, origin, object, false, values),
        Expression::CallExpression(call)
            if call
                .callee
                .get_identifier_reference()
                .is_some_and(|callee| callee.name == "convertAnnotatedSourceToFailureCase") =>
        {
            match call
                .arguments
                .first()
                .and_then(|argument| argument.as_expression())
            {
                Some(Expression::ObjectExpression(object)) => {
                    add_case(corpus, origin, object, true, values);
                }
                _ => corpus.other += 1,
            }
        }
        Expression::ArrayExpression(array) => {
            for (index, element) in array.elements.iter().enumerate() {
                match element.as_expression() {
                    Some(each) => add_element(corpus, &format!("{origin}[{index}]"), each, values),
                    None => corpus.other += 1,
                }
            }
        }
        other => match literal(other, values) {
            Some(Ok(text)) => corpus.templates.push((origin.to_owned(), text)),
            Some(Err(())) => corpus.dynamic += 1,
            None => corpus.other += 1,
        },
    }
}

/// The arrays a test file declares at its top, by name (`const VARIABLE_NAMES = [...]`).
type Arrays<'r, 'a> = [(String, &'r ArrayExpression<'a>)];

/// A value a mapped case is built from.
enum Input {
    /// A string.
    Text(String),
    /// An array of strings.
    List(Vec<String>),
    /// An object of strings, by property.
    Object(Vec<(String, String)>),
}

/// The text of an expression: a literal, a name given a value, `'text'.repeat(n)` or
/// `text.slice(n)`.
fn text(expression: &Expression, values: &Values) -> Option<String> {
    literal(expression, values)
        .and_then(Result::ok)
        .or_else(|| substitution(expression, values))
}

/// The input an expression gives, its names given their values.
fn input(expression: &Expression, values: &Values) -> Option<Input> {
    match expression {
        Expression::ArrayExpression(array) => array
            .elements
            .iter()
            .map(|element| element.as_expression().and_then(|each| text(each, values)))
            .collect::<Option<_>>()
            .map(Input::List),
        Expression::ObjectExpression(object) => object
            .properties
            .iter()
            .map(|each| match each {
                ObjectPropertyKind::ObjectProperty(property) => Some((
                    property.key.static_name()?.to_string(),
                    text(&property.value, values)?,
                )),
                ObjectPropertyKind::SpreadProperty(_) => None,
            })
            .collect::<Option<_>>()
            .map(Input::Object),
        other => text(other, values).map(Input::Text),
    }
}

/// The inputs a list gives: an array written there or declared at the top, or the result of
/// a `.map(…)` or `.flatMap(…)` on one.
fn inputs(expression: &Expression, arrays: &Arrays) -> Option<Vec<Input>> {
    let array = match expression {
        Expression::ArrayExpression(array) => &**array,
        Expression::Identifier(name) => arrays
            .iter()
            .find(|(each, _)| each == name.name.as_str())
            .map(|(_, array)| *array)?,
        Expression::CallExpression(_) => {
            let (object, pattern, body, flat) = mapping(expression)?;
            let mut found = Vec::new();
            for each in inputs(object, arrays)? {
                let values = bound_values(pattern, &each)?;
                match (flat, body) {
                    (true, Expression::ArrayExpression(results)) => {
                        for result in &results.elements {
                            found.push(input(result.as_expression()?, &values)?);
                        }
                    }
                    (false, result) => found.push(input(result, &values)?),
                    _ => return None,
                }
            }
            return Some(found);
        }
        _ => return None,
    };
    array
        .elements
        .iter()
        .map(|element| element.as_expression().and_then(|each| input(each, &[])))
        .collect()
}

/// A `.map(…)` or `.flatMap(…)` call: the list it is called on, the arrow's parameter, the
/// arrow's body, whether flat.
fn mapping<'r, 'a>(
    expression: &'r Expression<'a>,
) -> Option<(
    &'r Expression<'a>,
    &'r BindingPattern<'a>,
    &'r Expression<'a>,
    bool,
)> {
    let Expression::CallExpression(call) = expression else {
        return None;
    };
    let Expression::StaticMemberExpression(member) = &call.callee else {
        return None;
    };
    let flat = match member.property.name.as_str() {
        "map" => false,
        "flatMap" => true,
        _ => return None,
    };
    let Some(Expression::ArrowFunctionExpression(arrow)) = call
        .arguments
        .first()
        .and_then(|argument| argument.as_expression())
    else {
        return None;
    };
    let pattern = &arrow.params.items.first()?.pattern;
    Some((&member.object, pattern, arrow.body.as_expression()?, flat))
}

/// Adds the cases of a `...list.map(…)` or `.flatMap(…)`, each built with its values; whether
/// the spread is one.
fn add_mapped(corpus: &mut Corpus, origin: &str, expression: &Expression, arrays: &Arrays) -> bool {
    let Some((object, pattern, body, _)) = mapping(expression) else {
        return false;
    };
    let Some(each_input) = inputs(object, arrays) else {
        return false;
    };
    for (index, each) in each_input.iter().enumerate() {
        let Some(values) = bound_values(pattern, each) else {
            return false;
        };
        add_element(corpus, &format!("{origin}({index})"), body, &values);
    }
    true
}

/// The values a parameter takes from an input: a name its text, `[a, b]` a list's texts,
/// `{ a, b }` an object's texts.
fn bound_values(pattern: &BindingPattern, input: &Input) -> Option<Vec<(String, String)>> {
    match (pattern, input) {
        (BindingPattern::BindingIdentifier(name), Input::Text(value)) => {
            Some(vec![(name.name.to_string(), value.clone())])
        }
        (BindingPattern::ArrayPattern(names), Input::List(list)) => names
            .elements
            .iter()
            .zip(list)
            .map(|(name, value)| match name {
                Some(BindingPattern::BindingIdentifier(name)) => {
                    Some((name.name.to_string(), value.clone()))
                }
                _ => None,
            })
            .collect(),
        (BindingPattern::ObjectPattern(names), Input::Object(object)) => names
            .properties
            .iter()
            .map(|name| {
                let key = name.key.static_name()?.to_string();
                let value = object.iter().find(|(each, _)| *each == key)?.1.clone();
                Some((key, value))
            })
            .collect(),
        _ => None,
    }
}

/// Adds the templates of a `valid` or `invalid` array to the corpus.
fn add_cases(corpus: &mut Corpus, name: &str, array: &ArrayExpression, arrays: &Arrays) {
    for (index, element) in array.elements.iter().enumerate() {
        let origin = format!("{name}[{index}]");
        match element {
            ArrayExpressionElement::SpreadElement(spread) => {
                if !add_mapped(corpus, &origin, &spread.argument, arrays) {
                    corpus.other += 1;
                }
            }
            each => match each.as_expression() {
                Some(expression) => add_element(corpus, &origin, expression, &[]),
                None => corpus.other += 1,
            },
        }
    }
}

/// The array of cases an initialiser holds: the array itself, or the array a `.map(…)` that
/// gives each case a default (`[…].map((test) => ({ ...test, filename }))`) is called on.
fn cases_array<'r, 'a>(init: &'r Expression<'a>) -> Option<&'r ArrayExpression<'a>> {
    match init {
        Expression::ArrayExpression(array) => Some(array),
        Expression::CallExpression(call) => match &call.callee {
            Expression::StaticMemberExpression(member) if member.property.name == "map" => {
                match &member.object {
                    Expression::ArrayExpression(array) => Some(array),
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    }
}

/// The corpus of a test file: its exported `valid` and `invalid` arrays.
fn read_cases(text: &str) -> Corpus {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, text, SourceType::ts()).parse();
    let mut corpus = Corpus::default();
    let mut arrays: Vec<(String, &ArrayExpression)> = Vec::new();
    let mut exported = Vec::new();
    for statement in &parsed.program.body {
        let (variables, is_exported) = match statement {
            Statement::ExportDeclaration(export) => match &export.declaration {
                Declaration::VariableDeclaration(variables) => (variables, true),
                _ => continue,
            },
            Statement::VariableDeclaration(variables) => (variables, false),
            _ => continue,
        };
        for declarator in &variables.declarations {
            let Some(name) = declarator.id.get_identifier_name() else {
                continue;
            };
            let is_cases = is_exported && (name == "valid" || name == "invalid");
            match declarator.init.as_ref().and_then(cases_array) {
                Some(array) if is_cases => exported.push((name.to_string(), array)),
                Some(array) => arrays.push((name.to_string(), array)),
                // A list of cases this reading cannot open is counted, never passed over.
                None if is_cases => corpus.other += 1,
                None => {}
            }
        }
    }
    for (name, array) in exported {
        add_cases(&mut corpus, &name, array, &arrays);
    }
    corpus
}

/// The templates of the control-flow guide's fixture, each after its `@@@ template` line.
fn guide_templates(text: &str) -> Vec<(String, String)> {
    text.split("@@@ template ")
        .skip(1)
        .map(|part| {
            let (head, body) = part.split_once('\n').unwrap_or((part, ""));
            (head.to_owned(), body.to_owned())
        })
        .collect()
}

/// The errors of each template that does not read cleanly.
fn failures(file: &str, templates: &[(String, String)]) -> Vec<String> {
    templates
        .iter()
        .filter_map(|(origin, template)| {
            let errors = check(&parse(template));
            (!errors.is_empty()).then(|| {
                let messages: Vec<String> = errors.into_iter().map(|error| error.message).collect();
                format!("{file} {origin}: {}\n{template}", messages.join(" | "))
            })
        })
        .collect()
}

/// The fixtures' folder.
fn fixtures() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/templates")
}

#[test]
fn f52_ac1_reads_every_template_of_the_control_flow_guide() {
    let text = fs::read_to_string(fixtures().join("control-flow.txt")).unwrap_or_default();
    let templates = guide_templates(&text);

    assert_eq!(templates.len(), 10);
    assert_eq!(
        failures("control-flow.txt", &templates),
        Vec::<String>::new()
    );
}

#[test]
fn f52_ac1_reads_every_template_of_the_angular_eslint_template_rules_tests() {
    let folder = fixtures().join("angular-eslint");
    let mut files: Vec<String> = fs::read_dir(&folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    let mut counts = Vec::new();
    let mut failed = Vec::new();
    for file in &files {
        let corpus = read_cases(&fs::read_to_string(folder.join(file)).unwrap_or_default());
        failed.extend(failures(file, &corpus.templates));
        counts.push(format!(
            "{} {} {} {} {}",
            file.trim_end_matches(".txt"),
            corpus.templates.len(),
            corpus.typescript,
            corpus.dynamic,
            corpus.other
        ));
    }

    assert_eq!(failed, Vec::<String>::new());
    // Each file: its templates read, its TypeScript cases, its literals left dynamic, its other
    // elements; 1,669 templates in the 42 files of the plugin's `tests/rules/`, none left out.
    assert_eq!(
        counts,
        [
            "alt-text 31 0 0 0",
            "attributes-order 81 0 0 0",
            "banana-in-box 7 0 0 0",
            "button-has-type 23 0 0 0",
            "click-events-have-key-events 29 0 0 0",
            "conditional-complexity 23 0 0 0",
            "cyclomatic-complexity 13 0 0 0",
            "elements-content 8 0 0 0",
            "eqeqeq 33 0 0 0",
            "i18n 104 0 0 0",
            "interactive-supports-focus 34 0 0 0",
            "label-has-associated-control 11 0 0 0",
            "mouse-events-have-key-events 6 0 0 0",
            "no-any 30 0 0 0",
            "no-autofocus 18 0 0 0",
            "no-call-expression 27 0 0 0",
            "no-distracting-elements 7 0 0 0",
            "no-duplicate-attributes 58 0 0 0",
            "no-empty-control-flow 25 0 0 0",
            "no-inline-styles 69 0 0 0",
            "no-interpolation-in-attributes 14 0 0 0",
            "no-negated-async 24 0 0 0",
            "no-nested-tags 13 0 0 0",
            "no-non-null-assertion 81 0 0 0",
            "no-outerhtml 8 0 0 0",
            "no-positive-tabindex 20 0 0 0",
            "prefer-at-else 74 0 0 0",
            "prefer-at-empty 83 0 0 0",
            "prefer-built-in-pipes 21 0 0 0",
            "prefer-class-binding 17 0 0 0",
            "prefer-contextual-for-variables 230 0 0 0",
            "prefer-control-flow 15 0 0 0",
            "prefer-ngsrc 62 0 0 0",
            "prefer-self-closing-tags 50 0 0 0",
            "prefer-static-string-properties 32 0 0 0",
            "prefer-style-binding 24 0 0 0",
            "prefer-template-literal 181 0 0 0",
            "require-switch-default 9 0 0 0",
            "role-has-required-aria 11 0 0 0",
            "table-scope 14 0 0 0",
            "use-track-by-function 18 0 0 0",
            "valid-aria 31 0 0 0",
        ]
    );
}

#[test]
fn cleans_an_annotated_source_and_builds_mapped_cases_as_the_tests_do() {
    assert_eq!(cleaned("<a>\n~~~\n", &['~']), "<a>\n\n");
    assert_eq!(cleaned("<a #b>\n^^ ~~\n", &['^', '~']), "<a #b>\n   \n");
    let corpus = read_cases(
        "export const valid = ['<a>', `<b>${x}</b>`, { code: '<c>' }, { code: 'x', filename: 'a.ts' }, ...more, `${'<i>'.repeat(2)}`];\nexport const invalid = [convertAnnotatedSourceToFailureCase({ messages: [{ char: '~' }, { char: '^' }], annotatedSource: `<d #e>\\n~~ ^^\\n`, annotatedOutputs: ['<f>'] }), ...[['g', 'h']].flatMap(([one, two]) => [convertAnnotatedSourceToFailureCase({ messageId, annotatedSource: `<${one}>~` }), { code: `<${two}>` }])];",
    );
    assert_eq!(
        corpus.templates,
        [
            ("valid[0]".to_owned(), "<a>".to_owned()),
            ("valid[2] code".to_owned(), "<c>".to_owned()),
            ("valid[5]".to_owned(), "<i><i>".to_owned()),
            (
                "invalid[0] annotatedSource".to_owned(),
                "<d #e>\n   \n".to_owned()
            ),
            ("invalid[0] annotatedOutputs".to_owned(), "<f>".to_owned()),
            (
                "invalid[1](0)[0] annotatedSource".to_owned(),
                "<g>".to_owned()
            ),
            ("invalid[1](0)[1] code".to_owned(), "<h>".to_owned()),
        ]
    );
    assert_eq!((corpus.typescript, corpus.dynamic, corpus.other), (1, 1, 1));
}
