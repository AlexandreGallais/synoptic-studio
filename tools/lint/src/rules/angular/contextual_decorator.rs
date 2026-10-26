//! `@angular-eslint/contextual-decorator`: a member's Angular decorator only in a class Angular
//! reads it in, `@Input()` in a component or a directive, never in a pipe nor an injectable
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests). A constructor's parameter is
//! read when it is a parameter property, as there.

use oxc_ast::AstKind;
use oxc_ast::ast::{Decorator, MethodDefinitionKind};
use oxc_semantic::AstNode;

use super::classes::{angular_decorator, decorator_name, enclosing_class};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct ContextualDecorator;

/// The decorators of a class's members (`AngularInnerClassDecorators`).
const INNER_DECORATORS: [&str; 14] = [
    "Attribute",
    "ContentChild",
    "ContentChildren",
    "Host",
    "HostBinding",
    "HostListener",
    "Inject",
    "Input",
    "Optional",
    "Output",
    "Self",
    "SkipSelf",
    "ViewChild",
    "ViewChildren",
];

/// The member decorators every class Angular creates reads (`ANGULAR_CLASS_DECORATOR_MAPPER`
/// but for components and directives, which read them all).
const INJECTION_DECORATORS: [&str; 5] = ["Host", "Inject", "Optional", "Self", "SkipSelf"];

/// The decorators of a member, a parameter property's included.
fn member_decorators<'b, 'a>(kind: &'b AstKind<'a>) -> Option<&'b [Decorator<'a>]> {
    match kind {
        AstKind::MethodDefinition(method) if method.kind != MethodDefinitionKind::Constructor => {
            Some(&method.decorators)
        }
        AstKind::PropertyDefinition(property) => Some(&property.decorators),
        AstKind::FormalParameter(parameter) if parameter.has_modifier() => {
            Some(&parameter.decorators)
        }
        _ => None,
    }
}

impl Rule for ContextualDecorator {
    fn name(&self) -> &'static str {
        "@angular-eslint/contextual-decorator"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let kind = node.kind();
        let Some(decorators) = member_decorators(&kind).filter(|each| !each.is_empty()) else {
            return;
        };
        let Some(class_kind) = enclosing_class(source, node.id()).and_then(angular_decorator)
        else {
            return;
        };
        if matches!(class_kind, "Component" | "Directive") {
            return;
        }
        for decorator in decorators {
            let Some(name) = decorator_name(decorator) else {
                continue;
            };
            if INNER_DECORATORS.contains(&name) && !INJECTION_DECORATORS.contains(&name) {
                findings.push(source.finding(
                    self.name(),
                    decorator.span.start,
                    format!("Decorator out of context for \"@{class_kind}()\""),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContextualDecorator;
    use crate::testing::lint;

    /// The place and class kind of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&ContextualDecorator, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                let kind = each
                    .message
                    .strip_prefix("Decorator out of context for \"@")
                    .and_then(|rest| rest.strip_suffix("()\""))
                    .unwrap_or_default()
                    .to_owned();
                (each.line, each.column, kind)
            })
            .collect()
    }

    /// Every member decorator on a class of a decorator, one member a line from line 3.
    fn every_member(decorator: &str) -> String {
        format!(
            "@{decorator}({{}})\nclass Test {{\n  @ContentChild(Pane) pane: Pane;\n  @Input()\n  get label(): string {{\n    return '';\n  }}\n  @ViewChild(Pane)\n  set label(value: Pane) {{}}\n  constructor(\n    @Attribute('test') private readonly test: string,\n    @Inject(LOCALE_ID) localeId: string,\n    @Host() @Optional() private readonly host: DynamicHost,\n  ) {{}}\n  @HostListener('mouseover')\n  mouseOver() {{}}\n  @HostBinding('class.card') private isCard: boolean;\n  @Output() emitter = new EventEmitter<void>();\n  @ViewChildren(Pane) panes: QueryList<Pane>;\n  @ContentChildren(Pane) others: QueryList<Pane>;\n}}\n"
        )
    }

    #[test]
    fn f51_ac2_accepts_member_decorators_where_angular_reads_them() {
        // The plugin's valid cases: every one in a component or a directive, the injection ones
        // anywhere, none read in a class Angular does not know.
        for decorator in ["Component", "Directive"] {
            assert_eq!(check(&every_member(decorator)), [], "{decorator}");
        }
        for code in [
            "@Injectable()\nclass Test {\n  constructor(\n    @Inject(LOCALE_ID) private readonly localeId: string,\n    @Optional() @Self() public readonly test: Test,\n    @SkipSelf() @Host() protected readonly parent: Parent,\n  ) {}\n}\n",
            "class Test {\n  @Input() label: string;\n}\n",
            "@Pipe({})\nclass Test {\n  @Custom() label: string;\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_decorator_out_of_its_class_at_the_decorator() {
        // The plugin's invalid cases: each member's decorator, but `@Inject` on a plain parameter
        // and the injection ones; `    @Host() ` puts nothing found on line 13.
        for decorator in ["Injectable", "NgModule", "Pipe", "Service"] {
            let found: Vec<(usize, usize, String)> = [
                (3, 3),
                (4, 3),
                (8, 3),
                (11, 5),
                (15, 3),
                (17, 3),
                (18, 3),
                (19, 3),
                (20, 3),
            ]
            .into_iter()
            .map(|(line, column)| (line, column, decorator.to_owned()))
            .collect();

            assert_eq!(check(&every_member(decorator)), found, "{decorator}");
        }
        // Two classes of a file, the directive's input left.
        assert_eq!(
            check(
                "@Directive({ selector: 'test' })\nclass TestDirective {\n  @Input() label: string;\n}\n@Pipe({ name: 'test' })\nclass Test {\n  @Input() label: string;\n}\n"
            ),
            [(7, 3, "Pipe".to_owned())]
        );
    }
}
