//! `unicorn/no-unnecessary-string-trim`: `trim()` before `startsWith()` trims only the start,
//! `trimStart()`, and before `endsWith()` only the end, `trimEnd()`, when the searched text
//! cannot see the other side (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, ModuleExportName};
use oxc_semantic::AstNode;

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::known_values::{is_known_non_string, value_text};
use crate::rules::unicorn::syntax::{
    has_optional_chain, method_call, static_value, symbol_of, unwrap,
};
use crate::source::Source;

/// The rule.
pub struct NoUnnecessaryStringTrim;

/// Whether a receiver is `z.string()` of a `z` imported from `zod`, whose `trim()` is Zod's.
fn is_zod_string(source: &Source, expression: &Expression) -> bool {
    let Expression::CallExpression(call) = unwrap(expression) else {
        return false;
    };
    let Some(method) = method_call(call) else {
        return false;
    };
    if method.name != "string" || !call.arguments.is_empty() || !method.is_plain() {
        return false;
    }
    let Expression::Identifier(object) = method.object else {
        return false;
    };
    let Some(symbol) = symbol_of(source, object) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    let from_zod = nodes.ancestors(declaration).any(|node| {
        matches!(node.kind(), AstKind::ImportDeclaration(import) if import.source.value == "zod" && import.import_kind.is_value())
    });
    from_zod
        && match nodes.kind(declaration) {
            AstKind::ImportNamespaceSpecifier(_) => true,
            AstKind::ImportSpecifier(specifier) => {
                specifier.import_kind.is_value()
                    && match &specifier.imported {
                        ModuleExportName::IdentifierName(name) => name.name == "z",
                        ModuleExportName::IdentifierReference(name) => name.name == "z",
                        ModuleExportName::StringLiteral(name) => name.value == "z",
                    }
            }
            _ => false,
        }
}

/// Whether a value is written, or bound by a `const`, as something other than a string.
fn is_static_non_string(source: &Source, expression: &Expression) -> bool {
    let unwrapped = unwrap(expression);
    let value =
        crate::rules::unicorn::syntax::const_init(source, unwrapped).map_or(unwrapped, unwrap);
    matches!(
        value,
        Expression::ArrayExpression(_)
            | Expression::ArrowFunctionExpression(_)
            | Expression::ClassExpression(_)
            | Expression::FunctionExpression(_)
            | Expression::ObjectExpression(_)
    ) || static_value(source, value)
        .is_some_and(|value| !matches!(value, crate::rules::unicorn::syntax::Value::Text(_)))
}

impl Rule for NoUnnecessaryStringTrim {
    fn name(&self) -> &'static str {
        "unicorn/no-unnecessary-string-trim"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let Some(outer) = method_call(call) else {
            return;
        };
        if !matches!(outer.name, "startsWith" | "endsWith")
            || call.arguments.len() > 1
            || !outer.is_plain()
        {
            return;
        }
        let Expression::CallExpression(trim_call) = outer.object else {
            return;
        };
        let Some(trim) = method_call(trim_call) else {
            return;
        };
        if trim.name != "trim" || !trim_call.arguments.is_empty() || !trim.is_plain() {
            return;
        }
        if has_optional_chain(&call.callee) {
            return;
        }
        if is_static_non_string(source, trim.object)
            || is_known_non_string(source, trim.object)
            || is_zod_string(source, trim.object)
        {
            return;
        }
        if let Some(argument) = call.arguments.first() {
            let Some(search) = argument
                .as_expression()
                .and_then(|expression| static_value(source, expression))
            else {
                return;
            };
            let search = value_text(&search);
            let safe = if outer.name == "startsWith" {
                search == search.trim_end()
            } else {
                search == search.trim_start()
            };
            if !safe {
                return;
            }
        }
        let replacement = if outer.name == "startsWith" {
            "trimStart"
        } else {
            "trimEnd"
        };
        findings.push(source.fixed(
            self.name(),
            trim.property.start,
            format!(
                "Prefer `String#{replacement}()` before `String#{}()`.",
                outer.name
            ),
            Fix {
                start: trim.property.start,
                end: trim.property.end,
                text: replacement.to_owned(),
            },
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::NoUnnecessaryStringTrim;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&NoUnnecessaryStringTrim, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&NoUnnecessaryStringTrim, "playground/a.ts", code),
        )
    }

    #[test]
    fn fixes_the_documented_trims() {
        let start = "const result = value.trim().startsWith('-');";
        assert_eq!(
            check(start),
            ["Prefer `String#trimStart()` before `String#startsWith()`."]
        );
        assert_eq!(
            fix(start),
            "const result = value.trimStart().startsWith('-');"
        );
        let end = "const result = value.trim().endsWith('-');";
        assert_eq!(
            check(end),
            ["Prefer `String#trimEnd()` before `String#endsWith()`."]
        );
        assert_eq!(fix(end), "const result = value.trimEnd().endsWith('-');");
        assert_eq!(
            fix("const a = value.trim().startsWith();"),
            "const a = value.trimStart().startsWith();"
        );
        assert_eq!(
            fix("const a = value.trim().endsWith(1);"),
            "const a = value.trimEnd().endsWith(1);"
        );
    }

    #[test]
    fn accepts_the_documented_and_unknown_cases() {
        for code in [
            "const result = value.trim().startsWith('-', 1);",
            "const result = value.trim().startsWith('foo ');",
            "const result = value.trim().endsWith(' foo');",
            "const result = value.trim().startsWith(prefix);",
            "const result = value?.trim().startsWith('-');",
            "const result = value.trim?.().startsWith('-');",
            "const result = value.trim(x).startsWith('-');",
            "const result = [1].trim().startsWith('-');",
            "function f(value: number[]) { return value.trim().startsWith('-'); }",
            "import {z} from 'zod';\nconst a = z.string().trim().startsWith('-');",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
