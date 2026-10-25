//! `story-finish` run in repositories of its own (EN-103, EN-154): a bare `origin`, a main
//! checkout on the feature branch, each story in a linked worktree, as in a run, and records of
//! the test's own folder holding the feature's stories. GitHub's command line and Cargo are stubs
//! first on `PATH`: `gh pr create` merges by pushing the story onto its base, `cargo` records its
//! calls. Every command starts through `test_support::command`, without the inherited `GIT_*`
//! variables, its Git configuration a test file of its own; `AGENT_RECORDS_ROOT` names the test's
//! folder, never the real records.

#![cfg(unix)]

use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use knowledge::sql::read;
use knowledge::store::open_store;
use test_support::command;

const BASE: &str = "feature/f99-topic";
const FIRST: &str = "chore/en-901-first";
const SECOND: &str = "chore/en-902-second";

/// GitHub's command line: it writes where it runs and what for, and answers as `GH_*` say.
const GH: &str = r#"#!/bin/sh
echo "$PWD $*" >> "$STUB_LOG"
case "$1 $2" in
  "pr view")
    case "$3" in
      http*) echo "${GH_STATE:-MERGED}" ;;
      *) [ -n "$GH_EXISTING" ] || exit 1; echo "$GH_EXISTING https://github.com/owner/name/pull/1" ;;
    esac ;;
  "pr create")
    # GitHub's merge of the story into its base, `--base` being the third and fourth words.
    [ "${GH_STATE:-MERGED}" = MERGED ] && git push -q origin "HEAD:refs/heads/$4"
    echo https://github.com/owner/name/pull/1 ;;
  "repo view") echo owner/name ;;
  "api -X") [ -z "$GH_API_FAILS" ] || { echo "HTTP 422: Reference does not exist" >&2; exit 1; } ;;
esac
"#;

/// Cargo: it records its calls.
const CARGO: &str = r#"#!/bin/sh
echo "$PWD $*" >> "$STUB_LOG"
"#;

/// A test's folder: the repositories, the stubs, the Git configuration.
struct Lab {
    root: PathBuf,
}

impl Lab {
    fn new(name: &str) -> Lab {
        let root = env::temp_dir().join(format!("story-finish-{}-{name}", std::process::id()));

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("bin")).expect("folder");

        let root = fs::canonicalize(&root).expect("canonical");

        for (name, text) in [("gh", GH), ("cargo", CARGO)] {
            let path = root.join("bin").join(name);

            fs::write(&path, text).expect("stub");
            fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).expect("mode");
        }
        fs::write(
            root.join("gitconfig"),
            "[user]\n\tname = Test\n\temail = test@example.com\n[commit]\n\tgpgsign = false\n[init]\n\tdefaultBranch = main\n",
        )
        .expect("config");
        Lab { root }
    }

    /// A command of the test: the stubs first on `PATH`, the test's Git configuration.
    fn command(&self, program: &str, cwd: &Path) -> Command {
        let mut command = command(program);
        let path = format!(
            "{}:{}",
            self.root.join("bin").display(),
            env::var("PATH").unwrap_or_default()
        );

        command
            .current_dir(cwd)
            .env("PATH", path)
            .env("HOME", &self.root)
            .env("GIT_CONFIG_GLOBAL", self.root.join("gitconfig"))
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("STUB_LOG", self.root.join("stub.log"))
            .env_remove("GH_STATE")
            .env_remove("GH_EXISTING")
            .env_remove("GH_API_FAILS")
            .env("AGENT_RECORDS_ROOT", self.root.join("records"))
            .env_remove("AGENT_KNOWLEDGE_SNAPSHOT");
        command
    }

    fn git(&self, cwd: &Path, words: &[&str]) -> String {
        let output = self.command("git", cwd).args(words).output().expect("git");

        assert!(
            output.status.success(),
            "git {}: {}",
            words.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_owned()
    }

    fn main(&self) -> PathBuf {
        self.root.join("main")
    }

    fn worktree(&self, id: &str) -> PathBuf {
        self.root.join(id)
    }

    /// The calls made to the stubs since the last reading, which empties them.
    fn calls(&self) -> Vec<String> {
        let path = self.root.join("stub.log");
        let text = fs::read_to_string(&path).unwrap_or_default();

        let _ = fs::remove_file(&path);
        text.lines().map(str::to_owned).collect()
    }

    /// The records of the test's repositories, named after `origin.git`.
    fn records(&self) -> PathBuf {
        self.root.join("records/origin/records.sqlite")
    }

    /// A query's rows, each row's cells joined by `|`.
    fn rows(&self, query: &str) -> Vec<String> {
        let database = open_store(&self.records()).expect("records");

        read(&database, query)
            .expect("rows")
            .lines()
            .skip(1)
            .map(|line| line.replace('\t', "|"))
            .collect()
    }

    /// The remote, the main checkout on the feature branch, and the records: a feature of two
    /// stories, both `ready`, and a draft row EN-901's branch wrote.
    fn backlog(&self) {
        let origin = self.root.join("origin.git");
        let main = self.main();

        self.git(
            &self.root,
            &["init", "-q", "--bare", origin.to_str().expect("path")],
        );
        self.git(
            &self.root,
            &["clone", "-q", origin.to_str().expect("path"), "main"],
        );
        fs::write(main.join("README.md"), "# A project\n").expect("readme");
        self.git(&main, &["add", "."]);
        self.git(&main, &["commit", "-q", "-m", "start"]);
        self.git(&main, &["push", "-q", "-u", "origin", "HEAD:main"]);
        self.git(&main, &["switch", "-q", "-c", BASE]);
        self.git(&main, &["push", "-q", "-u", "origin", BASE]);
        open_store(&self.records())
            .expect("records")
            .execute_batch(&format!(
                "INSERT INTO features (id, epic, title, status, state) VALUES ('F99', 'E99', 'A feature', 'in-progress', 'merged');
                INSERT INTO stories (id, epic, feature, kind, title, status, topic, branch, state) VALUES
                  ('EN-901', 'E99', 'F99', 'EN', 'First', 'ready', 'first', 'develop', 'merged'),
                  ('EN-902', 'E99', 'F99', 'EN', 'Second', 'ready', 'second', 'develop', 'merged'),
                  ('EN-903', 'E99', 'F99', 'EN', 'Written by EN-901', 'draft', 'third', '{FIRST}', 'draft');"
            ))
            .expect("records written");
    }

    /// A story's worktree with one task pushed: no `done` commit since its status is in the
    /// records (EN-154).
    fn story(&self, branch: &str, id: &str) -> PathBuf {
        let folder = self.worktree(id);
        let path = folder.to_str().expect("path");

        self.git(
            &self.main(),
            &["worktree", "add", "-q", "-b", branch, path, BASE],
        );
        fs::write(folder.join(format!("{id}.txt")), "work\n").expect("work");
        self.git(&folder, &["add", "."]);
        self.git(
            &folder,
            &["commit", "-q", "-m", &format!("feat: work of {id}")],
        );
        self.git(&folder, &["push", "-q", "-u", "origin", branch]);
        folder
    }

    /// A story attached to `develop`, a `REV` or an epic's `SP`: the main checkout on `develop`,
    /// as in a run, the story's worktree made from it with one task pushed.
    fn develop_story(&self, branch: &str, id: &str) -> PathBuf {
        let folder = self.worktree(id);
        let main = self.main();

        self.git(&main, &["switch", "-q", "-c", "develop", "main"]);
        self.git(&main, &["push", "-q", "-u", "origin", "develop"]);
        self.git(
            &main,
            &[
                "worktree",
                "add",
                "-q",
                "-b",
                branch,
                folder.to_str().expect("path"),
                "develop",
            ],
        );
        fs::write(folder.join(format!("{id}.txt")), "work\n").expect("work");
        self.git(&folder, &["add", "."]);
        self.git(
            &folder,
            &["commit", "-q", "-m", &format!("docs: work of {id}")],
        );
        self.git(&folder, &["push", "-q", "-u", "origin", branch]);
        folder
    }

    /// `story-finish` run in a folder, with the stubs' answers given.
    fn finish(&self, folder: &Path, answers: &[(&str, &str)]) -> Output {
        self.finish_into(folder, BASE, answers)
    }

    /// `story-finish` into a base, run in a folder, with the stubs' answers given.
    fn finish_into(&self, folder: &Path, base: &str, answers: &[(&str, &str)]) -> Output {
        let body = self.root.join("body.md");

        fs::write(&body, "A body.\n").expect("body");
        self.command(env!("CARGO_BIN_EXE_story-finish"), folder)
            .args([base, "A title", body.to_str().expect("path")])
            .envs(answers.iter().copied())
            .output()
            .expect("story-finish")
    }
}

impl Drop for Lab {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn text(output: &Output) -> (String, String) {
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn f21_ac2_two_stories_of_one_feature_finish_one_after_the_other_without_a_conflict() {
    let lab = Lab::new("two");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");
    let second = lab.story(SECOND, "en-902");
    let finished = lab.finish(&first, &[]);
    let (out, err) = text(&finished);

    assert!(finished.status.success(), "{out}\n{err}");
    // EN-901's draft row EN-903 merged, EN-901 itself done.
    assert_eq!(
        out,
        format!(
            "https://github.com/owner/name/pull/1\nthe records: 1 row(s) of {FIRST} marked merged, EN-901 done\nmerged; {BASE} up to date, {FIRST} deleted here and on GitHub\n"
        )
    );
    assert!(!first.exists());
    lab.calls();

    let finished = lab.finish(&second, &[]);
    let (out, err) = text(&finished);
    let main = lab.main();

    assert!(finished.status.success(), "{out}\n{err}");
    assert!(
        out.contains(&format!("{SECOND} rebased on origin/{BASE} and pushed\n")),
        "{out}"
    );
    assert!(out.ends_with(&format!(
        "merged; {BASE} up to date, {SECOND} deleted here and on GitHub\n"
    )));
    // The base holds both stories' work, no status commit between them.
    assert_eq!(
        lab.git(
            &main,
            &["log", "--format=%s", "-3", &format!("origin/{BASE}")]
        ),
        "feat: work of en-902\nfeat: work of en-901\nstart"
    );
    assert_eq!(
        lab.rows("SELECT id, status, state FROM stories ORDER BY id"),
        [
            "EN-901|done|merged",
            "EN-902|done|merged",
            "EN-903|draft|merged"
        ]
    );
    assert_eq!(
        lab.git(&main, &["rev-parse", "HEAD"]),
        lab.git(&main, &["rev-parse", &format!("origin/{BASE}")])
    );
    assert!(!second.exists());
    assert_eq!(lab.git(&main, &["status", "--porcelain"]), "");
}

#[test]
fn f21_ac2_pushes_with_lease_a_branch_rebased_by_hand_before_its_pull_request() {
    let lab = Lab::new("lease");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");

    // The last commit made again by hand, not pushed: GitHub's branch holds the old one.
    lab.git(
        &first,
        &[
            "commit",
            "-q",
            "--amend",
            "-m",
            "feat: work of en-901, again",
        ],
    );

    let head = lab.git(&first, &["rev-parse", "HEAD"]);
    let finished = lab.finish(&first, &[("GH_STATE", "CLOSED")]);
    let (out, err) = text(&finished);
    let calls = lab.calls();

    assert_eq!(finished.status.code(), Some(1), "{out}\n{err}");
    assert_eq!(
        out,
        format!(
            "{FIRST} pushed with lease: its head differed from origin/{FIRST}\nhttps://github.com/owner/name/pull/1\n"
        )
    );
    assert_eq!(
        lab.git(&first, &["rev-parse", &format!("origin/{FIRST}")]),
        head
    );
    // Pushed before the pull request was opened.
    assert!(calls.iter().any(|call| call.contains(" pr create --base ")));
    // Not merged: the worktree, the branch and the records kept.
    assert!(
        err.contains(&format!("CLOSED after 3 min, {FIRST} kept")),
        "{err}"
    );
    assert!(first.exists());
    assert_eq!(
        lab.rows("SELECT status, state FROM stories WHERE id IN ('EN-901', 'EN-903') ORDER BY id"),
        ["ready|merged", "draft|draft"]
    );
}

#[test]
fn leaves_a_branch_already_on_its_base_and_its_remote_as_it_is() {
    let lab = Lab::new("same");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");
    let head = lab.git(&first, &["rev-parse", "HEAD"]);
    let finished = lab.finish(&first, &[("GH_STATE", "CLOSED")]);
    let (out, _) = text(&finished);

    assert_eq!(out, "https://github.com/owner/name/pull/1\n");
    assert_eq!(lab.git(&first, &["rev-parse", "HEAD"]), head);
}

#[test]
fn f18_ac7_aborts_a_rebase_that_meets_a_conflict_outside_the_status_rows() {
    let lab = Lab::new("conflict");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");
    let main = lab.main();
    let head = lab.git(&first, &["rev-parse", "HEAD"]);

    // Another story wrote the same file otherwise, merged into the base.
    fs::write(
        main.join("en-901.txt"),
        "the same file, written otherwise\n",
    )
    .expect("file");
    lab.git(&main, &["add", "."]);
    lab.git(&main, &["commit", "-q", "-m", "other"]);
    lab.git(&main, &["push", "-q"]);

    let finished = lab.finish(&first, &[]);
    let (_, err) = text(&finished);

    assert_eq!(finished.status.code(), Some(1));
    assert!(
        err.contains(&format!(
            "conflict rebasing on origin/{BASE}: rebase by hand, then run again"
        )),
        "{err}"
    );
    assert_eq!(lab.git(&first, &["rev-parse", "HEAD"]), head);
    assert_eq!(lab.git(&first, &["status", "--porcelain"]), "");
    assert_eq!(lab.git(&first, &["branch", "--show-current"]), FIRST);
    assert!(!lab.calls().iter().any(|call| call.contains("pr create")));
}

#[test]
fn f23_ac1_pushes_no_commit_after_the_merge_the_status_in_the_records() {
    let lab = Lab::new("rows");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");
    let main = lab.main();
    let last = lab.git(&first, &["rev-parse", "HEAD"]);

    // The main thread's own work, staged in the main checkout.
    fs::write(main.join("draft.md"), "a draft\n").expect("draft");
    lab.git(&main, &["add", "draft.md"]);

    let finished = lab.finish(&first, &[]);
    let (out, err) = text(&finished);
    let calls = lab.calls();

    assert!(finished.status.success(), "{out}\n{err}");
    // [F23.AC1] the base's head is the story's last commit: no commit after it, its status
    // written in the records instead.
    assert_eq!(
        lab.git(&main, &["rev-parse", &format!("origin/{BASE}")]),
        last
    );
    assert_eq!(
        lab.rows("SELECT status FROM stories WHERE id = 'EN-901'"),
        ["done"]
    );
    assert_eq!(lab.git(&main, &["status", "--porcelain"]), "A  draft.md");
    // The base's release binaries rebuilt where it is checked out (ADR-0078), the branch deleted
    // from there.
    let place = main.display().to_string();

    assert!(calls.contains(&format!(
        "{place} build --release --workspace --locked --quiet"
    )));
    assert!(calls.contains(&format!(
        "{place} api -X DELETE repos/owner/name/git/refs/heads/{FIRST}"
    )));
    assert_eq!(lab.git(&main, &["branch", "--list", FIRST]), "");
    assert_eq!(lab.git(&main, &["branch", "--show-current"]), BASE);
}

#[test]
fn opens_no_pull_request_into_develop_while_the_change_cites_an_id_the_records_lack() {
    let lab = Lab::new("ids");
    let branch = "docs/rev-906-review";

    lab.backlog();

    let review = lab.develop_story(branch, "rev-906");

    // A page citing EN-901, in the records, and EN-999, nowhere; a test citing EN-998 is not read.
    fs::create_dir_all(review.join("docs")).expect("docs");
    fs::write(review.join("docs/page.md"), "EN-901 then\nEN-999.\n").expect("page");
    fs::write(review.join("a.test.ts"), "// EN-998\n").expect("test");
    lab.git(&review, &["add", "."]);
    lab.git(&review, &["commit", "-q", "-m", "docs: cite"]);
    lab.git(&review, &["push", "-q"]);

    let finished = lab.finish_into(&review, "develop", &[]);
    let (out, err) = text(&finished);

    assert_eq!(finished.status.code(), Some(1), "{out}\n{err}");
    assert!(err.ends_with(":\ndocs/page.md:2: EN-999\n"), "{err}");
    assert!(!lab.calls().iter().any(|call| call.contains("pr create")));
    assert!(review.exists());
}

#[test]
fn f18_ac7_goes_straight_to_the_clean_up_of_a_story_already_merged() {
    let lab = Lab::new("merged");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");

    // GitHub merged it in a run that stopped before the clean-up.
    lab.git(&first, &["push", "-q", "origin", &format!("HEAD:{BASE}")]);

    let finished = lab.finish(&first, &[("GH_EXISTING", "MERGED")]);
    let (out, err) = text(&finished);
    let calls = lab.calls();

    assert!(finished.status.success(), "{out}\n{err}");
    assert_eq!(
        out,
        format!(
            "https://github.com/owner/name/pull/1\nthe records: 1 row(s) of {FIRST} marked merged, EN-901 done\nmerged; {BASE} up to date, {FIRST} deleted here and on GitHub\n"
        )
    );
    assert!(!calls.iter().any(|call| call.contains("pr create")));
    assert!(!first.exists());
}

#[test]
fn f18_ac7_prints_the_steps_left_when_the_clean_up_stops() {
    let lab = Lab::new("stopped");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");
    let finished = lab.finish(&first, &[("GH_API_FAILS", "1")]);
    let (_, err) = text(&finished);
    let main = lab.main().display().to_string();

    assert_eq!(finished.status.code(), Some(1));
    assert!(
        err.contains(&format!(
            "the clean-up stopped: Error: Command failed: gh api -X DELETE repos/owner/name/git/refs/heads/{FIRST}\nHTTP 422: Reference does not exist\n"
        )),
        "{err}"
    );
    assert!(err.contains("Run again from the worktree while it exists, or finish by hand, skipping the steps already done:\n"));
    assert!(err.trim_end().ends_with(&format!(
        "cd \"{main}\" && gh api -X DELETE repos/owner/name/git/refs/heads/{FIRST}"
    )));
}

#[test]
fn f18_ac7_deletes_the_branch_from_the_main_checkout_when_the_base_is_checked_out_nowhere() {
    let lab = Lab::new("nowhere");

    lab.backlog();

    let first = lab.story(FIRST, "en-901");
    let main = lab.main();

    // As in a run: the main checkout on another branch, the base checked out nowhere (PR #272).
    lab.git(&main, &["switch", "-q", "main"]);

    let finished = lab.finish(&first, &[]);
    let (out, err) = text(&finished);
    let calls = lab.calls();

    assert!(finished.status.success(), "{out}\n{err}");
    assert!(!first.exists());
    assert_eq!(lab.git(&main, &["branch", "--show-current"]), "main");
    assert_eq!(lab.git(&main, &["branch", "--list", FIRST]), "");
    assert_eq!(
        lab.git(&main, &["rev-parse", BASE]),
        lab.git(&main, &["rev-parse", &format!("origin/{BASE}")])
    );
    assert!(calls.contains(&format!(
        "{} api -X DELETE repos/owner/name/git/refs/heads/{FIRST}",
        main.display()
    )));
}

#[test]
fn f23_ac2_labels_a_review_into_develop_and_awaits_its_merge() {
    // [F23.AC2] A `REV` from `develop`: its pull request labelled `autonomous`, GitHub's merge
    // awaited, then the clean-up, as for any story.
    let lab = Lab::new("review");
    let branch = "docs/rev-906-review";

    lab.backlog();

    let review = lab.develop_story(branch, "rev-906");
    let finished = lab.finish_into(&review, "develop", &[]);
    let (out, err) = text(&finished);
    let calls = lab.calls();

    assert!(finished.status.success(), "{out}\n{err}");
    assert_eq!(
        out,
        format!(
            "https://github.com/owner/name/pull/1\nthe records: 0 row(s) of {branch} marked merged\nmerged; develop up to date, {branch} deleted here and on GitHub\n"
        )
    );
    assert!(
        calls
            .iter()
            .any(|call| call.contains(" pr create --base develop --label autonomous ")),
        "{calls:?}"
    );
    assert!(
        calls
            .iter()
            .any(|call| call.contains(" pr view https://github.com/owner/name/pull/1 ")),
        "{calls:?}"
    );
    assert!(!review.exists());
}

#[test]
fn f23_ac2_labels_an_epics_spike_into_develop_and_awaits_its_merge() {
    // [F23.AC2] An epic's refinement `SP` from `develop`, as a `REV` (REV-009): its pull request
    // labelled `autonomous`, GitHub's merge awaited, then the clean-up.
    let lab = Lab::new("spike");
    let branch = "docs/sp-930-refinement-of-e99";

    lab.backlog();

    let spike = lab.develop_story(branch, "sp-930");
    let finished = lab.finish_into(&spike, "develop", &[]);
    let (out, err) = text(&finished);
    let calls = lab.calls();

    assert!(finished.status.success(), "{out}\n{err}");
    assert_eq!(
        out,
        format!(
            "https://github.com/owner/name/pull/1\nthe records: 0 row(s) of {branch} marked merged\nmerged; develop up to date, {branch} deleted here and on GitHub\n"
        )
    );
    assert!(
        calls
            .iter()
            .any(|call| call.contains(" pr create --base develop --label autonomous ")),
        "{calls:?}"
    );
    assert!(
        calls
            .iter()
            .any(|call| call.contains(" pr view https://github.com/owner/name/pull/1 ")),
        "{calls:?}"
    );
    assert!(!spike.exists());
}

#[test]
fn f23_ac2_leaves_to_the_product_owners_merge_a_story_the_auto_merge_refuses() {
    // [F23.AC2] A story `feature-automerge.yml` refuses into `develop`, an `EN` straight from
    // it: opened without the label and never awaited; run again, it waits no more, and cleans up
    // once the Product Owner merged it.
    let lab = Lab::new("refused");
    let branch = "chore/en-930-straight-into-develop";
    let waits = format!(
        "{branch} waits for the Product Owner's merge into develop: `feature-automerge.yml` does \
         not take it (EN-118); run again once merged, to clean up\n"
    );

    lab.backlog();

    let story = lab.develop_story(branch, "en-930");
    let opened = lab.finish_into(&story, "develop", &[("GH_STATE", "OPEN")]);
    let (out, err) = text(&opened);
    let calls = lab.calls();

    assert!(opened.status.success(), "{out}\n{err}");
    assert_eq!(
        out,
        format!("https://github.com/owner/name/pull/1\n{waits}")
    );
    assert!(
        calls.iter().any(|call| call.ends_with(&format!(
            " pr create --base develop --title A title --body-file {}",
            lab.root.join("body.md").display()
        ))),
        "{calls:?}"
    );
    assert!(
        !calls.iter().any(|call| call.contains(" pr view https://")),
        "{calls:?}"
    );
    assert!(story.exists());

    // Run again while it is open: nothing opened, nothing awaited.
    let again = lab.finish_into(
        &story,
        "develop",
        &[("GH_STATE", "OPEN"), ("GH_EXISTING", "OPEN")],
    );
    let (out, err) = text(&again);
    let calls = lab.calls();

    assert!(again.status.success(), "{out}\n{err}");
    assert_eq!(
        out,
        format!("https://github.com/owner/name/pull/1\n{waits}")
    );
    assert!(
        !calls
            .iter()
            .any(|call| call.contains(" pr create") || call.contains(" pr view https://")),
        "{calls:?}"
    );

    // Merged by the Product Owner: the clean-up.
    lab.git(&story, &["push", "-q", "origin", "HEAD:develop"]);

    let merged = lab.finish_into(&story, "develop", &[("GH_EXISTING", "MERGED")]);
    let (out, err) = text(&merged);

    assert!(merged.status.success(), "{out}\n{err}");
    assert!(
        out.ends_with(&format!(
            "merged; develop up to date, {branch} deleted here and on GitHub\n"
        )),
        "{out}"
    );
    assert!(!story.exists());
}

#[test]
fn stops_without_its_three_arguments() {
    let output = command(env!("CARGO_BIN_EXE_story-finish"))
        .args(["feature/f99-topic", "a title"])
        .output()
        .expect("story-finish");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "usage: npm run story:finish -- <base> <title> <body file>\n"
    );
}
