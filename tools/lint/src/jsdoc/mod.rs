//! What the rules of `eslint-plugin-jsdoc` 65.3.1 share (EN-083): the `JSDoc` blocks of a file
//! cut into lines and tags (`block`), the block a declaration carries, found as
//! `@es-joy/jsdoccomment` finds it (`getJSDocComment`, one line at most between them), and the
//! places their findings point to (`iterateJsdoc.js`, `makeReport`).

pub mod block;
#[cfg(test)]
pub mod cases;
pub mod context;
pub mod edit;
pub mod flow;
pub mod params;
pub mod text;

use oxc_ast::AstKind;
use oxc_ast::ast::{Comment, FunctionType};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::{Finding, Fix};
use crate::source::Source;

use block::{Block, is_space};

/// A `JSDoc` block of a file.
pub struct Doc<'c> {
    /// Its comment.
    pub comment: Comment,
    /// Its text, from `/**` to `*/`.
    pub text: &'c str,
    /// Its lines and tags.
    pub block: Block,
    /// The blanks before it on its first line, as many as its column, each the line's first
    /// character when that is a blank, a space otherwise (`getIndentAndJSDoc`).
    pub indent: String,
}

impl Doc<'_> {
    /// A finding on the comment's first character.
    #[must_use]
    pub fn finding(&self, source: &Source, rule: &'static str, message: String) -> Finding {
        source.finding(rule, self.comment.span.start, message)
    }

    /// A finding on the first column of a line of the block, from 0, and the column after the
    /// comment's own when given (`makeReport` with a line and a column).
    #[must_use]
    pub fn finding_at(
        &self,
        source: &Source,
        rule: &'static str,
        line: usize,
        column: Option<usize>,
        message: String,
    ) -> Finding {
        let (first, start) = source.place(self.comment.span.start);

        Finding {
            path: source.path.to_owned(),
            line: first + line,
            column: column.map_or(1, |column| start + column),
            rule,
            message,
            fix: None,
        }
    }

    /// The fix that writes the block's lines back over the comment.
    #[must_use]
    pub fn rewrite(&self, block: &Block) -> Fix {
        self.replace(block::stringify(block))
    }

    /// The fix that writes a text over the comment.
    #[must_use]
    pub fn replace(&self, text: String) -> Fix {
        Fix {
            start: self.comment.span.start,
            end: self.comment.span.end,
            text,
        }
    }
}

/// The line end of a block line a fix adds: a carriage return when the file ends its lines with
/// CR LF (`utils.newLineEnd`).
#[must_use]
pub fn new_line_end(source: &Source) -> String {
    if source.text.contains("\r\n") {
        "\r".to_owned()
    } else {
        String::new()
    }
}

/// The text of a comment, delimiters included.
#[must_use]
pub fn comment_text<'a>(source: &Source<'_, 'a>, comment: &Comment) -> &'a str {
    source
        .text
        .get(comment.span.start as usize..comment.span.end as usize)
        .unwrap_or_default()
}

/// Whether a comment is a `JSDoc` block the rules read: `/**` then a blank.
#[must_use]
pub fn is_jsdoc_block(source: &Source, comment: &Comment) -> bool {
    comment.is_block()
        && comment_text(source, comment)
            .strip_prefix("/**")
            .is_some_and(|rest| rest.starts_with(is_space))
}

/// A comment parsed into a block, with its indent.
#[must_use]
pub fn doc<'c>(source: &Source<'_, 'c>, comment: Comment) -> Doc<'c> {
    let text = comment_text(source, &comment);
    let start = comment.span.start as usize;
    let line_start = source.text[..start].rfind('\n').map_or(0, |at| at + 1);
    let first = source.text[line_start..].chars().next().unwrap_or(' ');
    let fill = if first == ' ' || first == '\t' {
        first
    } else {
        ' '
    };
    let column = source.place(comment.span.start).1 - 1;

    Doc {
        comment,
        text,
        block: block::parse(text),
        indent: fill.to_string().repeat(column),
    }
}

/// Every `JSDoc` block of the file, in order (`iterateAllJsdocs`).
#[must_use]
pub fn all<'c>(source: &Source<'_, 'c>) -> Vec<Doc<'c>> {
    source
        .program
        .comments
        .iter()
        .filter(|comment| is_jsdoc_block(source, comment))
        .map(|comment| doc(source, comment.clone()))
        .collect()
}

/// What comes right before an offset, blanks skipped: a comment, or the last character of code.
pub enum Before {
    /// A comment ends there.
    Comment(Comment),
    /// A character of code ends there, at an offset.
    Code(char, u32),
    /// The file's start.
    Start,
}

/// What comes right before an offset, blanks skipped (`getTokenBefore` with comments).
pub fn before(source: &Source, offset: u32) -> Before {
    let kept = source.text[..offset as usize].trim_end_matches(is_space);
    let end = u32::try_from(kept.len()).unwrap_or(u32::MAX);
    let comments = &source.program.comments;
    let after = comments.partition_point(|comment| comment.span.end <= end);
    if let Some(comment) = after
        .checked_sub(1)
        .and_then(|index| comments.get(index))
        .filter(|comment| comment.span.end == end)
    {
        return Before::Comment(comment.clone());
    }
    kept.chars().next_back().map_or(Before::Start, |character| {
        Before::Code(
            character,
            end - u32::try_from(character.len_utf8()).unwrap_or(1),
        )
    })
}

/// Whether a comment ends right before an offset, blanks aside (`getCommentsBefore`).
#[must_use]
pub fn has_comment_before(source: &Source, offset: u32) -> bool {
    matches!(before(source, offset), Before::Comment(_))
}

/// The `JSDoc` comment right before a node starting at an offset: line comments between them
/// skipped, a `(` before the node too, and the node one line below the comment at most
/// (`findJSDocComment`, `maxLines` 1).
#[must_use]
pub fn comment_before(source: &Source, offset: u32) -> Option<Comment> {
    let mut current = offset;
    let (comment, measured_from) = loop {
        let mut paren = None;
        let mut found = before(source, current);
        if let Before::Code('(', at) = found {
            paren = Some(at);
            found = before(source, at);
        }
        match found {
            Before::Comment(comment) if !comment.is_block() => current = comment.span.start,
            Before::Comment(comment) => {
                let measured_from = paren.map_or(comment.span.end, |at| at + 1);
                break (comment, measured_from);
            }
            _ => return None,
        }
    };
    let text = comment_text(source, &comment);
    let jsdoc = text
        .strip_prefix("/**")
        .is_some_and(|rest| rest.starts_with(is_space));
    let gap = source
        .line(current)
        .checked_sub(source.line(measured_from.saturating_sub(1)));

    (jsdoc && gap.is_some_and(|gap| gap <= 1)).then_some(comment)
}

/// The comment right before a node starting at an offset that is not a `JSDoc` block: a line
/// comment or another block comment, a `(` before the node skipped, the node one line below it at
/// most (`findJSDocComment` with `nonJSDoc`).
#[must_use]
pub fn plain_comment_before(source: &Source, offset: u32) -> Option<Comment> {
    let mut paren = None;
    let mut found = before(source, offset);
    if let Before::Code('(', at) = found {
        paren = Some(at);
        found = before(source, at);
    }
    let Before::Comment(comment) = found else {
        return None;
    };
    let jsdoc = comment.is_block()
        && comment_text(source, &comment)
            .strip_prefix("/**")
            .is_some_and(|rest| rest.starts_with(is_space));
    let measured_from = paren.map_or(comment.span.end, |at| at + 1);
    let gap = source
        .line(offset)
        .checked_sub(source.line(measured_from.saturating_sub(1)));
    (!jsdoc && gap.is_some_and(|gap| gap <= 1)).then_some(comment)
}

/// The comment right after a node ending at an offset, `;` skipped, starting on the node's last
/// line (`getFollowingComment`, its first try).
#[must_use]
pub fn comment_after(source: &Source, end: u32) -> Option<Comment> {
    let mut at = end as usize;
    loop {
        let rest = &source.text[at..];
        let skipped = rest.len() - rest.trim_start_matches(is_space).len();
        at += skipped;
        if source.text[at..].starts_with(';') {
            at += 1;
            continue;
        }
        break;
    }
    let start = u32::try_from(at).unwrap_or(u32::MAX);
    let comments = &source.program.comments;
    let index = comments.partition_point(|comment| comment.span.start < start);
    comments
        .get(index)
        .filter(|comment| comment.span.start == start)
        .filter(|comment| source.line(end.saturating_sub(1)) == source.line(comment.span.start))
        .cloned()
}

/// Whether a node is an export declaration (`looksLikeExport`).
fn is_export(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::ExportDeclaration(_)
            | AstKind::ExportDefaultDeclaration(_)
            | AstKind::ExportNamedDeclaration(_)
            | AstKind::ExportFromDeclaration(_)
            | AstKind::ExportAllDeclaration(_)
            | AstKind::ExportSpecifier(_)
    )
}

/// Whether a node's type has `Function` in its `ESTree` name.
fn is_function_like(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::Function(_)
            | AstKind::ArrowFunctionExpression(_)
            | AstKind::TSFunctionType(_)
            | AstKind::FormalParameters(_)
            | AstKind::FunctionBody(_)
    )
}

/// Whether a comment before a node of this kind may document the function it holds
/// (`allowableCommentNode`).
fn is_allowable(kind: &AstKind) -> bool {
    matches!(
        kind,
        AstKind::AssignmentPattern(_)
            | AstKind::VariableDeclaration(_)
            | AstKind::ExpressionStatement(_)
            | AstKind::MethodDefinition(_)
            | AstKind::ObjectProperty(_)
            | AstKind::PropertyDefinition(_)
            | AstKind::ExportDefaultDeclaration(_)
            | AstKind::ReturnStatement(_)
    )
}

/// The node whose leading comment documents a node (`getReducedASTNode`): an exported
/// declaration's `export`, a function value's declaration or property.
#[must_use]
pub fn documented_node<'s, 'a>(
    source: &'s Source<'_, 'a>,
    node: &'s AstNode<'a>,
) -> &'s AstNode<'a> {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    match node.kind() {
        AstKind::TSInterfaceDeclaration(_)
        | AstKind::TSTypeAliasDeclaration(_)
        | AstKind::TSEnumDeclaration(_) => {
            if is_export(&parent.kind()) {
                parent
            } else {
                node
            }
        }
        AstKind::Class(class) if class.is_declaration() => {
            if is_export(&parent.kind()) {
                parent
            } else {
                node
            }
        }
        AstKind::Function(function) if function.r#type == FunctionType::FunctionDeclaration => {
            if is_export(&parent.kind()) {
                parent
            } else {
                node
            }
        }
        AstKind::Function(_)
        | AstKind::Class(_)
        | AstKind::ObjectExpression(_)
        | AstKind::ArrowFunctionExpression(_) => value_documented_node(source, node),
        _ => node,
    }
}

/// The node documenting a function, class or object value, from its parents.
fn value_documented_node<'s, 'a>(
    source: &'s Source<'_, 'a>,
    node: &'s AstNode<'a>,
) -> &'s AstNode<'a> {
    let nodes = source.semantic.nodes();
    let start = node.kind().span().start;
    let mut parent = nodes.parent_node(node.id());
    if matches!(
        parent.kind(),
        AstKind::CallExpression(_) | AstKind::NewExpression(_)
    ) {
        return node;
    }
    let mut current = start;
    loop {
        match before(source, current) {
            Before::Code('(', at) => current = at,
            Before::Comment(comment) if comment.is_block() => return node,
            _ => break,
        }
    }
    if has_comment_before(source, start) {
        return node;
    }
    while !has_comment_before(source, parent.kind().span().start)
        && !is_function_like(&parent.kind())
        && !is_allowable(&parent.kind())
    {
        if matches!(parent.kind(), AstKind::Program(_)) {
            return node;
        }
        parent = nodes.parent_node(parent.id());
    }
    let declaration = matches!(parent.kind(), AstKind::Function(function) if function.r#type == FunctionType::FunctionDeclaration);
    if declaration || matches!(parent.kind(), AstKind::Program(_)) {
        return node;
    }
    let grand = nodes.parent_node(parent.id());
    if matches!(grand.kind(), AstKind::ExportDeclaration(_)) {
        grand
    } else {
        parent
    }
}

/// The `JSDoc` comment a node carries (`getJSDocComment`).
#[must_use]
pub fn comment_of(source: &Source, node: &AstNode) -> Option<Comment> {
    comment_before(source, documented_node(source, node).kind().span().start)
}

/// Whether a node is one of the functions the rules read by default: an arrow function, a
/// function declaration or expression, or a declared function (`contextDefaults`).
#[must_use]
pub fn is_default_context(kind: &AstKind) -> bool {
    match kind {
        AstKind::ArrowFunctionExpression(_) => true,
        AstKind::Function(function) => matches!(
            function.r#type,
            FunctionType::FunctionDeclaration
                | FunctionType::FunctionExpression
                | FunctionType::TSDeclareFunction
        ),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_ast::AstKind;
    use oxc_parser::Parser;
    use oxc_semantic::SemanticBuilder;
    use oxc_span::SourceType;

    use super::{all, comment_of, comment_text};
    use crate::project::Project;
    use crate::source::Source;

    /// The comment each function of a code carries, by the function's name.
    fn documented(code: &str) -> Vec<(String, Option<String>)> {
        let allocator = Allocator::default();
        let program = Parser::new(&allocator, code, SourceType::ts())
            .parse()
            .program;
        let semantic = SemanticBuilder::new()
            .with_build_nodes(true)
            .build(&program)
            .semantic;
        let project = Project::new(std::path::Path::new("/project"));
        let source = Source::new("src/a.ts", &program, &semantic, &[], &project);

        semantic
            .nodes()
            .iter()
            .filter_map(|node| {
                match node.kind() {
                    AstKind::Function(function) => {
                        function.id.as_ref().map(|id| id.name.to_string())
                    }
                    AstKind::ArrowFunctionExpression(_) => Some("arrow".to_owned()),
                    _ => None,
                }
                .map(|name| {
                    let comment = comment_of(&source, node);
                    (
                        name,
                        comment.map(|each| comment_text(&source, &each).to_owned()),
                    )
                })
            })
            .collect()
    }

    #[test]
    fn finds_the_block_a_declaration_carries() {
        let code = "/** A. */\nexport function a(): void {}\n\n/** B. */\n// note\nfunction b(): void {}\n/** C. */\n\nfunction c(): void {}\n/** D. */\nconst d = (): void => {};\n/* E. */\nfunction e(): void {}\n";

        assert_eq!(
            documented(code),
            [
                ("a".to_owned(), Some("/** A. */".to_owned())),
                // A line comment between them is skipped.
                ("b".to_owned(), Some("/** B. */".to_owned())),
                // Two lines below the block: too far.
                ("c".to_owned(), None),
                // An arrow function's block sits before its `const`.
                ("arrow".to_owned(), Some("/** D. */".to_owned())),
                // `/*` is no `JSDoc` block.
                ("e".to_owned(), None),
            ]
        );
    }

    #[test]
    fn reads_every_jsdoc_block_with_its_indent() {
        let allocator = Allocator::default();
        let code = "/** A. */\nconst a = 1;\n  /**\n   * B.\n   */\n/***/\n/**x*/\n";
        let program = Parser::new(&allocator, code, SourceType::ts())
            .parse()
            .program;
        let semantic = SemanticBuilder::new().build(&program).semantic;
        let project = Project::new(std::path::Path::new("/project"));
        let source = Source::new("src/a.ts", &program, &semantic, &[], &project);
        let docs = all(&source);

        // `/***/` and `/**x*/` have no blank after `/**`: left out.
        assert_eq!(
            docs.iter()
                .map(|doc| (doc.text, doc.indent.as_str()))
                .collect::<Vec<_>>(),
            [("/** A. */", ""), ("/**\n   * B.\n   */", "  ")]
        );
    }
}
