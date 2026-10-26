//! A selector read into its parts: a port of `tokenize.js` and of the parsing of
//! `postcss-selector-parser` 7.1.6 (MIT, under ADR-0076), which Stylelint's rules read a rule's
//! selector with: a list of selectors, each of tags, classes, ids, attributes, pseudo-classes and
//! pseudo-elements (their arguments read as selectors), combinators, `&`, `*`, comments and
//! strings, each with its offset in the selector. The parts live in one list, each knowing its
//! parent and its children by index; arguments nested deeper than `NESTING_LIMIT` are an error,
//! as the library's own `maxNestingDepth`. Namespaces (`ns|a`) are read as a combinator `|`.

use crate::template::NESTING_LIMIT;

/// What a part is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A selector of the list, or of a pseudo's arguments.
    Selector,
    /// A type selector, `a`; also a word inside a pseudo's arguments (`2n`).
    Tag,
    /// A class, `.a`.
    Class,
    /// An id, `#a`.
    Id,
    /// An attribute selector, `[a="b"]`.
    Attribute,
    /// A pseudo-class or pseudo-element, `:hover`, `::before`, its arguments its selectors.
    Pseudo,
    /// A combinator: ` `, `>`, `+`, `~`, `||` or a named one, `/deep/`.
    Combinator,
    /// The nesting selector, `&`.
    Nesting,
    /// The universal selector, `*`.
    Universal,
    /// A comment, `/* … */`.
    Comment,
    /// A quoted string, or parentheses not after a pseudo.
    String,
}

/// An attribute selector's parts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Attribute {
    /// Its name.
    pub name: String,
    /// Its operator, `=`, `~=`, `|=`, `^=`, `$=` or `*=`.
    pub operator: Option<String>,
    /// Its value as written, quotes included.
    pub value: Option<String>,
    /// Its flag, `i` or `s`.
    pub flag: Option<String>,
}

/// A part of a selector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Part {
    /// What it is.
    pub kind: Kind,
    /// Its text: a tag's, a class's or an id's name unescaped, a pseudo's colons and name, a
    /// combinator's characters (` ` for a descendant), an attribute's text between its brackets.
    pub value: String,
    /// Its first byte in the selector.
    pub start: usize,
    /// The byte after its last.
    pub end: usize,
    /// Its parent: a selector's, the pseudo it is the argument of; a part's, its selector.
    pub parent: Option<usize>,
    /// A selector's parts, or a pseudo's selectors.
    pub nodes: Vec<usize>,
    /// The spaces before it.
    pub before: String,
    /// The spaces after it.
    pub after: String,
    /// An attribute selector's parts.
    pub attribute: Option<Attribute>,
}

impl Part {
    /// A part of a kind over a range.
    fn new(kind: Kind, value: String, start: usize, end: usize) -> Self {
        Self {
            kind,
            value,
            start,
            end,
            parent: None,
            nodes: Vec::new(),
            before: String::new(),
            after: String::new(),
            attribute: None,
        }
    }
}

/// A selector list read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Selectors {
    /// Every part.
    pub parts: Vec<Part>,
    /// The selectors of the list, in order.
    pub top: Vec<usize>,
}

impl Selectors {
    /// A part by its index.
    #[must_use]
    pub fn part(&self, index: usize) -> &Part {
        &self.parts[index]
    }

    /// Every part under the list, in the order written, each selector before its parts.
    #[must_use]
    pub fn walk(&self) -> Vec<usize> {
        let mut found = Vec::new();
        let mut stack: Vec<usize> = self.top.iter().rev().copied().collect();
        while let Some(index) = stack.pop() {
            found.push(index);
            stack.extend(self.parts[index].nodes.iter().rev());
        }
        found
    }

    /// The part before one in its selector.
    #[must_use]
    pub fn previous(&self, index: usize) -> Option<usize> {
        let siblings = &self.parts[self.parts[index].parent?].nodes;
        let at = siblings.iter().position(|each| *each == index)?;
        at.checked_sub(1).map(|before| siblings[before])
    }

    /// The part after one in its selector.
    #[must_use]
    pub fn next(&self, index: usize) -> Option<usize> {
        let siblings = &self.parts[self.parts[index].parent?].nodes;
        let at = siblings.iter().position(|each| *each == index)?;
        siblings.get(at + 1).copied()
    }

    /// A part's text in the selector it was read from.
    #[must_use]
    pub fn text<'t>(&self, selector: &'t str, index: usize) -> &'t str {
        let part = &self.parts[index];
        selector.get(part.start..part.end).unwrap_or_default()
    }
}

/// What a token is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Token {
    /// Spaces.
    Space,
    /// `+`, `>`, `~` or `|`, one or several.
    Combinator,
    /// A quoted string.
    Str,
    /// A comment.
    Comment,
    /// A word.
    Word,
    /// One of `* & ! , = $ ^ [ ] : ; ( ) /`.
    Char(u8),
}

/// A token over a range of the selector.
#[derive(Clone, Copy, Debug)]
struct Lexeme {
    /// What it is.
    token: Token,
    /// Its first byte.
    start: usize,
    /// The byte after it.
    end: usize,
}

/// Whether a byte ends a word.
const fn ends_word(byte: u8) -> bool {
    matches!(
        byte,
        b' ' | b'\t'
            | b'\n'
            | b'\r'
            | 0x0c
            | b'&'
            | b'*'
            | b'!'
            | b','
            | b':'
            | b';'
            | b'('
            | b')'
            | b'['
            | b']'
            | b'\''
            | b'"'
            | b'+'
            | b'|'
            | b'~'
            | b'>'
            | b'='
            | b'$'
            | b'^'
            | b'/'
    )
}

/// The last byte of an escape from its backslash (`consumeEscape`).
fn escape_end(css: &[u8], start: usize) -> usize {
    let mut next = start;
    let code = css.get(next + 1).copied();
    match code {
        Some(b'\t' | b'\n' | b'\r' | 0x0c) => {}
        Some(byte) if byte.is_ascii_hexdigit() => {
            let mut digits = 0;
            loop {
                next += 1;
                digits += 1;
                let following = css.get(next + 1).copied();
                if !(following.is_some_and(|byte| byte.is_ascii_hexdigit()) && digits < 6) {
                    if digits < 6 && following == Some(b' ') {
                        next += 1;
                    }
                    break;
                }
            }
        }
        _ => next += 1,
    }
    next
}

/// The selector's tokens.
fn tokenize(selector: &str) -> Result<Vec<Lexeme>, (String, usize)> {
    let css = selector.as_bytes();
    let mut tokens = Vec::new();
    let mut start = 0;
    while start < css.len() {
        let code = css[start];
        let (token, end) = match code {
            b' ' | b'\t' | b'\n' | b'\r' | 0x0c => {
                let mut next = start + 1;
                while css
                    .get(next)
                    .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0c))
                {
                    next += 1;
                }
                (Token::Space, next)
            }
            b'+' | b'>' | b'~' | b'|' => {
                let mut next = start + 1;
                while css
                    .get(next)
                    .is_some_and(|byte| matches!(byte, b'+' | b'>' | b'~' | b'|'))
                {
                    next += 1;
                }
                (Token::Combinator, next)
            }
            b'*' | b'&' | b'!' | b',' | b'=' | b'$' | b'^' | b'[' | b']' | b':' | b';' | b'('
            | b')' => (Token::Char(code), start + 1),
            b'\'' | b'"' => {
                let mut next = start;
                loop {
                    let Some(found) = css[next + 1..].iter().position(|byte| *byte == code) else {
                        return Err(("Unclosed quote".to_owned(), start));
                    };
                    next = next + 1 + found;
                    let escapes = css[..next]
                        .iter()
                        .rev()
                        .take_while(|byte| **byte == b'\\')
                        .count();
                    if escapes % 2 == 0 {
                        break;
                    }
                }
                (Token::Str, next + 1)
            }
            b'/' if css.get(start + 1) == Some(&b'*') => {
                let Some(found) = css[start + 2..].windows(2).position(|pair| pair == b"*/") else {
                    return Err(("Unclosed comment".to_owned(), start));
                };
                (Token::Comment, start + 2 + found + 2)
            }
            b'/' => (Token::Char(b'/'), start + 1),
            _ => {
                let mut next = start;
                loop {
                    let byte = css[next];
                    if ends_word(byte) {
                        break;
                    }
                    next = if byte == b'\\' {
                        escape_end(css, next) + 1
                    } else {
                        next + 1
                    };
                    if next >= css.len() {
                        break;
                    }
                }
                (Token::Word, next.min(css.len()))
            }
        };
        tokens.push(Lexeme { token, start, end });
        start = end;
    }
    Ok(tokens)
}

/// A name with its escapes resolved (`unesc`): `\31 0` is `10`, `\.` is `.`.
#[must_use]
pub fn unescape(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut characters = text.char_indices().peekable();
    while let Some((_, character)) = characters.next() {
        if character != '\\' {
            result.push(character);
            continue;
        }
        let mut hex = String::new();
        while hex.len() < 6 {
            match characters.peek() {
                Some((_, digit)) if digit.is_ascii_hexdigit() => {
                    hex.push(*digit);
                    characters.next();
                }
                _ => break,
            }
        }
        if hex.is_empty() {
            if let Some((_, escaped)) = characters.next() {
                result.push(escaped);
            }
            continue;
        }
        if hex.len() < 6 && characters.peek().is_some_and(|(_, space)| *space == ' ') {
            characters.next();
        }
        let code = u32::from_str_radix(&hex, 16).unwrap_or(0xfffd);
        result.push(
            char::from_u32(code)
                .filter(|_| code != 0)
                .unwrap_or('\u{fffd}'),
        );
    }
    result
}

/// The reading of a selector list.
struct Reader<'s> {
    /// The text.
    css: &'s str,
    /// Its tokens.
    tokens: Vec<Lexeme>,
    /// The next token.
    position: usize,
    /// The parts read.
    parts: Vec<Part>,
    /// The selectors of the list.
    top: Vec<usize>,
    /// The selector the next part goes into.
    current: usize,
    /// The spaces before the next part.
    spaces: String,
    /// How deep in pseudos' arguments.
    depth: usize,
}

/// A step of the reading.
type Read = Result<(), (String, usize)>;

impl Reader<'_> {
    /// The text of a token.
    fn content(&self, at: usize) -> &str {
        let token = self.tokens[at];
        &self.css[token.start..token.end]
    }

    /// The kind of the token at a position.
    fn kind(&self, at: usize) -> Option<Token> {
        self.tokens.get(at).map(|lexeme| lexeme.token)
    }

    /// A new selector starting at a byte, added to a parent's or the list's.
    fn selector(&mut self, start: usize, parent: Option<usize>) -> usize {
        let index = self.parts.len();
        let mut part = Part::new(Kind::Selector, String::new(), start, start);
        part.parent = parent;
        self.parts.push(part);
        match parent {
            Some(parent) => self.parts[parent].nodes.push(index),
            None => self.top.push(index),
        }
        index
    }

    /// Adds a part to the current selector, the spaces read before it given to it.
    fn push(&mut self, mut part: Part) -> usize {
        let index = self.parts.len();
        part.parent = Some(self.current);
        part.before = std::mem::take(&mut self.spaces);
        let end = part.end;
        self.parts.push(part);
        let selector = &mut self.parts[self.current];
        selector.nodes.push(index);
        selector.end = selector.end.max(end);
        index
    }

    /// The last part of the current selector.
    fn last(&self) -> Option<usize> {
        self.parts[self.current].nodes.last().copied()
    }

    /// Reads every token.
    fn read(&mut self) -> Read {
        while self.position < self.tokens.len() {
            self.parse(true)?;
        }
        Ok(())
    }

    /// Reads the part at the current token.
    fn parse(&mut self, throw_on_parenthesis: bool) -> Read {
        let lexeme = self.tokens[self.position];
        match lexeme.token {
            Token::Space => self.space(),
            Token::Comment => {
                let value = self.content(self.position).to_owned();
                self.push(Part::new(Kind::Comment, value, lexeme.start, lexeme.end));
                self.position += 1;
                Ok(())
            }
            Token::Char(b'(') => self.parentheses(),
            Token::Char(b')') if throw_on_parenthesis => {
                Err(("Expected an opening parenthesis.".to_owned(), lexeme.start))
            }
            Token::Char(b')') => Ok(()),
            Token::Char(b'[') => self.attribute(),
            Token::Char(b'$' | b'^' | b'=') | Token::Word => {
                self.word(None);
                Ok(())
            }
            Token::Char(b':') => self.pseudo(),
            Token::Char(b',') => {
                self.comma();
                Ok(())
            }
            Token::Char(b'*' | b'&') | Token::Str => {
                let kind = match lexeme.token {
                    Token::Char(b'*') => Kind::Universal,
                    Token::Char(b'&') => Kind::Nesting,
                    _ => Kind::String,
                };
                self.single(kind, lexeme);
                Ok(())
            }
            Token::Char(b'/') | Token::Combinator => self.combinator(),
            Token::Char(b']') => Err((
                "Expected an opening square bracket.".to_owned(),
                lexeme.start,
            )),
            Token::Char(b';') => Err((
                "Expected a backslash preceding the semicolon.".to_owned(),
                lexeme.start,
            )),
            Token::Char(_) => Err((
                format!(
                    "Unexpected '{}'. Escaping special characters with \\ may help.",
                    self.content(self.position)
                ),
                lexeme.start,
            )),
        }
    }

    /// A one-token part.
    fn single(&mut self, kind: Kind, lexeme: Lexeme) {
        let value = self.content(self.position).to_owned();
        self.push(Part::new(kind, value, lexeme.start, lexeme.end));
        self.position += 1;
    }

    /// Spaces: before a selector, after one, or a descendant combinator.
    fn space(&mut self) -> Read {
        let content = self.content(self.position).to_owned();
        let previous = self.position.checked_sub(1).and_then(|at| self.kind(at));
        let only_comments = self.parts[self.current]
            .nodes
            .iter()
            .all(|&index| self.parts[index].kind == Kind::Comment);
        if self.position == 0 || matches!(previous, Some(Token::Char(b',' | b'('))) || only_comments
        {
            self.spaces = content;
            self.position += 1;
        } else if self.position == self.tokens.len() - 1
            || matches!(self.kind(self.position + 1), Some(Token::Char(b',' | b')')))
        {
            if let Some(last) = self.last() {
                self.parts[last].after = content;
            }
            self.position += 1;
        } else {
            return self.combinator();
        }
        Ok(())
    }

    /// The next token that is neither a space nor a comment, from a position.
    fn next_meaningful(&self, from: usize) -> Option<usize> {
        (from..self.tokens.len())
            .find(|&at| !matches!(self.tokens[at].token, Token::Space | Token::Comment))
    }

    /// Whether `/name/` starts at a position.
    fn is_named_combinator(&self, at: usize) -> bool {
        self.kind(at) == Some(Token::Char(b'/'))
            && self.kind(at + 1) == Some(Token::Word)
            && self.kind(at + 2) == Some(Token::Char(b'/'))
    }

    /// A combinator, or the spaces at a selector's end.
    fn combinator(&mut self) -> Read {
        let meaningful = self.next_meaningful(self.position);
        let closing =
            meaningful.is_none_or(|at| matches!(self.tokens[at].token, Token::Char(b',' | b')')));
        if closing {
            let stop = meaningful.unwrap_or(self.tokens.len());
            let spaces = self.tokens[self.position..stop]
                .iter()
                .filter(|lexeme| lexeme.token == Token::Space)
                .map(|lexeme| &self.css[lexeme.start..lexeme.end])
                .collect::<String>();
            if let Some(last) = self.last() {
                self.parts[last].after.push_str(&spaces);
            }
            self.position = stop;
            return Ok(());
        }
        let first = self.tokens[self.position];
        let spaced = meaningful.is_some_and(|at| at > self.position);
        let before = if spaced {
            let stop = meaningful.unwrap_or(self.position);
            let text = self.css[first.start..self.tokens[stop - 1].end].to_owned();
            self.position = stop;
            text
        } else {
            String::new()
        };
        let lexeme = self.tokens[self.position];
        let mut part = if self.is_named_combinator(self.position) {
            let name = self.content(self.position + 1).to_lowercase();
            let end = self.tokens[self.position + 2].end;
            self.position += 3;
            Some(Part::new(
                Kind::Combinator,
                format!("/{name}/"),
                lexeme.start,
                end,
            ))
        } else if lexeme.token == Token::Combinator {
            self.position += 1;
            Some(Part::new(
                Kind::Combinator,
                self.css[lexeme.start..lexeme.end].to_owned(),
                lexeme.start,
                lexeme.end,
            ))
        } else if !spaced {
            return Err((
                format!(
                    "Unexpected '{}'. Escaping special characters with \\ may help.",
                    &self.css[lexeme.start..lexeme.end]
                ),
                lexeme.start,
            ));
        } else {
            None
        };
        if let Some(part) = part.as_mut() {
            part.before = before;
        }
        let mut part = part.unwrap_or_else(|| {
            Part::new(
                Kind::Combinator,
                " ".to_owned(),
                first.start,
                self.tokens[self.position - 1].end,
            )
        });
        if self.kind(self.position) == Some(Token::Space) {
            self.content(self.position).clone_into(&mut part.after);
            self.position += 1;
        }
        let pending = std::mem::take(&mut self.spaces);
        let index = self.push(part);
        self.parts[index].before.insert_str(0, &pending);
        Ok(())
    }

    /// `,`: the next selector of the list, or a trailing comma.
    fn comma(&mut self) {
        if self.position == self.tokens.len() - 1 {
            self.position += 1;
            return;
        }
        let start = self.tokens[self.position + 1].start;
        let parent = self.parts[self.current].parent;
        self.current = self.selector(start, parent);
        self.position += 1;
    }

    /// `(`: a pseudo's arguments, read as selectors, or parentheses added to the last part.
    fn parentheses(&mut self) -> Read {
        let opening = self.tokens[self.position];
        let last = self.last();
        let mut unbalanced = 1;
        self.position += 1;
        if let Some(pseudo) = last.filter(|&index| self.parts[index].kind == Kind::Pseudo) {
            if self.depth >= NESTING_LIMIT {
                return Err((
                    format!(
                        "Cannot parse selector: nesting depth exceeds the maximum of {NESTING_LIMIT}."
                    ),
                    opening.start,
                ));
            }
            self.depth += 1;
            let start = self
                .tokens
                .get(self.position)
                .map_or(opening.end, |each| each.start);
            let cache = self.current;
            self.current = self.selector(start, Some(pseudo));
            while self.position < self.tokens.len() && unbalanced > 0 {
                match self.tokens[self.position].token {
                    Token::Char(b'(') => unbalanced += 1,
                    Token::Char(b')') => unbalanced -= 1,
                    _ => {}
                }
                if unbalanced > 0 {
                    self.parse(false)?;
                } else {
                    let end = self.tokens[self.position].end;
                    self.parts[pseudo].end = end;
                    self.position += 1;
                }
            }
            self.depth -= 1;
            self.current = cache;
            let end = self.parts[pseudo].end;
            let selector = &mut self.parts[self.current];
            selector.end = selector.end.max(end);
        } else {
            let mut end = opening.end;
            while self.position < self.tokens.len() && unbalanced > 0 {
                match self.tokens[self.position].token {
                    Token::Char(b'(') => unbalanced += 1,
                    Token::Char(b')') => unbalanced -= 1,
                    _ => {}
                }
                end = self.tokens[self.position].end;
                self.position += 1;
            }
            let text = self.css[opening.start..end].to_owned();
            match last {
                Some(last) => {
                    self.parts[last].value.push_str(&text);
                    self.parts[last].end = end;
                }
                None => {
                    self.push(Part::new(Kind::String, text, opening.start, end));
                }
            }
        }
        if unbalanced > 0 {
            let at = self.tokens.get(self.position).unwrap_or(&opening).start;
            return Err(("Expected a closing parenthesis.".to_owned(), at));
        }
        Ok(())
    }

    /// `:` and `::`: a pseudo-class or pseudo-element, its name the word after its colons.
    fn pseudo(&mut self) -> Read {
        let start = self.tokens[self.position].start;
        let mut colons = String::new();
        while self.kind(self.position) == Some(Token::Char(b':')) {
            colons.push(':');
            self.position += 1;
        }
        match self.kind(self.position) {
            Some(Token::Word) => {
                self.word(Some((&colons, start)));
                Ok(())
            }
            Some(_) => Err((
                "Expected a pseudo-class or pseudo-element.".to_owned(),
                self.tokens[self.position].start,
            )),
            None => Err((
                "Expected a pseudo-class or pseudo-element.".to_owned(),
                self.css.len(),
            )),
        }
    }

    /// A word, cut at its `.` and `#` into a tag, classes and ids; after colons, its first
    /// piece is a pseudo's name.
    fn word(&mut self, pseudo: Option<(&str, usize)>) {
        let first = self.tokens[self.position];
        let mut end = first.end;
        while matches!(
            self.kind(self.position + 1),
            Some(Token::Char(b'$' | b'^' | b'=') | Token::Word)
        ) {
            self.position += 1;
            end = self.tokens[self.position].end;
            let current = self.content(self.position);
            if current.ends_with('\\') && self.kind(self.position + 1) == Some(Token::Space) {
                self.position += 1;
                end = self.tokens[self.position].end;
            }
        }
        let word = &self.css[first.start..end];
        let bytes = word.as_bytes();
        let percent = is_keyframes_percent(word);
        let mut classes: Vec<usize> = Vec::new();
        let mut ids: Vec<usize> = Vec::new();
        for (index, byte) in bytes.iter().enumerate() {
            let escaped = index > 0 && bytes[index - 1] == b'\\';
            if *byte == b'.' && !escaped && !percent {
                classes.push(index);
            } else if *byte == b'#' && !escaped && bytes.get(index + 1) != Some(&b'{') {
                ids.push(index);
            }
        }
        let mut cuts: Vec<usize> = std::iter::once(0)
            .chain(classes.iter().copied())
            .chain(ids.iter().copied())
            .collect();
        cuts.sort_unstable();
        cuts.dedup();
        for (order, &cut) in cuts.iter().enumerate() {
            let next = cuts.get(order + 1).copied().unwrap_or(word.len());
            let piece = &word[cut..next];
            let start = first.start + cut;
            let part = if order == 0
                && let Some((colons, pseudo_start)) = pseudo
            {
                Part::new(
                    Kind::Pseudo,
                    format!("{colons}{piece}"),
                    pseudo_start,
                    first.start + next,
                )
            } else if classes.contains(&cut) || ids.contains(&cut) {
                let kind = if classes.contains(&cut) {
                    Kind::Class
                } else {
                    Kind::Id
                };
                Part::new(kind, unescape(&piece[1..]), start, first.start + next)
            } else {
                Part::new(Kind::Tag, unescape(piece), start, first.start + next)
            };
            self.push(part);
        }
        self.position += 1;
    }

    /// `[…]`: an attribute selector.
    fn attribute(&mut self) -> Read {
        let opening = self.tokens[self.position];
        self.position += 1;
        while self.position < self.tokens.len()
            && self.tokens[self.position].token != Token::Char(b']')
        {
            self.position += 1;
        }
        let Some(closing) = self.tokens.get(self.position).copied() else {
            return Err((
                "Expected a closing square bracket.".to_owned(),
                opening.start,
            ));
        };
        let inside = &self.css[opening.end..closing.start];
        let mut part = Part::new(
            Kind::Attribute,
            inside.to_owned(),
            opening.start,
            closing.end,
        );
        part.attribute = Some(read_attribute(inside));
        self.push(part);
        self.position += 1;
        Ok(())
    }
}

/// Whether a word is a keyframe's decimal percentage, `12.5%`, whose dot cuts nothing.
fn is_keyframes_percent(word: &str) -> bool {
    let Some(number) = word.strip_suffix('%') else {
        return false;
    };
    let Some((whole, fraction)) = number.split_once('.') else {
        return false;
    };
    !whole.is_empty()
        && !fraction.is_empty()
        && whole.bytes().all(|byte| byte.is_ascii_digit())
        && fraction.bytes().all(|byte| byte.is_ascii_digit())
}

/// An attribute selector's name, operator, value and flag from its text.
fn read_attribute(inside: &str) -> Attribute {
    let text = inside.trim();
    let operator_at = text.find('=').map(|at| {
        let before = text.as_bytes().get(at.wrapping_sub(1)).copied();
        if matches!(before, Some(b'~' | b'|' | b'^' | b'$' | b'*')) {
            at - 1
        } else {
            at
        }
    });
    let Some(operator_at) = operator_at else {
        return Attribute {
            name: text.to_owned(),
            ..Attribute::default()
        };
    };
    let equals = text[operator_at..]
        .find('=')
        .map_or(operator_at, |at| operator_at + at);
    let rest = text[equals + 1..].trim();
    let (value, flag) = match rest.as_bytes().first() {
        Some(quote @ (b'"' | b'\'')) => {
            let close = rest[1..]
                .find(char::from(*quote))
                .map_or(rest.len(), |at| at + 2);
            let flag = rest[close..].trim();
            (&rest[..close], (!flag.is_empty()).then(|| flag.to_owned()))
        }
        _ => match rest.split_once(char::is_whitespace) {
            Some((value, flag)) => (value, Some(flag.trim().to_owned())),
            None => (rest, None),
        },
    };
    Attribute {
        name: text[..operator_at].trim().to_owned(),
        operator: Some(text[operator_at..=equals].to_owned()),
        value: Some(value.to_owned()),
        flag,
    }
}

/// A selector list read into its parts, or the error the library would throw and its byte.
///
/// # Errors
///
/// A string or comment left open, a bracket or parenthesis left unclosed, a character a
/// selector cannot hold, or arguments nested past `NESTING_LIMIT`.
pub fn parse(selector: &str) -> Result<Selectors, (String, usize)> {
    let tokens = tokenize(selector)?;
    let mut reader = Reader {
        css: selector,
        tokens,
        position: 0,
        parts: Vec::new(),
        top: Vec::new(),
        current: 0,
        spaces: String::new(),
        depth: 0,
    };
    reader.current = reader.selector(0, None);
    reader.read()?;
    Ok(Selectors {
        parts: reader.parts,
        top: reader.top,
    })
}

#[cfg(test)]
mod tests {
    use super::{Kind, Selectors, parse, unescape};
    use crate::template::NESTING_LIMIT;

    /// The parts of a list, each kind with its value and start, selectors marked by their
    /// count of parts.
    fn parts(selector: &str) -> Vec<(Kind, String, usize)> {
        let read = parse(selector).unwrap_or_default();
        read.walk()
            .into_iter()
            .map(|index| {
                let part = read.part(index);
                (part.kind, part.value.clone(), part.start)
            })
            .collect()
    }

    #[test]
    fn reads_compounds_combinators_and_lists() {
        assert_eq!(
            parts("a.b#c > d, *"),
            [
                (Kind::Selector, String::new(), 0),
                (Kind::Tag, "a".to_owned(), 0),
                (Kind::Class, "b".to_owned(), 1),
                (Kind::Id, "c".to_owned(), 3),
                (Kind::Combinator, ">".to_owned(), 6),
                (Kind::Tag, "d".to_owned(), 8),
                // The next selector starts at the token after the comma, its space.
                (Kind::Selector, String::new(), 10),
                (Kind::Universal, "*".to_owned(), 11),
            ]
        );
        // A descendant combinator at its first space; spaces at the ends are no combinator.
        assert_eq!(
            parts(" a  b "),
            [
                (Kind::Selector, String::new(), 0),
                (Kind::Tag, "a".to_owned(), 1),
                (Kind::Combinator, " ".to_owned(), 2),
                (Kind::Tag, "b".to_owned(), 4),
            ]
        );
    }

    #[test]
    fn reads_pseudos_their_arguments_attributes_and_nesting() {
        let read = parse("&:not(.a, b)::before[x|=\"y\" i]").unwrap_or_default();
        let kinds: Vec<Kind> = read
            .walk()
            .iter()
            .map(|&index| read.part(index).kind)
            .collect();

        assert_eq!(
            kinds,
            [
                Kind::Selector,
                Kind::Nesting,
                Kind::Pseudo,
                Kind::Selector,
                Kind::Class,
                Kind::Selector,
                Kind::Tag,
                Kind::Pseudo,
                Kind::Attribute,
            ]
        );
        let not = read.part(read.walk()[2]);
        // `:not(.a, b)` from byte 1 to 12.
        assert_eq!((not.value.as_str(), not.start, not.end), (":not", 1, 12));
        let attribute = read
            .part(read.walk()[8])
            .attribute
            .clone()
            .unwrap_or_default();
        assert_eq!(
            (
                attribute.name.as_str(),
                attribute.operator.as_deref(),
                attribute.value.as_deref(),
                attribute.flag.as_deref()
            ),
            ("x", Some("|="), Some("\"y\""), Some("i"))
        );
        assert_eq!(read.previous(read.walk()[2]), Some(read.walk()[1]));
        assert_eq!(read.next(read.walk()[7]), Some(read.walk()[8]));
        assert_eq!(
            read.text("&:not(.a, b)::before[x|=\"y\" i]", read.walk()[7]),
            "::before"
        );
    }

    #[test]
    fn keeps_escapes_interpolations_and_keyframe_percentages_whole() {
        assert_eq!(unescape("\\31 0\\.a"), "10.a");
        assert_eq!(unescape("a\\"), "a");
        assert_eq!(
            parts("#{$a}.b 12.5%"),
            [
                (Kind::Selector, String::new(), 0),
                (Kind::Tag, "#{$a}".to_owned(), 0),
                (Kind::Class, "b".to_owned(), 5),
                (Kind::Combinator, " ".to_owned(), 7),
                (Kind::Tag, "12.5%".to_owned(), 8),
            ]
        );
        // A selector and its named combinator.
        assert_eq!(parts("/deep/").len(), 2);
        assert_eq!(parts("a /deep/ b")[2].1, "/deep/");
        assert_eq!(parts("a(b)")[1].1, "a(b)");
    }

    #[test]
    fn reports_what_a_selector_cannot_hold() {
        let error = |selector: &str| parse(selector).err();

        assert_eq!(
            error("a!"),
            Some((
                "Unexpected '!'. Escaping special characters with \\ may help.".to_owned(),
                1
            ))
        );
        assert_eq!(
            error("[a"),
            Some(("Expected a closing square bracket.".to_owned(), 0))
        );
        assert_eq!(error("a)").map(|each| each.1), Some(1));
        assert_eq!(
            error(":not(a").map(|each| each.0),
            Some("Expected a closing parenthesis.".to_owned())
        );
        assert_eq!(
            error("'a").map(|each| each.0),
            Some("Unclosed quote".to_owned())
        );
        assert_eq!(
            error("/* a").map(|each| each.0),
            Some("Unclosed comment".to_owned())
        );
        assert!(error(":").is_some() && error(":(").is_some() && error("a;").is_some());
        assert!(error("]").is_some());
        let deep = format!(
            "{}{}",
            ":is(".repeat(NESTING_LIMIT + 1),
            ")".repeat(NESTING_LIMIT + 1)
        );
        assert!(error(&deep).is_some_and(|each| each.0.contains("nesting depth")));
        let read: Selectors = parse("a,").unwrap_or_default();
        assert_eq!(read.top.len(), 1);
    }
}
