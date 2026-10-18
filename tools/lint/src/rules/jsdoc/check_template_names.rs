//! `jsdoc/check-template-names`: each name of a `@template` tag is a type parameter of the class,
//! interface, type alias or function declaration it documents, or a type its tags read
//! (`eslint-plugin-jsdoc` 65.3.1, `src/rules/checkTemplateNames.js`).

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Declaration, ExportDefaultDeclarationKind, FunctionType, TSTypeParameterDeclaration,
};
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::jsdoc::block::{Block, trim};
use crate::jsdoc::{self, Doc, context, text};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct CheckTemplateNames;

/// The names of a `@template` tag: split at its top-level commas, a default left out
/// (`parseClosureTemplateTag`).
#[must_use]
pub fn template_names(name: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth: usize = 0;
    let mut current = String::new();
    for character in name.chars() {
        if matches!(character, '<' | '{' | '(') {
            depth += 1;
        } else if matches!(character, '>' | '}' | ')') {
            depth = depth.saturating_sub(1);
        }
        if character == ',' && depth == 0 {
            parts.push(std::mem::take(&mut current));
        } else {
            current.push(character);
        }
    }
    parts.push(current);
    parts
        .iter()
        .map(|part| {
            let part = trim(part);
            match part.find('=') {
                Some(equal) => part[..equal]
                    .strip_prefix('[')
                    .unwrap_or(&part[..equal])
                    .to_owned(),
                None => part.to_owned(),
            }
        })
        .collect()
}

/// The names of the types of a block's tags of these names.
fn tag_type_names(block: &Block, tags: &[&str]) -> Vec<String> {
    block
        .tags
        .iter()
        .filter(|tag| tags.contains(&tag.tag.as_str()))
        .flat_map(|tag| text::type_names(&tag.r#type).read)
        .collect()
}

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

/// What a documented declaration gives: its type parameters, and whether its block's
/// parameter and return types count; a class's members' blocks counted too.
fn declared_names(source: &Source, kind: &AstKind, block: &Block) -> Option<Vec<String>> {
    let mut names = Vec::new();
    let class = |class: &oxc_ast::ast::Class, names: &mut Vec<String>| {
        names.extend(parameters(class.type_parameters.as_deref()));
        for element in &class.body.body {
            if let Some(comment) = jsdoc::comment_before(source, element.span().start) {
                let inner = jsdoc::block::parse(jsdoc::comment_text(source, &comment));
                names.extend(tag_type_names(&inner, &["param", "returns", "type"]));
            }
        }
    };
    match kind {
        AstKind::Class(declared) if declared.is_declaration() => class(declared, &mut names),
        AstKind::TSInterfaceDeclaration(interface) => {
            names.extend(parameters(interface.type_parameters.as_deref()));
        }
        AstKind::TSTypeAliasDeclaration(alias) => {
            names.extend(parameters(alias.type_parameters.as_deref()));
        }
        AstKind::Function(function) if function.r#type == FunctionType::FunctionDeclaration => {
            names.extend(parameters(function.type_parameters.as_deref()));
            names.extend(tag_type_names(block, &["param", "returns"]));
        }
        AstKind::ExportDeclaration(export) => match &export.declaration {
            Declaration::ClassDeclaration(declared) => class(declared, &mut names),
            Declaration::TSInterfaceDeclaration(interface) => {
                names.extend(parameters(interface.type_parameters.as_deref()));
            }
            Declaration::TSTypeAliasDeclaration(alias) => {
                names.extend(parameters(alias.type_parameters.as_deref()));
            }
            Declaration::FunctionDeclaration(function) => {
                names.extend(parameters(function.type_parameters.as_deref()));
                names.extend(tag_type_names(block, &["param", "returns"]));
            }
            _ => return None,
        },
        AstKind::ExportDefaultDeclaration(export) => match &export.declaration {
            ExportDefaultDeclarationKind::ClassDeclaration(declared) => class(declared, &mut names),
            ExportDefaultDeclarationKind::FunctionDeclaration(function) => {
                names.extend(parameters(function.type_parameters.as_deref()));
                names.extend(tag_type_names(block, &["param", "returns"]));
            }
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(interface) => {
                names.extend(parameters(interface.type_parameters.as_deref()));
            }
            _ => return None,
        },
        _ => return None,
    }
    names.extend(tag_type_names(
        block,
        &["augments", "extends", "implements"],
    ));
    Some(names)
}

impl CheckTemplateNames {
    /// The findings of one block, its owner's kind given.
    fn check(
        &self,
        source: &Source,
        doc: &Doc,
        owner: Option<&AstKind>,
        findings: &mut Vec<Finding>,
    ) {
        let block = &doc.block;
        let has = |name: &str| block.tags.iter().filter(|tag| tag.tag == name).count();
        let used: Vec<String> = if has("callback") > 0 || has("function") > 0 {
            tag_type_names(block, &["param", "returns"])
        } else if has("typedef") == 1 {
            tag_type_names(block, &["typedef", "property"])
        } else {
            match owner.and_then(|kind| declared_names(source, kind, block)) {
                Some(names) => names,
                None => return,
            }
        };
        for tag in block.tags.iter().filter(|tag| tag.tag == "template") {
            for name in template_names(&tag.name) {
                if !used.contains(&name) {
                    findings.push(doc.finding_at(
                        source,
                        self.name(),
                        tag.lines[0],
                        None,
                        format!("@template {name} not in use"),
                    ));
                }
            }
        }
    }
}

impl Rule for CheckTemplateNames {
    fn name(&self) -> &'static str {
        "jsdoc/check-template-names"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let owners = context::owners(source);
        for doc in jsdoc::all(source) {
            let owner = owners
                .get(&doc.comment.span.start)
                .map(|id| source.semantic.nodes().kind(*id));
            self.check(source, &doc, owner.as_ref(), findings);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckTemplateNames, template_names};
    use crate::jsdoc::cases::check;

    #[test]
    fn reports_the_documented_cases_as_eslint_does() {
        check(&CheckTemplateNames, "check-template-names", "src/a.ts");
    }

    #[test]
    fn splits_template_names_at_their_top_level_commas() {
        assert_eq!(template_names("T, U"), ["T", "U"]);
        assert_eq!(template_names("[T=string], Map<K, V>"), ["T", "Map<K, V>"]);
    }
}
