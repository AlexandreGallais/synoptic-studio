//! `unicorn/prefer-single-object-destructuring`: two adjacent destructurings of the same local
//! `const` are one, and a `const` only destructured by the next declaration is destructured
//! from its initializer directly (`eslint-plugin-unicorn` 78.0.0). Autofixed, as in `ESLint`,
//! unless a comment sits in the range.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BindingPattern, Expression, ObjectPattern, Statement, VariableDeclaration,
    VariableDeclarationKind, VariableDeclarator,
};
use oxc_semantic::AstNode;
use oxc_span::{GetSpan, Span};

use crate::finding::{Finding, Fix};
use crate::rules::Rule;
use crate::rules::unicorn::syntax::{has_comment, parenthesized_text, symbol_of, text, unwrap};
use crate::source::Source;

/// The rule.
pub struct PreferSingleObjectDestructuring;

/// A declaration of one object destructuring, plain, from a local `const`.
struct Supported<'b, 'a> {
    /// The declaration.
    declaration: &'b VariableDeclaration<'a>,
    /// Its pattern.
    pattern: &'b ObjectPattern<'a>,
    /// The name destructured.
    source: &'b oxc_ast::ast::IdentifierReference<'a>,
}

/// The declaration a statement is, when it declares one name or pattern, neither `declare` nor
/// `var`.
fn single<'b, 'a>(
    statement: &'b Statement<'a>,
) -> Option<(&'b VariableDeclaration<'a>, &'b VariableDeclarator<'a>)> {
    let Statement::VariableDeclaration(declaration) = statement else {
        return None;
    };
    match declaration.declarations.as_slice() {
        [declarator] if !declaration.declare => Some((declaration, declarator)),
        _ => None,
    }
}

/// Whether a name reads a `const` the file declares once, not with `declare`.
fn is_local_const(source: &Source, identifier: &oxc_ast::ast::IdentifierReference) -> bool {
    let Some(symbol) = symbol_of(source, identifier) else {
        return false;
    };
    let scoping = source.semantic.scoping();
    if scoping.symbol_redeclarations(symbol).len() > 1 {
        return false;
    }
    let nodes = source.semantic.nodes();
    let declaration = scoping.symbol_declaration(symbol);
    matches!(nodes.kind(declaration), AstKind::VariableDeclarator(_))
        && matches!(nodes.parent_kind(declaration), AstKind::VariableDeclaration(statement)
            if statement.kind == VariableDeclarationKind::Const && !statement.declare)
}

/// The declaration a statement is, when it destructures plain names from a local `const`.
fn supported<'b, 'a>(source: &Source, statement: &'b Statement<'a>) -> Option<Supported<'b, 'a>> {
    let (declaration, declarator) = single(statement)?;
    if !matches!(
        declaration.kind,
        VariableDeclarationKind::Const | VariableDeclarationKind::Let
    ) || declarator.type_annotation.is_some()
    {
        return None;
    }
    let BindingPattern::ObjectPattern(pattern) = &declarator.id else {
        return None;
    };
    let simple = pattern.rest.is_none()
        && !pattern.properties.is_empty()
        && pattern.properties.iter().all(|property| {
            !property.computed && matches!(property.value, BindingPattern::BindingIdentifier(_))
        });
    let Some(Expression::Identifier(init)) = &declarator.init else {
        return None;
    };
    (simple && is_local_const(source, init)).then_some(Supported {
        declaration,
        pattern,
        source: init,
    })
}

/// The keyword of a declaration.
fn keyword(kind: VariableDeclarationKind) -> &'static str {
    match kind {
        VariableDeclarationKind::Let => "let",
        _ => "const",
    }
}

/// The texts of a pattern's properties.
fn property_texts<'a>(source: &Source<'_, 'a>, pattern: &ObjectPattern) -> Vec<&'a str> {
    pattern
        .properties
        .iter()
        .map(|property| text(source, property.span))
        .collect()
}

impl PreferSingleObjectDestructuring {
    /// The finding of a `const` only destructured by the next declaration.
    fn inline(&self, source: &Source, first: &Statement, second: &Statement) -> Option<Finding> {
        let (declaration, declarator) = single(first)?;
        if declaration.kind != VariableDeclarationKind::Const
            || declarator.type_annotation.is_some()
        {
            return None;
        }
        let BindingPattern::BindingIdentifier(binding) = &declarator.id else {
            return None;
        };
        let init = declarator.init.as_ref()?;
        let next = supported(source, second)?;
        let scoping = source.semantic.scoping();
        let symbol = binding.symbol_id();
        if symbol_of(source, next.source) != Some(symbol) {
            return None;
        }
        let references = scoping.get_resolved_reference_ids(symbol);
        let only_there = references.iter().all(|id| {
            let node = scoping.get_reference(*id).node_id();
            source.semantic.nodes().kind(node).span() == next.source.span
        });
        let inlinable = matches!(
            unwrap(init),
            Expression::Identifier(_)
                | Expression::StaticMemberExpression(_)
                | Expression::ComputedMemberExpression(_)
                | Expression::PrivateFieldExpression(_)
                | Expression::ChainExpression(_)
                | Expression::CallExpression(_)
                | Expression::NewExpression(_)
                | Expression::TaggedTemplateExpression(_)
                | Expression::AwaitExpression(_)
        );
        if !only_there || !inlinable {
            return None;
        }
        let range = Span::new(first.span().start, second.span().end);
        let message = format!(
            "Prefer destructuring directly from the initializer of `{}`.",
            binding.name
        );
        let replacement = format!(
            "{} {} = {};",
            keyword(next.declaration.kind),
            text(source, next.pattern.span),
            parenthesized_text(source, init)
        );
        Some(self.problem(source, binding.span.start, message, range, replacement))
    }

    /// The finding of two destructurings of the same `const`.
    fn merge(&self, source: &Source, first: &Statement, second: &Statement) -> Option<Finding> {
        let first = supported(source, first)?;
        let second = supported(source, second)?;
        if first.declaration.kind != second.declaration.kind
            || first.source.name != second.source.name
        {
            return None;
        }
        let keys: Vec<Option<std::borrow::Cow<str>>> = first
            .pattern
            .properties
            .iter()
            .map(|property| property.key.static_name())
            .collect();
        let duplicate = second
            .pattern
            .properties
            .iter()
            .any(|property| keys.contains(&property.key.static_name()));
        if duplicate {
            return None;
        }
        let range = Span::new(first.declaration.span.start, second.declaration.span.end);
        let replacement = format!(
            "{} {{{}, {}}} = {};",
            keyword(first.declaration.kind),
            property_texts(source, first.pattern).join(", "),
            property_texts(source, second.pattern).join(", "),
            first.source.name
        );
        Some(self.problem(
            source,
            second.declaration.span.start,
            format!(
                "Prefer a single object destructuring declaration from `{}`.",
                first.source.name
            ),
            range,
            replacement,
        ))
    }

    /// A finding, fixed unless a comment would be lost.
    fn problem(
        &self,
        source: &Source,
        at: u32,
        message: String,
        range: Span,
        replacement: String,
    ) -> Finding {
        if has_comment(source, range) {
            return source.finding(self.name(), at, message);
        }
        source.fixed(
            self.name(),
            at,
            message,
            Fix {
                start: range.start,
                end: range.end,
                text: replacement,
            },
        )
    }
}

impl Rule for PreferSingleObjectDestructuring {
    fn name(&self) -> &'static str {
        "unicorn/prefer-single-object-destructuring"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let list: &[Statement] = match node.kind() {
            AstKind::Program(program) => &program.body,
            AstKind::BlockStatement(block) => &block.body,
            AstKind::FunctionBody(body) => &body.statements,
            AstKind::StaticBlock(block) => &block.body,
            AstKind::SwitchCase(case) => &case.consequent,
            _ => return,
        };
        for pair in list.windows(2) {
            if let Some(finding) = self
                .inline(source, &pair[0], &pair[1])
                .or_else(|| self.merge(source, &pair[0], &pair[1]))
            {
                findings.push(finding);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferSingleObjectDestructuring;
    use crate::testing::{NONE, fixed, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(
            &PreferSingleObjectDestructuring,
            "playground/a.ts",
            code,
        ))
    }

    /// The code once fixed.
    fn fix(code: &str) -> String {
        fixed(
            code,
            &lint(&PreferSingleObjectDestructuring, "playground/a.ts", code),
        )
    }

    #[test]
    fn merges_the_documented_destructurings() {
        let code = "const foo = {};\nconst {bar} = foo;\nconst {baz} = foo;";
        assert_eq!(
            check(code),
            ["Prefer a single object destructuring declaration from `foo`."]
        );
        assert_eq!(fix(code), "const foo = {};\nconst {bar, baz} = foo;");
        let inline = "const foo = getFoo();\nconst {bar, baz} = foo;";
        assert_eq!(
            check(inline),
            ["Prefer destructuring directly from the initializer of `foo`."]
        );
        assert_eq!(fix(inline), "const {bar, baz} = getFoo();");
        assert_eq!(
            fix("async function f() { const a = await g(); let {b} = a; }"),
            "async function f() { let {b} = await g(); }"
        );
    }

    #[test]
    fn reports_without_a_fix_over_a_comment() {
        let code = "const foo = {};\nconst {bar} = foo; // first\nconst {baz} = foo;";
        assert_eq!(
            check(code),
            ["Prefer a single object destructuring declaration from `foo`."]
        );
        assert_eq!(fix(code), code);
    }

    #[test]
    fn accepts_the_documented_and_unsupported_forms() {
        for code in [
            "let foo = {};\nconst {bar} = foo;\nconst {baz} = foo;",
            "import foo from 'foo';\nconst {bar} = foo;\nconst {baz} = foo;",
            "const {bar} = foo;\nconsole.log(bar);\nconst {baz} = foo;",
            "const foo = {};\nconst {bar} = foo;\nconst {bar: b} = foo;",
            "const foo = {};\nconst {bar} = foo;\nlet {baz} = foo;",
            "const foo = {};\nconst {bar = 1} = foo;\nconst {baz} = foo;",
            "const foo = {};\nconst {bar, ...rest} = foo;\nconst {baz} = foo;",
            "const foo = getFoo();\nconst {bar} = foo;\nuse(foo);",
            "const foo: Foo = getFoo();\nconst {bar} = foo;",
            "const foo = () => 1;\nconst {name} = foo;",
            "const foo = getFoo();\nconst {bar}: T = foo;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
