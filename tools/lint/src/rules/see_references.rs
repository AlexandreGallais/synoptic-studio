//! `local/see-references`: every `@see` of a `JSDoc` block cites a reference of a known form — a
//! `REF-*`, `DERIV-*`, `RULE-*` or `ADR-NNNN` id, the id of a backlog item, `EN-151` or `F48`, or a `docs/…`
//! page that exists — and every top-level function of a kind that computes or rules has one
//! (docs/conventions/documentation.md). The ids are read by their form alone: the bibliography,
//! the derivations, the decisions and the backlog live in the records, which the CI cannot read,
//! and `knowledge check-ids` checks each id against them, its source verified, before a pull
//! request into `develop` (E49). It guards against references of no known form.

use crate::finding::Finding;
use crate::project::Project;
use crate::rules::Rule;
use crate::source::{Source, tag_values};

/// The kinds whose functions cite a source: a calculation, a geometry or a business rule; a
/// `format` or a `procedure`, the playground's clicks and displays among them, may cite one but
/// need not (Product Owner, 2026-10-25).
const KINDS_CITING: [&str; 3] = ["math", "geometry", "domain"];

/// The kinds of stories the backlog has held, the prefix of their ids.
const STORY_KINDS: [&str; 9] = ["US", "EN", "SP", "CHK", "AUD", "VAL", "NAM", "REV", "RET"];

/// The rule.
pub struct SeeReferences;

/// What a `@see` target is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// It exists and may be cited.
    Valid,
    /// Nothing of that name exists.
    Missing,
    /// It is none of the families a `@see` may cite.
    UnknownFormat,
}

/// Whether a target is the id of a backlog item by its form: `E` or `F` and two digits, or a
/// story's kind, a dash and three digits.
#[must_use]
pub fn is_backlog_id(target: &str) -> bool {
    let digits = |text: &str, count: usize| {
        text.len() == count && text.bytes().all(|byte| byte.is_ascii_digit())
    };

    match target.split_once('-') {
        Some((kind, number)) => STORY_KINDS.contains(&kind) && digits(number, 3),
        None => target
            .split_at_checked(1)
            .is_some_and(|(letter, number)| matches!(letter, "E" | "F") && digits(number, 2)),
    }
}

/// Whether a text is a prefix followed by at least one character of a set.
fn is_identifier(target: &str, prefix: &str, allowed: fn(char) -> bool) -> bool {
    target
        .strip_prefix(prefix)
        .is_some_and(|rest| !rest.is_empty() && rest.chars().all(allowed))
}

/// The status of a `@see` target: a `REF-`, `DERIV-`, `RULE-` or `ADR-` id and a backlog id by
/// their form alone, since the bibliography, the derivations, the rules, the decisions and the
/// backlog live in the
/// records, which the CI cannot read, and `knowledge check-ids` checks each id cited against
/// them before a pull request into `develop` (E49); a `docs/` page by the file it names.
#[must_use]
pub fn status(project: &Project, target: &str) -> Status {
    let is_reference = is_identifier(target, "REF-", |letter| {
        letter.is_ascii_uppercase() || letter.is_ascii_digit() || letter == '-'
    });
    let is_derivation = is_identifier(target, "DERIV-", |letter| {
        letter.is_ascii_lowercase() || letter.is_ascii_digit() || letter == '-'
    });
    let is_rule = is_identifier(target, "RULE-", |letter| {
        letter.is_ascii_lowercase() || letter.is_ascii_digit() || letter == '-'
    });
    let is_decision = target.len() == "ADR-0000".len()
        && is_identifier(target, "ADR-", |letter| letter.is_ascii_digit());

    if is_reference || is_derivation || is_rule || is_decision || is_backlog_id(target) {
        return Status::Valid;
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
        for function in source.top_level_functions() {
            let kind = source.kind(&function).unwrap_or_default();
            if KINDS_CITING.contains(&kind) && source.tags(&function, "see").is_empty() {
                let message = format!(
                    "`{kind}` function without `@see`: cite a REF-*, DERIV-*, RULE-*, ADR-*, a backlog id or a docs/ page."
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
                    Status::UnknownFormat => format!(
                        "`@see {target}`: expected REF-*, DERIV-*, RULE-*, ADR-NNNN, a backlog id (EN-151, F48) or docs/…/file.md."
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

    /// A repository with one page of the docs.
    fn documentation() -> PathBuf {
        let root = fixture("see-references");
        for folder in ["docs/domain", "src"] {
            fs::create_dir_all(root.join(folder)).unwrap_or_default();
        }
        fs::write(root.join("docs/domain/shapes.md"), "# Shapes\n").unwrap_or_default();
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
            [
                "`math` function without `@see`: cite a REF-*, DERIV-*, RULE-*, ADR-*, a backlog id or a docs/ page."
            ]
        );
        assert_eq!(found("function f(): void {}"), NONE);
    }

    #[test]
    fn lets_a_procedure_or_a_format_go_without_a_see() {
        for kind in ["procedure", "format"] {
            assert_eq!(
                found(&format!("/** @kind {kind} */\nfunction f(): void {{}}")),
                NONE
            );
        }
        assert_eq!(found("/** @kind domain */\nfunction f(): void {}").len(), 1);
    }

    #[test]
    fn reports_a_page_of_the_docs_that_does_not_exist() {
        assert_eq!(
            found(&citing("docs/nope.md")),
            ["`@see docs/nope.md` does not exist in the documentation."]
        );
    }

    #[test]
    fn reports_a_target_of_no_known_form() {
        for target in [
            "https://example.org",
            "ADR-12",
            "ADR-00000",
            "REF-a",
            "DERIV-A",
            "RULE-Shapes",
            "docs/a.txt",
            "EN-15",
            "F100",
            "XX-001",
            "en-151",
        ] {
            assert_eq!(
                found(&citing(target)),
                [format!(
                    "`@see {target}`: expected REF-*, DERIV-*, RULE-*, ADR-NNNN, a backlog id (EN-151, F48) or docs/…/file.md."
                )]
            );
        }
    }

    #[test]
    fn accepts_the_ids_by_their_form_and_a_page_that_exists() {
        // The ids by their form alone: the CI has no records to read them in (E49).
        for target in [
            "REF-OK",
            "REF-NOT-IN-ANY-FILE",
            "DERIV-fit",
            "RULE-shapes-2-corner-radius",
            "ADR-0001",
            "ADR-0000",
            "docs/domain/shapes.md#section",
            "EN-151",
            "US-010",
            "RET-001",
            "F48",
            "E01",
        ] {
            assert_eq!(found(&citing(target)), NONE, "{target}");
        }
    }

    #[test]
    fn checks_every_jsdoc_block_of_the_file_at_its_place() {
        let findings = lint_at(
            &SeeReferences,
            &documentation(),
            "src/a.ts",
            "const A = 1;\n/** A value. @see REF-a */\nconst B = 2;\n// @see REF-a\n",
        );

        assert_eq!(
            (findings.len(), findings[0].line, findings[0].column),
            (1, 2, 1)
        );
    }
}
