//! `@angular-eslint/no-uncalled-signals`: a signal is called before a test, a logical or binary
//! operator, a unary operator but `delete`, or a read of a function's own member (`length`,
//! `name`…), where the signal itself, a function, is never the value meant
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests). The plugin asks TypeScript
//! for the type; here a value is a signal when the file shows it (`signals::is_signal`). The
//! plugin's call added after the signal is a suggestion, never applied by `--fix`: none here.

use oxc_ast::AstKind;
use oxc_ast::ast::{Expression, UnaryOperator};
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::signals::is_signal;
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoUncalledSignals;

/// The members of a function the plugin reads as a sign of a signal taken for its value.
const FUNCTION_MEMBERS: [&str; 5] = ["arguments", "caller", "length", "name", "toString"];

/// The plugin's message.
const MESSAGE: &str = "Doing logic operations on signals will give unexpected results, you probably want to invoke the signal to get its value";

impl Rule for NoUncalledSignals {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-uncalled-signals"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let checked: Vec<&Expression> = match node.kind() {
            AstKind::ConditionalExpression(conditional) => vec![&conditional.test],
            AstKind::DoWhileStatement(statement) => vec![&statement.test],
            AstKind::ForStatement(statement) => statement.test.iter().collect(),
            AstKind::IfStatement(statement) => vec![&statement.test],
            AstKind::SwitchCase(case) => case.test.iter().collect(),
            AstKind::WhileStatement(statement) => vec![&statement.test],
            AstKind::UnaryExpression(unary) if unary.operator != UnaryOperator::Delete => {
                vec![&unary.argument]
            }
            AstKind::LogicalExpression(logical) => vec![&logical.left, &logical.right],
            AstKind::BinaryExpression(binary) => vec![&binary.left, &binary.right],
            AstKind::StaticMemberExpression(member)
                if FUNCTION_MEMBERS.contains(&member.property.name.as_str()) =>
            {
                vec![&member.object]
            }
            _ => return,
        };
        for expression in checked {
            if is_signal(source, node.id(), expression) {
                findings.push(source.finding(self.name(), expression.span().start, MESSAGE.into()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MESSAGE, NoUncalledSignals};
    use crate::testing::lint;

    /// The plugin's first line, on which each case's code goes on: the types it names, imported.
    const IMPORT: &str = "import { effect, InputSignal, InputSignalWithTransform, ModelSignal, signal, Signal, WritableSignal } from \"@angular/core\";";

    /// The place of each finding on a piece of code written after the import.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(
            &NoUncalledSignals,
            "playground/src/app/a.ts",
            &format!("{IMPORT}\n{code}"),
        )
        .into_iter()
        .map(|each| {
            assert_eq!(each.message, MESSAGE);
            (each.line - 1, each.column)
        })
        .collect()
    }

    #[test]
    fn f51_ac1_accepts_signals_called_and_values_not_seen_as_signals() {
        // The plugin's valid cases.
        let mut codes: Vec<String> = [
            "let a = 1;\nif (a) { }",
            "let a = true;\nif (!a) { }",
            "let a: Signal<boolean>;\nlet b = a() ? 1 : 2;",
            "let a: Signal<boolean>;\ndo {} while (a());",
            "let a: Signal<boolean>;\nfor (let i = 0; a(); i++) { }",
            "let a: Signal<boolean>;\nfor (;;) { }",
            "let a: Signal<boolean>;\nif (a()) { }",
            "let a: Signal<boolean>;\nswitch (true) {\n  case a():\n    break;\n  default:\n    break;\n}",
            "let a: Signal<boolean>;\nwhile (a()) { }",
            "let a: Signal<boolean>;\nif (!a()) { }",
            "let a: Signal<boolean>;\nif (a() || true) { }",
            "let a: Signal<boolean>;\nif (a() == \"hello\") { }",
            "let a: Signal<boolean>;\nif (false || (a() ?? true)) { }",
            "let a: Signal<boolean>;\nif (false) {\n  a\n}",
            "let a: Signal<boolean> | null;\nif (a) { }",
            "let a: Signal<boolean> | undefined;\nif (a) { }",
            "let a: Signal<boolean> | NonSignal;\nif (a) { }\ninterface NonSignal {}",
            "let a: Signal<boolean>;\nlet b = a() ?? true;",
            "let a: Signal<boolean>;\nconst b = a() || true;",
            "let a: Signal<boolean>;\nlet b = a;",
            "let a: Signal<boolean>;\nlet b = !a();",
            "let a: Signal<number>;\nlet b = -a();",
            "class AppComponent {\n  prop = viewChild<AppComponent | undefined>();\n  test() {\n    delete this.prop;\n  }\n}",
            "function test(): boolean {\n  let a: Signal<boolean>;\n  return !a();\n}",
            "function getSignal(): Signal<boolean> {}\nif (getSignal()()) { }",
            "let a: Signal<number[]>;\nlet b = a.prototype;",
            "export class AppComponent {\n  readonly test: Signal<boolean>;\n  constructor() {\n    effect(() => {\n      if (this.test()) { }\n    });\n  }\n}",
            "export class AppComponent {\n  readonly test: Signal<boolean>;\n  constructor() {\n    const t = this.test;\n    effect(() => {\n      if (t()) { }\n    });\n  }\n}",
            "let a: WritableSignal<string>;\nlet b: boolean;\nlet c = b && a.set('');",
            "let a: { b: WritableSignal<boolean> };\ntrue && a.b.set(false);",
            "function a(b: WritableSignal<boolean> | InputSignal<boolean>) {\n  return 'set' in b || 'applyValueToInputSignal' in b[SIGNAL];\n}",
            "function a(b: { c: WritableSignal<boolean> }) {\n  true && ((x) => b.c.set(x))(true);\n}",
        ]
        .map(str::to_owned)
        .to_vec();
        codes.extend(
            ["arguments", "caller", "length", "name", "toString()"]
                .map(|member| format!("let a: Signal<number[]>;\nlet b = a().{member};")),
        );
        // Left out, the type only TypeScript reads: a signal deep in an object's written type.
        codes.push("let a: { b: { c: { d: { e: Signal<number>; } } } }\nif (a.b.c.d.e) { }".into());
        // A local `computed` is not Angular's.
        codes.push("const computed = (x: number) => x;\nconst a = computed(1);\nif (a) { }".into());
        for code in codes {
            assert_eq!(check(&code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac1_reports_a_signal_taken_for_its_value_at_the_signal() {
        // The plugin's invalid cases, each on its second line (the first holds the declaration).
        for (code, column) in [
            ("let a: Signal<boolean>;\nlet b = a ? 1 : 2;", 9),
            ("let a: Signal<boolean>;\ndo {} while (a);", 14),
            ("let a: Signal<boolean>;\nfor (let i = 0; a; i++) { }", 17),
            ("let a: Signal<boolean>;\nconst flag = !a;", 15),
            ("let a: Signal<boolean>;\nif (a) { }", 5),
            ("let a: Signal<boolean>;\nwhile (a) {}", 8),
            ("let a: Signal<boolean>;\nif (!a) { }", 6),
            ("let a: Signal<number>;\nif (a === 1) { }", 5),
            ("let a: Signal<number>;\nif (1 === a) { }", 11),
            ("let a: Signal<number>;\nlet b = a || 1;", 9),
            ("let a: Signal<number>;\nlet b = 1 || a;", 14),
            (
                "let a: Signal<number>;\nlet b = 1 || 2 || 3 || a || 4 || 5;",
                24,
            ),
            (
                "function getSignal(): Signal<number> { }\nif (getSignal()) { }",
                5,
            ),
            ("let a: InputSignal<boolean>;\nif (a) { }", 5),
            (
                "let a: InputSignalWithTransform<boolean, boolean | string>;\nif (a) { }",
                5,
            ),
            ("let a: ModelSignal<boolean>;\nif (a) { }", 5),
            ("let a: WritableSignal<boolean>;\nif (a) { }", 5),
        ] {
            assert_eq!(check(code), [(2, column)], "{code}");
        }
        // In a function, after a blank line: `  return !` puts the signal at column 11, line 4.
        assert_eq!(
            check("function test(): boolean {\n  let a: Signal<boolean>;\n\n  return !a;\n}"),
            [(4, 11)]
        );
        // A switch case: `  case ` puts it at column 8, line 3.
        assert_eq!(
            check("let a: Signal<boolean>;\nswitch (true) {\n  case a:\n    break;\n}"),
            [(3, 8)]
        );
        // A function's own members read on the signal.
        for member in ["arguments", "caller", "length", "name", "toString()"] {
            let code = format!("let a: Signal<number[]>;\nlet b = a.{member};");
            assert_eq!(check(&code), [(2, 9)], "{code}");
        }
        // What the file shows beyond the plugin's cases: a signal made by a factory of Angular's,
        // read through `this` or through a constant.
        assert_eq!(check("const a = signal(0);\nif (a) { }"), [(2, 5)]);
        assert_eq!(
            check("class A {\n  readonly a = signal(0);\n  b() {\n    return !this.a;\n  }\n}"),
            [(4, 13)]
        );
        assert_eq!(
            check("const a = signal(0).asReadonly();\nconst b = a;\nif (b) { }"),
            [(3, 5)]
        );
    }
}
