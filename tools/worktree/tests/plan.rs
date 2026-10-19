//! Tests of `worktree-new` (EN-052): the plan from a story's file name, then the tool run in a
//! repository of its own. `[F18.AC8]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use worktree::{
    Request, base_branch, branch_refusal, branch_type, plan, story_id, story_of_branch,
    story_of_file, story_on_branches,
};

const FILE: &str = "E17-F18-US-061-font-list-held-while-the-default-font-is-read.md";

fn files() -> Vec<String> {
    vec![
        "README.md".to_owned(),
        "E17-F18-EN-052-worktrees-named-after-their-story.md".to_owned(),
        FILE.to_owned(),
        "E15-REV-003-review-e15.md".to_owned(),
    ]
}

fn remotes() -> Vec<String> {
    vec![
        "origin/develop".to_owned(),
        "origin/feature/f17-text-fixes".to_owned(),
        "origin/feature/f18-safe-and-cheap-parallel-run".to_owned(),
    ]
}

#[test]
fn reads_a_story_id_in_either_case() {
    assert_eq!(story_id("us-061").as_deref(), Some("US-061"));
    assert_eq!(story_id(" EN-052 ").as_deref(), Some("EN-052"));
    assert_eq!(story_id("US-61"), None);
    assert_eq!(story_id("US061"), None);
    assert_eq!(story_id("-061"), None);
    assert_eq!(story_id("U1-061"), None);
}

#[test]
fn reads_the_story_of_a_branch() {
    assert_eq!(
        story_of_branch("chore/en-100-git-sequences-as-rust-commands").as_deref(),
        Some("EN-100")
    );
    assert_eq!(
        story_of_branch("docs/rev-003-review-e15").as_deref(),
        Some("REV-003")
    );
    assert_eq!(story_of_branch("chore/en-100").as_deref(), Some("EN-100"));
    assert_eq!(story_of_branch("chore/tidy-up"), None);
    assert_eq!(story_of_branch("develop"), None);
}

#[test]
fn reads_the_feature_and_topic_from_a_file_name() {
    let story = story_of_file("US-061", FILE).expect("the story's file");

    assert_eq!(story.kind, "US");
    assert_eq!(story.feature.as_deref(), Some("F18"));
    assert_eq!(story.topic, "font-list-held-while-the-default-font-is-read");

    let review = story_of_file("REV-003", "E15-REV-003-review-e15.md").expect("an epic's story");

    assert_eq!(review.feature, None);
    assert_eq!(review.topic, "review-e15");
    // Another story, a file without topic, a file that is not Markdown.
    assert_eq!(story_of_file("US-062", FILE), None);
    assert_eq!(story_of_file("US-061", "E17-F18-US-061.md"), None);
    assert_eq!(story_of_file("US-061", "E17-F18-US-061-topic.txt"), None);
}

#[test]
fn types_a_branch_by_the_story_kind() {
    assert_eq!(branch_type("US"), Some("feat"));
    assert_eq!(branch_type("EN"), Some("chore"));
    assert_eq!(branch_type("SP"), Some("docs"));
    assert_eq!(branch_type("AUD"), Some("docs"));
    assert_eq!(branch_type("VAL"), Some("feat"));
    // No `NAM` nor `RET` story since EN-123: the pre-push refuses their branches (AUD-025).
    assert_eq!(branch_type("NAM"), None);
    assert_eq!(branch_type("RET"), None);
    assert_eq!(branch_type("XY"), None);
}

#[test]
fn f23_ac2_refuses_the_branches_the_pre_push_refuses() {
    // [F23.AC2] one branch form, the pre-push's (`git_hooks::branch`, AUD-025): a topic without
    // a story id, or the id of a kind the auto-merge does not take, is refused here too.
    for name in [
        "docs/topic-only",
        "fix/a-fix",
        "chore/nam-003-naming",
        "docs/ret-005-retro",
        "chore/en-12-short-id",
    ] {
        assert!(branch_refusal(name).is_some(), "{name}");
    }
    for name in [
        "docs/aud-025-audit-f23",
        "chore/en-118-a-story",
        "fix/us-061-x",
    ] {
        assert_eq!(branch_refusal(name), None, "{name}");
    }
}

#[test]
fn refuses_a_remote_name_as_a_branch() {
    // 2026-10-16: a word split gone wrong in zsh created a branch named `origin/feature/…`.
    let remote = |name: &str| branch_refusal(name).is_some_and(|reason| reason.contains("remote"));

    assert!(remote("origin/feature/f18-safe-and-cheap-parallel-run"));
    assert!(remote("refs/heads/feat/us-061-x"));
    assert!(branch_refusal("feature/f18-safe").is_some());
    assert!(branch_refusal("feat/US-061-x").is_some());
    assert!(branch_refusal("feat/us-061--x").is_some());
    assert_eq!(branch_refusal("feat/us-061-font-list"), None);
}

#[test]
fn f22_ac4_starts_from_the_one_feature_branch_on_github_a_review_from_develop() {
    // [F22.AC4] A story of a feature starts from its feature branch, a `REV` from `develop`
    // (ADR-0086).
    assert_eq!(
        base_branch(Some("F18"), &remotes()).as_deref(),
        Ok("origin/feature/f18-safe-and-cheap-parallel-run")
    );
    assert_eq!(
        base_branch(None, &remotes()).as_deref(),
        Ok("origin/develop")
    );
    assert!(base_branch(Some("F19"), &remotes()).is_err());

    let twice = [remotes(), vec!["origin/feature/f18-old".to_owned()]].concat();

    assert!(base_branch(Some("F18"), &twice).is_err());
}

#[test]
fn plans_the_worktree_of_a_story() {
    let files = files();
    let remotes = remotes();
    let request = Request {
        id: "us-061",
        branch_type: None,
        story_files: &files,
        remote_branches: &remotes,
        branch_files: &[],
        main: Path::new("/work/repo"),
    };
    let made = plan(&request, |_| false).expect("a plan");

    assert_eq!(
        made.dir,
        PathBuf::from("/work/repo/.claude/worktrees/us-061")
    );
    assert_eq!(
        made.branch,
        "feat/us-061-font-list-held-while-the-default-font-is-read"
    );
    assert_eq!(made.base, "origin/feature/f18-safe-and-cheap-parallel-run");

    let typed = Request {
        id: "EN-052",
        branch_type: Some("refactor"),
        ..request
    };

    assert_eq!(
        plan(&typed, |_| false).map(|made| made.branch).as_deref(),
        Ok("refactor/en-052-worktrees-named-after-their-story")
    );
}

#[test]
fn refuses_what_it_cannot_make() {
    let files = files();
    let remotes = remotes();
    let request = Request {
        id: "US-061",
        branch_type: None,
        story_files: &files,
        remote_branches: &remotes,
        branch_files: &[],
        main: Path::new("/work/repo"),
    };
    let refusal = |request: &Request| plan(request, |_| false).expect_err("a refusal");

    assert!(
        plan(&request, |_| true)
            .expect_err("a refusal")
            .contains("already exists")
    );
    assert!(
        refusal(&Request {
            id: "US-999",
            ..request
        })
        .contains("no file")
    );
    assert!(
        refusal(&Request {
            id: "story",
            ..request
        })
        .contains("not a story id")
    );
    assert!(
        refusal(&Request {
            branch_type: Some("feature"),
            ..request
        })
        .contains("not a branch type")
    );
    assert!(
        refusal(&Request {
            remote_branches: &[],
            ..request
        })
        .contains("on GitHub")
    );
}

/// A story file written on a feature branch only, never on `develop`.
const ONLY_ON_THE_FEATURE: &str = "E17-F18-EN-200-only-on-the-feature.md";

/// The story files of two feature branches of `origin`, the second holding `EN-200` or not.
fn branch_files(second_holds_it: bool) -> Vec<(String, Vec<String>)> {
    let second = if second_holds_it {
        vec!["README.md".to_owned(), ONLY_ON_THE_FEATURE.to_owned()]
    } else {
        vec!["README.md".to_owned()]
    };

    vec![
        (
            "origin/feature/f17-text-fixes".to_owned(),
            vec!["README.md".to_owned()],
        ),
        (
            "origin/feature/f18-safe-and-cheap-parallel-run".to_owned(),
            vec![FILE.to_owned(), ONLY_ON_THE_FEATURE.to_owned()],
        ),
        ("origin/feature/f19-other".to_owned(), second),
    ]
}

#[test]
fn f23_ac2_starts_a_review_or_an_epics_spike_from_develop() {
    // [F23.AC2] A story attached to `develop`, a `REV` or an epic's refinement `SP`, has no
    // feature in its file name: its worktree starts from `origin/develop`, on a `docs/` branch.
    let files = [
        "E17-REV-006-review-2026-10-18.md".to_owned(),
        "E18-SP-030-refinement-of-e18.md".to_owned(),
    ];
    let remotes = remotes();
    let made = |id: &str| {
        plan(
            &Request {
                id,
                branch_type: None,
                story_files: &files,
                remote_branches: &remotes,
                branch_files: &[],
                main: Path::new("/work/repo"),
            },
            |_| false,
        )
        .map(|made| (made.branch, made.base))
    };

    assert_eq!(
        made("REV-006"),
        Ok((
            "docs/rev-006-review-2026-10-18".to_owned(),
            "origin/develop".to_owned()
        ))
    );
    assert_eq!(
        made("sp-030"),
        Ok((
            "docs/sp-030-refinement-of-e18".to_owned(),
            "origin/develop".to_owned()
        ))
    );
}

#[test]
fn f23_ac2_finds_a_story_missing_from_the_checkout_on_the_one_feature_branch_holding_it() {
    // [F23.AC2] The main checkout on `develop` lacks EN-200's file, which only F18's branch
    // holds: the story is read from there and its worktree starts from that branch.
    let files = files();
    let remotes = remotes();
    let held = branch_files(false);
    let request = Request {
        id: "EN-200",
        branch_type: None,
        story_files: &files,
        remote_branches: &remotes,
        branch_files: &held,
        main: Path::new("/work/repo"),
    };
    let made = plan(&request, |_| false).expect("a plan");

    assert_eq!(made.branch, "chore/en-200-only-on-the-feature");
    assert_eq!(made.base, "origin/feature/f18-safe-and-cheap-parallel-run");
    assert_eq!(
        story_on_branches("EN-200", &held).map(|(story, branch)| (story.topic, branch)),
        Ok((
            "only-on-the-feature".to_owned(),
            "origin/feature/f18-safe-and-cheap-parallel-run".to_owned()
        ))
    );
    // A story the checkout holds is read there, whatever the branches hold.
    assert_eq!(
        plan(
            &Request {
                id: "US-061",
                ..request
            },
            |_| false
        )
        .map(|made| made.branch)
        .as_deref(),
        Ok("feat/us-061-font-list-held-while-the-default-font-is-read")
    );
}

#[test]
fn f23_ac2_refuses_a_story_two_feature_branches_hold_naming_them() {
    let files = files();
    let remotes = remotes();
    let twice = branch_files(true);
    let request = Request {
        id: "EN-200",
        branch_type: None,
        story_files: &files,
        remote_branches: &remotes,
        branch_files: &twice,
        main: Path::new("/work/repo"),
    };
    let refusal = plan(&request, |_| false).expect_err("a refusal");

    assert!(
        refusal.contains("EN-200")
            && refusal.contains("several feature branches")
            && refusal.contains(
                "origin/feature/f18-safe-and-cheap-parallel-run, origin/feature/f19-other"
            ),
        "{refusal}"
    );
    // Held nowhere: the checkout and the branches both named.
    let nowhere = plan(
        &Request {
            id: "EN-201",
            ..request
        },
        |_| false,
    )
    .expect_err("a refusal");

    assert!(
        nowhere.contains("no file") && nowhere.contains("feature branch of origin"),
        "{nowhere}"
    );
    assert!(story_on_branches("EN-201", &twice).is_err());
}

/// A command cut off from the Git repository running the tests: a hook (`pre-push` running
/// `check:all`) sets `GIT_DIR` and its kin, which would point every `git` call at that repository.
fn isolated(program: &str, dir: &Path) -> Command {
    let mut command = test_support::command(program);

    command
        .current_dir(dir)
        .env("GIT_AUTHOR_NAME", "test")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com");
    command
}

fn git(dir: &Path, arguments: &[&str]) -> String {
    let output = isolated("git", dir)
        .args(["-c", "commit.gpgsign=false"])
        .args(arguments)
        .output()
        .unwrap();

    assert!(output.status.success(), "{output:?}");
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

/// The repository's own `node_modules`, which the tool links into each worktree.
fn node_modules() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../node_modules")
}

/// An executable `pre-push` hook writing `name` into the file `$MARKER`.
fn write_hook(path: &Path, name: &str) {
    fs::write(path, format!("#!/bin/sh\necho {name} > \"$MARKER\"\n")).unwrap();
    // Unix-only: Git runs a hook only when it is executable there; Windows has no such bit.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

/// A main checkout with a story file, `node_modules` and a `pre-push` hook naming its checkout,
/// its remote holding the feature branch.
fn repository(name: &str) -> PathBuf {
    repository_with(name, &[])
}

/// The same main checkout, holding more files: (path, text).
fn repository_with(name: &str, files: &[(&str, &str)]) -> PathBuf {
    // Git names folders by their real path: macOS's `/var` is a link to `/private/var`.
    let root = std::env::temp_dir()
        .canonicalize()
        .unwrap()
        .join(format!("worktree-new-{name}-{}", std::process::id()));
    let main = root.join("main");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(main.join("docs/backlog/stories")).unwrap();
    fs::create_dir_all(main.join(".githooks")).unwrap();
    // Unix-only, as the tool's own link: Windows makes a folder link by another call.
    #[cfg(unix)]
    std::os::unix::fs::symlink(node_modules(), main.join("node_modules")).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(node_modules(), main.join("node_modules")).unwrap();
    fs::write(
        main.join(".gitignore"),
        "node_modules\n.claude/worktrees/\n/target/\n",
    )
    .unwrap();
    fs::write(main.join("docs/backlog/stories").join(FILE), "# US-061\n").unwrap();
    write_hook(&main.join(".githooks/pre-push"), "main");
    for (path, text) in files {
        let path = main.join(path);

        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }
    // An absolute remote: a relative one would be read from each worktree's folder.
    let remote = root.join("remote.git");
    let remote = remote.to_str().unwrap();

    git(&main, &["init", "--quiet", "--bare", remote]);
    git(&main, &["init", "--quiet", "--initial-branch=develop"]);
    git(&main, &["remote", "add", "origin", remote]);
    git(&main, &["add", "--all"]);
    git(&main, &["commit", "--quiet", "-m", "chore: start"]);
    git(&main, &["push", "--quiet", "origin", "develop"]);
    git(
        &main,
        &[
            "push",
            "--quiet",
            "origin",
            "develop:feature/f18-safe-and-cheap-parallel-run",
        ],
    );
    main
}

fn worktree_new(dir: &Path, id: &str) -> Output {
    isolated(env!("CARGO_BIN_EXE_worktree-new"), dir)
        .arg(id)
        .output()
        .unwrap()
}

#[test]
fn f18_ac8_makes_a_worktree_named_after_its_story() {
    // [F18.AC8] `worktree-new US-061` gives `.claude/worktrees/us-061` on the story's branch.
    let main = repository("named");
    let made = worktree_new(&main, "US-061");
    let dir = main.join(".claude/worktrees/us-061");

    assert!(made.status.success(), "{made:?}");
    assert_eq!(
        git(&dir, &["branch", "--show-current"]),
        "feat/us-061-font-list-held-while-the-default-font-is-read"
    );
    assert_eq!(
        git(&dir, &["rev-parse", "HEAD"]),
        git(
            &main,
            &[
                "rev-parse",
                "origin/feature/f18-safe-and-cheap-parallel-run"
            ]
        )
    );
    assert_eq!(
        fs::read_link(dir.join("node_modules")).unwrap(),
        main.join("node_modules")
    );

    // From the worktree itself, the same story again: the folder is there, nothing is touched.
    let again = worktree_new(&dir, "us-061");

    assert!(!again.status.success());
    assert!(
        String::from_utf8_lossy(&again.stderr).contains("already exists"),
        "{again:?}"
    );

    let _ = fs::remove_dir_all(main.parent().unwrap());
}

#[test]
fn f19_ac9_builds_the_release_binaries_of_a_new_worktree() {
    // [F19.AC9] A worktree holding the Cargo workspace gets its tools' binaries, as `prepare`
    // builds them in a clone: the scripts and hooks call `target/release/<tool>` (EN-068).
    let main = repository_with(
        "build",
        &[
            (
                "Cargo.toml",
                "[workspace]\nmembers = [\"tools/*\"]\nresolver = \"3\"\n",
            ),
            (
                "Cargo.lock",
                "version = 4\n\n[[package]]\nname = \"hello\"\nversion = \"0.1.0\"\n",
            ),
            (
                "tools/hello/Cargo.toml",
                "[package]\nname = \"hello\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
            ),
            ("tools/hello/src/main.rs", "fn main() {}\n"),
        ],
    );
    let made = worktree_new(&main, "US-061");

    assert!(made.status.success(), "{made:?}");
    assert!(
        main.join(".claude/worktrees/us-061/target/release/hello")
            .exists()
    );

    let _ = fs::remove_dir_all(main.parent().unwrap());
}

#[test]
fn f18_ac8_a_push_from_a_new_worktree_runs_its_own_pre_push() {
    // The hooks path absolute, as Husky's was once, so a worktree ran the main checkout's
    // `pre-push` (EN-055): the tool sets it to the relative `.githooks` (EN-108).
    let main = repository("hooks");
    let hooks = main.join(".githooks");

    git(
        &main,
        &["config", "core.hooksPath", hooks.to_str().unwrap()],
    );

    let made = worktree_new(&main, "US-061");
    let dir = main.join(".claude/worktrees/us-061");

    assert!(made.status.success(), "{made:?}");
    assert_eq!(
        git(&main, &["config", "--get", "core.hooksPath"]),
        ".githooks"
    );

    // The story's branch changes its own `pre-push`: that one runs.
    let marker = main.parent().unwrap().join("marker");

    write_hook(&dir.join(".githooks/pre-push"), "worktree");
    git(
        &dir,
        &["commit", "--quiet", "--all", "-m", "chore: change the hook"],
    );
    let pushed = isolated("git", &dir)
        .args(["push", "--quiet", "origin", "HEAD"])
        .env("MARKER", &marker)
        .output()
        .unwrap();

    assert!(pushed.status.success(), "{pushed:?}");
    assert_eq!(fs::read_to_string(&marker).unwrap(), "worktree\n");

    let _ = fs::remove_dir_all(main.parent().unwrap());
}

/// A story file committed on F18's branch of `origin` only, then pushed to each other branch
/// named; the main checkout back on `develop`, without the file.
fn story_only_on_feature_branches(main: &Path, others: &[&str]) {
    let feature = "feature/f18-safe-and-cheap-parallel-run";

    git(
        main,
        &[
            "switch",
            "--quiet",
            "-c",
            "side",
            &format!("origin/{feature}"),
        ],
    );
    fs::write(
        main.join("docs/backlog/stories").join(ONLY_ON_THE_FEATURE),
        "# EN-200\n",
    )
    .unwrap();
    git(main, &["add", "--all"]);
    git(main, &["commit", "--quiet", "-m", "docs: write EN-200"]);
    for branch in [feature].iter().chain(others) {
        git(
            main,
            &["push", "--quiet", "origin", &format!("side:{branch}")],
        );
    }
    git(main, &["switch", "--quiet", "develop"]);
    git(main, &["branch", "--quiet", "-D", "side"]);
    assert!(
        !main
            .join("docs/backlog/stories")
            .join(ONLY_ON_THE_FEATURE)
            .exists()
    );
}

#[test]
fn f23_ac2_worktree_new_in_the_main_checkout_finds_a_story_only_on_its_feature_branch() {
    // [F23.AC2] Run in the main checkout on `develop`, which lacks the file: found by
    // `git ls-tree` on the feature branches of `origin`, the worktree made from that branch.
    let main = repository("ls-tree");

    story_only_on_feature_branches(&main, &[]);

    let made = worktree_new(&main, "EN-200");
    let dir = main.join(".claude/worktrees/en-200");

    assert!(made.status.success(), "{made:?}");
    assert_eq!(
        git(&dir, &["branch", "--show-current"]),
        "chore/en-200-only-on-the-feature"
    );
    assert_eq!(
        git(&dir, &["rev-parse", "HEAD"]),
        git(
            &main,
            &[
                "rev-parse",
                "origin/feature/f18-safe-and-cheap-parallel-run"
            ]
        )
    );
    assert!(
        dir.join("docs/backlog/stories")
            .join(ONLY_ON_THE_FEATURE)
            .exists()
    );

    let _ = fs::remove_dir_all(main.parent().unwrap());
}

#[test]
fn f23_ac2_worktree_new_refuses_a_story_two_feature_branches_hold() {
    let main = repository("ls-tree-twice");

    story_only_on_feature_branches(&main, &["feature/f19-other"]);

    let made = worktree_new(&main, "EN-200");
    let said = String::from_utf8_lossy(&made.stderr);

    assert!(!made.status.success());
    assert!(
        said.contains("origin/feature/f18-safe-and-cheap-parallel-run, origin/feature/f19-other"),
        "{made:?}"
    );
    assert!(!main.join(".claude/worktrees/en-200").exists());

    let _ = fs::remove_dir_all(main.parent().unwrap());
}

#[test]
fn lists_the_story_files_of_a_checkout() {
    let top = std::env::temp_dir().join(format!("worktree-stories-{}", std::process::id()));
    let _ = fs::remove_dir_all(&top);

    // No backlog: an error each caller reads its own way.
    assert!(worktree::story_files(&top).is_err());
    fs::create_dir_all(top.join(worktree::STORIES)).expect("folders");
    fs::write(top.join(worktree::STORIES).join(FILE), "").expect("story");

    assert_eq!(worktree::story_files(&top).expect("listed"), [FILE]);
    fs::remove_dir_all(&top).expect("cleaned");
}
