//! Tests of the stale release binaries (EN-104, ADR-0078), on a workspace of its own whose files'
//! times are set.

use std::fs::{self, File, FileTimes};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use stale_binaries::{binaries, dep_info_files, package_name, stale_binaries, verdict};

/// A file written with its text, then given a time `seconds` after a fixed instant.
fn file(path: &Path, text: &str, seconds: u64) {
    fs::create_dir_all(path.parent().expect("a parent")).expect("folder");
    fs::write(path, text).expect("written");
    File::options()
        .write(true)
        .open(path)
        .and_then(|opened| {
            opened.set_times(
                FileTimes::new().set_modified(
                    SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000 + seconds),
                ),
            )
        })
        .expect("time set");
}

/// A workspace of two crates: `alpha`, one `main.rs`; `beta`, the binaries `x` and `y`. `alpha`
/// was built after its source, `x` before its own, `y` never; `y.rs`, newer than `x`, is no source
/// of `x`.
fn workspace(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("stale-binaries-{name}-{}", std::process::id()));
    let release = root.join("target/release");
    let alpha = root.join("tools/alpha/src/main.rs");
    let x = root.join("tools/b/src/bin/x.rs");

    let _ = fs::remove_dir_all(&root);
    file(
        &root.join("tools/alpha/Cargo.toml"),
        "[package]\nname = \"alpha\"\n\n[dependencies]\nname = \"not this\"\n",
        0,
    );
    file(&alpha, "fn main() {}\n", 10);
    file(
        &root.join("tools/b/Cargo.toml"),
        "[package]\nname = \"beta\"\n",
        0,
    );
    file(&x, "fn main() {}\n", 30);
    file(&root.join("tools/b/src/bin/y.rs"), "fn main() {}\n", 40);
    file(&root.join("tools/b/src/bin/notes.txt"), "", 50);
    file(&release.join("alpha"), "", 20);
    file(
        &release.join("alpha.d"),
        &format!("{}: {}\n", release.join("alpha").display(), alpha.display()),
        20,
    );
    file(&release.join("x"), "", 20);
    file(
        &release.join("x.d"),
        &format!("{}: {}\n", release.join("x").display(), x.display()),
        20,
    );
    root
}

#[test]
fn reads_the_package_name_under_package_only() {
    assert_eq!(
        package_name("[package]\nname = \"hooks\"\nversion = \"0.1.0\"\n"),
        "hooks"
    );
    assert_eq!(package_name("[dependencies]\nname = \"x\"\n"), "");
    assert_eq!(package_name("[package]\nnamed = \"x\"\n"), "");
}

#[test]
fn reads_the_files_of_a_dep_info_spaces_escaped() {
    assert_eq!(
        dep_info_files(
            "/r/SVG\\ editor/target/release/a: /r/SVG\\ editor/a.rs /r/b.rs\n\n/r/b.rs:\n"
        ),
        [
            PathBuf::from("/r/SVG editor/a.rs"),
            PathBuf::from("/r/b.rs")
        ]
    );
    assert_eq!(dep_info_files(""), Vec::<PathBuf>::new());
}

#[test]
fn tells_a_fresh_binary_from_a_stale_or_missing_one() {
    let at = |seconds| SystemTime::UNIX_EPOCH + Duration::from_secs(seconds);

    assert_eq!(verdict("a", Some(at(5)), Some(("s.rs", at(4)))), None);
    // Written in the same second as its build: fresh, as Cargo would leave it.
    assert_eq!(verdict("a", Some(at(5)), Some(("s.rs", at(5)))), None);
    assert_eq!(
        verdict("a", Some(at(5)), Some(("s.rs", at(6)))).as_deref(),
        Some("target/release/a is older than s.rs")
    );
    assert_eq!(
        verdict("a", None, None).as_deref(),
        Some("target/release/a is missing")
    );
    assert_eq!(verdict("a", Some(at(5)), None), None);
}

#[test]
fn f21_ac1_names_each_binary_older_than_its_sources_and_each_missing_one() {
    let root = workspace("lib");

    assert_eq!(binaries(&root), ["alpha", "x", "y"]);
    assert_eq!(
        stale_binaries(&root),
        [
            "target/release/x is older than tools/b/src/bin/x.rs",
            "target/release/y is missing"
        ]
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn names_the_newest_source_and_reads_only_the_listed_ones() {
    // `g`, built at 20, lists two sources, written at 10 and 25: the newest, `b.rs`, makes it
    // stale (the oldest would not); `new.rs`, at 60, is listed by no dep-info: no line for it.
    let root = std::env::temp_dir().join(format!("stale-binaries-newest-{}", std::process::id()));
    let release = root.join("target/release");
    let (a, b) = (root.join("tools/g/src/a.rs"), root.join("tools/g/src/b.rs"));

    let _ = fs::remove_dir_all(&root);
    file(
        &root.join("tools/g/Cargo.toml"),
        "[package]\nname = \"g\"\n",
        0,
    );
    file(&root.join("tools/g/src/main.rs"), "fn main() {}\n", 0);
    file(&a, "", 10);
    file(&b, "", 25);
    file(&root.join("tools/g/src/new.rs"), "", 60);
    file(&release.join("g"), "", 20);
    file(
        &release.join("g.d"),
        &format!(
            "{}: {} {}\n",
            release.join("g").display(),
            a.display(),
            b.display()
        ),
        20,
    );
    assert_eq!(
        stale_binaries(&root),
        ["target/release/g is older than tools/g/src/b.rs"]
    );
    file(&b, "", 15);
    assert_eq!(stale_binaries(&root), [] as [String; 0]);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn fails_naming_the_stale_binaries_and_passes_once_they_are_rebuilt() {
    let root = workspace("binary");
    let run = || {
        test_support::command(env!("CARGO_BIN_EXE_stale-binaries"))
            .arg(&root)
            .output()
            .expect("runs")
    };
    let output = run();

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "target/release/x is older than tools/b/src/bin/x.rs\ntarget/release/y is missing\n"
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("run `npm run rust:build`"));
    // Rebuilt: both binaries written after their sources.
    file(&root.join("target/release/x"), "", 60);
    file(&root.join("target/release/y"), "", 60);
    let output = run();

    assert!(output.status.success());
    assert_eq!(output.stdout, [] as [u8; 0]);
    let _ = fs::remove_dir_all(&root);
}
