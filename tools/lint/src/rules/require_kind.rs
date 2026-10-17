//! `local/require-kind`: every top-level function carries exactly one known `@kind` tag
//! (ADR-0011).

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::kind_names;
use crate::source::Source;

/// The rule.
pub struct RequireKind;

impl Rule for RequireKind {
    fn name(&self) -> &'static str {
        "local/require-kind"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let kinds = kind_names();

        for function in source.top_level_functions() {
            let values = source.tags(&function, "kind");
            let message = match values.as_slice() {
                [] => "Top-level function without `@kind` tag (ADR-0011).".to_owned(),
                [_, _, ..] => {
                    "A function has exactly one `@kind`: split it if it mixes concerns.".to_owned()
                }
                [kind] if kinds.contains(kind) => continue,
                [kind] => format!(
                    "Unknown kind `{kind}`; expected one of: {}.",
                    kinds.join(", ")
                ),
            };
            findings.push(source.finding(self.name(), Source::name_offset(&function), message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RequireKind;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&RequireKind, "src/a.ts", code))
    }

    #[test]
    fn reports_a_function_without_kind() {
        assert_eq!(
            found("function f(): void {}"),
            ["Top-level function without `@kind` tag (ADR-0011)."]
        );
        assert_eq!(found("/** Doc. */\nexport function f(): void {}").len(), 1);
    }

    #[test]
    fn reports_two_kinds_and_an_unknown_or_empty_one() {
        assert_eq!(
            found(
                "/**\n * Doc.\n *\n * @kind math\n * @kind procedure\n */\nfunction f(): void {}"
            ),
            ["A function has exactly one `@kind`: split it if it mixes concerns."]
        );
        assert_eq!(
            found("/** @kind magic */\nfunction f(): void {}"),
            ["Unknown kind `magic`; expected one of: math, geometry, domain, format, procedure."]
        );
        assert_eq!(
            found("/** @kind */\nfunction f(): void {}"),
            ["Unknown kind ``; expected one of: math, geometry, domain, format, procedure."]
        );
    }

    #[test]
    fn accepts_one_known_kind_and_skips_inner_functions_and_arrows() {
        assert_eq!(found("/** @kind math */\nfunction f(): void {}"), NONE);
        assert_eq!(
            found("/** @kind procedure */\nexport function f(): void {}"),
            NONE
        );
        assert_eq!(
            found("/** @kind math */\nfunction f(): void {\n  function inner(): void {}\n}"),
            NONE
        );
        assert_eq!(found("const f = (): number => 1;"), NONE);
    }

    #[test]
    fn points_at_the_function_name() {
        let findings = lint(&RequireKind, "src/a.ts", "\nexport function f(): void {}");

        assert_eq!((findings[0].line, findings[0].column), (2, 17));
    }
}
