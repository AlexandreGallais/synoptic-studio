//! `import-x/no-extraneous-dependencies`: a package imported is one `package.json` lists where
//! the file may use it (`eslint-plugin-import-x` 4.17.1, `docs/rules/no-extraneous-dependencies.md`),
//! with the project's options: in the library, the playground and the rest, `dependencies` only
//! (which stays empty), a bundled package allowed; in the tools and the tests
//! (`scopes::TOOLING_FILES`, `scopes::TEST_FILES`), development, optional and peer packages too.
//! An import of types only, a Node.js module, a relative path and a package not installed (which
//! the plugin cannot resolve) are left. No fix.

use crate::finding::Finding;
use crate::glob::matches_any;
use crate::package::Dependencies;
use crate::rules::Rule;
use crate::scopes::{TEST_FILES, TOOLING_FILES};
use crate::source::Source;

use super::sources::module_sources;

/// The rule.
pub struct NoExtraneousDependencies;

/// Node.js's modules without their `node:` prefix (`module.builtinModules`, Node.js 24).
const NODE_MODULES: [&str; 58] = [
    "assert",
    "assert/strict",
    "async_hooks",
    "buffer",
    "child_process",
    "cluster",
    "console",
    "constants",
    "crypto",
    "dgram",
    "diagnostics_channel",
    "dns",
    "dns/promises",
    "domain",
    "events",
    "fs",
    "fs/promises",
    "http",
    "http2",
    "https",
    "inspector",
    "inspector/promises",
    "module",
    "net",
    "os",
    "path",
    "path/posix",
    "path/win32",
    "perf_hooks",
    "process",
    "punycode",
    "querystring",
    "readline",
    "readline/promises",
    "repl",
    "stream",
    "stream/consumers",
    "stream/promises",
    "stream/web",
    "string_decoder",
    "sys",
    "timers",
    "timers/promises",
    "tls",
    "trace_events",
    "tty",
    "url",
    "util",
    "util/types",
    "v8",
    "vm",
    "wasi",
    "worker_threads",
    "zlib",
    "_http_agent",
    "_http_client",
    "_http_common",
    "_stream_readable",
];

/// The package a path names: its scope and name, or its first segment.
fn package_of(specifier: &str) -> &str {
    let mut segments = specifier.splitn(3, '/');
    let first = segments.next().unwrap_or_default();
    match segments.next() {
        Some(second) if first.starts_with('@') => &specifier[..first.len() + 1 + second.len()],
        _ => first,
    }
}

/// Whether a path names an installed package: neither relative, absolute nor Node.js's.
fn is_external(specifier: &str) -> bool {
    !(specifier.starts_with('.')
        || specifier.starts_with('/')
        || specifier.starts_with("node:")
        || NODE_MODULES.contains(&specifier))
}

/// What a file may import, by the fields of `package.json`.
#[derive(Clone, Copy)]
struct Allowed {
    /// `devDependencies`, `optionalDependencies` and `peerDependencies`.
    development: bool,
}

/// The message on a package the file may not import, when it may not.
fn verdict(dependencies: &Dependencies, package: &str, allowed: Allowed) -> Option<String> {
    let listed = |names: &[String]| names.iter().any(|name| name == package);
    if listed(&dependencies.dependencies)
        || listed(&dependencies.bundled)
        || (allowed.development
            && (listed(&dependencies.dev)
                || listed(&dependencies.optional)
                || listed(&dependencies.peer)))
    {
        return None;
    }
    Some(if listed(&dependencies.dev) {
        format!("'{package}' should be listed in the project's dependencies, not devDependencies.")
    } else if listed(&dependencies.optional) {
        format!(
            "'{package}' should be listed in the project's dependencies, not optionalDependencies."
        )
    } else {
        format!(
            "'{package}' should be listed in the project's dependencies. Run 'npm i {package}' to add it"
        )
    })
}

impl Rule for NoExtraneousDependencies {
    fn name(&self) -> &'static str {
        "import-x/no-extraneous-dependencies"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let allowed = Allowed {
            development: matches_any(TOOLING_FILES, source.path)
                || matches_any(TEST_FILES, source.path),
        };
        let dependencies = source.project.dependencies();

        for each in module_sources(source) {
            if each.types_only || !is_external(each.specifier) {
                continue;
            }
            let package = package_of(each.specifier);
            let installed = source
                .project
                .is_folder(&source.project.root.join("node_modules").join(package));
            if !installed {
                continue;
            }
            if let Some(message) = verdict(dependencies, package, allowed) {
                findings.push(source.finding(self.name(), each.node, message));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::{NoExtraneousDependencies, package_of};
    use crate::testing::{fixture, lint_at, messages};

    /// The documentation's `package.json`, its packages installed, and `lodash` beside them.
    fn project() -> PathBuf {
        let root = fixture("no-extraneous-dependencies");
        let package = r#"{
  "name": "my-project",
  "dependencies": {
    "builtin-modules": "^1.1.1",
    "lodash.cond": "^4.2.0",
    "lodash.find": "^4.2.0",
    "pkg-up": "^1.0.0"
  },
  "devDependencies": { "ava": "^0.13.0", "eslint": "^2.4.0" },
  "optionalDependencies": { "lodash.isarray": "^4.0.0" },
  "peerDependencies": { "react": ">=15.0.0 <16.0.0" },
  "bundledDependencies": ["@generated/foo"]
}"#;
        fs::write(root.join("package.json"), package).unwrap_or_default();
        for name in [
            "lodash",
            "lodash.find",
            "lodash.isarray",
            "ava",
            "react",
            "@generated/foo",
            "foo",
        ] {
            fs::create_dir_all(root.join("node_modules").join(name)).unwrap_or_default();
        }
        root
    }

    /// The messages of the findings on a file of the project.
    fn found(path: &str, code: &str) -> Vec<String> {
        messages(&lint_at(&NoExtraneousDependencies, &project(), path, code))
    }

    #[test]
    fn reports_a_package_missing_or_listed_for_development_only() {
        // The documentation's failing cases under the project's options.
        let missing =
            "'lodash' should be listed in the project's dependencies. Run 'npm i lodash' to add it";
        assert_eq!(
            found(
                "src/a.ts",
                "var _ = require('lodash');\nimport _ from 'lodash';"
            ),
            [missing, missing]
        );
        assert_eq!(
            found("src/a.ts", "import react from 'react';"),
            ["'react' should be listed in the project's dependencies. Run 'npm i react' to add it"]
        );
        assert_eq!(
            found(
                "src/a.ts",
                "import test from 'ava';\nvar test = require('ava');"
            ),
            [
                "'ava' should be listed in the project's dependencies, not devDependencies.",
                "'ava' should be listed in the project's dependencies, not devDependencies."
            ]
        );
        assert_eq!(
            found("playground/a.ts", "import isArray from 'lodash.isarray';"),
            [
                "'lodash.isarray' should be listed in the project's dependencies, not optionalDependencies."
            ]
        );
        assert_eq!(found("src/a.ts", "export * from 'ava/sub';").len(), 1);
    }

    #[test]
    fn lets_listed_builtin_relative_and_type_imports_through() {
        // The documentation's passing cases, its development ones in the tools and the tests.
        for (path, code) in [
            (
                "src/a.ts",
                "var path = require('path');\nvar foo = require('./foo');",
            ),
            ("src/a.ts", "import fs from 'node:fs';"),
            ("src/a.ts", "import find from 'lodash.find';"),
            ("src/a.ts", "import foo from '@generated/foo';"),
            ("src/a.ts", "import type { MyType } from 'foo';"),
            ("src/a.ts", "import nothing from 'not-installed';"),
            (
                "scripts/a.ts",
                "import test from 'ava';\nimport react from 'react';",
            ),
            ("src/a.test.ts", "import isArray from 'lodash.isarray';"),
        ] {
            assert_eq!(found(path, code), Vec::<String>::new(), "{path}: {code}");
        }
    }

    #[test]
    fn names_a_package_by_its_scope_and_name() {
        assert_eq!(package_of("@a/b/c"), "@a/b");
        assert_eq!(package_of("vitest/config"), "vitest");
        assert_eq!(package_of("a"), "a");
    }
}
