//! `functional/no-classes`: no class, declared or as an expression (`eslint-plugin-functional`
//! 10.0.1, `docs/rules/no-classes.md`), without the options the project leaves unset. No fix.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoClasses;

/// The plugin's message.
const MESSAGE: &str = "Unexpected class, use functions not classes.";

impl Rule for NoClasses {
    fn name(&self) -> &'static str {
        "functional/no-classes"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        if let AstKind::Class(class) = node.kind() {
            findings.push(source.finding(self.name(), class.span.start, MESSAGE.to_owned()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoClasses;
    use crate::testing::lint;

    /// The places of the findings.
    fn places(code: &str) -> Vec<(usize, usize)> {
        lint(&NoClasses, "src/a/b.ts", code)
            .iter()
            .map(|each| (each.line, each.column))
            .collect()
    }

    #[test]
    fn reports_a_class_declared_or_as_an_expression() {
        // The documentation's case, then an expression and an exported class.
        let dog = "class Dog {\n  constructor(name, age) {\n    this.name = name;\n    this.age = age;\n  }\n\n  get ageInDogYears() {\n    return 7 * this.age;\n  }\n}\n\nconst dogA = new Dog(\"Jasper\", 2);\n\nconsole.log(`${dogA.name} is ${dogA.ageInDogYears} in dog years.`);";

        assert_eq!(places(dog), [(1, 1)]);
        assert_eq!(places("const A = class {};"), [(1, 11)]);
        assert_eq!(places("export class B {}"), [(1, 8)]);
    }

    #[test]
    fn lets_functions_and_objects_through() {
        let code = "function getAgeInDogYears(age) {\n  return 7 * age;\n}\n\nconst dogA = {\n  name: \"Jasper\",\n  age: 2,\n};\n\nconsole.log(`${dogA.name} is ${getAgeInDogYears(dogA.age)} in dog years.`);";

        assert_eq!(places(code), Vec::<(usize, usize)>::new());
    }
}
