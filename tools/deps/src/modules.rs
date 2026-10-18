//! What one TypeScript module imports and exports, read from Oxc's module record (`REF-OXC-CRATE`,
//! `oxc_parser`): its imports and their names, its own exports, its re-exports, `export *` and
//! `import()` of a literal, each with the offset the report gives its line and column from.

use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;
use oxc_syntax::module_record::{ExportExportName, ExportImportName, ImportImportName};

/// What an import takes of a module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Taken {
    /// One export by its name, `default` for a default import.
    Name(String),
    /// Every export: `import * as`, `export * as`, `import()`.
    All,
    /// Nothing: `import "./x"`, or `export * from`, which passes names on without using them.
    Nothing,
}

/// One import, re-export or `import()`: its specifier, what it takes, where its specifier is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    /// The specifier as written, without its quotes.
    pub specifier: String,
    /// What it takes.
    pub taken: Taken,
    /// The offset of the specifier's literal.
    pub offset: u32,
}

/// One export of the module: its own declaration, or a re-export of another module's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export {
    /// The exported name, `default` for a default export.
    pub name: String,
    /// What it declares: `function`, `class`, `interface`, `type`, `enum`, or empty.
    pub kind: &'static str,
    /// Whether it is a type: an interface, a type alias, an enum, or `export type`.
    pub is_type: bool,
    /// The offset of its name.
    pub offset: u32,
    /// For a re-export, its specifier and the name it takes there.
    pub from: Option<(String, Taken)>,
    /// The names its own declaration names, for the types its signature uses; empty for a
    /// re-export.
    pub names: Vec<String>,
}

/// A module's imports and exports.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Module {
    /// Every import, re-export, `export *` and `import()` of a literal.
    pub imports: Vec<Import>,
    /// Every export, its own and re-exported.
    pub exports: Vec<Export>,
    /// The specifiers of its `export *`.
    pub stars: Vec<String>,
}

/// The kind of declaration an `export` statement opens with.
fn kind_of(statement: &str) -> &'static str {
    let rest = statement.trim_start_matches("export").trim_start();
    let rest = rest
        .strip_prefix("declare ")
        .unwrap_or(rest)
        .trim_start_matches("async ")
        .trim_start_matches("abstract ")
        .trim_start_matches("const enum");

    [
        ("function", "function"),
        ("class", "class"),
        ("interface", "interface"),
        ("type ", "type"),
        ("type{", "type"),
        ("enum", "enum"),
    ]
    .iter()
    .find(|(start, _)| rest.starts_with(start))
    .map_or_else(
        || {
            if statement.contains("const enum") {
                "enum"
            } else {
                ""
            }
        },
        |(_, kind)| kind,
    )
}

/// The identifiers a declaration's text holds, each once, in their order.
fn identifiers(statement: &str) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    for word in statement.split(|character: char| {
        !(character.is_alphanumeric() || character == '_' || character == '$')
    }) {
        if word
            .chars()
            .next()
            .is_some_and(|first| !first.is_ascii_digit())
            && !names.iter().any(|name| name == word)
        {
            names.push(word.to_owned());
        }
    }
    names
}

/// The text of a string literal, its quotes dropped; `None` for any other expression.
fn literal(text: &str) -> Option<String> {
    let quote = text
        .chars()
        .next()
        .filter(|first| matches!(first, '"' | '\'' | '`'))?;
    let inner = text.strip_prefix(quote)?.strip_suffix(quote)?;

    (quote != '`' || !inner.contains("${")).then(|| inner.to_owned())
}

/// What an `ExportImportName` takes.
fn taken_by(name: &ExportImportName) -> Taken {
    match name {
        ExportImportName::Name(name) => Taken::Name(name.name.to_string()),
        ExportImportName::All => Taken::All,
        ExportImportName::AllButDefault | ExportImportName::Null => Taken::Nothing,
    }
}

/// The imports and exports of a module's source; `None` when Oxc cannot parse it.
#[must_use]
pub fn read(source: &str) -> Option<Module> {
    let allocator = Allocator::default();
    let parsed = Parser::new(&allocator, source, SourceType::ts()).parse();

    if parsed.fatal_error {
        return None;
    }
    let record = &parsed.module_record;
    let text = |start: u32, end: u32| source.get(start as usize..end as usize).unwrap_or("");
    let mut module = Module::default();

    for (specifier, requests) in &record.requested_modules {
        for request in requests.iter().filter(|request| request.is_import) {
            module.imports.push(Import {
                specifier: specifier.to_string(),
                taken: Taken::Nothing,
                offset: request.span.start,
            });
        }
    }
    for entry in &record.import_entries {
        module.imports.push(Import {
            specifier: entry.module_request.name.to_string(),
            taken: match &entry.import_name {
                ImportImportName::Name(name) => Taken::Name(name.name.to_string()),
                ImportImportName::NamespaceObject => Taken::All,
                ImportImportName::Default(_) => Taken::Name("default".to_owned()),
            },
            offset: entry.module_request.span.start,
        });
    }
    for dynamic in &record.dynamic_imports {
        if let Some(specifier) = literal(text(
            dynamic.module_request.start,
            dynamic.module_request.end,
        )) {
            module.imports.push(Import {
                specifier,
                taken: Taken::All,
                offset: dynamic.module_request.start,
            });
        }
    }
    for entry in record
        .local_export_entries
        .iter()
        .chain(&record.indirect_export_entries)
    {
        let (name, offset) = match &entry.export_name {
            ExportExportName::Name(name) => (name.name.to_string(), name.span.start),
            ExportExportName::Default(span) => ("default".to_owned(), span.start),
            ExportExportName::Null => continue,
        };
        let from = entry.module_request.as_ref().map(|request| {
            module.imports.push(Import {
                specifier: request.name.to_string(),
                taken: Taken::Nothing,
                offset: request.span.start,
            });
            (request.name.to_string(), taken_by(&entry.import_name))
        });
        let statement = text(entry.statement_span.start, entry.statement_span.end);
        let kind = if from.is_some() {
            if entry.is_type { "type" } else { "" }
        } else {
            kind_of(statement)
        };
        let names = if from.is_some() {
            Vec::new()
        } else {
            identifiers(statement)
        };

        module.exports.push(Export {
            name,
            kind,
            is_type: entry.is_type || matches!(kind, "interface" | "type" | "enum"),
            offset,
            from,
            names,
        });
    }
    for entry in &record.star_export_entries {
        if let Some(request) = &entry.module_request {
            module.stars.push(request.name.to_string());
            module.imports.push(Import {
                specifier: request.name.to_string(),
                taken: Taken::Nothing,
                offset: request.span.start,
            });
        }
    }
    Some(module)
}

/// The line and column, from 1, of an offset in a text; the column counts characters.
#[must_use]
pub fn line_column(text: &str, offset: u32) -> (usize, usize) {
    let before = text.get(..offset as usize).unwrap_or(text);
    let line = before.matches('\n').count() + 1;
    let column = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;

    (line, column)
}
