//! `eslint-plugin-import-x`'s rules that oxlint lacks (EN-084), each one module with the cases
//! of its documentation in the plugin's 4.17.1 as tests, with the project's options, reading the
//! module paths a file names (`sources`). On for every TypeScript file.

pub mod no_extraneous_dependencies;
pub mod no_restricted_paths;
pub mod sources;

use super::Rule;

/// The rules, in the order of their names.
pub const RULES: &[&dyn Rule] = &[
    &no_extraneous_dependencies::NoExtraneousDependencies,
    &no_restricted_paths::NoRestrictedPaths,
];

/// Their names, on for every TypeScript file.
pub const NAMES: &[&str] = &[
    "import-x/no-extraneous-dependencies",
    "import-x/no-restricted-paths",
];

#[cfg(test)]
mod tests {
    use super::{NAMES, RULES};

    #[test]
    fn lists_each_rule_once_under_its_name() {
        let names: Vec<&str> = RULES.iter().map(|rule| rule.name()).collect();

        assert_eq!(names, NAMES);
        assert!(NAMES.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
