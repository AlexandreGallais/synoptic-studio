//! `local/component-files`: a component is three files of one name, `zoom-menu.component.ts`,
//! `.html` and `.scss`, its template and styles never inline (NOTE-0036, finding 27; F51.AC2).
//! Its `templateUrl` and `styleUrl` name the files beside it, which exist; `template`, `styles`
//! and `styleUrls` are refused, `component-max-inline-declarations` at 0 measuring the inline
//! ones too.

use oxc_ast::AstKind;
use oxc_ast::ast::ObjectProperty;
use oxc_semantic::AstNode;
use oxc_span::GetSpan;

use super::Rule;
use super::angular::bindings::written;
use super::angular::classes::key_identifier;
use super::angular::metadata::{class_decorator, decorator_properties};
use crate::finding::Finding;
use crate::source::Source;

/// The rule.
pub struct ComponentFiles;

/// The keys of a component's own files, the extension each names and what it holds.
const FILES: [(&str, &str, &str); 2] = [
    ("templateUrl", "html", "template is"),
    ("styleUrl", "scss", "styles are"),
];

/// The keys of inline declarations and of a list of stylesheets.
const REFUSED: [&str; 3] = ["template", "styles", "styleUrls"];

/// Checks one of a component's files: named by its key, after the component, beside it.
fn check_file(
    rule: &'static str,
    (key, extension, what): (&str, &str, &str),
    (decorator_start, properties): (u32, &[&ObjectProperty]),
    source: &Source,
    findings: &mut Vec<Finding>,
) {
    let stem = source.name().strip_suffix(".ts").unwrap_or(source.name());
    let wanted = format!("./{stem}.{extension}");
    let Some(property) = properties
        .iter()
        .find(|property| key_identifier(&property.key) == Some(key))
    else {
        findings.push(source.finding(
            rule,
            decorator_start,
            format!("A component's {what} its own file: `{key}: \"{wanted}\"`"),
        ));
        return;
    };
    let at = property.value.span().start;
    if written(&property.value).map(|url| url.text) != Some(wanted.as_str()) {
        findings.push(source.finding(
            rule,
            at,
            format!("`{key}` names the file after the component: \"{wanted}\""),
        ));
    } else if !source
        .project
        .is_file(&source.folder().join(format!("{stem}.{extension}")))
    {
        findings.push(source.finding(
            rule,
            at,
            format!("\"{wanted}\" is not beside the component"),
        ));
    }
}

impl Rule for ComponentFiles {
    fn name(&self) -> &'static str {
        "local/component-files"
    }

    fn checks_nodes(&self) -> bool {
        true
    }

    fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
        let AstKind::Class(class) = node.kind() else {
            return;
        };
        let Some(decorator) = class_decorator(class, "Component") else {
            return;
        };
        let properties = decorator_properties(decorator);
        for property in &properties {
            if let Some(key) = key_identifier(&property.key).filter(|key| REFUSED.contains(key)) {
                findings.push(source.finding(
                    self.name(),
                    property.span.start,
                    format!(
                        "A component keeps no `{key}`: its template and styles are its own files"
                    ),
                ));
            }
        }
        for file in FILES {
            check_file(
                self.name(),
                file,
                (decorator.span.start, &properties),
                source,
                findings,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ComponentFiles;
    use crate::testing::{fixture, lint_at, write_files};

    /// The place and message of each finding on a component's metadata in
    /// `zoom-menu.component.ts`, beside the files given.
    fn check(files: &[&str], metadata: &str) -> Vec<(usize, usize, String)> {
        let root = fixture("component-files");
        let folder = "playground/src/app/zoom-menu";
        let paths: Vec<String> = files
            .iter()
            .map(|file| format!("{folder}/{file}"))
            .collect();
        let paths: Vec<&str> = paths.iter().map(String::as_str).collect();
        write_files(&root, &paths);
        let code = format!(
            "@Component({{\n  selector: 'app-zoom-menu',\n{metadata}}})\nexport class ZoomMenuComponent {{}}\n"
        );
        lint_at(
            &ComponentFiles,
            &root,
            &format!("{folder}/zoom-menu.component.ts"),
            &code,
        )
        .into_iter()
        .map(|each| (each.line, each.column, each.message))
        .collect()
    }

    /// Both files beside the component.
    const BOTH: [&str; 2] = ["zoom-menu.component.html", "zoom-menu.component.scss"];

    #[test]
    fn f51_ac2_accepts_a_component_of_three_files_of_one_name() {
        assert_eq!(
            check(
                &BOTH,
                "  templateUrl: './zoom-menu.component.html',\n  styleUrl: `./zoom-menu.component.scss`,\n"
            ),
            []
        );
        // A directive, and a class Angular does not know.
        let root = fixture("component-files-other");
        let code = "@Directive({ selector: '[appA]' })\nclass A {}\nclass B {}\n";
        assert_eq!(
            lint_at(
                &ComponentFiles,
                &root,
                "playground/src/app/a.directive.ts",
                code
            ),
            []
        );
    }

    #[test]
    fn f51_ac2_refuses_inline_declarations_at_their_property() {
        let metadata = "  template: '<p></p>',\n  styles: [':host {}'],\n  styleUrls: ['./zoom-menu.component.scss'],\n  templateUrl: './zoom-menu.component.html',\n  styleUrl: './zoom-menu.component.scss',\n";
        let found: Vec<(usize, usize)> = check(&BOTH, metadata)
            .into_iter()
            .map(|(line, column, message)| {
                assert!(message.starts_with("A component keeps no `"), "{message}");
                (line, column)
            })
            .collect();

        assert_eq!(found, [(3, 3), (4, 3), (5, 3)]);
    }

    #[test]
    fn f51_ac2_refuses_a_file_missing_misnamed_or_absent() {
        // Neither key: both reported at the decorator.
        assert_eq!(
            check(&BOTH, ""),
            [
                (
                    1,
                    1,
                    "A component's template is its own file: `templateUrl: \"./zoom-menu.component.html\"`".to_owned()
                ),
                (
                    1,
                    1,
                    "A component's styles are its own file: `styleUrl: \"./zoom-menu.component.scss\"`".to_owned()
                ),
            ]
        );
        // Misnamed: `  templateUrl: ` is 15 columns, the value at 16; `  styleUrl: ` 12, at 13.
        assert_eq!(
            check(&BOTH, "  templateUrl: './menu.html',\n  styleUrl: '../zoom-menu.component.scss',\n"),
            [
                (
                    3,
                    16,
                    "`templateUrl` names the file after the component: \"./zoom-menu.component.html\"".to_owned()
                ),
                (
                    4,
                    13,
                    "`styleUrl` names the file after the component: \"./zoom-menu.component.scss\"".to_owned()
                ),
            ]
        );
        // Named right, the stylesheet not written.
        assert_eq!(
            check(
                &["zoom-menu.component.html"],
                "  templateUrl: './zoom-menu.component.html',\n  styleUrl: './zoom-menu.component.scss',\n"
            ),
            [(
                4,
                13,
                "\"./zoom-menu.component.scss\" is not beside the component".to_owned()
            )]
        );
    }
}
