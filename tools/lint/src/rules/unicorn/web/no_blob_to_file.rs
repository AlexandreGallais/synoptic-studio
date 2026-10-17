//! `unicorn/no-blob-to-file`: a `Blob` wrapped in `new File([blob], name)` only to be given to
//! `URL.createObjectURL()` or a `FormData`'s `append()` or `set()` is a `Blob` already
//! (`eslint-plugin-unicorn` 78.0.0, `docs/rules/no-blob-to-file.md`). Only that narrow shape: a
//! lone `const` of a block, used once, of a `const` blob made before it. Reported only: `ESLint`
//! only suggests.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, ArrayExpressionElement, BindingPattern, Expression, NewExpression};
use oxc_semantic::{AstNode, NodeId, SymbolId};
use oxc_span::GetSpan;

use crate::expressions::const_value;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::dom::nodes::static_string;
use crate::rules::unicorn::syntax::{const_init, method_call, symbol_of};
use crate::source::Source;

/// The rule.
pub struct NoBlobToFile;

/// Whether a `new` calls a global constructor with a number of arguments in a range, none spread.
fn is_new_global(
    source: &Source,
    new: &NewExpression,
    name: &str,
    low: usize,
    high: usize,
) -> bool {
    matches!(&new.callee, Expression::Identifier(callee) if callee.name == name && symbol_of(source, callee).is_none())
        && (low..=high).contains(&new.arguments.len())
        && !new.arguments.iter().any(Argument::is_spread)
}

/// The blob a `new File([blob], 'name')` wraps: a `const` made before it by `new Blob(…)` or
/// `new File(…, …)`; its symbol and its name.
fn wrapped_blob(source: &Source, new: &NewExpression) -> Option<(SymbolId, String)> {
    if !is_new_global(source, new, "File", 2, 2) {
        return None;
    }
    let (Some(Expression::ArrayExpression(bits)), Some(name)) = (
        new.arguments[0].as_expression(),
        new.arguments[1].as_expression(),
    ) else {
        return None;
    };
    static_string(name)?;
    let [ArrayExpressionElement::Identifier(blob)] = bits.elements.as_slice() else {
        return None;
    };
    let symbol = symbol_of(source, blob)?;
    if source
        .semantic
        .scoping()
        .symbol_redeclarations(symbol)
        .len()
        > 1
    {
        return None;
    }
    let Expression::NewExpression(made) = const_value(source, symbol)? else {
        return None;
    };
    let is_blob = (is_new_global(source, made, "Blob", 0, 1)
        || is_new_global(source, made, "File", 2, 2))
        && made.span.start <= new.span.start;
    let name = source.semantic.scoping().symbol_name(symbol).to_owned();
    is_blob.then_some((symbol, name))
}

/// Whether the one use of the file is given to `URL.createObjectURL()`, or as the value of a
/// `FormData`'s `append()` or `set()`.
fn is_supported_use(source: &Source, reference: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    let span = nodes.kind(reference).span();
    let AstKind::CallExpression(call) = nodes.parent_kind(reference) else {
        return false;
    };
    let Some(method) = method_call(call) else {
        return false;
    };
    if !method.is_plain() || call.arguments.iter().any(Argument::is_spread) {
        return false;
    }
    match method.name {
        "createObjectURL" => {
            call.arguments.len() == 1
                && call.arguments[0].span() == span
                && matches!(method.object, Expression::Identifier(object)
                    if object.name == "URL" && symbol_of(source, object).is_none())
        }
        "append" | "set" => {
            (2..=3).contains(&call.arguments.len())
                && call.arguments[1].span() == span
                && matches!(const_init(source, method.object), Some(Expression::NewExpression(form))
                    if is_new_global(source, form, "FormData", 0, 0))
        }
        _ => false,
    }
}

impl Rule for NoBlobToFile {
    fn name(&self) -> &'static str {
        "unicorn/no-blob-to-file"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::VariableDeclarator(declarator) = node.kind() else {
            return;
        };
        let (BindingPattern::BindingIdentifier(file), Some(Expression::NewExpression(new))) =
            (&declarator.id, &declarator.init)
        else {
            return;
        };
        let nodes = source.semantic.nodes();
        let declaration = nodes.parent_id(node.id());
        let standalone = matches!(nodes.kind(declaration),
            AstKind::VariableDeclaration(statement) if statement.kind.is_const() && statement.declarations.len() == 1)
            && matches!(
                nodes.parent_kind(declaration),
                AstKind::Program(_) | AstKind::BlockStatement(_) | AstKind::FunctionBody(_)
            );
        if !standalone {
            return;
        }
        let Some((blob, name)) = wrapped_blob(source, new) else {
            return;
        };
        let Some(symbol) = file.symbol_id.get() else {
            return;
        };
        let scoping = source.semantic.scoping();
        let references = scoping.get_resolved_reference_ids(symbol);
        if references.len() != 1 {
            return;
        }
        let reference = scoping.get_reference(references[0]).node_id();
        let same_blob = scoping
            .find_binding(nodes.get_node(reference).scope_id(), name.as_str().into())
            == Some(blob);
        if same_blob && is_supported_use(source, reference) {
            findings.push(source.finding(
                self.name(),
                new.span.start,
                "Use the original `Blob` instead of converting it to a `File`.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoBlobToFile;
    use crate::testing::lint;

    /// The places of the findings.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoBlobToFile, "scripts/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_blobs_wrapped_for_nothing() {
        // The documentation's three cases.
        assert_eq!(
            check(
                "const blob = new Blob();\nconst file = new File([blob], 'image.jpg');\nURL.createObjectURL(file);\nconst formData = new FormData();\nfunction f() {\n  const second = new File([blob], 'image.jpg');\n  formData.append('file', second);\n}\nfunction g() {\n  const third = new File([blob], `image.jpg`);\n  formData.set('file', third);\n}"
            ),
            [(2, 14), (6, 18), (10, 17)]
        );
    }

    #[test]
    fn leaves_other_files() {
        for code in [
            "const blob = new Blob();\nURL.createObjectURL(blob);",
            "const blob = new Blob([], { type: 'a' }, 1);\nconst file = new File([blob], 'a');\nURL.createObjectURL(file);",
            "const blob = new Blob();\nconst file = new File([blob], name);\nURL.createObjectURL(file);",
            "const blob = new Blob();\nconst file = new File([blob, blob], 'a');\nURL.createObjectURL(file);",
            "const blob = new Blob();\nconst file = new File([blob], 'a');\nURL.createObjectURL(file);\nuse(file);",
            "const blob = new Blob();\nconst file = new File([blob], 'a'), other = 1;\nURL.createObjectURL(file);",
            "let blob = new Blob();\nconst file = new File([blob], 'a');\nURL.createObjectURL(file);",
            "const blob = new Blob();\nconst file = new File([blob], 'a');\nupload(file);",
            "const blob = new Blob();\nconst form = getForm();\nconst file = new File([blob], 'a');\nform.append('f', file);",
            "const blob = new Blob();\nconst file = new File([blob], 'a');\nfunction f(blob) { URL.createObjectURL(file); }",
        ] {
            assert_eq!(check(code), Vec::new(), "{code}");
        }
    }
}
