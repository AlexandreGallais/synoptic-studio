//! What `no-experimental` and `no-developer-preview` share (`@angular-eslint/eslint-plugin`
//! 22.6.0, `utils/jsdoc.ts`): a name read, outside an import or an export, whose declaration
//! carries a tag in its `JSDoc` block, a declaration of the file or one an Angular package exports
//! (`declarations`). The plugin follows any symbol TypeScript resolves, a member of a typed object
//! among them; the tree follows names only (`lint_rules`).

use oxc_ast::AstKind;
use oxc_semantic::{AstNode, NodeId};
use oxc_span::GetSpan;

use super::declarations::{entry_of, exported_tags};
use crate::expressions::symbol;
use crate::source::Source;

/// Whether a node sits in an import or an export, before any block, class, function, union or
/// declarator (`isInsideExportOrImport`).
fn in_import_or_export(source: &Source, id: NodeId) -> bool {
    for ancestor in source.semantic.nodes().ancestors(id) {
        match ancestor.kind() {
            AstKind::ExportNamedDeclaration(_)
            | AstKind::ExportFromDeclaration(_)
            | AstKind::ImportDeclaration(_) => return true,
            AstKind::BlockStatement(_)
            | AstKind::Class(_)
            | AstKind::TSInterfaceDeclaration(_)
            | AstKind::Function(_)
            | AstKind::Program(_)
            | AstKind::TSUnionType(_)
            | AstKind::VariableDeclarator(_) => return false,
            _ => {}
        }
    }
    false
}

/// Where the `JSDoc` block of a declaration node is attached: its statement, or the export
/// around it.
fn documented_at(source: &Source, id: NodeId) -> u32 {
    let nodes = source.semantic.nodes();
    let mut current = nodes.get_node(id);
    if matches!(current.kind(), AstKind::VariableDeclarator(_)) {
        current = nodes.parent_node(current.id());
    }
    let parent = nodes.parent_node(current.id());
    match parent.kind() {
        AstKind::ExportDeclaration(_) | AstKind::ExportDefaultDeclaration(_) => {
            parent.kind().span().start
        }
        _ => current.kind().span().start,
    }
}

/// Whether a name read at a node is declared with a tag: the name read and its offset.
#[must_use]
pub fn tagged_use<'a>(
    source: &Source<'_, 'a>,
    node: &AstNode<'a>,
    tag: &str,
) -> Option<(&'a str, u32)> {
    let AstKind::IdentifierReference(name) = node.kind() else {
        return None;
    };
    let symbol = symbol(source, name)?;
    if in_import_or_export(source, node.id()) {
        return None;
    }
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let tagged = match nodes.kind(declaration) {
        AstKind::ImportSpecifier(specifier) => {
            let from = nodes
                .ancestors(declaration)
                .find_map(|ancestor| match ancestor.kind() {
                    AstKind::ImportDeclaration(import) => Some(import.source.value.as_str()),
                    _ => None,
                })?;
            let entry = entry_of(&source.folder(), from)?;
            let tags = source.project.declarations(&entry, exported_tags);
            tags.get(specifier.imported.name().as_str())
                .is_some_and(|tags| tags.iter().any(|each| each == tag))
        }
        AstKind::VariableDeclarator(_)
        | AstKind::Function(_)
        | AstKind::Class(_)
        | AstKind::TSInterfaceDeclaration(_)
        | AstKind::TSTypeAliasDeclaration(_)
        | AstKind::TSEnumDeclaration(_) => source
            .jsdoc_before(documented_at(source, declaration))
            .is_some_and(|block| {
                super::declarations::block_tags(block)
                    .iter()
                    .any(|each| each == tag)
            }),
        _ => false,
    };
    tagged.then_some((name.name.as_str(), name.span.start))
}

/// A project whose playground holds an `@angular/core` of declarations, for the rules' tests:
/// `experimentalFunction` and `ExperimentalClass` tagged `@experimental`, `previewFunction` and
/// `PreviewClass` (`Preview$1` in its chunk) `@developerPreview`, `regularFunction` neither.
#[cfg(test)]
#[must_use]
pub fn angular_project() -> std::path::PathBuf {
    let root = crate::testing::fixture("angular-tags");
    let types = root.join("playground/node_modules/@angular/core/types");
    std::fs::create_dir_all(&types).unwrap_or_default();
    let entry = "export { previewFunction, PreviewClass } from './_chunk.js';\nexport { experimentalFunction, ExperimentalClass, regularFunction };\n/** @experimental 22.0 */\ndeclare function experimentalFunction(): void;\n/** @experimental 22.0 */\ndeclare class ExperimentalClass {}\n/** @publicApi */\ndeclare function regularFunction(): void;\n";
    let chunk = "/** @developerPreview 20.0 */\ndeclare function previewFunction(): void;\n/** @developerPreview 20.0 */\ndeclare class Preview$1 {}\nexport { previewFunction, Preview$1 as PreviewClass };\n";
    std::fs::write(types.join("core.d.ts"), entry).unwrap_or_default();
    std::fs::write(types.join("_chunk.d.ts"), chunk).unwrap_or_default();
    root
}

/// The plugin's cases of a name declared in the file with a tag, the same for both rules: the
/// code, then the line, the column and the name of the one finding.
#[cfg(test)]
#[must_use]
pub fn invalid_cases(tag: &str) -> Vec<(String, usize, usize, &'static str)> {
    [
        // The declaration's block on line 1, the use on line 3 at the column given.
        ("interface Test {};\nconst test: Test = {};", 13, "Test"),
        ("interface Test {};\nPartial<Test>;", 9, "Test"),
        ("class Test {}\nconst test: Test = {};", 13, "Test"),
        ("class Test {}\nconst test = new Test();", 18, "Test"),
        ("const Test = class {};\nconst test = new Test();", 18, "Test"),
        ("type Test = 'a' | 'b';\nconst test: Test = 'a';", 13, "Test"),
        ("function test() {}\ntest();", 1, "test"),
        ("const test = () => {}\ntest();", 1, "test"),
        ("function test(param = '') {}\ntest();", 1, "test"),
        ("let test: number = 1;\nconst result = test++;", 16, "test"),
        ("const test = 'test';\nconst myString = test + '-suffix';", 18, "test"),
        ("const test = [];\nfor (const value of test) {}", 21, "test"),
        ("const test = true;\nconst value = test ? 'yes' : 'no';", 15, "test"),
        ("const test = 'test';\nconst another = test;", 17, "test"),
        ("const test = 'test';\nconsole.log(test);", 13, "test"),
        ("const test = { a: { b: {} } };\ntest.a.b;", 1, "test"),
        ("const test = {};\nconst result = { ...test };", 21, "test"),
        ("const test = [];\nconst result = [ ...test ];", 21, "test"),
        ("const x = 1;\nconst { y = x } = {};", 13, "x"),
        ("const test = 'test';\nconst myString = `${test}-suffix`;", 21, "test"),
        ("enum Test {\n  member = 1,\n}\nTest.member;", 1, "Test"),
        (
            "function $localize2(value: TemplateStringsArray) {\n  return value;\n}\nconst result = $localize2`Hello World!`;",
            16,
            "$localize2",
        ),
        // Exported, the block before `export`.
        ("export function test() {}\ntest();", 1, "test"),
    ]
    .into_iter()
    .map(|(code, column, name)| {
        let code = format!("/** @{tag} */\n{code}\n");
        let line = code.lines().count();
        (code, line, column, name)
    })
    .collect()
}

/// The plugin's valid cases with a tag, and the cases the tree leaves out: a member of a typed
/// object, read through its type only TypeScript knows.
#[cfg(test)]
#[must_use]
pub fn valid_cases(tag: &str) -> Vec<String> {
    [
        "/** Not tagged */\ninterface Test {\n  /** @publicApi */\n  a1: string;\n  /** @TAG */\n  a2: string;\n};\nconst test: Test = {};\ntest.a1 = 'value';",
        "/** Not tagged */\nclass Test {\n  /** @deprecated */\n  a1: string;\n  /** @TAG */\n  a2: string;\n}\nconst test = new Test();\ntest.a1 = 'value';",
        "/** Not tagged */\nconst test = {};\nif (test) {}",
        "/** Not tagged */\nfunction test() {}\ntest();",
        "/** @TAG */\ndeclare module \"some-module\" {}",
        "/** @TAG */\ndeclare function test(): void;",
        "/** @TAG */\ndeclare const test: () => void;",
        "import { something } from './non-existing';\nconst myVar = something;",
        // Left out: members.
        "interface Test {\n  good?: () => void;\n  /** @TAG */\n  bad?: () => void;\n};\nconst test: Test = {};\ntest.bad();",
        "class Test {\n  /** @TAG */\n  func() {}\n}\nconst test = new Test();\ntest.func();",
        "enum Test {\n  member1 = 1,\n  /** @TAG */\n  member2 = 2,\n}\nTest.member2;",
        "class Test {\n  good?: string;\n  /** @TAG */\n  bad?: string;\n}\nconst { good, bad } = new Test();",
    ]
    .iter()
    .map(|code| code.replace("TAG", tag))
    .collect()
}
