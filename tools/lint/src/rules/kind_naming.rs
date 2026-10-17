//! `local/kind-naming`: a top-level function's name fits its kind — a verb for a `procedure`, a
//! conversion for a `format`, no vague verb for the pure kinds — and starts with a predicate word
//! exactly when it returns a boolean (docs/conventions/naming.md).

use oxc_ast::ast::{Function, TSType};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::{FORMAT_PREFIXES, PREDICATE_PREFIXES, PROCEDURE_VERBS, VAGUE_PREFIXES};
use crate::source::Source;

/// The rule.
pub struct KindNaming;

/// The first lower-case word of a camelCase name, empty when it starts with a capital.
fn first_word(name: &str) -> &str {
    let end = name
        .find(|letter: char| !letter.is_ascii_lowercase())
        .unwrap_or(name.len());

    &name[..end]
}

/// Whether a name reads as a conversion: `toX`, `parseX`… or `xToY`.
fn reads_as_conversion(name: &str) -> bool {
    let after_prefix = FORMAT_PREFIXES.iter().any(|prefix| {
        name.strip_prefix(prefix)
            .is_some_and(|rest| rest.starts_with(|letter: char| letter.is_ascii_uppercase()))
    });
    let infix = name.starts_with(|letter: char| letter.is_ascii_lowercase())
        && name.char_indices().skip(1).any(|(at, _)| {
            name[at..].strip_prefix("To").is_some_and(|rest| {
                rest.starts_with(|letter: char| letter.is_ascii_uppercase())
                    && name[1..at]
                        .chars()
                        .all(|letter| letter.is_ascii_alphanumeric())
            })
        });

    after_prefix || infix
}

/// Whether a function is declared as returning a boolean or a type predicate.
fn returns_boolean(function: &Function) -> bool {
    function.return_type.as_ref().is_some_and(|annotation| {
        matches!(
            annotation.type_annotation,
            TSType::TSBooleanKeyword(_) | TSType::TSTypePredicate(_)
        )
    })
}

/// What a name breaks, predicate consistency first, then the rule of its kind.
fn violation(name: &str, kind: &str, function: &Function) -> Option<String> {
    let word = first_word(name);
    let is_predicate_name = PREDICATE_PREFIXES.contains(&word);

    if returns_boolean(function) != is_predicate_name {
        return Some(if is_predicate_name {
            format!("`{name}` starts with `{word}` but does not return a boolean.")
        } else {
            format!(
                "`{name}` returns a boolean: start its name with a predicate word (`is`, `has`, `can`…)."
            )
        });
    }
    match kind {
        "procedure" if !PROCEDURE_VERBS.contains(&word) => Some(format!(
            "`procedure` `{name}` must start with an allowed verb (tools/lint/src/settings.rs), not `{word}`."
        )),
        "format" if !reads_as_conversion(name) => Some(format!(
            "`format` function `{name}` must read as a conversion: `xToY`, `toX`, `parseX`, `formatX`…"
        )),
        "math" | "geometry" | "domain" if VAGUE_PREFIXES.contains(&word) => Some(format!(
            "`{kind}` function `{name}` starts with vague `{word}`: name the concept itself."
        )),
        _ => None,
    }
}

impl Rule for KindNaming {
    fn name(&self) -> &'static str {
        "local/kind-naming"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        for function in source.top_level_functions() {
            let Some(id) = &function.function.id else {
                continue;
            };
            let kind = source.kind(&function).unwrap_or_default();
            if let Some(message) = violation(&id.name, kind, function.function) {
                findings.push(source.finding(self.name(), id.span.start, message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{KindNaming, reads_as_conversion};
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn found(code: &str) -> Vec<String> {
        messages(&lint(&KindNaming, "src/a.ts", code))
    }

    #[test]
    fn asks_a_procedure_for_a_verb() {
        assert_eq!(
            found("/** @kind procedure */\nfunction pathElement(): void {}"),
            [
                "`procedure` `pathElement` must start with an allowed verb (tools/lint/src/settings.rs), not `path`."
            ]
        );
        assert_eq!(
            found("/** @kind procedure */\nfunction renderShape(): void {}"),
            NONE
        );
    }

    #[test]
    fn asks_a_format_function_to_read_as_a_conversion() {
        assert_eq!(
            found("/** @kind format */\nfunction svgPath(): string { return ''; }"),
            [
                "`format` function `svgPath` must read as a conversion: `xToY`, `toX`, `parseX`, `formatX`…"
            ]
        );
        assert_eq!(
            found("/** @kind format */\nfunction pathToSvg(): string { return ''; }"),
            NONE
        );
        assert_eq!(
            found("/** @kind format */\nfunction toSvg(): string { return ''; }"),
            NONE
        );
    }

    #[test]
    fn refuses_a_vague_verb_on_the_pure_kinds() {
        assert_eq!(
            found("/** @kind math */\nfunction getDot(): number { return 0; }"),
            ["`math` function `getDot` starts with vague `get`: name the concept itself."]
        );
        assert_eq!(
            found("/** @kind math */\nfunction dot(): number { return 0; }"),
            NONE
        );
    }

    #[test]
    fn keeps_predicate_words_for_booleans() {
        assert_eq!(
            found("/** @kind domain */\nfunction connectable(): boolean { return true; }"),
            [
                "`connectable` returns a boolean: start its name with a predicate word (`is`, `has`, `can`…)."
            ]
        );
        assert_eq!(
            found("/** @kind domain */\nfunction isPort(): number { return 0; }"),
            ["`isPort` starts with `is` but does not return a boolean."]
        );
        assert_eq!(
            found("/** @kind domain */\nfunction canConnect(): boolean { return true; }"),
            NONE
        );
        assert_eq!(
            found("/** @kind domain */\nfunction isPort(x: unknown): x is string { return true; }"),
            NONE
        );
    }

    #[test]
    fn checks_only_the_predicate_words_without_a_kind() {
        assert_eq!(found("function getAnything(): number { return 0; }"), NONE);
        assert_eq!(
            found("function isAnything(): number { return 0; }").len(),
            1
        );
    }

    #[test]
    fn reads_the_format_pattern_as_the_regular_expression_did() {
        // `^(?:(?:to|from|parse|serialize|format|encode|decode)[A-Z]|[a-z][A-Za-z0-9]*To[A-Z])`
        for name in ["toSvg", "fromJson", "decodeX", "aToB", "svg2ToPath"] {
            assert!(reads_as_conversion(name), "{name}");
        }
        for name in [
            "to", "tosvg", "Tox", "ToX", "svgTo", "svgTox", "a_bToC", "svgPath",
        ] {
            assert!(!reads_as_conversion(name), "{name}");
        }
    }
}
