//! `unicorn/prefer-array-iterable-methods`: no `for…of` over `array.entries()` that drops the
//! index or the value — the array itself, or `array.keys()`, says it (`eslint-plugin-unicorn`
//! 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{ArrayPattern, BindingPattern, ForStatementLeft, VariableDeclarationKind};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::syntax::{
    Edit, Kind, combined_fix, has_comment, kind_of, method_call_tail, method_named, text,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferArrayIterableMethods;

/// Whether a slot of the pattern binds nothing the loop reads: a hole, or a name never read.
fn is_unused(source: &Source, element: Option<&BindingPattern>) -> bool {
    match element {
        None => true,
        Some(BindingPattern::BindingIdentifier(identifier)) => {
            identifier.symbol_id.get().is_some_and(|symbol| {
                source
                    .semantic
                    .scoping()
                    .get_resolved_reference_ids(symbol)
                    .is_empty()
            })
        }
        Some(_) => false,
    }
}

/// The pattern's new text, a space added where a keyword touches it.
fn pattern_text(source: &Source, pattern: &ArrayPattern, binding: &str) -> String {
    let before = source.text[..pattern.span.start as usize]
        .chars()
        .last()
        .is_some_and(|character| !character.is_whitespace());
    let after = source.text[pattern.span.end as usize..]
        .chars()
        .next()
        .is_some_and(|character| !character.is_whitespace());
    format!(
        "{}{binding}{}",
        if before { " " } else { "" },
        if after { " " } else { "" }
    )
}

impl Rule for PreferArrayIterableMethods {
    fn name(&self) -> &'static str {
        "unicorn/prefer-array-iterable-methods"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::ForOfStatement(statement) = node.kind() else {
            return;
        };
        let ForStatementLeft::VariableDeclaration(declaration) = &statement.left else {
            return;
        };
        let [declarator] = declaration.declarations.as_slice() else {
            return;
        };
        let BindingPattern::ArrayPattern(pattern) = &declarator.id else {
            return;
        };
        let slots = pattern.elements.len() + usize::from(pattern.rest.is_some());
        let lexical = matches!(
            declaration.kind,
            VariableDeclarationKind::Const | VariableDeclarationKind::Let
        );
        if statement.r#await || !lexical || !(1..=2).contains(&slots) {
            return;
        }
        let Some(entries) = method_named(&statement.right, &["entries"])
            .filter(|method| method.is_plain() && method.call.arguments.is_empty())
        else {
            return;
        };
        if kind_of(source, entries.object) != Some(Kind::Array) {
            return;
        }
        let index = pattern.elements.first().and_then(Option::as_ref);
        let value = pattern.elements.get(1).and_then(Option::as_ref);
        let (index_unused, value_unused) = if pattern.rest.is_some() && pattern.elements.len() == 1
        {
            (is_unused(source, index), false)
        } else {
            (is_unused(source, index), is_unused(source, value))
        };
        if index_unused == value_unused {
            return;
        }
        let comments = has_comment(source, pattern.span) || has_comment(source, entries.call.span);
        if index_unused {
            let Some(value) =
                value.filter(|value| !matches!(value, BindingPattern::AssignmentPattern(_)))
            else {
                return;
            };
            let message = "Prefer iterating the array directly over `Array#entries()` when the index is unused.";
            let fix = (!comments)
                .then(|| {
                    combined_fix(
                        source,
                        vec![
                            Edit {
                                span: pattern.span,
                                text: pattern_text(source, pattern, text(source, value.span())),
                            },
                            Edit {
                                span: method_call_tail(source, &entries),
                                text: String::new(),
                            },
                        ],
                    )
                })
                .flatten();
            findings.push(match fix {
                Some(fix) => {
                    source.fixed(self.name(), entries.property.start, message.to_owned(), fix)
                }
                None => source.finding(self.name(), entries.property.start, message.to_owned()),
            });
            return;
        }
        let Some(BindingPattern::BindingIdentifier(index)) = index else {
            return;
        };
        let message = "Prefer `Array#keys()` over `Array#entries()` when the value is unused.";
        let fix = (!comments)
            .then(|| {
                combined_fix(
                    source,
                    vec![
                        Edit {
                            span: pattern.span,
                            text: pattern_text(source, pattern, index.name.as_str()),
                        },
                        Edit {
                            span: entries.property,
                            text: "keys".to_owned(),
                        },
                    ],
                )
            })
            .flatten();
        findings.push(match fix {
            Some(fix) => source.fixed(self.name(), entries.property.start, message.to_owned(), fix),
            None => source.finding(self.name(), entries.property.start, message.to_owned()),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::PreferArrayIterableMethods;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The declarations the cases read.
    const SETUP: &str = "const array = [1, 2];\n";

    /// The messages on a piece of code after the declarations.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferArrayIterableMethods,
            "playground/a.ts",
            &format!("{SETUP}{code}"),
        ))
    }

    /// The code after the fixes, the declarations left out.
    fn fix(code: &str) -> String {
        let all = format!("{SETUP}{code}");
        fixed(
            &all,
            &lint(&PreferArrayIterableMethods, "playground/a.ts", &all),
        )[SETUP.len()..]
            .to_owned()
    }

    #[test]
    fn reports_and_fixes_the_documented_loops() {
        assert_eq!(
            check("for (const [, value] of array.entries()) {\n\tfoo(value);\n}"),
            [
                "Prefer iterating the array directly over `Array#entries()` when the index is unused."
            ]
        );
        assert_eq!(
            fix("for (const [, value] of array.entries()) {\n\tfoo(value);\n}"),
            "for (const value of array) {\n\tfoo(value);\n}"
        );
        assert_eq!(
            check("for (const [index] of array.entries()) {\n\tfoo(index);\n}"),
            ["Prefer `Array#keys()` over `Array#entries()` when the value is unused."]
        );
        assert_eq!(
            fix("for (const [index] of array.entries()) {\n\tfoo(index);\n}"),
            "for (const index of array.keys()) {\n\tfoo(index);\n}"
        );
        assert_eq!(
            fix("for (const[i, v]of array.entries()) foo(i);"),
            "for (const i of array.keys()) foo(i);"
        );
        assert_eq!(
            fix("for (let [i, { a }] of array.entries()) foo(a);"),
            "for (let { a } of array) foo(a);"
        );
        assert_eq!(
            fix("for (const [, v] of [1, 2].entries()) foo(v);"),
            "for (const v of [1, 2]) foo(v);"
        );
    }

    #[test]
    fn accepts_loops_that_use_both_or_other_receivers() {
        for code in [
            "for (const [index, value] of array.entries()) {\n\tfoo(index, value);\n}",
            "for (const [index, value] of array.entries()) {}",
            "for (const [, , extra] of array.entries()) {}",
            "for (const [, value = 1] of array.entries()) { foo(value); }",
            "for (const [, ...rest] of array.entries()) { foo(rest); }",
            "for (const [i, v] of map.entries()) { foo(v); }",
            "for (const [i, v] of array.entries(1)) { foo(v); }",
            "for (const [i, v] of array?.entries()) { foo(v); }",
            "for (var [i, v] of array.entries()) { foo(v); }",
            "async function f() { for await (const [i, v] of array.entries()) { foo(v); } }",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
