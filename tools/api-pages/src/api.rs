//! The library's public API: the exports of the modules its entry reaches through
//! `export * from`, as `TypeDoc`'s `entryPoints` read them; a module no barrel reaches is
//! internal and never published.

use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use crate::extract;
use crate::model::{Body, Export, Kind, Type};

/// The public exports and what the generator cannot read.
#[derive(Debug, Default)]
pub struct Api {
    /// The exports, in the order their modules were reached.
    pub exports: Vec<Export>,
    /// What cannot be read, each with its place.
    pub problems: Vec<String>,
}

/// The file a specifier names from a module's folder: `./x` as `x.ts`, then `x/index.ts`.
fn resolve(folder: &Path, specifier: &str) -> Option<PathBuf> {
    let base = folder.join(specifier);
    let file = base.with_extension("ts");

    if file.is_file() {
        Some(file)
    } else {
        Some(base.join("index.ts")).filter(|index| index.is_file())
    }
}

/// A path relative to the entry's folder, with `/` on every platform.
fn relative(root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    relative
        .components()
        .map(|part| part.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// The public API of the library whose entry module is given.
#[must_use]
pub fn read(entry: &Path) -> Api {
    let root = entry.parent().unwrap_or(Path::new(".")).to_path_buf();
    let mut api = Api::default();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([entry.to_path_buf()]);
    let mut returns_of = Vec::new();

    while let Some(path) = queue.pop_front() {
        if !seen.insert(path.clone()) {
            continue;
        }
        let file = relative(&root, &path);
        let source = match fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) => {
                api.problems
                    .push(format!("{file}: cannot be read: {error}"));
                continue;
            }
        };
        let mut module = extract::module(&source, &file);
        let folder = path.parent().unwrap_or(Path::new(".")).to_path_buf();

        for specifier in &module.re_exports {
            match resolve(&folder, specifier) {
                Some(next) => queue.push_back(next),
                None => api
                    .problems
                    .push(format!("{file}: `{specifier}` names no module")),
            }
        }

        api.exports.append(&mut module.exports);
        api.problems.append(&mut module.problems);
        returns_of.append(&mut module.returns_of);
    }

    for (constant, function) in returns_of {
        settle_return_type(&mut api, &constant, &function);
    }

    api
}

/// A constant whose value is a call takes the written return type of that exported function.
fn settle_return_type(api: &mut Api, constant: &str, function: &str) {
    let returns = api.exports.iter().find_map(|export| match &export.body {
        Body::Function(signature) if export.name == function => Some(signature.returns.clone()),
        _ => None,
    });
    let Some(export) = api
        .exports
        .iter_mut()
        .find(|export| export.kind == Kind::Variable && export.name == constant)
    else {
        return;
    };

    if let Some(returns) = returns {
        export.body = Body::Variable {
            value: returns,
            literal: None,
        }
    } else {
        export.body = Body::Variable {
            value: Type::Intrinsic("unknown".to_owned()),
            literal: None,
        };
        api.problems.push(format!(
            "{}:{}: the generator cannot read this constant's type: `{function}` is no exported function, write its type annotation",
            export.file, export.line
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A temporary source tree, removed when the test ends.
    struct Tree(PathBuf);

    impl Drop for Tree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn tree(name: &str, files: &[(&str, &str)]) -> Tree {
        let root =
            std::env::temp_dir().join(format!("api-pages-api-{}-{name}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        for (path, text) in files {
            let path = root.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        }
        Tree(root)
    }

    #[test]
    fn follows_the_barrels_once_each_and_leaves_the_rest_out() {
        let tree = tree(
            "barrels",
            &[
                (
                    "index.ts",
                    "export * from \"./math\";\nexport type * from \"./model\";\nexport * from \"./math\";\n",
                ),
                ("math/index.ts", "export * from \"./half\";\n"),
                (
                    "math/half.ts",
                    "/** H. */\nexport const HALF = 0.5;\n/** Q. */\nexport const QUARTER = halfOf(HALF);\n",
                ),
                ("model/index.ts", "export type * from \"./size\";\n"),
                (
                    "model/size.ts",
                    "/** S. */\nexport type Size = number;\n/** F. */\nexport function halfOf(size: Size): number { return size; }\n",
                ),
                (
                    "model/internal.ts",
                    "/** I. */\nexport type Internal = number;\n",
                ),
            ],
        );
        let api = read(&tree.0.join("index.ts"));

        assert!(api.problems.is_empty(), "{:?}", api.problems);
        let names: Vec<_> = api.exports.iter().map(|each| each.name.as_str()).collect();
        assert_eq!(names, vec!["HALF", "QUARTER", "Size", "halfOf"]);
        assert_eq!(api.exports[2].file, "model/size.ts");
        assert_eq!(
            api.exports[1].body,
            Body::Variable {
                value: Type::Intrinsic("number".to_owned()),
                literal: None
            }
        );
    }

    #[test]
    fn names_a_missing_module_and_a_call_it_cannot_type() {
        let tree = tree(
            "missing",
            &[
                (
                    "index.ts",
                    "export * from \"./gone\";\nexport * from \"./a\";\n",
                ),
                (
                    "a.ts",
                    "/** A. */\nexport const A = Math.round(1);\n/** B. */\nexport const B = round(1);\n",
                ),
            ],
        );
        let api = read(&tree.0.join("index.ts"));

        assert_eq!(
            api.problems,
            vec![
                "index.ts: `./gone` names no module",
                "a.ts:2: the generator cannot read this constant's type: write its type annotation",
                "a.ts:4: the generator cannot read this constant's type: `round` is no exported function, write its type annotation",
            ]
        );
    }

    #[test]
    fn names_an_entry_it_cannot_read() {
        let api = read(Path::new("/nonexistent-api-pages/index.ts"));

        assert_eq!(api.problems.len(), 1);
        assert!(api.problems[0].starts_with("index.ts: cannot be read"));
    }
}
