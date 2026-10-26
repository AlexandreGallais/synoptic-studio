//! The `JSDoc` tags of what an Angular package exports, read from its declarations where
//! `@angular-eslint/eslint-plugin` 22.6.0 asks TypeScript's checker for a symbol's tags
//! (`utils/jsdoc.ts`, `hasJsDocTag`): the entry `types/<name>.d.ts` of the installed package, each
//! exported name followed through its `export { … } from` and `import { … } from` to the
//! declaration of a chunk, and the tags of the `JSDoc` block before each of its declarations, an
//! overload's included. A member of a class or an interface is not followed (`lint_rules`).

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use oxc_allocator::Allocator;
use oxc_ast::ast::{Declaration, ImportDeclarationSpecifier, Program, Statement};
use oxc_parser::Parser;
use oxc_span::SourceType;

/// Each exported name of a package's entry and the tags of its declarations.
pub type Tags = HashMap<String, Vec<String>>;

/// How far exports are followed from file to file.
const DEPTH: usize = 8;

/// Where an exported name leads.
enum Target {
    /// A declaration of the file, by its name there.
    Local(String),
    /// An export of another file, by its name there.
    Remote(PathBuf, String),
}

/// What a declaration file tells.
#[derive(Default)]
struct Module {
    /// The tags of each name declared, all its declarations' together.
    tagged: HashMap<String, Vec<String>>,
    /// Where each exported name leads.
    exports: HashMap<String, Target>,
    /// The files whose exports this one passes on whole, `export * from`.
    stars: Vec<PathBuf>,
}

/// The tags a `JSDoc` block holds, `experimental` in `@experimental 22.0`.
#[must_use]
pub fn block_tags(block: &str) -> Vec<String> {
    block
        .split('@')
        .skip(1)
        .map(|rest| {
            rest.chars()
                .take_while(char::is_ascii_alphanumeric)
                .collect::<String>()
        })
        .filter(|tag| !tag.is_empty())
        .collect()
}

/// The declaration file a relative specifier of a declaration file names: `./a.js` is `./a.d.ts`.
fn target(folder: &Path, specifier: &str) -> Option<PathBuf> {
    let relative = specifier.strip_prefix("./")?;
    let stem = relative
        .strip_suffix(".js")
        .or_else(|| relative.strip_suffix(".d.ts"))
        .unwrap_or(relative);
    Some(folder.join(format!("{stem}.d.ts")))
}

/// The names a declaration binds.
fn declared_names<'a>(declaration: &Declaration<'a>) -> Vec<&'a str> {
    match declaration {
        Declaration::VariableDeclaration(variables) => variables
            .declarations
            .iter()
            .filter_map(|declarator| declarator.id.get_binding_identifier())
            .map(|name| name.name.as_str())
            .collect(),
        Declaration::FunctionDeclaration(function) => {
            function.id.iter().map(|name| name.name.as_str()).collect()
        }
        Declaration::ClassDeclaration(class) => {
            class.id.iter().map(|name| name.name.as_str()).collect()
        }
        Declaration::TSTypeAliasDeclaration(alias) => vec![alias.id.name.as_str()],
        Declaration::TSInterfaceDeclaration(interface) => vec![interface.id.name.as_str()],
        Declaration::TSEnumDeclaration(declaration) => vec![declaration.id.name.as_str()],
        _ => Vec::new(),
    }
}

/// The tags of the `JSDoc` block attached to the statement starting at an offset.
fn tags_before(program: &Program, text: &str, offset: u32) -> Vec<String> {
    program
        .comments
        .iter()
        .rfind(|comment| comment.attached_to == offset)
        .filter(|comment| comment.is_block())
        .and_then(|comment| text.get(comment.span.start as usize..comment.span.end as usize))
        .filter(|block| block.starts_with("/**"))
        .map(block_tags)
        .unwrap_or_default()
}

/// Reads a declaration file.
fn read_module(path: &Path) -> Module {
    let mut module = Module::default();
    let Ok(text) = fs::read_to_string(path) else {
        return module;
    };
    let folder = path.parent().unwrap_or(path);
    let allocator = Allocator::default();
    let program = Parser::new(&allocator, &text, SourceType::d_ts())
        .parse()
        .program;
    let mut imports: HashMap<String, (PathBuf, String)> = HashMap::new();

    for statement in &program.body {
        let tags = tags_before(&program, &text, statement_start(statement));
        let declare = |names: Vec<&str>, module: &mut Module| {
            for name in names {
                module
                    .tagged
                    .entry(name.to_owned())
                    .or_default()
                    .extend(tags.iter().cloned());
            }
        };
        match statement {
            Statement::ImportDeclaration(import) => {
                let Some(file) = target(folder, &import.source.value) else {
                    continue;
                };
                for specifier in import.specifiers.iter().flatten() {
                    if let ImportDeclarationSpecifier::ImportSpecifier(specifier) = specifier {
                        imports.insert(
                            specifier.local.name.to_string(),
                            (file.clone(), specifier.imported.name().to_string()),
                        );
                    }
                }
            }
            Statement::ExportDeclaration(export) => {
                let names = declared_names(&export.declaration);
                for name in &names {
                    module
                        .exports
                        .insert((*name).to_owned(), Target::Local((*name).to_owned()));
                }
                declare(names, &mut module);
            }
            Statement::ExportNamedDeclaration(export) => {
                for specifier in &export.specifiers {
                    module.exports.insert(
                        specifier.exported.name().to_string(),
                        Target::Local(specifier.local.name().to_string()),
                    );
                }
            }
            Statement::ExportFromDeclaration(export) => {
                let Some(file) = target(folder, &export.source.value) else {
                    continue;
                };
                for specifier in &export.specifiers {
                    module.exports.insert(
                        specifier.exported.name().to_string(),
                        Target::Remote(file.clone(), specifier.local.name().to_string()),
                    );
                }
            }
            Statement::ExportAllDeclaration(export) if export.exported.is_none() => {
                module.stars.extend(target(folder, &export.source.value));
            }
            other => {
                if let Some(declaration) = other.as_declaration() {
                    declare(declared_names(declaration), &mut module);
                }
            }
        }
    }
    // A local export of an imported name leads to the file it comes from.
    for leads in module.exports.values_mut() {
        if let Target::Local(name) = leads
            && !module.tagged.contains_key(name.as_str())
            && let Some((file, imported)) = imports.get(name.as_str())
        {
            *leads = Target::Remote(file.clone(), imported.clone());
        }
    }
    module
}

/// Where a statement starts, the node its block is attached to.
fn statement_start(statement: &Statement) -> u32 {
    oxc_span::GetSpan::span(statement).start
}

/// The modules read, by path.
struct Reader {
    /// Each module read once.
    modules: HashMap<PathBuf, Module>,
}

impl Reader {
    /// A module, read once.
    fn module(&mut self, path: &Path) -> &Module {
        self.modules
            .entry(path.to_path_buf())
            .or_insert_with(|| read_module(path))
    }

    /// The tags of an exported name of a file, followed at most `depth` files.
    fn tags(
        &mut self,
        path: &Path,
        name: &str,
        depth: usize,
        seen: &mut HashSet<PathBuf>,
    ) -> Vec<String> {
        if depth == 0 {
            return Vec::new();
        }
        let module = self.module(path);
        let (leads, stars) = match module.exports.get(name) {
            Some(Target::Local(local)) => {
                return module.tagged.get(local).cloned().unwrap_or_default();
            }
            Some(Target::Remote(file, remote)) => {
                (Some((file.clone(), remote.clone())), Vec::new())
            }
            None => (None, module.stars.clone()),
        };
        if let Some((file, remote)) = leads {
            return self.tags(&file, &remote, depth - 1, seen);
        }
        for star in stars {
            if seen.insert(star.clone()) {
                let found = self.tags(&star, name, depth - 1, seen);
                if !found.is_empty() {
                    return found;
                }
            }
        }
        Vec::new()
    }

    /// The names a file exports, those of the files it passes on whole included.
    fn names(&mut self, path: &Path, depth: usize, seen: &mut HashSet<PathBuf>) -> Vec<String> {
        if depth == 0 || !seen.insert(path.to_path_buf()) {
            return Vec::new();
        }
        let module = self.module(path);
        let mut names: Vec<String> = module.exports.keys().cloned().collect();
        for star in module.stars.clone() {
            names.extend(self.names(&star, depth - 1, seen));
        }
        names
    }
}

/// The tags of each name a package's entry exports, those without tags left out.
#[must_use]
pub fn exported_tags(entry: &Path) -> Tags {
    let mut reader = Reader {
        modules: HashMap::new(),
    };
    let names = reader.names(entry, DEPTH, &mut HashSet::new());
    names
        .into_iter()
        .filter_map(|name| {
            let tags = reader.tags(entry, &name, DEPTH, &mut HashSet::new());
            (!tags.is_empty()).then_some((name, tags))
        })
        .collect()
}

/// The entry of an Angular module a specifier names, from a folder: `@angular/core` is
/// `node_modules/@angular/core/types/core.d.ts` in the nearest folder holding the package,
/// `@angular/core/rxjs-interop` its `types/rxjs-interop.d.ts`.
#[must_use]
pub fn entry_of(folder: &Path, specifier: &str) -> Option<PathBuf> {
    let rest = specifier.strip_prefix("@angular/")?;
    let (package, subpath) = rest.split_once('/').unwrap_or((rest, ""));
    let file = if subpath.is_empty() {
        package.to_owned()
    } else {
        subpath.replace('/', "-")
    };
    folder.ancestors().find_map(|ancestor| {
        let candidate = ancestor
            .join("node_modules/@angular")
            .join(package)
            .join("types")
            .join(format!("{file}.d.ts"));
        candidate.is_file().then_some(candidate)
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{block_tags, entry_of, exported_tags};
    use crate::testing::fixture;

    #[test]
    fn f51_ac1_reads_the_tags_of_a_block() {
        assert_eq!(
            block_tags("/**\n * Runs.\n *\n * @experimental 22.0\n * @see x\n */"),
            ["experimental", "see"]
        );
        assert_eq!(block_tags("/** No tag. */"), Vec::<String>::new());
    }

    #[test]
    fn f51_ac1_follows_each_export_to_its_tagged_declaration() {
        let root = fixture("angular-declarations");
        let types = root.join("node_modules/@angular/core/types");
        fs::create_dir_all(&types).unwrap_or_default();
        let entry = "import { Imported } from './_chunk.js';\nexport { previewInChunk, Plain } from './_chunk.js';\nexport * from './_star.js';\nexport { experimentalLocal, regular, Renamed$1 as Renamed, Imported };\n/** @experimental 22.0 */\ndeclare function experimentalLocal(): void;\n/** One overload. */\ndeclare function regular(): void;\ndeclare function regular(a: number): void;\n/** @developerPreview 20.0 */\ndeclare class Renamed$1 {}\n/** @experimental */\nexport declare const direct: number;\n";
        let chunk = "/** @developerPreview 19.0 */\ndeclare const previewInChunk: number;\ndeclare const Plain: number;\n/** @experimental */\ninterface Imported {}\nexport { previewInChunk, Plain, Imported };\n";
        let star = "/** @experimental */\nexport declare function starred(): void;\n";
        fs::write(types.join("core.d.ts"), entry).unwrap_or_default();
        fs::write(types.join("_chunk.d.ts"), chunk).unwrap_or_default();
        fs::write(types.join("_star.d.ts"), star).unwrap_or_default();

        let found = entry_of(&root.join("src/app"), "@angular/core");
        let tags = found.as_deref().map(exported_tags).unwrap_or_default();
        let mut names: Vec<(&str, &str)> = tags
            .iter()
            .flat_map(|(name, tags)| tags.iter().map(move |tag| (name.as_str(), tag.as_str())))
            .collect();
        names.sort_unstable();

        // `regular`'s and `Plain`'s blocks hold no tag; `starred` comes through `export *`.
        assert_eq!(
            names,
            [
                ("Imported", "experimental"),
                ("Renamed", "developerPreview"),
                ("direct", "experimental"),
                ("experimentalLocal", "experimental"),
                ("previewInChunk", "developerPreview"),
                ("starred", "experimental"),
            ]
        );
        assert_eq!(
            entry_of(&root.join("src"), "@angular/core/rxjs-interop"),
            None
        );
        assert_eq!(entry_of(&root, "rxjs"), None);
    }
}
