//! Tests of `worktree-new` (EN-052): the plan from a story the records hold (EN-154), then the tool run in a
//! repository of its own. `[F18.AC8]` marks a test proving the feature's criterion.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use worktree::{
    Request, Story, base_branch, branch_refusal, branch_type, plan, story_id, story_of_branch,
    story_of_row, story_statement,
};

/// US-061 as the records hold it.
fn us_061() -> Story {
    Story {
        id: "US-061".to_owned(),
        kind: "US".to_owned(),
        feature: Some("F18".to_owned()),
        topic: "font-list-held-while-the-default-font-is-read".to_owned(),
    }
}

/// A story of an epic alone, as the records hold a review or an epic's spike.
fn of_epic(id: &str, topic: &str) -> Story {
    Story {
        id: id.to_owned(),
        kind: id.split_once('-').map_or("", |(kind, _)| kind).to_owned(),
        feature: None,
        topic: topic.to_owned(),
    }
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
fn reads_the_story_of_a_row_of_the_records() {
    let row = |cells: [&str; 3]| cells.map(str::to_owned).to_vec();

    assert_eq!(
        story_of_row(
            "US-061",
            &row(["US", "F18", "font-list-held-while-the-default-font-is-read"])
        ),
        Some(us_061())
    );
    assert_eq!(
        story_of_row("REV-003", &row(["REV", "", "review-e15"])),
        Some(of_epic("REV-003", "review-e15"))
    );
    // A row without topic or kind, or of another shape: none.
    assert_eq!(story_of_row("US-061", &row(["US", "F18", ""])), None);
    assert_eq!(story_of_row("US-061", &row(["", "F18", "x"])), None);
    assert_eq!(story_of_row("US-061", &["US".to_owned()]), None);
    assert_eq!(
        story_statement("US-061").as_deref(),
        Some("SELECT kind, feature, topic FROM stories WHERE id = 'US-061'")
    );
    // Nothing but an id enters a statement.
    assert_eq!(story_statement("US-061' OR '1'='1"), None);
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
    let remotes = remotes();
    let story = us_061();
    let request = Request {
        id: "us-061",
        branch_type: None,
        topic: None,
        story: Some(&story),
        remote_branches: &remotes,
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

    let enabler = Story {
        id: "EN-052".to_owned(),
        kind: "EN".to_owned(),
        topic: "worktrees-named-after-their-story".to_owned(),
        ..us_061()
    };
    let typed = Request {
        id: "EN-052",
        branch_type: Some("refactor"),
        story: Some(&enabler),
        ..request
    };

    assert_eq!(
        plan(&typed, |_| false).map(|made| made.branch).as_deref(),
        Ok("refactor/en-052-worktrees-named-after-their-story")
    );
}

#[test]
fn refuses_what_it_cannot_make() {
    let remotes = remotes();
    let story = us_061();
    let request = Request {
        id: "US-061",
        branch_type: None,
        topic: None,
        story: Some(&story),
        remote_branches: &remotes,
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
            story: None,
            ..request
        })
        .contains("no story US-999 in the records")
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

#[test]
fn f23_ac2_starts_a_review_or_an_epics_spike_from_develop() {
    // [F23.AC2] A story attached to `develop`, a `REV` or an epic's refinement `SP`, has no
    // feature in the records: its worktree starts from `origin/develop`, on a `docs/` branch.
    let remotes = remotes();
    let made = |id: &str, story: &Story| {
        plan(
            &Request {
                id,
                branch_type: None,
                topic: None,
                story: Some(story),
                remote_branches: &remotes,
                main: Path::new("/work/repo"),
            },
            |_| false,
        )
        .map(|made| (made.branch, made.base))
    };

    assert_eq!(
        made("REV-006", &of_epic("REV-006", "review-2026-10-18")),
        Ok((
            "docs/rev-006-review-2026-10-18".to_owned(),
            "origin/develop".to_owned()
        ))
    );
    assert_eq!(
        made("sp-030", &of_epic("SP-030", "refinement-of-e18")),
        Ok((
            "docs/sp-030-refinement-of-e18".to_owned(),
            "origin/develop".to_owned()
        ))
    );
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

/// A main checkout with `node_modules` and a `pre-push` hook naming its checkout,
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
    fs::create_dir_all(main.join("docs")).unwrap();
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

/// A copy of `worktree-new` in a folder of its own beside a `knowledge` script standing for the
/// records: it holds US-061 of F18 and EN-200 of F18, a story written on the feature branch
/// alone, and fails as records without `AGENT_RECORDS_ROOT` do for EN-300.
fn worktree_new_beside_records(main: &Path) -> PathBuf {
    let folder = main.parent().unwrap().join("bin");
    let binary = folder.join("worktree-new");
    let knowledge = folder.join("knowledge");

    fs::create_dir_all(&folder).unwrap();
    // Copied once: a binary written again over one just run is killed on macOS, its signature
    // read from the first copy.
    if !binary.exists() {
        fs::copy(env!("CARGO_BIN_EXE_worktree-new"), &binary).unwrap();
    }
    fs::write(
        &knowledge,
        "#!/bin/sh\ncase \"$2\" in\n  *\"'US-061'\"*) printf 'kind\\tfeature\\ttopic\\nUS\\tF18\\tfont-list-held-while-the-default-font-is-read\\n';;\n  *\"'EN-200'\"*) printf 'kind\\tfeature\\ttopic\\nEN\\tF18\\tonly-on-the-feature\\n';;\n  *\"'EN-300'\"*) echo 'knowledge: AGENT_RECORDS_ROOT is not set' >&2; exit 1;;\n  *) printf 'kind\\tfeature\\ttopic\\n';;\nesac\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&knowledge, fs::Permissions::from_mode(0o755)).unwrap();
    }
    binary
}

fn worktree_new(dir: &Path, id: &str) -> Output {
    let main = dir
        .ancestors()
        .find(|folder| folder.file_name().is_some_and(|name| name == "main"))
        .unwrap();

    isolated(worktree_new_beside_records(main).to_str().unwrap(), dir)
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

#[test]
fn f23_ac2_worktree_new_in_the_main_checkout_finds_in_the_records_a_story_written_on_its_feature_branch()
 {
    // [F23.AC2] EN-200 is written in the records on F18's branch, which `develop` lacks: the main
    // checkout on `develop` finds it there, and makes its worktree from that feature branch.
    let main = repository("records");
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

    // A story the records lack, and records that cannot be read: refused, nothing made.
    let unknown = worktree_new(&main, "EN-201");
    let unread = worktree_new(&main, "EN-300");

    assert!(!unknown.status.success());
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("no story EN-201 in the records"),
        "{unknown:?}"
    );
    assert!(!unread.status.success());
    assert!(
        String::from_utf8_lossy(&unread.stderr).contains("AGENT_RECORDS_ROOT is not set"),
        "{unread:?}"
    );
    assert!(!main.join(".claude/worktrees/en-201").exists());

    let _ = fs::remove_dir_all(main.parent().unwrap());
}

#[test]
fn rev009_names_a_bubble_s_branch_by_its_own_type_and_topic_from_develop() {
    // A review's bubble, `build/rev-009-tool-cards-and-telemetry`, was made by hand: the branch
    // took the story's topic (four worktrees of REV-008 and REV-009, 2026-10-24).
    let remotes = remotes();
    let story = of_epic("REV-009", "review-of-2026-10-25");
    let request = Request {
        id: "REV-009",
        branch_type: Some("build"),
        topic: Some("tool-cards-and-telemetry"),
        story: Some(&story),
        remote_branches: &remotes,
        main: Path::new("/work/repo"),
    };
    let made = plan(&request, |_| false).expect("a plan");

    assert_eq!(made.branch, "build/rev-009-tool-cards-and-telemetry");
    assert_eq!(made.base, "origin/develop");
    assert_eq!(
        made.dir,
        PathBuf::from("/work/repo/.claude/worktrees/rev-009")
    );
    // A topic is a branch name's part: one in capitals is refused as the branch is.
    assert!(
        plan(
            &Request {
                topic: Some("Tool-Cards"),
                ..request
            },
            |_| false
        )
        .is_err()
    );
}

#[test]
fn rev009_starts_an_epic_s_spike_the_records_lack_from_develop_when_given_its_topic() {
    // SP-033 is not in the records yet: given its topic, it starts from `develop`.
    let remotes = remotes();
    let request = Request {
        id: "SP-033",
        branch_type: None,
        topic: Some("regroup-the-epics"),
        story: None,
        remote_branches: &remotes,
        main: Path::new("/work/repo"),
    };
    let made = plan(&request, |_| false).expect("a plan");

    assert_eq!(made.branch, "docs/sp-033-regroup-the-epics");
    assert_eq!(made.base, "origin/develop");
    // Without a topic it is refused; a user story belongs to a feature the records give.
    assert!(
        plan(
            &Request {
                topic: None,
                ..request
            },
            |_| false
        )
        .is_err_and(|reason| reason.contains("no story SP-033 in the records"))
    );
    assert!(
        plan(
            &Request {
                id: "US-099",
                ..request
            },
            |_| false
        )
        .is_err_and(|reason| reason.contains("no story US-099 in the records"))
    );
}
