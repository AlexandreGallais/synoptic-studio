//! `@angular-eslint/sort-keys-in-type-decorator`: the keys of a component's, a directive's, an
//! injectable's, a module's or a pipe's metadata in one order, `selector` first
//! (`@angular-eslint/eslint-plugin` 22.6.0, the cases of its tests), with its default orders and
//! `allowUnconfiguredProperties`; a key outside the order is left where it is, after them. The
//! plugin's sorting fix is not carried.

use oxc_ast::AstKind;
use oxc_semantic::AstNode;

use super::classes::key_identifier;
use super::metadata::{class_decorator_of, decorator_properties};
use crate::finding::Finding;
use crate::rules::Rule;
use crate::source::Source;

/// The rule.
pub struct SortKeysInTypeDecorator;

/// The order of a decorator's keys (`DEFAULT_ORDER`, after Angular's API pages).
fn order(decorator: &str) -> &'static [&'static str] {
    match decorator {
        "Component" => &[
            "selector",
            "imports",
            "standalone",
            "templateUrl",
            "template",
            "styleUrl",
            "styleUrls",
            "styles",
            "providers",
            "changeDetection",
            "encapsulation",
            "viewProviders",
            "host",
            "hostDirectives",
            "inputs",
            "outputs",
            "animations",
            "schemas",
            "exportAs",
            "queries",
            "preserveWhitespaces",
            "jit",
            "moduleId",
            "interpolation",
        ],
        "Directive" => &[
            "selector",
            "standalone",
            "providers",
            "host",
            "hostDirectives",
            "inputs",
            "outputs",
            "exportAs",
            "queries",
            "jit",
        ],
        "Injectable" => &[
            "providedIn",
            "useClass",
            "useExisting",
            "useFactory",
            "useValue",
            "deps",
        ],
        "NgModule" => &[
            "id",
            "imports",
            "declarations",
            "providers",
            "exports",
            "bootstrap",
            "schemas",
            "jit",
        ],
        _ => &["name", "standalone", "pure"],
    }
}

impl Rule for SortKeysInTypeDecorator {
    fn name(&self) -> &'static str {
        "@angular-eslint/sort-keys-in-type-decorator"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some((decorator, kind)) = class_decorator_of(
            class,
            &["Component", "Directive", "Injectable", "NgModule", "Pipe"],
        ) else {
            return;
        };
        let expected = order(kind);
        let properties = decorator_properties(decorator);
        if properties.len() <= 1 {
            return;
        }
        let names: Vec<Option<&str>> = properties
            .iter()
            .map(|property| key_identifier(&property.key))
            .collect();
        let configured = |name: Option<&str>| name.is_some_and(|name| expected.contains(&name));
        let first_configured = names.iter().position(|name| configured(*name));
        let first_other = names.iter().position(|name| !configured(*name));
        let at = match (first_configured, first_other) {
            (Some(configured), Some(other)) if other < configured => Some(other),
            _ => {
                let present: Vec<(usize, &str)> = names
                    .iter()
                    .enumerate()
                    .filter_map(|(index, name)| {
                        name.filter(|name| expected.contains(name))
                            .map(|name| (index, name))
                    })
                    .collect();
                let sorted: Vec<&str> = expected
                    .iter()
                    .copied()
                    .filter(|key| present.iter().any(|(_, name)| name == key))
                    .collect();
                present
                    .iter()
                    .zip(&sorted)
                    .find(|((_, name), key)| name != *key)
                    .map(|((index, _), _)| *index)
            }
        };
        let Some(at) = at else {
            return;
        };
        let wanted: Vec<&str> = expected
            .iter()
            .copied()
            .filter(|key| names.contains(&Some(key)))
            .collect();
        findings.push(source.finding(
            self.name(),
            properties[at].span.start,
            format!(
                "Keys in @{kind} decorator should be ordered: {}",
                wanted.join(", ")
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::SortKeysInTypeDecorator;
    use crate::testing::lint;

    /// The line, column and message of each finding on a piece of code.
    fn check(code: &str) -> Vec<(usize, usize, String)> {
        lint(&SortKeysInTypeDecorator, "playground/src/app/a.ts", code)
            .into_iter()
            .map(|each| (each.line, each.column, each.message))
            .collect()
    }

    #[test]
    fn f51_ac2_accepts_keys_in_the_default_order() {
        // The plugin's valid cases under the default orders.
        for code in [
            "@Component({\n  selector: 'app-test',\n  imports: [CommonModule],\n  standalone: true,\n  templateUrl: './test.component.html',\n  template: '<div>Test</div>',\n  styleUrl: './test.component.css',\n  styleUrls: ['./test.component.css'],\n  styles: ['div { color: red; }'],\n  providers: [TestService],\n  changeDetection: ChangeDetectionStrategy.OnPush,\n  encapsulation: ViewEncapsulation.None,\n  viewProviders: [ViewService],\n  host: { '[class.test]': 'true' },\n  hostDirectives: [TestDirective],\n  inputs: ['value'],\n  outputs: ['change'],\n  animations: [trigger('test', [])],\n  schemas: [CUSTOM_ELEMENTS_SCHEMA],\n  exportAs: 'appTest',\n  queries: { contentChild: new ContentChild('test') },\n  preserveWhitespaces: false,\n  jit: true,\n  moduleId: 'test-module',\n  interpolation: ['{{', '}}']\n})\nexport class TestComponent {}\n",
            "@Directive({\n  selector: '[appTest]',\n  standalone: true,\n  providers: [TestService],\n  host: { '[class.test]': 'true' },\n  hostDirectives: [OtherDirective],\n  inputs: ['value'],\n  outputs: ['change'],\n  exportAs: 'appTest',\n  queries: { contentChild: new ContentChild('test') },\n  jit: true\n})\nexport class TestDirective {}\n",
            "@NgModule({\n  id: 'test-module',\n  imports: [CommonModule],\n  declarations: [TestComponent],\n  providers: [TestService],\n  exports: [TestComponent],\n  bootstrap: [AppComponent],\n  schemas: [CUSTOM_ELEMENTS_SCHEMA],\n  jit: true\n})\nexport class TestModule {}\n",
            "@Pipe({\n  name: 'testPipe',\n  standalone: true,\n  pure: false\n})\nexport class TestPipe {}\n",
            "@Type({\n  c: 'c',\n  a: 'a'\n})\nclass Test {}\n",
            "@Component({\n  selector: 'app-root',\n  templateUrl: './app.component.html',\n  styleUrl: './app.component.css'\n})\nclass Test {}\n",
            "@Component({\n  selector: 'app-root',\n  custom: true\n})\nclass Test {}\n",
            "@Injectable({\n  providedIn: 'root'\n})\nclass TestService {}\n",
            "@Component({\n  selector: 'app-root', // a comment\n  /* another */\n  imports: [\n    CommonModule,\n  ],\n  encapsulation: ViewEncapsulation.None\n})\nclass Test {}\n",
        ] {
            assert_eq!(check(code), [], "{code}");
        }
    }

    #[test]
    fn f51_ac2_reports_the_first_key_out_of_order_at_its_property() {
        // The plugin's invalid cases, the keys from line 2 at column 3.
        for (code, line, wanted) in [
            (
                "@Component({\n  changeDetection: ChangeDetectionStrategy.OnPush,\n  selector: 'app-root',\n  imports: [CommonModule],\n  standalone: true,\n  templateUrl: './app.component.html',\n  styleUrl: './app.component.css',\n  encapsulation: ViewEncapsulation.None\n})\nclass Test {}\n",
                2,
                "Component decorator should be ordered: selector, imports, standalone, templateUrl, styleUrl, changeDetection, encapsulation",
            ),
            (
                "@Directive({\n  standalone: true,\n  selector: '[app-test]'\n})\nclass Test {}\n",
                2,
                "Directive decorator should be ordered: selector, standalone",
            ),
            (
                "@NgModule({\n  declarations: [AppComponent],\n  imports: [CommonModule]\n})\nclass Test {}\n",
                2,
                "NgModule decorator should be ordered: imports, declarations",
            ),
            (
                "@Pipe({\n  standalone: true,\n  name: 'myPipe'\n})\nclass Test {}\n",
                2,
                "Pipe decorator should be ordered: name, standalone",
            ),
            (
                "@Component({\n  styleUrl: './app.component.css',\n  selector: 'app-root',\n  templateUrl: './app.component.html'\n})\nclass Test {}\n",
                2,
                "Component decorator should be ordered: selector, templateUrl, styleUrl",
            ),
            (
                "@NgModule({\n  exports: [AppComponent],\n  declarations: [AppComponent]\n})\nclass Test {}\n",
                2,
                "NgModule decorator should be ordered: declarations, exports",
            ),
            (
                "@Injectable({\n  useFactory: createService,\n  providedIn: 'root'\n})\nclass TestService {}\n",
                2,
                "Injectable decorator should be ordered: providedIn, useFactory",
            ),
            (
                // The first key in its place, the second out of it.
                "@Component({\n  selector: 'app-root',\n  styleUrl: './a.css',\n  templateUrl: './a.html'\n})\nclass Test {}\n",
                3,
                "Component decorator should be ordered: selector, templateUrl, styleUrl",
            ),
            (
                // A key outside the order before every key of it.
                "@Component({\n  custom: true,\n  selector: 'app-root'\n})\nclass Test {}\n",
                2,
                "Component decorator should be ordered: selector",
            ),
        ] {
            assert_eq!(
                check(code),
                [(line, 3, format!("Keys in @{wanted}"))],
                "{code}"
            );
        }
    }
}
