//! What the rules' tests share: a rule run on a piece of code, as `RuleTester` ran the rules it
//! carries, and fixture folders on the disk.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use oxc_allocator::Allocator;

use crate::finding::Finding;
use crate::project::Project;
use crate::rules::Rule;
use crate::settings::REFERENCE_SOURCES;

/// The findings of one rule on a piece of code, the file at a path of a root.
#[must_use]
pub fn lint_at(rule: &dyn Rule, root: &Path, path: &str, code: &str) -> Vec<Finding> {
    let project = Project::new(root, REFERENCE_SOURCES);

    crate::lint_text(&project, path, code, &[rule], &Allocator::default())
}

/// The findings of one rule on a piece of code, in a file of a root that does not exist.
#[must_use]
pub fn lint(rule: &dyn Rule, path: &str, code: &str) -> Vec<Finding> {
    lint_at(rule, Path::new("/project"), path, code)
}

/// The messages of the findings, in order.
#[must_use]
pub fn messages(findings: &[Finding]) -> Vec<String> {
    findings
        .iter()
        .map(|finding| finding.message.clone())
        .collect()
}

/// A text with the fixes of its findings applied.
#[must_use]
pub fn fixed(code: &str, findings: &[Finding]) -> String {
    let fixes: Vec<_> = findings
        .iter()
        .filter_map(|each| each.fix.as_ref())
        .collect();

    crate::finding::apply(code, &fixes)
}

/// A new empty folder under the system's temporary folder, named after a test.
///
/// # Panics
///
/// When the folder cannot be made: a test on files never written would pass for nothing.
#[must_use]
pub fn fixture(name: &str) -> PathBuf {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    let folder = std::env::temp_dir().join(format!(
        "lint-{name}-{}-{}",
        std::process::id(),
        COUNT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).expect("the fixture folder");
    folder
}

/// Writes files into a fixture folder, their folders created, each holding `export {};`.
///
/// # Panics
///
/// When a file or its folder cannot be written.
pub fn write_files(root: &Path, files: &[&str]) {
    for file in files {
        let path = root.join(file);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("the file's folder");
        }
        fs::write(path, "export {};\n").expect("the file");
    }
}

/// No finding, no message, no value: what a test expects of a case the rule accepts.
pub const NONE: [&str; 0] = [];
