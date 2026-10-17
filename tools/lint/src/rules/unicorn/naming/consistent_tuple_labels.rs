//! `unicorn/consistent-tuple-labels`: a tuple type labels all its elements or none, a rest
//! counting as labeled by its own label (`eslint-plugin-unicorn` 78.0.0). No fix: a label
//! cannot be guessed.

use oxc_ast::AstKind;
use oxc_ast::ast::{TSTupleElement, TSType};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ConsistentTupleLabels;

/// Whether an element is labeled, `name: T` or `...name: T[]`.
fn is_labeled(element: &TSTupleElement) -> bool {
    match element {
        TSTupleElement::TSNamedTupleMember(_) => true,
        TSTupleElement::TSRestType(rest) => {
            matches!(rest.type_annotation, TSType::TSNamedTupleMember(_))
        }
        _ => false,
    }
}

impl Rule for ConsistentTupleLabels {
    fn name(&self) -> &'static str {
        "unicorn/consistent-tuple-labels"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::TSTupleType(tuple) = node.kind() else {
            return;
        };
        let elements = &tuple.element_types;
        if elements.len() < 2 {
            return;
        }
        let unlabeled: Vec<&TSTupleElement> = elements
            .iter()
            .filter(|element| !is_labeled(element))
            .collect();
        if unlabeled.is_empty() || unlabeled.len() == elements.len() {
            return;
        }
        for element in unlabeled {
            findings.push(source.finding(
                self.name(),
                element.span().start,
                "This tuple element should have a label, just like the other elements.".to_owned(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ConsistentTupleLabels;
    use crate::testing::{NONE, lint, messages};

    /// The messages on a piece of code.
    fn check(code: &str) -> Vec<String> {
        messages(&lint(&ConsistentTupleLabels, "playground/a.ts", code))
    }

    const MESSAGE: &str = "This tuple element should have a label, just like the other elements.";

    #[test]
    fn reports_the_documented_unlabeled_elements() {
        assert_eq!(check("type Point = [x: number, number];"), [MESSAGE]);
        assert_eq!(
            check("type Route = [name: string, ...string[]];"),
            [MESSAGE]
        );
        assert_eq!(
            check("type A = [number, y: number, string];"),
            [MESSAGE, MESSAGE]
        );
    }

    #[test]
    fn accepts_the_documented_consistent_tuples() {
        for code in [
            "type Point = [x: number, y: number];",
            "type Point = [number, number];",
            "type Route = [name: string, ...segments: string[]];",
            "type One = [x: number];",
            "type Empty = [];",
        ] {
            assert_eq!(check(code), NONE, "{code}");
        }
    }
}
