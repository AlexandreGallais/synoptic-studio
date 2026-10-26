//! `local/barrel-exports`: a folder's `index.ts` only re-exports, `export * from "./x";`, every
//! module and barrelled sub-folder of its folder, sorted (ADR-0012). Autofixed: the barrel is
//! written again from the folder, a line written `export type * from "./x";` kept so (EN-138).

use std::cmp::Ordering;

use oxc_ast::ast::Statement;
use oxc_span::GetSpan;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct BarrelExports;

/// Whether a file name is a module of the folder's public surface: a `.ts` file other than the
/// barrel, a test, a declaration file or a test runner's global setup, which Vitest loads in its
/// own process (EN-176).
fn is_public_module(name: &str) -> bool {
    name.rsplit_once('.')
        .is_some_and(|(_, extension)| extension == "ts")
        && name != "index.ts"
        && !name.ends_with(".d.ts")
        && !name.ends_with(".test.ts")
        && !name.ends_with(".setup.ts")
}

/// The order of a character in `localeCompare("en")`, for the names of modules: punctuation,
/// then digits, then letters, case aside.
fn weight(letter: char) -> (u8, char) {
    let lower = letter.to_ascii_lowercase();
    let class = if lower.is_ascii_digit() {
        2
    } else if lower.is_alphabetic() {
        3
    } else {
        1
    };
    (class, lower)
}

/// Two specifiers compared as `localeCompare("en")` compares them: by letter case aside, then a
/// lower-case letter before its capital.
#[must_use]
pub fn locale_order(left: &str, right: &str) -> Ordering {
    left.chars()
        .map(weight)
        .cmp(right.chars().map(weight))
        .then_with(|| {
            left.chars()
                .map(|letter| letter.is_ascii_uppercase())
                .cmp(right.chars().map(|letter| letter.is_ascii_uppercase()))
        })
}

/// The specifiers a barrel re-exports: each module and each sub-folder with a barrel, sorted.
fn expected(source: &Source) -> Vec<String> {
    let folder = source.folder();
    let mut specifiers: Vec<String> = source
        .project
        .entries(&folder)
        .iter()
        .filter_map(|entry| {
            if entry.is_folder {
                source
                    .project
                    .has_index(&folder.join(&entry.name))
                    .then(|| format!("./{}", entry.name))
            } else if entry.is_file && is_public_module(&entry.name) {
                Some(format!("./{}", entry.name.trim_end_matches(".ts")))
            } else {
                None
            }
        })
        .collect();
    specifiers.sort_by(|left, right| locale_order(left, right));
    specifiers
}

impl Rule for BarrelExports {
    fn name(&self) -> &'static str {
        "local/barrel-exports"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut actual = Vec::new();
        let mut types_only = Vec::new();
        for statement in &source.program.body {
            match statement {
                Statement::ExportAllDeclaration(export)
                    if export.exported.is_none()
                        && export
                            .source
                            .value
                            .as_str()
                            .is_some_and(|specifier| specifier.starts_with("./")) =>
                {
                    actual.extend(export.source.value.as_str());
                    if export.export_kind.is_type() {
                        types_only.extend(export.source.value.as_str());
                    }
                }
                _ => {
                    findings.push(
                        source.finding(
                            self.name(),
                            statement.span().start,
                            "A barrel only contains `export * from \"./module\";` statements."
                                .to_owned(),
                        ),
                    );
                    return;
                }
            }
        }
        let expected = expected(source);
        if actual == expected {
            return;
        }
        let text = expected.iter().fold(String::new(), |text, specifier| {
            let kind = if types_only.contains(&specifier.as_str()) {
                "export type * from \""
            } else {
                "export * from \""
            };
            text + kind + specifier + "\";\n"
        });
        let fix = Fix {
            start: 0,
            end: u32::try_from(source.text.len()).unwrap_or(u32::MAX),
            text,
        };
        let at = source
            .program
            .body
            .first()
            .map_or(0, |statement| statement.span().start);
        let message = format!(
            "Barrel out of sync with its folder; expected: {}.",
            expected.join(", ")
        );
        findings.push(source.fixed(self.name(), at, message, fix));
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use std::fs;
    use std::path::PathBuf;

    use super::{BarrelExports, locale_order};
    use crate::testing::{NONE, fixed, fixture, lint_at, messages, write_files};

    /// Two modules, a test, a global setup (EN-176), a barrelled sub-folder, a folder without a
    /// barrel and an image.
    fn folder() -> PathBuf {
        let root = fixture("barrel");
        write_files(
            &root,
            &[
                "index.ts",
                "vector.ts",
                "angle.ts",
                "angle.test.ts",
                "fonts.setup.ts",
                "arc/index.ts",
                "types.d.ts",
            ],
        );
        fs::create_dir_all(root.join("assets")).unwrap_or_default();
        fs::write(root.join("assets/logo.svg"), "<svg/>").unwrap_or_default();
        root
    }

    /// The barrel of the fixture folder.
    const EXPECTED: &str =
        "export * from \"./angle\";\nexport * from \"./arc\";\nexport * from \"./vector\";\n";

    #[test]
    fn writes_a_barrel_missing_modules_again() {
        let code = "export * from \"./vector\";";
        let findings = lint_at(&BarrelExports, &folder(), "index.ts", code);

        assert_eq!(
            messages(&findings),
            ["Barrel out of sync with its folder; expected: ./angle, ./arc, ./vector."]
        );
        assert_eq!(fixed(code, &findings), EXPECTED);
    }

    #[test]
    fn sorts_an_unsorted_barrel() {
        let code =
            "export * from \"./vector\";\nexport * from \"./arc\";\nexport * from \"./angle\";\n";
        let findings = lint_at(&BarrelExports, &folder(), "index.ts", code);

        assert_eq!(fixed(code, &findings), EXPECTED);
    }

    #[test]
    fn f31_ac2_npm_run_fix_keeps_a_type_only_reexport_when_local_barrel_exports_writes_the_barrel()
    {
        // The card of 2026-10-18: `npm run fix`, `tools/lint`'s `--fix`, met a barrel out of sync
        // with a module just added (`epic-17-f22-lines.ts`) and `local/barrel-exports` wrote it
        // again as `export *` lines only, `export type * from "./checklist-line"` with them.
        let code = "export type * from \"./angle\";\nexport * from \"./vector\";\n";
        let findings = lint_at(&BarrelExports, &folder(), "index.ts", code);

        assert_eq!(
            fixed(code, &findings),
            "export type * from \"./angle\";\nexport * from \"./arc\";\nexport * from \"./vector\";\n"
        );
        // In sync, a type-only re-export is left as it is.
        let synced = "export type * from \"./angle\";\nexport * from \"./arc\";\nexport * from \"./vector\";\n";

        assert_eq!(
            messages(&lint_at(&BarrelExports, &folder(), "index.ts", synced)),
            NONE
        );
    }

    #[test]
    fn refuses_anything_but_star_reexports() {
        for code in [
            "export { a } from \"./vector\";",
            "export const A = 1;",
            "export * as a from \"./vector\";",
            "export * from \"../vector\";",
        ] {
            assert_eq!(
                messages(&lint_at(&BarrelExports, &folder(), "index.ts", code)),
                ["A barrel only contains `export * from \"./module\";` statements."],
                "{code}"
            );
        }
    }

    #[test]
    fn accepts_a_barrel_in_sync() {
        assert_eq!(
            messages(&lint_at(&BarrelExports, &folder(), "index.ts", EXPECTED)),
            NONE
        );
    }

    #[test]
    fn sorts_as_locale_compare_does() {
        assert_eq!(locale_order("./a-b", "./ab"), Ordering::Less);
        assert_eq!(locale_order("./a2", "./ab"), Ordering::Less);
        assert_eq!(locale_order("./B", "./a"), Ordering::Greater);
        assert_eq!(locale_order("./a", "./A"), Ordering::Less);
        assert_eq!(locale_order("./a", "./a"), Ordering::Equal);
    }
}
