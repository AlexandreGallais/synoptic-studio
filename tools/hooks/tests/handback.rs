//! Tests of the hand-back check and of the `check-handback` hook (EN-088).

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use hooks::handback::{
    Branch, Commit, LISTED_COMMITS, Refs, agent_transcript, allow_output, base_of,
    earlier_handbacks, finish_line, is_checked, missing, read_refs, refusal, story_of,
};
use hooks::json::{parse, quote, write};
use hooks::worktrees::{prompt_worktree, worktree_id};

/// A branch with one commit, its head and its copy on `origin`.
fn branch(head: &str, remote: Option<&str>) -> Branch {
    Branch {
        name: "chore/en-123-a-topic".into(),
        head: head.into(),
        remote: remote.map(Into::into),
        commits: vec![Commit {
            hash: "abcdef1234567890".into(),
            subject: "feat(tooling): add a thing".into(),
        }],
    }
}

#[test]
fn checks_only_a_subagent_that_may_write() {
    assert!(is_checked("a1", "general-purpose"));
    assert!(is_checked("a1", "story-worker"));
    // The main thread has no agent id; an internal agent has an empty type.
    assert!(!is_checked("", ""));
    assert!(!is_checked("", "general-purpose"));
    assert!(!is_checked("a1", ""));
    for reader in [
        "Explore",
        "Plan",
        "auditor",
        "math-expert",
        "functional-analyst",
        "claude-code-guide",
        "statusline-setup",
    ] {
        assert!(!is_checked("a1", reader), "{reader}");
    }
}

#[test]
fn finds_the_subagent_transcript_beside_the_session() {
    assert_eq!(
        agent_transcript("/p/abc123.jsonl", "def456"),
        Path::new("/p/abc123/subagents/agent-def456.jsonl")
    );
}

#[test]
fn finds_the_first_worktree_named() {
    let text = r#"{"content":"Worktree: `/r/SVG editor/.claude/worktrees/en-088`, then .claude/worktrees/us-001"}"#;

    assert_eq!(worktree_id(text), Some("en-088"));
    assert_eq!(
        worktree_id(r#"cd "/r/.claude/worktrees/us-061" || exit 9"#),
        Some("us-061")
    );
    // A folder named without a story after it, or none at all.
    assert_eq!(worktree_id(".claude/worktrees/ and more"), None);
    assert_eq!(worktree_id("nothing here"), None);
}

#[test]
fn names_the_story_by_its_branch_then_its_folder() {
    // A folder named otherwise than its story (AUD-022): the branch tells.
    assert_eq!(
        story_of("chore/en-087-drop-the-page", "en-087-drop").as_deref(),
        Some("EN-087")
    );
    assert_eq!(
        story_of("docs/f21-refinement", "en-088").as_deref(),
        Some("EN-088")
    );
    assert_eq!(story_of("docs/f21-refinement", "f21-refinement"), None);
}

#[test]
fn counts_from_the_feature_branch_or_develop_never_the_whole_history() {
    // The stories as the records hold them (EN-154).
    let story = |id: &str, kind: &str, feature: Option<&str>| worktree::Story {
        id: id.to_owned(),
        kind: kind.to_owned(),
        feature: feature.map(str::to_owned),
        topic: "a-topic".to_owned(),
    };
    let files = [
        story("EN-088", "EN", Some("F20")),
        story("REV-009", "REV", None),
    ];
    // `f2-b` listed before `f20-a`: the prefix `feature/f20-` keeps them apart.
    let remotes = [
        "origin/develop".to_owned(),
        "origin/feature/f2-b".to_owned(),
        "origin/feature/f20-a".to_owned(),
    ];

    assert_eq!(
        base_of("EN-088", &files, &remotes).as_deref(),
        Some("origin/feature/f20-a")
    );
    assert_eq!(
        base_of("REV-009", &files, &remotes).as_deref(),
        Some("origin/develop")
    );
    // No story in the records, or no feature branch on origin: no base.
    assert_eq!(base_of("EN-089", &files, &remotes), None);
    assert_eq!(base_of("EN-088", &files, &remotes[..2]), None);
}

#[test]
fn reads_the_branch_its_copy_and_the_remote_branches_from_one_listing() {
    let listing = "  1111 refs/heads/feature/f20-a-diet\n\
                   * 2222 refs/heads/chore/en-123-a-topic\n\
                   \u{20} 3333 refs/remotes/origin/chore/en-123-a-topic\n\
                   \u{20} 4444 refs/remotes/origin/feature/f2-other\n\
                   \u{20} 1111 refs/remotes/origin/feature/f20-a-diet";

    assert_eq!(
        read_refs(listing),
        Some(Refs {
            name: "chore/en-123-a-topic".into(),
            head: "2222".into(),
            remote: Some("3333".into()),
            remote_branches: vec![
                "origin/chore/en-123-a-topic".into(),
                "origin/feature/f2-other".into(),
                "origin/feature/f20-a-diet".into(),
            ],
        })
    );
    // Never pushed; a detached head names no branch.
    assert_eq!(
        read_refs("* 2222 refs/heads/b").and_then(|refs| refs.remote),
        None
    );
    assert_eq!(read_refs("  2222 refs/heads/b"), None);
}

#[test]
fn counts_the_hand_backs_before_this_one() {
    let call = |id: &str| {
        format!(
            r#"{{"type":"assistant","message":{{"content":[{{"type":"tool_use","id":"{id}","name":"SubagentHandback","input":{{"message":"r"}}}}]}}}}"#
        )
    };
    let lines = [call("t1"), call("t2"), "{".to_owned()].join("\n");
    // A command that only names the tool is no hand-back.
    let bash = r#"{"type":"assistant","message":{"content":[{"type":"tool_use","id":"b1","name":"Bash","input":{"command":"grep SubagentHandback x"}}]}}"#;

    // The current call t2 already written: one earlier; not written yet: two.
    assert_eq!(earlier_handbacks(&lines, "t2"), 1);
    assert_eq!(earlier_handbacks(&lines, "t3"), 2);
    assert_eq!(earlier_handbacks("", "t1"), 0);
    assert_eq!(earlier_handbacks(bash, "t1"), 0);
}

#[test]
fn finds_nothing_missing_when_pushed_and_named() {
    let pushed = branch("abcdef1234567890", Some("abcdef1234567890"));

    assert_eq!(
        missing(&pushed, "Commits: abcdef1 add a thing"),
        [] as [std::string::String; 0]
    );
    // The subject names it as well as the hash.
    assert_eq!(
        missing(&pushed, "feat(tooling): add a thing"),
        [] as [std::string::String; 0]
    );
}

#[test]
fn names_what_is_missing() {
    let unpushed = branch("abcdef1234567890", None);
    let behind = branch("abcdef1234567890", Some("1111111222222"));
    let report = "abcdef1";

    assert_eq!(
        missing(&unpushed, report),
        ["push `chore/en-123-a-topic`: origin has no such branch"]
    );
    assert_eq!(
        missing(&behind, report),
        ["push `chore/en-123-a-topic`: origin at 1111111, the worktree at abcdef1"]
    );
    assert_eq!(
        missing(
            &branch("abcdef1234567890", Some("abcdef1234567890")),
            "done"
        ),
        ["name each commit in the report: abcdef1 feat(tooling): add a thing"]
    );
}

#[test]
fn lists_ten_unnamed_commits_and_counts_the_others() {
    let mut many = branch("abcdef1234567890", Some("abcdef1234567890"));

    many.commits = (0..12)
        .map(|index| Commit {
            hash: format!("{index:07}0000"),
            subject: format!("s{index}"),
        })
        .collect();
    let items = missing(&many, "done");
    let listed: Vec<String> = (0..LISTED_COMMITS)
        .map(|index| format!("{index:07} s{index}"))
        .collect();

    // 12 unnamed, 10 listed: 2 more.
    assert_eq!(LISTED_COMMITS, 10);
    assert_eq!(
        items,
        [format!(
            "name each commit in the report: {}; and 2 more",
            listed.join("; ")
        )]
    );
    many.commits.truncate(LISTED_COMMITS);
    assert_eq!(
        missing(&many, "done"),
        [format!(
            "name each commit in the report: {}",
            listed.join("; ")
        )]
    );
}

#[test]
fn words_the_refusal_and_the_line_for_the_main_thread() {
    let items = ["push `b`: origin has no such branch".to_owned()];

    assert_eq!(
        refusal("EN-123", &items),
        "Hand-back of EN-123 refused once (check-handback, EN-088): push `b`: origin has no such branch. Do it, or hand back again saying why not."
    );
    assert_eq!(
        finish_line(
            "EN-123",
            "chore/en-123-a-topic",
            "abcdef1",
            Some("feature/f20-a-diet"),
            &[]
        ),
        "check-handback: EN-123 on `chore/en-123-a-topic` pushed at abcdef1; next: `npm run story:finish -- feature/f20-a-diet`."
    );
    assert_eq!(
        finish_line("EN-123", "b", "abcdef1", None, &items),
        "check-handback: EN-123 on `b` handed back with: push `b`: origin has no such branch."
    );
}

#[test]
fn puts_the_line_above_the_report() {
    let input = parse(r#"{"message":"Report.","n":1}"#).expect("JSON");
    let output = parse(&allow_output(&input, "Line.")).expect("JSON");
    let decision = output.get("hookSpecificOutput");

    assert_eq!(decision.get("hookEventName").text(), "PreToolUse");
    assert_eq!(decision.get("permissionDecision").text(), "allow");
    // The message first, the other fields kept, nothing added.
    assert_eq!(
        write(decision.get("updatedInput")),
        r#"{"message":"Line.\n\nReport.","n":1}"#
    );
}

#[test]
fn writes_json_back() {
    let text = r#"{"a":[1,2.5,true,null],"b":"x\"y","c":{}}"#;

    assert_eq!(write(&parse(text).expect("JSON")), text);
    assert_eq!(quote("é"), "\"é\"");
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself, and a set-up inheriting it rewrote its config. Nor
/// the user's or the system's Git config: an empty home of its own.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);
    let home = std::env::temp_dir().join(format!("check-handback-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    command.env("HOME", home).env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

/// A copy of the hook beside a `knowledge` script standing for the records, in a test's folder:
/// it holds EN-123 of F20 while the file `en-123` is there, so that no test reads the real
/// records (EN-154).
fn beside_records(root: &Path) -> String {
    let folder = root.join("bin");
    let binary = folder.join("check-handback");
    let script = folder.join("knowledge");

    fs::create_dir_all(&folder).expect("bin");
    // Copied once: a binary written again over one just run is killed on macOS, its signature
    // read from the first copy.
    if !binary.exists() {
        fs::copy(env!("CARGO_BIN_EXE_check-handback"), &binary).expect("hook copied");
    }
    fs::write(
        &script,
        format!(
            "#!/bin/sh\nprintf 'kind\\tfeature\\ttopic\\n'\ncase \"$2\" in *\"'EN-123'\"*) [ -f '{}' ] && printf 'EN\\tF20\\ta-topic\\n';; esac\nexit 0\n",
            root.join("en-123").display()
        ),
    )
    .expect("script");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).expect("script runs");
    }
    binary.display().to_string()
}

/// Runs Git in a folder, panicking on a failure.
fn git(folder: &Path, arguments: &[&str]) -> String {
    let output = clean("git")
        .args(arguments)
        .current_dir(folder)
        .output()
        .expect("git runs");

    assert!(output.status.success(), "git {arguments:?}");
    String::from_utf8(output.stdout)
        .expect("UTF-8")
        .trim()
        .to_owned()
}

/// A project with a remote, a feature branch and a story's worktree of one commit, not pushed.
fn project(root: &Path) -> std::path::PathBuf {
    project_in(root, "en-123")
}

/// The same, its worktree in a folder of this name.
fn project_in(root: &Path, folder: &str) -> std::path::PathBuf {
    let remote = root.join("remote.git");
    let worktree = root.join("project/.claude/worktrees").join(folder);

    fs::create_dir_all(&worktree).expect("folders");
    git(root, &["init", "-q", "--bare", "remote.git"]);
    git(&worktree, &["init", "-q", "-b", "feature/f20-a-diet"]);
    for (key, value) in [
        ("user.name", "t"),
        ("user.email", "t@t"),
        ("commit.gpgsign", "false"),
    ] {
        git(&worktree, &["config", key, value]);
    }
    git(
        &worktree,
        &["remote", "add", "origin", &remote.display().to_string()],
    );
    git(&worktree, &["commit", "-q", "--allow-empty", "-m", "start"]);
    git(&worktree, &["push", "-q", "origin", "feature/f20-a-diet"]);
    git(&worktree, &["switch", "-q", "-c", "chore/en-123-a-topic"]);
    // EN-123 in the records the `knowledge` script stands for, while this file is there.
    fs::write(root.join("en-123"), "").expect("story recorded");
    git(
        &worktree,
        &[
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "feat(tooling): add a thing",
        ],
    );
    worktree
}

/// Runs the hook on a hand-back of this agent, its transcript holding these lines.
fn hook(root: &Path, agent_type: &str, lines: &[String], message: &str) -> String {
    let transcript = root.join("session.jsonl");
    let folder = root.join("session/subagents");

    fs::create_dir_all(&folder).expect("folder");
    fs::write(folder.join("agent-a1.jsonl"), lines.join("\n") + "\n").expect("transcript");
    let input = format!(
        r#"{{"tool_name":"SubagentHandback","tool_input":{{"message":{}}},"tool_use_id":"now","agent_id":"a1","agent_type":"{agent_type}","cwd":{},"transcript_path":{}}}"#,
        quote(message),
        quote(&root.join("project").display().to_string()),
        quote(&transcript.display().to_string())
    );
    let mut child = clean(&beside_records(root))
        .env("CLAUDE_PROJECT_DIR", root.join("project"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("hook starts");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input");
    let output = child.wait_with_output().expect("hook ends");

    assert!(output.status.success());
    String::from_utf8(output.stdout).expect("UTF-8")
}

#[test]
fn f20_ac3_the_hook_refuses_once_then_lets_through_and_names_story_finish() {
    let root = std::env::temp_dir().join(format!("check-handback-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let worktree = project(&root);
    let head = git(&worktree, &["rev-parse", "--short=7", "HEAD"]);
    let prompt =
        r#"{"type":"user","message":{"content":"Worktree: `/x/.claude/worktrees/en-123`"}}"#
            .to_owned();
    let earlier = r#"{"type":"assistant","message":{"content":[{"type":"tool_use","id":"before","name":"SubagentHandback","input":{"message":"r"}}]}}"#.to_owned();
    let decision = |output: &str| {
        parse(output)
            .expect("JSON")
            .get("hookSpecificOutput")
            .clone()
    };

    // Not pushed: refused once.
    let refused = decision(&hook(
        &root,
        "general-purpose",
        std::slice::from_ref(&prompt),
        &head,
    ));
    assert_eq!(refused.get("permissionDecision").text(), "deny");
    assert!(
        refused
            .get("permissionDecisionReason")
            .text()
            .contains("origin has no such branch")
    );

    // Handed back again: through, with what is missing above the report.
    let through = decision(&hook(
        &root,
        "general-purpose",
        &[prompt.clone(), earlier],
        &head,
    ));
    let message = through.get("updatedInput").get("message").text().to_owned();
    assert_eq!(through.get("permissionDecision").text(), "allow");
    assert!(
        message
            .starts_with("check-handback: EN-123 on `chore/en-123-a-topic` handed back with: push"),
        "{message}"
    );

    // Pushed and named: the line for the main thread.
    git(&worktree, &["push", "-q", "-u", "origin", "HEAD"]);
    let pushed = decision(&hook(
        &root,
        "general-purpose",
        std::slice::from_ref(&prompt),
        &format!("Commits: {head}"),
    ));
    let message = pushed.get("updatedInput").get("message").text().to_owned();
    assert_eq!(
        message,
        format!(
            "check-handback: EN-123 on `chore/en-123-a-topic` pushed at {head}; next: `npm run story:finish -- feature/f20-a-diet`.\n\nCommits: {head}"
        )
    );

    // A read-only agent, an agent without a worktree: nothing.
    assert_eq!(hook(&root, "auditor", &[prompt], "r"), "");
    assert_eq!(hook(&root, "general-purpose", &["{}".to_owned()], "r"), "");
    fs::remove_dir_all(&root).expect("cleaned");
}

/// A fresh folder for one test.
fn scratch(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("check-handback-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("folder");
    root
}

#[test]
fn sets_up_a_project() {
    let root = scratch("set-up");

    project(&root);
    fs::remove_dir_all(&root).expect("cleaned");
}

#[test]
fn the_set_up_leaves_the_repository_of_git_dir_alone() {
    let root = scratch("decoy");
    let decoy = root.join("decoy");

    fs::create_dir_all(&decoy).expect("decoy");
    git(&decoy, &["init", "-q"]);
    let config = decoy.join(".git/config");
    let before = fs::read_to_string(&config).expect("config");

    // The set-up run as under the pre-push hook, `GIT_DIR` naming another repository.
    let run = Command::new(std::env::current_exe().expect("test binary"))
        .args(["--exact", "sets_up_a_project"])
        .env("GIT_DIR", decoy.join(".git"))
        .output()
        .expect("set-up runs");

    // The set-up ran, and passed, without touching the decoy.
    assert!(run.status.success());
    assert!(String::from_utf8_lossy(&run.stdout).contains("1 passed"));
    assert_eq!(fs::read_to_string(&config).expect("config"), before);
    fs::remove_dir_all(&root).expect("cleaned");
}

/// The decision of the hook's answer.
fn decision_of(output: &str) -> hooks::json::Value {
    parse(output)
        .expect("JSON")
        .get("hookSpecificOutput")
        .clone()
}

/// The prompt of a worker naming its worktree's folder.
fn prompt_naming(folder: &str) -> String {
    format!(
        r#"{{"type":"user","message":{{"content":"Worktree: `/x/.claude/worktrees/{folder}`"}}}}"#
    )
}

#[test]
fn the_hook_reads_the_story_from_the_branch_when_the_folder_is_named_otherwise() {
    let root = scratch("folder");
    let worktree = project_in(&root, "en-123-drop");
    let head = git(&worktree, &["rev-parse", "--short=7", "HEAD"]);

    git(&worktree, &["push", "-q", "-u", "origin", "HEAD"]);
    let through = decision_of(&hook(
        &root,
        "general-purpose",
        &[prompt_naming("en-123-drop")],
        &format!("Commits: {head}"),
    ));

    // The feature branch's own commit `start` is not the story's: nothing to name but `head`.
    assert_eq!(through.get("permissionDecision").text(), "allow");
    assert_eq!(
        through.get("updatedInput").get("message").text(),
        format!(
            "check-handback: EN-123 on `chore/en-123-a-topic` pushed at {head}; next: `npm run story:finish -- feature/f20-a-diet`.\n\nCommits: {head}"
        )
    );
    fs::remove_dir_all(&root).expect("cleaned");
}

#[test]
fn the_hook_checks_no_commit_naming_without_a_base() {
    let root = scratch("no-base");
    let worktree = project(&root);
    let head = git(&worktree, &["rev-parse", "--short=7", "HEAD"]);

    git(&worktree, &["push", "-q", "-u", "origin", "HEAD"]);
    fs::remove_file(root.join("en-123")).expect("story removed from the records");
    let through = decision_of(&hook(
        &root,
        "general-purpose",
        &[prompt_naming("en-123")],
        "Names no commit.",
    ));

    // No story in the records, so no base: the push alone is checked, never the whole history.
    assert_eq!(through.get("permissionDecision").text(), "allow");
    assert_eq!(
        through.get("updatedInput").get("message").text(),
        format!(
            "check-handback: EN-123 on `chore/en-123-a-topic` pushed at {head}; next: `npm run story:finish`.\n\nNames no commit."
        )
    );
    fs::remove_dir_all(&root).expect("cleaned");
}

#[test]
fn f31_ac1_reads_a_worker_s_worktree_from_the_brief_s_cd() {
    // The brief's line, as the transcript's JSON escapes it, after another worktree named first
    // (AUD-033): the first such line names the worker's worktree.
    let transcript = r#"{"content":"Unlike `/r/.claude/worktrees/en-2`, this one.\nEach call starts with `cd \"/r/SVG editor/.claude/worktrees/en-1\" || exit 9`, never `cd \"/r/.claude/worktrees/en-3\" || exit 9`."}"#;

    assert_eq!(prompt_worktree(transcript), Some("en-1"));
    assert_eq!(
        prompt_worktree("see .claude/worktrees/en-2\ncd \"/r/.claude/worktrees/en-1\" || exit 9"),
        Some("en-1")
    );
    // No brief's line, or a `cd` without its `|| exit`: the first worktree named.
    assert_eq!(
        prompt_worktree("Worktree: `/r/.claude/worktrees/en-2`."),
        Some("en-2")
    );
    assert_eq!(
        prompt_worktree("/r/.claude/worktrees/en-2, cd \"/r/.claude/worktrees/en-1\" && ls"),
        Some("en-2")
    );
    assert_eq!(prompt_worktree("cd \"/r/other\" || exit 9"), None);
    assert_eq!(
        prompt_worktree(
            "cd \"/r/.claude/worktrees/en-2\" || true; cd \"/r/.claude/worktrees/en-1\" || exit 9"
        ),
        Some("en-1")
    );
    assert_eq!(prompt_worktree("nothing"), None);
}
