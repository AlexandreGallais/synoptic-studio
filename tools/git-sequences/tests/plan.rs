//! Tests of what `git:sync` and `git:commit` run (EN-100), decided without Git.

use git_sequences::{
    Branch, Push, Request, Source, Target, branch_of, commit_refusal, commit_steps, fetch_step,
    message_refusal, parse, sync_steps, sync_target,
};

fn words(line: &str) -> Vec<String> {
    line.split_whitespace().map(str::to_owned).collect()
}

const STORY: &str = "chore/en-100-git-sequences-as-rust-commands";

fn files() -> Vec<String> {
    vec![
        "README.md".to_owned(),
        "E17-F20-EN-100-git-sequences-as-rust-commands.md".to_owned(),
        "E15-REV-003-review-e15.md".to_owned(),
    ]
}

fn remotes() -> Vec<String> {
    vec![
        "origin/develop".to_owned(),
        "origin/main".to_owned(),
        "origin/feature/f20-a-context-diet".to_owned(),
        "origin/chore/en-100-git-sequences-as-rust-commands".to_owned(),
    ]
}

#[test]
fn parses_a_sync_with_or_without_its_base() {
    assert_eq!(parse(&words("sync")), Ok(Request::Sync { base: None }));
    assert_eq!(
        parse(&words("sync --base origin/develop")),
        Ok(Request::Sync {
            base: Some("origin/develop".to_owned())
        })
    );
    // A base missing, an option for a base, an unknown word, no subcommand.
    assert!(parse(&words("sync --base")).is_err());
    assert!(parse(&words("sync --base --push")).is_err());
    assert!(parse(&words("sync now")).is_err());
    assert!(parse(&[]).is_err());
    assert!(parse(&words("rebase")).is_err());
}

#[test]
fn parses_a_commit_its_message_paths_and_push() {
    assert_eq!(
        parse(&words("commit -F /tmp/m.txt docs/a.md tools/b.rs")),
        Ok(Request::Commit {
            message: Source::File("/tmp/m.txt".to_owned()),
            paths: words("docs/a.md tools/b.rs"),
            push: Push::No,
        })
    );
    assert_eq!(
        parse(&words("commit --push -F - -- -odd-name.md")),
        Ok(Request::Commit {
            message: Source::Stdin,
            paths: words("-odd-name.md"),
            push: Push::Plain,
        })
    );
    assert_eq!(
        parse(&words("commit --file m.txt --push-with-lease")),
        Ok(Request::Commit {
            message: Source::File("m.txt".to_owned()),
            paths: Vec::new(),
            push: Push::Lease,
        })
    );
    // No message, a flag without its file, a bypass of the hooks asked for.
    assert!(parse(&words("commit a.md")).is_err());
    assert!(parse(&words("commit -F")).is_err());
    assert!(
        parse(&words("commit -F m.txt --no-verify a.md"))
            .is_err_and(|reason| reason.contains("--no-verify"))
    );
    assert!(parse(&words("commit -F m.txt --force")).is_err());
}

#[test]
fn tells_main_a_story_branch_and_the_shared_ones_apart() {
    assert_eq!(branch_of("main\n"), Branch::Main);
    assert_eq!(branch_of("HEAD"), Branch::Detached);
    assert_eq!(branch_of(""), Branch::Detached);
    assert_eq!(branch_of(STORY), Branch::Story(STORY.to_owned()));
    assert_eq!(
        branch_of("feature/f20-a-context-diet"),
        Branch::Other("feature/f20-a-context-diet".to_owned())
    );
    assert_eq!(branch_of("develop"), Branch::Other("develop".to_owned()));
}

#[test]
fn rebases_a_story_on_its_feature_branch() {
    // [EN-100.AC1] the feature branch of F20, found from the story file named by the branch.
    assert_eq!(
        sync_target(&branch_of(STORY), None, &files(), &remotes()),
        Ok(Target::Rebase(
            "origin/feature/f20-a-context-diet".to_owned()
        ))
    );
    // An epic's story starts from develop.
    assert_eq!(
        sync_target(
            &branch_of("docs/rev-003-review-e15"),
            None,
            &files(),
            &remotes()
        ),
        Ok(Target::Rebase("origin/develop".to_owned()))
    );
    // A base typed wins.
    assert_eq!(
        sync_target(&branch_of(STORY), Some("origin/develop"), &[], &[]),
        Ok(Target::Rebase("origin/develop".to_owned()))
    );
}

#[test]
fn moves_a_shared_branch_forward_and_never_rebases_it() {
    let feature = branch_of("feature/f20-a-context-diet");

    assert_eq!(
        sync_target(&feature, None, &files(), &remotes()),
        Ok(Target::FastForward)
    );
    assert!(
        sync_target(&feature, Some("origin/develop"), &files(), &remotes())
            .is_err_and(|reason| reason.contains("never rebased"))
    );
}

#[test]
fn refuses_to_sync_main_a_detached_head_or_a_story_without_base() {
    assert!(
        sync_target(&Branch::Main, None, &files(), &remotes())
            .is_err_and(|reason| reason.contains("main"))
    );
    assert!(sync_target(&Branch::Detached, None, &files(), &remotes()).is_err());
    // No story file, no feature branch on GitHub, no id in the branch name.
    assert!(
        sync_target(&branch_of("chore/en-101-other"), None, &files(), &remotes())
            .is_err_and(|reason| reason.contains("EN-101"))
    );
    assert!(sync_target(&branch_of(STORY), None, &files(), &[]).is_err());
    assert!(sync_target(&branch_of("chore/tidy-up"), None, &files(), &remotes()).is_err());
}

#[test]
fn syncs_by_a_fetch_then_one_step() {
    assert_eq!(fetch_step(), words("fetch --quiet origin"));
    assert_eq!(
        sync_steps(&Target::Rebase("origin/feature/f20-x".to_owned())),
        vec![words("rebase --quiet origin/feature/f20-x")]
    );
    assert_eq!(
        sync_steps(&Target::FastForward),
        vec![words("merge --ff-only --quiet @{upstream}")]
    );
}

#[test]
fn commits_the_files_named_then_pushes_without_bypass() {
    assert_eq!(
        commit_steps(&words("a.md b.rs"), Push::Plain),
        vec![
            words("add -- a.md b.rs"),
            words("commit --quiet -F -"),
            words("push --quiet -u origin HEAD"),
        ]
    );
    // Nothing named: what is staged already is committed.
    assert_eq!(
        commit_steps(&[], Push::No),
        vec![words("commit --quiet -F -")]
    );
    assert_eq!(
        commit_steps(&[], Push::Lease).last(),
        Some(&words("push --quiet --force-with-lease -u origin HEAD"))
    );

    for push in [Push::No, Push::Plain, Push::Lease] {
        let every_word: Vec<String> = commit_steps(&words("a.md"), push)
            .into_iter()
            .flatten()
            .collect();

        assert!(!every_word.iter().any(|word| word == "--no-verify"
            || word == "-n"
            || word == "--force"
            || word == "-f"));
    }
}

#[test]
fn refuses_to_commit_on_main_and_to_force_a_shared_branch() {
    assert!(commit_refusal(&Branch::Main, Push::No).is_some());
    assert!(commit_refusal(&Branch::Detached, Push::No).is_some());
    assert!(commit_refusal(&branch_of("develop"), Push::Lease).is_some());
    assert_eq!(commit_refusal(&branch_of(STORY), Push::Lease), None);
}

#[test]
fn f20_ac7_never_pushes_develop_but_pushes_a_feature_branch() {
    // `develop` takes merged pull requests only (AUD-022): committed there, never pushed.
    assert_eq!(
        commit_refusal(&branch_of("develop"), Push::Plain).as_deref(),
        Some("`develop` takes merged pull requests only: git:commit never pushes it")
    );
    assert_eq!(commit_refusal(&branch_of("develop"), Push::No), None);
    // The main thread pushes the shared files of a merge on the feature branch.
    assert_eq!(
        commit_refusal(&branch_of("feature/f20-a-context-diet"), Push::Plain),
        None
    );
}

#[test]
fn refuses_an_empty_message() {
    assert!(message_refusal("").is_some());
    assert!(message_refusal(" \n\t\n").is_some());
    assert_eq!(
        message_refusal("chore(tooling): add\n\nRefs: EN-100.T1\n"),
        None
    );
}
