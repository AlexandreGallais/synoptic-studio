//! `@angular-eslint/no-implicit-take-until-destroyed`: `takeUntilDestroyed()` without a
//! `DestroyRef` only in an injection context, the constructor or a field of a class Angular
//! creates (not an `NgModule`), a method only they call, or a provider's or a token's factory
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests).

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, MethodDefinition, MethodDefinitionKind, PropertyKey};
use oxc_semantic::{AstNode, NodeId};

use super::classes::{angular_decorator, enclosing_class};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::rules::unicorn::syntax::descendants;
use crate::source::Source;

/// The rule.
pub struct NoImplicitTakeUntilDestroyed;

/// The plugin's message.
const MESSAGE: &str = "`takeUntilDestroyed()` must be called with an explicit `DestroyRef` parameter when used outside of an injection context. See more at https://angular.dev/guide/di/dependency-injection-context and https://angular.dev/ecosystem/rxjs-interop/take-until-destroyed";

/// Whether a node is a function given as a provider's `useFactory` or a token's `factory`.
fn is_factory(source: &Source, node: &AstNode) -> bool {
    let function = match node.kind() {
        AstKind::Function(function) => function.is_expression(),
        AstKind::ArrowFunctionExpression(_) => true,
        _ => false,
    };
    function
        && matches!(source.semantic.nodes().parent_kind(node.id()), AstKind::ObjectProperty(property)
            if matches!(&property.key, PropertyKey::StaticIdentifier(key) if key.name == "factory" || key.name == "useFactory"))
}

/// Whether the class around a node is one Angular creates in an injection context: decorated,
/// but not an `NgModule`.
fn in_injected_class(source: &Source, id: NodeId) -> bool {
    enclosing_class(source, id)
        .and_then(angular_decorator)
        .is_some_and(|decorator| decorator != "NgModule")
}

/// A method's name and whether it is private (`#name`).
fn method_name<'a>(method: &MethodDefinition<'a>) -> Option<(&'a str, bool)> {
    match &method.key {
        PropertyKey::StaticIdentifier(key) => Some((key.name.as_str(), false)),
        PropertyKey::PrivateIdentifier(key) => Some((key.name.as_str(), true)),
        _ => None,
    }
}

/// Whether a node holds a call of `this.<name>()` or `this.#<name>()`.
fn calls_method(source: &Source, id: NodeId, name: &str, private: bool) -> bool {
    let node = source.semantic.nodes().get_node(id);
    std::iter::once(node)
        .chain(descendants(source, id))
        .any(|each| match each.kind() {
            AstKind::CallExpression(call) => match &call.callee {
                Expression::StaticMemberExpression(member) => {
                    !private
                        && matches!(member.object, Expression::ThisExpression(_))
                        && member.property.name == name
                }
                Expression::PrivateFieldExpression(member) => {
                    private
                        && matches!(member.object, Expression::ThisExpression(_))
                        && member.field.name == name
                }
                _ => false,
            },
            _ => false,
        })
}

/// Whether a method is called only from the class's constructor or field initializers, at least
/// once (`isMethodCalledFromInjectionContext`); its calls of itself left out.
fn called_from_injection_context(
    source: &Source,
    method_id: NodeId,
    method: &MethodDefinition,
) -> bool {
    let Some((name, private)) = method_name(method) else {
        return false;
    };
    let nodes = source.semantic.nodes();
    let body = nodes.parent_id(method_id);
    let members = descendants(source, body).filter(|each| nodes.parent_id(each.id()) == body);
    let mut called = false;
    for member in members {
        if member.id() == method_id || !calls_method(source, member.id(), name, private) {
            continue;
        }
        match member.kind() {
            AstKind::MethodDefinition(other) if other.kind == MethodDefinitionKind::Constructor => {
                called = true;
            }
            AstKind::PropertyDefinition(_) => called = true,
            _ => return false,
        }
    }
    called
}

/// Whether a call runs in an injection context (`isInInjectionContext`).
fn in_injection_context(source: &Source, id: NodeId) -> bool {
    let nodes = source.semantic.nodes();
    std::iter::once(nodes.get_node(id))
        .chain(nodes.ancestors(id))
        .any(|current| {
            if is_factory(source, current) {
                return true;
            }
            match current.kind() {
                AstKind::PropertyDefinition(_) => in_injected_class(source, current.id()),
                AstKind::MethodDefinition(method)
                    if method.kind == MethodDefinitionKind::Constructor =>
                {
                    in_injected_class(source, current.id())
                }
                AstKind::MethodDefinition(method) => {
                    in_injected_class(source, current.id())
                        && called_from_injection_context(source, current.id(), method)
                }
                _ => false,
            }
        })
}

impl Rule for NoImplicitTakeUntilDestroyed {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-implicit-take-until-destroyed"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::CallExpression(call) = node.kind() else {
            return;
        };
        let implicit = matches!(&call.callee, Expression::Identifier(callee) if callee.name == "takeUntilDestroyed")
            && call.arguments.is_empty();
        if implicit && !in_injection_context(source, node.id()) {
            findings.push(source.finding(self.name(), call.span.start, MESSAGE.into()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MESSAGE, NoImplicitTakeUntilDestroyed};
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(
            &NoImplicitTakeUntilDestroyed,
            "playground/src/app/a.ts",
            code,
        )
        .into_iter()
        .map(|each| {
            assert_eq!(each.message, MESSAGE);
            (each.line, each.column)
        })
        .collect()
    }

    /// A class of a decorator with members, one a line, indented by two spaces.
    fn class(decorator: &str, members: &[&str]) -> String {
        format!(
            "{decorator}\nclass Test {{\n  {}\n}}\n",
            members.join("\n  ")
        )
    }

    /// The subscription the plugin's cases write in a constructor or a method.
    const SUBSCRIBE: &str = "  this.data$.pipe(takeUntilDestroyed()).subscribe();";

    #[test]
    fn f51_ac1_accepts_injection_contexts_and_an_explicit_destroy_ref() {
        let mut codes: Vec<String> = [
            "@Component()",
            "@Directive()",
            "@Injectable()",
            "@Pipe({ name: 'myPipe' })",
            "@Service()",
        ]
        .iter()
        .flat_map(|decorator| {
            [
                class(decorator, &["constructor() {", SUBSCRIBE, "}"]),
                class(
                    decorator,
                    &["private data = this.http.get('/api').pipe(takeUntilDestroyed());"],
                ),
            ]
        })
        .collect();
        codes.extend([
            "const MY_TOKEN = new InjectionToken('my-token', {\n  factory: () => {\n    return someObservable.pipe(takeUntilDestroyed());\n  }\n});\n".to_owned(),
            class("@Component()", &["ngOnInit() {", "  this.data$.pipe(takeUntilDestroyed(this.destroyRef)).subscribe();", "}"]),
            class("@Injectable()", &["loadData() {", "  this.data$.pipe(takeUntilDestroyed(this.destroyRef)).subscribe();", "}"]),
            "@Component({\n  providers: [\n    {\n      provide: SomeToken,\n      useFactory: () => data$.pipe(takeUntilDestroyed())\n    }\n  ]\n})\nclass Test {}\n".to_owned(),
            class("@Component()", &["#myObservable = new Subject();", "constructor() {", "  this.#someLogic();", "}", "#someLogic() {", "  this.#myObservable.pipe(takeUntilDestroyed()).subscribe();", "}"]),
            class("@Component()", &["constructor() {", "  this.initSubscriptions();", "}", "private initSubscriptions() {", SUBSCRIBE, "}"]),
            class("@Directive()", &["constructor() {", "  this.setup();", "}", "private setup() {", SUBSCRIBE, "}"]),
            class("@Component()", &["private data = this.init();", "constructor() {", "  this.init();", "}", "private init() {", "  return this.data$.pipe(takeUntilDestroyed());", "}"]),
            class("@Component()", &["constructor() {", "  this.init(0);", "}", "private init(attempt: number) {", "  if (attempt < 3) {", "    this.init(attempt + 1);", "  }", SUBSCRIBE, "}"]),
        ]);
        for code in codes {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_an_implicit_destroy_ref_outside_them_at_the_call() {
        // In a method's body, `    this.data$.pipe(` is 20 columns: the call at 21, on the line
        // after the method's (line 4 for a first member).
        for decorator in ["@Component()", "@Directive()", "@Injectable()"] {
            for method in [
                "ngOnInit() {",
                "ngAfterViewInit() {",
                "anyMethod() {",
                "ngOnDestroy() {",
            ] {
                let code = class(decorator, &[method, SUBSCRIBE, "}"]);
                assert_eq!(check(&code), [(4, 21)], "{code}");
            }
        }
        // A constructor of a class Angular does not create, or of an NgModule.
        for decorator in ["// plain", "@NgModule()"] {
            let code = class(decorator, &["constructor() {", SUBSCRIBE, "}"]);
            assert_eq!(check(&code), [(4, 21)], "{code}");
        }
        // An NgModule's field: `  private data = this.http.get('/api').pipe(` is 44 columns.
        let code = class(
            "@NgModule()",
            &["private data = this.http.get('/api').pipe(takeUntilDestroyed());"],
        );
        assert_eq!(check(&code), [(3, 45)], "{code}");
        // A method called from a hook, alone or beside the constructor or a field: the members
        // start on line 3, one a line; `    return this.data$.pipe(` is 27 columns.
        let alone = class(
            "@Component()",
            &[
                "constructor() {}",
                "",
                "ngOnInit() {",
                "  this.loadData();",
                "}",
                "",
                "loadData() {",
                SUBSCRIBE,
                "}",
            ],
        );
        assert_eq!(check(&alone), [(10, 21)], "{alone}");
        let both = class(
            "@Component()",
            &[
                "constructor() {",
                "  this.loadData();",
                "}",
                "",
                "ngOnInit() {",
                "  this.loadData();",
                "}",
                "",
                "loadData() {",
                SUBSCRIBE,
                "}",
            ],
        );
        assert_eq!(check(&both), [(12, 21)], "{both}");
        let field = class(
            "@Component()",
            &[
                "private data = this.loadData();",
                "",
                "ngOnInit() {",
                "  this.loadData();",
                "}",
                "",
                "private loadData() {",
                "  return this.data$.pipe(takeUntilDestroyed());",
                "}",
            ],
        );
        assert_eq!(check(&field), [(10, 28)], "{field}");
        let private = class(
            "@Injectable()",
            &[
                "constructor() {",
                "  this.#subscribe();",
                "}",
                "",
                "refresh() {",
                "  this.#subscribe();",
                "}",
                "",
                "#subscribe() {",
                SUBSCRIBE,
                "}",
            ],
        );
        assert_eq!(check(&private), [(12, 21)], "{private}");
    }
}
