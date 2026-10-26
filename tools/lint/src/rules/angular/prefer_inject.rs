//! `@angular-eslint/prefer-inject`: no dependency injected through a constructor's parameter of a
//! class Angular creates, `inject()` instead (`@angular-eslint/eslint-plugin` 22.6.0, the cases of
//! its tests). The plugin's `ClassDeclaration:has(Decorator…)` reads every decorator under the
//! class; the class's own are read here, the only ones that make it Angular's.

use oxc_ast::AstKind;
use oxc_ast::ast::{ClassElement, FormalParameter, MethodDefinitionKind, TSType};
use oxc_semantic::AstNode;

use super::classes::decorator_name;
use super::metadata::class_decorator_of;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct PreferInject;

/// The decorators of a class whose constructor Angular injects.
const INJECTED: [&str; 5] = ["Component", "Directive", "Injectable", "Pipe", "Service"];

/// The parameter decorators that only injection reads.
const INJECTION_DECORATORS: [&str; 5] = ["Host", "Inject", "Optional", "Self", "SkipSelf"];

/// Whether a parameter is injected (`shouldReportParameter`): an injection decorator, a modifier,
/// or a written type other than a primitive's.
fn is_injected(parameter: &FormalParameter) -> bool {
    let decorated = parameter.decorators.iter().any(|decorator| {
        decorator_name(decorator).is_some_and(|name| INJECTION_DECORATORS.contains(&name))
    });
    if decorated || parameter.has_modifier() {
        return true;
    }
    parameter
        .type_annotation
        .as_ref()
        .is_some_and(|annotation| {
            !matches!(
                annotation.type_annotation,
                TSType::TSStringKeyword(_)
                    | TSType::TSNumberKeyword(_)
                    | TSType::TSBooleanKeyword(_)
                    | TSType::TSBigIntKeyword(_)
                    | TSType::TSSymbolKeyword(_)
                    | TSType::TSAnyKeyword(_)
                    | TSType::TSUnknownKeyword(_)
            )
        })
}

impl Rule for PreferInject {
    fn name(&self) -> &'static str {
        "@angular-eslint/prefer-inject"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if class_decorator_of(class, &INJECTED).is_none() {
            return;
        }
        for element in &class.body.body {
            let ClassElement::MethodDefinition(method) = element else {
                continue;
            };
            if method.kind != MethodDefinitionKind::Constructor {
                continue;
            }
            for parameter in method
                .value
                .params
                .items
                .iter()
                .filter(|each| is_injected(each))
            {
                findings.push(source.finding(
                    self.name(),
                    parameter.span.start,
                    "Prefer using the inject() function over constructor parameter injection. Use Angular's migration schematic to automatically refactor: ng generate @angular/core:inject".into(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PreferInject;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&PreferInject, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert!(
                    each.message
                        .starts_with("Prefer using the inject() function")
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_inject_empty_constructors_and_primitives() {
        // The plugin's valid cases.
        for code in [
            "class PlainClass {\n  constructor(private value: string) {}\n}\n",
            "@Injectable()\nclass UserService {\n  private http = inject(HttpClient);\n}\n",
            "@Service()\nclass UserService {\n  private http = inject(HttpClient);\n}\n",
            "@Component({})\nclass MyComponent {\n  constructor() {}\n}\n",
            "@Component({})\nclass MyComponent extends BaseComponent {\n  constructor() {\n    super();\n  }\n}\n",
            "@Injectable()\nclass Logger {\n  constructor(level: string) {}\n}\n",
            // A parameter without a written type, and a module Angular does not inject this way.
            "@Injectable()\nclass Logger {\n  constructor(level) {}\n}\n",
            "@NgModule({})\nclass Module {\n  constructor(router: Router) {}\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_injected_parameter_at_its_start() {
        // The plugin's invalid cases: `  constructor(` is 14 columns, the parameter at 15; one a
        // line, at column 5, its decorators included.
        for (code, places) in [
            (
                "@Injectable()\nclass UserService {\n  constructor(private http: HttpClient) {}\n}\n",
                vec![(3, 15)],
            ),
            (
                "@Service()\nclass UserService {\n  constructor(private http: HttpClient) {}\n}\n",
                vec![(3, 15)],
            ),
            (
                "@Component({})\nclass MyComponent {\n  constructor(\n    private userService: UserService,\n    private http: HttpClient\n  ) {}\n}\n",
                vec![(4, 5), (5, 5)],
            ),
            (
                "@Injectable()\nclass ConfigService {\n  constructor(\n    @Inject(CONFIG_TOKEN) private config: AppConfig,\n  ) {}\n}\n",
                vec![(4, 5)],
            ),
            (
                "@Injectable()\nclass ConfigService {\n  constructor(\n    @Optional() private logger?: LoggerService,\n  ) {}\n}\n",
                vec![(4, 5)],
            ),
            (
                "@Injectable()\nclass ConfigService {\n  constructor(\n    @Self() control: NgControl,\n    @SkipSelf() skip: string,\n    @Host() host: unknown,\n  ) {}\n}\n",
                vec![(4, 5), (5, 5), (6, 5)],
            ),
            (
                "@Component({})\nclass MyComponent extends BaseComponent {\n  constructor(\n    private service: MyService,\n  ) {\n    super();\n  }\n}\n",
                vec![(4, 5)],
            ),
            (
                "@Component({})\nclass MyComponent {\n  constructor(elementRef: ElementRef) {}\n}\n",
                vec![(3, 15)],
            ),
        ] {
            assert_eq!(check(code), places, "{code}");
        }
    }
}
