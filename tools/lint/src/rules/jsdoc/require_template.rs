//! `jsdoc/require-template`: each type parameter of a documented class, function, interface or
//! type alias has its `@template` tag (`eslint-plugin-jsdoc` 65.3.1,
//! `src/rules/requireTemplate.js`, separate tags not required).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Declaration, ExportDefaultDeclarationKind, FunctionType, TSTypeParameterDeclaration,
};

use crate::finding::Finding;
use crate::jsdoc::block::Block;
use crate::jsdoc::{self, Doc, context, text};
use crate::rules::Rule;
use crate::source::Source;

use super::check_template_names::template_names;

/// The rule.
pub struct RequireTemplate;

/// The names of a declaration's type parameters.
fn parameters(declared: Option<&TSTypeParameterDeclaration>) -> Vec<String> {
    declared
        .map(|declared| {
            declared
                .params
                .iter()
                .map(|parameter| parameter.name.name.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// The type parameters of the declaration a block's owner holds (`handleTypes`).
fn declared(kind: &AstKind) -> Option<Vec<String>> {
    Some(match kind {
        AstKind::Class(class) if class.is_declaration() => {
            parameters(class.type_parameters.as_deref())
        }
        AstKind::Function(function)
            if matches!(
                function.r#type,
                FunctionType::FunctionDeclaration | FunctionType::TSDeclareFunction
            ) =>
        {
            parameters(function.type_parameters.as_deref())
        }
        AstKind::TSInterfaceDeclaration(interface) => {
            parameters(interface.type_parameters.as_deref())
        }
        AstKind::TSTypeAliasDeclaration(alias) => parameters(alias.type_parameters.as_deref()),
        AstKind::ExportDefaultDeclaration(export) => match &export.declaration {
            ExportDefaultDeclarationKind::ClassDeclaration(class) => {
                parameters(class.type_parameters.as_deref())
            }
            ExportDefaultDeclarationKind::FunctionDeclaration(function) => {
                parameters(function.type_parameters.as_deref())
            }
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(interface) => {
                parameters(interface.type_parameters.as_deref())
            }
            _ => return None,
        },
        AstKind::ExportDeclaration(export) => match &export.declaration {
            Declaration::ClassDeclaration(class) => parameters(class.type_parameters.as_deref()),
            Declaration::FunctionDeclaration(function) => {
                parameters(function.type_parameters.as_deref())
            }
            Declaration::TSInterfaceDeclaration(interface) => {
                parameters(interface.type_parameters.as_deref())
            }
            Declaration::TSTypeAliasDeclaration(alias) => {
                parameters(alias.type_parameters.as_deref())
            }
            _ => return None,
        },
        _ => return None,
    })
}

/// The single upper-case letters a block's tags of these names read as types, each with the line
/// of the first tag reading it, and the names those types declare.
fn used_letters(block: &Block, tags: &[&str]) -> (Vec<(String, usize)>, Vec<String>) {
    let mut used: Vec<(String, usize)> = Vec::new();
    let mut declared = Vec::new();
    for tag in block
        .tags
        .iter()
        .filter(|tag| tags.contains(&tag.tag.as_str()))
    {
        let names = text::type_names(&tag.r#type);
        declared.extend(names.declared);
        for name in names.read {
            let single = name.len() == 1 && name.chars().all(|each| each.is_ascii_uppercase());
            if single && !used.iter().any(|(each, _)| *each == name) {
                used.push((name, tag.lines[0]));
            }
        }
    }
    (used, declared)
}

impl RequireTemplate {
    /// The findings of one block, its owner's kind given.
    fn check(
        &self,
        source: &Source,
        doc: &Doc,
        owner: Option<(&AstKind, &oxc_semantic::AstNode)>,
        findings: &mut Vec<Finding>,
    ) {
        let avoided = match owner {
            Some((_, node)) => context::avoids(source, doc, node, false),
            None => {
                ["ignore", "override", "implements"]
                    .iter()
                    .any(|tag| context::has_tag(doc, tag))
                    || doc
                        .block
                        .tags
                        .iter()
                        .any(|tag| tag.tag == "inheritDoc" || tag.tag == "inheritdoc")
            }
        };
        if avoided {
            return;
        }
        let block = &doc.block;
        let mut templates: Vec<String> = block
            .tags
            .iter()
            .filter(|tag| tag.tag == "template")
            .flat_map(|tag| template_names(&tag.name))
            .collect();
        let has = |name: &str| block.tags.iter().filter(|tag| tag.tag == name).count();
        let letters = if has("callback") > 0 || has("function") > 0 {
            used_letters(block, &["param", "returns"])
        } else if has("typedef") == 1 {
            let (mut used, mut declared) = used_letters(block, &["typedef"]);
            let (more, more_declared) = used_letters(block, &["property"]);
            for (name, line) in more {
                if !used.iter().any(|(each, _)| *each == name) {
                    used.push((name, line));
                }
            }
            declared.append(&mut more_declared.clone());
            (used, declared)
        } else {
            let Some(names) = owner.and_then(|(kind, _)| declared(kind)) else {
                return;
            };
            for name in names {
                if !templates.contains(&name) {
                    findings.push(doc.finding(
                        source,
                        self.name(),
                        format!("Missing @template {name}"),
                    ));
                }
            }
            return;
        };
        let (used, declared) = letters;
        templates.extend(declared);
        for (name, line) in used {
            if !templates.contains(&name) {
                findings.push(doc.finding_at(
                    source,
                    self.name(),
                    line,
                    None,
                    format!("Missing @template {name}"),
                ));
            }
        }
    }
}

impl Rule for RequireTemplate {
    fn name(&self) -> &'static str {
        "jsdoc/require-template"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let owners = context::owners(source);
        let nodes = source.semantic.nodes();
        for doc in jsdoc::all(source) {
            let owner = owners
                .get(&doc.comment.span.start)
                .map(|id| nodes.get_node(*id));
            let kind = owner.map(oxc_semantic::AstNode::kind);
            self.check(source, &doc, kind.as_ref().zip(owner), findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireTemplate;
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&RequireTemplate, "require-template", "src/a.ts", &[]);
    }
}
