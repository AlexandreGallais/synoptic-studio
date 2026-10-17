//! `unicorn/consistent-destructuring`: a property read again from an object whose `const`
//! destructuring already bound it is read through that binding (`eslint-plugin-unicorn` 78.0.0).
//! A write to the object or its property, a call between the two, or an `in` guard around the
//! read leaves it alone. Its fix is a suggestion in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    BinaryOperator, BindingPattern, Expression, LogicalOperator, PropertyKey,
    StaticMemberExpression, UnaryOperator, VariableDeclarationKind,
};
use oxc_semantic::{AstNode, NodeId, ScopeId, SymbolId};
use oxc_span::{GetSpan, Span};

use super::syntax::{symbol_of, text, unwrap};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConsistentDestructuring;

/// A `const` destructuring of a name or `this`.
struct Declaration<'a> {
    /// The node of the declarator.
    id: NodeId,
    /// The object destructured.
    object: &'a Expression<'a>,
    /// The binding of that object, when it is a name the file declares.
    root: Option<SymbolId>,
    /// Whether that object is a name.
    named: bool,
    /// The node `this` is bound by, when the object is `this`.
    this_boundary: Option<NodeId>,
    /// Each property destructured into a plain name: its key, the name and its binding.
    properties: Vec<(&'a str, &'a str, SymbolId)>,
}

/// A write to a property of a name or `this`.
struct Write<'a> {
    /// The property's name.
    property: &'a str,
    /// The name written through, when it is one.
    name: Option<&'a str>,
    /// Its binding.
    root: Option<SymbolId>,
    /// The node `this` is bound by, when written through `this`.
    this_boundary: Option<NodeId>,
    /// Where the write starts.
    start: u32,
    /// Its variable scope.
    scope: ScopeId,
}

impl Rule for ConsistentDestructuring {
    fn name(&self) -> &'static str {
        "unicorn/consistent-destructuring"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let mut declarations: Vec<Declaration> = Vec::new();
        let mut writes: Vec<Write> = Vec::new();
        let mut members: Vec<(&AstNode, &StaticMemberExpression)> = Vec::new();
        for node in source.semantic.nodes().iter() {
            match node.kind() {
                AstKind::VariableDeclarator(declarator) => {
                    if let Some(declaration) = declaration(source, node, declarator) {
                        declarations.push(declaration);
                    }
                }
                AstKind::StaticMemberExpression(member) => {
                    if is_left_hand_side(source, node.id(), member.span) {
                        if let Some(write) = write(source, node, member) {
                            writes.push(write);
                        }
                    } else {
                        members.push((node, member));
                    }
                }
                _ => {}
            }
        }
        for (node, member) in members {
            if is_reported(source, node, member, &declarations, &writes) {
                findings.push(source.finding(
                    self.name(),
                    member.span.start,
                    "Use destructured variables over properties.".to_owned(),
                ));
            }
        }
    }
}

/// The declaration a declarator is, when it destructures a name or `this` into a `const`.
fn declaration<'a>(
    source: &Source<'_, 'a>,
    node: &AstNode<'a>,
    declarator: &'a oxc_ast::ast::VariableDeclarator<'a>,
) -> Option<Declaration<'a>> {
    let BindingPattern::ObjectPattern(pattern) = &declarator.id else {
        return None;
    };
    let AstKind::VariableDeclaration(statement) = source.semantic.nodes().parent_kind(node.id())
    else {
        return None;
    };
    if statement.kind != VariableDeclarationKind::Const {
        return None;
    }
    let object = declarator.init.as_ref()?;
    let (root, named, this_boundary) = match object {
        Expression::Identifier(identifier) => (symbol_of(source, identifier), true, None),
        Expression::ThisExpression(this) => {
            (None, false, Some(this_boundary(source, this.node_id())))
        }
        _ => return None,
    };
    let properties = pattern
        .properties
        .iter()
        .filter(|property| !property.computed)
        .filter_map(|property| {
            let PropertyKey::StaticIdentifier(key) = &property.key else {
                return None;
            };
            let BindingPattern::BindingIdentifier(binding) = &property.value else {
                return None;
            };
            Some((
                key.name.as_str(),
                binding.name.as_str(),
                binding.symbol_id(),
            ))
        })
        .collect();
    Some(Declaration {
        id: node.id(),
        object,
        root,
        named,
        this_boundary,
        properties,
    })
}

/// The write a member assigned, updated or deleted is, when read through a name or `this`.
fn write<'a>(
    source: &Source<'_, 'a>,
    node: &AstNode<'a>,
    member: &'a StaticMemberExpression<'a>,
) -> Option<Write<'a>> {
    let (name, root, this_boundary) = match unwrap(&member.object) {
        Expression::Identifier(identifier) => (
            Some(identifier.name.as_str()),
            symbol_of(source, identifier),
            None,
        ),
        Expression::ThisExpression(this) => {
            (None, None, Some(this_boundary(source, this.node_id())))
        }
        _ => return None,
    };
    Some(Write {
        property: member.property.name.as_str(),
        name,
        root,
        this_boundary,
        start: member.span.start,
        scope: variable_scope(source, node.scope_id()),
    })
}

/// Whether a member stands where a value is written: assigned, updated, deleted, the target of
/// a `for…of` or `for…in`, or of a destructuring assignment (`isLeftHandSide`).
fn is_left_hand_side(source: &Source, id: NodeId, span: Span) -> bool {
    match source.semantic.nodes().parent_kind(id) {
        AstKind::AssignmentExpression(assignment) => assignment.left.span() == span,
        AstKind::AssignmentTargetWithDefault(target) => target.binding.span() == span,
        AstKind::AssignmentTargetPropertyProperty(property) => property.binding.span() == span,
        AstKind::ArrayAssignmentTarget(_)
        | AstKind::AssignmentTargetRest(_)
        | AstKind::UpdateExpression(_) => true,
        AstKind::ForOfStatement(statement) => statement.left.span() == span,
        AstKind::ForInStatement(statement) => statement.left.span() == span,
        AstKind::UnaryExpression(unary) => unary.operator == UnaryOperator::Delete,
        _ => false,
    }
}

/// Whether a member is a callee, a tag, or a computed or written one, which the rule skips.
fn is_ignored(source: &Source, id: NodeId, member: &StaticMemberExpression) -> bool {
    match source.semantic.nodes().parent_kind(id) {
        AstKind::CallExpression(call) => call.callee.span() == member.span,
        AstKind::NewExpression(new) => new.callee.span() == member.span,
        AstKind::TaggedTemplateExpression(tagged) => tagged.tag.span() == member.span,
        _ => false,
    }
}

/// The node that binds `this` where a node stands: the nearest function that is no arrow, a
/// class's field or static block, or the file.
fn this_boundary(source: &Source, id: NodeId) -> NodeId {
    source
        .semantic
        .nodes()
        .ancestors(id)
        .find(|node| {
            matches!(
                node.kind(),
                AstKind::Function(_)
                    | AstKind::PropertyDefinition(_)
                    | AstKind::AccessorProperty(_)
                    | AstKind::StaticBlock(_)
                    | AstKind::Program(_)
            )
        })
        .map_or(id, AstNode::id)
}

/// The scope a scope's `var` declarations would go to: the nearest function's or the file's.
fn variable_scope(source: &Source, scope: ScopeId) -> ScopeId {
    let scoping = source.semantic.scoping();
    scoping
        .scope_ancestors(scope)
        .find(|each| scoping.scope_flags(*each).is_var())
        .unwrap_or(scope)
}

/// Whether a member read is one the rule reports.
fn is_reported(
    source: &Source,
    node: &AstNode,
    member: &StaticMemberExpression,
    declarations: &[Declaration],
    writes: &[Write],
) -> bool {
    if is_ignored(source, node.id(), member) {
        return false;
    }
    let object = unwrap(&member.object);
    let key = text(source, object.span());
    let scoping = source.semantic.scoping();
    let member_scope = node.scope_id();
    let found = declarations.iter().rev().any(|declaration| {
        text(source, declaration.object.span()) == key
            && declaration.properties.iter().any(|(key, name, symbol)| {
                *key == member.property.name.as_str()
                    && scoping.find_binding(member_scope, oxc_ast::ast::Ident::from(*name))
                        == Some(*symbol)
            })
            && is_matching(source, node, member, declaration, writes)
    });
    found && !is_in_guarded_branch(source, node.id(), member)
}

/// Whether a declaration still holds the member's value where it is read.
fn is_matching(
    source: &Source,
    node: &AstNode,
    member: &StaticMemberExpression,
    declaration: &Declaration,
    writes: &[Write],
) -> bool {
    let declaration_end = declaration.object.span().end;
    if declaration_end >= member.span.start {
        return false;
    }
    let object = unwrap(&member.object);
    let (name, root, boundary) = match object {
        Expression::Identifier(identifier) => (
            Some(identifier.name.as_str()),
            symbol_of(source, identifier),
            None,
        ),
        Expression::ThisExpression(this) => {
            (None, None, Some(this_boundary(source, this.node_id())))
        }
        _ => (None, None, None),
    };
    if declaration.named && name.is_some() && root != declaration.root {
        return false;
    }
    if declaration.this_boundary.is_some() && boundary != declaration.this_boundary {
        return false;
    }
    let nodes = source.semantic.nodes();
    let declaration_scope = nodes.get_node(declaration.id).scope_id();
    let scoping = source.semantic.scoping();
    if !scoping
        .scope_ancestors(node.scope_id())
        .any(|each| each == declaration_scope)
    {
        return false;
    }
    let member_variable_scope = variable_scope(source, node.scope_id());
    if let Some(root) = declaration.root {
        let reassigned = scoping.get_resolved_reference_ids(root).iter().any(|id| {
            let reference = scoping.get_reference(*id);
            let at = nodes.kind(reference.node_id()).span().start;
            reference.is_write()
                && at >= declaration_end
                && (variable_scope(source, reference.scope_id()) != member_variable_scope
                    || at <= member.span.start)
        });
        if reassigned {
            return false;
        }
    }
    let property_written = writes.iter().any(|write| {
        write.property == member.property.name.as_str()
            && write.name == name
            && write.root == root
            && write.this_boundary == boundary
            && (write.scope != member_variable_scope
                || (write.start >= declaration_end && write.start <= member.span.start))
    });
    if property_written {
        return false;
    }
    !has_call_between(
        source,
        Span::new(declaration_end, member.span.start),
        variable_scope(source, declaration_scope),
        member.span,
    )
}

/// Whether a call in the declaration's variable scope, whose callee starts with a name written
/// in a range, may run between the destructuring and the read: one around the read runs after it.
fn has_call_between(source: &Source, range: Span, scope: ScopeId, read: Span) -> bool {
    let nodes = source.semantic.nodes();
    nodes.iter().any(|node| {
        let AstKind::IdentifierReference(identifier) = node.kind() else {
            return false;
        };
        if identifier.span.start < range.start || identifier.span.end > range.end {
            return false;
        }
        let Some(call) = enclosing_call(source, node.id()) else {
            return false;
        };
        let call_node = nodes.get_node(call);
        let call_span = call_node.kind().span();
        variable_scope(source, call_node.scope_id()) == scope
            && (read.start < call_span.start || read.start > call_span.end)
    })
}

/// The call, `new` or tagged template whose callee a name starts, through members read from it.
fn enclosing_call(source: &Source, id: NodeId) -> Option<NodeId> {
    let nodes = source.semantic.nodes();
    let mut current = id;
    loop {
        let span = nodes.kind(current).span();
        let parent = nodes.parent_node(current);
        match parent.kind() {
            AstKind::CallExpression(call) if call.callee.span() == span => {
                return Some(parent.id());
            }
            AstKind::NewExpression(new) if new.callee.span() == span => return Some(parent.id()),
            AstKind::TaggedTemplateExpression(tagged) if tagged.tag.span() == span => {
                return Some(parent.id());
            }
            AstKind::StaticMemberExpression(member) if member.object.span() == span => {}
            AstKind::ComputedMemberExpression(member) if member.object.span() == span => {}
            AstKind::PrivateFieldExpression(member) if member.object.span() == span => {}
            _ => return None,
        }
        current = parent.id();
    }
}

/// Whether a read sits in the branch an `in` check of its property guards: the consequent of a
/// ternary or an `if`, or the right of `&&`, inside the same function declaration.
fn is_in_guarded_branch(source: &Source, id: NodeId, member: &StaticMemberExpression) -> bool {
    let nodes = source.semantic.nodes();
    let mut child = id;
    for parent in nodes.ancestors(id) {
        let child_span = nodes.kind(child).span();
        let guarded = match parent.kind() {
            AstKind::Function(function) if function.is_declaration() => return false,
            AstKind::MethodDefinition(_)
            | AstKind::PropertyDefinition(_)
            | AstKind::AccessorProperty(_) => return false,
            AstKind::ConditionalExpression(conditional) => {
                conditional.consequent.span() == child_span
                    && has_matching_in(source, &conditional.test, member)
            }
            AstKind::LogicalExpression(logical) => {
                logical.operator == LogicalOperator::And
                    && logical.right.span() == child_span
                    && has_matching_in(source, &logical.left, member)
            }
            AstKind::IfStatement(statement) => {
                statement.consequent.span() == child_span
                    && has_matching_in(source, &statement.test, member)
            }
            _ => false,
        };
        if guarded {
            return true;
        }
        child = parent.id();
    }
    false
}

/// Whether a test is, or joins with `&&`, `"property" in object` for the member's object.
fn has_matching_in(source: &Source, test: &Expression, member: &StaticMemberExpression) -> bool {
    match test {
        Expression::BinaryExpression(binary) if binary.operator == BinaryOperator::In => {
            matches!(&binary.left, Expression::StringLiteral(literal)
                if literal.value == member.property.name)
                && text(source, binary.right.span()) == text(source, member.object.span())
        }
        Expression::LogicalExpression(logical) if logical.operator == LogicalOperator::And => {
            has_matching_in(source, &logical.left, member)
                || has_matching_in(source, &logical.right, member)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::ConsistentDestructuring;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&ConsistentDestructuring, "playground/a.ts", code))
    }

    const USE: [&str; 1] = ["Use destructured variables over properties."];

    #[test]
    fn reports_the_documented_reads_of_a_destructured_property() {
        assert_eq!(check("const {a} = foo;\nconsole.log(foo.a);"), USE);
        assert_eq!(check("const {bar} = foo;\nconst {a} = foo.bar;"), USE);
        assert_eq!(check("const {a: b} = foo;\nconsole.log(foo.a);"), USE);
        assert_eq!(check("const {a} = foo;\nconsole.log((foo as T).a);"), USE);
        assert_eq!(check("const {a} = foo;\nconsole.log(foo.a.b);"), USE);
        assert_eq!(
            check("const {a} = foo;\nfunction f() { return foo.a; }"),
            USE
        );
    }

    #[test]
    fn accepts_the_documented_reads_left_alone() {
        for code in [
            "const {a} = foo;\nconsole.log(a);",
            "console.log(foo.a, foo.b);",
            "const {a} = foo;\nconsole.log(a, foo.b);",
            "const {\n\ta: {b},\n} = foo;\nconsole.log(foo.a.c);",
            "const {bar} = foo;\nconst {a} = bar;",
            "const {a} = foo;\nconsole.log(a, foo.b());",
            "const {a} = foo.bar;\nconsole.log(foo.bar.a);",
            "const {a} = foo;\nupdate(foo);\nconsole.log(foo.a);",
            "const {a} = foo;\nfoo.update();\nconsole.log(foo.a);",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }

    #[test]
    fn leaves_writes_calls_guards_shadows_and_other_scopes() {
        for code in [
            "const {a} = foo;\nfoo.a = 1;",
            "const {a} = foo;\nfoo.a();",
            "const {a} = foo;\nfoo.a`x`;",
            "const {a} = foo;\nfoo.a = 2;\nconsole.log(foo.a);",
            "let {a} = foo;\nconsole.log(foo.a);",
            "const {a = 1} = foo;\nconsole.log(foo.a);",
            "const {[k]: a} = foo;\nconsole.log(foo.a);",
            "console.log(foo.a);\nconst {a} = foo;",
            "const {a} = foo;\nfunction f(a) { return foo.a; }",
            "const {a} = foo;\nfunction f(foo) { return foo.a; }",
            "function f() { const {a} = foo; }\nconsole.log(foo.a);",
            "const {a} = foo;\nconst x = 'a' in foo ? foo.a : 0;",
            "const {a} = foo;\nif ('a' in foo && ok) { console.log(foo.a); }",
            "const {a} = foo;\nconst b = foo['a'];",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
