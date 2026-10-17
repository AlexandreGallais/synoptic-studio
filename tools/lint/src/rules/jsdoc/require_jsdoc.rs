//! `jsdoc/require-jsdoc` with the project's contexts (`docs/conventions/documentation.md`): every
//! function declaration, class, method, type alias, interface and module-level constant has a
//! block, and in the library every type member too, which the API pages read; an overload's
//! implementation may lean on its first signature's block (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/requireJsdoc.js`, no fixer).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Declaration, FunctionType, MethodDefinitionKind, MethodDefinitionType, PropertyKey, Statement,
    TSSignature,
};
use oxc_semantic::{AstNode, NodeId};
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::glob::matches_any;
use crate::jsdoc::{comment_before, comment_of, documented_node};
use crate::rules::Rule;
use crate::scopes::LIBRARY_FILES;
use crate::source::Source;

/// The rule.
pub struct RequireJsdoc;

/// The name of a key that is an identifier.
fn key_name<'a>(key: &PropertyKey<'a>) -> Option<&'a str> {
    match key {
        PropertyKey::StaticIdentifier(id) => Some(id.name.as_str()),
        _ => None,
    }
}

/// The name an overload carries: a function's, or a method's (`getCurrentOverloadName`).
fn current_name<'a>(kind: &AstKind<'a>) -> Option<&'a str> {
    match kind {
        AstKind::Function(function)
            if matches!(
                function.r#type,
                FunctionType::FunctionDeclaration | FunctionType::TSDeclareFunction
            ) =>
        {
            function.id.as_ref().map(|id| id.name.as_str())
        }
        AstKind::ExportDeclaration(export) => match &export.declaration {
            Declaration::FunctionDeclaration(function) => {
                function.id.as_ref().map(|id| id.name.as_str())
            }
            _ => None,
        },
        AstKind::MethodDefinition(method) if !method.computed => key_name(&method.key),
        _ => None,
    }
}

/// The name a previous overload signature carries (`getPreviousOverloadName`).
fn previous_name<'a>(kind: &AstKind<'a>) -> Option<&'a str> {
    match kind {
        AstKind::Function(function) if function.r#type == FunctionType::TSDeclareFunction => {
            function.id.as_ref().map(|id| id.name.as_str())
        }
        AstKind::ExportDeclaration(export) => match &export.declaration {
            Declaration::FunctionDeclaration(function)
                if function.r#type == FunctionType::TSDeclareFunction =>
            {
                function.id.as_ref().map(|id| id.name.as_str())
            }
            _ => None,
        },
        AstKind::MethodDefinition(method) if !method.computed => key_name(&method.key),
        _ => None,
    }
}

/// The spans of the statements or class members around a node, if it sits in a list.
fn sibling_spans(kind: &AstKind) -> Option<Vec<Span>> {
    let statements = |list: &[Statement]| Some(list.iter().map(GetSpan::span).collect());
    match kind {
        AstKind::Program(program) => statements(&program.body),
        AstKind::BlockStatement(block) => statements(&block.body),
        AstKind::StaticBlock(block) => statements(&block.body),
        AstKind::TSModuleBlock(block) => statements(&block.body),
        AstKind::ClassBody(body) => Some(body.body.iter().map(GetSpan::span).collect()),
        _ => None,
    }
}

/// The node right before a node in its list (`getPreviousOverloadSibling`).
fn previous_sibling<'s, 'a>(
    source: &'s Source<'_, 'a>,
    node: &AstNode<'a>,
) -> Option<&'s AstNode<'a>> {
    let nodes = source.semantic.nodes();
    let parent = nodes.parent_node(node.id());
    let spans = sibling_spans(&parent.kind())?;
    let index = spans.iter().position(|span| *span == node.kind().span())?;
    let previous = *spans.get(index.checked_sub(1)?)?;
    nodes
        .iter()
        .find(|each| nodes.parent_id(each.id()) == parent.id() && each.kind().span() == previous)
}

/// Whether a method has no body: abstract, or a signature (`bodyless`).
fn is_bodyless(kind: &AstKind) -> bool {
    matches!(kind, AstKind::MethodDefinition(method)
        if method.r#type == MethodDefinitionType::TSAbstractMethodDefinition || method.value.body.is_none())
}

/// Whether two methods are overloads of one another, the first without body
/// (`isMatchingMethodOverloadSibling`).
fn is_method_overload(current: &AstKind, previous: &AstKind) -> bool {
    let (AstKind::MethodDefinition(now), AstKind::MethodDefinition(before)) = (current, previous)
    else {
        return false;
    };
    let plain = |kind: MethodDefinitionKind| {
        matches!(
            kind,
            MethodDefinitionKind::Method | MethodDefinitionKind::Constructor
        )
    };
    plain(now.kind)
        && plain(before.kind)
        && !now.computed
        && !before.computed
        && is_bodyless(previous)
        && key_name(&now.key).is_some()
        && key_name(&now.key) == key_name(&before.key)
        && now.kind == before.kind
        && now.r#static == before.r#static
}

/// Whether a node carries a block, or the overload signature before it does
/// (`getJSDocComment` with `checkOverloads`).
fn is_documented(source: &Source, node: &AstNode) -> bool {
    if comment_of(source, node).is_some() {
        return true;
    }
    let reduced = documented_node(source, node);
    let Some(name) = current_name(&reduced.kind()) else {
        return false;
    };
    let Some(previous) = previous_sibling(source, reduced) else {
        return false;
    };
    let is_method = matches!(reduced.kind(), AstKind::MethodDefinition(_));
    previous_name(&previous.kind()) == Some(name)
        && (!is_method || is_method_overload(&reduced.kind(), &previous.kind()))
        && is_documented(source, previous)
}

/// Whether a method's class implements an interface of the file whose member of that name
/// carries a block (`isExemptedImplementer`).
fn is_documented_implementer(source: &Source, node: &AstNode) -> bool {
    let nodes = source.semantic.nodes();
    let AstKind::MethodDefinition(method) = nodes.parent_kind(node.id()) else {
        return false;
    };
    let Some(method_name) = key_name(&method.key) else {
        return false;
    };
    let class = nodes.parent_kind(nodes.parent_id(nodes.parent_id(node.id())));
    let AstKind::Class(class) = class else {
        return false;
    };
    if !class.is_declaration() {
        return false;
    }
    class.implements.iter().any(|implemented| {
        let oxc_ast::ast::TSTypeName::IdentifierReference(interface_name) = &implemented.expression
        else {
            return false;
        };
        let interface_name = interface_name.name.as_str();
        nodes.iter().any(|each| {
            let AstKind::TSInterfaceDeclaration(interface) = each.kind() else {
                return false;
            };
            interface.id.name == interface_name
                && interface
                    .body
                    .body
                    .iter()
                    .find(|member| {
                        let key = match member {
                            TSSignature::TSMethodSignature(signature) => key_name(&signature.key),
                            TSSignature::TSPropertySignature(signature) => key_name(&signature.key),
                            _ => None,
                        };
                        key == Some(method_name)
                    })
                    .is_some_and(|member| comment_before(source, member.span().start).is_some())
        })
    })
}

/// The node a context checks, if the node is one: the function of a method.
fn checked<'s, 'a>(
    source: &'s Source<'_, 'a>,
    node: &'s AstNode<'a>,
    library: bool,
) -> Option<NodeId> {
    let nodes = source.semantic.nodes();
    match node.kind() {
        AstKind::TSTypeAliasDeclaration(_)
        | AstKind::TSInterfaceDeclaration(_)
        | AstKind::Class(_) => Some(node.id()),
        AstKind::TSPropertySignature(_) if library => Some(node.id()),
        AstKind::VariableDeclaration(_)
            if matches!(nodes.parent_kind(node.id()), AstKind::Program(_)) =>
        {
            Some(node.id())
        }
        AstKind::ExportDeclaration(export)
            if matches!(export.declaration, Declaration::VariableDeclaration(_)) =>
        {
            Some(node.id())
        }
        AstKind::Function(function) if function.r#type == FunctionType::FunctionDeclaration => {
            Some(node.id())
        }
        AstKind::MethodDefinition(_) => nodes
            .iter()
            .find(|each| {
                nodes.parent_id(each.id()) == node.id()
                    && matches!(each.kind(), AstKind::Function(_))
            })
            .map(AstNode::id),
        _ => None,
    }
}

impl Rule for RequireJsdoc {
    fn name(&self) -> &'static str {
        "jsdoc/require-jsdoc"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let library = matches_any(LIBRARY_FILES, source.path);
        let nodes = source.semantic.nodes();
        for node in nodes.iter() {
            let Some(id) = checked(source, node, library) else {
                continue;
            };
            let target = nodes.get_node(id);
            if is_documented(source, target) || is_documented_implementer(source, target) {
                continue;
            }
            findings.push(source.finding(
                self.name(),
                target.kind().span().start,
                "Missing JSDoc comment.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireJsdoc;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireJsdoc, "require-jsdoc", "src/a.ts");
    }

    #[test]
    fn leaves_type_members_to_the_library() {
        check(&RequireJsdoc, "require-jsdoc-scripts", "scripts/a.ts");
    }
}
