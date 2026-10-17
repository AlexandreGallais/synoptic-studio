//! `local/see-references`: every `@see` of a `JSDoc` block points to an existing and verified
//! reference — `REF-*` in the bibliography, `DERIV-*`, `ADR-NNNN` or a `docs/…` page — and every
//! top-level function of a known kind has one (docs/conventions/documentation.md). It guards
//! against invented references.

use std::path::Path;

use crate::finding::Finding;
use crate::project::Project;
use crate::rules::Rule;
use crate::settings::kind_names;
use crate::source::{Source, tag_values};

/// The rule.
pub struct SeeReferences;

/// What a `@see` target is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// It exists and may be cited.
    Valid,
    /// Nothing of that name exists.
    Missing,
    /// Its bibliography row is marked unverified.
    Unverified,
    /// It is none of the families a `@see` may cite.
    UnknownFormat,
}

/// Whether a text is a prefix followed by at least one character of a set.
fn is_identifier(target: &str, prefix: &str, allowed: fn(char) -> bool) -> bool {
    target
        .strip_prefix(prefix)
        .is_some_and(|rest| !rest.is_empty() && rest.chars().all(allowed))
}

/// The status of a `@see` target.
#[must_use]
pub fn status(project: &Project, target: &str) -> Status {
    let sources = project.sources;
    if is_identifier(target, "REF-", |letter| {
        letter.is_ascii_uppercase() || letter.is_ascii_digit() || letter == '-'
    }) {
        return match project.bibliography().get(target) {
            None => Status::Missing,
            Some(true) => Status::Valid,
            Some(false) => Status::Unverified,
        };
    }
    if is_identifier(target, "DERIV-", |letter| {
        letter.is_ascii_lowercase() || letter.is_ascii_digit() || letter == '-'
    }) {
        let file = Path::new(sources.derivations_directory)
            .join(format!("{}.md", &target["DERIV-".len()..]));
        return found(project.is_file(&project.root.join(file)));
    }
    if target.len() == "ADR-0000".len()
        && is_identifier(target, "ADR-", |letter| letter.is_ascii_digit())
    {
        let prefix = format!("{}-", &target["ADR-".len()..]);
        return found(
            project
                .decisions()
                .iter()
                .any(|name| name.starts_with(&prefix)),
        );
    }
    let path = target.split('#').next().unwrap_or_default();
    if target.starts_with("docs/")
        && path
            .rsplit_once('.')
            .is_some_and(|(_, extension)| extension == "md")
        && path.len() > "docs/.md".len()
        && !target.contains(char::is_whitespace)
    {
        return found(project.root.join(path).exists());
    }
    Status::UnknownFormat
}

/// Valid when found, missing otherwise.
fn found(exists: bool) -> Status {
    if exists {
        Status::Valid
    } else {
        Status::Missing
    }
}

impl Rule for SeeReferences {
    fn name(&self) -> &'static str {
        "local/see-references"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let kinds = kind_names();
        for function in source.top_level_functions() {
            let kind = source.kind(&function).unwrap_or_default();
            if kinds.contains(&kind) && source.tags(&function, "see").is_empty() {
                let message = format!(
                    "`{kind}` function without `@see`: cite a REF-*, DERIV-*, ADR-* or docs/ page."
                );
                findings.push(source.finding(self.name(), Source::name_offset(&function), message));
            }
        }

        for comment in source
            .program
            .comments
            .iter()
            .filter(|comment| source.is_jsdoc(comment))
        {
            for target in tag_values(Some(source.comment_value(comment)), "see") {
                let message = match status(source.project, target) {
                    Status::Valid => continue,
                    Status::Missing => {
                        format!("`@see {target}` does not exist in the documentation.")
                    }
                    Status::Unverified => format!(
                        "`@see {target}` is marked [unverified] in docs/references.md: verify it before citing it."
                    ),
                    Status::UnknownFormat => format!(
                        "`@see {target}`: expected REF-*, DERIV-*, ADR-NNNN or docs/…/file.md."
                    ),
                };
                findings.push(source.finding(self.name(), comment.span.start, message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::SeeReferences;
    use crate::testing::{NONE, fixture, lint_at, messages};

    /// A repository with one verified and one unverified reference, a derivation and an ADR.
    fn documentation() -> PathBuf {
        let root = fixture("see-references");
        for folder in ["docs/adr", "docs/derivations", "src"] {
            fs::create_dir_all(root.join(folder)).unwrap_or_default();
        }
        let files = [
            (
                "docs/references.md",
                "| ID | Ref | Use | Status |\n|---|---|---|---|\n| `REF-OK` | x | y | [verified] |\n| `REF-TODO` | x | y | [unverified] |\n",
            ),
            ("docs/derivations/fit.md", "# fit\n"),
            ("docs/adr/0001-scope.md", "# scope\n"),
        ];
        for (file, text) in files {
            fs::write(root.join(file), text).unwrap_or_default();
        }
        root
    }

    /// A math function citing a target.
    fn citing(target: &str) -> String {
        format!("/**\n * Doc.\n *\n * @kind math\n * @see {target}\n */\nfunction f(): void {{}}")
    }

    /// The messages on a piece of code in the repository.
    fn found(code: &str) -> Vec<String> {
        messages(&lint_at(&SeeReferences, &documentation(), "src/a.ts", code))
    }

    #[test]
    fn asks_a_function_of_a_kind_for_a_see() {
        assert_eq!(
            found("/** @kind math */\nfunction f(): void {}"),
            ["`math` function without `@see`: cite a REF-*, DERIV-*, ADR-* or docs/ page."]
        );
        assert_eq!(found("function f(): void {}"), NONE);
    }

    #[test]
    fn reports_a_target_that_does_not_exist() {
        for target in ["REF-NOPE", "DERIV-nope", "ADR-0002", "docs/nope.md"] {
            assert_eq!(
                found(&citing(target)),
                [format!(
                    "`@see {target}` does not exist in the documentation."
                )]
            );
        }
    }

    #[test]
    fn reports_an_unverified_reference_and_an_unknown_format() {
        assert_eq!(
            found(&citing("REF-TODO")),
            [
                "`@see REF-TODO` is marked [unverified] in docs/references.md: verify it before citing it."
            ]
        );
        for target in [
            "https://example.org",
            "ADR-12",
            "REF-a",
            "DERIV-A",
            "docs/a.txt",
        ] {
            assert_eq!(
                found(&citing(target)),
                [format!(
                    "`@see {target}`: expected REF-*, DERIV-*, ADR-NNNN or docs/…/file.md."
                )]
            );
        }
    }

    #[test]
    fn accepts_existing_and_verified_targets() {
        for target in [
            "REF-OK",
            "DERIV-fit",
            "ADR-0001",
            "docs/references.md#section",
        ] {
            assert_eq!(found(&citing(target)), NONE);
        }
        // Every kind of the project asks for a `@see`, `procedure` included.
        assert_eq!(
            found("/** @kind procedure */\nfunction f(): void {}").len(),
            1
        );
    }

    #[test]
    fn checks_every_jsdoc_block_of_the_file_at_its_place() {
        let findings = lint_at(
            &SeeReferences,
            &documentation(),
            "src/a.ts",
            "const A = 1;\n/** A value. @see REF-NOPE */\nconst B = 2;\n// @see REF-NOPE\n",
        );

        assert_eq!(
            (findings.len(), findings[0].line, findings[0].column),
            (1, 2, 1)
        );
    }
}
