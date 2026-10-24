//! Tests of `release notes` and `release check` from the binary (EN-117, F22.AC2) on temporary
//! repositories whose history is recorded here: `main` at `v0.2.0`, then on `develop` a fix, a
//! log, a feature merged by GitHub's merge commit, a feat and its revert; the features' text
//! drafted without a file nor a branch, then checked, rewritten and checked again.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

use release::draft::NO_CHANGE_NOTICE;

const PACKAGE: &str = "{\n  \"name\": \"editor\",\n  \"version\": \"0.0.0-development\",\n  \"repository\": {\n    \"type\": \"git\",\n    \"url\": \"git+https://github.com/o/r.git\"\n  }\n}\n";

/// A temporary repository, removed when the test ends, even on a failure.
struct Repository(PathBuf);

impl Drop for Repository {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Runs Git in a folder, its inherited `GIT_*` variables removed, and returns its output.
fn git(folder: &Path, args: &[&str]) -> String {
    let output = test_support::command("git")
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "-c",
            "tag.gpgsign=false",
        ])
        .args(args)
        .current_dir(folder)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// Writes a file of the repository, its folders made.
fn write(folder: &Path, file: &str, text: &str) {
    let path = folder.join(file);

    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

/// A commit of a new file named after the number of commits; its hash.
fn commit(folder: &Path, message: &str) -> String {
    let count = git(folder, &["rev-list", "--count", "--all"]);

    write(folder, &format!("work/{count}.txt"), message);
    git(folder, &["add", "."]);
    git(folder, &["commit", "-q", "-m", message]);
    git(folder, &["rev-parse", "HEAD"])
}

/// A repository whose `main` is at `v0.2.0`, `develop` checked out from it: `package.json`, a
/// story of a feature in the backlog.
fn repository(name: &str) -> Repository {
    let folder = std::env::temp_dir().join(format!("release-notes-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&folder);

    fs::create_dir_all(&folder).unwrap();
    git(&folder, &["init", "-q", "-b", "main"]);
    write(&folder, "package.json", PACKAGE);
    write(
        &folder,
        "docs/backlog/stories/E1-F16-US-041-colors.md",
        "---\nid: US-041\nepic: E1\nfeature: F16\ntitle: the colors chosen\nstatus: done\n---\n",
    );
    write(
        &folder,
        "docs/backlog/features/E1-F16-colors.md",
        "---\nid: F16\nepic: E1\ntitle: Colors\nstatus: done\n---\n",
    );
    git(&folder, &["add", "."]);
    git(&folder, &["commit", "-q", "-m", "chore: start"]);
    git(&folder, &["tag", "v0.2.0"]);
    git(&folder, &["switch", "-q", "-c", "develop"]);
    Repository(folder)
}

/// Runs the release binary in a folder.
fn release(folder: &Path, args: &[&str]) -> Output {
    test_support::command(env!("CARGO_BIN_EXE_release"))
        .args(args)
        .current_dir(folder)
        .output()
        .unwrap()
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The recorded history `main` lacks: a log and a fix on `develop`, F16 merged as pull request
/// #12 with its story's feat and fix and a tooling commit, a feat and its revert. Returns the
/// fix's hash.
fn history_of_f16(folder: &Path) -> String {
    commit(folder, "docs(docs): log the run");
    let fix = commit(folder, "fix(model): refuse a NaN");
    git(folder, &["switch", "-q", "-c", "feature/f16-colors"]);
    commit(
        folder,
        "feat(model): give a text its colors\n\nRefs: US-041.T1",
    );
    commit(folder, "fix(playground): keep the hue\n\nRefs: US-041.T2");
    commit(folder, "feat(tooling): a status line");
    git(folder, &["switch", "-q", "develop"]);
    git(
        folder,
        &[
            "merge",
            "-q",
            "--no-ff",
            "-m",
            "Merge pull request #12 from o/feature/f16-colors",
            "feature/f16-colors",
        ],
    );
    commit(folder, "feat(model): add a flag");
    git(folder, &["revert", "--no-edit", "HEAD"]);
    fix
}

#[test]
fn f22_ac2_drafts_the_features_text_without_a_file_nor_a_branch() {
    let repository = repository("draft");
    let folder = &repository.0;
    let fix = history_of_f16(folder);
    let branches = git(folder, &["branch", "--format=%(refname:short)"]);

    let drafted = release(folder, &["notes", "main"]);

    assert!(drafted.status.success(), "{}", stderr(&drafted));
    // The story's feat and fix are one Added line, with the feature's pull request; the fix of no
    // story is a line with its commit; the log, the tooling, the feat and its revert are left out.
    assert_eq!(
        stdout(&drafted),
        format!(
            "### Added\n\n- [draft] The colors chosen ([#12](https://github.com/o/r/pull/12))\n\n\
             ### Fixed\n\n- [draft] Refuse a NaN ([`{}`](https://github.com/o/r/commit/{fix}))\n",
            &fix[..7]
        )
    );
    let said = stderr(&drafted);
    assert!(
        said.contains("Added · F16 Colors · US-041: The colors chosen\n"),
        "{said}"
    );
    assert!(
        said.contains("feat(model): give a text its colors\n"),
        "{said}"
    );
    assert!(
        said.contains("2 line(s) drafted from the commits main lacks"),
        "{said}"
    );
    // Nothing written, no branch made, the checkout where it was.
    assert_eq!(
        git(folder, &["status", "--porcelain", "--untracked-files=all"]),
        ""
    );
    assert_eq!(
        git(folder, &["branch", "--format=%(refname:short)"]),
        branches
    );
    assert_eq!(git(folder, &["branch", "--show-current"]), "develop");

    // The draft is refused until each line is rewritten; what follows `---` is not read.
    let body = folder.join("body.md");
    let attribution = "\n---\n\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n";
    fs::write(&body, stdout(&drafted) + attribution).unwrap();
    let refused = release(folder, &["check", "body.md"]);
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(
        stderr(&refused)
            .lines()
            .map(|line| line.split(": a drafted").next().unwrap_or_default())
            .collect::<Vec<_>>(),
        ["body.md:3", "body.md:7"]
    );

    let rewritten = fs::read_to_string(&body)
        .unwrap()
        .replace("[draft] The colors chosen", "Choose a text's colors")
        .replace("[draft] ", "");
    fs::write(&body, rewritten).unwrap();
    let checked = release(folder, &["check", "body.md"]);
    assert!(checked.status.success(), "{}", stderr(&checked));
    assert_eq!(
        stdout(&checked),
        "body.md: the release text follows the rules release check reads\n"
    );
}

#[test]
fn f22_ac3_drafts_the_notice_for_tooling_only_and_nothing_without_a_commit() {
    let repository = repository("tooling");
    let folder = &repository.0;

    let nothing = release(folder, &["notes", "main"]);
    assert_eq!(nothing.status.code(), Some(1));
    assert_eq!(
        stderr(&nothing),
        "release notes: no commit since main: no text to draft\n"
    );

    commit(folder, "feat(tooling): a status line");
    commit(folder, "docs(docs): log the run");
    let tooling = release(folder, &["notes", "main"]);
    assert!(tooling.status.success(), "{}", stderr(&tooling));
    assert_eq!(stdout(&tooling), format!("{NO_CHANGE_NOTICE}\n"));
    assert!(stderr(&tooling).contains("0 line(s) drafted"));
    write(folder, "body.md", &stdout(&tooling));
    let checked = release(folder, &["check", "body.md"]);
    assert!(checked.status.success(), "{}", stderr(&checked));
}

#[test]
fn f22_ac2_reads_origin_main_by_default_and_needs_the_repository_s_address() {
    let repository = repository("default");
    let folder = &repository.0;
    commit(folder, "fix(io): x");

    // No remote here: the default base is missing, and Git says so.
    let missing = release(folder, &["notes"]);
    assert_eq!(missing.status.code(), Some(1));
    let said = stderr(&missing);
    assert!(said.starts_with("release notes: git log "), "{said}");
    assert!(said.contains("origin/main..HEAD"), "{said}");

    git(folder, &["update-ref", "refs/remotes/origin/main", "main"]);
    assert!(release(folder, &["notes"]).status.success());

    write(folder, "package.json", "{}");
    let anonymous = release(folder, &["notes", "main"]);
    assert_eq!(anonymous.status.code(), Some(1));
    assert_eq!(
        stderr(&anonymous),
        "release notes: package.json names no repository url\n"
    );
}

#[test]
fn says_how_to_call_it() {
    let usage = repository("usage");

    for args in [
        &["prepare"][..],
        &["notes", "main", "develop"][..],
        &["check"][..],
        &["check", "a.md", "b.md"][..],
    ] {
        let output = release(&usage.0, args);

        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(
            stderr(&output).starts_with("usage: target/release/release"),
            "{}",
            stderr(&output)
        );
    }

    let missing = release(&usage.0, &["check", "missing.md"]);
    assert_eq!(missing.status.code(), Some(1));
    assert!(
        stderr(&missing).starts_with("release check: missing.md: "),
        "{}",
        stderr(&missing)
    );
}
