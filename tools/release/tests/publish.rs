//! Tests of `release publish` (EN-117, F22.AC2, F22.AC3): the version computed from the last tag
//! and the commits since, the GitHub release made on `main`'s head with the bodies of the pull
//! requests merged since that tag joined and the test report of the last one's CI, no commit nor
//! tag made by Git; nothing done without a commit since the tag; a release of tooling only a
//! patch with its notice; the body of a Dependabot pull request left out (AUD-024); refused for a
//! body that breaks a rule, two notices, notes that disagree with the commits (AUD-024), a head
//! that is not a merge, a pull request without a successful CI run. GitHub is played by a fake:
//! nothing here tags nor releases the real repository.

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

use release::publish::{Published, publish};

/// A folder of its own, removed when the test ends.
struct Folder(PathBuf);

impl Drop for Folder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn folder(name: &str) -> Folder {
    let path = std::env::temp_dir().join(format!(
        "release-publish-test-{}-{name}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&path);

    Folder(path)
}

/// An answer of the fake: its output, or the error a command printed.
type Answer = Result<String, String>;

/// The answers of a fake Git and GitHub, keyed by the call's first words, and each call made.
struct Fake {
    answers: Vec<(String, Answer)>,
    calls: RefCell<Vec<String>>,
}

impl Fake {
    fn new(answers: Vec<(String, Answer)>) -> Fake {
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
            .find(|(key, _)| call.starts_with(key.as_str()))
            .map_or(Ok(String::new()), |(_, answer)| answer.clone())
    }
}

/// One answer.
fn answer(key: &str, output: Answer) -> (String, Answer) {
    (key.to_owned(), output)
}

/// `git log`'s record of a commit: its hash of one repeated digit, its subject, its body.
fn record(digit: char, subject: &str, body: &str) -> String {
    format!(
        "{}\u{1f}{subject}\u{1f}{body}\u{1e}\n",
        digit.to_string().repeat(40)
    )
}

/// The answers of `main` after `tag`: the commits given, merged into `main` as pull requests #21,
/// the latest, then #20, whose bodies are given; the head `aaaa` merges `bbbb`, whose CI passed.
fn on_main(tag: &str, commits: &str, bodies: [&str; 2]) -> Vec<(String, Answer)> {
    let merges = "m1\u{1f}p1 q1\u{1f}Merge pull request #21 from o/develop\u{1e}\n\
                  m2\u{1f}p2 q2\u{1f}Merge pull request #20 from o/develop\u{1e}\n\
                  m3\u{1f}p3 q3\u{1f}Merge branch 'main' into develop\u{1e}\n";

    vec![
        answer("git describe", Ok(format!("{tag}\n"))),
        answer("git log --no-merges", Ok(commits.to_owned())),
        answer("git log --first-parent", Ok(merges.to_owned())),
        answer("gh pr view 21", Ok(format!("{}\n", bodies[0]))),
        answer("gh pr view 20", Ok(format!("{}\n", bodies[1]))),
        answer("git rev-parse HEAD^2", Ok("bbbb\n".to_owned())),
        answer("git rev-parse HEAD", Ok("aaaa\n".to_owned())),
        answer("gh run list", Ok("4242\n".to_owned())),
    ]
}

/// A feature's commit, which raises the minor version.
fn a_feature() -> String {
    record(
        '1',
        "feat(model): give a text its colors",
        "Refs: US-041.T1\n",
    ) + &record('2', "docs(docs): log the run", "")
}

/// The body of #21, as GitHub gives a body written in its page: `\r\n`, then the attribution
/// after a break.
const BODY_21: &str = "### Added\r\n\r\n- Choose a text's colors ([#12](https://github.com/o/r/pull/12))\r\n\r\n---\r\n\r\n🤖 Generated with [Claude Code](https://claude.com/claude-code)";

/// The body of #20.
const BODY_20: &str = "### Fixed\n\n- Refuse a NaN ([#11](https://github.com/o/r/pull/11))";

fn published(fake: &Fake, out: &Folder) -> Result<Published, String> {
    let mut run = |program: &str, args: &[&str]| fake.answer(program, args);

    publish(&mut run, "refs/heads/main", &out.0)
}

#[test]
fn f28_ac2_refuses_a_branch_other_than_main_and_makes_nothing() {
    let out = folder("branch");
    let fake = Fake::new(on_main("v0.2.0", &a_feature(), [BODY_21, BODY_20]));
    let mut run = |program: &str, args: &[&str]| fake.answer(program, args);

    // The button pressed on develop, then GITHUB_REF unset, as on a laptop: no call at all.
    assert_eq!(
        publish(&mut run, "refs/heads/develop", &out.0),
        Err(
            "the run is on the ref \"refs/heads/develop\", not \"refs/heads/main\": a release is \
             made from the branch main only, so press the button on main (GITHUB_REF)"
                .to_owned()
        )
    );
    assert_eq!(
        publish(&mut run, "", &out.0),
        Err(
            "the run is on the ref \"\", not \"refs/heads/main\": a release is made from the \
             branch main only, so press the button on main (GITHUB_REF)"
                .to_owned()
        )
    );
    // A tag named main, whose short name GITHUB_REF_NAME is main too (REF-GH-VARIABLES), is not
    // the branch main; nor is a short name, nor a branch whose name holds main.
    assert!(publish(&mut run, "refs/tags/main", &out.0).is_err());
    assert!(publish(&mut run, "main", &out.0).is_err());
    assert!(publish(&mut run, "refs/heads/feature/main", &out.0).is_err());
    assert!(fake.calls.borrow().is_empty(), "{:?}", fake.calls.borrow());
    assert!(!out.0.exists());
}

#[test]
fn f22_ac2_releases_the_head_with_the_bodies_joined_and_the_test_report() {
    let out = folder("released");
    let fake = Fake::new(on_main("v0.2.0", &a_feature(), [BODY_21, BODY_20]));
    let out_text = out.0.to_string_lossy().into_owned();

    // A feat since v0.2.0, no break: 0.3.0.
    assert_eq!(
        published(&fake, &out),
        Ok(Published::Released("v0.3.0".to_owned()))
    );
    let range = "v0.2.0..HEAD";
    let merges = format!("git log --first-parent --merges --format=%H%x1f%P%x1f%s%x1e {range}");
    assert_eq!(
        *fake.calls.borrow(),
        [
            "git describe --tags --abbrev=0 --match v[0-9]*.[0-9]*.[0-9]* HEAD".to_owned(),
            format!("git log --no-merges --format=%H%x1f%s%x1f%b%x1e {range}"),
            merges.clone(),
            "git rev-list --no-merges p1..q1".to_owned(),
            "git rev-list --no-merges p2..q2".to_owned(),
            merges,
            "gh pr view 21 --json body --jq .body".to_owned(),
            "gh pr view 20 --json body --jq .body".to_owned(),
            "git rev-parse HEAD".to_owned(),
            "git rev-parse HEAD^2".to_owned(),
            "gh run list --workflow ci.yml --commit bbbb --status success --limit 1 --json databaseId --jq .[0].databaseId".to_owned(),
            format!("gh run download 4242 --name test-report --dir {out_text}"),
            format!(
                "gh release create v0.3.0 --target aaaa --title 0.3.0 --notes-file {out_text}/notes.md {out_text}/test-report.xml"
            ),
        ]
    );
    // #21's and #20's groups, in the order of Common Changelog; the attribution left out.
    assert_eq!(
        fs::read_to_string(out.0.join("notes.md")).unwrap(),
        "### Added\n\n- Choose a text's colors ([#12](https://github.com/o/r/pull/12))\n\n\
         ### Fixed\n\n- Refuse a NaN ([#11](https://github.com/o/r/pull/11))\n"
    );
    // No call commits, tags by Git or pushes: GitHub makes the tag with the release.
    assert!(!fake.calls.borrow().iter().any(|call| {
        call.starts_with("git commit")
            || call.starts_with("git tag")
            || call.starts_with("git push")
    }));
}

#[test]
fn f22_ac2_computes_the_version_from_the_last_tag_and_the_commits() {
    let version = |tag: &str, commits: &str| {
        let out = folder(&format!("version-{tag}"));
        let fake = Fake::new(on_main(tag, commits, [BODY_21, BODY_20]));

        published(&fake, &out)
    };
    let breaking = record('1', "refactor(geometry)!: one path writer", "");
    let fix = record('1', "fix(io): read the tables once", "");

    // REF-SEMVER, before 1.0.0 a break raises the minor version, from 1.0.0 the major one.
    assert_eq!(
        version("v0.2.0", &breaking),
        Ok(Published::Released("v0.3.0".to_owned()))
    );
    assert_eq!(
        version("v1.4.2", &breaking),
        Ok(Published::Released("v2.0.0".to_owned()))
    );
    assert_eq!(
        version("v1.4.2", &a_feature()),
        Ok(Published::Released("v1.5.0".to_owned()))
    );
    assert_eq!(
        version("v0.2.0", &fix),
        Ok(Published::Released("v0.2.1".to_owned()))
    );
}

#[test]
fn f22_ac3_does_nothing_without_a_commit_since_the_last_tag() {
    let out = folder("nothing");
    let fake = Fake::new(on_main("v0.3.0", "", [BODY_21, BODY_20]));

    assert_eq!(
        published(&fake, &out),
        Ok(Published::Nothing("v0.3.0".to_owned()))
    );
    assert!(
        !fake
            .calls
            .borrow()
            .iter()
            .any(|call| call.starts_with("gh ")),
        "{:?}",
        fake.calls.borrow()
    );
    assert!(!out.0.exists());
}

/// The notice of no change, copied by hand from `docs/tooling/releases.md` (How the draft is
/// made), not taken from the code it tests.
const NOTICE: &str =
    "_No change to the library nor the playground: documentation and tooling only._";

/// Commits of the tooling and the docs only, which no user of the library sees.
fn tooling_only() -> String {
    record('1', "feat(tooling): a status line", "") + &record('2', "docs(docs): log the run", "")
}

#[test]
fn f22_ac3_releases_tooling_only_as_a_patch_with_its_notice() {
    let out = folder("tooling");
    let evidence = format!("{NOTICE}\r\n\r\n---\r\n\r\nThe evidence.");
    let fake = Fake::new(on_main("v0.2.0", &tooling_only(), [&evidence, NOTICE]));

    // A feat of the tooling raises nothing: 0.2.0 → 0.2.1.
    assert_eq!(
        published(&fake, &out),
        Ok(Published::Released("v0.2.1".to_owned()))
    );
    assert_eq!(
        fs::read_to_string(out.0.join("notes.md")).unwrap(),
        format!("{NOTICE}\n")
    );
}

#[test]
fn f22_ac3_refuses_notes_that_disagree_with_the_commits() {
    let out = folder("disagree");
    let released = |commits: &str, bodies: [&str; 2]| {
        let fake = Fake::new(on_main("v0.2.0", commits, bodies));
        let result = published(&fake, &out);

        assert!(
            !fake
                .calls
                .borrow()
                .iter()
                .any(|call| call.starts_with("gh release")),
            "{:?}",
            fake.calls.borrow()
        );
        result
    };

    // Tooling only, but #21's body lists a change: the notes would say what no commit did.
    assert_eq!(
        released(&tooling_only(), [BODY_21, NOTICE]),
        Err(
            "pull request #21 lists changes, but no commit since v0.2.0 changes the library nor \
             the playground: write the notice of no change in its body (docs/tooling/releases.md)"
                .to_owned()
        )
    );
    // A feat of the library, but no body lists a change: a minor version with empty notes.
    let refused = "commits since v0.2.0 change the library or the playground, but no pull \
                   request's body lists a change: write the features' texts in the review's body \
                   (docs/tooling/releases.md)"
        .to_owned();
    assert_eq!(
        released(&a_feature(), [NOTICE, NOTICE]),
        Err(refused.clone())
    );
    assert_eq!(
        released(&a_feature(), ["_Read the guide._", NOTICE]),
        Err(refused)
    );
}

/// The merges of `main` after the tag with Dependabot's security update #22 the latest, then the
/// reviews #21 and #20.
const WITH_DEPENDABOT: &str = "m0\u{1f}p0 q0\u{1f}Merge pull request #22 from o/dependabot/cargo/regex-1.11.1\u{1e}\n\
     m1\u{1f}p1 q1\u{1f}Merge pull request #21 from o/develop\u{1e}\n\
     m2\u{1f}p2 q2\u{1f}Merge pull request #20 from o/develop\u{1e}\n";

/// Dependabot's body, which `release check` refuses: a paragraph, then details.
const DEPENDABOT_BODY: &str = "Bumps [regex](https://github.com/rust-lang/regex) from 1.11.0 to \
                               1.11.1.\n<details>\n<summary>Commits</summary>\n</details>";

/// Dependabot's commit, outside Conventional Commits.
fn dependabot() -> String {
    record(
        '3',
        "Bump regex from 1.11.0 to 1.11.1",
        "Signed-off-by: dependabot[bot] <support@github.com>\n",
    )
}

#[test]
fn f22_ac2_leaves_out_the_body_of_a_dependabot_pull_request() {
    let with = |commits: &str, merges: &str| {
        let mut answers = on_main("v0.2.0", commits, [BODY_21, BODY_20]);

        answers.insert(0, answer("git log --first-parent", Ok(merges.to_owned())));
        answers.insert(0, answer("gh pr view 22", Ok(DEPENDABOT_BODY.to_owned())));
        Fake::new(answers)
    };
    let out = folder("dependabot");
    let fake = with(&(dependabot() + &a_feature()), WITH_DEPENDABOT);

    // Its commit is no feat: the review's feat alone gives 0.3.0, its notes #21's and #20's.
    assert_eq!(
        published(&fake, &out),
        Ok(Published::Released("v0.3.0".to_owned()))
    );
    assert!(
        !fake
            .calls
            .borrow()
            .iter()
            .any(|call| call.starts_with("gh pr view 22")),
        "{:?}",
        fake.calls.borrow()
    );
    assert_eq!(
        fs::read_to_string(out.0.join("notes.md")).unwrap(),
        "### Added\n\n- Choose a text's colors ([#12](https://github.com/o/r/pull/12))\n\n\
         ### Fixed\n\n- Refuse a NaN ([#11](https://github.com/o/r/pull/11))\n"
    );

    // Dependabot's alone since the tag: a patch, its notes the notice of no change.
    let alone = "m0\u{1f}p0 q0\u{1f}Merge pull request #22 from o/dependabot/npm_and_yarn/vite-7.1.2\u{1e}\n";
    let only = with(&dependabot(), alone);
    assert_eq!(
        published(&only, &out),
        Ok(Published::Released("v0.2.1".to_owned()))
    );
    assert_eq!(
        fs::read_to_string(out.0.join("notes.md")).unwrap(),
        format!("{NOTICE}\n")
    );
}

#[test]
fn refuses_a_body_that_breaks_a_rule_and_two_notices() {
    let out = folder("bodies");
    let drafted = BODY_20.replace("- Refuse", "- [draft] Refuse");
    let fake = Fake::new(on_main("v0.2.0", &a_feature(), [BODY_21, &drafted]));

    assert_eq!(
        published(&fake, &out),
        Err(
            "pull request #20:3: a drafted line is rewritten in the imperative mood, its \
             `[draft]` mark removed (release check)"
                .to_owned()
        )
    );
    assert!(
        !fake
            .calls
            .borrow()
            .iter()
            .any(|call| call.starts_with("gh release"))
    );

    let empty = Fake::new(on_main("v0.2.0", &a_feature(), [BODY_21, ""]));
    assert!(
        published(&empty, &out)
            .unwrap_err()
            .starts_with("pull request #20:1: a release text lists its changes by group"),
    );

    let guide = format!("_Read the guide._\n\n{BODY_20}");
    let other = BODY_21.replace("### Added", "_Another._\r\n\r\n### Added");
    let notices = Fake::new(on_main("v0.2.0", &a_feature(), [&other, &guide]));
    assert_eq!(
        published(&notices, &out),
        Err(
            "pull requests #21 and #20 each hold a notice: a release holds one at most, so \
             write it in one body"
                .to_owned()
        )
    );
}

#[test]
fn refuses_no_tag_a_head_not_merged_a_pull_request_without_ci_and_a_failing_command() {
    let out = folder("refused");
    let refused = |first: (String, Answer)| {
        let mut answers = on_main("v0.2.0", &a_feature(), [BODY_21, BODY_20]);
        answers.insert(0, first);

        published(&Fake::new(answers), &out)
    };

    assert_eq!(
        refused(answer(
            "git describe",
            Err("fatal: No names found".to_owned())
        )),
        Err("no release tag vx.y.z behind HEAD: fatal: No names found".to_owned())
    );
    assert_eq!(
        refused(answer(
            "git rev-parse HEAD^2",
            Err("unknown revision".to_owned())
        )),
        Err("aaaa is not the merge commit of a pull request".to_owned())
    );
    for no_run in ["null\n", "\n"] {
        assert_eq!(
            refused(answer("gh run list", Ok(no_run.to_owned()))),
            Err("no successful CI run of bbbb, the pull request's head: no test report".to_owned())
        );
    }
    assert_eq!(
        refused(answer("gh pr view 20", Err("HTTP 404".to_owned()))),
        Err("HTTP 404".to_owned())
    );
    assert_eq!(
        refused(answer("gh release create", Err("HTTP 403".to_owned()))),
        Err("HTTP 403".to_owned())
    );
}

/// A fake `gh`: logs each call, gives a pull request's body from the folder `$FAKE_GH_BODIES`,
/// run 4242, writes the report where `--dir` says, and logs the notes of `--notes-file`.
#[cfg(unix)]
const FAKE_GH: &str = "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$FAKE_GH_LOG\"\ncase \"$1 $2\" in\n\
     \"pr view\") cat \"$FAKE_GH_BODIES/$3.md\";;\n\
     \"run list\") echo 4242;;\n\
     \"run download\") while [ $# -gt 0 ]; do [ \"$1\" = --dir ] && touch \"$2/test-report.xml\"; shift; done;;\n\
     \"release create\") while [ $# -gt 0 ]; do [ \"$1\" = --notes-file ] && cat \"$2\" >> \"$FAKE_GH_LOG\"; shift; done;;\n\
     esac\n";

/// Runs Git in a folder, its inherited `GIT_*` variables removed; its output.
#[cfg(unix)]
fn git(root: &std::path::Path, args: &[&str]) -> String {
    let output = test_support::command("git")
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["-c", "commit.gpgsign=false", "-c", "tag.gpgsign=false"])
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

/// A repository whose `main`, tagged `v0.2.0`, merges `develop` and its feat as pull request #20:
/// the pull request's head and the merge commit.
#[cfg(unix)]
fn merged_review(root: &std::path::Path) -> (String, String) {
    git(root, &["init", "-q", "-b", "main"]);
    git(
        root,
        &["commit", "-q", "--allow-empty", "-m", "chore: start"],
    );
    git(root, &["tag", "v0.2.0"]);
    git(root, &["switch", "-q", "-c", "develop"]);
    let message = "feat(model): give a text its colors";
    git(root, &["commit", "-q", "--allow-empty", "-m", message]);
    let pull_request_head = git(root, &["rev-parse", "HEAD"]);
    git(root, &["switch", "-q", "main"]);
    let merge = "Merge pull request #20 from o/develop";
    git(root, &["merge", "-q", "--no-ff", "-m", merge, "develop"]);

    (pull_request_head, git(root, &["rev-parse", "HEAD"]))
}

/// The binary on a repository whose `main` merged a review, GitHub played by a fake `gh` first on
/// the `PATH`, a shell script: Unix only, for its execute bit.
#[cfg(unix)]
#[test]
fn f22_ac2_publishes_from_the_binary_then_does_nothing_once_tagged() {
    use std::os::unix::fs::PermissionsExt;

    let folder = folder("binary");
    let root = &folder.0;
    fs::create_dir_all(root).unwrap();
    let (pull_request_head, merge) = merged_review(root);
    let bin = root.join("bin");
    let bodies = bin.join("bodies");
    let log = bin.join("gh.log");
    fs::create_dir_all(&bodies).unwrap();
    fs::write(bin.join("gh"), FAKE_GH).unwrap();
    fs::set_permissions(bin.join("gh"), fs::Permissions::from_mode(0o755)).unwrap();
    fs::write(bodies.join("20.md"), BODY_21).unwrap();
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let publish = || {
        test_support::command(env!("CARGO_BIN_EXE_release"))
            .arg("publish")
            .env("GITHUB_REF", "refs/heads/main")
            .env("PATH", &path)
            .env("FAKE_GH_LOG", &log)
            .env("FAKE_GH_BODIES", &bodies)
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
    assert!(said.contains("pr view 20 --json body"), "{said}");
    let listed = format!("run list --workflow ci.yml --commit {pull_request_head} ");
    assert!(said.contains(&listed), "{said}");
    let created = format!("release create v0.3.0 --target {merge} --title 0.3.0 ");
    assert!(said.contains(&created), "{said}");
    assert!(said.contains("- Choose a text's colors ([#12]"), "{said}");
    assert!(!said.contains("Claude Code"), "{said}");
    // No commit, no tag made here: the head is still the merge, its tag GitHub's.
    assert_eq!(git(root, &["rev-parse", "HEAD"]), merge);
    assert_eq!(git(root, &["tag", "--list"]), "v0.2.0");

    // The tag GitHub made, fetched by the next run's checkout: no commit since, nothing done.
    git(root, &["tag", "v0.3.0"]);
    let again = publish();
    assert!(again.status.success());
    assert_eq!(
        String::from_utf8_lossy(&again.stdout),
        "no commit since v0.3.0: nothing to release\n"
    );
    assert_eq!(fs::read_to_string(&log).unwrap(), said);

    git(root, &["tag", "-d", "v0.3.0"]);
    fs::write(bodies.join("20.md"), "A paragraph.").unwrap();
    let broken = publish();
    assert_eq!(broken.status.code(), Some(1));
    let refused = String::from_utf8_lossy(&broken.stderr).into_owned();
    assert!(
        refused.starts_with("release publish: pull request #20:1: a notice is one sentence"),
        "{refused}"
    );
}

/// The binary pressed on a tag named `main`: exit 1 naming the ref, no Git nor GitHub call, `gh`
/// played by the fake whose log would hold any call.
#[cfg(unix)]
#[test]
fn f28_ac2_exits_1_from_the_binary_on_a_branch_other_than_main() {
    use std::os::unix::fs::PermissionsExt;

    let folder = folder("binary-branch");
    let root = &folder.0;
    let bin = root.join("bin");
    let log = bin.join("gh.log");
    fs::create_dir_all(&bin).unwrap();
    fs::write(bin.join("gh"), FAKE_GH).unwrap();
    fs::set_permissions(bin.join("gh"), fs::Permissions::from_mode(0o755)).unwrap();
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );

    let refused = test_support::command(env!("CARGO_BIN_EXE_release"))
        .arg("publish")
        .env("GITHUB_REF", "refs/tags/main")
        .env("GITHUB_REF_NAME", "main")
        .env("PATH", &path)
        .env("FAKE_GH_LOG", &log)
        .current_dir(root)
        .output()
        .unwrap();

    // A tag named main: its short name is main, its ref not the branch's.
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&refused.stderr),
        "release publish: the run is on the ref \"refs/tags/main\", not \"refs/heads/main\": a \
         release is made from the branch main only, so press the button on main (GITHUB_REF)\n"
    );
    assert_eq!(String::from_utf8_lossy(&refused.stdout), "");
    assert!(!log.exists());
}
