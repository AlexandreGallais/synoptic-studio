//! The tree of an Angular template: a port of `_TreeBuilder` (`ml_parser/parser.ts`) and of the
//! nodes of `ml_parser/ast.ts` of `@angular/compiler` 22.2.3 (MIT, REF-ANGULAR-ML-PARSER), with
//! the messages of its errors. The selectorless components and directives are left out with the
//! lexer's syntax, and an element's `i18n` metadata, which the compiler fills later. The compiler
//! adds a node to its parent when it opens; here an open element or block waits on the stack and
//! joins its parent when it closes, which gives the same order. The tests are this module's own
//! cases.

use super::lexer::{ParseError, Span, Token, TokenType, tokenize};
use super::tags::{TagDefinition, definition, merge_ns_and_name, ns_prefix, split_ns_name};

/// A node of the template.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Node {
    /// `<name …>…</name>`.
    Element(Element),
    /// Text, its interpolations and entities among its tokens.
    Text(Text),
    /// `<!-- … -->`.
    Comment(Comment),
    /// An ICU expansion, `{value, plural, …}`.
    Expansion(Expansion),
    /// `@name (parameters) { … }`.
    Block(Block),
    /// `@let name = value;`.
    LetDeclaration(LetDeclaration),
}

/// An element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    /// Its name, `:svg:path` under a namespace.
    pub name: String,
    /// Its attributes, bindings among them, in the order written.
    pub attrs: Vec<Attribute>,
    /// Its children.
    pub children: Vec<Node>,
    /// Whether written `<name/>`.
    pub is_self_closing: bool,
    /// From its start tag to its end tag, or to its start tag's end when it has none.
    pub span: Span,
    /// Its start tag.
    pub start_span: Span,
    /// Its end tag, when written.
    pub end_span: Option<Span>,
    /// Whether void (`<input>`).
    pub is_void: bool,
    /// The `//` and `/* */` comments inside its start tag.
    pub comments: Vec<StartTagComment>,
}

/// An attribute as written: a binding's name keeps its brackets (`[x]`, `(x)`, `*x`, `#x`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    /// Its name, `:xlink:href` under a namespace.
    pub name: String,
    /// Its value, entities decoded, empty when none.
    pub value: String,
    /// From its name to its value's end.
    pub span: Span,
    /// Its name.
    pub key_span: Span,
    /// Its value, without the quotes, when written.
    pub value_span: Option<Span>,
    /// Its value's tokens: texts, interpolations and entities.
    pub value_tokens: Vec<Token>,
}

/// A comment inside a start tag.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartTagComment {
    /// Its content.
    pub text: String,
    /// Whether written `/* */` rather than `//`.
    pub multiline: bool,
    /// Where.
    pub span: Span,
}

/// Text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Text {
    /// Its value, entities decoded and interpolations as written.
    pub value: String,
    /// Where.
    pub span: Span,
    /// Its tokens: texts, interpolations and entities.
    pub tokens: Vec<Token>,
}

/// A comment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Comment {
    /// Its content, trimmed; `None` for a comment cut short.
    pub value: Option<String>,
    /// Where.
    pub span: Span,
}

/// An ICU expansion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expansion {
    /// The value switched on, as written.
    pub switch_value: String,
    /// `plural`, `select`…
    pub kind: String,
    /// Its cases.
    pub cases: Vec<ExpansionCase>,
    /// Where.
    pub span: Span,
    /// The value switched on.
    pub switch_value_span: Span,
}

/// A case of an ICU expansion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpansionCase {
    /// Its value (`=0`, `other`).
    pub value: String,
    /// Its content.
    pub expression: Vec<Node>,
    /// Where.
    pub span: Span,
    /// Its value.
    pub value_span: Span,
    /// Its content, braces included.
    pub expression_span: Span,
}

/// A block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    /// Its name: `if`, `else if`, `else`, `for`, `empty`, `switch`, `case`, `default`, `defer`…
    pub name: String,
    /// Its parameters, each up to a `;`.
    pub parameters: Vec<BlockParameter>,
    /// Its children.
    pub children: Vec<Node>,
    /// From `@` to its `}`, or to its start's end when unclosed.
    pub span: Span,
    /// Its `@name`.
    pub name_span: Span,
    /// From `@` to its `{`.
    pub start_span: Span,
    /// Its `}`, when written.
    pub end_span: Option<Span>,
}

/// A block's parameter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockParameter {
    /// Its text.
    pub expression: String,
    /// Where.
    pub span: Span,
}

/// A `@let` declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetDeclaration {
    /// Its name.
    pub name: String,
    /// Its value, as written.
    pub value: String,
    /// From `@let` to its `;`.
    pub span: Span,
    /// Its name.
    pub name_span: Span,
    /// Its value.
    pub value_span: Span,
}

/// A template's nodes and its errors, the lexer's first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parsed {
    /// The nodes at its root.
    pub nodes: Vec<Node>,
    /// Its errors.
    pub errors: Vec<ParseError>,
}

/// A template's source read into its tree (`HtmlParser.parse` with `parseTemplate`'s options).
#[must_use]
pub fn parse(source: &str) -> Parsed {
    let tokenized = tokenize(source);
    let mut builder = TreeBuilder::new(tokenized.tokens);
    builder.build();
    let mut errors = tokenized.errors;
    errors.extend(builder.errors);
    Parsed {
        nodes: builder.root,
        errors,
    }
}

/// An open element or block.
enum Container {
    /// An element.
    Element(Element),
    /// A block.
    Block(Block),
}

impl Container {
    /// Its name.
    fn name(&self) -> &str {
        match self {
            Self::Element(element) => &element.name,
            Self::Block(block) => &block.name,
        }
    }

    /// Its children.
    fn children(&mut self) -> &mut Vec<Node> {
        match self {
            Self::Element(element) => &mut element.children,
            Self::Block(block) => &mut block.children,
        }
    }

    /// The node it becomes.
    fn into_node(self) -> Node {
        match self {
            Self::Element(element) => Node::Element(element),
            Self::Block(block) => Node::Block(block),
        }
    }

    /// Records its end.
    fn close(&mut self, end: Option<Span>) {
        let (span, end_span) = match self {
            Self::Element(element) => (&mut element.span, &mut element.end_span),
            Self::Block(block) => (&mut block.span, &mut block.end_span),
        };
        *end_span = end;
        if let Some(end) = end {
            span.end = end.end;
        }
    }
}

/// Which open containers a closing tag or `}` may close.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Expected<'n> {
    /// An element of that name.
    Element(&'n str),
    /// Any block.
    Block,
}

/// The parser's state (`_TreeBuilder`).
struct TreeBuilder {
    /// The tokens.
    tokens: Vec<Token>,
    /// The index of the token read next.
    index: usize,
    /// The open elements and blocks.
    stack: Vec<Container>,
    /// The nodes at the root.
    root: Vec<Node>,
    /// The errors.
    errors: Vec<ParseError>,
}

impl TreeBuilder {
    /// A builder over tokens ending with `Eof`.
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            stack: Vec::new(),
            root: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Builds the tree (`build`).
    fn build(&mut self) {
        while self.peek().kind != TokenType::Eof {
            match self.peek().kind {
                TokenType::TagOpenStart | TokenType::IncompleteTagOpen => {
                    let token = self.advance();
                    self.consume_element_start_tag(&token);
                }
                TokenType::TagClose => {
                    let token = self.advance();
                    self.consume_element_end_tag(&token);
                }
                TokenType::CdataStart => {
                    self.close_void_element();
                    self.advance();
                    let text = self.advance();
                    self.consume_text(text);
                    self.advance_if(TokenType::CdataEnd);
                }
                TokenType::CommentStart => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_comment(&token);
                }
                TokenType::Text | TokenType::RawText | TokenType::EscapableRawText => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_text(token);
                }
                TokenType::ExpansionFormStart => {
                    let token = self.advance();
                    self.consume_expansion(&token);
                }
                TokenType::BlockOpenStart => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_block_open(&token, false);
                }
                TokenType::BlockClose => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_block_close(&token);
                }
                TokenType::IncompleteBlockOpen => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_block_open(&token, true);
                }
                TokenType::LetStart => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_let(&token);
                }
                TokenType::IncompleteLet => {
                    self.close_void_element();
                    let token = self.advance();
                    self.consume_incomplete_let(&token);
                }
                _ => {
                    self.advance();
                }
            }
        }
        for container in &self.stack {
            if let Container::Block(block) = container {
                self.errors.push(ParseError {
                    span: block.span,
                    message: format!("Unclosed block \"{}\"", block.name),
                });
            }
        }
        while !self.stack.is_empty() {
            self.pop_one();
        }
    }

    // Tokens.

    /// The token read next.
    fn peek(&self) -> &Token {
        &self.tokens[self.index.min(self.tokens.len() - 1)]
    }

    /// The token read next, moved past; the last one stays.
    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if self.index < self.tokens.len() - 1 {
            self.index += 1;
        }
        token
    }

    /// The token read next when of a kind, moved past.
    fn advance_if(&mut self, kind: TokenType) -> Option<Token> {
        (self.peek().kind == kind).then(|| self.advance())
    }

    /// The first part of a token.
    fn part(token: &Token, index: usize) -> &str {
        token.parts.get(index).map_or("", String::as_str)
    }

    // The stack.

    /// The innermost open container.
    fn container(&self) -> Option<&Container> {
        self.stack.last()
    }

    /// The definition of an open element; `None` for a block.
    fn container_definition(container: &Container) -> Option<TagDefinition> {
        match container {
            Container::Element(element) => Some(definition(&element.name)),
            Container::Block(_) => None,
        }
    }

    /// The innermost open element.
    fn closest_element(&self) -> Option<&Element> {
        self.stack
            .iter()
            .rev()
            .find_map(|container| match container {
                Container::Element(element) => Some(element),
                Container::Block(_) => None,
            })
    }

    /// Adds a node to the innermost open container, or to the root.
    fn add_to_parent(&mut self, node: Node) {
        match self.stack.last_mut() {
            Some(parent) => parent.children().push(node),
            None => self.root.push(node),
        }
    }

    /// Opens a container, closing the innermost when it is closed by this child.
    fn push_container(&mut self, container: Container, is_closed_by_child: bool) {
        if is_closed_by_child {
            self.pop_one();
        }
        self.stack.push(container);
    }

    /// Closes the innermost container, adding it to its parent.
    fn pop_one(&mut self) {
        if let Some(container) = self.stack.pop() {
            self.add_to_parent(container.into_node());
        }
    }

    /// Closes the innermost container a closing matches, and those above it; whether no
    /// container that needs its own end was skipped (`_popContainer`).
    fn pop_container(&mut self, expected: Expected, end: Option<Span>) -> bool {
        let mut unexpected_close = false;
        for index in (0..self.stack.len()).rev() {
            let container = &self.stack[index];
            let matches = match (expected, container) {
                (Expected::Element(name), Container::Element(element)) => element.name == name,
                (Expected::Block, Container::Block(_)) => true,
                _ => false,
            };
            if matches {
                self.stack[index].close(end);
                while self.stack.len() > index {
                    self.pop_one();
                }
                return !unexpected_close;
            }
            if Self::container_definition(container).is_none_or(|tag| !tag.closed_by_parent) {
                unexpected_close = true;
            }
        }
        false
    }

    /// Closes an open void element before anything that cannot be its child.
    fn close_void_element(&mut self) {
        if self
            .container()
            .and_then(Self::container_definition)
            .is_some_and(|tag| tag.is_void())
        {
            self.pop_one();
        }
    }

    // Nodes.

    /// A comment's text and end.
    fn consume_comment(&mut self, token: &Token) {
        let text = self.advance_if(TokenType::RawText);
        let end = self.advance_if(TokenType::CommentEnd);
        let value = text.map(|text| Self::part(&text, 0).trim().to_owned());
        let span = end.map_or(token.span, |end| Span {
            end: end.span.end,
            ..token.span
        });
        self.add_to_parent(Node::Comment(Comment { value, span }));
    }

    /// An ICU expansion and its cases.
    fn consume_expansion(&mut self, token: &Token) {
        let switch_value = self.advance();
        let kind = self.advance();
        let mut cases = Vec::new();
        while self.peek().kind == TokenType::ExpansionCaseValue {
            let Some(case) = self.parse_expansion_case() else {
                return;
            };
            cases.push(case);
        }
        if self.peek().kind != TokenType::ExpansionFormEnd {
            self.errors.push(ParseError {
                span: self.peek().span,
                message: "Invalid ICU message. Missing '}'.".to_owned(),
            });
            return;
        }
        let span = Span {
            end: self.peek().span.end,
            ..token.span
        };
        self.add_to_parent(Node::Expansion(Expansion {
            switch_value: Self::part(&switch_value, 0).to_owned(),
            kind: Self::part(&kind, 0).to_owned(),
            cases,
            span,
            switch_value_span: switch_value.span,
        }));
        self.advance();
    }

    /// An ICU case, its content read by a builder of its own.
    fn parse_expansion_case(&mut self) -> Option<ExpansionCase> {
        let value = self.advance();
        if self.peek().kind != TokenType::ExpansionCaseExpStart {
            self.errors.push(ParseError {
                span: self.peek().span,
                message: "Invalid ICU message. Missing '{'.".to_owned(),
            });
            return None;
        }
        let start = self.advance();
        let mut tokens = self.collect_expansion_tokens(&start)?;
        let end = self.advance();
        tokens.push(Token {
            kind: TokenType::Eof,
            parts: Vec::new(),
            span: end.span,
        });
        let mut case_builder = Self::new(tokens);
        case_builder.build();
        if !case_builder.errors.is_empty() {
            self.errors.append(&mut case_builder.errors);
            return None;
        }
        Some(ExpansionCase {
            value: Self::part(&value, 0).to_owned(),
            expression: case_builder.root,
            span: Span {
                end: end.span.end,
                ..value.span
            },
            value_span: value.span,
            expression_span: Span {
                end: end.span.end,
                ..start.span
            },
        })
    }

    /// The tokens of an ICU case, up to its `}`.
    fn collect_expansion_tokens(&mut self, start: &Token) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut stack = vec![TokenType::ExpansionCaseExpStart];
        let missing = |builder: &mut Self| {
            builder.errors.push(ParseError {
                span: start.span,
                message: "Invalid ICU message. Missing '}'.".to_owned(),
            });
        };
        loop {
            let kind = self.peek().kind;
            if matches!(
                kind,
                TokenType::ExpansionFormStart | TokenType::ExpansionCaseExpStart
            ) {
                stack.push(kind);
            }
            if kind == TokenType::ExpansionCaseExpEnd {
                if stack.last() == Some(&TokenType::ExpansionCaseExpStart) {
                    stack.pop();
                    if stack.is_empty() {
                        return Some(tokens);
                    }
                } else {
                    missing(self);
                    return None;
                }
            }
            if kind == TokenType::ExpansionFormEnd {
                if stack.last() == Some(&TokenType::ExpansionFormStart) {
                    stack.pop();
                } else {
                    missing(self);
                    return None;
                }
            }
            if kind == TokenType::Eof {
                missing(self);
                return None;
            }
            tokens.push(self.advance());
        }
    }

    /// Text with the texts, interpolations and entities after it, entities decoded.
    fn consume_text(&mut self, token: Token) {
        let start = token.span;
        let mut text = Self::part(&token, 0).to_owned();
        let mut first = token;
        if text.starts_with('\n')
            && self.container().is_some_and(|parent| {
                let tag = Self::container_definition(parent);
                let empty = match parent {
                    Container::Element(element) => element.children.is_empty(),
                    Container::Block(block) => block.children.is_empty(),
                };
                empty && tag.is_some_and(|tag| tag.ignore_first_lf)
            })
        {
            text.remove(0);
            first.parts = vec![text.clone()];
        }
        let mut end = start;
        let mut tokens = vec![first];
        while matches!(
            self.peek().kind,
            TokenType::Interpolation | TokenType::Text | TokenType::EncodedEntity
        ) {
            let next = self.advance();
            match next.kind {
                TokenType::Interpolation => text.push_str(&decode_entities(&next.parts.concat())),
                TokenType::EncodedEntity => text.push_str(Self::part(&next, 0)),
                _ => text.push_str(&next.parts.concat()),
            }
            end = next.span;
            tokens.push(next);
        }
        if !text.is_empty() {
            self.add_to_parent(Node::Text(Text {
                value: text,
                span: Span {
                    end: end.end,
                    ..start
                },
                tokens,
            }));
        }
    }

    /// A start tag and its attributes; the element opened, or closed at once.
    fn consume_element_start_tag(&mut self, start: &Token) {
        let mut attrs = Vec::new();
        let mut comments = Vec::new();
        loop {
            match self.peek().kind {
                TokenType::AttrName => {
                    let name = self.advance();
                    attrs.push(self.consume_attr(&name));
                }
                TokenType::InElementComment => {
                    let comment = self.advance();
                    comments.push(StartTagComment {
                        text: Self::part(&comment, 0).to_owned(),
                        multiline: Self::part(&comment, 1) == "multi",
                        span: comment.span,
                    });
                }
                _ => break,
            }
        }
        let full_name = self.element_full_name(start);
        let tag = definition(&full_name);
        let mut self_closing = false;
        if self.peek().kind == TokenType::TagOpenEndVoid {
            self.advance();
            self_closing = true;
            if !(tag.can_self_close() || ns_prefix(&full_name).is_some() || tag.is_void()) {
                self.errors.push(ParseError {
                    span: start.span,
                    message: format!(
                        "Only void, custom and foreign elements can be self closed \"{}\"",
                        Self::part(start, 1)
                    ),
                });
            }
        } else if self.peek().kind == TokenType::TagOpenEnd {
            self.advance();
        }
        let span = Span {
            end: self.peek().span.full_start,
            ..start.span
        };
        let element = Element {
            name: full_name.clone(),
            attrs,
            children: Vec::new(),
            is_self_closing: self_closing,
            span,
            start_span: span,
            end_span: None,
            is_void: tag.is_void(),
            comments,
        };
        let is_closed_by_child = self
            .container()
            .and_then(Self::container_definition)
            .is_some_and(|parent| parent.is_closed_by_child(&full_name));
        self.push_container(Container::Element(element), is_closed_by_child);
        if self_closing {
            self.pop_container(Expected::Element(&full_name), Some(span));
        } else if start.kind == TokenType::IncompleteTagOpen {
            self.pop_container(Expected::Element(&full_name), None);
            self.errors.push(ParseError {
                span,
                message: format!("Opening tag \"{full_name}\" not terminated."),
            });
        }
    }

    /// An end tag, closing its element.
    fn consume_element_end_tag(&mut self, end: &Token) {
        let full_name = self.element_full_name(end);
        if definition(&full_name).is_void() {
            self.errors.push(ParseError {
                span: end.span,
                message: format!(
                    "Void elements do not have end tags \"{}\"",
                    Self::part(end, 1)
                ),
            });
        } else if !self.pop_container(Expected::Element(&full_name), Some(end.span)) {
            self.errors.push(ParseError {
                span: end.span,
                message: format!(
                    "Unexpected closing tag \"{full_name}\". It may happen when the tag has already been closed by another tag. For more info see https://www.w3.org/TR/html5/syntax.html#closing-elements-that-have-implied-end-tags"
                ),
            });
        }
    }

    /// An attribute's name and value, entities decoded.
    fn consume_attr(&mut self, name: &Token) -> Attribute {
        let full_name = merge_ns_and_name(Self::part(name, 0), Self::part(name, 1));
        let mut end = name.span.end;
        self.advance_if(TokenType::AttrQuote);
        let mut value = String::new();
        let mut value_tokens = Vec::new();
        let mut value_span = None;
        if self.peek().kind == TokenType::AttrValueText {
            let first = self.peek().span;
            let mut value_end = first.end;
            while matches!(
                self.peek().kind,
                TokenType::AttrValueText
                    | TokenType::AttrValueInterpolation
                    | TokenType::EncodedEntity
            ) {
                let token = self.advance();
                match token.kind {
                    TokenType::AttrValueInterpolation => {
                        value.push_str(&decode_entities(&token.parts.concat()));
                    }
                    TokenType::EncodedEntity => value.push_str(Self::part(&token, 0)),
                    _ => value.push_str(&token.parts.concat()),
                }
                value_end = token.span.end;
                end = token.span.end;
                value_tokens.push(token);
            }
            value_span = Some(Span {
                end: value_end,
                ..first
            });
        }
        if let Some(quote) = self.advance_if(TokenType::AttrQuote) {
            end = quote.span.end;
        }
        Attribute {
            name: full_name,
            value,
            span: Span { end, ..name.span },
            key_span: name.span,
            value_span,
            value_tokens,
        }
    }

    /// A block's start and parameters; an incomplete one closed at once with its error.
    fn consume_block_open(&mut self, token: &Token, incomplete: bool) {
        let mut parameters = Vec::new();
        while self.peek().kind == TokenType::BlockParameter {
            let parameter = self.advance();
            parameters.push(BlockParameter {
                expression: Self::part(&parameter, 0).to_owned(),
                span: parameter.span,
            });
        }
        if !incomplete {
            self.advance_if(TokenType::BlockOpenEnd);
        }
        let span = Span {
            end: self.peek().span.full_start,
            ..token.span
        };
        let name = Self::part(token, 0).to_owned();
        let block = Block {
            name: name.clone(),
            parameters,
            children: Vec::new(),
            span,
            name_span: token.span,
            start_span: span,
            end_span: None,
        };
        self.push_container(Container::Block(block), false);
        if incomplete {
            self.pop_container(Expected::Block, None);
            self.errors.push(ParseError {
                span,
                message: format!(
                    "Incomplete block \"{name}\". If you meant to write the @ character, you should use the \"&#64;\" HTML entity instead."
                ),
            });
        }
    }

    /// A `}` closing the innermost block.
    fn consume_block_close(&mut self, token: &Token) {
        let initial = self.stack.len();
        let top = self.container().map(|top| top.name().to_owned());
        if !self.pop_container(Expected::Block, Some(token.span)) {
            let message = if self.stack.len() < initial {
                format!(
                    "Unexpected closing block. The block may have been closed earlier. Did you forget to close the <{}> element? If you meant to write the `}}` character, you should use the \"&#125;\" HTML entity instead.",
                    top.unwrap_or_default()
                )
            } else {
                "Unexpected closing block. The block may have been closed earlier. If you meant to write the `}` character, you should use the \"&#125;\" HTML entity instead.".to_owned()
            };
            self.errors.push(ParseError {
                span: token.span,
                message,
            });
        }
    }

    /// A complete `@let`.
    fn consume_let(&mut self, start: &Token) {
        let name = Self::part(start, 0).to_owned();
        if self.peek().kind != TokenType::LetValue {
            self.errors.push(ParseError {
                span: start.span,
                message: format!(
                    "Invalid @let declaration \"{name}\". Declaration must have a value."
                ),
            });
            return;
        }
        let value = self.advance();
        if self.peek().kind != TokenType::LetEnd {
            self.errors.push(ParseError {
                span: start.span,
                message: format!(
                    "Unterminated @let declaration \"{name}\". Declaration must be terminated with a semicolon."
                ),
            });
            return;
        }
        let end = self.advance();
        let name_span = Span {
            start: start.span.end - offset_of(&name),
            end: start.span.end,
            full_start: start.span.end - offset_of(&name),
        };
        self.add_to_parent(Node::LetDeclaration(LetDeclaration {
            name,
            value: Self::part(&value, 0).to_owned(),
            span: Span {
                end: end.span.end,
                ..start.span
            },
            name_span,
            value_span: value.span,
        }));
    }

    /// A `@let` cut short: its node when it has a name, and its error.
    fn consume_incomplete_let(&mut self, token: &Token) {
        let name = Self::part(token, 0).to_owned();
        if !name.is_empty() {
            let name_start = token.span.end - offset_of(&name);
            self.add_to_parent(Node::LetDeclaration(LetDeclaration {
                name: name.clone(),
                value: String::new(),
                span: token.span,
                name_span: Span {
                    start: name_start,
                    end: token.span.end,
                    full_start: name_start,
                },
                value_span: Span {
                    start: token.span.start,
                    end: token.span.start,
                    full_start: token.span.start,
                },
            }));
        }
        let quoted = if name.is_empty() {
            String::new()
        } else {
            format!(" \"{name}\"")
        };
        self.errors.push(ParseError {
            span: token.span,
            message: format!(
                "Incomplete @let declaration{quoted}. @let declarations must be written as `@let <name> = <value>;`"
            ),
        });
    }

    /// A tag's full name: its prefix, or its definition's, or its parent element's namespace.
    fn element_full_name(&self, token: &Token) -> String {
        let name = Self::part(token, 1);
        let mut prefix = Self::part(token, 0);
        if prefix.is_empty() {
            prefix = definition(name)
                .implicit_namespace_prefix()
                .unwrap_or_default();
        }
        if prefix.is_empty()
            && let Some(parent) = self.closest_element()
        {
            let parent_tag = definition(split_ns_name(&parent.name).1);
            if !parent_tag.prevent_namespace_inheritance() {
                prefix = ns_prefix(&parent.name).unwrap_or_default();
            }
        }
        merge_ns_and_name(prefix, name)
    }
}

/// The length of a name, as an offset back from the end of its token.
fn offset_of(name: &str) -> u32 {
    super::lexer::offset(name.len())
}

/// Entities written inside an interpolation decoded, as the compiler's `decodeEntity` does:
/// a named one, `&#x…;` and `&#…;`; any other `&…;` kept.
fn decode_entities(text: &str) -> String {
    let mut decoded = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(at) = rest.find('&') {
        decoded.push_str(&rest[..at]);
        let after = &rest[at + 1..];
        let Some(end) = after.find(';').filter(|end| *end > 0) else {
            decoded.push('&');
            rest = after;
            continue;
        };
        let entity = &after[..end];
        if let Some(text) = decode_entity(entity) {
            decoded.push_str(&text);
        } else {
            decoded.push('&');
            decoded.push_str(entity);
            decoded.push(';');
        }
        rest = &after[end + 1..];
    }
    decoded.push_str(rest);
    decoded
}

/// The text of an entity's name, `#x41` or `#65` or `amp`.
fn decode_entity(entity: &str) -> Option<String> {
    if let Some(text) = super::entities::named(entity) {
        return Some(text);
    }
    let (digits, radix) = match entity.strip_prefix('#') {
        Some(hex) if hex.starts_with(['x', 'X']) => (&hex[1..], 16),
        Some(decimal) => (decimal, 10),
        None => return None,
    };
    if digits.is_empty() || !digits.chars().all(|each| each.is_digit(radix)) {
        return None;
    }
    u32::from_str_radix(digits, radix)
        .ok()
        .and_then(char::from_u32)
        .map(String::from)
}

#[cfg(test)]
mod tests {
    use super::{Node, parse};

    /// The tree written short: each node's kind and name or value, its children indented.
    fn humanize(source: &str) -> Vec<String> {
        let parsed = parse(source);
        let mut lines = Vec::new();
        walk(&parsed.nodes, 0, &mut lines);
        lines
    }

    /// Writes the nodes at a depth.
    fn walk(nodes: &[Node], depth: usize, lines: &mut Vec<String>) {
        for node in nodes {
            let line = match node {
                Node::Element(element) => {
                    let attrs: Vec<String> = element
                        .attrs
                        .iter()
                        .map(|attr| format!("{}={}", attr.name, attr.value))
                        .collect();
                    format!("Element {} [{}]", element.name, attrs.join(" "))
                }
                Node::Text(text) => format!("Text {:?}", text.value),
                Node::Comment(comment) => format!("Comment {:?}", comment.value),
                Node::Expansion(expansion) => {
                    format!("Expansion {} {}", expansion.switch_value, expansion.kind)
                }
                Node::Block(block) => {
                    let parameters: Vec<&str> = block
                        .parameters
                        .iter()
                        .map(|parameter| parameter.expression.as_str())
                        .collect();
                    format!("Block {} ({})", block.name, parameters.join("; "))
                }
                Node::LetDeclaration(declaration) => {
                    format!("Let {} = {}", declaration.name, declaration.value)
                }
            };
            lines.push(format!("{}{line}", "  ".repeat(depth)));
            match node {
                Node::Element(element) => walk(&element.children, depth + 1, lines),
                Node::Block(block) => walk(&block.children, depth + 1, lines),
                Node::Expansion(expansion) => {
                    for case in &expansion.cases {
                        lines.push(format!("{}Case {}", "  ".repeat(depth + 1), case.value));
                        walk(&case.expression, depth + 2, lines);
                    }
                }
                _ => {}
            }
        }
    }

    /// Each error's message and its range.
    fn errors(source: &str) -> Vec<(String, u32, u32)> {
        parse(source)
            .errors
            .into_iter()
            .map(|error| (error.message, error.span.start, error.span.end))
            .collect()
    }

    #[test]
    fn builds_elements_attributes_text_and_comments() {
        assert_eq!(
            humanize("<div class=\"a\" [x]=\"b &amp;&amp; c\">t {{ d }}&amp;<!-- e --></div>"),
            [
                "Element div [class=a [x]=b && c]",
                "  Text \"t {{ d }}&\"",
                "  Comment Some(\"e\")",
            ]
        );
    }

    #[test]
    fn closes_void_and_implied_elements() {
        assert_eq!(
            humanize("<ul><li>a<li>b<input>c</ul><p>d<div>e</div>"),
            [
                "Element ul []",
                "  Element li []",
                "    Text \"a\"",
                "  Element li []",
                "    Text \"b\"",
                "    Element input []",
                "    Text \"c\"",
                "Element p []",
                "  Text \"d\"",
                "Element div []",
                "  Text \"e\"",
            ]
        );
    }

    #[test]
    fn gives_svg_children_their_namespace() {
        assert_eq!(
            humanize("<svg><path/><foreignObject><div></div></foreignObject></svg>"),
            [
                "Element :svg:svg []",
                "  Element :svg:path []",
                "  Element :svg:foreignObject []",
                "    Element div []",
            ]
        );
    }

    #[test]
    fn builds_control_flow_blocks_and_let() {
        assert_eq!(
            humanize(
                "@if (a; as b) {x} @else if (c) {y} @else {z}\n@for (i of l; track i) {<i>{{i}}</i>} @empty {none}\n@switch (k) { @case (1) {one} @default {other} }\n@defer (on viewport) {big} @placeholder {small}\n@let n = a | p: 1;"
            ),
            [
                "Block if (a; as b)",
                "  Text \"x\"",
                "Text \" \"",
                "Block else if (c)",
                "  Text \"y\"",
                "Text \" \"",
                "Block else ()",
                "  Text \"z\"",
                "Text \"\\n\"",
                "Block for (i of l; track i)",
                "  Element i []",
                "    Text \"{{i}}\"",
                "Text \" \"",
                "Block empty ()",
                "  Text \"none\"",
                "Text \"\\n\"",
                "Block switch (k)",
                "  Text \" \"",
                "  Block case (1)",
                "    Text \"one\"",
                "  Text \" \"",
                "  Block default ()",
                "    Text \"other\"",
                "  Text \" \"",
                "Text \"\\n\"",
                "Block defer (on viewport)",
                "  Text \"big\"",
                "Text \" \"",
                "Block placeholder ()",
                "  Text \"small\"",
                "Text \"\\n\"",
                "Let n = a | p: 1",
            ]
        );
    }

    #[test]
    fn builds_icu_expansions() {
        assert_eq!(
            humanize("{n, plural, =0 {none} other {<b>{{n}}</b>}}"),
            [
                "Expansion n plural",
                "  Case =0",
                "    Text \"none\"",
                "  Case other",
                "    Element b []",
                "      Text \"{{n}}\"",
            ]
        );
    }

    #[test]
    fn drops_the_first_line_feed_of_a_pre_and_a_textarea() {
        assert_eq!(
            humanize("<pre>\na</pre><textarea>\nb</textarea>"),
            [
                "Element pre []",
                "  Text \"a\"",
                "Element textarea []",
                "  Text \"b\"",
            ]
        );
    }

    #[test]
    fn spans_an_element_from_its_start_tag_to_its_end_tag() {
        let parsed = parse("<a href=\"x\">t</a>");
        let Some(Node::Element(element)) = parsed.nodes.first() else {
            panic!("an element");
        };
        // `<a href="x">` is bytes 0 to 12, `</a>` 13 to 17.
        assert_eq!((element.start_span.start, element.start_span.end), (0, 12));
        assert_eq!(
            element.end_span.map(|span| (span.start, span.end)),
            Some((13, 17))
        );
        assert_eq!((element.span.start, element.span.end), (0, 17));
        let attr = &element.attrs[0];
        // `href="x"`: the name 3 to 7, the value 9 to 10, the whole 3 to 11.
        assert_eq!((attr.key_span.start, attr.key_span.end), (3, 7));
        assert_eq!(
            attr.value_span.map(|span| (span.start, span.end)),
            Some((9, 10))
        );
        assert_eq!((attr.span.start, attr.span.end), (3, 11));
    }

    #[test]
    fn reports_the_compilers_tree_errors() {
        assert_eq!(
            errors("<div/>"),
            [(
                "Only void, custom and foreign elements can be self closed \"div\"".to_owned(),
                0,
                4
            )]
        );
        assert_eq!(
            errors("</b>"),
            [(
                "Unexpected closing tag \"b\". It may happen when the tag has already been closed by another tag. For more info see https://www.w3.org/TR/html5/syntax.html#closing-elements-that-have-implied-end-tags".to_owned(),
                0,
                4
            )]
        );
        assert_eq!(
            errors("<br></br>"),
            [("Void elements do not have end tags \"br\"".to_owned(), 4, 9)]
        );
        assert_eq!(errors("@if (a) {")[0].0, "Unclosed block \"if\"");
        assert_eq!(
            errors("}")[0].0,
            "Unexpected closing block. The block may have been closed earlier. If you meant to write the `}` character, you should use the \"&#125;\" HTML entity instead."
        );
        assert_eq!(
            errors("@if (a) {<div>}")[0].0,
            "Unexpected closing block. The block may have been closed earlier. Did you forget to close the <div> element? If you meant to write the `}` character, you should use the \"&#125;\" HTML entity instead."
        );
        assert_eq!(
            errors("@if (a)")[0].0,
            "Incomplete block \"if\". If you meant to write the @ character, you should use the \"&#64;\" HTML entity instead."
        );
        assert_eq!(
            errors("@let x;")[0].0,
            "Incomplete @let declaration \"x\". @let declarations must be written as `@let <name> = <value>;`"
        );
        assert_eq!(
            errors("<a b=\"c\" <i></i>")[0].0,
            "Opening tag \"a\" not terminated."
        );
        assert_eq!(
            errors("{a, b, =0 {y}")[0].0,
            "Invalid ICU message. Missing '}'."
        );
        assert_eq!(
            errors("@let a = 1"),
            [(
                "Incomplete @let declaration \"a\". @let declarations must be written as `@let <name> = <value>;`".to_owned(),
                0,
                10
            )]
        );
    }

    #[test]
    fn decodes_entities_inside_interpolations_as_the_compiler_does() {
        assert_eq!(
            super::decode_entities("a &amp;&amp; b &#x41;&#66; &nope; & c &;"),
            "a && b AB &nope; & c &;"
        );
    }
}
