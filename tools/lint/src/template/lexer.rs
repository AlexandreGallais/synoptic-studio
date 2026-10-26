//! The lexer of Angular templates: a port of `ml_parser/lexer.ts` and `ml_parser/tokens.ts` of
//! `@angular/compiler` 22.2.3 (MIT, REF-ANGULAR-ML-PARSER), with the options `parseTemplate`
//! gives it: ICU expansions, `@` blocks and `@let` read, the leading trivia (space, line feed,
//! carriage return, tab) left out of each token's start; and line endings kept as written, as
//! `@angular-eslint/template-parser` asks (`preserveLineEndings`), so that every offset of an
//! expression is the source's.
//! Left out, as `parseTemplate` never asks them of a template file: the escaped-string cursor
//! (an inline template's JavaScript escapes), a range of the source, the selectorless syntax
//! (`<MyComp>`, `@Dir(…)`), and the `_inInterpolation` flag, which the compiler never sets.
//! Offsets are bytes of the source; the compiler's `$EOF` is the character `\0`, as there, so a
//! NUL in the text ends a scan where the compiler's would. The tests are this module's own cases.

use super::entities;
use super::tags::{self, ContentType};

/// A token's kind, `TokenType` in the compiler's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenType {
    /// `<prefix:name`: parts the prefix and the name.
    TagOpenStart,
    /// `>` closing a start tag.
    TagOpenEnd,
    /// `/>` closing a start tag.
    TagOpenEndVoid,
    /// `</prefix:name>`: parts the prefix and the name.
    TagClose,
    /// A start tag cut short: parts the prefix and the name.
    IncompleteTagOpen,
    /// Text: part the text, entities and interpolations apart.
    Text,
    /// The text of a `<textarea>` or a `<title>`.
    EscapableRawText,
    /// The text of a `<script>`, a `<style>`, a comment or a CDATA section.
    RawText,
    /// `{{ expression }}` in text: parts `{{`, the expression and `}}` when closed.
    Interpolation,
    /// `&name;` or `&#…;`: parts the text it stands for and the entity as written.
    EncodedEntity,
    /// `<!--`.
    CommentStart,
    /// `-->`.
    CommentEnd,
    /// A `//` or `/* */` comment inside a start tag: parts the content and `single` or `multi`.
    InElementComment,
    /// `<![CDATA[`.
    CdataStart,
    /// `]]>`.
    CdataEnd,
    /// An attribute's name: parts the prefix and the name.
    AttrName,
    /// The quote around an attribute's value: part the quote.
    AttrQuote,
    /// An attribute value's text.
    AttrValueText,
    /// `{{ expression }}` in an attribute's value.
    AttrValueInterpolation,
    /// `<!…>`: part its content.
    DocType,
    /// `{` opening an ICU expansion.
    ExpansionFormStart,
    /// An ICU case's value (`=0`, `other`).
    ExpansionCaseValue,
    /// `{` opening an ICU case.
    ExpansionCaseExpStart,
    /// `}` closing an ICU case.
    ExpansionCaseExpEnd,
    /// `}` closing an ICU expansion.
    ExpansionFormEnd,
    /// `@name`: part the block's name (`if`, `else if`, `for`…).
    BlockOpenStart,
    /// `{` opening a block's content.
    BlockOpenEnd,
    /// `}` closing a block.
    BlockClose,
    /// A block's parameter, between `(` and `)`, up to a `;`: part its text.
    BlockParameter,
    /// A block cut short: part its name.
    IncompleteBlockOpen,
    /// `@let name`: part the name.
    LetStart,
    /// A `@let`'s value: part its text.
    LetValue,
    /// `;` ending a `@let`.
    LetEnd,
    /// A `@let` cut short: part the name read.
    IncompleteLet,
    /// `<Component` (selectorless syntax, never read here).
    ComponentOpenStart,
    /// `>` of a selectorless component (never read here).
    ComponentOpenEnd,
    /// `/>` of a selectorless component (never read here).
    ComponentOpenEndVoid,
    /// `</Component>` (never read here).
    ComponentClose,
    /// A selectorless component cut short (never read here).
    IncompleteComponentOpen,
    /// `@Directive` (never read here).
    DirectiveName,
    /// `(` of a selectorless directive (never read here).
    DirectiveOpen,
    /// `)` of a selectorless directive (never read here).
    DirectiveClose,
    /// `<?…>`: part its content.
    ProcessingInstruction,
    /// The end of the source.
    Eof,
}

/// A range of the source: `start` past the leading trivia, `full_start` before it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Span {
    /// The first byte past the leading trivia.
    pub start: u32,
    /// The byte after the range.
    pub end: u32,
    /// The first byte, trivia included.
    pub full_start: u32,
}

impl Span {
    /// A range without trivia.
    #[must_use]
    pub fn new(start: usize, end: usize) -> Self {
        let start = offset(start);
        Self {
            start,
            end: offset(end),
            full_start: start,
        }
    }
}

/// An offset as the linter's findings hold it.
#[must_use]
pub fn offset(at: usize) -> u32 {
    u32::try_from(at).unwrap_or(u32::MAX)
}

/// A token: its kind, its parts and its range.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    /// Its kind.
    pub kind: TokenType,
    /// Its parts, as the compiler gives them.
    pub parts: Vec<String>,
    /// Its range.
    pub span: Span,
}

/// A malformed template: where, and the compiler's message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    /// Where.
    pub span: Span,
    /// The compiler's message.
    pub message: String,
}

/// The tokens of a source and its errors.
#[derive(Debug)]
pub struct Tokenized {
    /// The tokens, the last one `Eof`, adjacent texts merged.
    pub tokens: Vec<Token>,
    /// The errors, in the order met.
    pub errors: Vec<ParseError>,
}

/// The compiler's `$EOF`.
const EOF: char = '\0';

/// The blocks the lexer reads after `@` (`@else` covers `@else if`).
const SUPPORTED_BLOCKS: [&str; 13] = [
    "@if",
    "@else",
    "@for",
    "@switch",
    "@case",
    "@default",
    "@empty",
    "@defer",
    "@placeholder",
    "@loading",
    "@boundary",
    "@error",
    "@content",
];

/// The characters left out of a token's start (`LEADING_TRIVIA_CHARS`).
const LEADING_TRIVIA: [char; 4] = [' ', '\n', '\r', '\t'];

/// Why a step stopped: the compiler's `CursorError` (a read past the end) or `ParseError`.
enum Fail {
    /// A read past the end, at an offset.
    Cursor(&'static str, usize),
    /// A malformed construct.
    Parse(ParseError),
}

/// A step of the lexer.
type Step<T> = Result<T, Fail>;

/// Where a run of text or of an attribute's value ends.
#[derive(Clone, Copy)]
enum End {
    /// Text: a tag, a block, a `@let`, a `}` or an ICU boundary (`_isTextEnd`).
    Text,
    /// An interpolation in text stops early at a tag (`_isTagStart`).
    TagStart,
    /// A quoted value: its quote.
    Quote(char),
    /// An unquoted value: a name's end.
    Unquoted,
}

/// What ends a raw text.
enum RawEnd<'n> {
    /// A fixed marker (`-->`, `]]>`).
    Marker(&'static str),
    /// The element's end tag, its name in any case.
    TagClose(&'n str),
}

/// The tokens and errors of a template's source.
#[must_use]
pub fn tokenize(source: &str) -> Tokenized {
    let mut lexer = Lexer {
        text: source,
        cursor: 0,
        current: None,
        expansions: Vec::new(),
        tokens: Vec::new(),
        errors: Vec::new(),
    };
    lexer.run();
    Tokenized {
        tokens: merge_text_tokens(lexer.tokens),
        errors: lexer.errors,
    }
}

/// The lexer's state (`_Tokenizer`).
struct Lexer<'t> {
    /// The source.
    text: &'t str,
    /// The offset read next.
    cursor: usize,
    /// The token begun: its kind and its start.
    current: Option<(TokenType, usize)>,
    /// The ICU expansions and cases open (`_expansionCaseStack`).
    expansions: Vec<TokenType>,
    /// The tokens ended.
    tokens: Vec<Token>,
    /// The errors.
    errors: Vec<ParseError>,
}

impl Lexer<'_> {
    /// Reads the whole source (`tokenize`).
    fn run(&mut self) {
        while self.peek() != EOF {
            let start = self.cursor;
            if let Err(fail) = self.step(start) {
                self.handle_error(fail);
            }
        }
        self.begin(TokenType::Eof, self.cursor);
        self.end_token(Vec::new());
    }

    /// Reads one construct from a start.
    fn step(&mut self, start: usize) -> Step<()> {
        if self.attempt_char('<') {
            if self.attempt_char('!') {
                if self.attempt_char('[') {
                    self.consume_cdata(start)
                } else if self.attempt_char('-') {
                    self.consume_comment(start)
                } else {
                    self.consume_doc_type(start)
                }
            } else if self.attempt_char('/') {
                self.consume_tag_close(start)
            } else if self.attempt_char('?') {
                self.consume_processing_instruction(start)
            } else {
                self.consume_tag_open(start)
            }
        } else if self.peek() == '@' && self.is_let_start() {
            self.consume_let_declaration(start)
        } else if self.is_block_start() {
            self.consume_block_start(start)
        } else if !self.in_expansion_case() && !self.in_expansion_form() && self.attempt_char('}') {
            self.begin(TokenType::BlockClose, start);
            self.end_token(Vec::new());
            Ok(())
        } else if self.tokenize_expansion_form()? {
            Ok(())
        } else {
            self.consume_with_interpolation(
                TokenType::Text,
                TokenType::Interpolation,
                End::Text,
                End::TagStart,
            )
        }
    }

    // The cursor.

    /// The character at the cursor, `EOF` at the end.
    fn peek(&self) -> char {
        self.text
            .get(self.cursor..)
            .and_then(|rest| rest.chars().next())
            .unwrap_or(EOF)
    }

    /// The character after the cursor's.
    fn peek_next(&self) -> char {
        let mut rest = self.text.get(self.cursor..).unwrap_or_default().chars();
        rest.next();
        rest.next().unwrap_or(EOF)
    }

    /// Moves past the character at the cursor; a read past the end fails as the compiler's.
    fn advance(&mut self) -> Step<()> {
        if self.cursor >= self.text.len() {
            return Err(Fail::Cursor("Unexpected character \"EOF\"", self.cursor));
        }
        self.cursor += self.peek().len_utf8();
        Ok(())
    }

    /// The text between an offset and the cursor.
    fn chars_from(&self, start: usize) -> &str {
        self.text.get(start..self.cursor).unwrap_or_default()
    }

    /// A range from an offset to the cursor, its leading trivia left out when asked.
    fn span_from(&self, start: usize, trivia: bool) -> Span {
        let mut first = start;
        if trivia {
            first += self
                .chars_from(start)
                .chars()
                .take_while(|each| LEADING_TRIVIA.contains(each))
                .map(char::len_utf8)
                .sum::<usize>();
        }
        Span {
            start: offset(first),
            end: offset(self.cursor),
            full_start: offset(start),
        }
    }

    // Tokens and errors.

    /// Begins a token at an offset.
    fn begin(&mut self, kind: TokenType, start: usize) {
        self.current = Some((kind, start));
    }

    /// Begins a token at the cursor.
    fn begin_here(&mut self, kind: TokenType) {
        self.begin(kind, self.cursor);
    }

    /// Ends the token begun at the cursor; its index.
    fn end_token(&mut self, parts: Vec<String>) -> usize {
        self.end_token_at(parts, self.cursor)
    }

    /// Ends the token begun at an offset; its index.
    fn end_token_at(&mut self, parts: Vec<String>, end: usize) -> usize {
        let (kind, start) = self.current.take().unwrap_or((TokenType::Text, end));
        let here = self.cursor;
        self.cursor = end;
        let span = self.span_from(start, true);
        self.cursor = here;
        self.tokens.push(Token { kind, parts, span });
        self.tokens.len() - 1
    }

    /// An error, the token begun dropped (`_createError`).
    fn create_error(&mut self, message: String, span: Span) -> Fail {
        let mut message = message;
        if self.in_expansion_form() {
            message.push_str(
                " (Do you have an unescaped \"{\" in your template? Use \"{{ '{' }}\") to escape it.)",
            );
        }
        self.current = None;
        Fail::Parse(ParseError { span, message })
    }

    /// The error of an unexpected character at the cursor, spanning from an offset.
    fn unexpected(&mut self, start: usize) -> Fail {
        let message = unexpected_character(self.peek());
        let span = self.span_from(start, false);
        self.create_error(message, span)
    }

    /// Records an error (`handleError`).
    fn handle_error(&mut self, fail: Fail) {
        let error = match fail {
            Fail::Cursor(message, at) => {
                let span = Span::new(at.min(self.cursor), self.cursor);
                match self.create_error(message.to_owned(), span) {
                    Fail::Parse(error) => error,
                    Fail::Cursor(..) => return,
                }
            }
            Fail::Parse(error) => error,
        };
        self.errors.push(error);
    }

    // Reading.

    /// Moves past a character when it is the one at the cursor.
    fn attempt_char(&mut self, expected: char) -> bool {
        if self.peek() == expected && self.cursor < self.text.len() {
            self.cursor += expected.len_utf8();
            true
        } else {
            false
        }
    }

    /// Moves past a character in any ASCII case; no move otherwise.
    fn attempt_char_case_insensitive(&mut self, expected: char) -> bool {
        let found = self.peek();
        if found != EOF && found.eq_ignore_ascii_case(&expected) {
            self.cursor += found.len_utf8();
            true
        } else {
            false
        }
    }

    /// Requires a character at the cursor.
    fn require_char(&mut self, expected: char) -> Step<()> {
        let location = self.cursor;
        if self.attempt_char(expected) {
            Ok(())
        } else {
            Err(self.unexpected(location))
        }
    }

    /// Moves past a string when the source goes on with it; no move otherwise.
    fn attempt_str(&mut self, expected: &str) -> bool {
        if self
            .text
            .get(self.cursor..)
            .is_some_and(|rest| rest.starts_with(expected))
        {
            self.cursor += expected.len();
            true
        } else {
            false
        }
    }

    /// Moves past a string in any ASCII case, as far as it matches.
    fn attempt_str_case_insensitive(&mut self, expected: &str) -> bool {
        expected
            .chars()
            .all(|each| self.attempt_char_case_insensitive(each))
    }

    /// Requires a string at the cursor.
    fn require_str(&mut self, expected: &str) -> Step<()> {
        let location = self.cursor;
        if self.attempt_str(expected) {
            Ok(())
        } else {
            Err(self.unexpected(location))
        }
    }

    /// Moves until a predicate holds on the character at the cursor.
    fn attempt_until(&mut self, mut predicate: impl FnMut(char) -> bool) -> Step<()> {
        while !predicate(self.peek()) {
            self.advance()?;
        }
        Ok(())
    }

    /// Moves past the whitespace (`_attemptCharCodeUntilFn(isNotWhitespace)`).
    fn skip_whitespace(&mut self) -> Step<()> {
        self.attempt_until(is_not_whitespace)
    }

    /// Moves until a predicate holds, at least some characters.
    fn require_until(&mut self, predicate: impl FnMut(char) -> bool, length: usize) -> Step<()> {
        let start = self.cursor;
        self.attempt_until(predicate)?;
        if self.chars_from(start).chars().count() < length {
            return Err(self.unexpected(start));
        }
        Ok(())
    }

    /// Moves until a character.
    fn attempt_until_char(&mut self, expected: char) -> Step<()> {
        self.attempt_until(|each| each == expected)
    }

    /// Moves until a predicate holds outside quotes, a backslash escaping the next character.
    fn attempt_until_ignore_quotes(&mut self, predicate: impl Fn(char) -> bool) -> Step<()> {
        while self.peek() != EOF {
            let character = self.peek();
            if predicate(character) {
                break;
            }
            if is_quote(character) {
                self.advance()?;
                loop {
                    let inner = self.peek();
                    if inner == '\\' {
                        self.advance()?;
                    } else if inner == character {
                        break;
                    }
                    self.advance()?;
                }
            }
            self.advance()?;
        }
        Ok(())
    }

    /// The character at the cursor, moved past.
    fn read_char(&mut self) -> Step<char> {
        let character = self.peek();
        self.advance()?;
        Ok(character)
    }

    /// The text up to a character.
    fn read_until(&mut self, expected: char) -> Step<String> {
        let start = self.cursor;
        self.attempt_until_char(expected)?;
        Ok(self.chars_from(start).to_owned())
    }

    /// Whether the source goes on with a string.
    fn peek_str(&self, expected: &str) -> bool {
        self.text
            .get(self.cursor..)
            .is_some_and(|rest| rest.starts_with(expected))
    }

    // Blocks and `@let`.

    /// Whether a supported block starts at the cursor.
    fn is_block_start(&self) -> bool {
        self.peek() == '@' && SUPPORTED_BLOCKS.iter().any(|name| self.peek_str(name))
    }

    /// Whether a `@let` starts at the cursor.
    fn is_let_start(&self) -> bool {
        self.peek() == '@' && self.peek_str("@let")
    }

    /// A block's name, `else if` and `default never` with their spaces normalised.
    fn block_name(&mut self) -> Step<String> {
        let mut spaces_allowed = false;
        let start = self.cursor;
        self.attempt_until(|code| {
            if is_whitespace(code) {
                return !spaces_allowed;
            }
            if is_block_name_char(code) {
                spaces_allowed = true;
                return false;
            }
            true
        })?;
        let name = self.chars_from(start).trim();
        Ok(if two_words(name, "else", "if") {
            "else if".to_owned()
        } else if two_words(name, "default", "never") {
            "default never".to_owned()
        } else {
            name.to_owned()
        })
    }

    /// `@name (parameters) {`.
    fn consume_block_start(&mut self, start: usize) -> Step<()> {
        self.require_char('@')?;
        self.begin(TokenType::BlockOpenStart, start);
        let name = self.block_name()?;
        let opening = self.end_token(vec![name.clone()]);

        if self.peek() == '(' {
            self.advance()?;
            self.consume_block_parameters()?;
            self.skip_whitespace()?;
            if self.attempt_char(')') {
                self.skip_whitespace()?;
            } else {
                self.tokens[opening].kind = TokenType::IncompleteBlockOpen;
                return Ok(());
            }
        }

        if name == "default never" && self.attempt_char(';') {
            self.begin_here(TokenType::BlockOpenEnd);
            self.end_token(Vec::new());
            self.begin_here(TokenType::BlockClose);
            self.end_token(Vec::new());
            return Ok(());
        }

        if self.attempt_char('{') {
            self.begin_here(TokenType::BlockOpenEnd);
            self.end_token(Vec::new());
        } else if self.is_block_start() && (name == "case" || name == "default") {
            // Consecutive `@case`s share the content of the last one.
            self.begin_here(TokenType::BlockOpenEnd);
            self.end_token(Vec::new());
            self.begin_here(TokenType::BlockClose);
            self.end_token(Vec::new());
        } else {
            self.tokens[opening].kind = TokenType::IncompleteBlockOpen;
        }
        Ok(())
    }

    /// A block's parameters, each up to a `;` outside quotes and parentheses.
    fn consume_block_parameters(&mut self) -> Step<()> {
        self.attempt_until(is_block_parameter_char)?;
        while self.peek() != ')' && self.peek() != EOF {
            self.begin_here(TokenType::BlockParameter);
            let start = self.cursor;
            let mut quote: Option<char> = None;
            let mut open_parens = 0_usize;

            while (self.peek() != ';' && self.peek() != EOF) || quote.is_some() {
                let character = self.peek();
                if character == '\\' {
                    self.advance()?;
                } else if Some(character) == quote {
                    quote = None;
                } else if quote.is_none() && is_quote(character) {
                    quote = Some(character);
                } else if character == '(' && quote.is_none() {
                    open_parens += 1;
                } else if character == ')' && quote.is_none() {
                    if open_parens == 0 {
                        break;
                    }
                    open_parens -= 1;
                }
                self.advance()?;
            }

            let parameter = self.chars_from(start).to_owned();
            self.end_token(vec![parameter]);
            self.attempt_until(is_block_parameter_char)?;
        }
        Ok(())
    }

    /// `@let name = value;`.
    fn consume_let_declaration(&mut self, start: usize) -> Step<()> {
        self.require_str("@let")?;
        self.begin(TokenType::LetStart, start);

        if is_whitespace(self.peek()) {
            self.skip_whitespace()?;
        } else {
            let written = self.chars_from(start).to_owned();
            let token = self.end_token(vec![written]);
            self.tokens[token].kind = TokenType::IncompleteLet;
            return Ok(());
        }

        let name = self.let_name()?;
        let opening = self.end_token(vec![name]);
        self.skip_whitespace()?;
        if !self.attempt_char('=') {
            self.tokens[opening].kind = TokenType::IncompleteLet;
            return Ok(());
        }
        self.attempt_until(|code| is_not_whitespace(code) && !is_new_line(code))?;

        let value_start = self.cursor;
        self.begin(TokenType::LetValue, value_start);
        self.attempt_until_ignore_quotes(|code| code == ';')?;
        let value = self.chars_from(value_start).to_owned();
        self.end_token(vec![value]);

        if self.peek() == ';' {
            self.begin_here(TokenType::LetEnd);
            self.advance()?;
            self.end_token(Vec::new());
        } else {
            self.tokens[opening].kind = TokenType::IncompleteLet;
            self.tokens[opening].span = self.span_from(start, false);
        }
        Ok(())
    }

    /// A `@let`'s name: letters, `$`, `_`, and digits after the first.
    fn let_name(&mut self) -> Step<String> {
        let start = self.cursor;
        let mut allow_digit = false;
        self.attempt_until(|code| {
            if code.is_ascii_alphabetic()
                || code == '$'
                || code == '_'
                || (allow_digit && code.is_ascii_digit())
            {
                allow_digit = true;
                return false;
            }
            true
        })?;
        Ok(self.chars_from(start).trim().to_owned())
    }

    // ICU expansions.

    /// Reads an ICU boundary when one is at the cursor; whether it did.
    fn tokenize_expansion_form(&mut self) -> Step<bool> {
        if self.is_expansion_form_start() {
            self.consume_expansion_form_start()?;
            return Ok(true);
        }
        if self.peek() != '}' && self.in_expansion_form() {
            self.consume_expansion_case_start()?;
            return Ok(true);
        }
        if self.peek() == '}' {
            if self.in_expansion_case() {
                self.begin_here(TokenType::ExpansionCaseExpEnd);
                self.require_char('}')?;
                self.end_token(Vec::new());
                self.skip_whitespace()?;
                self.expansions.pop();
                return Ok(true);
            }
            if self.in_expansion_form() {
                self.begin_here(TokenType::ExpansionFormEnd);
                self.require_char('}')?;
                self.end_token(Vec::new());
                self.expansions.pop();
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// `{value, type,`.
    fn consume_expansion_form_start(&mut self) -> Step<()> {
        self.begin_here(TokenType::ExpansionFormStart);
        self.require_char('{')?;
        self.end_token(Vec::new());
        self.expansions.push(TokenType::ExpansionFormStart);

        self.begin_here(TokenType::RawText);
        let condition = self.read_until(',')?;
        self.end_token(vec![condition]);
        self.require_char(',')?;
        self.skip_whitespace()?;

        self.begin_here(TokenType::RawText);
        let kind = self.read_until(',')?;
        self.end_token(vec![kind]);
        self.require_char(',')?;
        self.skip_whitespace()
    }

    /// `value {`.
    fn consume_expansion_case_start(&mut self) -> Step<()> {
        self.begin_here(TokenType::ExpansionCaseValue);
        let value = self.read_until('{')?.trim().to_owned();
        self.end_token(vec![value]);
        self.skip_whitespace()?;

        self.begin_here(TokenType::ExpansionCaseExpStart);
        self.require_char('{')?;
        self.end_token(Vec::new());
        self.skip_whitespace()?;
        self.expansions.push(TokenType::ExpansionCaseExpStart);
        Ok(())
    }

    /// Whether an ICU case is the innermost open.
    fn in_expansion_case(&self) -> bool {
        self.expansions.last() == Some(&TokenType::ExpansionCaseExpStart)
    }

    /// Whether an ICU expansion is the innermost open.
    fn in_expansion_form(&self) -> bool {
        self.expansions.last() == Some(&TokenType::ExpansionFormStart)
    }

    /// Whether a `{` not opening an interpolation is at the cursor.
    fn is_expansion_form_start(&self) -> bool {
        self.peek() == '{' && !self.peek_str("{{")
    }

    // Text, interpolations and entities.

    /// Whether a run of text or of a value ends at the cursor.
    fn at_end(&self, end: End) -> bool {
        match end {
            End::Text => self.is_text_end(),
            End::TagStart => self.is_tag_start(),
            End::Quote(quote) => self.peek() == quote,
            End::Unquoted => is_name_end(self.peek()),
        }
    }

    /// Whether text ends at the cursor (`_isTextEnd`).
    fn is_text_end(&self) -> bool {
        if self.is_tag_start() || self.peek() == EOF {
            return true;
        }
        if self.is_expansion_form_start() || (self.peek() == '}' && self.in_expansion_case()) {
            return true;
        }
        !self.in_expansion_case()
            && !self.in_expansion_form()
            && (self.is_block_start() || self.is_let_start() || self.peek() == '}')
    }

    /// Whether a tag, a comment or a declaration starts at the cursor.
    fn is_tag_start(&self) -> bool {
        self.peek() == '<' && {
            let next = self.peek_next();
            next.is_ascii_alphabetic() || matches!(next, '/' | '!' | '?')
        }
    }

    /// Text with its interpolations and entities, up to its end.
    fn consume_with_interpolation(
        &mut self,
        text_kind: TokenType,
        interpolation_kind: TokenType,
        end: End,
        interpolation_end: End,
    ) -> Step<()> {
        self.begin_here(text_kind);
        let mut parts = String::new();
        while !self.at_end(end) {
            let current = self.cursor;
            if self.attempt_str("{{") {
                self.end_token_at(vec![std::mem::take(&mut parts)], current);
                self.consume_interpolation(interpolation_kind, current, interpolation_end)?;
                self.begin_here(text_kind);
            } else if self.peek() == '&' {
                self.end_token(vec![std::mem::take(&mut parts)]);
                self.consume_entity(text_kind)?;
                self.begin_here(text_kind);
            } else {
                parts.push(self.read_char()?);
            }
        }
        self.end_token(vec![parts]);
        Ok(())
    }

    /// `{{ expression }}`, its end a `}}` outside quotes, or cut short by a tag or an end.
    fn consume_interpolation(&mut self, kind: TokenType, start: usize, premature: End) -> Step<()> {
        let mut parts = vec!["{{".to_owned()];
        self.begin(kind, start);
        let expression_start = self.cursor;
        let mut quote: Option<char> = None;
        let mut in_comment = false;

        while self.peek() != EOF && !self.at_end(premature) {
            let current = self.cursor;
            if self.is_tag_start() {
                parts.push(
                    self.text
                        .get(expression_start..current)
                        .unwrap_or_default()
                        .to_owned(),
                );
                self.end_token(parts);
                return Ok(());
            }
            if quote.is_none() {
                if self.attempt_str("}}") {
                    parts.push(
                        self.text
                            .get(expression_start..current)
                            .unwrap_or_default()
                            .to_owned(),
                    );
                    parts.push("}}".to_owned());
                    self.end_token(parts);
                    return Ok(());
                } else if self.attempt_str("//") {
                    in_comment = true;
                }
            }
            let character = self.peek();
            self.advance()?;
            if character == '\\' {
                self.advance()?;
            } else if Some(character) == quote {
                quote = None;
            } else if !in_comment && quote.is_none() && is_quote(character) {
                quote = Some(character);
            }
        }
        parts.push(self.chars_from(expression_start).to_owned());
        self.end_token(parts);
        Ok(())
    }

    /// `&name;`, `&#decimal;` or `&#xhex;`; a `&` not ending in `;` is text.
    fn consume_entity(&mut self, text_kind: TokenType) -> Step<()> {
        self.begin_here(TokenType::EncodedEntity);
        let start = self.cursor;
        self.advance()?;
        if self.attempt_char('#') {
            let is_hex = self.attempt_char('x') || self.attempt_char('X');
            let code_start = self.cursor;
            self.attempt_until(is_digit_entity_end)?;
            if self.peek() != ';' {
                self.advance()?;
                let kind = if is_hex { "hexadecimal" } else { "decimal" };
                let message = format!(
                    "Unable to parse entity \"{}\" - {kind} character reference entities must end with \";\"",
                    self.chars_from(start)
                );
                let span = Span::new(self.cursor, self.cursor);
                return Err(self.create_error(message, span));
            }
            let digits = self.chars_from(code_start).to_owned();
            self.advance()?;
            let Some(character) = code_point(&digits, if is_hex { 16 } else { 10 }) else {
                let message = unknown_entity(self.chars_from(start));
                let span = Span::new(self.cursor, self.cursor);
                return Err(self.create_error(message, span));
            };
            let written = self.chars_from(start).to_owned();
            self.end_token(vec![character.to_string(), written]);
        } else {
            let name_start = self.cursor;
            self.attempt_until(is_named_entity_end)?;
            if self.peek() == ';' {
                let name = self.chars_from(name_start).to_owned();
                self.advance()?;
                let Some(text) = entities::named(&name) else {
                    let span = self.span_from(start, false);
                    return Err(self.create_error(unknown_entity(&name), span));
                };
                self.end_token(vec![text, format!("&{name};")]);
            } else {
                self.begin(text_kind, start);
                self.cursor = name_start;
                self.end_token(vec!["&".to_owned()]);
            }
        }
        Ok(())
    }

    /// Text up to an end marker, its entities read when the element's content is escapable.
    fn consume_raw_text(&mut self, consume_entities: bool, end: &RawEnd) -> Step<()> {
        let kind = if consume_entities {
            TokenType::EscapableRawText
        } else {
            TokenType::RawText
        };
        self.begin_here(kind);
        let mut parts = String::new();
        loop {
            let tag_close_start = self.cursor;
            let found = self.raw_end(end);
            self.cursor = tag_close_start;
            if found {
                break;
            }
            if consume_entities && self.peek() == '&' {
                self.end_token(vec![std::mem::take(&mut parts)]);
                self.consume_entity(TokenType::EscapableRawText)?;
                self.begin_here(TokenType::EscapableRawText);
            } else {
                parts.push(self.read_char()?);
            }
        }
        self.end_token(vec![parts]);
        Ok(())
    }

    /// Whether a raw text's end is at the cursor, the cursor moved past what matched.
    fn raw_end(&mut self, end: &RawEnd) -> bool {
        match end {
            RawEnd::Marker(marker) => self.attempt_str(marker),
            RawEnd::TagClose(name) => {
                if !self.attempt_char('<') || !self.attempt_char('/') {
                    return false;
                }
                self.skip_spaces();
                if !self.attempt_str_case_insensitive(name) {
                    return false;
                }
                self.skip_spaces();
                self.attempt_char('>')
            }
        }
    }

    /// Moves past whitespace, never past the end.
    fn skip_spaces(&mut self) {
        while is_whitespace(self.peek()) {
            self.cursor += self.peek().len_utf8();
        }
    }

    // Comments and declarations.

    /// `<!-- … -->`.
    fn consume_comment(&mut self, start: usize) -> Step<()> {
        self.begin(TokenType::CommentStart, start);
        self.require_char('-')?;
        self.end_token(Vec::new());
        self.consume_raw_text(false, &RawEnd::Marker("-->"))?;
        self.begin_here(TokenType::CommentEnd);
        self.require_str("-->")?;
        self.end_token(Vec::new());
        Ok(())
    }

    /// `<![CDATA[ … ]]>`.
    fn consume_cdata(&mut self, start: usize) -> Step<()> {
        self.begin(TokenType::CdataStart, start);
        self.require_str("CDATA[")?;
        self.end_token(Vec::new());
        self.consume_raw_text(false, &RawEnd::Marker("]]>"))?;
        self.begin_here(TokenType::CdataEnd);
        self.require_str("]]>")?;
        self.end_token(Vec::new());
        Ok(())
    }

    /// `<!…>`.
    fn consume_doc_type(&mut self, start: usize) -> Step<()> {
        self.begin(TokenType::DocType, start);
        let content_start = self.cursor;
        self.attempt_until_char('>')?;
        let content = self.chars_from(content_start).to_owned();
        self.advance()?;
        self.end_token(vec![content]);
        Ok(())
    }

    /// `<?…?>`.
    fn consume_processing_instruction(&mut self, start: usize) -> Step<()> {
        self.begin(TokenType::ProcessingInstruction, start);
        let content_start = self.cursor;
        self.attempt_until_ignore_quotes(|code| code == '?' || code == '>')?;
        let end = self.peek();
        let content = self.chars_from(content_start).to_owned();
        self.advance()?;
        if end == '?' {
            self.require_char('>')?;
        }
        self.end_token(vec![content]);
        Ok(())
    }

    /// A `//` comment inside a start tag, up to the line's end.
    fn consume_single_line_comment(&mut self, start: usize) -> Step<()> {
        let content_start = self.cursor;
        self.attempt_until(|code| is_new_line(code) || code == EOF)?;
        let span_end = self.cursor;
        let content = self.chars_from(content_start).to_owned();
        self.begin(TokenType::InElementComment, start);
        self.end_token_at(vec![content, "single".to_owned()], span_end);
        self.skip_whitespace()
    }

    /// A `/* */` comment inside a start tag.
    fn consume_multi_line_comment(&mut self, start: usize) -> Step<()> {
        let content_start = self.cursor;
        while !(self.peek() == EOF || (self.peek() == '*' && self.peek_next() == '/')) {
            self.advance()?;
        }
        let content = self.chars_from(content_start).to_owned();
        let mut span_end = self.cursor;
        if self.attempt_str("*/") {
            span_end = self.cursor;
            self.skip_whitespace()?;
        }
        self.begin(TokenType::InElementComment, start);
        self.end_token_at(vec![content, "multi".to_owned()], span_end);
        Ok(())
    }

    // Tags and attributes.

    /// A name with its optional prefix, `prefix:name`, up to an end.
    fn consume_prefix_and_name(&mut self, end: impl FnMut(char) -> bool) -> Step<Vec<String>> {
        let start = self.cursor;
        let mut prefix = String::new();
        while self.peek() != ':' && !is_prefix_end(self.peek()) {
            self.advance()?;
        }
        let name_start = if self.peek() == ':' {
            self.chars_from(start).clone_into(&mut prefix);
            self.advance()?;
            self.cursor
        } else {
            start
        };
        self.require_until(end, usize::from(!prefix.is_empty()))?;
        let name = self.chars_from(name_start).to_owned();
        Ok(vec![prefix, name])
    }

    /// A start tag, then the raw text of a `<script>`, `<style>`, `<textarea>` or `<title>`.
    fn consume_tag_open(&mut self, start: usize) -> Step<()> {
        let mut opening = None;
        match self.tag_open_body(start, &mut opening) {
            Ok((prefix, name)) => {
                let Some(opening) = opening else {
                    return Ok(());
                };
                match tags::definition(&name).content_type(&prefix) {
                    ContentType::RawText => {
                        self.consume_raw_text_with_tag_close(opening, &name, false)
                    }
                    ContentType::EscapableRawText | ContentType::EscapableRawTextOutsideSvg => {
                        self.consume_raw_text_with_tag_close(opening, &name, true)
                    }
                    ContentType::ParsableData => Ok(()),
                }
            }
            Err(Fail::Parse(_)) => {
                if let Some(opening) = opening {
                    // An error before the tag closed: the tag is incomplete.
                    self.tokens[opening].kind = TokenType::IncompleteTagOpen;
                } else {
                    // An invalid start tag is read as a `<` of text, merged with its neighbours.
                    self.begin(TokenType::Text, start);
                    self.end_token(vec!["<".to_owned()]);
                }
                Ok(())
            }
            Err(fail) => Err(fail),
        }
    }

    /// A start tag's name, attributes and end; its prefix and name.
    fn tag_open_body(
        &mut self,
        start: usize,
        opening: &mut Option<usize>,
    ) -> Step<(String, String)> {
        if !self.peek().is_ascii_alphabetic() {
            return Err(self.unexpected(start));
        }
        self.begin(TokenType::TagOpenStart, start);
        let parts = self.consume_prefix_and_name(is_name_end)?;
        let (prefix, name) = (parts[0].clone(), parts[1].clone());
        *opening = Some(self.end_token(parts));
        self.skip_whitespace()?;

        loop {
            let comment_start = self.cursor;
            if self.attempt_str("//") {
                self.consume_single_line_comment(comment_start)?;
                continue;
            }
            if self.attempt_str("/*") {
                self.consume_multi_line_comment(comment_start)?;
                continue;
            }
            if is_attribute_terminator(self.peek()) {
                break;
            }
            self.consume_attribute()?;
        }

        let kind = if self.attempt_char('/') {
            TokenType::TagOpenEndVoid
        } else {
            TokenType::TagOpenEnd
        };
        self.begin_here(kind);
        self.require_char('>')?;
        self.end_token(Vec::new());
        Ok((prefix, name))
    }

    /// The raw text of an element and its end tag.
    fn consume_raw_text_with_tag_close(
        &mut self,
        opening: usize,
        name: &str,
        consume_entities: bool,
    ) -> Step<()> {
        self.consume_raw_text(consume_entities, &RawEnd::TagClose(name))?;
        self.begin_here(TokenType::TagClose);
        self.require_until(|code| code == '>', 3)?;
        self.advance()?;
        let parts = self.tokens[opening].parts.clone();
        self.end_token(parts);
        Ok(())
    }

    /// An attribute: its name, then `=` and its value when written.
    fn consume_attribute(&mut self) -> Step<()> {
        self.consume_attribute_name()?;
        self.skip_whitespace()?;
        if self.attempt_char('=') {
            self.skip_whitespace()?;
            self.consume_attribute_value()?;
        }
        self.skip_whitespace()
    }

    /// An attribute's name; a `[…]` name may hold any character but a line end inside brackets.
    fn consume_attribute_name(&mut self) -> Step<()> {
        let first = self.peek();
        if first == '\'' || first == '"' {
            let message = unexpected_character(first);
            let span = Span::new(self.cursor, self.cursor);
            return Err(self.create_error(message, span));
        }
        self.begin_here(TokenType::AttrName);
        let parts = if first == '[' {
            let mut open_brackets = 0_i32;
            self.consume_prefix_and_name(|code| {
                if code == '[' {
                    open_brackets += 1;
                } else if code == ']' {
                    open_brackets -= 1;
                }
                if open_brackets <= 0 {
                    is_name_end(code)
                } else {
                    is_new_line(code)
                }
            })?
        } else {
            self.consume_prefix_and_name(is_name_end)?
        };
        self.end_token(parts);
        Ok(())
    }

    /// An attribute's value, quoted or not, with its interpolations and entities.
    fn consume_attribute_value(&mut self) -> Step<()> {
        let quote = self.peek();
        if quote == '\'' || quote == '"' {
            self.consume_quote(quote)?;
            self.consume_with_interpolation(
                TokenType::AttrValueText,
                TokenType::AttrValueInterpolation,
                End::Quote(quote),
                End::Quote(quote),
            )?;
            self.consume_quote(quote)
        } else {
            self.consume_with_interpolation(
                TokenType::AttrValueText,
                TokenType::AttrValueInterpolation,
                End::Unquoted,
                End::Unquoted,
            )
        }
    }

    /// The quote around a value.
    fn consume_quote(&mut self, quote: char) -> Step<()> {
        self.begin_here(TokenType::AttrQuote);
        self.require_char(quote)?;
        self.end_token(vec![quote.to_string()]);
        Ok(())
    }

    /// `</prefix:name>`.
    fn consume_tag_close(&mut self, start: usize) -> Step<()> {
        self.begin(TokenType::TagClose, start);
        self.skip_whitespace()?;
        let parts = self.consume_prefix_and_name(is_name_end)?;
        self.skip_whitespace()?;
        self.require_char('>')?;
        self.end_token(parts);
        Ok(())
    }
}

/// The compiler's message for an unexpected character.
fn unexpected_character(character: char) -> String {
    if character == EOF {
        "Unexpected character \"EOF\"".to_owned()
    } else {
        format!("Unexpected character \"{character}\"")
    }
}

/// The compiler's message for an unknown entity.
fn unknown_entity(entity: &str) -> String {
    format!("Unknown entity \"{entity}\" - use the \"&#<decimal>;\" or  \"&#x<hex>;\" syntax")
}

/// The character of a numeric reference, read as `parseInt` reads its leading digits; a lone
/// surrogate, which JavaScript's string holds and Rust's does not, as U+FFFD.
fn code_point(digits: &str, radix: u32) -> Option<char> {
    let leading: String = digits
        .chars()
        .take_while(|each| each.is_digit(radix))
        .collect();
    let value = u32::from_str_radix(&leading, radix).ok()?;
    if (0xD800..=0xDFFF).contains(&value) {
        return Some(char::REPLACEMENT_CHARACTER);
    }
    char::from_u32(value)
}

/// Whether a name is two words split by spaces other than line ends (`ELSE_IF_PATTERN`).
fn two_words(name: &str, first: &str, second: &str) -> bool {
    let Some(rest) = name.strip_prefix(first) else {
        return false;
    };
    let after = rest.trim_start_matches(|each: char| each.is_whitespace() && !is_new_line(each));
    after.len() < rest.len() && after.starts_with(second)
}

/// The compiler's whitespace: tab to space, and the no-break space.
fn is_whitespace(code: char) -> bool {
    ('\t'..=' ').contains(&code) || code == '\u{a0}'
}

/// Not whitespace, or the end.
fn is_not_whitespace(code: char) -> bool {
    !is_whitespace(code) || code == EOF
}

/// A line feed or a carriage return.
fn is_new_line(code: char) -> bool {
    code == '\n' || code == '\r'
}

/// A quote of JavaScript: `'`, `"` or a backtick.
fn is_quote(code: char) -> bool {
    matches!(code, '\'' | '"' | '`')
}

/// The end of a tag or attribute name.
fn is_name_end(code: char) -> bool {
    is_whitespace(code) || matches!(code, '>' | '<' | '/' | '\'' | '"' | '=' | EOF)
}

/// The end of a name's prefix: anything but an ASCII letter or digit.
fn is_prefix_end(code: char) -> bool {
    !code.is_ascii_alphanumeric()
}

/// The end of a numeric reference's digits.
fn is_digit_entity_end(code: char) -> bool {
    code == ';' || code == EOF || !code.is_ascii_hexdigit()
}

/// The end of a named reference's name.
fn is_named_entity_end(code: char) -> bool {
    code == ';' || code == EOF || !code.is_ascii_alphanumeric()
}

/// A character of a block's name.
fn is_block_name_char(code: char) -> bool {
    code.is_ascii_alphanumeric() || code == '_'
}

/// A character starting a block's parameter: not `;` nor whitespace.
fn is_block_parameter_char(code: char) -> bool {
    code != ';' && is_not_whitespace(code)
}

/// The end of a start tag's attributes.
fn is_attribute_terminator(code: char) -> bool {
    matches!(code, '/' | '>' | '<' | EOF)
}

/// Adjacent texts, and adjacent value texts, merged into one token (`mergeTextTokens`).
fn merge_text_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut merged: Vec<Token> = Vec::with_capacity(tokens.len());
    for token in tokens {
        if let Some(last) = merged.last_mut()
            && last.kind == token.kind
            && matches!(token.kind, TokenType::Text | TokenType::AttrValueText)
        {
            let text = token.parts.first().cloned().unwrap_or_default();
            if let Some(first) = last.parts.first_mut() {
                first.push_str(&text);
            }
            last.span.end = token.span.end;
            continue;
        }
        merged.push(token);
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::TokenType::{
        AttrName, AttrQuote, AttrValueInterpolation, AttrValueText, BlockClose, BlockOpenEnd,
        BlockOpenStart, BlockParameter, CdataEnd, CdataStart, CommentEnd, CommentStart, DocType,
        EncodedEntity, Eof, EscapableRawText, ExpansionCaseExpEnd, ExpansionCaseExpStart,
        ExpansionCaseValue, ExpansionFormEnd, ExpansionFormStart, InElementComment,
        IncompleteBlockOpen, IncompleteLet, IncompleteTagOpen, Interpolation, LetEnd, LetStart,
        LetValue, ProcessingInstruction, RawText, TagClose, TagOpenEnd, TagOpenEndVoid,
        TagOpenStart, Text,
    };
    use super::{TokenType, tokenize};

    /// Each token's kind and parts, as the compiler's tests humanise them.
    fn parts(source: &str) -> Vec<(TokenType, Vec<String>)> {
        tokenize(source)
            .tokens
            .into_iter()
            .map(|token| (token.kind, token.parts))
            .collect()
    }

    /// A kind and its parts, written short.
    fn t(kind: TokenType, parts: &[&str]) -> (TokenType, Vec<String>) {
        (kind, parts.iter().map(|each| (*each).to_owned()).collect())
    }

    /// Each error's message and its range.
    fn errors(source: &str) -> Vec<(String, u32, u32)> {
        tokenize(source)
            .errors
            .into_iter()
            .map(|error| (error.message, error.span.start, error.span.end))
            .collect()
    }

    #[test]
    fn reads_tags_attributes_and_their_values() {
        assert_eq!(
            parts("<a href=\"x\" b='y' c=z d>t</a><br/>"),
            [
                t(TagOpenStart, &["", "a"]),
                t(AttrName, &["", "href"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["x"]),
                t(AttrQuote, &["\""]),
                t(AttrName, &["", "b"]),
                t(AttrQuote, &["'"]),
                t(AttrValueText, &["y"]),
                t(AttrQuote, &["'"]),
                t(AttrName, &["", "c"]),
                t(AttrValueText, &["z"]),
                t(AttrName, &["", "d"]),
                t(TagOpenEnd, &[]),
                t(Text, &["t"]),
                t(TagClose, &["", "a"]),
                t(TagOpenStart, &["", "br"]),
                t(TagOpenEndVoid, &[]),
                t(Eof, &[]),
            ]
        );
    }

    #[test]
    fn reads_prefixes_and_binding_names() {
        assert_eq!(
            parts(
                "<svg:g xlink:href=\"a\" [x.y]=\"b\" [(m)]=\"c\" (click)=\"d()\" *ngIf=\"e\" #ref [attr.a b]>"
            ),
            [
                t(TagOpenStart, &["svg", "g"]),
                t(AttrName, &["xlink", "href"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["a"]),
                t(AttrQuote, &["\""]),
                t(AttrName, &["", "[x.y]"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["b"]),
                t(AttrQuote, &["\""]),
                t(AttrName, &["", "[(m)]"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["c"]),
                t(AttrQuote, &["\""]),
                t(AttrName, &["", "(click)"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["d()"]),
                t(AttrQuote, &["\""]),
                t(AttrName, &["", "*ngIf"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["e"]),
                t(AttrQuote, &["\""]),
                t(AttrName, &["", "#ref"]),
                // Inside brackets a space belongs to the name.
                t(AttrName, &["", "[attr.a b]"]),
                t(TagOpenEnd, &[]),
                t(Eof, &[]),
            ]
        );
    }

    #[test]
    fn reads_interpolations_in_text_and_values() {
        assert_eq!(
            parts("a {{ b | c: '}}' }} d<i title=\"x{{y}}\"></i>{{ e"),
            [
                t(Text, &["a "]),
                t(Interpolation, &["{{", " b | c: '}}' ", "}}"]),
                t(Text, &[" d"]),
                t(TagOpenStart, &["", "i"]),
                t(AttrName, &["", "title"]),
                t(AttrQuote, &["\""]),
                t(AttrValueText, &["x"]),
                t(AttrValueInterpolation, &["{{", "y", "}}"]),
                t(AttrValueText, &[""]),
                t(AttrQuote, &["\""]),
                t(TagOpenEnd, &[]),
                t(TagClose, &["", "i"]),
                t(Text, &[""]),
                // An interpolation left open runs to the end, without `}}`.
                t(Interpolation, &["{{", " e"]),
                t(Text, &[""]),
                t(Eof, &[]),
            ]
        );
        // A tag ends an interpolation early.
        assert_eq!(
            parts("{{ a <b>")[..2],
            [t(Text, &[""]), t(Interpolation, &["{{", " a "])]
        );
        // Line ends are kept as written (`preserveLineEndings`), in a text, an interpolation
        // left open or closed, and a raw text.
        assert_eq!(
            parts("a\r\n{{\r b }}\r\n{{ c\r")[..5],
            [
                t(Text, &["a\r\n"]),
                t(Interpolation, &["{{", "\r b ", "}}"]),
                t(Text, &["\r\n"]),
                t(Interpolation, &["{{", " c\r"]),
                t(Text, &[""]),
            ]
        );
        assert_eq!(
            parts("<textarea>a\r\nb</textarea>")[2],
            t(EscapableRawText, &["a\r\nb"])
        );
    }

    #[test]
    fn decodes_entities_and_keeps_a_lone_ampersand_as_text() {
        assert_eq!(
            parts("&amp;&#64;&#x41;a & b&c"),
            [
                t(Text, &[""]),
                t(EncodedEntity, &["&", "&amp;"]),
                t(Text, &[""]),
                t(EncodedEntity, &["@", "&#64;"]),
                t(Text, &[""]),
                t(EncodedEntity, &["A", "&#x41;"]),
                t(Text, &["a & b&c"]),
                t(Eof, &[]),
            ]
        );
        // `&#12a;` scans hexadecimal digits, then reads them as `parseInt("12a", 10)` does: 12.
        assert_eq!(parts("&#12a;")[1], t(EncodedEntity, &["\u{c}", "&#12a;"]));
        // `&#12 `: the space after the digits is not `;`, read into the message, the error at 5.
        assert_eq!(
            errors("&#12 x"),
            [(
                "Unable to parse entity \"&#12 \" - decimal character reference entities must end with \";\"".to_owned(),
                5,
                5
            )]
        );
        assert_eq!(
            errors("&nope;"),
            [(
                "Unknown entity \"nope\" - use the \"&#<decimal>;\" or  \"&#x<hex>;\" syntax"
                    .to_owned(),
                0,
                6
            )]
        );
        assert_eq!(
            errors("&#;")[0].0,
            "Unknown entity \"&#;\" - use the \"&#<decimal>;\" or  \"&#x<hex>;\" syntax"
        );
        assert_eq!(
            parts("&#xD800;")[1],
            t(EncodedEntity, &["\u{FFFD}", "&#xD800;"])
        );
    }

    #[test]
    fn reads_comments_cdata_doctype_and_processing_instructions() {
        assert_eq!(
            parts("<!-- a -->\r\n<![CDATA[b]]><!doctype html><?xml v?>"),
            [
                t(CommentStart, &[]),
                t(RawText, &[" a "]),
                t(CommentEnd, &[]),
                t(Text, &["\r\n"]),
                t(CdataStart, &[]),
                t(RawText, &["b"]),
                t(CdataEnd, &[]),
                t(DocType, &["doctype html"]),
                t(ProcessingInstruction, &["xml v"]),
                t(Eof, &[]),
            ]
        );
        assert_eq!(
            parts("<a // one\n /* two */ b>")[..4],
            [
                t(TagOpenStart, &["", "a"]),
                t(InElementComment, &[" one", "single"]),
                t(InElementComment, &[" two ", "multi"]),
                t(AttrName, &["", "b"]),
            ]
        );
    }

    #[test]
    fn reads_raw_and_escapable_raw_text_up_to_the_end_tag() {
        assert_eq!(
            parts("<script>a<b>&amp;</SCRIPT ><textarea>c&amp;</textarea>"),
            [
                t(TagOpenStart, &["", "script"]),
                t(TagOpenEnd, &[]),
                t(RawText, &["a<b>&amp;"]),
                t(TagClose, &["", "script"]),
                t(TagOpenStart, &["", "textarea"]),
                t(TagOpenEnd, &[]),
                t(EscapableRawText, &["c"]),
                t(EncodedEntity, &["&", "&amp;"]),
                t(EscapableRawText, &[""]),
                t(TagClose, &["", "textarea"]),
                t(Eof, &[]),
            ]
        );
    }

    #[test]
    fn reads_blocks_their_parameters_and_their_ends() {
        assert_eq!(
            parts(
                "@if (a; as b) {x} @else   if (c) {} @else {}@for (i of l; track i.id) {}@empty {}"
            ),
            [
                t(BlockOpenStart, &["if"]),
                t(BlockParameter, &["a"]),
                t(BlockParameter, &["as b"]),
                t(BlockOpenEnd, &[]),
                t(Text, &["x"]),
                t(BlockClose, &[]),
                t(Text, &[" "]),
                t(BlockOpenStart, &["else if"]),
                t(BlockParameter, &["c"]),
                t(BlockOpenEnd, &[]),
                t(BlockClose, &[]),
                t(Text, &[" "]),
                t(BlockOpenStart, &["else"]),
                t(BlockOpenEnd, &[]),
                t(BlockClose, &[]),
                t(BlockOpenStart, &["for"]),
                t(BlockParameter, &["i of l"]),
                t(BlockParameter, &["track i.id"]),
                t(BlockOpenEnd, &[]),
                t(BlockClose, &[]),
                t(BlockOpenStart, &["empty"]),
                t(BlockOpenEnd, &[]),
                t(BlockClose, &[]),
                t(Eof, &[]),
            ]
        );
        // A `;` or a `)` inside quotes or parentheses stays in the parameter.
        assert_eq!(
            parts("@switch (f(';)', (g))) {}")[1],
            t(BlockParameter, &["f(';)', (g))"])
        );
    }

    #[test]
    fn reads_consecutive_cases_default_never_and_incomplete_blocks() {
        assert_eq!(
            parts("@case (1) @case (2) {a}@default never;@if (x"),
            [
                t(BlockOpenStart, &["case"]),
                t(BlockParameter, &["1"]),
                t(BlockOpenEnd, &[]),
                t(BlockClose, &[]),
                t(BlockOpenStart, &["case"]),
                t(BlockParameter, &["2"]),
                t(BlockOpenEnd, &[]),
                t(Text, &["a"]),
                t(BlockClose, &[]),
                t(BlockOpenStart, &["default never"]),
                t(BlockOpenEnd, &[]),
                t(BlockClose, &[]),
                t(IncompleteBlockOpen, &["if"]),
                t(BlockParameter, &["x"]),
                t(Eof, &[]),
            ]
        );
        assert_eq!(
            parts("@defer;x")[..2],
            [t(IncompleteBlockOpen, &["defer"]), t(Text, &[";x"])]
        );
        // After a name character, spaces belong to the name, as for `else if`.
        assert_eq!(
            parts("@defer x")[..2],
            [t(IncompleteBlockOpen, &["defer x"]), t(Eof, &[])]
        );
        // `@` not opening a known block is text.
        assert_eq!(parts("a@b.c")[0], t(Text, &["a@b.c"]));
    }

    #[test]
    fn reads_let_declarations_complete_or_not() {
        assert_eq!(
            parts("@let total = a + ';' + b;@let x@let y = 1"),
            [
                t(LetStart, &["total"]),
                t(LetValue, &["a + ';' + b"]),
                t(LetEnd, &[]),
                t(IncompleteLet, &["x"]),
                t(IncompleteLet, &["y"]),
                t(LetValue, &["1"]),
                t(Eof, &[]),
            ]
        );
        assert_eq!(parts("@letx")[0], t(IncompleteLet, &["@let"]));
    }

    #[test]
    fn reads_icu_expansions() {
        assert_eq!(
            parts("{n, plural, =0 {none} other {{{n}} <b>x</b>}}"),
            [
                t(ExpansionFormStart, &[]),
                t(RawText, &["n"]),
                t(RawText, &["plural"]),
                t(ExpansionCaseValue, &["=0"]),
                t(ExpansionCaseExpStart, &[]),
                t(Text, &["none"]),
                t(ExpansionCaseExpEnd, &[]),
                t(ExpansionCaseValue, &["other"]),
                t(ExpansionCaseExpStart, &[]),
                t(Text, &[""]),
                t(Interpolation, &["{{", "n", "}}"]),
                t(Text, &[" "]),
                t(TagOpenStart, &["", "b"]),
                t(TagOpenEnd, &[]),
                t(Text, &["x"]),
                t(TagClose, &["", "b"]),
                t(ExpansionCaseExpEnd, &[]),
                t(ExpansionFormEnd, &[]),
                t(Eof, &[]),
            ]
        );
    }

    #[test]
    fn spans_leave_the_leading_trivia_out_of_the_start() {
        let tokens = tokenize("<a>\n  b</a>").tokens;
        // `<a>` is bytes 0 to 3; the text "\n  b" bytes 3 to 7, its start past the trivia at 6.
        assert_eq!((tokens[0].span.start, tokens[0].span.end), (0, 2));
        assert_eq!((tokens[1].span.start, tokens[1].span.end), (2, 3));
        assert_eq!(
            (
                tokens[2].span.full_start,
                tokens[2].span.start,
                tokens[2].span.end
            ),
            (3, 6, 7)
        );
        assert_eq!((tokens[3].span.start, tokens[3].span.end), (7, 11));
    }

    #[test]
    fn reports_malformed_tags_and_ends() {
        // A `<` not starting a tag is text; an unclosed start tag is incomplete without an error.
        assert_eq!(parts("a < b")[0], t(Text, &["a < b"]));
        assert_eq!(
            parts("<a b=\"c\" <d>")[..2],
            [t(IncompleteTagOpen, &["", "a"]), t(AttrName, &["", "b"])]
        );
        assert_eq!(errors("<a b=\"c\" <d>"), Vec::<(String, u32, u32)>::new());
        assert_eq!(
            errors("<a b=\"c"),
            [("Unexpected character \"EOF\"".to_owned(), 7, 7)]
        );
        assert_eq!(
            errors("</a"),
            [("Unexpected character \"EOF\"".to_owned(), 3, 3)]
        );
        assert_eq!(
            errors("<!-- x"),
            [("Unexpected character \"EOF\"".to_owned(), 6, 6)]
        );
        assert_eq!(
            errors("<a \"b\">"),
            Vec::<(String, u32, u32)>::new(),
            "an attribute starting with a quote makes the tag incomplete, its error dropped"
        );
        assert_eq!(parts("<a \"b\">")[0], t(IncompleteTagOpen, &["", "a"]));
        // An error inside an ICU expansion asks whether a `{` was meant.
        assert_eq!(
            errors("{a, b, c"),
            [(
                "Unexpected character \"EOF\" (Do you have an unescaped \"{\" in your template? Use \"{{ '{' }}\") to escape it.)".to_owned(),
                8,
                8
            )]
        );
    }

    #[test]
    fn treats_a_nul_as_the_end_as_the_compiler_does() {
        assert_eq!(parts("a\0b"), [t(Text, &["a"]), t(Eof, &[])]);
    }
}
