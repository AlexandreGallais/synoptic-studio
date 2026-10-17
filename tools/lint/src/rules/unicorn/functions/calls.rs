//! The local functions of a file whose every use is a direct call, and those calls
//! (`eslint-plugin-unicorn` 78.0.0, `utils/track-local-function-calls.js`): a function declared
//! once, by a declaration or a `const` holding a function or an arrow, neither exported nor
//! reassigned, read only as the callee of a call or a `new` — through TypeScript's wrappers — a
//! read inside a type query aside. A file calling `eval` directly has none. Classes cannot occur
//! here (`functional/no-classes`), so their constructors and private methods are left out.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    Argument, ArrowFunctionExpression, BindingPattern, Expression, FormalParameters, Function,
    VariableDeclarationKind,
};
use oxc_ast_visit::Visit;
use oxc_semantic::{NodeId, ScopeId, SymbolId};
use oxc_span::{GetSpan, Span};

use crate::expressions::unwrapped;
use crate::source::Source;

/// A local function: a declaration or expression, or an arrow.
#[derive(Clone, Copy)]
pub enum Callable<'a> {
    /// `function f() {}`, or a function expression held by a `const`.
    Function(&'a Function<'a>),
    /// An arrow held by a `const`.
    Arrow(&'a ArrowFunctionExpression<'a>),
}

impl<'a> Callable<'a> {
    /// Its parameters.
    #[must_use]
    pub fn parameters(&self) -> &'a FormalParameters<'a> {
        match self {
            Self::Function(function) => &function.params,
            Self::Arrow(arrow) => &arrow.params,
        }
    }

    /// Whether it is async or a generator.
    #[must_use]
    pub fn pauses(&self) -> bool {
        match self {
            Self::Function(function) => function.r#async || function.generator,
            Self::Arrow(arrow) => arrow.r#async,
        }
    }

    /// Its range.
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::Function(function) => function.span,
            Self::Arrow(arrow) => arrow.span,
        }
    }

    /// Whether its body reads `arguments`, nested functions left out.
    #[must_use]
    pub fn reads_arguments(&self) -> bool {
        let mut search = ArgumentsSearch::default();
        match self {
            Self::Function(function) => {
                if let Some(body) = &function.body {
                    search.visit_function_body(body);
                }
            }
            Self::Arrow(arrow) => search.visit_arrow_function_body(&arrow.body),
        }
        search.found
    }

    /// Its own scope.
    #[must_use]
    pub fn scope(&self) -> Option<ScopeId> {
        match self {
            Self::Function(function) => function.scope_id.get(),
            Self::Arrow(arrow) => arrow.scope_id.get(),
        }
    }
}

/// The search of a body for `arguments`, nested functions left out.
#[derive(Default)]
struct ArgumentsSearch {
    found: bool,
}

impl<'a> Visit<'a> for ArgumentsSearch {
    fn visit_identifier_reference(&mut self, name: &oxc_ast::ast::IdentifierReference<'a>) {
        self.found |= name.name == "arguments";
    }

    fn visit_function(&mut self, _function: &Function<'a>, _flags: oxc_semantic::ScopeFlags) {}
}

/// A call of a local function: a call or a `new`, by its node.
#[derive(Clone, Copy)]
pub struct Call<'a> {
    /// The call's node.
    pub id: NodeId,
    /// Its range.
    pub span: Span,
    /// Its arguments.
    pub arguments: &'a [Argument<'a>],
    /// Whether it is a call, not a `new`.
    pub is_call: bool,
}

/// A local function and its calls.
pub struct Target<'a> {
    /// The function.
    pub callable: Callable<'a>,
    /// Its name's variable.
    pub symbol: SymbolId,
    /// Where a finding on it points: its name.
    pub name_at: u32,
    /// Its calls, in the order of the file.
    pub calls: Vec<Call<'a>>,
}

/// The function a variable is declared as, and where its name stands.
fn declared<'a>(source: &Source<'_, 'a>, symbol: SymbolId) -> Option<(Callable<'a>, u32, NodeId)> {
    let nodes = source.semantic.nodes();
    let declaration = source.semantic.scoping().symbol_declaration(symbol);
    match nodes.kind(declaration) {
        AstKind::Function(function) if function.body.is_some() && function.id.is_some() => {
            let at = function
                .id
                .as_ref()
                .map_or(function.span.start, |id| id.span.start);
            Some((Callable::Function(function), at, declaration))
        }
        AstKind::VariableDeclarator(declarator) => {
            let BindingPattern::BindingIdentifier(name) = &declarator.id else {
                return None;
            };
            let is_const = matches!(nodes.parent_kind(declaration),
                AstKind::VariableDeclaration(each) if each.kind == VariableDeclarationKind::Const);
            let callable = match unwrapped(declarator.init.as_ref()?) {
                Expression::FunctionExpression(function) => Callable::Function(function),
                Expression::ArrowFunctionExpression(arrow) => Callable::Arrow(arrow),
                _ => return None,
            };
            is_const.then_some((callable, name.span.start, nodes.parent_id(declaration)))
        }
        _ => None,
    }
}

/// Whether a declaration is exported.
fn is_exported(source: &Source, declaration: NodeId) -> bool {
    matches!(
        source.semantic.nodes().parent_kind(declaration),
        AstKind::ExportDeclaration(_) | AstKind::ExportDefaultDeclaration(_)
    )
}

/// The call a name is the callee of, through TypeScript's wrappers.
fn call_of<'a>(source: &Source<'_, 'a>, name: NodeId) -> Option<Call<'a>> {
    let nodes = source.semantic.nodes();
    let mut current = name;
    loop {
        let span = nodes.kind(current).span();
        let parent = nodes.parent_id(current);
        match nodes.kind(parent) {
            AstKind::TSAsExpression(_)
            | AstKind::TSSatisfiesExpression(_)
            | AstKind::TSNonNullExpression(_)
            | AstKind::TSTypeAssertion(_) => current = parent,
            AstKind::CallExpression(call) if call.callee.span() == span => {
                return Some(Call {
                    id: parent,
                    span: call.span,
                    arguments: &call.arguments,
                    is_call: true,
                });
            }
            AstKind::NewExpression(call) if call.callee.span() == span => {
                return Some(Call {
                    id: parent,
                    span: call.span,
                    arguments: &call.arguments,
                    is_call: false,
                });
            }
            _ => return None,
        }
    }
}

/// Whether the file calls `eval` directly.
fn calls_eval(source: &Source) -> bool {
    source.semantic.nodes().iter().any(|node| {
        matches!(node.kind(), AstKind::CallExpression(call) if call.callee.is_specific_id("eval"))
    })
}

/// The local functions every use of which is a direct call, with their calls.
#[must_use]
pub fn targets<'a>(source: &Source<'_, 'a>) -> Vec<Target<'a>> {
    if calls_eval(source) {
        return Vec::new();
    }
    let scoping = source.semantic.scoping();
    let nodes = source.semantic.nodes();
    let mut found = Vec::new();
    for symbol in scoping.symbol_ids() {
        let Some((callable, name_at, declaration)) = declared(source, symbol) else {
            continue;
        };
        if scoping.symbol_redeclarations(symbol).len() > 1 || is_exported(source, declaration) {
            continue;
        }
        let mut calls = Vec::new();
        let mut direct = true;
        for reference in scoping.get_resolved_references(symbol) {
            let in_type_query = nodes
                .ancestor_kinds(reference.node_id())
                .any(|kind| matches!(kind, AstKind::TSTypeQuery(_)));
            if in_type_query {
                continue;
            }
            match call_of(source, reference.node_id()) {
                Some(call) if !reference.is_write() => calls.push(call),
                _ => direct = false,
            }
        }
        if direct {
            calls.sort_by_key(|call| call.span.start);
            found.push(Target {
                callable,
                symbol,
                name_at,
                calls,
            });
        }
    }
    found
}

/// Whether a range lies inside another.
#[must_use]
pub fn inside(inner: Span, outer: Span) -> bool {
    inner.start >= outer.start && inner.end <= outer.end
}

#[cfg(test)]
mod tests {
    use super::targets;
    use crate::finding::Finding;
    use crate::rules::Rule;
    use crate::source::Source;
    use crate::testing::lint;

    /// Reports each tracked function with its number of calls.
    struct Probe;

    impl Rule for Probe {
        fn name(&self) -> &'static str {
            "probe/calls"
        }

        fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
            for target in targets(source) {
                let message = format!("{} calls", target.calls.len());
                findings.push(source.finding(self.name(), target.name_at, message));
                let _ = (
                    target.symbol,
                    target.callable.scope(),
                    target.callable.span(),
                );
            }
        }
    }

    /// The tracked functions' lines and calls.
    fn found(code: &str) -> Vec<(usize, String)> {
        lint(&Probe, "scripts/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.message))
            .collect()
    }

    #[test]
    fn tracks_local_functions_called_directly() {
        let code = "function a() {}\na(); a();\nconst b = (x: number) => x;\nb(1);\n(b as F)(2);\nconst c = function () {};\nnew c();\nfunction d() {}\nconst e = d;\nexport function f() {}\nf();\nlet g = () => 1;\nconst h = 1;\nfunction k() {}\ntype T = typeof k;\nk();";
        assert_eq!(
            found(code),
            [
                (1, "2 calls".to_owned()),
                (3, "2 calls".to_owned()),
                (6, "1 calls".to_owned()),
                (14, "1 calls".to_owned()),
            ]
        );
        assert_eq!(found("function a() {}\na();\neval('a');"), []);
    }
}
