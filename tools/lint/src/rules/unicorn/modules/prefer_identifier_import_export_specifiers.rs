//! `unicorn/prefer-identifier-import-export-specifiers`: a string written where an identifier
//! would do, in an import or export specifier, an `export * as` or an import attribute's key
//! (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-identifier-import-export-specifiers.md`). Autofixed, a space added where the
//! name would touch a word.

use oxc_ast::AstKind;
use oxc_ast::ast::{ImportAttributeKey, ModuleExportName, StringLiteral};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::globals::is_identifier_name;
use crate::expressions::text;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferIdentifierImportExportSpecifiers;

/// Whether a character belongs to a word, so that a name written beside it needs a space.
fn is_word(character: Option<char>) -> bool {
    character.is_some_and(|each| each == '$' || each == '_' || each.is_alphanumeric())
}

/// The finding on a string that could be an identifier.
fn finding(rule: &'static str, source: &Source, literal: &StringLiteral) -> Option<Finding> {
    let identifier = literal.value.as_str();
    if !is_identifier_name(identifier) {
        return None;
    }
    let span = literal.span;
    let before = source
        .text
        .get(..span.start as usize)
        .and_then(|text| text.chars().next_back());
    let after = source
        .text
        .get(span.end as usize..)
        .and_then(|text| text.chars().next());
    let replacement = format!(
        "{}{identifier}{}",
        if is_word(before) { " " } else { "" },
        if is_word(after) { " " } else { "" }
    );
    let message = format!(
        "Prefer identifier `{identifier}` over string literal `{}`.",
        text(source, span)
    );
    let fix = Fix {
        start: span.start,
        end: span.end,
        text: replacement,
    };
    Some(source.fixed(rule, span.start, message, fix))
}

/// The string a module export name is written as, when it is one.
fn literal<'s, 'a>(name: &'s ModuleExportName<'a>) -> Option<&'s StringLiteral<'a>> {
    match name {
        ModuleExportName::StringLiteral(literal) => Some(literal),
        _ => None,
    }
}

impl Rule for PreferIdentifierImportExportSpecifiers {
    fn name(&self) -> &'static str {
        "unicorn/prefer-identifier-import-export-specifiers"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let strings: Vec<&StringLiteral> = match node.kind() {
            AstKind::ImportSpecifier(specifier) => {
                literal(&specifier.imported).into_iter().collect()
            }
            AstKind::ExportSpecifier(specifier) => {
                let from_module = matches!(
                    source.semantic.nodes().parent_kind(node.id()),
                    AstKind::ExportFromDeclaration(_)
                );
                let renamed = specifier.local.span() != specifier.exported.span();
                let mut strings: Vec<&StringLiteral> =
                    literal(&specifier.exported).into_iter().collect();
                if from_module && renamed {
                    strings.extend(literal(&specifier.local));
                }
                strings
            }
            AstKind::ExportAllDeclaration(export) => export
                .exported
                .as_ref()
                .and_then(literal)
                .into_iter()
                .collect(),
            AstKind::ImportAttribute(attribute) => match &attribute.key {
                ImportAttributeKey::StringLiteral(key) => vec![key],
                ImportAttributeKey::Identifier(_) => Vec::new(),
            },
            _ => return,
        };
        findings.extend(
            strings
                .into_iter()
                .filter_map(|each| finding(self.name(), source, each)),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::PreferIdentifierImportExportSpecifiers;
    use crate::testing::{fixed, lint};

    /// The places of the findings, and the code fixed.
    fn check(code: &str) -> (Vec<(usize, usize)>, String) {
        let findings = lint(&PreferIdentifierImportExportSpecifiers, "src/a/b.ts", code);
        let places = findings
            .iter()
            .map(|each| (each.line, each.column))
            .collect();
        (places, fixed(code, &findings))
    }

    #[test]
    fn writes_identifiers_for_strings_that_are_names() {
        // The documentation's four cases, then `export * as` and a name touching `as`.
        assert_eq!(
            check("import {'foo' as foo} from 'foo';"),
            (vec![(1, 9)], "import {foo as foo} from 'foo';".to_owned())
        );
        assert_eq!(check("export {foo as 'bar'};").1, "export {foo as bar};");
        assert_eq!(
            check("export {'foo' as bar} from 'foo';").1,
            "export {foo as bar} from 'foo';"
        );
        assert_eq!(
            check("import foo from 'foo' with {'type': 'json'};").1,
            "import foo from 'foo' with {type: 'json'};"
        );
        assert_eq!(
            check("export * as 'a' from 'foo';").1,
            "export * as a from 'foo';"
        );
        assert_eq!(
            check("export {'a'as b} from 'foo';").1,
            "export {a as b} from 'foo';"
        );
        assert_eq!(
            check("const foo = 1;\nexport {foo as'bar'};").1,
            "const foo = 1;\nexport {foo as bar};"
        );
        assert_eq!(
            check("export {'a'} from 'foo';"),
            (vec![(1, 9)], "export {a} from 'foo';".to_owned())
        );
    }

    #[test]
    fn leaves_strings_that_are_not_names() {
        for code in [
            "import {'a string' as aString} from 'foo';",
            "export {foo as 'a-b'};",
            "import {foo} from 'foo';",
            "import foo from 'foo' with {type: 'json'};",
            "export * as 'a b' from 'foo';",
        ] {
            assert_eq!(check(code).0, Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
