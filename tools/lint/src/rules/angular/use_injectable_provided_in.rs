//! `@angular-eslint/use-injectable-provided-in`: an `@Injectable()` names its injector in
//! `providedIn`, never `undefined` nor `null`, so that it is tree-shaken (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests), with its default options: no class name ignored, `null`
//! refused. An HTTP interceptor is left, as there. The suggested injectors are not carried.

use oxc_ast::AstKind;
use oxc_ast::ast::{Argument, Expression};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::metadata::{class_decorator, implemented_names, metadata_property};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct UseInjectableProvidedIn;

impl Rule for UseInjectableProvidedIn {
    fn name(&self) -> &'static str {
        "@angular-eslint/use-injectable-provided-in"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(decorator) = class_decorator(class, "Injectable") else {
            return;
        };
        if implemented_names(class).contains(&"HttpInterceptor") {
            return;
        }
        let Expression::CallExpression(call) = &decorator.expression else {
            return;
        };
        let at = match (
            call.arguments.first(),
            metadata_property(decorator, "providedIn"),
        ) {
            (None, _) | (Some(Argument::ObjectExpression(_)), None) => decorator.span.start,
            (_, Some(property))
                if matches!(&property.value, Expression::NullLiteral(_))
                    || matches!(&property.value, Expression::Identifier(name) if name.name == "undefined") =>
            {
                property.value.span().start
            }
            _ => return,
        };
        findings.push(source.finding(
            self.name(),
            at,
            "The `providedIn` property is mandatory for `Injectables`".into(),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::UseInjectableProvidedIn;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&UseInjectableProvidedIn, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert_eq!(
                    each.message,
                    "The `providedIn` property is mandatory for `Injectables`"
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_an_injector_named_and_interceptors() {
        // The plugin's valid cases with its default options.
        for code in [
            "class Test {}\n",
            "const options = {};\n@Injectable(options)\nclass Test {}\n",
            "@Injectable({\n  providedIn: `any`,\n})\nclass Test {}\n",
            "@Injectable({\n  'providedIn': 'root',\n})\nclass Test {}\n",
            "@Injectable({\n  ['providedIn']: SomeModule,\n})\nclass Test {}\n",
            "@Injectable({\n  [`providedIn`]: providedIn(),\n})\nclass Test {}\n",
            "@Injectable()\nclass Test implements HttpInterceptor {}\n",
            "@Injectable()\nclass Test implements ng.HttpInterceptor {}\n",
            "@CustomInjectable()\nclass Test {}\n",
            "@Service()\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_a_missing_injector_at_the_decorator_and_an_empty_one_at_its_value() {
        // The plugin's invalid cases: `@Injectable({ providedIn: ` is 26 columns, the value at 27;
        // the computed keys add 4 (`['…']`).
        for (code, place) in [
            ("@Injectable()\nclass Test {}\n", (1, 1)),
            ("@Injectable({})\nclass Test {}\n", (1, 1)),
            (
                "const providedIn = 'anotherProperty';\n@Injectable({ [providedIn]: [] })\nclass Test {}\n",
                (2, 1),
            ),
            (
                "@Injectable({ providedIn: null })\nclass Test {}\n",
                (1, 27),
            ),
            (
                "@Injectable({ ['providedIn']: undefined })\nclass Test {}\n\n@Injectable()\nclass HttpPostInterceptor implements HttpInterceptor {}\n",
                (1, 31),
            ),
            (
                "@Injectable({ [`providedIn`]: undefined })\nclass Test {}\n",
                (1, 31),
            ),
        ] {
            assert_eq!(check(code), [place], "{code}");
        }
    }
}
