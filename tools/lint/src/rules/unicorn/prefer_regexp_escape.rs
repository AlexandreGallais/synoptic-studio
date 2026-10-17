//! `unicorn/prefer-regexp-escape`: a string is escaped for a regular expression with
//! `RegExp.escape()`, not a hand-written `replace(/[.*+?^${}()|[\]\\]/g, '\\$&')` nor
//! `escape-string-regexp` or Lodash's `escapeRegExp` (`eslint-plugin-unicorn` 78.0.0). The
//! `replace()` snippet is autofixed, as in `ESLint`; a package's call is reported only, its
//! replacement being a suggestion.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, BindingPattern, CallExpression, Expression, ImportDeclarationSpecifier,
    ModuleExportName, PropertyKey, RegExpFlags,
};
use oxc_semantic::SymbolId;
use oxc_span::GetSpan;

use super::known_values::is_known_non_string;
use super::syntax::{Value, has_comment, method_call, parenthesized_text, static_value, symbol_of};
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferRegexpEscape;

/// What the rule reports.
const MESSAGE: &str =
    "Prefer `RegExp.escape()` for escaping strings to use in regular expressions.";

/// The packages whose default export escapes a string for a regular expression.
const ESCAPE_PACKAGES: [&str; 3] = [
    "escape-string-regexp",
    "lodash.escaperegexp",
    "lodash/escapeRegExp",
];

/// The packages of Lodash.
const LODASH_PACKAGES: [&str; 2] = ["lodash", "lodash-es"];

/// The characters `RegExp.escape()` escapes that a hand-written class lists.
const ESCAPE_CHARACTERS: &str = "\\^$.*+?()[]{}|/-";

/// The characters of a character class listing only characters to escape, `[…]`, not negated.
fn class_characters(pattern: &str) -> Option<Vec<char>> {
    let inner = pattern.strip_prefix('[')?.strip_suffix(']')?;
    if pattern.starts_with("[^") {
        return None;
    }
    let characters: Vec<char> = inner.chars().collect();
    let mut found: Vec<char> = Vec::new();
    let mut index = 0;
    while index < characters.len() {
        let mut character = characters[index];
        let mut escaped = false;
        if character == '\\' {
            index += 1;
            character = *characters.get(index)?;
            escaped = true;
        }
        let last = characters.len() - 1;
        if character == '-' && !escaped && index != 0 && index != last {
            return None;
        }
        if character == ']' && !escaped && index != 0 {
            return None;
        }
        if !ESCAPE_CHARACTERS.contains(character) {
            return None;
        }
        if !found.contains(&character) {
            found.push(character);
        }
        index += 1;
    }
    Some(found)
}

/// Whether a call is `text.replace(/[…]/g, '\\$&')` with a class of eight or more such characters.
fn is_escape_replace(call: &CallExpression) -> bool {
    let Some(method) = method_call(call) else {
        return false;
    };
    if method.name != "replace" || !method.is_plain() || call.arguments.len() != 2 {
        return false;
    }
    let pattern = matches!(&call.arguments[0], Argument::RegExpLiteral(literal)
        if literal.regex.flags.contains(RegExpFlags::G)
            && !literal.regex.flags.contains(RegExpFlags::Y)
            && class_characters(literal.regex.pattern.text.as_str()).is_some_and(|found| found.len() >= 8));
    let replacement =
        matches!(&call.arguments[1], Argument::StringLiteral(literal) if literal.value == "\\$&");
    pattern && replacement
}

/// The bindings an import or a `require()` makes of escape functions and of Lodash.
fn tracked_bindings(source: &Source) -> (Vec<SymbolId>, Vec<SymbolId>) {
    let (mut functions, mut lodash) = (Vec::new(), Vec::new());
    for node in source.semantic.nodes().iter() {
        match node.kind() {
            AstKind::ImportDeclaration(import) => {
                let package = import.source.value.as_str();
                for specifier in import.specifiers.iter().flatten() {
                    match specifier {
                        ImportDeclarationSpecifier::ImportDefaultSpecifier(default) => {
                            if ESCAPE_PACKAGES.contains(&package) {
                                functions.push(default.local.symbol_id());
                            } else if LODASH_PACKAGES.contains(&package) {
                                lodash.push(default.local.symbol_id());
                            }
                        }
                        ImportDeclarationSpecifier::ImportNamespaceSpecifier(namespace) => {
                            if LODASH_PACKAGES.contains(&package) {
                                lodash.push(namespace.local.symbol_id());
                            }
                        }
                        ImportDeclarationSpecifier::ImportSpecifier(named) => {
                            let imported = match &named.imported {
                                ModuleExportName::IdentifierName(name) => Some(name.name.as_str()),
                                ModuleExportName::IdentifierReference(name) => {
                                    Some(name.name.as_str())
                                }
                                ModuleExportName::StringLiteral(_) => None,
                            };
                            if (ESCAPE_PACKAGES.contains(&package) && imported == Some("default"))
                                || (LODASH_PACKAGES.contains(&package)
                                    && imported == Some("escapeRegExp"))
                            {
                                functions.push(named.local.symbol_id());
                            }
                        }
                    }
                }
            }
            AstKind::VariableDeclarator(declarator) => {
                let Some(Expression::CallExpression(call)) = &declarator.init else {
                    continue;
                };
                let required = match (&call.callee, call.arguments.as_slice()) {
                    (Expression::Identifier(callee), [Argument::StringLiteral(name)])
                        if callee.name == "require" =>
                    {
                        name.value.as_str()
                    }
                    _ => continue,
                };
                match &declarator.id {
                    BindingPattern::BindingIdentifier(binding) => {
                        if ESCAPE_PACKAGES.contains(&required) {
                            functions.push(binding.symbol_id());
                        } else if LODASH_PACKAGES.contains(&required) {
                            lodash.push(binding.symbol_id());
                        }
                    }
                    BindingPattern::ObjectPattern(pattern)
                        if LODASH_PACKAGES.contains(&required) =>
                    {
                        for property in &pattern.properties {
                            let named = matches!(&property.key, PropertyKey::StaticIdentifier(key) if key.name == "escapeRegExp");
                            if let (true, BindingPattern::BindingIdentifier(binding)) =
                                (named, &property.value)
                            {
                                functions.push(binding.symbol_id());
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    (functions, lodash)
}

/// Whether an argument is a constant other than a string.
fn is_static_non_string(source: &Source, expression: &Expression) -> bool {
    static_value(source, expression).is_some_and(|value| !matches!(value, Value::Text(_)))
}

impl Rule for PreferRegexpEscape {
    fn name(&self) -> &'static str {
        "unicorn/prefer-regexp-escape"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let (functions, lodash) = tracked_bindings(source);
        for node in source.semantic.nodes().iter() {
            let AstKind::CallExpression(call) = node.kind() else {
                continue;
            };
            self.check_call(source, call, &functions, &lodash, findings);
        }
    }
}

impl PreferRegexpEscape {
    /// Reports a hand-written escape, fixed, or a package's escape call.
    fn check_call(
        &self,
        source: &Source,
        call: &CallExpression,
        functions: &[SymbolId],
        lodash: &[SymbolId],
        findings: &mut Vec<Finding>,
    ) {
        if is_escape_replace(call) {
            let Some(method) = method_call(call) else {
                return;
            };
            if is_known_non_string(source, method.object) {
                return;
            }
            if matches!(method.object, Expression::Super(_)) || has_comment(source, call.span) {
                findings.push(source.finding(self.name(), call.span.start, MESSAGE.to_owned()));
                return;
            }
            findings.push(source.fixed(
                self.name(),
                call.span.start,
                MESSAGE.to_owned(),
                Fix {
                    start: call.span.start,
                    end: call.span.end,
                    text: format!(
                        "RegExp.escape({})",
                        parenthesized_text(source, method.object)
                    ),
                },
            ));
            return;
        }
        let tracked_function = !call.optional
            && call.arguments.len() == 1
            && matches!(&call.callee, Expression::Identifier(callee)
                if symbol_of(source, callee).is_some_and(|symbol| functions.contains(&symbol)));
        let lodash_call = call.arguments.len() == 1
            && method_call(call).is_some_and(|method| {
                method.name == "escapeRegExp"
                    && method.is_plain()
                    && matches!(method.object, Expression::Identifier(object)
                    if match symbol_of(source, object) {
                        Some(symbol) => lodash.contains(&symbol),
                        None => matches!(object.name.as_str(), "_" | "lodash"),
                    })
            });
        if !(tracked_function || lodash_call) {
            return;
        }
        let Some(argument) = call.arguments[0].as_expression() else {
            return;
        };
        let mutable_member = matches!(
            argument,
            Expression::StaticMemberExpression(_) | Expression::ComputedMemberExpression(_)
        );
        if (!mutable_member && is_known_non_string(source, argument))
            || is_static_non_string(source, argument)
        {
            return;
        }
        findings.push(source.finding(self.name(), call.span().start, MESSAGE.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::{MESSAGE, PreferRegexpEscape};
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&PreferRegexpEscape, "playground/a.ts", code))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&PreferRegexpEscape, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_hand_written_escapes_and_packages() {
        let classic = "const escaped = string.replace(/[.*+?^${}()|[\\]\\\\]/g, '\\\\$&');";
        assert_eq!(check(classic), [MESSAGE]);
        assert_eq!(fix(classic), "const escaped = RegExp.escape(string);");
        assert_eq!(
            fix("const escaped = (a + b).replace(/[|\\\\{}()[\\]^$+*?.]/g, '\\\\$&');"),
            "const escaped = RegExp.escape((a + b));"
        );
        assert_eq!(
            check(
                "import escapeStringRegexp from 'escape-string-regexp';\n\nconst escaped = escapeStringRegexp(string);"
            ),
            [MESSAGE]
        );
        assert_eq!(check("const escaped = _.escapeRegExp(string);"), [MESSAGE]);
        assert_eq!(
            check("const {escapeRegExp: e} = require('lodash');\nconst a = e(s);"),
            [MESSAGE]
        );
        assert_eq!(
            check("import {escapeRegExp} from 'lodash-es';\nconst a = escapeRegExp(s);"),
            [MESSAGE]
        );
    }

    #[test]
    fn accepts_the_documented_valid_forms() {
        for code in [
            "const escaped = RegExp.escape(string);",
            "const escaped = string.replace(/[.*+?^$]/g, '\\\\$&');",
            "const escaped = string.replace(/[.*+?^${}()|[\\]\\\\]/gy, '\\\\$&');",
            "const escaped = string.replace(/[.*+?^${}()|[\\]\\\\]/g, '$&');",
            "const escaped = string.replace(/[^.*+?^${}()|[\\]\\\\]/g, '\\\\$&');",
            "const escaped = [1].replace(/[.*+?^${}()|[\\]\\\\]/g, '\\\\$&');",
            "import escapeStringRegexp from 'escape-string-regexp';\nconst a = escapeStringRegexp(1);",
            "const a = escapeStringRegexp(s);",
            "const _ = other; const a = _.escapeRegExp(s);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
