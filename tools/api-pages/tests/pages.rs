//! Tests of `api-pages` on a library of its own, `fixtures/library`: modules of `src/` copied
//! unchanged, so that each page is compared byte for byte with the one `TypeDoc` wrote for them on
//! 2026-10-17 (`fixtures/typedoc`, `.txt` so that no formatter touches them). `[F19.AC7]` marks
//! a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A temporary output folder, removed when the test ends, even on a failure.
struct Folder(PathBuf);

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
        let _ = fs::remove_file(&self.0);
    }
}

/// An empty temporary folder named after the test.
fn folder(name: &str) -> Folder {
    let path = std::env::temp_dir().join(format!("api-pages-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&path);
    Folder(path)
}

/// The fixtures' folder.
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

/// `api-pages` run on an entry module, its pages written into a folder.
fn run(entry: &Path, out: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_api-pages"))
        .arg(entry)
        .arg(out)
        .arg("/api/")
        .arg("editor")
        .output()
        .unwrap()
}

/// The pages of the fixture library, written into a fresh folder.
fn written(name: &str) -> Folder {
    let out = folder(name);
    let output = run(&fixtures().join("library").join("index.ts"), &out.0);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    out
}

#[test]
fn f19_ac7_writes_each_page_as_typedoc_wrote_it() {
    let out = written("same");
    let expected = fixtures().join("typedoc");
    let mut compared = 0;

    for kind in ["functions", "type-aliases", "variables"] {
        for entry in fs::read_dir(expected.join(kind)).unwrap() {
            let path = entry.unwrap().path();
            let page = path.file_stem().unwrap().to_string_lossy().into_owned();
            let ours = fs::read_to_string(out.0.join(kind).join(&page)).unwrap();
            let theirs = fs::read_to_string(&path).unwrap();
            assert_eq!(ours, theirs, "{kind}/{page}");
            compared += 1;
        }
    }

    assert_eq!(compared, 11);
}

#[test]
fn f19_ac7_writes_one_page_per_export_and_no_other() {
    let out = written("count");
    let count = |kind: &str| fs::read_dir(out.0.join(kind)).unwrap().count();

    assert_eq!(
        (
            count("functions"),
            count("type-aliases"),
            count("variables")
        ),
        (3, 5, 3)
    );
}

#[test]
fn f19_ac7_writes_the_index_and_the_sidebar_sorted_by_kind_and_name() {
    let out = written("index");
    let index = fs::read_to_string(out.0.join("index.md")).unwrap();
    let sidebar = fs::read_to_string(out.0.join("api-sidebar.json")).unwrap();

    assert_eq!(
        index,
        "# editor\n\n## Type Aliases\n\n- [Alignment](type-aliases/Alignment.md)\n- [Arc](type-aliases/Arc.md)\n- [Point](type-aliases/Point.md)\n- [ScanStep](type-aliases/ScanStep.md)\n- [Vector](type-aliases/Vector.md)\n\n## Variables\n\n- [ALIGNMENTS](variables/ALIGNMENTS.md)\n- [DEGREES\\_PER\\_TURN](variables/DEGREES_PER_TURN.md)\n- [TEXT\\_RANGES](variables/TEXT_RANGES.md)\n\n## Functions\n\n- [add](functions/add.md)\n- [bisection](functions/bisection.md)\n- [parseCollectionFaces](functions/parseCollectionFaces.md)\n"
    );
    assert_eq!(
        sidebar,
        "[{\"text\":\"Type Aliases\",\"collapsed\":true,\"items\":[{\"text\":\"Alignment\",\"link\":\"/api/type-aliases/Alignment.md\"},{\"text\":\"Arc\",\"link\":\"/api/type-aliases/Arc.md\"},{\"text\":\"Point\",\"link\":\"/api/type-aliases/Point.md\"},{\"text\":\"ScanStep\",\"link\":\"/api/type-aliases/ScanStep.md\"},{\"text\":\"Vector\",\"link\":\"/api/type-aliases/Vector.md\"}]},{\"text\":\"Variables\",\"collapsed\":true,\"items\":[{\"text\":\"ALIGNMENTS\",\"link\":\"/api/variables/ALIGNMENTS.md\"},{\"text\":\"DEGREES_PER_TURN\",\"link\":\"/api/variables/DEGREES_PER_TURN.md\"},{\"text\":\"TEXT_RANGES\",\"link\":\"/api/variables/TEXT_RANGES.md\"}]},{\"text\":\"Functions\",\"collapsed\":true,\"items\":[{\"text\":\"add\",\"link\":\"/api/functions/add.md\"},{\"text\":\"bisection\",\"link\":\"/api/functions/bisection.md\"},{\"text\":\"parseCollectionFaces\",\"link\":\"/api/functions/parseCollectionFaces.md\"}]}]"
    );
}

#[test]
fn empties_the_folder_first() {
    let out = folder("stale");
    fs::create_dir_all(out.0.join("functions")).unwrap();
    fs::write(out.0.join("functions").join("removed.md"), "old").unwrap();

    let output = run(&fixtures().join("library").join("index.ts"), &out.0);

    assert!(output.status.success());
    assert!(!out.0.join("functions").join("removed.md").exists());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("api-pages: 11 pages written to {}\n", out.0.display())
    );
}

#[test]
fn refuses_a_wrong_number_of_arguments() {
    let output = Command::new(env!("CARGO_BIN_EXE_api-pages"))
        .arg("src/index.ts")
        .output()
        .unwrap();

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("usage: api-pages"));
}

#[test]
fn names_a_folder_it_cannot_write() {
    let out = folder("file");
    fs::write(&out.0, "a file, not a folder").unwrap();
    let blocked = out.0.join("api");

    let output = run(&fixtures().join("library").join("index.ts"), &blocked);

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("api-pages: cannot write"));
}
