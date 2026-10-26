//! The expressions of a template, parsed by Oxc once their pipes are split off. Angular's
//! expression syntax is JavaScript's with pipes (`value | name: argument`), the lowest operator,
//! which may also sit inside brackets (`(items | async)?.length`); Angular has no bitwise `|`, so
//! every single `|` outside a string or a regular expression is a pipe. A text's top-level pipes
//! are split off, each pipe kept as a node with its arguments (split at its `:`s, a ternary's
//! `:` kept); a bracketed element holding pipes of its own is read the same way, as a nested
//! expression, and replaced by underscores of the same length in the text handed to Oxc, so that
//! every offset Oxc gives stays the text's. This splitting is the project's own, after the
//! grammar of `parsePipe` in `expression_parser/parser.ts` of `@angular/compiler` 22.2.3
//! (REF-ANGULAR-ML-PARSER); the tests are this module's own cases.

use oxc_allocator::Allocator;
use oxc_ast::ast::Expression;
use oxc_parser::Parser;
use oxc_span::SourceType;

use super::lexer::{ParseError, Span, offset};

/// An expression with its pipes, as a binding or an interpolation writes it.
#[derive(Debug)]
pub struct Bound<'a> {
    /// The whole text in the template.
    pub span: Span,
    /// The part before the first pipe, parsed; `None` when blank.
    pub expression: Option<Expression<'a>>,
    /// The offset in the template of the text Oxc read: an Oxc offset plus this one. Exact unless
    /// the value, read decoded, writes an entity (`[x]="a &amp;&amp; b"`).
    pub start: u32,
    /// Its pipes, in order: the first applied first.
    pub pipes: Vec<Pipe<'a>>,
    /// The pipe expressions inside its brackets, each replaced by underscores in `expression`.
    pub nested: Vec<Self>,
}

/// A pipe: `| name: argument: argument`.
#[derive(Debug)]
pub struct Pipe<'a> {
    /// Its name.
    pub name: String,
    /// Its name in the template.
    pub name_span: Span,
    /// Its arguments.
    pub arguments: Vec<Bound<'a>>,
    /// From `|` to its last argument.
    pub span: Span,
}

/// A text's characters at its top level, outside strings and brackets, and its top-level groups.
struct Scan {
    /// Each top-level character and its byte.
    top: Vec<(usize, char)>,
    /// Each top-level bracketed group, a template literal's `${ }` among them.
    groups: Vec<Group>,
}

/// A bracketed group.
struct Group {
    /// The byte after its opening (`(`, `[`, `{` or `${`).
    content_start: usize,
    /// Its closing's byte.
    close: usize,
    /// Whether an object literal's or a block's braces, whose elements are `key: value`.
    braces: bool,
}

/// An open bracket or template literal while scanning.
enum Frame {
    /// A bracket, its opening character, the byte after it, whether `${`.
    Group(char, usize, bool),
    /// A template literal.
    Template,
}

/// A text's top level, or the message of what is unbalanced and its byte.
fn scan(text: &str) -> Result<Scan, (String, usize)> {
    let characters: Vec<(usize, char)> = text.char_indices().collect();
    let mut stack: Vec<Frame> = Vec::new();
    let mut top = Vec::new();
    let mut groups = Vec::new();
    let mut index = 0;
    // The last two significant characters, which tell a regular expression from a division.
    let (mut previous, mut before) = (None, None);
    while let Some(&(at, character)) = characters.get(index) {
        let next = characters.get(index + 1).map(|each| each.1);
        if matches!(stack.last(), Some(Frame::Template)) {
            match character {
                '\\' => index += 1,
                '`' => {
                    stack.pop();
                }
                '$' if next == Some('{') => {
                    stack.push(Frame::Group('{', at + 2, true));
                    (previous, before) = (Some('('), None);
                    index += 1;
                }
                _ => {}
            }
            index += 1;
            continue;
        }
        let mut seen = character;
        match character {
            '\'' | '"' => {
                index += 1;
                loop {
                    match characters.get(index) {
                        None => return Err(("Unterminated quote".to_owned(), at)),
                        Some(&(_, '\\')) => index += 2,
                        Some(&(_, inner)) if inner == character => break,
                        Some(_) => index += 1,
                    }
                }
            }
            '/' if starts_regex(previous, before) => {
                index = regex_end(&characters, index)
                    .ok_or_else(|| ("Unterminated regular expression".to_owned(), at))?;
                // A regular expression ends an operand, as `)` does.
                seen = ')';
            }
            '`' => stack.push(Frame::Template),
            '(' | '[' | '{' => stack.push(Frame::Group(character, at + 1, false)),
            ')' | ']' | '}' => {
                let expected = match character {
                    ')' => '(',
                    ']' => '[',
                    _ => '{',
                };
                match stack.pop() {
                    Some(Frame::Group(opening, content_start, template)) if opening == expected => {
                        if !stack.iter().any(|frame| matches!(frame, Frame::Group(..))) {
                            groups.push(Group {
                                content_start,
                                close: at,
                                braces: opening == '{' && !template,
                            });
                        }
                    }
                    _ => return Err((format!("Unexpected token '{character}'"), at)),
                }
            }
            _ => {
                if stack.is_empty() {
                    top.push((at, character));
                }
            }
        }
        if !character.is_whitespace() {
            (previous, before) = (Some(seen), previous);
        }
        index += 1;
    }
    if stack.is_empty() {
        Ok(Scan { top, groups })
    } else {
        Err(("Missing closing bracket".to_owned(), text.len()))
    }
}

/// Whether a `/` starts a regular expression rather than divides, by the last two significant
/// characters before it, as `isStartOfRegex` of the expression lexer of `@angular/compiler`
/// 22.2.3 tells by the tokens: at the start, or after an operator, `(`, `[`, `,` or `:`; after a
/// `!` only when it negates, which an operand before it (a name, `)` or `]`) denies.
fn starts_regex(previous: Option<char>, before: Option<char>) -> bool {
    match previous {
        None => true,
        Some('!') => !before.is_some_and(|each| {
            each.is_ascii_alphanumeric() || matches!(each, '_' | '$' | ')' | ']')
        }),
        Some(each) => matches!(
            each,
            '+' | '-'
                | '*'
                | '/'
                | '%'
                | '^'
                | '?'
                | '='
                | '<'
                | '>'
                | '&'
                | '|'
                | '('
                | '['
                | ','
                | ':'
        ),
    }
}

/// The index of the closing `/` of the regular expression starting at an index (`scanRegex`):
/// its body ends at a `/` neither escaped nor in a character class; `None` when it does not
/// end. Its flags, letters, are read after it as a name, which ends an operand as they do.
fn regex_end(characters: &[(usize, char)], start: usize) -> Option<usize> {
    let mut index = start + 1;
    let (mut escaped, mut class) = (false, false);
    loop {
        let &(_, character) = characters.get(index)?;
        match character {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '[' => class = true,
            ']' => class = false,
            '/' if !class => break,
            _ => {}
        }
        index += 1;
    }
    Some(index)
}

/// The bytes of a text's top-level pipes: single `|`s.
fn pipes(text: &str, scan: &Scan) -> Vec<usize> {
    let bytes = text.as_bytes();
    scan.top
        .iter()
        .filter(|(at, character)| {
            *character == '|'
                && bytes.get(at + 1) != Some(&b'|')
                && (*at == 0 || bytes.get(at - 1) != Some(&b'|'))
        })
        .map(|(at, _)| *at)
        .collect()
}

/// The bytes of a text's top-level separators, `,` or `;`.
fn separators(scan: &Scan, separator: &[char]) -> Vec<usize> {
    scan.top
        .iter()
        .filter(|(_, character)| separator.contains(character))
        .map(|(at, _)| *at)
        .collect()
}

/// The bytes of the top-level `:`s of a text that end no ternary's `?`.
fn colons(text: &str, scan: &Scan) -> Vec<usize> {
    let bytes = text.as_bytes();
    let mut pending = 0_usize;
    let mut found = Vec::new();
    for (at, character) in &scan.top {
        match character {
            '?' if bytes.get(at + 1) != Some(&b'.')
                && bytes.get(at + 1) != Some(&b'?')
                && (*at == 0 || bytes.get(at - 1) != Some(&b'?')) =>
            {
                pending += 1;
            }
            ':' if pending > 0 => pending -= 1,
            ':' => found.push(*at),
            _ => {}
        }
    }
    found
}

/// The ranges of a text between separators.
fn split_ranges(length: usize, at: &[usize]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::with_capacity(at.len() + 1);
    let mut start = 0;
    for each in at {
        ranges.push((start, *each));
        start = each + 1;
    }
    ranges.push((start, length));
    ranges
}

/// The top-level pieces of a text split at `;`s (and `,`s when asked), each with its byte,
/// blank ones left out; the whole text when its brackets do not balance, for Oxc to report.
#[must_use]
pub fn split_top_level<'t>(text: &'t str, separator: &[char]) -> Vec<(usize, &'t str)> {
    let at = scan(text).map_or_else(|_| Vec::new(), |scan| separators(&scan, separator));
    split_ranges(text.len(), &at)
        .into_iter()
        .map(|(start, end)| (start, &text[start..end]))
        .filter(|(_, piece)| !piece.trim().is_empty())
        .collect()
}

/// A binding's text read: its expression, its pipes and its nested pipe expressions.
///
/// # Errors
///
/// When Oxc refuses the expression or an argument, or a pipe has no name.
pub fn read<'a>(allocator: &'a Allocator, text: &str, start: u32) -> Result<Bound<'a>, ParseError> {
    let found = scan(text).map_err(|(message, at)| error(message, start, at))?;
    let at = pipes(text, &found);
    let expression_end = at.first().copied().unwrap_or(text.len());
    let (expression, nested) = parse_part(allocator, &text[..expression_end], start)?;
    let mut bound = Bound {
        span: Span::new(start as usize, start as usize + text.len()),
        expression,
        start,
        pipes: Vec::new(),
        nested,
    };
    let ranges = split_ranges(text.len(), &at);
    for (from, to) in ranges.into_iter().skip(1) {
        bound
            .pipes
            .push(read_pipe(allocator, &text[from..to], start + offset(from))?);
    }
    Ok(bound)
}

/// Where an expression of a template binding's microsyntax ends: at the first top-level space
/// after a whole operand that a key follows (`condition then foo`, `items index as i`), as
/// `parseTemplateBindings` stops at a token that cannot go on with the expression; `in` and
/// `instanceof` go on with it, and `typeof`, `void`, `new`, `delete` and `await` ask for one.
#[must_use]
pub fn expression_end(text: &str) -> usize {
    /// Whether a character belongs to a word.
    fn is_word(character: char) -> bool {
        character.is_ascii_alphanumeric() || matches!(character, '_' | '$')
    }
    let Ok(found) = scan(text) else {
        return text.len();
    };
    for (at, character) in &found.top {
        if !character.is_whitespace() {
            continue;
        }
        let before = text[..*at].trim_end();
        let after = text[*at..].trim_start();
        let (Some(last), Some(next)) = (before.chars().last(), after.chars().next()) else {
            continue;
        };
        let previous_word = before
            .rsplit(|each| !is_word(each))
            .next()
            .unwrap_or_default();
        let next_word = after
            .split(|each| !is_word(each))
            .next()
            .unwrap_or_default();
        let operand_end = is_word(last) || matches!(last, ')' | ']' | '}' | '\'' | '"' | '`');
        let key_start = (next.is_ascii_alphabetic() || matches!(next, '_' | '$'))
            && !matches!(next_word, "in" | "instanceof");
        let operator_word = matches!(
            previous_word,
            "typeof" | "void" | "new" | "delete" | "await" | "in" | "instanceof"
        );
        if operand_end && key_start && !operator_word {
            return *at;
        }
    }
    text.len()
}

/// An action's text read: its statements split at `;`, none with a pipe.
///
/// # Errors
///
/// When a statement cannot be read, or holds a pipe.
pub fn read_action<'a>(
    allocator: &'a Allocator,
    text: &str,
    start: u32,
) -> Result<Vec<Bound<'a>>, ParseError> {
    let mut statements = Vec::new();
    for (at, statement) in split_top_level(text, &[';']) {
        let bound = read(allocator, statement, start + offset(at))?;
        if let Some(pipe) = bound.pipes.first() {
            return Err(ParseError {
                span: pipe.span,
                message: "Cannot have a pipe in an action expression".to_owned(),
            });
        }
        statements.push(bound);
    }
    Ok(statements)
}

/// A pipe's text after its `|`: its name, then its arguments after `:`s.
fn read_pipe<'a>(allocator: &'a Allocator, text: &str, start: u32) -> Result<Pipe<'a>, ParseError> {
    let name_start = text.len() - text.trim_start().len();
    let name_length = text[name_start..]
        .find(|each: char| !(each.is_ascii_alphanumeric() || each == '_' || each == '$'))
        .unwrap_or(text.len() - name_start);
    if name_length == 0 {
        return Err(error(
            "Unexpected end of a pipe, expected identifier or keyword".to_owned(),
            start,
            name_start,
        ));
    }
    let name_end = name_start + name_length;
    let span = Span::new(start as usize - 1, start as usize + text.len());
    let rest = &text[name_end..];
    let mut arguments = Vec::new();
    let after = rest.trim_start();
    if !after.is_empty() {
        let colon = name_end + rest.len() - after.len();
        if !after.starts_with(':') {
            return Err(error(
                format!(
                    "Unexpected token '{}' after the pipe's name",
                    after.chars().next().unwrap_or_default()
                ),
                start,
                colon,
            ));
        }
        let list = &text[colon + 1..];
        let found = scan(list).map_err(|(message, at)| error(message, start, colon + 1 + at))?;
        for (from, to) in split_ranges(list.len(), &colons(list, &found)) {
            let base = start + offset(colon + 1 + from);
            arguments.push(read(allocator, &list[from..to], base)?);
        }
    }
    Ok(Pipe {
        name: text[name_start..name_end].to_owned(),
        name_span: Span::new(start as usize + name_start, start as usize + name_end),
        arguments,
        span,
    })
}

/// A part without top-level pipes parsed by Oxc, its bracketed pipe expressions read apart.
fn parse_part<'a>(
    allocator: &'a Allocator,
    text: &str,
    start: u32,
) -> Result<(Option<Expression<'a>>, Vec<Bound<'a>>), ParseError> {
    if text.trim().is_empty() {
        return Ok((None, Vec::new()));
    }
    let mut masked = text.to_owned();
    let mut nested = Vec::new();
    mask(allocator, text, start, 0, &mut masked, &mut nested)?;
    let source = allocator.alloc_str(&masked);
    match Parser::new(allocator, source, SourceType::ts()).parse_expression() {
        Ok(expression) => Ok((Some(expression), nested)),
        Err(diagnostics) => {
            let (message, at) = diagnostics.first().map_or_else(
                || ("Invalid expression".to_owned(), 0),
                |diagnostic| {
                    (
                        diagnostic.message.to_string(),
                        diagnostic
                            .labels
                            .first()
                            .map_or(0, oxc_span::LabeledSpan::offset),
                    )
                },
            );
            Err(error(message, start, at as usize))
        }
    }
}

/// Reads the pipe expressions inside a text's top-level brackets as nested expressions and
/// masks them with underscores, from a byte of the masked text.
fn mask<'a>(
    allocator: &'a Allocator,
    text: &str,
    start: u32,
    from: usize,
    masked: &mut String,
    nested: &mut Vec<Bound<'a>>,
) -> Result<(), ParseError> {
    let Ok(found) = scan(text) else {
        // Unbalanced: Oxc reports it on the whole text.
        return Ok(());
    };
    for group in &found.groups {
        let content = &text[group.content_start..group.close];
        let Ok(inner) = scan(content) else {
            continue;
        };
        for (element_start, element_end) in split_ranges(content.len(), &separators(&inner, &[',']))
        {
            let mut element_start = element_start;
            let element = &content[element_start..element_end];
            if group.braces
                && let Ok(pair) = scan(element)
                && let Some(colon) = colons(element, &pair).first()
            {
                element_start += colon + 1;
            }
            let element = &content[element_start..element_end];
            let absolute = from + group.content_start + element_start;
            let Ok(element_scan) = scan(element) else {
                continue;
            };
            if pipes(element, &element_scan).is_empty() {
                mask(allocator, element, start, absolute, masked, nested)?;
            } else {
                nested.push(read(allocator, element, start + offset(absolute))?);
                masked.replace_range(
                    absolute..absolute + element.len(),
                    &"_".repeat(element.len()),
                );
            }
        }
    }
    Ok(())
}

/// An error at a byte of a text.
fn error(message: String, start: u32, at: usize) -> ParseError {
    let at = start as usize + at;
    ParseError {
        span: Span::new(at, at),
        message,
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Expression;

    use super::{read, read_action, split_top_level};

    /// The source text of an expression, from its Oxc span.
    fn text_of<'t>(text: &'t str, expression: Option<&Expression>) -> &'t str {
        use oxc_span::GetSpan;
        expression.map_or("", |expression| {
            let span = expression.span();
            &text[span.start as usize..span.end as usize]
        })
    }

    #[test]
    fn parses_an_expression_without_pipes() {
        let allocator = Allocator::default();
        let bound =
            read(&allocator, " a.b?.c(d!) ", 10).unwrap_or_else(|error| panic!("{error:?}"));

        assert!(matches!(
            bound.expression,
            Some(Expression::CallExpression(_) | Expression::ChainExpression(_))
        ));
        assert_eq!(
            text_of(" a.b?.c(d!) ", bound.expression.as_ref()),
            "a.b?.c(d!)"
        );
        assert_eq!(
            (bound.span.start, bound.span.end, bound.start),
            (10, 22, 10)
        );
        assert!(bound.pipes.is_empty() && bound.nested.is_empty());
    }

    #[test]
    fn splits_pipes_and_their_arguments_off_keeping_or_and_ternaries() {
        let allocator = Allocator::default();
        let text = "a || b | date: c ? 'x:y' : d : 'e' | uppercase";
        let bound = read(&allocator, text, 0).unwrap_or_else(|error| panic!("{error:?}"));

        assert_eq!(text_of(text, bound.expression.as_ref()), "a || b");
        let names: Vec<&str> = bound.pipes.iter().map(|pipe| pipe.name.as_str()).collect();
        assert_eq!(names, ["date", "uppercase"]);
        // `date` starts at byte 9: `a || b | ` is 9 bytes.
        assert_eq!(
            (bound.pipes[0].name_span.start, bound.pipes[0].name_span.end),
            (9, 13)
        );
        let arguments: Vec<&str> = bound.pipes[0]
            .arguments
            .iter()
            .map(|argument| {
                let span = argument.span;
                &text[span.start as usize..span.end as usize]
            })
            .collect();
        assert_eq!(arguments, [" c ? 'x:y' : d ", " 'e' "]);
        assert!(bound.pipes[1].arguments.is_empty());
    }

    #[test]
    fn reads_pipes_inside_brackets_as_nested_expressions() {
        let allocator = Allocator::default();
        let text = "(items | async)?.length + f(a, b | p: 1) + {k: v | q}.k + `${t | r}`";
        let bound = read(&allocator, text, 100).unwrap_or_else(|error| panic!("{error:?}"));

        assert!(bound.pipes.is_empty());
        let nested: Vec<(&str, &str)> = bound
            .nested
            .iter()
            .map(|each| {
                let span = each.span;
                (
                    &text[(span.start - 100) as usize..(span.end - 100) as usize],
                    each.pipes[0].name.as_str(),
                )
            })
            .collect();
        assert_eq!(
            nested,
            [
                ("items | async", "async"),
                (" b | p: 1", "p"),
                (" v | q", "q"),
                ("t | r", "r")
            ]
        );
        assert!(bound.expression.is_some());
    }

    #[test]
    fn reports_what_oxc_refuses_and_a_pipe_without_a_name_at_their_byte() {
        let allocator = Allocator::default();

        let refused = read(&allocator, "a +", 50).err();
        assert_eq!(refused.map(|error| error.span.start), Some(53));
        let unnamed = read(&allocator, "a | ", 0).err();
        assert_eq!(
            unnamed.map(|error| (error.message, error.span.start)),
            Some((
                "Unexpected end of a pipe, expected identifier or keyword".to_owned(),
                4
            ))
        );
        let after_name = read(&allocator, "a | p x", 0).err();
        assert_eq!(after_name.map(|error| error.span.start), Some(6));
        let unbalanced = read(&allocator, "f(a", 0).err();
        assert_eq!(
            unbalanced.map(|error| error.message),
            Some("Missing closing bracket".to_owned())
        );
        assert!(read(&allocator, "'a", 0).is_err());
        assert!(read(&allocator, "a)", 0).is_err());
        // An argument whose brackets do not close, a nested pipe without a name, a nested
        // expression Oxc refuses: each is reported.
        assert_eq!(
            read(&allocator, "a | p: (b", 0)
                .err()
                .map(|error| (error.message, error.span.start)),
            Some(("Missing closing bracket".to_owned(), 9))
        );
        assert!(read(&allocator, "f(a | )", 0).is_err());
        assert!(read(&allocator, "f(a + | p)", 0).is_err());
        // Brackets that do not close inside a group are left to Oxc, which reports them.
        assert!(read(&allocator, "f([a | p)", 0).is_err());
        assert!(read(&allocator, "  ", 0).is_ok_and(|bound| bound.expression.is_none()));
    }

    #[test]
    fn f52_ac1_reads_a_regular_expression_s_body_as_one_operand() {
        let allocator = Allocator::default();
        let pipe_names = |text: &str| {
            read(&allocator, text, 0)
                .map(|bound| {
                    bound
                        .pipes
                        .iter()
                        .map(|pipe| (pipe.name.clone(), pipe.arguments.len()))
                        .collect::<Vec<_>>()
                })
                .map_err(|error| error.message)
        };

        // Angular 22.2.3 reads each without an error (run by the auditor): the `|` of the body
        // is no pipe, its `:` no argument's end, its `)` in a class closes nothing.
        assert_eq!(pipe_names(" /a|b/.test(x) "), Ok(vec![]));
        assert_eq!(pipe_names(" a | p: /:/ "), Ok(vec![("p".to_owned(), 1)]));
        assert_eq!(pipe_names(" x | p: /[)]/ "), Ok(vec![("p".to_owned(), 1)]));
        assert_eq!(pipe_names(" !/a|b/i.test(x) "), Ok(vec![]));
        assert_eq!(pipe_names(" f(/a\\/b|c/, !!/d|e/) "), Ok(vec![]));
        assert_eq!(
            pipe_names(" x | p: /[/]|a/ "),
            Ok(vec![("p".to_owned(), 1)])
        );
        assert_eq!(
            pipe_names(" `${/[(]/.source}` | p "),
            Ok(vec![("p".to_owned(), 0)])
        );
        // A `/` after an operand divides: after a name, a `)`, a non-null `!`, a regular
        // expression.
        assert_eq!(pipe_names(" a / b | p "), Ok(vec![("p".to_owned(), 0)]));
        assert_eq!(
            pipe_names(" (a) / 2 + a! / b | p "),
            Ok(vec![("p".to_owned(), 0)])
        );
        assert_eq!(
            pipe_names(" /a/.source.length / 2 | p "),
            Ok(vec![("p".to_owned(), 0)])
        );
        assert_eq!(
            pipe_names(" /a "),
            Err("Unterminated regular expression".to_owned())
        );
    }

    #[test]
    fn reads_an_actions_statements_and_refuses_a_pipe_there() {
        let allocator = Allocator::default();
        let statements = read_action(&allocator, "a = $event; b(';'); ", 0)
            .unwrap_or_else(|error| panic!("{error:?}"));

        assert_eq!(statements.len(), 2);
        assert_eq!(statements[1].start, 11);
        let piped = read_action(&allocator, "a | p", 0).err();
        assert_eq!(
            piped.map(|error| (error.message, error.span.start)),
            Some(("Cannot have a pipe in an action expression".to_owned(), 2))
        );
        assert_eq!(
            split_top_level("a; (b; c)", &[';']),
            [(0, "a"), (2, " (b; c)")]
        );
    }
}
