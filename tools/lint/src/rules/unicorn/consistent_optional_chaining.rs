//! `unicorn/consistent-optional-chaining`: two members of the same base on each side of `&&` or
//! `||` read it the same way: no `?.` on the right of `&&`, and `?.` on both sides of `||` or on
//! neither (`eslint-plugin-unicorn` 78.0.0). Its fixes are suggestions in `ESLint`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, LogicalOperator};
use oxc_semantic::AstNode;
use oxc_span::Span;

use super::syntax::{same_reference, unwrap_chain};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConsistentOptionalChaining;

/// A member read: its base, whether it is optional, and where it starts.
struct Member<'b, 'a> {
    /// The object read from.
    object: &'b Expression<'a>,
    /// Whether it reads with `?.`.
    optional: bool,
    /// Where it starts.
    span: Span,
}

/// The member an expression is, through `?.` chains and TypeScript's wrappers.
fn member_of<'b, 'a>(expression: &'b Expression<'a>) -> Option<Member<'b, 'a>> {
    let unwrapped = unwrap_chain(expression);
    let chain_member = match unwrapped {
        Expression::ChainExpression(chain) => match &chain.expression {
            oxc_ast::ast::ChainElement::StaticMemberExpression(member) => {
                return Some(Member {
                    object: &member.object,
                    optional: member.optional,
                    span: member.span,
                });
            }
            oxc_ast::ast::ChainElement::ComputedMemberExpression(member) => {
                return Some(Member {
                    object: &member.object,
                    optional: member.optional,
                    span: member.span,
                });
            }
            _ => None,
        },
        _ => None,
    };
    chain_member.or(match unwrapped {
        Expression::StaticMemberExpression(member) => Some(Member {
            object: &member.object,
            optional: member.optional,
            span: member.span,
        }),
        Expression::ComputedMemberExpression(member) => Some(Member {
            object: &member.object,
            optional: member.optional,
            span: member.span,
        }),
        _ => None,
    })
}

/// Whether a base is a name, `this` or `super`, or a path of members from one.
fn is_supported_base(expression: &Expression) -> bool {
    match unwrap_chain(expression) {
        Expression::Identifier(_) | Expression::ThisExpression(_) | Expression::Super(_) => true,
        other => member_of(other).is_some_and(|member| is_supported_base(member.object)),
    }
}

impl Rule for ConsistentOptionalChaining {
    fn name(&self) -> &'static str {
        "unicorn/consistent-optional-chaining"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::LogicalExpression(logical) = node.kind() else {
            return;
        };
        let (Some(left), Some(right)) = (member_of(&logical.left), member_of(&logical.right))
        else {
            return;
        };
        let (left_base, right_base) = (unwrap_chain(left.object), unwrap_chain(right.object));
        if !is_supported_base(left_base)
            || !is_supported_base(right_base)
            || !same_reference(left_base, right_base)
        {
            return;
        }
        let message = match logical.operator {
            LogicalOperator::And if right.optional => "Remove unnecessary optional chaining.",
            LogicalOperator::Or if left.optional != right.optional => {
                if right.optional {
                    "Remove unnecessary optional chaining."
                } else {
                    "Use optional chaining consistently."
                }
            }
            _ => return,
        };
        findings.push(source.finding(self.name(), right.span.start, message.to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::ConsistentOptionalChaining;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&ConsistentOptionalChaining, "playground/a.ts", code))
    }

    #[test]
    fn reports_the_documented_inconsistent_chains() {
        assert_eq!(
            check("foo?.bar || foo.baz;"),
            ["Use optional chaining consistently."]
        );
        assert_eq!(
            check("foo.bar || foo?.baz;"),
            ["Remove unnecessary optional chaining."]
        );
        assert_eq!(
            check("foo?.bar && foo?.baz;"),
            ["Remove unnecessary optional chaining."]
        );
        assert_eq!(
            check("foo.bar && foo?.baz;"),
            ["Remove unnecessary optional chaining."]
        );
        assert_eq!(
            check("a.b?.c || a.b.d;"),
            ["Use optional chaining consistently."]
        );
    }

    #[test]
    fn accepts_the_documented_consistent_chains() {
        for code in [
            "foo?.bar || foo?.baz;",
            "foo.bar || foo.baz;",
            "foo?.bar && foo.baz;",
            "foo.bar && foo.baz;",
            "foo && foo.bar;",
            "foo?.bar ?? foo.baz;",
            "foo?.bar || other.baz;",
            "f().a?.b || f().a.c;",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
