//! `functional/readonly-type`: a type literal is made read-only with the `readonly` keyword on
//! its members, never by `Readonly<…>` around it (`eslint-plugin-functional` 10.0.1,
//! `docs/rules/readonly-type.md`, the project's option `keyword`). As the plugin, a literal is
//! reported when the nearest type reference around it, at any depth, is `Readonly`, at that
//! name. Autofixed as the plugin fixes it: the wrapper removed, `readonly` written before each
//! property and index signature of the literal that lacks it.

use oxc_ast::AstKind;
use oxc_ast::ast::{TSSignature, TSTypeLiteral, TSTypeName, TSTypeReference};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::expressions::text;
use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ReadonlyType;

/// The plugin's message.
const MESSAGE: &str =
    "Readonly type using 'Readonly<T>' is forbidden. Use 'readonly' keyword instead.";

/// The nearest type reference around a node, when it is `Readonly`.
fn readonly_around<'a>(
    node: &AstNode<'a>,
    source: &Source<'_, 'a>,
) -> Option<&'a TSTypeReference<'a>> {
    let reference =
        source
            .semantic
            .nodes()
            .ancestors(node.id())
            .find_map(|ancestor| match ancestor.kind() {
                AstKind::TSTypeReference(reference) => Some(reference),
                _ => None,
            })?;

    matches!(&reference.type_name, TSTypeName::IdentifierReference(name) if name.name == "Readonly")
        .then_some(reference)
}

/// The text of the reference without its wrapper, `readonly` written before each member of the
/// literal that may carry it and lacks it.
fn unwrapped(
    source: &Source,
    reference: &TSTypeReference,
    literal: &TSTypeLiteral,
) -> Option<String> {
    let whole = text(source, reference.span);
    let opened = whole
        .strip_prefix("Readonly")?
        .trim_start()
        .strip_prefix('<')?;
    let inner = opened.trim_end().strip_suffix('>')?.trim_end();
    let start = reference.span.start as usize + (whole.len() - opened.len());
    let mut result = String::with_capacity(inner.len() + 9 * literal.members.len());
    let mut from = start;

    for member in &literal.members {
        let (at, readonly) = match member {
            TSSignature::TSPropertySignature(each) => (each.span.start, each.readonly),
            TSSignature::TSIndexSignature(each) => (each.span.start, each.readonly),
            _ => continue,
        };
        let at = at as usize;
        if readonly || at < start || at > start + inner.len() {
            continue;
        }
        result.push_str(source.text.get(from..at)?);
        result.push_str("readonly ");
        from = at;
    }
    result.push_str(source.text.get(from..start + inner.len())?);
    Some(result)
}

impl Rule for ReadonlyType {
    fn name(&self) -> &'static str {
        "functional/readonly-type"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TSTypeLiteral(literal) = node.kind() else {
            return;
        };
        let Some(reference) = readonly_around(node, source) else {
            return;
        };
        let at = reference.type_name.span().start;
        let finding = match unwrapped(source, reference, literal) {
            Some(fixed) => source.fixed(
                self.name(),
                at,
                MESSAGE.to_owned(),
                Fix {
                    start: reference.span.start,
                    end: reference.span.end,
                    text: fixed,
                },
            ),
            None => source.finding(self.name(), at, MESSAGE.to_owned()),
        };
        findings.push(finding);
    }
}

#[cfg(test)]
mod tests {
    use super::ReadonlyType;
    use crate::testing::{fixed, lint};

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&ReadonlyType, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    /// The code with the findings' fixes applied.
    fn fix(code: &str) -> String {
        fixed(code, &lint(&ReadonlyType, "src/a/b.ts", code))
    }

    #[test]
    fn reports_a_literal_wrapped_in_readonly_and_writes_the_keyword() {
        // The documentation's case.
        let code = "type Foo = Readonly<{\n  bar: string;\n  baz: number;\n}>;";

        assert_eq!(places(code), [(1, 12)]);
        assert_eq!(
            fix(code),
            "type Foo = {\n  readonly bar: string;\n  readonly baz: number;\n};"
        );
    }

    #[test]
    fn keeps_what_is_already_readonly_and_reads_any_depth() {
        assert_eq!(
            fix("type A = Readonly<{ readonly a: 1; [k: string]: 1 }>;"),
            "type A = { readonly a: 1; readonly [k: string]: 1 };"
        );
        assert_eq!(
            fix("type B = Readonly < { b: 1 } >;"),
            // The space after `<` stays, as the plugin's pattern leaves it.
            "type B =  { readonly b: 1 };"
        );
        // A literal inside a union inside `Readonly` is reported; one inside `Array` is not.
        assert_eq!(places("type C = Readonly<{ c: 1 } | null>;"), [(1, 10)]);
        assert_eq!(
            fix("type C = Readonly<{ c: 1 } | null>;"),
            "type C = { readonly c: 1 } | null;"
        );
        assert_eq!(
            places("type D = Readonly<Array<{ d: 1 }>>;"),
            Vec::<(usize, usize)>::new()
        );
    }

    #[test]
    fn lets_the_keyword_and_other_wrappers_through() {
        // The documentation's correct cases.
        for code in [
            "type Foo = {\n  readonly bar: string;\n  readonly baz: number;\n};\n\ntype Foo2 = {\n  readonly bar: string;\n  baz: number;\n};",
            "type E = Readonly<Point>;",
            "type F = Partial<{ f: 1 }>;",
            "type G = readonly { g: 1 }[];",
        ] {
            assert_eq!(places(code), Vec::<(usize, usize)>::new(), "{code}");
        }
    }
}
