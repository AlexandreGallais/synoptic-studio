//! The declarations the blocks document (`eslint-plugin-jsdoc` 65.3.1, `src/iterateJsdoc.js`):
//! the functions and other nodes a rule reads with the block each carries, the node a block
//! belongs to when a rule reads every block (the first node, in the walk's order, that carries
//! it), and the blocks a rule leaves alone (`utils.avoidDocs`).

use std::collections::HashMap;

use oxc_ast::AstKind;
use oxc_ast::ast::{MethodDefinitionKind, PropertyKind};
use oxc_semantic::{AstNode, NodeId};

use super::{Doc, comment_of, doc};
use crate::source::Source;

/// The nodes a test accepts, each with the block it carries, in the walk's order.
pub fn documented<'s, 'a>(
    source: &'s Source<'_, 'a>,
    accepts: impl Fn(&AstKind) -> bool,
) -> Vec<(&'s AstNode<'a>, Doc<'a>)> {
    source
        .semantic
        .nodes()
        .iter()
        .filter(|node| accepts(&node.kind()))
        .filter_map(|node| comment_of(source, node).map(|comment| (node, doc(source, comment))))
        .collect()
}

/// The node each block belongs to, by the block's first byte: the first node of the walk,
/// the program left out, that carries it (`iterateAllJsdocs`, `*:not(Program)`).
#[must_use]
pub fn owners(source: &Source) -> HashMap<u32, NodeId> {
    let mut owners = HashMap::new();
    for node in source.semantic.nodes().iter() {
        if matches!(node.kind(), AstKind::Program(_)) {
            continue;
        }
        if let Some(comment) = comment_of(source, node) {
            owners
                .entry(comment.span.start)
                .or_insert_with(|| node.id());
        }
    }
    owners
}

/// The node's own kind as a method or an accessor: the kind of the method or property holding a
/// function, or of the node itself.
fn method_kind(source: &Source, node: &AstNode) -> Option<&'static str> {
    let kind_of = |kind: AstKind| match kind {
        AstKind::MethodDefinition(method) => Some(match method.kind {
            MethodDefinitionKind::Constructor => "constructor",
            MethodDefinitionKind::Get => "get",
            MethodDefinitionKind::Set => "set",
            MethodDefinitionKind::Method => "method",
        }),
        AstKind::ObjectProperty(property) => Some(match property.kind {
            PropertyKind::Get => "get",
            PropertyKind::Set => "set",
            PropertyKind::Init => "init",
        }),
        _ => None,
    };
    kind_of(source.semantic.nodes().parent_kind(node.id()))
}

/// Whether a block holds a tag, its case ignored (`utils.hasTag`).
#[must_use]
pub fn has_tag(doc: &Doc, name: &str) -> bool {
    doc.block
        .tags
        .iter()
        .any(|tag| tag.tag.eq_ignore_ascii_case(name))
}

/// The block of the class around a node, the node itself included (`utils.getClassJsdoc`).
fn class_doc<'a>(source: &Source<'_, 'a>, node: &AstNode<'a>) -> Option<Doc<'a>> {
    let nodes = source.semantic.nodes();
    let class = std::iter::once(node)
        .chain(nodes.ancestors(node.id()))
        .find(|each| matches!(each.kind(), AstKind::Class(_)))?;
    comment_of(source, class).map(|comment| doc(source, comment))
}

/// Whether a rule leaves a block alone: it holds `@ignore`, `@override` or `@implements`, or its
/// class's block does; it documents a constructor, a getter or a setter when the rule does not
/// check them; or it holds `@inheritDoc` (`utils.avoidDocs`, the settings' defaults).
#[must_use]
pub fn avoids(source: &Source, doc: &Doc, node: &AstNode, checks_special_methods: bool) -> bool {
    let class = class_doc(source, node);
    let replaced = ["ignore", "override", "implements"]
        .iter()
        .any(|tag| has_tag(doc, tag) || class.as_ref().is_some_and(|class| has_tag(class, tag)));
    if replaced {
        return true;
    }
    if !checks_special_methods {
        let own = match node.kind() {
            AstKind::MethodDefinition(method)
                if method.kind == MethodDefinitionKind::Constructor =>
            {
                Some("constructor")
            }
            _ => None,
        };
        let kind = own.or_else(|| method_kind(source, node));
        if kind == Some("constructor")
            || has_tag(doc, "class")
            || has_tag(doc, "constructor")
            || matches!(kind, Some("get" | "set"))
        {
            return true;
        }
    }
    doc.block
        .tags
        .iter()
        .any(|tag| tag.tag == "inheritDoc" || tag.tag == "inheritdoc")
}

/// Whether the block of the class around a node holds a tag (`utils.classHasTag`).
#[must_use]
pub fn class_has_tag(source: &Source, node: &AstNode, name: &str) -> bool {
    class_doc(source, node).is_some_and(|class| has_tag(&class, name))
}

/// Whether a node is a constructor or the function of one (`isConstructor`).
#[must_use]
pub fn is_constructor(source: &Source, node: &AstNode) -> bool {
    matches!(node.kind(), AstKind::MethodDefinition(method) if method.kind == MethodDefinitionKind::Constructor)
        || method_kind(source, node) == Some("constructor")
}

/// Whether a block holds one of these tags, case aside (`utils.hasATag`).
#[must_use]
pub fn has_a_tag(doc: &Doc, names: &[&str]) -> bool {
    names.iter().any(|name| has_tag(doc, name))
}
