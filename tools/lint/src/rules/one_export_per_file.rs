//! `local/one-export-per-file`: a module exports one value, named like the file in kebab-case
//! (ADR-0019). A type may sit beside it only when it is part of the value's declaration, its
//! contract; a module without a value exports exactly one type. A type and a value sharing a
//! name count once.

use oxc_ast::ast::{BindingPattern, Declaration, Program, Statement};
use oxc_span::{GetSpan, Span};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct OneExportPerFile;

/// A name an `export <declaration>` statement declares.
#[derive(Clone, Copy, Debug)]
struct Exported<'a> {
    /// The name.
    name: &'a str,
    /// Whether it is a type alias or an interface.
    is_type: bool,
    /// Where the name is written.
    at: u32,
    /// The declaration holding it, whose text is the value's contract.
    declaration: Span,
}

/// The names an exported declaration declares.
fn declared<'a>(declaration: &Declaration<'a>) -> Vec<Exported<'a>> {
    let single = |name: &'a str, at: Span, is_type: bool| {
        vec![Exported {
            name,
            is_type,
            at: at.start,
            declaration: declaration.span(),
        }]
    };
    match declaration {
        Declaration::VariableDeclaration(variables) => variables
            .declarations
            .iter()
            .filter_map(|declarator| match &declarator.id {
                BindingPattern::BindingIdentifier(id) => Some(Exported {
                    name: id.name.as_str(),
                    is_type: false,
                    at: id.span.start,
                    declaration: declarator.span,
                }),
                _ => None,
            })
            .collect(),
        Declaration::FunctionDeclaration(function) => function
            .id
            .as_ref()
            .map_or_else(Vec::new, |id| single(id.name.as_str(), id.span, false)),
        Declaration::ClassDeclaration(class) => class
            .id
            .as_ref()
            .map_or_else(Vec::new, |id| single(id.name.as_str(), id.span, false)),
        Declaration::TSTypeAliasDeclaration(alias) => {
            single(alias.id.name.as_str(), alias.id.span, true)
        }
        Declaration::TSInterfaceDeclaration(interface) => {
            single(interface.id.name.as_str(), interface.id.span, true)
        }
        Declaration::TSEnumDeclaration(enumeration) => {
            single(enumeration.id.name.as_str(), enumeration.id.span, false)
        }
        Declaration::TSNamespaceDeclaration(namespace) => {
            single(namespace.id.name.as_str(), namespace.id.span, false)
        }
        Declaration::TSImportEqualsDeclaration(import) => {
            single(import.id.name.as_str(), import.id.span, false)
        }
        Declaration::TSExternalModuleDeclaration(_) | Declaration::TSGlobalDeclaration(_) => {
            Vec::new()
        }
    }
}

/// The statement exporting otherwise than `export <declaration>`, with its message.
fn irregular(program: &Program) -> Option<(u32, &'static str)> {
    let reexport = program.body.iter().find(|statement| {
        matches!(
            statement,
            Statement::ExportAllDeclaration(_) | Statement::ExportFromDeclaration(_)
        )
    });
    if let Some(statement) = reexport {
        return Some((
            statement.span().start,
            "Re-exports belong to the folder's `index.ts`.",
        ));
    }
    program
        .body
        .iter()
        .find(|statement| matches!(statement, Statement::ExportNamedDeclaration(_)))
        .map(|statement| {
            (
                statement.span().start,
                "Export the declaration itself (`export function …`), not an `export { … }` list.",
            )
        })
}

/// The file name of an identifier in kebab-case: `formatSvgNumber` → `format-svg-number`,
/// `toSVGPath` → `to-svg-path`, `SVG_DECIMALS` → `svg-decimals`.
#[must_use]
pub fn kebab_case(name: &str) -> String {
    let letters: Vec<char> = name.chars().collect();
    let mut lower_upper = Vec::with_capacity(letters.len() + 4);
    let mut index = 0;
    while index < letters.len() {
        lower_upper.push(letters[index]);
        let pair = letters.get(index + 1).is_some_and(char::is_ascii_uppercase)
            && (letters[index].is_ascii_lowercase() || letters[index].is_ascii_digit());
        if pair {
            lower_upper.push('-');
            lower_upper.push(letters[index + 1]);
            index += 1;
        }
        index += 1;
    }
    let mut words = String::with_capacity(lower_upper.len() + 4);
    for (at, letter) in lower_upper.iter().enumerate() {
        let acronym_ends = letter.is_ascii_uppercase()
            && at > 0
            && lower_upper[at - 1].is_ascii_uppercase()
            && lower_upper
                .get(at + 1)
                .is_some_and(char::is_ascii_lowercase);
        if acronym_ends {
            words.push('-');
        }
        words.push(*letter);
    }
    words.replace('_', "-").to_lowercase()
}

/// Whether a name appears in a text as a whole word.
fn has_word(text: &str, name: &str) -> bool {
    let is_word = |letter: char| letter.is_ascii_alphanumeric() || letter == '_';
    text.match_indices(name).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + name.len()..].chars().next();
        before.is_none_or(|letter| !is_word(letter)) && after.is_none_or(|letter| !is_word(letter))
    })
}

/// The problems of a module's exports, in the order `ESLint` reported them.
fn problems(exported: &[Exported], expected: &str, text: &str) -> Vec<(u32, String)> {
    let values: Vec<&Exported> = exported.iter().filter(|each| !each.is_type).collect();
    let types: Vec<&Exported> = exported.iter().filter(|each| each.is_type).collect();
    let Some(main) = values.first().or(types.first()) else {
        return Vec::new();
    };
    let mut found: Vec<(u32, String)> = values
        .iter()
        .skip(1)
        .map(|each| {
            (
                each.at,
                format!(
                    "One exported value per module; move `{}` to its own kebab-case file.",
                    each.name
                ),
            )
        })
        .collect();
    if values.is_empty() {
        found.extend(types.iter().skip(1).map(|each| {
            (
                each.at,
                format!(
                    "A type-only module exports exactly one type; move `{}` to its own kebab-case file.",
                    each.name
                ),
            )
        }));
    } else {
        let contract = &text[main.declaration.start as usize..main.declaration.end as usize];
        found.extend(
            types
                .iter()
                .filter(|each| !has_word(contract, each.name))
                .map(|each| {
                    (
                        each.at,
                        format!(
                            "Type `{}` is not part of the exported value's signature: move it to its own kebab-case file.",
                            each.name
                        ),
                    )
                }),
        );
    }
    let file_name = kebab_case(main.name);
    if file_name != expected {
        found.push((
            main.at,
            format!(
                "The file exports `{}`: rename the file `{file_name}.ts` (found `{expected}.ts`).",
                main.name
            ),
        ));
    }
    found
}

impl Rule for OneExportPerFile {
    fn name(&self) -> &'static str {
        "local/one-export-per-file"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let program = source.program;
        let exported: Vec<Exported> = program
            .body
            .iter()
            .filter_map(|statement| match statement {
                Statement::ExportDeclaration(export) => Some(declared(&export.declaration)),
                _ => None,
            })
            .flatten()
            .collect();

        if let Some((at, message)) = irregular(program) {
            findings.push(source.finding(self.name(), at, message.to_owned()));
            return;
        }
        if exported.is_empty() {
            findings.push(source.finding(
                self.name(),
                0,
                "A library module exports exactly one value or type.".to_owned(),
            ));
            return;
        }
        let expected = source.name().strip_suffix(".ts").unwrap_or(source.name());
        for (at, message) in problems(&exported, expected, source.text) {
            findings.push(source.finding(self.name(), at, message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OneExportPerFile, kebab_case};
    use crate::testing::{NONE, lint, messages};

    /// The module named after the function `filletSetback`.
    const FILLET_SETBACK: &str = "src/geometry/fillet-setback.ts";

    /// The messages on a piece of code at a path.
    fn found(path: &str, code: &str) -> Vec<String> {
        messages(&lint(&OneExportPerFile, path, code))
    }

    #[test]
    fn asks_for_the_file_named_after_the_export() {
        assert_eq!(
            found(
                "src/geometry/filletSetback.ts",
                "export function filletSetback(): void {}"
            ),
            [
                "The file exports `filletSetback`: rename the file `fillet-setback.ts` (found `filletSetback.ts`)."
            ]
        );
        assert_eq!(
            found(FILLET_SETBACK, "export function other(): void {}"),
            ["The file exports `other`: rename the file `other.ts` (found `fillet-setback.ts`)."]
        );
    }

    #[test]
    fn reports_a_second_value_and_an_unrelated_type() {
        assert_eq!(
            found(
                FILLET_SETBACK,
                "export function filletSetback(): void {}\nexport function other(): void {}"
            ),
            ["One exported value per module; move `other` to its own kebab-case file."]
        );
        assert_eq!(
            found(
                FILLET_SETBACK,
                "export type Unrelated = number;\nexport function filletSetback(): void {}"
            ),
            [
                "Type `Unrelated` is not part of the exported value's signature: move it to its own kebab-case file."
            ]
        );
    }

    #[test]
    fn reports_a_second_type_of_a_type_only_module() {
        assert_eq!(
            found(
                "src/math/point.ts",
                "export type Point = number;\nexport type Vector = number;"
            ),
            [
                "A type-only module exports exactly one type; move `Vector` to its own kebab-case file."
            ]
        );
    }

    #[test]
    fn reports_no_export_a_reexport_and_an_export_list() {
        assert_eq!(
            found(FILLET_SETBACK, "const A = 1;"),
            ["A library module exports exactly one value or type."]
        );
        assert_eq!(
            found(FILLET_SETBACK, "export * from \"./x\";"),
            ["Re-exports belong to the folder's `index.ts`."]
        );
        assert_eq!(
            found(
                FILLET_SETBACK,
                "export { a } from \"./x\";\nexport function filletSetback(): void {}"
            ),
            ["Re-exports belong to the folder's `index.ts`."]
        );
        assert_eq!(
            found(
                FILLET_SETBACK,
                "function filletSetback(): void {}\nexport { filletSetback };"
            ),
            ["Export the declaration itself (`export function …`), not an `export { … }` list."]
        );
    }

    #[test]
    fn accepts_one_value_named_like_the_file_and_its_contract() {
        assert_eq!(
            found(FILLET_SETBACK, "export function filletSetback(): void {}"),
            NONE
        );
        assert_eq!(
            found(
                "src/math/point.ts",
                "export type Point = { readonly x: number };"
            ),
            NONE
        );
        // The types are part of the function's contract: allowed next to it.
        assert_eq!(
            found(
                FILLET_SETBACK,
                "export type Setback = number;\nexport type Corner = number;\nexport function filletSetback(corner: Corner): Setback { return corner; }"
            ),
            NONE
        );
        assert_eq!(
            found(
                FILLET_SETBACK,
                "type Local = number;\nconst HALF: Local = 2;\nexport function filletSetback(): Local { return HALF; }"
            ),
            NONE
        );
        assert_eq!(
            found(FILLET_SETBACK, "export const filletSetback = 1;"),
            NONE
        );
        assert_eq!(
            found("src/io/svg-decimals.ts", "export const SVG_DECIMALS = 5;"),
            NONE
        );
        assert_eq!(
            found(
                "src/io/to-svg-path.ts",
                "export function toSVGPath(): void {}"
            ),
            NONE
        );
    }

    #[test]
    fn counts_once_a_type_and_a_value_sharing_a_name() {
        assert_eq!(
            found(
                "src/model/shape.ts",
                "export type Shape = { readonly x: number };\nexport const Shape = { x: 1 };"
            ),
            NONE
        );
    }

    #[test]
    fn writes_identifiers_in_kebab_case() {
        assert_eq!(kebab_case("formatSvgNumber"), "format-svg-number");
        assert_eq!(kebab_case("toSVGPath"), "to-svg-path");
        assert_eq!(kebab_case("SVG_DECIMALS"), "svg-decimals");
        assert_eq!(kebab_case("aBC"), "a-bc");
        assert_eq!(kebab_case("point2D"), "point2-d");
        assert_eq!(kebab_case("XMLHttpRequest"), "xml-http-request");
    }
}
