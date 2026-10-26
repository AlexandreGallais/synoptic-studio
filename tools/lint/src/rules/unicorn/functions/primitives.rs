//! The primitive value a literal writes, compared as `Object.is` compares them
//! (`eslint-plugin-unicorn` 78.0.0, `utils/get-primitive-literal-value.js`): a string, a template
//! without substitution, a number or a big integer, signed or not, a boolean or `null`, through
//! TypeScript's wrappers.

use oxc_ast::ast::{Expression, UnaryOperator};
use oxc_str::JSStr;

use crate::expressions::unwrapped;

/// A primitive value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Primitive {
    /// A string.
    Text(String),
    /// A number, by its bits: `0` and `-0` differ.
    Number(u64),
    /// A big integer, by its digits and sign.
    BigInt(String),
    /// `true` or `false`.
    Boolean(bool),
    /// `null`.
    Null,
}

/// The primitive value an expression writes.
#[must_use]
pub fn primitive(expression: &Expression) -> Option<Primitive> {
    match unwrapped(expression) {
        Expression::StringLiteral(each) => Some(Primitive::Text(each.value.as_str()?.to_owned())),
        Expression::TemplateLiteral(each) if each.expressions.is_empty() => Some(Primitive::Text(
            each.quasis
                .iter()
                .filter_map(|quasi| quasi.value.cooked)
                .map(JSStr::as_str)
                .collect::<Option<String>>()?,
        )),
        Expression::NumericLiteral(each) => Some(Primitive::Number(each.value.to_bits())),
        Expression::BigIntLiteral(each) => Some(Primitive::BigInt(each.value.to_string())),
        Expression::BooleanLiteral(each) => Some(Primitive::Boolean(each.value)),
        Expression::NullLiteral(_) => Some(Primitive::Null),
        Expression::UnaryExpression(unary) => match (unary.operator, unwrapped(&unary.argument)) {
            (UnaryOperator::UnaryNegation, Expression::NumericLiteral(each)) => {
                Some(Primitive::Number((-each.value).to_bits()))
            }
            (UnaryOperator::UnaryPlus, Expression::NumericLiteral(each)) => {
                Some(Primitive::Number(each.value.to_bits()))
            }
            (UnaryOperator::UnaryNegation, Expression::BigIntLiteral(each)) => {
                Some(Primitive::BigInt(format!("-{}", each.value)))
            }
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use oxc_allocator::Allocator;
    use oxc_ast::ast::Statement;
    use oxc_parser::Parser;
    use oxc_span::SourceType;

    use super::{Primitive, primitive};

    /// The primitive value of an expression written alone.
    fn value(code: &str) -> Option<Primitive> {
        let allocator = Allocator::default();
        let parsed = Parser::new(&allocator, code, SourceType::ts()).parse();
        match parsed.program.body.first() {
            Some(Statement::ExpressionStatement(statement)) => primitive(&statement.expression),
            _ => None,
        }
    }

    #[test]
    fn reads_literals_signs_and_templates() {
        assert_eq!(value("('px')"), Some(Primitive::Text("px".into())));
        assert_eq!(value("`px`"), value("('px')"));
        assert_eq!(value("1"), value("+1"));
        assert_ne!(value("0"), value("-0"));
        assert_eq!(value("-1.5"), Some(Primitive::Number((-1.5_f64).to_bits())));
        assert_eq!(value("-2n"), Some(Primitive::BigInt("-2".into())));
        assert_eq!(value("2n"), Some(Primitive::BigInt("2".into())));
        assert_eq!(value("true as boolean"), Some(Primitive::Boolean(true)));
        assert_eq!(value("null"), Some(Primitive::Null));
        for code in ["undefined", "x", "`${x}`", "-x", "!1", "/a/"] {
            assert_eq!(value(code), None, "{code}");
        }
    }
}
