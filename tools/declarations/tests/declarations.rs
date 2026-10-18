//! Tests of `declarations` on a library of its own, written in a temporary folder.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A temporary folder, removed when the test ends, even on a failure.
struct Folder(PathBuf);

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A library of the given modules, (path, text), in a folder named after the test.
fn library(name: &str, modules: &[(&str, &str)]) -> Folder {
    let root = std::env::temp_dir().join(format!("declarations-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&root);

    for (path, text) in modules {
        let file = root.join("src").join(path);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }

    Folder(root)
}

/// Runs the tool from the library's `src` into its `types`.
fn declarations(root: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_declarations"))
        .arg(root.join("src"))
        .arg(root.join("types"))
        .output()
        .unwrap()
}

#[test]
fn writes_one_declaration_file_per_module_its_tests_left_out() {
    let folder = library(
        "modules",
        &[
            (
                "math/half.ts",
                "/** Half of a value. */\nexport function half(value: number): number {\n  return value / 2;\n}\n",
            ),
            ("math/half.test.ts", "half(2);\n"),
            ("index.ts", "export * from \"./math/half\";\n"),
        ],
    );

    let output = declarations(&folder.0);

    assert!(output.status.success());
    assert_eq!(
        fs::read_to_string(folder.0.join("types/math/half.d.ts")).unwrap(),
        "/** Half of a value. */\nexport declare function half(value: number): number;\n"
    );
    assert_eq!(
        fs::read_to_string(folder.0.join("types/index.d.ts")).unwrap(),
        "export * from \"./math/half\";\n"
    );
    assert!(!folder.0.join("types/math/half.test.d.ts").exists());
}

#[test]
fn reads_a_constant_type_from_a_plain_value() {
    let folder = library("literal", &[("turn.ts", "export const TURN = 360;\n")]);

    assert!(declarations(&folder.0).status.success());
    assert_eq!(
        fs::read_to_string(folder.0.join("types/turn.d.ts")).unwrap(),
        "export declare const TURN = 360;\n"
    );
}

#[test]
fn refuses_a_constant_whose_type_is_neither_written_nor_plain_and_writes_nothing() {
    let folder = library(
        "computed",
        &[
            ("turn.ts", "export const TURN = 360;\n"),
            (
                "half-turn.ts",
                "import { TURN } from \"./turn\";\n\nexport const HALF_TURN = TURN / 2;\n",
            ),
        ],
    );

    let output = declarations(&folder.0);
    let errors = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1));
    assert!(errors.contains("half-turn.ts, line 3: TS9010"), "{errors}");
    assert!(!folder.0.join("types").exists());
}

#[test]
fn reports_a_syntax_error_with_its_module() {
    let folder = library("syntax", &[("broken.ts", "export const = ;\n")]);

    let output = declarations(&folder.0);

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("broken.ts, line"));
}

#[test]
fn asks_for_both_folders() {
    let output = Command::new(env!("CARGO_BIN_EXE_declarations"))
        .arg("src")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn reports_a_missing_source_folder() {
    let folder = library("missing", &[]);

    assert_eq!(declarations(&folder.0).status.code(), Some(2));
}
