//! `import-x/no-restricted-paths`: a layer imports only the layers it lists, and the playground
//! the library's public API alone (`eslint-plugin-import-x` 4.17.1,
//! `docs/rules/no-restricted-paths.md`), with the project's zones, built from
//! `settings::LAYERS` as `eslint/settings/layers.ts` built them. A relative path is resolved as
//! the plugin's Node.js resolver resolves it — the file, then with `.ts`, `.js` or `.json`, then
//! the folder's `index` — and a path that resolves to nothing is left. No fix.

use std::path::{Component, Path};

use crate::finding::Finding;
use crate::rules::Rule;
use crate::settings::{LAYERS, PUBLIC_API_PATH};
use crate::source::Source;

use super::sources::module_sources;

/// The rule.
pub struct NoRestrictedPaths;

/// A zone: the files it covers, the folders they may not import, an exception, and its message.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Zone {
    /// The folder of the files it covers.
    target: &'static str,
    /// The folders those files may not import.
    from: Vec<&'static str>,
    /// A file they may import all the same.
    except: Option<&'static str>,
    /// The reason, after the plugin's message.
    message: String,
}

/// The extensions the resolver tries, in order.
const EXTENSIONS: [&str; 3] = ["ts", "js", "json"];

/// The zones: one per layer of the library that may not import every other one, then the
/// playground's.
fn zones() -> Vec<Zone> {
    let library: Vec<_> = LAYERS
        .iter()
        .filter(|layer| layer.path.starts_with("src/"))
        .collect();
    let mut zones: Vec<Zone> = library
        .iter()
        .map(|layer| Zone {
            target: layer.path,
            from: library
                .iter()
                .filter(|other| other.name != layer.name && !layer.imports.contains(&other.name))
                .map(|other| other.path)
                .collect(),
            except: None,
            message: format!(
                "Layer `{}` may only import: {}.",
                layer.name,
                if layer.imports.is_empty() {
                    "nothing".to_owned()
                } else {
                    layer.imports.join(", ")
                }
            ),
        })
        .filter(|zone| !zone.from.is_empty())
        .collect();
    zones.push(Zone {
        target: "playground",
        from: vec!["src"],
        except: Some(PUBLIC_API_PATH),
        message: format!("The playground only uses the public API ({PUBLIC_API_PATH})."),
    });
    zones
}

/// Whether a path from the root lies in a folder, or is it.
fn contains(folder: &str, path: &str) -> bool {
    path == folder
        || path
            .strip_prefix(folder)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// A path with its `.` and `..` segments resolved, from the root.
fn normalized(path: &Path) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => parts.push(part.to_string_lossy().into_owned()),
            Component::ParentDir => {
                parts.pop()?;
            }
            _ => {}
        }
    }
    Some(parts.join("/"))
}

/// The file a relative path names from a file, from the root, when it exists.
fn resolved(source: &Source, specifier: &str) -> Option<String> {
    let folder = Path::new(source.path).parent().unwrap_or(Path::new(""));
    let base = normalized(&folder.join(specifier))?;
    let exists = |candidate: &str| source.project.is_file(&source.project.root.join(candidate));
    let index = if base.is_empty() {
        "index".to_owned()
    } else {
        format!("{base}/index")
    };
    let candidates = std::iter::once(base.clone())
        .filter(|_| !specifier.ends_with('/'))
        .chain(
            EXTENSIONS
                .iter()
                .map(|extension| format!("{base}.{extension}")),
        )
        .chain(
            EXTENSIONS
                .iter()
                .map(|extension| format!("{index}.{extension}")),
        );

    candidates.into_iter().find(|candidate| exists(candidate))
}

impl Rule for NoRestrictedPaths {
    fn name(&self) -> &'static str {
        "import-x/no-restricted-paths"
    }

    fn check_file(&self, source: &Source, findings: &mut Vec<Finding>) {
        let zones: Vec<Zone> = zones()
            .into_iter()
            .filter(|zone| contains(zone.target, source.path))
            .collect();
        if zones.is_empty() {
            return;
        }
        for each in module_sources(source) {
            if !each.specifier.starts_with('.') {
                continue;
            }
            let Some(file) = resolved(source, each.specifier) else {
                continue;
            };
            for zone in &zones {
                let excepted = zone.except.is_some_and(|except| contains(except, &file));
                for _ in zone.from.iter().filter(|from| contains(from, &file)) {
                    if !excepted {
                        findings.push(source.finding(
                            self.name(),
                            each.literal,
                            format!(
                                "Unexpected path \"{}\" imported in restricted zone. {}",
                                each.specifier, zone.message
                            ),
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{NoRestrictedPaths, zones};
    use crate::testing::{fixture, lint_at, messages, write_files};

    /// The messages of the findings on a file of a project holding the layers' modules.
    fn found(path: &str, code: &str) -> Vec<String> {
        let root = fixture("no-restricted-paths");
        write_files(
            &root,
            &[
                "src/index.ts",
                "src/math/index.ts",
                "src/math/vector/norm.ts",
                "src/geometry/index.ts",
                "src/model/index.ts",
                "src/render/index.ts",
                "src/io/data.json",
                "playground/a.ts",
            ],
        );
        messages(&lint_at(&NoRestrictedPaths, &root, path, code))
    }

    #[test]
    fn reports_an_import_of_a_layer_not_listed() {
        // The documentation's case (a zone's `from` imported from its `target`), with the
        // project's zones: a lower layer importing a higher one, and the playground importing
        // past the public API.
        assert_eq!(
            found("src/math/a.ts", "import { b } from \"../geometry\";"),
            [
                "Unexpected path \"../geometry\" imported in restricted zone. Layer `math` may only import: nothing."
            ]
        );
        assert_eq!(
            found(
                "src/geometry/a/b.ts",
                "export * from \"../../model\";\nimport type { R } from \"../../render\";"
            ),
            [
                "Unexpected path \"../../model\" imported in restricted zone. Layer `geometry` may only import: math.",
                "Unexpected path \"../../render\" imported in restricted zone. Layer `geometry` may only import: math."
            ]
        );
        assert_eq!(
            found(
                "playground/a/b.ts",
                "import { norm } from \"../../src/math/vector/norm\";"
            ),
            [
                "Unexpected path \"../../src/math/vector/norm\" imported in restricted zone. The playground only uses the public API (src/index.ts)."
            ]
        );
        assert_eq!(
            found("src/model/a.ts", "const d = require(\"../io/data.json\");").len(),
            1
        );
    }

    #[test]
    fn lets_listed_layers_the_public_api_and_unresolved_paths_through() {
        for (path, code) in [
            ("src/geometry/a.ts", "import { n } from \"../math\";"),
            (
                "src/render/a.ts",
                "import { m } from \"../model\";\nimport { g } from \"../geometry/index\";",
            ),
            ("playground/a/b.ts", "import { a } from \"../../src\";"),
            (
                "playground/a/b.ts",
                "import { a } from \"../../src/index.ts\";",
            ),
            (
                "src/math/a.ts",
                "import { b } from \"../geometry/missing\";",
            ),
            (
                "src/math/a.ts",
                "import { b } from \"./sibling\";\nimport v from \"vitest\";",
            ),
            ("scripts/a.ts", "import { b } from \"../src/render\";"),
            ("src/index.ts", "export * from \"./render\";"),
        ] {
            assert_eq!(found(path, code), Vec::<String>::new(), "{path}: {code}");
        }
    }

    #[test]
    fn builds_one_zone_per_layer_that_may_not_import_every_other() {
        let built = zones();
        let targets: Vec<&str> = built.iter().map(|zone| zone.target).collect();

        // `interaction` may import every other layer of the library: it has no zone.
        assert_eq!(
            targets,
            [
                "src/math",
                "src/geometry",
                "src/model",
                "src/routing",
                "src/io",
                "src/render",
                "playground"
            ]
        );
        assert_eq!(
            built[0].from,
            [
                "src/geometry",
                "src/model",
                "src/routing",
                "src/io",
                "src/render",
                "src/interaction"
            ]
        );
        assert_eq!(built[5].from, ["src/interaction"]);
    }
}
