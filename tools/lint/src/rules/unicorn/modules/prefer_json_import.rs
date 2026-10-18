//! `unicorn/prefer-json-import`: a top-level variable set to `JSON.parse()` of a static `.json`
//! file read whole with `fs` — `readFileSync(…)`, or `await readFile(…)` of `fs/promises` or
//! `fs.promises` — in UTF-8, where a JSON import would do (`eslint-plugin-unicorn` 78.0.0,
//! `docs/rules/prefer-json-import.md`). The path is a string known statically or
//! `new URL(path, import.meta.url)`. Reported only: the plugin offers a suggestion, never a fix.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Expression, ImportDeclarationSpecifier, ImportOrExportKind, ModuleExportName,
    ObjectPropertyKind, PropertyKey, PropertyKind,
};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{Value, static_value, symbol_of, unwrap};
use crate::source::Source;

/// The rule.
pub struct PreferJsonImport;

/// The rule's message.
const MESSAGE: &str = "Prefer a JSON import over reading and parsing a JSON file.";

/// Whether a call reads a file whole with `fs`: `readFileSync`, or when awaited `readFile` of
/// `fs/promises` or `fs.promises.readFile`, imported by value.
fn is_file_read(source: &Source, expression: &Expression, awaited: bool) -> bool {
    let Expression::CallExpression(call) = expression else {
        return false;
    };
    if call.optional || call.arguments.is_empty() || call.arguments.len() > 2 {
        return false;
    }
    let mut properties: Vec<&str> = Vec::new();
    let mut callee = &call.callee;
    while let Expression::StaticMemberExpression(member) = callee {
        if member.optional {
            return false;
        }
        properties.insert(0, member.property.name.as_str());
        callee = &member.object;
    }
    let Expression::Identifier(name) = callee else {
        return false;
    };
    let Some(symbol) = symbol_of(source, name) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let AstKind::ImportDeclaration(import) = nodes.parent_kind(declaration) else {
        return false;
    };
    if import.import_kind == ImportOrExportKind::Type {
        return false;
    }
    let Some(specifier) = import.specifiers.iter().flatten().find(|each| match each {
        ImportDeclarationSpecifier::ImportSpecifier(specifier) => {
            specifier.local.symbol_id.get() == Some(symbol)
        }
        ImportDeclarationSpecifier::ImportDefaultSpecifier(specifier) => {
            specifier.local.symbol_id.get() == Some(symbol)
        }
        ImportDeclarationSpecifier::ImportNamespaceSpecifier(specifier) => {
            specifier.local.symbol_id.get() == Some(symbol)
        }
    }) else {
        return false;
    };
    if let ImportDeclarationSpecifier::ImportSpecifier(specifier) = specifier {
        if specifier.import_kind == ImportOrExportKind::Type {
            return false;
        }
        let imported = match &specifier.imported {
            ModuleExportName::IdentifierName(name) => name.name.as_str(),
            ModuleExportName::StringLiteral(name) => name.value.as_str(),
            ModuleExportName::IdentifierReference(name) => name.name.as_str(),
        };
        if imported != "default" {
            properties.insert(0, imported);
        }
    }
    let module = import.source.value.as_str();
    let module = module.strip_prefix("node:").unwrap_or(module);
    let method = properties.join(".");
    if awaited {
        (module == "fs/promises" && method == "readFile")
            || (module == "fs" && method == "promises.readFile")
    } else {
        module == "fs" && method == "readFileSync"
    }
}

/// Whether a read's encoding is UTF-8 or left out: no argument, a string, `null`, `undefined`,
/// or an object holding only `encoding`.
fn is_utf8(source: &Source, encoding: Option<&Expression>) -> bool {
    let Some(encoding) = encoding else {
        return true;
    };
    let value = match unwrap(encoding) {
        Expression::ObjectExpression(object) => {
            let [ObjectPropertyKind::ObjectProperty(property)] = object.properties.as_slice()
            else {
                return false;
            };
            let named = matches!(&property.key, PropertyKey::StaticIdentifier(key) if key.name == "encoding")
                || matches!(&property.key, PropertyKey::StringLiteral(key) if key.value == "encoding");
            if property.computed || property.method || property.kind != PropertyKind::Init || !named
            {
                return false;
            }
            &property.value
        }
        other => other,
    };
    match static_value(source, value) {
        Some(Value::Null | Value::Undefined) => true,
        Some(Value::Text(text)) => {
            let lower = text.to_ascii_lowercase();
            lower == "utf8" || lower == "utf-8"
        }
        _ => false,
    }
}

/// Whether a path is a `.json` file known statically, or one relative to the module.
fn is_json_path(source: &Source, path: &Expression) -> bool {
    let path = match unwrap(path) {
        Expression::NewExpression(url)
            if url.callee.is_specific_id("URL") && url.arguments.len() == 2 =>
        {
            let base = url.arguments[1].as_expression();
            let from_module = matches!(base, Some(Expression::StaticMemberExpression(member))
                if !member.optional && member.property.name == "url"
                    && matches!(&member.object, Expression::ImportMeta(_)));
            match (from_module, url.arguments[0].as_expression()) {
                (true, Some(path)) => path,
                _ => return false,
            }
        }
        other => other,
    };
    matches!(static_value(source, path), Some(Value::Text(text)) if text.rsplit_once('.').is_some_and(|(_, extension)| extension == "json"))
}

impl Rule for PreferJsonImport {
    fn name(&self) -> &'static str {
        "unicorn/prefer-json-import"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::VariableDeclarator(declarator) = node.kind() else {
            return;
        };
        let nodes = source.semantic.nodes();
        let declaration = nodes.parent_id(node.id());
        let top_level = match nodes.parent_kind(declaration) {
            AstKind::Program(_) => true,
            AstKind::ExportDeclaration(_) => {
                matches!(
                    nodes.parent_kind(nodes.parent_id(declaration)),
                    AstKind::Program(_)
                )
            }
            _ => false,
        };
        let Some(init) = &declarator.init else {
            return;
        };
        let Expression::CallExpression(parse) = unwrap(init) else {
            return;
        };
        let is_parse = matches!(&parse.callee, Expression::StaticMemberExpression(member)
            if !member.optional && member.property.name == "parse" && member.object.is_specific_id("JSON"));
        if !top_level || !is_parse || parse.optional || parse.arguments.len() != 1 {
            return;
        }
        let Some(argument) = parse.arguments[0].as_expression() else {
            return;
        };
        let (read, awaited) = match unwrap(argument) {
            Expression::AwaitExpression(wait) => (unwrap(&wait.argument), true),
            other => (other, false),
        };
        let Expression::CallExpression(call) = read else {
            return;
        };
        let encoding = call.arguments.get(1).and_then(|each| each.as_expression());
        let path = call.arguments.first().and_then(|each| each.as_expression());
        if is_file_read(source, read, awaited)
            && is_utf8(source, encoding)
            && path.is_some_and(|path| is_json_path(source, path))
        {
            findings.push(source.finding(self.name(), parse.span().start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferJsonImport;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferJsonImport, "tests/a.test.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_static_json_file_read_at_the_top() {
        // The documentation's two cases, then the other imports it names.
        assert_eq!(
            places(
                "import fs from 'node:fs';\nconst data = JSON.parse(fs.readFileSync('./data.json', 'utf8'));"
            ),
            [(2, 14)]
        );
        assert_eq!(
            places(
                "import fs from 'node:fs';\nconst settings = JSON.parse(await fs.promises.readFile(new URL('./settings.json', import.meta.url), 'utf8'));"
            ),
            [(2, 18)]
        );
        assert_eq!(
            places(
                "import { readFileSync } from 'fs';\nexport const a = JSON.parse(readFileSync('a.json'));"
            ),
            [(2, 18)]
        );
        assert_eq!(
            places(
                "import { readFile as read } from 'node:fs/promises';\nconst a = JSON.parse(await read('a.json', { encoding: 'UTF-8' }));"
            ),
            [(2, 11)]
        );
        assert_eq!(
            places(
                "import * as fs from 'node:fs';\nconst path = './a.json';\nconst a = JSON.parse(fs.readFileSync(path, null)) as A;"
            ),
            [(3, 11)]
        );
    }

    #[test]
    fn leaves_dynamic_paths_nested_reads_and_other_encodings() {
        for code in [
            "import data from './data.json' with {type: 'json'};",
            "import fs from 'node:fs';\nconst dynamicData = JSON.parse(fs.readFileSync(filename, 'utf8'));",
            "import fs from 'node:fs';\nfunction loadFreshData() {\n\treturn JSON.parse(fs.readFileSync('./data.json', 'utf8'));\n}",
            "import fs from 'node:fs';\nconst a = JSON.parse(fs.readFileSync('./a.json', 'latin1'));",
            "import fs from 'node:fs';\nconst a = JSON.parse(fs.readFileSync('./a.json', { encoding: 'utf8', flag: 'r' }));",
            "import fs from 'node:fs';\nconst a = JSON.parse(fs.readFileSync('./a.txt', 'utf8'));",
            "import fs from 'node:fs';\nconst a = JSON.parse(fs.readFileSync('./a.json'), reviver);",
            "import { readFile } from 'node:fs';\nconst a = JSON.parse(readFile('./a.json'));",
            "import { readFileSync } from 'node:fs';\nconst a = JSON.parse(await readFileSync('./a.json'));",
            "const fs = f();\nconst a = JSON.parse(fs.readFileSync('./a.json'));",
            "import fs from 'node:fs';\nconst a = JSON.parse(fs.readFileSync(new URL('./a.json', base)));",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
