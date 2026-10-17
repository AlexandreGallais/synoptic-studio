//! `jsdoc/informative-docs` with `@kind` left out: a description, or a tag's, says more than the
//! name it documents (`eslint-plugin-jsdoc` 65.3.1, `src/rules/informativeDocs.js`, on
//! `are-docs-informative` 0.1.1): once the words of the names and the useless words (`a`, `an`,
//! `i`, `in`, `of`, `s`, `the`) are taken out, a word is left.

use std::collections::BTreeSet;

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, Declaration, ExportDefaultDeclarationKind, Expression, PropertyKey,
};
use oxc_semantic::NodeId;

use crate::finding::Finding;
use crate::jsdoc::block::trim;
use crate::jsdoc::{self, Doc, context, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct InformativeDocs;

/// The words a description may hold without saying anything.
const USELESS: [&str; 7] = ["a", "an", "i", "in", "of", "s", "the"];

/// The words of a text: letters, digits and `_` runs, split where a lower-case letter meets an
/// upper-case one, lower-cased, `a` read as `an` and `our` (`splitTextIntoWords`).
fn words(text: &str) -> Vec<String> {
    let spaced: String = text
        .chars()
        .map(|each| {
            if each.is_alphabetic() || each.is_numeric() || each == '_' {
                each
            } else {
                ' '
            }
        })
        .collect();
    let characters: Vec<char> = spaced.chars().collect();
    let mut split = String::new();
    let mut index = 0;
    while index < characters.len() {
        split.push(characters[index]);
        if index + 1 < characters.len()
            && characters[index].is_lowercase()
            && characters[index + 1].is_uppercase()
        {
            split.push(' ');
            split.push(characters[index + 1]);
            index += 2;
            continue;
        }
        index += 1;
    }
    split
        .trim()
        .split(' ')
        .flat_map(|word| {
            let lower = word.to_lowercase();
            if lower == "a" {
                vec!["an".to_owned(), "our".to_owned()]
            } else {
                vec![lower]
            }
        })
        .filter(|word| !word.is_empty())
        .collect()
}

/// Whether a text says more than names (`areDocsInformative`).
fn is_informative(docs: &str, names: &str) -> bool {
    let mut left: BTreeSet<String> = words(docs).into_iter().collect();
    for name in words(names) {
        left.remove(&name);
    }
    for useless in USELESS {
        left.remove(useless);
    }
    !left.is_empty()
}

/// The name of a binding, when it is an identifier.
fn binding_names(pattern: &BindingPattern) -> Vec<String> {
    match pattern {
        BindingPattern::BindingIdentifier(id) => vec![id.name.to_string()],
        _ => Vec::new(),
    }
}

/// The names of a property key, when it is an identifier.
fn key_names(key: &PropertyKey) -> Vec<String> {
    match key {
        PropertyKey::StaticIdentifier(id) => vec![id.name.to_string()],
        PropertyKey::Identifier(id) => vec![id.name.to_string()],
        _ => Vec::new(),
    }
}

/// The names an expression gives: an identifier, or a named function or class.
fn expression_names(expression: &Expression) -> Vec<String> {
    match expression {
        Expression::Identifier(id) => vec![id.name.to_string()],
        Expression::FunctionExpression(function) => {
            function.id.iter().map(|id| id.name.to_string()).collect()
        }
        Expression::ClassExpression(class) => {
            class.id.iter().map(|id| id.name.to_string()).collect()
        }
        _ => Vec::new(),
    }
}

/// The names a declaration gives.
fn declaration_names(declaration: &Declaration) -> Vec<String> {
    match declaration {
        Declaration::VariableDeclaration(variable) => variable
            .declarations
            .first()
            .map(|first| {
                let mut names = binding_names(&first.id);
                names.extend(first.init.iter().flat_map(expression_names));
                names
            })
            .unwrap_or_default(),
        Declaration::FunctionDeclaration(function) => {
            function.id.iter().map(|id| id.name.to_string()).collect()
        }
        Declaration::ClassDeclaration(class) => {
            class.id.iter().map(|id| id.name.to_string()).collect()
        }
        Declaration::TSTypeAliasDeclaration(alias) => vec![alias.id.name.to_string()],
        Declaration::TSInterfaceDeclaration(interface) => vec![interface.id.name.to_string()],
        Declaration::TSEnumDeclaration(declaration) => vec![declaration.id.name.to_string()],
        Declaration::TSNamespaceDeclaration(namespace) => vec![namespace.id.name.to_string()],
        _ => Vec::new(),
    }
}

/// The names a node gives (`getNamesFromNode`).
fn node_names(source: &Source, id: NodeId) -> Vec<String> {
    let nodes = source.semantic.nodes();
    match nodes.kind(id) {
        AstKind::MethodDefinition(method) => {
            let class = nodes.parent_id(nodes.parent_id(id));
            let mut names = node_names(source, class);
            names.extend(key_names(&method.key));
            names
        }
        AstKind::PropertyDefinition(property) => {
            let class = nodes.parent_id(nodes.parent_id(id));
            let mut names = node_names(source, class);
            names.extend(key_names(&property.key));
            names
        }
        AstKind::AccessorProperty(property) => {
            let class = nodes.parent_id(nodes.parent_id(id));
            let mut names = node_names(source, class);
            names.extend(key_names(&property.key));
            names
        }
        AstKind::Class(class) => class.id.iter().map(|id| id.name.to_string()).collect(),
        AstKind::Function(function) => function.id.iter().map(|id| id.name.to_string()).collect(),
        AstKind::TSEnumDeclaration(declaration) => vec![declaration.id.name.to_string()],
        AstKind::TSEnumMember(member) => match &member.id {
            oxc_ast::ast::TSEnumMemberName::Identifier(id) => vec![id.name.to_string()],
            _ => Vec::new(),
        },
        AstKind::TSInterfaceDeclaration(interface) => vec![interface.id.name.to_string()],
        AstKind::TSTypeAliasDeclaration(alias) => vec![alias.id.name.to_string()],
        AstKind::TSNamespaceDeclaration(namespace) => vec![namespace.id.name.to_string()],
        AstKind::ExportDeclaration(export) => declaration_names(&export.declaration),
        AstKind::ExportDefaultDeclaration(export) => match &export.declaration {
            ExportDefaultDeclarationKind::FunctionDeclaration(function) => {
                function.id.iter().map(|id| id.name.to_string()).collect()
            }
            ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                class.id.iter().map(|id| id.name.to_string()).collect()
            }
            ExportDefaultDeclarationKind::Identifier(id) => vec![id.name.to_string()],
            _ => Vec::new(),
        },
        AstKind::ObjectProperty(property) => key_names(&property.key),
        AstKind::VariableDeclaration(variable) => variable
            .declarations
            .first()
            .map(|first| {
                let mut names = binding_names(&first.id);
                names.extend(first.init.iter().flat_map(expression_names));
                names
            })
            .unwrap_or_default(),
        AstKind::VariableDeclarator(declarator) => {
            let mut names = binding_names(&declarator.id);
            names.extend(declarator.init.iter().flat_map(expression_names));
            names
        }
        AstKind::IdentifierReference(id) => vec![id.name.to_string()],
        AstKind::BindingIdentifier(id) => vec![id.name.to_string()],
        AstKind::IdentifierName(id) => vec![id.name.to_string()],
        _ => Vec::new(),
    }
}

impl InformativeDocs {
    /// The findings of one block.
    fn check(&self, source: &Source, doc: &Doc, names: &[String], findings: &mut Vec<Finding>) {
        let names = names.join(" ");
        let redundant = |text: &str, extra: &str| {
            let kept = trim(text);
            !kept.is_empty() && !is_informative(kept, &format!("{extra} {names}"))
        };
        let description = text::description(&doc.block);
        let mut description_reported = false;
        for tag in doc.block.tags.iter().filter(|tag| tag.tag != "kind") {
            if redundant(&tag.description, &tag.name) {
                findings.push(doc.finding_at(
                    source,
                    self.name(),
                    tag.lines[0],
                    None,
                    "This tag description only repeats the name it describes.".to_owned(),
                ));
                description_reported |=
                    tag.description == description.text && tag.lines[0] == description.last_line;
            }
        }
        if !description_reported && redundant(&description.text, "") {
            findings.push(doc.finding(
                source,
                self.name(),
                "This description only repeats the name it describes.".to_owned(),
            ));
        }
    }
}

impl Rule for InformativeDocs {
    fn name(&self) -> &'static str {
        "jsdoc/informative-docs"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let owners = context::owners(source);
        for doc in jsdoc::all(source) {
            let names = owners
                .get(&doc.comment.span.start)
                .map(|id| node_names(source, *id))
                .unwrap_or_default();
            self.check(source, &doc, &names, findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InformativeDocs, is_informative};
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&InformativeDocs, "informative-docs", "src/a.ts", &[]);
    }

    #[test]
    fn reads_words_split_by_case_aliases_and_useless_words() {
        // `the` is useless, `user` and `name` come from `userName`: nothing left.
        assert!(!is_informative("The user name.", "userName"));
        // `a` reads as `an` and `our`: `our` is left.
        assert!(is_informative("A user.", "user"));
        assert!(is_informative(
            "Name of the user who signed in.",
            "userName"
        ));
    }
}
