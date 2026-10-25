//! Tests of the three checks of `api-pages`, each on a copy of `fixtures/library` with one
//! planted case: the run fails, names each case with its file and line, and writes no page.
//! `[F19.AC7]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};

/// A temporary copy of the fixture library and an output folder, removed when the test ends.
struct Library(PathBuf);

impl Drop for Library {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Every file of a folder copied into another.
fn copy(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let path = entry.unwrap().path();
        let target = to.join(path.file_name().unwrap());
        if path.is_dir() {
            copy(&path, &target);
        } else {
            fs::copy(&path, &target).unwrap();
        }
    }
}

/// A copy of the fixture library, one module added and its barrel re-exporting it.
fn planted(name: &str, module: &str) -> Library {
    let root = std::env::temp_dir().join(format!("api-pages-checks-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    copy(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/library"),
        &root.join("src"),
    );
    fs::write(root.join("src/math/vector/planted.ts"), module).unwrap();
    let barrel = root.join("src/math/vector/index.ts");
    let text = fs::read_to_string(&barrel).unwrap();
    fs::write(barrel, format!("{text}export * from \"./planted\";\n")).unwrap();
    Library(root)
}

/// The run's exit code and its error output; the output folder must stay unwritten.
fn failure(library: &Library) -> (Option<i32>, String) {
    let out = library.0.join("api");
    let output = test_support::command(env!("CARGO_BIN_EXE_api-pages"))
        .arg(library.0.join("src/index.ts"))
        .arg(&out)
        .arg("/api/")
        .arg("editor")
        .output()
        .unwrap();
    assert!(!out.exists(), "a page was written");
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn f19_ac7_fails_on_an_export_or_a_property_without_tsdoc() {
    let library = planted(
        "undocumented",
        "export function moved(by: number): number {\n  return by;\n}\n\n/** A box. */\nexport type Box = {\n  readonly width: number;\n};\n",
    );

    assert_eq!(
        failure(&library),
        (
            Some(1),
            "math/vector/planted.ts:1: `moved` has no TSDoc block\nmath/vector/planted.ts:7: `Box.width` has no TSDoc block\napi-pages: 2 problem(s), no page written\n".to_owned()
        )
    );
}

#[test]
fn f19_ac7_fails_on_a_property_inside_a_type_argument_or_a_function_type() {
    let library = planted(
        "nested",
        "/** A box. */\nexport type Box = Readonly<{\n  width: number;\n}>;\n\n/** A reader. */\nexport type Reader = (options: { readonly strict: boolean }) => number;\n",
    );

    assert_eq!(
        failure(&library),
        (
            Some(1),
            "math/vector/planted.ts:3: `Box.width` has no TSDoc block\nmath/vector/planted.ts:7: `Reader.strict` has no TSDoc block\napi-pages: 2 problem(s), no page written\n".to_owned()
        )
    );
}

#[test]
fn f19_ac7_fails_on_a_link_naming_nothing_exported() {
    let library = planted(
        "link",
        "/**\n * Moved, as {@link add} does, unlike {@link substract}.\n *\n * @see {@link Pointt}\n */\nexport function moved(by: number): number {\n  return by;\n}\n",
    );

    assert_eq!(
        failure(&library),
        (
            Some(1),
            "math/vector/planted.ts:6: `{@link substract}` in `moved` names no export\nmath/vector/planted.ts:6: `{@link Pointt}` in `moved` names no export\napi-pages: 2 problem(s), no page written\n".to_owned()
        )
    );
}

#[test]
fn f19_ac7_fails_on_an_internal_name_reachable_from_the_api() {
    let library = planted(
        "internal",
        "import type { Corner } from \"../../geometry/contour/corner\";\n\ninterface Local {\n  readonly x: number;\n}\n\n/** Moved. */\nexport function moved(corner: Corner, local: Local): number {\n  return local.x;\n}\n",
    );

    assert_eq!(
        failure(&library),
        (
            Some(1),
            "math/vector/planted.ts:8: `moved` uses `Corner`, which the API does not export\nmath/vector/planted.ts:8: `moved` uses `Local`, which the API does not export\napi-pages: 2 problem(s), no page written\n".to_owned()
        )
    );
}

#[test]
fn fails_on_what_it_cannot_read() {
    let library = planted("unread", "/** C. */\nexport const C = { x: 1 };\n");

    assert_eq!(
        failure(&library),
        (
            Some(1),
            "math/vector/planted.ts:2: the generator cannot read this constant's type: write its type annotation\napi-pages: 1 problem(s), no page written\n".to_owned()
        )
    );
}
