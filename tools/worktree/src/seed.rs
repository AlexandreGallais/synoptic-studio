//! The release build of a new worktree seeded from the main checkout's (EN-113): `target/release`
//! copied before `cargo build`, so that Cargo rebuilds only what differs instead of every crate.
//! Measured on 2026-10-18 on a copy of the feature branch: 34.3 s from nothing, 15.7 s seeded
//! from a `develop` checkout's build (0.5 s of copy for 3,467 files and 1.2 GB, APFS cloning each
//! file), the workspace's own crates and those `develop` lacks compiled again.

use std::fs;
use std::io;
use std::path::Path;

/// The release build's folder, from a checkout's root.
pub const RELEASE: &str = "target/release";

/// The folders of the build left out of the copy: `incremental` is per checkout, and empty in a
/// release build without `CARGO_INCREMENTAL`.
const SKIPPED: [&str; 1] = ["incremental"];

/// Copies the main checkout's `target/release` into a new worktree's, when the worktree holds the
/// Cargo workspace, the main checkout has a release build and the worktree none yet; the number of
/// files copied, `None` when nothing was to copy. Each file is copied by `fs::copy`, which clones
/// it on APFS; a symbolic link is left out.
///
/// # Errors
///
/// When a folder cannot be read or created, or a file copied.
pub fn seed_release(main: &Path, worktree: &Path) -> io::Result<Option<u64>> {
    let from = main.join(RELEASE);
    let to = worktree.join(RELEASE);

    if !worktree.join("Cargo.toml").is_file() || !from.is_dir() || to.exists() {
        return Ok(None);
    }

    copy_tree(&from, &to).map(Some)
}

/// Copies a folder's files and folders into a new one, [`SKIPPED`] left out at every level; the
/// number of files copied.
fn copy_tree(from: &Path, to: &Path) -> io::Result<u64> {
    fs::create_dir_all(to)?;

    let mut copied = 0;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let name = entry.file_name();

        if kind.is_dir() && !SKIPPED.iter().any(|skipped| name == *skipped) {
            copied += copy_tree(&entry.path(), &to.join(&name))?;
        } else if kind.is_file() {
            fs::copy(entry.path(), to.join(&name))?;
            copied += 1;
        }
    }

    Ok(copied)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A new folder of the test's own.
    fn scratch(name: &str) -> std::path::PathBuf {
        let folder =
            std::env::temp_dir().join(format!("worktree-seed-{}-{name}", std::process::id()));

        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("folder");
        folder
    }

    /// A main checkout with a release build: a binary, a dependency, an incremental cache.
    fn main_checkout(root: &Path) -> std::path::PathBuf {
        let main = root.join("main");

        fs::create_dir_all(main.join("target/release/deps")).expect("deps");
        fs::create_dir_all(main.join("target/release/incremental/a")).expect("incremental");
        fs::write(main.join("target/release/page"), "binary").expect("binary");
        fs::write(main.join("target/release/deps/liba.rlib"), "rlib").expect("rlib");
        fs::write(main.join("target/release/incremental/a/b"), "cache").expect("cache");
        main
    }

    #[test]
    fn copies_the_release_build_without_its_incremental_cache() {
        let root = scratch("copied");
        let main = main_checkout(&root);
        let worktree = root.join("worktree");

        fs::create_dir_all(&worktree).expect("worktree");
        fs::write(worktree.join("Cargo.toml"), "").expect("manifest");

        assert_eq!(seed_release(&main, &worktree).expect("seeded"), Some(2));
        assert_eq!(
            fs::read_to_string(worktree.join("target/release/page")).expect("page"),
            "binary"
        );
        assert_eq!(
            fs::read_to_string(worktree.join("target/release/deps/liba.rlib")).expect("rlib"),
            "rlib"
        );
        assert!(!worktree.join("target/release/incremental").exists());
        fs::remove_dir_all(&root).expect("cleaned");
    }

    #[test]
    fn copies_nothing_without_a_workspace_a_build_or_into_a_build() {
        let root = scratch("nothing");
        let main = main_checkout(&root);
        let bare = root.join("bare");
        let built = root.join("built");
        let empty = root.join("empty");

        // No Cargo workspace in the worktree: nothing to build, nothing copied.
        fs::create_dir_all(&bare).expect("bare");
        assert_eq!(seed_release(&main, &bare).expect("checked"), None);
        assert!(!bare.join("target").exists());
        // A release build already there is left as it is.
        fs::create_dir_all(built.join("target/release")).expect("built");
        fs::write(built.join("Cargo.toml"), "").expect("manifest");
        assert_eq!(seed_release(&main, &built).expect("checked"), None);
        assert!(!built.join("target/release/page").exists());
        // A main checkout without a release build.
        let fresh = root.join("fresh");

        fs::create_dir_all(&empty).expect("empty");
        fs::create_dir_all(&fresh).expect("fresh");
        fs::write(fresh.join("Cargo.toml"), "").expect("manifest");
        assert_eq!(seed_release(&empty, &fresh).expect("checked"), None);
        assert!(!fresh.join("target").exists());
        fs::remove_dir_all(&root).expect("cleaned");
    }
}
