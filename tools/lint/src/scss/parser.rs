//! The reading of a style sheet into its tree: a port of `lib/parser.js` of `postcss` 8.5.30 with
//! the overrides of `scss-parser.js` of `postcss-scss` 4.0.9 (MIT, under ADR-0076), the parser
//! Stylelint reads SCSS with: an at-rule's name joined to an interpolation written against it,
//! the `//` comments, and a declaration holding nested properties (`font: 12px { family: x }`).
//! `PostCSS` stops at its first error; so does this reader, which gives the error's message and
//! byte. A container nested deeper than `NESTING_LIMIT` is an error too, as the rules walk the
//! tree's ancestors.

use super::tokenizer::{Kind as TokenKind, Token, Tokenizer, js_or};
use super::tree::{Kind, Node, Sheet};
use crate::template::NESTING_LIMIT;

/// An error of the reading: its message, `PostCSS`'s, and the byte it is reported at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// What is wrong.
    pub message: String,
    /// Where.
    pub offset: usize,
}

/// The result of a step of the reading.
type Read<T> = Result<T, ParseError>;

/// A sheet's tree, or the first error of its reading. The text is read as given: a byte order
/// mark, which `PostCSS`'s `Input` drops before parsing, is dropped by the caller
/// (`scss::lint_text`), whose places and fixes take it into account.
///
/// # Errors
///
/// The first error `PostCSS` would throw, or a nesting deeper than `NESTING_LIMIT`.
pub fn parse(css: &str) -> Read<Sheet> {
    let mut parser = Parser {
        css,
        tokenizer: Tokenizer::new(css),
        sheet: Sheet {
            nodes: vec![Node {
                children: Some(Vec::new()),
                ..Node::new(Kind::Root, 0)
            }],
        },
        current: 0,
        depth: 0,
        spaces: String::new(),
        semicolon: false,
    };
    parser.parse()?;
    Ok(parser.sheet)
}

/// An error at a byte.
fn error<T>(message: impl Into<String>, offset: usize) -> Read<T> {
    Err(ParseError {
        message: message.into(),
        offset,
    })
}

/// Whether a token is a space or a comment.
const fn is_trivia(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Space | TokenKind::Comment { .. })
}

/// Whether a token is a space.
const fn is_space(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Space)
}

/// The text of a JavaScript `\s` class: white space or the byte order mark.
fn is_js_space(character: char) -> bool {
    character.is_whitespace() || character == '\u{feff}'
}

/// A comment's text split as `/^(\s*)([^]*\S)(\s*)$/` splits it: the spaces before, the text,
/// the spaces after; all spaces, the text empty and the spaces before.
fn split_comment(text: &str) -> (String, String, String) {
    let trimmed = text.trim_start_matches(is_js_space);
    if trimmed.is_empty() {
        return (text.to_owned(), String::new(), String::new());
    }
    let left = &text[..text.len() - trimmed.len()];
    let body = trimmed.trim_end_matches(is_js_space);
    let right = &trimmed[body.len()..];
    (left.to_owned(), body.to_owned(), right.to_owned())
}

/// The parser over a sheet.
struct Parser<'t> {
    /// The text.
    css: &'t str,
    /// Its tokens.
    tokenizer: Tokenizer<'t>,
    /// The tree built.
    sheet: Sheet,
    /// The container the next node goes into.
    current: usize,
    /// How many containers the current one is nested in.
    depth: usize,
    /// The spaces read since the last node.
    spaces: String,
    /// Whether the last declaration ended with `;`.
    semicolon: bool,
}

impl Parser<'_> {
    /// A token's text.
    fn text(&self, token: &Token) -> &str {
        token.text(self.css)
    }

    /// The text of tokens, joined.
    fn joined(&self, tokens: &[Token]) -> String {
        tokens.iter().map(|token| self.text(token)).collect()
    }

    /// The next token.
    fn next_token(&mut self) -> Read<Option<Token>> {
        self.tokenizer.next_token().map_err(|unclosed| ParseError {
            message: format!("Unclosed {}", unclosed.what),
            offset: unclosed.offset,
        })
    }

    /// The current container.
    fn current(&mut self) -> &mut Node {
        &mut self.sheet.nodes[self.current]
    }

    /// Reads every token into the tree.
    fn parse(&mut self) -> Read<()> {
        while !self.tokenizer.end_of_file() {
            let Some(token) = self.next_token()? else {
                break;
            };
            match token.kind {
                TokenKind::Space => self.spaces.push_str(token.text(self.css)),
                TokenKind::Char(b';') => self.free_semicolon(token),
                TokenKind::Char(b'}') => self.end(token)?,
                TokenKind::Comment { inline } => self.comment(token, inline),
                TokenKind::AtWord => self.at_rule(token)?,
                TokenKind::Char(b'{') => self.empty_rule(token)?,
                _ => self.other(token)?,
            }
        }
        self.end_file()
    }

    /// Adds a node to the current container; its index.
    fn init(&mut self, mut node: Node) -> usize {
        let index = self.sheet.nodes.len();
        node.parent = Some(self.current);
        node.before = std::mem::take(&mut self.spaces);
        if !node.is(Kind::Comment) {
            self.semicolon = false;
        }
        self.sheet.nodes.push(node);
        if let Some(children) = self.current().children.as_mut() {
            children.push(index);
        }
        index
    }

    /// Opens a container, unless it nests deeper than the limit.
    fn open(&mut self, index: usize) -> Read<()> {
        if self.depth >= NESTING_LIMIT {
            return error(
                format!("Blocks nested more than {NESTING_LIMIT} deep"),
                self.sheet.nodes[index].start,
            );
        }
        self.depth += 1;
        self.current = index;
        Ok(())
    }

    /// An at-rule, its name joined to a word written against it (`@include#{…}`).
    fn at_rule(&mut self, token: Token) -> Read<()> {
        let mut name = self.text(&token).to_owned();
        let mut previous = token;
        while !self.tokenizer.end_of_file() {
            let Some(next) = self.next_token()? else {
                break;
            };
            if next.kind == TokenKind::Word
                && previous.last().is_some_and(|last| next.start == last + 1)
            {
                name.push_str(self.text(&next));
                previous = next;
            } else {
                self.tokenizer.back(next);
                break;
            }
        }
        if name.len() == 1 {
            return error("At-rule without name", token.start);
        }
        let index = self.init(Node {
            name: name[1..].to_owned(),
            ..Node::new(Kind::AtRule, token.start)
        });
        let mut params: Vec<Token> = Vec::new();
        let mut brackets: Vec<u8> = Vec::new();
        let mut last = false;
        let mut open = false;
        while !self.tokenizer.end_of_file() {
            let Some(token) = self.next_token()? else {
                break;
            };
            match token.kind {
                TokenKind::Open => brackets.push(b')'),
                TokenKind::Char(b'[') => brackets.push(b']'),
                TokenKind::Char(b'{') if !brackets.is_empty() => brackets.push(b'}'),
                TokenKind::Char(close) if brackets.last() == Some(&close) => {
                    brackets.pop();
                }
                _ => {}
            }
            if brackets.is_empty() {
                match token.kind {
                    TokenKind::Char(b';') => {
                        self.sheet.nodes[index].end = Some(token.start + 1);
                        self.semicolon = true;
                        break;
                    }
                    TokenKind::Char(b'{') => {
                        open = true;
                        break;
                    }
                    TokenKind::Char(b'}') => {
                        let previous = params.iter().rev().find(|each| !is_space(each));
                        if let Some(at) = previous.and_then(Token::last_or_position) {
                            self.sheet.nodes[index].end = Some(at + 1);
                        }
                        self.end(token)?;
                        break;
                    }
                    _ => params.push(token),
                }
            } else {
                params.push(token);
            }
            if self.tokenizer.end_of_file() {
                last = true;
                break;
            }
        }
        let between = self.trivia_from_end(&mut params);
        if params.is_empty() {
            self.sheet.nodes[index].between = between;
        } else {
            let leading = params.iter().take_while(|each| is_trivia(each)).count();
            let after_name = self.joined(&params[..leading]);
            params.drain(..leading);
            let (value, raw) = self.raw(&params, false);
            let node = &mut self.sheet.nodes[index];
            node.after_name = after_name;
            node.params = value;
            node.raw = raw;
            node.between = between;
            if last {
                let at = params.last().and_then(Token::last_or_position);
                let node = &mut self.sheet.nodes[index];
                node.end = at.map(|at| at + 1);
                self.spaces = std::mem::take(&mut node.between);
            }
        }
        if open {
            self.sheet.nodes[index].children = Some(Vec::new());
            self.open(index)?;
        }
        Ok(())
    }

    /// Checks a value holding `:` for a missed `;`.
    fn check_missed_semicolon(&self, tokens: &[Token]) -> Read<()> {
        let Some(colon) = self.colon(tokens)? else {
            return Ok(());
        };
        let mut found = 0;
        let mut token = tokens[colon];
        for each in tokens[..colon].iter().rev() {
            token = *each;
            if !is_space(each) {
                found += 1;
                if found == 2 {
                    break;
                }
            }
        }
        let offset = if token.kind == TokenKind::Word {
            token.end
        } else {
            token.start
        };
        error("Missed semicolon", offset)
    }

    /// The index of the first `:` outside parentheses, `progid:` excepted.
    fn colon(&self, tokens: &[Token]) -> Read<Option<usize>> {
        let mut brackets = 0_i32;
        let mut previous: Option<&Token> = None;
        for (index, token) in tokens.iter().enumerate() {
            match token.kind {
                TokenKind::Open => brackets += 1,
                TokenKind::Char(b')') => brackets -= 1,
                _ => {}
            }
            if brackets == 0 && token.kind == TokenKind::Char(b':') {
                match previous {
                    None => return error("Double colon", token.start),
                    Some(word) if word.kind == TokenKind::Word && self.text(word) == "progid" => {
                        continue;
                    }
                    Some(_) => return Ok(Some(index)),
                }
            }
            previous = Some(token);
        }
        Ok(None)
    }

    /// A comment.
    fn comment(&mut self, token: Token, inline: bool) {
        let text = self.text(&token);
        let inside = if inline {
            &text[2..]
        } else {
            &text[2..text.len() - 2]
        };
        let (left, mut body, right) = split_comment(inside);
        if inline {
            body = escape_comment(&body);
        }
        self.init(Node {
            end: Some(token.end),
            text: body,
            left,
            right,
            inline,
            ..Node::new(Kind::Comment, token.start)
        });
    }

    /// A declaration from its tokens.
    fn declaration(&mut self, mut tokens: Vec<Token>, custom: bool) -> Read<()> {
        let index = self.init(Node::new(Kind::Declaration, tokens[0].start));
        let last = tokens[tokens.len() - 1];
        if last.kind == TokenKind::Char(b';') {
            self.semicolon = true;
            tokens.pop();
        }
        let end = js_or(last.last_or_position(), find_last_with_position(&tokens));
        self.sheet.nodes[index].end = end.map(|at| at + 1);

        let mut start = 0;
        while tokens[start].kind != TokenKind::Word {
            if start == tokens.len() - 1 {
                return self.unknown_word(&tokens[start..=start]);
            }
            start += 1;
        }
        let before = self.joined(&tokens[..start]);
        self.sheet.nodes[index].start = tokens[start].start;
        let prop_start = start;
        while start < tokens.len()
            && !matches!(
                tokens[start].kind,
                TokenKind::Char(b':') | TokenKind::Space | TokenKind::Comment { .. }
            )
        {
            start += 1;
        }
        let mut prop = self.joined(&tokens[prop_start..start]);
        let between_start = start;
        while start < tokens.len() {
            let token = tokens[start];
            start += 1;
            if token.kind == TokenKind::Char(b':') {
                break;
            }
            if token.kind == TokenKind::Word
                && self
                    .text(&token)
                    .bytes()
                    .any(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
            {
                return self.unknown_word(&[token]);
            }
        }
        let mut between = self.joined(&tokens[between_start..start]);
        let mut before = format!("{}{before}", self.sheet.nodes[index].before);
        if prop.starts_with(['_', '*']) {
            before.push_str(&prop[..1]);
            prop.remove(0);
        }
        let first_spaces_start = start;
        while start < tokens.len() && is_trivia(&tokens[start]) {
            start += 1;
        }
        let mut first_spaces: Vec<Token> = tokens[first_spaces_start..start].to_vec();
        let mut tokens = tokens.split_off(start);
        let (important, important_raw) = self.important(&mut tokens, true, 0);
        if tokens.iter().any(|token| !is_trivia(token)) {
            between.push_str(&self.joined(&first_spaces));
            first_spaces.clear();
        }
        first_spaces.extend(tokens.iter().copied());
        let (value, raw) = self.raw(&first_spaces, custom);
        let node = &mut self.sheet.nodes[index];
        node.before = before;
        node.prop = prop;
        node.between = between;
        node.important = important;
        node.important_raw = important_raw;
        node.raw = raw;
        let checks = value.contains(':') && !custom;
        node.value = value;
        if checks {
            self.check_missed_semicolon(&tokens)?;
        }
        Ok(())
    }

    /// Takes a value's `!important` off its tokens, from the last token down to `lowest`
    /// excluded; whether there was one, and its raw text when not ` !important`.
    /// `insensitive`: whether `!IMPORTANT` counts, as for a declaration but not a nested one.
    fn important(
        &self,
        tokens: &mut Vec<Token>,
        insensitive: bool,
        lowest: usize,
    ) -> (bool, Option<String>) {
        let same = |text: &str, wanted: &str| {
            if insensitive {
                text.eq_ignore_ascii_case(wanted)
            } else {
                text == wanted
            }
        };
        let mut index = tokens.len();
        while index > lowest {
            index -= 1;
            let token = tokens[index];
            let text = self.text(&token);
            if same(text, "!important") {
                let mut string = self.joined(&tokens[index..]);
                tokens.truncate(index);
                let spaces = tokens
                    .iter()
                    .rev()
                    .take_while(|each| is_space(each))
                    .count();
                let at = tokens.len() - spaces;
                string = format!("{}{string}", self.joined(&tokens[at..]));
                tokens.truncate(at);
                let raw = (string != " !important").then_some(string);
                return (true, raw);
            } else if same(text, "important") {
                let mut cache = tokens.clone();
                let mut string = String::new();
                let mut inner = index;
                while inner > 0 {
                    let kind = cache[inner].kind;
                    if string.trim().starts_with('!') && kind != TokenKind::Space {
                        break;
                    }
                    let popped = cache.pop().map(|each| self.text(&each).to_owned());
                    string = format!("{}{string}", popped.unwrap_or_default());
                    inner -= 1;
                }
                if string.trim().starts_with('!') {
                    *tokens = cache;
                    return (true, Some(string));
                }
            }
            if !is_trivia(&token) {
                break;
            }
        }
        (false, None)
    }

    /// An empty rule, `{` without a selector.
    fn empty_rule(&mut self, token: Token) -> Read<()> {
        let index = self.init(Node {
            children: Some(Vec::new()),
            ..Node::new(Kind::Rule, token.start)
        });
        self.open(index)
    }

    /// `}`: closes the current container.
    fn end(&mut self, token: Token) -> Read<()> {
        let semicolon = std::mem::take(&mut self.semicolon);
        let spaces = std::mem::take(&mut self.spaces);
        let node = self.current();
        if !node.nodes().is_empty() {
            node.semicolon = semicolon;
        }
        node.after.push_str(&spaces);
        match node.parent {
            Some(parent) => {
                node.end = Some(token.start + 1);
                self.current = parent;
                self.depth -= 1;
                Ok(())
            }
            None => error("Unexpected }", token.start),
        }
    }

    /// The end of the text: a container left open is an error.
    fn end_file(&mut self) -> Read<()> {
        if self.current != 0 {
            return error("Unclosed block", self.sheet.nodes[self.current].start);
        }
        let semicolon = self.semicolon;
        let spaces = std::mem::take(&mut self.spaces);
        let end = self.tokenizer.position();
        let root = self.current();
        if !root.nodes().is_empty() {
            root.semicolon = semicolon;
        }
        root.after.push_str(&spaces);
        root.end = Some(end);
        Ok(())
    }

    /// `;` outside a declaration: a rule's own semicolon, or spaces.
    fn free_semicolon(&mut self, token: Token) {
        self.spaces
            .push_str(self.css.get(token.start..token.end).unwrap_or_default());
        let last = self.sheet.nodes[self.current].nodes().last().copied();
        if let Some(previous) = last {
            let node = &self.sheet.nodes[previous];
            if node.is(Kind::Rule) && node.own_semicolon.is_none() {
                let spaces = std::mem::take(&mut self.spaces);
                let node = &mut self.sheet.nodes[previous];
                node.own_semicolon = Some(spaces);
                node.end = Some(token.start + 1);
            }
        }
    }

    /// A declaration or a rule, from its first token, or an unknown word.
    fn other(&mut self, first: Token) -> Read<()> {
        let custom = self.text(&first).starts_with("--");
        let mut end = false;
        let mut colon = false;
        let mut bracket: Option<Token> = None;
        let mut brackets: Vec<u8> = Vec::new();
        let mut tokens: Vec<Token> = Vec::new();
        let mut current = Some(first);
        while let Some(token) = current {
            tokens.push(token);
            match token.kind {
                TokenKind::Open | TokenKind::Char(b'[') => {
                    bracket.get_or_insert(token);
                    brackets.push(if token.kind == TokenKind::Open {
                        b')'
                    } else {
                        b']'
                    });
                }
                TokenKind::Char(b'{') if custom && colon => {
                    bracket.get_or_insert(token);
                    brackets.push(b'}');
                }
                kind if brackets.is_empty() => match kind {
                    TokenKind::Char(b';') => {
                        if colon {
                            return self.declaration(tokens, custom);
                        }
                        break;
                    }
                    TokenKind::Char(b'{') => return self.rule(tokens),
                    TokenKind::Char(b'}') => {
                        if let Some(close) = tokens.pop() {
                            self.tokenizer.back(close);
                        }
                        end = true;
                        break;
                    }
                    TokenKind::Char(b':') => colon = true,
                    _ => {}
                },
                TokenKind::Char(close) if brackets.last() == Some(&close) => {
                    brackets.pop();
                    if brackets.is_empty() {
                        bracket = None;
                    }
                }
                _ => {}
            }
            current = self.next_token()?;
        }
        if self.tokenizer.end_of_file() {
            end = true;
        }
        if let Some(open) = bracket.filter(|_| !brackets.is_empty()) {
            return error("Unclosed bracket", open.start);
        }
        if end && colon {
            if !custom {
                while tokens.last().is_some_and(is_trivia) {
                    if let Some(token) = tokens.pop() {
                        self.tokenizer.back(token);
                    }
                }
            }
            return self.declaration(tokens, custom);
        }
        self.unknown_word(&tokens)
    }

    /// The value of tokens, comments between words kept, and the raw text when it differs.
    fn raw(&self, tokens: &[Token], custom: bool) -> (String, Option<String>) {
        let mut value = String::new();
        let mut last = ' ';
        let mut clean = true;
        for (index, token) in tokens.iter().enumerate() {
            let text = self.text(token);
            match token.kind {
                TokenKind::Space if index == tokens.len() - 1 && !custom => clean = false,
                TokenKind::Comment { .. } => {
                    let safe = |neighbour: Option<&Token>| neighbour.is_none_or(is_space);
                    let previous = index.checked_sub(1).and_then(|at| tokens.get(at));
                    if !safe(previous) && !safe(tokens.get(index + 1)) {
                        if last == ',' {
                            clean = false;
                        } else {
                            value.push_str(text);
                            last = text.chars().last().unwrap_or(last);
                        }
                    } else {
                        clean = false;
                    }
                }
                _ => {
                    value.push_str(text);
                    last = text.chars().last().unwrap_or(last);
                }
            }
        }
        let raw = (!clean).then(|| self.joined(tokens));
        (value, raw)
    }

    /// A rule, or in SCSS a declaration with nested properties, from its tokens up to `{`.
    fn rule(&mut self, mut tokens: Vec<Token>) -> Read<()> {
        if let Some(value) = self.nested_value(&tokens)
            && !value.trim().is_empty()
            && !value.starts_with(|first: char| {
                first == '#' || first == ':' || first == '-' || first.is_ascii_alphabetic()
            })
        {
            return self.nested_declaration(tokens);
        }
        tokens.pop();
        let index = self.init(Node {
            children: Some(Vec::new()),
            ..Node::new(Kind::Rule, tokens[0].start)
        });
        let between = self.trivia_from_end(&mut tokens);
        let (selector, raw) = self.raw(&tokens, false);
        let node = &mut self.sheet.nodes[index];
        node.between = between;
        node.selector = selector;
        node.raw = raw;
        self.open(index)
    }

    /// The text after a rule's first `:` outside parentheses, on its first line, comments and
    /// `{` left out; none without such a colon.
    fn nested_value(&self, tokens: &[Token]) -> Option<String> {
        let mut colon = false;
        let mut brackets = 0_i32;
        let mut value = String::new();
        for token in tokens {
            if colon {
                if !matches!(
                    token.kind,
                    TokenKind::Comment { .. } | TokenKind::Char(b'{')
                ) {
                    value.push_str(self.text(token));
                }
            } else if token.kind == TokenKind::Space && self.text(token).contains('\n') {
                break;
            } else if token.kind == TokenKind::Open {
                brackets += 1;
            } else if token.kind == TokenKind::Char(b')') {
                brackets -= 1;
            } else if brackets == 0 && token.kind == TokenKind::Char(b':') {
                colon = true;
            }
        }
        colon.then_some(value)
    }

    /// A declaration holding nested properties, `font: 12px {`.
    fn nested_declaration(&mut self, mut tokens: Vec<Token>) -> Read<()> {
        tokens.pop();
        let index = self.init(Node {
            children: Some(Vec::new()),
            ..Node::new(Kind::Declaration, tokens[0].start)
        });
        if let Some(last) = tokens.iter().rev().find(|token| !is_space(token)) {
            let at = match last.last() {
                Some(end) if end != 0 => end,
                _ => last.start,
            };
            self.sheet.nodes[index].end = Some(at + 1);
        }
        let leading = tokens
            .iter()
            .take_while(|token| token.kind != TokenKind::Word)
            .count();
        let mut before = self.sheet.nodes[index].before.clone();
        before.push_str(&self.joined(&tokens[..leading]));
        tokens.drain(..leading);
        if let Some(first) = tokens.first().filter(|first| first.start != 0) {
            self.sheet.nodes[index].start = first.start;
        }
        let name = tokens
            .iter()
            .take_while(|token| {
                !matches!(
                    token.kind,
                    TokenKind::Char(b':') | TokenKind::Space | TokenKind::Comment { .. }
                )
            })
            .count();
        let mut prop = self.joined(&tokens[..name]);
        tokens.drain(..name);
        let colon = tokens
            .iter()
            .position(|token| token.kind == TokenKind::Char(b':'))
            .map_or(tokens.len(), |at| at + 1);
        let mut between = self.joined(&tokens[..colon]);
        tokens.drain(..colon);
        if prop.starts_with(['_', '*']) {
            before.push_str(&prop[..1]);
            prop.remove(0);
        }
        let spaces = tokens.iter().take_while(|token| is_trivia(token)).count();
        between.push_str(&self.joined(&tokens[..spaces]));
        tokens.drain(..spaces);
        // `postcss-scss` stops before the first token here (`i > 0`), `PostCSS` at it.
        let (important, important_raw) = self.important(&mut tokens, false, 1);
        let (value, raw) = self.raw(&tokens, false);
        let checks = value.contains(':');
        let node = &mut self.sheet.nodes[index];
        node.before = before;
        node.prop = prop;
        node.between = between;
        node.important = important;
        node.important_raw = important_raw;
        node.value = value;
        node.raw = raw;
        if checks {
            self.check_missed_semicolon(&tokens)?;
        }
        self.open(index)
    }

    /// Takes the spaces and comments off the end of tokens; their text.
    fn trivia_from_end(&self, tokens: &mut Vec<Token>) -> String {
        let count = tokens
            .iter()
            .rev()
            .take_while(|each| is_trivia(each))
            .count();
        let at = tokens.len() - count;
        let text = self.joined(&tokens[at..]);
        tokens.truncate(at);
        text
    }

    /// The error of a word that is neither a declaration nor a rule.
    fn unknown_word<T>(&self, tokens: &[Token]) -> Read<T> {
        error(
            format!("Unknown word {}", self.text(&tokens[0])),
            tokens[0].start,
        )
    }
}

/// The last byte of the last token that has one, as `findLastWithPosition`.
fn find_last_with_position(tokens: &[Token]) -> Option<usize> {
    tokens.iter().rev().find_map(Token::last_or_position)
}

/// An inline comment's text with each `*/` and `/*` made `*//*`, in one pass as
/// `replace(/(\*\/|\/\*)/g, '*//*')`, so that it prints as a CSS comment.
fn escape_comment(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find(['*', '/']) {
        result.push_str(&rest[..at]);
        let pair = rest.get(at..at + 2);
        if matches!(pair, Some("*/" | "/*")) {
            result.push_str("*//*");
            rest = &rest[at + 2..];
        } else {
            result.push_str(&rest[at..=at]);
            rest = &rest[at + 1..];
        }
    }
    result.push_str(rest);
    result
}

#[cfg(test)]
mod tests {
    use super::{ParseError, parse};
    use crate::lines::Lines;
    use crate::scss::tree::{Kind, Node, Sheet};
    use crate::template::NESTING_LIMIT;

    /// A sheet's tree; an empty one when it cannot be read, which the assertions then refuse.
    fn read(css: &str) -> Sheet {
        parse(css).unwrap_or_default()
    }

    /// A node by its path of child indexes from the root, `root.first.first` being `[0, 0]`.
    fn at<'s>(sheet: &'s Sheet, path: &[usize]) -> &'s Node {
        let mut index = 0;
        for &child in path {
            index = sheet.nodes[index].nodes()[child];
        }
        &sheet.nodes[index]
    }

    /// A node's start and end as `PostCSS`'s `source` gives them: the end the line and column of
    /// its last character.
    fn source(css: &str, node: &Node) -> ((usize, usize), (usize, usize)) {
        let lines = Lines::new(css);
        let end = node.end.unwrap_or_default();
        (lines.place(css, node.start), lines.place(css, end - 1))
    }

    /// The cases of `test/parse.test.js` of `postcss-scss` 4.0.9 run on its own parser, each by
    /// its title; `eachTest` of `postcss-parser-tests`, which compares whole trees printed as
    /// JSON, is not ported.
    #[test]
    fn passes_the_parse_tests_of_postcss_scss() {
        // parses nested rules.
        let css = "a { b {} }";
        let sheet = read(css);
        assert_eq!(at(&sheet, &[0, 0]).selector, "b");
        assert_eq!(source(css, at(&sheet, &[0, 0])), ((1, 5), (1, 8)));
        // parses at-rules inside rules.
        assert_eq!(at(&read("a { @media {} }"), &[0, 0]).name, "media");
        // parses variables.
        let sheet = read("$var: 1;");
        assert_eq!(
            (
                at(&sheet, &[0]).prop.as_str(),
                at(&sheet, &[0]).value.as_str()
            ),
            ("$var", "1")
        );
        // parses inline comments.
        let sheet = read("\n// a \n/* b */");
        let first = at(&sheet, &[0]);
        assert_eq!(sheet.nodes[0].nodes().len(), 2);
        assert_eq!(
            (
                first.text.as_str(),
                first.before.as_str(),
                first.inline,
                first.left.as_str(),
                first.right.as_str()
            ),
            ("a", "\n", true, " ", " ")
        );
        assert_eq!(at(&sheet, &[1]).text, "b");
        // parses empty inline comments.
        let sheet = read("//\n// ");
        let (first, last) = (at(&sheet, &[0]), at(&sheet, &[1]));
        assert_eq!(
            (
                first.text.as_str(),
                first.left.as_str(),
                first.right.as_str()
            ),
            ("", "", "")
        );
        assert_eq!(
            (
                last.text.as_str(),
                last.before.as_str(),
                last.left.as_str(),
                last.right.as_str()
            ),
            ("", "\n", " ", "")
        );
        // parses inline comments inside selector: the raw selector keeps them as written.
        assert_eq!(
            at(&read("a\n// c/**/\nb { }"), &[0]).raw.as_deref(),
            Some("a\n// c/**/\nb")
        );
        // does not parse comments inside brackets; … and spaces.
        assert_eq!(
            at(&read("a { cursor: url(http://ya.ru) }"), &[0, 0]).value,
            "url(http://ya.ru)"
        );
        assert_eq!(
            at(&read("a { cursor: url( http://ya.ru ) }"), &[0, 0]).value,
            "url( http://ya.ru )"
        );
    }

    /// The interpolation and nested-property cases of the same file.
    #[test]
    fn passes_the_interpolation_tests_of_postcss_scss() {
        // parses interpolation.
        let sheet = read("#{$selector}:hover { #{$prop}-size: #{$color} }");
        assert_eq!(at(&sheet, &[0]).selector, "#{$selector}:hover");
        assert_eq!(at(&sheet, &[0, 0]).prop, "#{$prop}-size");
        assert_eq!(at(&sheet, &[0, 0]).value, "#{$color}");
        // parses interpolation inside word; parses non-interpolation.
        assert_eq!(at(&read(".#{class} {}"), &[0]).selector, ".#{class}");
        assert_eq!(at(&read("\\#{ color: black }"), &[0]).selector, "\\#");
        // parses interpolation inside interpolation.
        assert_eq!(
            at(&read("$column: #{\"#{&}__column\"};"), &[0]).value,
            "#{\"#{&}__column\"}"
        );
        // parses interpolation that's the entire at-rule; at the beginning of, within, right
        // after an at-rule; in an at-rule's value.
        for (css, name, params) in [
            ("@#{$var} param { }", "#{$var}", "param"),
            ("@#{$var}suffix param { }", "#{$var}suffix", "param"),
            (
                "@before#{$var}after param { }",
                "before#{$var}after",
                "param",
            ),
            ("@media#{$var} { }", "media#{$var}", ""),
            ("@media #{$var} { }", "media", "#{$var}"),
        ] {
            let sheet = read(css);
            assert_eq!(
                (
                    at(&sheet, &[0]).name.as_str(),
                    at(&sheet, &[0]).params.as_str()
                ),
                (name, params)
            );
        }
        // parses interpolation in url().
        assert_eq!(
            at(&read("image: url(#{get(path)}.png)"), &[0]).value,
            "url(#{get(path)}.png)"
        );
        // parses text in rules; parses semicolon in rules.
        let sheet = read("a { margin:text { left: 10px; }}");
        assert_eq!(at(&sheet, &[0, 0]).selector, "margin:text");
        assert_eq!(at(&sheet, &[0, 0, 0]).prop, "left");
        let sheet = read("a { test(a: 1) { left: 10px; }}");
        assert_eq!(at(&sheet, &[0, 0]).selector, "test(a: 1)");
        assert_eq!(at(&sheet, &[0, 0, 0]).prop, "left");
        // parsers prefixed pseudo in rules.
        let sheet = read("input:-moz-focusring { left: 1px }");
        assert_eq!(at(&sheet, &[0]).selector, "input:-moz-focusring");
        assert_eq!(at(&sheet, &[0, 0]).prop, "left");
        // parses nested props as rule.
        let css = "a { margin: { left: 10px; }}";
        let sheet = read(css);
        assert_eq!(at(&sheet, &[0, 0]).selector, "margin:");
        assert_eq!(at(&sheet, &[0, 0, 0]).prop, "left");
        assert_eq!(source(css, at(&sheet, &[0, 0])), ((1, 5), (1, 27)));
        // parses nested props with value; with space-less digit; with important.
        for (css, between, important, end) in [
            ("a { margin: 0 { left: 10px; }}", ": ", false, 29),
            ("a { margin:0 { left: 10px; }}", ":", false, 28),
            ("a { margin: 0!important { left: 10px; }}", ": ", true, 39),
        ] {
            let sheet = read(css);
            let nested = at(&sheet, &[0, 0]);
            assert!(nested.is_nested(), "{css}");
            assert_eq!(
                (nested.prop.as_str(), nested.value.as_str()),
                ("margin", "0")
            );
            assert_eq!(
                (nested.between.as_str(), nested.important),
                (between, important)
            );
            assert_eq!(at(&sheet, &[0, 0, 0]).prop, "left");
            assert_eq!(at(&sheet, &[0, 0, 0]).value, "10px");
            assert_eq!(source(css, nested), ((1, 5), (1, end)));
        }
        // parses nested props with new line as rule.
        let css = "a { \n margin  \n:0 { left: 10px; }}";
        let sheet = read(css);
        assert_eq!(at(&sheet, &[0, 0]).selector, "margin  \n:0");
        assert_eq!(source(css, at(&sheet, &[0, 0])), ((2, 2), (3, 18)));
        // parses interpolation with variable.
        assert_eq!(at(&read("&:#{$var} {}"), &[0]).selector, "&:#{$var}");
        // parses comment inside comment: its text made printable as a CSS comment.
        assert_eq!(at(&read("a {\n//a/*b*/c\n}"), &[0, 0]).text, "a*//*b*//*c");
        // parses complex interpolation; parses interpolation inside string.
        assert_eq!(
            at(&read("content: #{fn(\"\\\"}\")};"), &[0]).value,
            "#{fn(\"\\\"}\")}"
        );
        assert_eq!(
            at(&read("content: \"#{fn(\"\\\"}\")}\";"), &[0]).value,
            "\"#{fn(\"\\\"}\")}\""
        );
    }

    /// The cases of `test/location.test.js` of `postcss-scss` 4.0.9: each node's offsets.
    #[test]
    fn passes_the_location_tests_of_postcss_scss() {
        let slice = |css: &'static str, node: &Node| &css[node.start..node.end.unwrap_or_default()];
        let css = "a{//xxx\n}";
        let sheet = read(css);
        assert_eq!(slice(css, at(&sheet, &[0])), "a{//xxx\n}");
        assert_eq!(slice(css, at(&sheet, &[0, 0])), "//xxx");
        let css = "a { margin: 0 { left: 10px; }}";
        let sheet = read(css);
        assert_eq!(slice(css, at(&sheet, &[0])), css);
        assert_eq!(slice(css, at(&sheet, &[0, 0])), "margin: 0 { left: 10px; }");
        assert_eq!(slice(css, at(&sheet, &[0, 0, 0])), "left: 10px;");
    }

    /// The error `parse` gives, its message and offset.
    fn failure(css: &str) -> Option<(String, usize)> {
        parse(css)
            .err()
            .map(|ParseError { message, offset }| (message, offset))
    }

    #[test]
    fn stops_at_the_first_error_as_postcss_throws_it() {
        let cases: [(&str, &str, usize); 11] = [
            ("a {", "Unclosed block", 0),
            ("}", "Unexpected }", 0),
            ("a { b: (c; }", "Unclosed bracket", 7),
            ("a { b }", "Unknown word b", 4),
            // After the word two words before the colon: `c`, its last byte 7, plus one.
            ("a { b: c\n d: e }", "Missed semicolon", 8),
            ("a { b:: c }", "Double colon", 6),
            ("@ { }", "At-rule without name", 0),
            ("a { b: 'c }", "Unclosed string", 7),
            ("/* a", "Unclosed comment", 0),
            ("#{a", "Unclosed interpolation", 0),
            // A word without a colon before `;`.
            ("a { b; }", "Unknown word b", 4),
        ];
        for (css, message, offset) in cases {
            assert_eq!(failure(css), Some((message.to_owned(), offset)), "{css}");
        }
        // `progid:` is no colon of a value, and a custom property's value reads anything.
        assert_eq!(failure("a { filter: progid:DXImage.x(a=1) }"), None);
        assert_eq!(failure("a { --b: { c: d } }"), None);
    }

    #[test]
    fn f53_ac2_reads_the_sheets_of_the_playground_s_syntax() {
        let css = "@use 'sass:math';\n@forward 'a' show b;\n@layer base, theme;\n$gap: 4px !default;\n%box { margin: 0; }\n@mixin pad($size) {\n  padding: $size;\n}\n.a {\n  // a note\n  @include pad(math.div($gap, 2));\n  &:hover, &.b { color: var(--c); }\n  > .d { @extend %box; }\n}\n";
        let sheet = read(css);
        let kinds: Vec<(Option<Kind>, String)> = sheet
            .walk()
            .into_iter()
            .map(|index| {
                let node = &sheet.nodes[index];
                let label = format!("{}{}{}{}", node.name, node.selector, node.prop, node.text);
                (node.kind, label)
            })
            .collect();

        assert_eq!(
            kinds,
            [
                (Some(Kind::AtRule), "use".to_owned()),
                (Some(Kind::AtRule), "forward".to_owned()),
                (Some(Kind::AtRule), "layer".to_owned()),
                (Some(Kind::Declaration), "$gap".to_owned()),
                (Some(Kind::Rule), "%box".to_owned()),
                (Some(Kind::Declaration), "margin".to_owned()),
                (Some(Kind::AtRule), "mixin".to_owned()),
                (Some(Kind::Declaration), "padding".to_owned()),
                (Some(Kind::Rule), ".a".to_owned()),
                (Some(Kind::Comment), "a note".to_owned()),
                (Some(Kind::AtRule), "include".to_owned()),
                (Some(Kind::Rule), "&:hover, &.b".to_owned()),
                (Some(Kind::Declaration), "color".to_owned()),
                (Some(Kind::Rule), "> .d".to_owned()),
                (Some(Kind::AtRule), "extend".to_owned()),
            ]
        );
        // Each node with its span: `@include pad(math.div($gap, 2));`, `;` included.
        let include = &sheet.nodes[sheet.walk()[10]];
        assert_eq!(
            &css[include.start..include.end.unwrap_or_default()],
            "@include pad(math.div($gap, 2));"
        );
        assert_eq!(include.params, "pad(math.div($gap, 2))");
        let gap = &sheet.nodes[sheet.walk()[3]];
        assert_eq!((gap.value.as_str(), gap.important), ("4px !default", false));
        let previous = sheet.previous(sheet.walk()[1]);
        assert_eq!(
            (previous, sheet.next(sheet.walk()[13])),
            (Some(sheet.walk()[0]), None)
        );
        assert_eq!(
            sheet.ancestors(sheet.walk()[14]),
            [sheet.walk()[13], sheet.walk()[8], 0]
        );
    }

    #[test]
    fn reads_important_comments_and_hacks_into_their_raws() {
        let sheet = read("a { _b: c ! important; d: e/**/f !IMPORTANT; g: h /* i */; }");
        let (first, second, third) = (
            at(&sheet, &[0, 0]),
            at(&sheet, &[0, 1]),
            at(&sheet, &[0, 2]),
        );

        // `_b`: the hack goes to `before`; `! important`, split, keeps its raw text.
        assert_eq!((first.prop.as_str(), first.before.as_str()), ("b", " _"));
        assert_eq!(
            (first.important, first.important_raw.as_deref()),
            (true, Some(" ! important"))
        );
        // A comment between words stays in the value; `!IMPORTANT` keeps its raw case.
        assert_eq!(
            (second.value.as_str(), second.important_raw.as_deref()),
            ("e/**/f", Some(" !IMPORTANT"))
        );
        // A comment at the end is left out of the value, the space before it kept, as `raw`
        // keeps every space but the last token; its raw text kept.
        assert_eq!(
            (third.value.as_str(), third.raw.as_deref()),
            ("h ", Some("h /* i */"))
        );
        // A rule's own `;` after its block, and a semicolon alone, only spaces.
        let sheet = read("a {};\n;b { c: d }");
        assert_eq!(at(&sheet, &[0]).own_semicolon.as_deref(), Some(";"));
        assert_eq!(at(&sheet, &[1]).before, "\n;");
        // An at-rule ended by the text's end, or by its parent's `}`.
        let css = "@a b ";
        let sheet = read(css);
        assert_eq!(
            (at(&sheet, &[0]).params.as_str(), at(&sheet, &[0]).end),
            ("b", Some(4))
        );
        assert_eq!(sheet.nodes[0].after, " ");
        let sheet = read("a { @b c }");
        assert_eq!(at(&sheet, &[0, 0]).end, Some(8));
        assert_eq!(at(&read("{ a: b }"), &[0]).selector, "");
    }

    #[test]
    fn refuses_blocks_nested_past_the_limit_without_overflowing() {
        let deep = |levels: usize| format!("{}{}", "a{".repeat(levels), "}".repeat(levels));

        assert!(parse(&deep(NESTING_LIMIT)).is_ok());
        let error = parse(&deep(NESTING_LIMIT + 1)).err();
        assert_eq!(
            error.map(|each| (each.message, each.offset)),
            Some((
                format!("Blocks nested more than {NESTING_LIMIT} deep"),
                2 * NESTING_LIMIT
            ))
        );
        // Deep brackets in a value nest nothing: read flat, without recursion.
        let value = format!("a {{ b: {}{}; }}", "f(".repeat(5_000), ")".repeat(5_000));
        assert!(parse(&value).is_ok());
    }
}
