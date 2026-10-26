//! `@angular-eslint/no-input-rename`: an input is bound by its own name, never an alias, but the
//! directive's own selector and `aria-` attributes (`@angular-eslint/eslint-plugin` 22.6.0, the
//! cases of its tests), with its default options. The plugin's list of ARIA attributes comes from
//! `aria-query`; any alias starting `aria-` stands for it here. The plugin's removal of an alias
//! is not carried.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::bindings::{
    Direction, alias_allowed, aliases, class_selectors, entry_parts, member_identifier, members,
    metadata_entries,
};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct NoInputRename;

/// A kebab-case name in camelCase (`kebabToCamelCase`): `aria-label` is `ariaLabel`.
fn camel_case(name: &str) -> String {
    let mut result = String::new();
    let mut characters = name.chars().peekable();
    while let Some(character) = characters.next() {
        match characters.peek() {
            Some(next) if character == '-' && next.is_ascii_alphabetic() => {
                result.push(next.to_ascii_uppercase());
                characters.next();
            }
            _ => result.push(character),
        }
    }
    result
}

/// Whether an alias is an ARIA attribute bound to the property of its camelCase name.
fn is_aria(property: &str, alias: &str) -> bool {
    alias.starts_with("aria-") && camel_case(alias) == property
}

impl Rule for NoInputRename {
    fn name(&self) -> &'static str {
        "@angular-eslint/no-input-rename"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let selectors = class_selectors(class);
        let refused = |property: &str, alias: &str| {
            !is_aria(property, alias)
                && (alias == property
                    || !alias_allowed(&selectors, property, alias, Direction::Input))
        };
        let mut found: Vec<u32> = Vec::new();
        for entry in metadata_entries(class, Direction::Input) {
            let (property, alias) = entry_parts(entry.text, true);
            if alias.is_some_and(|alias| refused(&property, &alias)) {
                found.push(entry.at);
            }
        }
        for member in members(class, Direction::Input) {
            let Some(property) = member_identifier(&member) else {
                continue;
            };
            for alias in aliases(&member, Direction::Input) {
                if refused(property, alias.text) {
                    found.push(alias.at);
                }
            }
        }
        for at in found {
            findings.push(source.finding(
                self.name(),
                at,
                "Input bindings should not be aliased (https://angular.dev/guide/components/inputs#choosing-input-names)".into(),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NoInputRename;
    use crate::testing::lint;

    /// The place of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize)> {
        lint(&NoInputRename, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| {
                assert!(
                    each.message
                        .starts_with("Input bindings should not be aliased")
                );
                (each.line, each.column)
            })
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_inputs_by_their_own_name_and_the_selector_s() {
        // The plugin's valid cases with its default options.
        for code in [
            "class Test {}\n",
            "@Page({\n  inputs: ['play', popstate, `online`, 'obsolete: obsol', 'store: storage'],\n})\nclass Test {}\n",
            "@Directive()\nclass Test {\n  @Input() buttonChange = new EventEmitter<'change'>();\n}\n",
            "@Directive()\nclass Test {\n  buttonChange = input(1);\n}\n",
            "@Component({\n  inputs,\n})\nclass Test {}\n",
            "@Directive({\n  inputs: [func(), 'a'],\n})\nclass Test {}\n",
            "@Component({\n  selector: 'qx-menuitem',\n  hostDirectives: [{\n    directive: CdkMenuItem,\n    inputs: ['cdkMenuItemDisabled: disabled'],\n  }]\n})\nclass Test {}\n",
            "@Component({})\nclass Test {\n  @Input() set setter(setter: string) {}\n}\n",
            "const change = 'change';\n@Component()\nclass Test {\n  touchMove = input(1, { alias: change });\n}\n",
            "const blur = 'blur';\nconst click = 'click';\n@Directive()\nclass Test {\n  @Input(blur) [click]: EventEmitter<Blur>;\n}\n",
            "@Directive({\n  'selector': 'foo',\n  'inputs': [`test: foo`]\n})\nclass Test {}\n",
            "@Component({\n  selector: '[foo], test',\n})\nclass Test {\n  @Input('foo') label: string;\n}\n",
            "@Component({\n  selector: '[foo], test',\n})\nclass Test {\n  label = input(1, { alias: 'foo' });\n}\n",
            "@Directive({\n  selector: 'foo'\n})\nclass Test {\n  @Input('aria-label') ariaLabel: string;\n}\n",
            "@Directive({\n  selector: 'foo'\n})\nclass Test {\n  ariaLabel = input(1, { alias: 'aria-label' });\n}\n",
            "@Directive({\n  selector: 'foo'\n})\nclass Test {\n  @Input('fooMyColor') myColor: string;\n}\n",
            "@Directive({\n  selector: 'img[fooDirective]'\n})\nclass Test {\n  @Input('fooDirective') foo: Foo;\n}\n",
            "@Directive({\n  selector: 'img[fooDirective]'\n})\nclass Test {\n  foo = input.required<number>({ alias: 'fooDirective' });\n}\n",
            "@Component({\n  selector: 'foo'\n})\nclass Test {\n  @Input({ required: true }) name: string;\n}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_each_alias_at_its_text() {
        // The plugin's invalid cases: the alias's quote.
        for (code, place) in [
            (
                "@Component({\n  inputs: ['a: b']\n})\nclass Test {}\n",
                (2, 12),
            ),
            (
                "@Component({\n  ['inputs']: ['orientation: orientation'],\n})\nclass Test {}\n",
                (2, 16),
            ),
            (
                "@Component()\nclass Test {\n  @Custom() @Input(`change`) _change = getInput();\n}\n",
                (3, 20),
            ),
            (
                "@Component()\nclass Test {\n  _change = input(1, { alias: `change` });\n}\n",
                (3, 31),
            ),
            (
                "@Component()\nclass Test {\n  _change = input.required<number>({ alias: `change` });\n}\n",
                (3, 45),
            ),
            (
                "@Directive()\nclass Test {\n  @Input('change') change = 1;\n}\n",
                (3, 10),
            ),
            (
                "@Directive()\nclass Test {\n  change = input(1, { alias: 'change' });\n}\n",
                (3, 30),
            ),
            (
                "@Directive()\nclass Test {\n  @Input({ alias: 'change' }) change = 1;\n}\n",
                (3, 19),
            ),
            (
                "@Component()\nclass Test {\n  @Input(`devicechange`) set setter(setter: string) {}\n}\n",
                (3, 10),
            ),
        ] {
            assert_eq!(check(code), [place], "{code}");
        }
    }
}
