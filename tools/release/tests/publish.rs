//! Tests of `release publish` (EN-112, F21.AC6): the GitHub release made on the merge commit with
//! the entry as its notes and the test report of the pull request's CI, no commit made; nothing
//! done for a version already released; refused for an entry that breaks a rule, a head that is
//! not a merge, a pull request without a successful CI run. GitHub's answers are given by a fake.

use std::cell::RefCell;
use std::fs;
#[cfg(unix)]
use std::path::Path;
use std::path::PathBuf;

use release::changelog::HEADER;
use release::publish::{Published, publish};

const PACKAGE: &str = "{\n  \"name\": \"editor\",\n  \"version\": \"0.3.0\",\n  \"repository\": {\n    \"url\": \"git+https://github.com/o/r.git\"\n  }\n}\n";

/// A changelog whose latest entry, 0.3.0, follows the rules.
fn changelog() -> String {
    format!(
        "{HEADER}\n## [0.3.0] - 2026-10-18\n\n### Added\n\n- Choose a text's colors ([#12](https://github.com/o/r/pull/12))\n\n[0.3.0]: https://github.com/o/r/releases/tag/v0.3.0\n"
    )
}

/// A folder of its own, removed when the test ends.
struct Folder(PathBuf);

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// A folder holding `package.json` and `CHANGELOG.md`.
fn folder(name: &str, changelog: &str) -> Folder {
    let path = std::env::temp_dir().join(format!(
        "release-publish-test-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&path);

    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("package.json"), PACKAGE).unwrap();
    fs::write(path.join("CHANGELOG.md"), changelog).unwrap();
    Folder(path)
}

/// The answers of a fake Git and GitHub, keyed by the call's first words, and each call made.
struct Fake {
    answers: Vec<(&'static str, Result<&'static str, &'static str>)>,
    calls: RefCell<Vec<String>>,
}

impl Fake {
    fn new(answers: Vec<(&'static str, Result<&'static str, &'static str>)>) -> Fake {
        Fake {
            answers,
            calls: RefCell::new(Vec::new()),
        }
    }

    /// Answers a call by the first answer whose key starts it; a call without one succeeds empty.
    fn answer(&self, program: &str, args: &[&str]) -> Result<String, String> {
        let call = format!("{program} {}", args.join(" "));

        self.calls.borrow_mut().push(call.clone());
        self.answers
            .iter()
            .find(|(key, _)| call.starts_with(key))
            .map_or(Ok(String::new()), |(_, answer)| {
                answer.map(str::to_owned).map_err(str::to_owned)
            })
    }
}

/// The answers of a merge into `main` whose pull request's CI passed.
fn merged() -> Vec<(&'static str, Result<&'static str, &'static str>)> {
    vec![
        ("gh release view", Err("release not found")),
        ("git rev-parse HEAD^2", Ok("bbbb\n")),
        ("git rev-parse HEAD", Ok("aaaa\n")),
        ("gh run list", Ok("4242\n")),
    ]
}

fn published(folder: &Folder, fake: &Fake) -> Result<Published, String> {
    let mut run = |program: &str, args: &[&str]| fake.answer(program, args);

    publish(&folder.0, &mut run, &folder.0.join("out"))
}

#[test]
fn f21_ac6_releases_the_merge_commit_with_the_entry_and_the_test_report() {
    let folder = folder("released", &changelog());
    let fake = Fake::new(merged());
    let out = folder.0.join("out");
    let out_text = out.to_string_lossy().into_owned();

    assert_eq!(
        published(&folder, &fake),
        Ok(Published::Released("v0.3.0".to_owned()))
    );
    // The run of the pull request's head, `HEAD^2`; the release on the merge commit, `HEAD`.
    assert_eq!(
        *fake.calls.borrow(),
        [
            "gh release view v0.3.0 --json tagName".to_owned(),
            "git rev-parse HEAD".to_owned(),
            "git rev-parse HEAD^2".to_owned(),
            "gh run list --workflow ci.yml --commit bbbb --status success --limit 1 --json databaseId --jq .[0].databaseId".to_owned(),
            format!("gh run download 4242 --name test-report --dir {out_text}"),
            format!(
                "gh release create v0.3.0 --target aaaa --title 0.3.0 --notes-file {out_text}/notes.md {out_text}/test-report.xml"
            ),
        ]
    );
    assert_eq!(
        fs::read_to_string(out.join("notes.md")).unwrap(),
        "### Added\n\n- Choose a text's colors ([#12](https://github.com/o/r/pull/12))\n"
    );
    // No call commits, tags by Git or pushes.
    assert!(!fake.calls.borrow().iter().any(|call| {
        call.starts_with("git commit")
            || call.starts_with("git tag")
            || call.starts_with("git push")
    }));
}

#[test]
fn f21_ac6_does_nothing_for_a_version_already_released() {
    let folder = folder("already", &changelog());
    let fake = Fake::new(vec![("gh release view", Ok("{\"tagName\":\"v0.3.0\"}"))]);

    assert_eq!(
        published(&folder, &fake),
        Ok(Published::Already("v0.3.0".to_owned()))
    );
    assert_eq!(fake.calls.borrow().len(), 1);
}

#[test]
fn refuses_a_broken_entry_a_head_not_merged_and_a_pull_request_without_ci() {
    let drafted = folder(
        "drafted",
        &changelog().replace("- Choose", "- [draft] Choose"),
    );
    let fake = Fake::new(merged());
    let refused = published(&drafted, &fake).unwrap_err();

    assert!(
        refused.starts_with("CHANGELOG.md:9: a drafted line is rewritten"),
        "{refused}"
    );
    assert!(fake.calls.borrow().is_empty());

    let folder = folder("refused", &changelog());
    let mut not_merged = merged();
    not_merged.insert(0, ("git rev-parse HEAD^2", Err("unknown revision")));
    assert_eq!(
        published(&folder, &Fake::new(not_merged)),
        Err("aaaa is not the merge commit of a pull request".to_owned())
    );

    for no_run in ["null\n", "\n"] {
        let mut without_ci = merged();
        without_ci.insert(0, ("gh run list", Ok(no_run)));
        assert_eq!(
            published(&folder, &Fake::new(without_ci)),
            Err("no successful CI run of bbbb, the pull request's head: no test report".to_owned())
        );
    }

    let mut failing = merged();
    failing.insert(0, ("gh release create", Err("HTTP 403")));
    assert_eq!(
        published(&folder, &Fake::new(failing)),
        Err("HTTP 403".to_owned())
    );

    let missing =
        Folder(std::env::temp_dir().join(format!("release-publish-none-{}", std::process::id())));
    assert!(
        published(&missing, &Fake::new(merged()))
            .unwrap_err()
            .starts_with("package.json: ")
    );
}

/// A fake `gh`: logs each call, says a release exists when its marker file does, gives run 4242,
/// writes the report where `--dir` says, and logs the notes of `--notes-file`.
#[cfg(unix)]
const FAKE_GH: &str = "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$FAKE_GH_LOG\"\ncase \"$1 $2\" in\n\
     \"release view\") [ -f \"$FAKE_GH_RELEASED\" ] && exit 0; echo 'release not found' >&2; exit 1;;\n\
     \"run list\") echo 4242;;\n\
     \"run download\") while [ $# -gt 0 ]; do [ \"$1\" = --dir ] && touch \"$2/test-report.xml\"; shift; done;;\n\
     \"release create\") while [ $# -gt 0 ]; do [ \"$1\" = --notes-file ] && cat \"$2\" >> \"$FAKE_GH_LOG\"; shift; done;;\n\
     esac\n";

/// Runs Git in a folder, its inherited `GIT_*` variables removed; its output.
#[cfg(unix)]
fn git(root: &Path, args: &[&str]) -> String {
    let output = test_support::command("git")
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["-c", "commit.gpgsign=false"])
        .args(args)
        .current_dir(root)
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// A repository whose `main` merges `release/0.3.0` as pull request #20: the pull request's head
/// and the merge commit.
#[cfg(unix)]
fn merged_release(root: &Path) -> (String, String) {
    git(root, &["init", "-q", "-b", "main"]);
    git(
        root,
        &["commit", "-q", "--allow-empty", "-m", "chore: start"],
    );
    git(root, &["switch", "-q", "-c", "release/0.3.0"]);
    fs::write(root.join("CHANGELOG.md"), changelog()).unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", "chore: release 0.3.0"]);
    let pull_request_head = git(root, &["rev-parse", "HEAD"]);
    git(root, &["switch", "-q", "main"]);
    let message = "Merge pull request #20 from o/release/0.3.0";
    git(
        root,
        &["merge", "-q", "--no-ff", "-m", message, "release/0.3.0"],
    );

    (pull_request_head, git(root, &["rev-parse", "HEAD"]))
}

/// The binary on a repository whose head merges a release branch, GitHub played by a fake `gh`
/// first on the `PATH`, a shell script: Unix only, for its execute bit.
#[cfg(unix)]
#[test]
fn f21_ac6_publishes_from_the_binary_and_says_so_to_the_workflow() {
    use std::os::unix::fs::PermissionsExt;

    let folder = folder("binary", "");
    let root = &folder.0;
    let (pull_request_head, merge) = merged_release(root);
    let bin = root.join("bin");
    let log = root.join("gh.log");
    let output = root.join("github-output");
    fs::create_dir_all(&bin).unwrap();
    fs::write(bin.join("gh"), FAKE_GH).unwrap();
    fs::set_permissions(bin.join("gh"), fs::Permissions::from_mode(0o755)).unwrap();
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let released_marker = root.join("released");
    let publish = || {
        test_support::command(env!("CARGO_BIN_EXE_release"))
            .arg("publish")
            .env("PATH", &path)
            .env("FAKE_GH_LOG", &log)
            .env("FAKE_GH_RELEASED", &released_marker)
            .env("GITHUB_OUTPUT", &output)
            .current_dir(root)
            .output()
            .unwrap()
    };

    let first = publish();
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(String::from_utf8_lossy(&first.stdout).starts_with("released v0.3.0"));
    let said = fs::read_to_string(&log).unwrap();
    let listed = format!("run list --workflow ci.yml --commit {pull_request_head} ");
    assert!(said.contains(&listed), "{said}");
    let created = format!("release create v0.3.0 --target {merge} --title 0.3.0 ");
    assert!(said.contains(&created), "{said}");
    assert!(said.contains("- Choose a text's colors ([#12]"), "{said}");
    assert_eq!(
        fs::read_to_string(&output).unwrap(),
        "released=true\ntag=v0.3.0\n"
    );
    // No commit: the head is still the merge.
    assert_eq!(git(root, &["rev-parse", "HEAD"]), merge);

    fs::write(&released_marker, "").unwrap();
    let second = publish();
    assert!(second.status.success());
    assert!(String::from_utf8_lossy(&second.stdout).contains("v0.3.0 is already released"));
    let outputs = fs::read_to_string(&output).unwrap();
    assert!(
        outputs.ends_with("released=false\ntag=v0.3.0\n"),
        "{outputs}"
    );

    fs::write(root.join("CHANGELOG.md"), "# Changelog\n").unwrap();
    let broken = publish();
    assert_eq!(broken.status.code(), Some(1));
    let refused = String::from_utf8_lossy(&broken.stderr).into_owned();
    assert!(
        refused.starts_with("release publish: CHANGELOG.md:1:"),
        "{refused}"
    );
}
