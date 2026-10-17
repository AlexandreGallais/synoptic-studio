//! The project's linter on Oxc's crates (EN-075): the project's own rules and the rules oxlint
//! lacks, run after oxlint by `npm run lint` on the same targets. Each file is read and parsed
//! once, its semantic model built once, and every rule its scopes turn on runs over that one AST;
//! the files are spread over the machine's threads.

pub mod directives;
pub mod finding;
pub mod glob;
pub mod project;
pub mod rules;
pub mod scopes;
pub mod settings;
pub mod source;
#[cfg(test)]
pub mod testing;
pub mod walk;

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use oxc_allocator::Allocator;
use oxc_parser::config::RuntimeParserConfig;
use oxc_parser::{ParseOptions, Parser};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

use finding::Finding;
use project::Project;
use rules::Rule;
use source::Source;

/// The rule a file that cannot be parsed is reported under.
pub const PARSE_RULE: &str = "lint/parse";

/// The findings of a set of rules on a file's text.
#[must_use]
pub fn lint_text(
    project: &Project,
    path: &str,
    text: &str,
    rules: &[&dyn Rule],
    allocator: &Allocator,
) -> Vec<Finding> {
    let needs_tokens = rules.iter().any(|rule| rule.needs_tokens());
    let options = ParseOptions {
        preserve_parens: false,
        ..ParseOptions::default()
    };
    let parsed = Parser::new(allocator, text, SourceType::ts())
        .with_options(options)
        .with_config(RuntimeParserConfig::new(needs_tokens))
        .parse();
    let program = parsed.program;

    if let Some(error) = parsed.diagnostics.errors().next() {
        let offset = error
            .labels
            .first()
            .map_or(0, oxc_span::LabeledSpan::offset);
        let semantic = SemanticBuilder::new().build(&program).semantic;
        let source = Source::new(path, &program, &semantic, &[], project);

        return vec![source.finding(PARSE_RULE, offset, error.message.to_string())];
    }

    let semantic = SemanticBuilder::new()
        .with_build_nodes(true)
        .build(&program)
        .semantic;
    let source = Source::new(path, &program, &semantic, &parsed.tokens, project);
    let mut findings = Vec::new();

    for rule in rules {
        rule.check_file(&source, &mut findings);
    }
    let node_rules: Vec<&&dyn Rule> = rules.iter().filter(|rule| rule.checks_nodes()).collect();
    if !node_rules.is_empty() {
        for node in semantic.nodes().iter() {
            for rule in &node_rules {
                rule.check_node(node, &source, &mut findings);
            }
        }
    }
    directives::apply(&source, findings)
}

/// The findings of each file under its scopes' rules, read from the disk, over every thread.
#[must_use]
pub fn lint_files(project: &Project, paths: &[String]) -> Vec<Finding> {
    let next = AtomicUsize::new(0);
    let threads = thread::available_parallelism().map_or(1, usize::from);
    let mut findings: Vec<Finding> = thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    let mut allocator = Allocator::default();
                    let mut found = Vec::new();
                    while let Some(path) = paths.get(next.fetch_add(1, Ordering::Relaxed)) {
                        found.extend(lint_path(project, path, &allocator));
                        allocator.reset();
                    }
                    found
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap_or_default())
            .collect()
    });
    findings.sort_by(|a, b| (&a.path, a.line, a.column).cmp(&(&b.path, b.line, b.column)));
    findings
}

/// The findings of one file under its scopes' rules.
fn lint_path(project: &Project, path: &str, allocator: &Allocator) -> Vec<Finding> {
    let rules = rules::named(&scopes::rules_for(path));
    if rules.is_empty() {
        return Vec::new();
    }
    match fs::read_to_string(project.root.join(path)) {
        Ok(text) => lint_text(project, path, &text, &rules, allocator),
        Err(error) => vec![Finding {
            path: path.to_owned(),
            line: 1,
            column: 1,
            rule: PARSE_RULE,
            message: format!("Cannot read the file: {error}."),
            fix: None,
        }],
    }
}

/// Writes the fixes of the findings that carry one into their files; the findings left.
///
/// # Errors
///
/// When a fixed file cannot be read or written.
pub fn fix(project: &Project, findings: Vec<Finding>) -> std::io::Result<Vec<Finding>> {
    let (fixable, left): (Vec<Finding>, Vec<Finding>) = findings
        .into_iter()
        .partition(|finding| finding.fix.is_some());
    let mut paths: Vec<&str> = fixable
        .iter()
        .map(|finding| finding.path.as_str())
        .collect();
    paths.dedup();

    for path in paths {
        let fixes: Vec<&finding::Fix> = fixable
            .iter()
            .filter(|finding| finding.path == path)
            .filter_map(|finding| finding.fix.as_ref())
            .collect();
        let absolute = project.root.join(path);
        let text = fs::read_to_string(&absolute)?;
        fs::write(&absolute, finding::apply(&text, &fixes))?;
    }
    Ok(left)
}

/// Runs the linter from a root on its arguments, `[--fix] <targets…>`; the report and whether
/// it found nothing.
#[must_use]
pub fn run(root: &Path, arguments: &[String]) -> (String, bool) {
    let fixing = arguments.iter().any(|argument| argument == "--fix");
    let targets: Vec<String> = arguments
        .iter()
        .filter(|argument| *argument != "--fix")
        .cloned()
        .collect();
    if targets.is_empty() {
        return (
            "usage: lint [--fix] <files or folders…>\n".to_owned(),
            false,
        );
    }

    let project = Project::new(root, settings::REFERENCE_SOURCES);
    let paths = walk::files(root, &targets);
    let mut findings = lint_files(&project, &paths);
    let mut report = String::new();
    if fixing {
        match fix(&project, findings) {
            Ok(left) => findings = left,
            Err(error) => {
                let _ = writeln!(report, "lint: cannot write a fix: {error}");
                return (report, false);
            }
        }
    }
    for finding in &findings {
        let _ = writeln!(report, "{finding}");
    }
    let _ = writeln!(
        report,
        "lint: {} finding(s) in {} file(s)",
        findings.len(),
        paths.len()
    );
    (report, findings.is_empty())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use oxc_ast::AstKind;
    use oxc_semantic::AstNode;

    use super::{PARSE_RULE, fix, lint_files, run};
    use crate::finding::{Finding, Fix};
    use crate::project::Project;
    use crate::rules::Rule;
    use crate::settings::REFERENCE_SOURCES;
    use crate::source::Source;
    use crate::testing::{fixture, lint, write_files};

    /// Reports each top-level function's kind, and how many tokens the file holds.
    struct ProbeFile;

    impl Rule for ProbeFile {
        fn name(&self) -> &'static str {
            "probe/file"
        }

        fn needs_tokens(&self) -> bool {
            true
        }

        fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
            for function in source.top_level_functions() {
                let message = format!(
                    "kind {:?}, {} tokens",
                    source.kind(&function),
                    source.tokens.len()
                );
                findings.push(source.finding(self.name(), Source::name_offset(&function), message));
            }
        }
    }

    /// Reports each dynamic import, from the walk over the nodes.
    struct ProbeNode;

    impl Rule for ProbeNode {
        fn name(&self) -> &'static str {
            "probe/node"
        }

        fn checks_nodes(&self) -> bool {
            true
        }

        fn check_node(&self, node: &AstNode, source: &Source, findings: &mut Vec<Finding>) {
            if let AstKind::ImportExpression(import) = node.kind() {
                findings.push(source.finding(self.name(), import.span.start, "import".into()));
            }
        }
    }

    #[test]
    fn reports_a_file_it_cannot_parse_at_its_error() {
        let findings = lint(&ProbeFile, "src/a.ts", "const a = ;\n");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule, PARSE_RULE);
        assert_eq!((findings[0].line, findings[0].column), (1, 11));
    }

    #[test]
    fn reads_the_jsdoc_of_exported_and_local_functions_and_keeps_tokens() {
        let code = "/** @kind math */\nexport function f(): void {}\n// note\nfunction g(): void {}\n/** @kind a @kind b */\nfunction h(): void {}\ndeclare function i(): void;\nexport default function j(): void {}\n";
        let findings = lint(&ProbeFile, "src/a.ts", code);
        let places: Vec<(usize, usize, &str)> = findings
            .iter()
            .map(|each| (each.line, each.column, each.message.as_str()))
            .collect();

        // Tokens: 9 on line 2 (`export function f ( ) : void { }`), 8 on lines 4, 6 and 7, 10 on
        // line 8: 43.
        assert_eq!(
            places,
            [
                (2, 17, "kind Some(\"math\"), 43 tokens"),
                (4, 10, "kind None, 43 tokens"),
                (6, 10, "kind None, 43 tokens"),
            ]
        );
    }

    #[test]
    fn walks_the_nodes_once_for_the_node_rules() {
        let findings = lint(
            &ProbeNode,
            "src/a.ts",
            "const a = import(\"x\");\nconst b = () => import(\"y\");\n",
        );

        assert_eq!(
            findings.iter().map(|each| each.line).collect::<Vec<_>>(),
            [1, 2]
        );
    }

    #[test]
    fn counts_columns_in_utf16_units() {
        let findings = lint(
            &ProbeNode,
            "src/a.ts",
            "const a = \"é𝄞\", b = import(\"x\");\n",
        );

        // `const a = "é𝄞", b = ` is 10 + 5 (é one unit, 𝄞 two) + 6 = 21 units: column 22.
        assert_eq!(findings[0].column, 22);
    }

    #[test]
    fn silences_the_next_line_and_reports_unused_or_unjustified_directives() {
        let code = "// lint-disable-next-line probe/node -- loading is the point\nconst a = import(\"x\");\n/* lint-disable-next-line probe/node, probe/file */\nconst b = 1;\n// lint-disable-next-lines probe/node\nconst c = import(\"y\");\n";
        let findings = lint(&ProbeNode, "src/a.ts", code);
        let found: Vec<(usize, &str, &str)> = findings
            .iter()
            .map(|each| (each.line, each.rule, each.message.as_str()))
            .collect();

        assert_eq!(
            found,
            [
                (6, "probe/node", "import"),
                (
                    3,
                    "lint/disable-directive",
                    "A lint-disable directive gives its justification after `--`."
                ),
                (
                    3,
                    "lint/disable-directive",
                    "Unused lint-disable directive: `probe/node` reports nothing on the next line."
                ),
                (
                    3,
                    "lint/disable-directive",
                    "Unused lint-disable directive: `probe/file` reports nothing on the next line."
                ),
            ]
        );
    }

    #[test]
    fn writes_the_fixes_into_their_files() {
        let root = fixture("fix");
        write_files(&root, &["a.ts"]);
        let project = Project::new(&root, REFERENCE_SOURCES);
        let fixable = Finding {
            path: "a.ts".into(),
            line: 1,
            column: 1,
            rule: "probe/file",
            message: "fix".into(),
            fix: Some(Fix {
                start: 7,
                end: 9,
                text: "{ a }".into(),
            }),
        };
        let left = Finding {
            fix: None,
            ..fixable.clone()
        };

        let result = fix(&project, vec![fixable, left.clone()]);

        assert_eq!(result.ok(), Some(vec![left]));
        assert_eq!(
            fs::read_to_string(root.join("a.ts")).ok().as_deref(),
            Some("export { a };\n")
        );
        let missing = Finding {
            path: "missing.ts".into(),
            line: 1,
            column: 1,
            rule: "probe/file",
            message: "fix".into(),
            fix: Some(Fix {
                start: 0,
                end: 0,
                text: String::new(),
            }),
        };
        assert!(fix(&project, vec![missing]).is_err());
    }

    #[test]
    fn walks_the_targets_and_prints_each_finding_then_a_count() {
        let root = fixture("run");
        write_files(
            &root,
            &[
                "scripts/a.ts",
                "scripts/b.md",
                "dist/c.ts",
                "scripts/sub/d.ts",
            ],
        );

        let (report, clean) = run(
            &root,
            &["scripts".into(), "dist".into(), "./scripts/a.ts".into()],
        );

        assert_eq!(
            report,
            "scripts/sub/d.ts:1:1 local/folder-has-index Folder `scripts/sub` has no `index.ts`: create its barrel.\nlint: 1 finding(s) in 2 file(s)\n"
        );
        assert!(!clean);
        assert_eq!(
            crate::walk::files(&root, &[".".into(), "missing".into()]),
            ["scripts/a.ts", "scripts/sub/d.ts"]
        );
        // lint-staged's absolute paths are read from the root; a file outside it is left out.
        let absolute = root.join("scripts/a.ts").to_string_lossy().into_owned();
        assert_eq!(
            crate::walk::files(&root, &[absolute, "/elsewhere/a.ts".into()]),
            ["scripts/a.ts"]
        );
        let (usage, clean) = run(&root, &["--fix".into()]);
        assert!(usage.starts_with("usage: lint"));
        assert!(!clean);
    }

    #[test]
    fn fixes_what_it_can_and_prints_the_rest() {
        let root = fixture("run-fix");
        // The site's theme: imports checked, no barrel asked.
        let theme = [
            "docs/.vitepress/a.ts",
            "docs/.vitepress/c.ts",
            "docs/.vitepress/clean.ts",
        ];
        write_files(&root, &theme);
        fs::write(root.join(theme[0]), "import \"./c.ts\";\n").unwrap_or_default();

        let (report, clean) = run(&root, &["--fix".into(), "docs".into()]);

        assert_eq!(report, "lint: 0 finding(s) in 3 file(s)\n");
        assert!(clean);
        assert_eq!(
            fs::read_to_string(root.join(theme[0])).ok().as_deref(),
            Some("import \"./c\";\n")
        );
    }

    #[test]
    fn reports_a_file_it_cannot_read() {
        let root = fixture("unreadable");
        let project = Project::new(&root, REFERENCE_SOURCES);

        let findings = lint_files(&project, &["src/missing.ts".into()]);

        assert_eq!(findings.len(), 1);
        assert!(findings[0].message.starts_with("Cannot read the file"));
    }
}
