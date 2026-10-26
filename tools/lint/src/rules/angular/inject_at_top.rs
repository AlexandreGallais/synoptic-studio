//! `@angular-eslint/inject-at-top`: a class Angular creates sets its `inject()` fields before any
//! other member, since fields are set in their written order (`@angular-eslint/eslint-plugin`
//! 22.6.0, the cases of its tests), without `additionalInjectFunctions`. The class's own
//! decorators are read, the plugin's `:has` reading any under the class.

use oxc_ast::AstKind;
use oxc_ast::ast::{
    ArrowFunctionExpression, CallExpression, Class, ClassElement, Expression, Function,
};
use oxc_ast_visit::{Visit, walk};
use oxc_semantic::{AstNode, ScopeFlags};

use super::metadata::class_decorator_of;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct InjectAtTop;

/// The decorators of a class Angular creates.
const CREATED: [&str; 5] = ["Component", "Directive", "Injectable", "Pipe", "Service"];

/// The search for an `inject()` run when a field is set (`findEagerInjectName`): a function or a
/// class is entered only when called at once.
#[derive(Default)]
struct Eager {
    /// Whether one was found.
    found: bool,
}

impl<'a> Visit<'a> for Eager {
    fn visit_call_expression(&mut self, call: &CallExpression<'a>) {
        match &call.callee {
            Expression::Identifier(callee) if callee.name == "inject" => self.found = true,
            Expression::ArrowFunctionExpression(arrow) => {
                walk::walk_arrow_function_expression(self, arrow);
                for argument in &call.arguments {
                    self.visit_argument(argument);
                }
            }
            Expression::FunctionExpression(function) => {
                walk::walk_function(self, function, ScopeFlags::Function);
                for argument in &call.arguments {
                    self.visit_argument(argument);
                }
            }
            _ => walk::walk_call_expression(self, call),
        }
    }

    fn visit_arrow_function_expression(&mut self, _arrow: &ArrowFunctionExpression<'a>) {}

    fn visit_function(&mut self, _function: &Function<'a>, _flags: ScopeFlags) {}

    fn visit_class(&mut self, _class: &Class<'a>) {}
}

/// Whether setting a field runs `inject()`.
fn injects(value: &Expression) -> bool {
    let mut eager = Eager::default();
    eager.visit_expression(value);
    eager.found
}

impl Rule for InjectAtTop {
    fn name(&self) -> &'static str {
        "@angular-eslint/inject-at-top"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        if class_decorator_of(class, &CREATED).is_none() {
            return;
        }
        let mut seen_other = false;
        for element in &class.body.body {
            if element.r#static() {
                continue;
            }
            if let ClassElement::PropertyDefinition(property) = element
                && property.value.as_ref().is_some_and(injects)
            {
                if seen_other {
                    findings.push(source.finding(
                        self.name(),
                        property.span.start,
                        "Move this inject() to the top of the class. Class fields are initialized in the order they are written, so anything declared above this line cannot safely use the service yet.".into(),
                    ));
                }
                continue;
            }
            seen_other = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InjectAtTop;
    use crate::testing::lint;

    /// The lines of the findings on a class body, each found at column 3.
    fn check(decorator: &str, body: &str) -> Vec<usize> {
        let code = format!("{decorator}\nclass Test {{\n{body}}}\n");
        lint(&InjectAtTop, "playground/src/app/a.ts", &code)
            .into_iter()
            .map(|each| {
                assert!(each.message.starts_with("Move this inject() to the top"));
                assert_eq!(each.column, 3, "{code}");
                each.line
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_inject_fields_first_and_lazy_ones_anywhere() {
        // The plugin's valid cases, the custom inject functions apart.
        assert_eq!(
            check(
                "",
                "  private get userName() {\n    return this.userService.name;\n  }\n  private readonly userService = inject(UserService);\n"
            ),
            Vec::<usize>::new()
        );
        for body in [
            "  private readonly userService = inject(UserService);\n  protected readonly pageTitle = `Dashboard ${this.fullName}`;\n  private get fullName() {\n    return '';\n  }\n",
            "  private readonly http = inject(HttpClient);\n  private readonly router = inject(Router);\n  doThing() {}\n",
            "  private value = 42;\n  private get doubled() {\n    return this.value * 2;\n  }\n",
            "  private readonly optional = inject(SOME_TOKEN, { optional: true });\n  foo() {}\n",
            "  static readonly DEFAULT = 'default';\n  private readonly myService = inject(MyService);\n",
            "  private readonly count = 0;\n  private readonly userServiceFactory = () => inject(UserService);\n",
            "  private value = 0;\n  build() {\n    return inject(HttpClient);\n  }\n",
            "  private flag = false;\n  private readonly factory = function () { return inject(UserService); };\n",
            "  private readonly strategy = cond ? inject(Primary) : inject(Fallback);\n  private count = 0;\n",
            "  private readonly logger = inject(LoggerService, { optional: true }) ?? noopLogger;\n  private count = 0;\n",
            "  private readonly config = inject(AppConfig, { optional: true })!;\n  private count = 0;\n",
            "  private readonly flags = inject(FlagsService) as FeatureFlags;\n  private count = 0;\n",
            "  private readonly traced = withTracing(inject(HttpClient));\n  private count = 0;\n",
            "  private readonly client = new ApiClient(inject(HttpClient));\n  private count = 0;\n",
            "  private readonly handlers = [inject(HandlerA), inject(HandlerB)];\n  private count = 0;\n",
            "  private readonly deps = { users: inject(UserService) };\n  private count = 0;\n",
            "  private readonly userService = (audit(), inject(UserService));\n  private count = 0;\n",
            "  private readonly id = `${inject(UserService).id}`;\n  private count = 0;\n",
            "  private readonly client = (() => inject(ApiClient))();\n  private count = 0;\n",
            "  private count = 0;\n  private readonly factory = () => (cond ? inject(A) : inject(B));\n",
            "  private count = 0;\n  private readonly factory = withFactory(() => inject(UserService));\n",
            "  private count = 0;\n  private readonly client = new ApiClient(function () { return inject(HttpClient); });\n",
            "  private count = 0;\n  private readonly factory = () => collect([inject(A), inject(B)]);\n",
            "  private count = 0;\n  private readonly t = injectTranslations();\n",
            // A class expression's `inject()` runs when it is created, not when the field is set.
            "  private count = 0;\n  private readonly type = class { a = inject(A); };\n",
        ] {
            assert_eq!(check("@Component({})", body), Vec::<usize>::new(), "{body}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_inject_field_below_another_member_at_the_field() {
        // The plugin's invalid cases: the class body starts on line 3.
        for (decorator, body, lines) in [
            (
                "@Component({})",
                "  protected readonly pageTitle = `Dashboard ${this.fullName}`;\n  private get fullName() {\n    return '';\n  }\n  private readonly userService = inject(UserService);\n",
                vec![7],
            ),
            (
                "@Injectable()",
                "  private count = 0;\n  private readonly http = inject(HttpClient);\n",
                vec![4],
            ),
            (
                "@Directive({})",
                "  doThing() {}\n  private readonly myService = inject(MyService);\n",
                vec![4],
            ),
            (
                "@Component({})",
                "  constructor() {}\n  private readonly myService = inject(MyService);\n",
                vec![4],
            ),
            (
                "@Injectable()",
                "  private readonly http = inject(HttpClient);\n  private count = 0;\n  private readonly router = inject(Router);\n",
                vec![5],
            ),
            (
                "@Component({})",
                "  private count = 0;\n  private readonly a = inject(A);\n  private readonly b = inject(B);\n",
                vec![4, 5],
            ),
            (
                "@Service()",
                "  private get baseUrl() {\n    return this.http.baseUrl;\n  }\n  private readonly http = inject(HttpClient);\n",
                vec![6],
            ),
        ] {
            assert_eq!(check(decorator, body), lines, "{body}");
        }
        // Every wrapping shape the plugin lists, the immediately called functions among them.
        for value in [
            "cond ? inject(A) : inject(B)",
            "inject(A, { optional: true }) ?? noop",
            "inject(A, { optional: true })!",
            "inject(A) as B",
            "withTracing(inject(A))",
            "new ApiClient(inject(A))",
            "[inject(A), inject(B)]",
            "{ users: inject(A) }",
            "(audit(), inject(A))",
            "`${inject(A).id}`",
            "(() => inject(A))()",
            "collect([inject(A), { router: inject(B) }])",
            "(function () { return inject(A); })()",
            "(() => withTracing(inject(A)))()",
        ] {
            let body = format!("  private count = 0;\n  private readonly a = {value};\n");

            assert_eq!(check("@Component({})", &body), [4], "{value}");
        }
    }
}
