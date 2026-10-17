//! Tests of the hand-back check and of the `check-handback` hook (EN-088).

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use hooks::handback::{
    Branch, Commit, Refs, agent_transcript, allow_output, earlier_handbacks, feature_number,
    finish_line, is_checked, missing, read_refs, refusal, story_id, worktree_id,
};
use hooks::json::{parse, quote, write};

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
        "verifier",
        "doc-editor",
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
fn names_the_story_and_its_feature() {
    assert_eq!(story_id("en-088"), "EN-088");
    assert_eq!(
        feature_number("E17-F20-EN-088-a-hook.md", "EN-088"),
        Some("f20".to_owned())
    );
    assert_eq!(feature_number("E17-F20-EN-089-other.md", "EN-088"), None);
    assert_eq!(feature_number("README.md", "EN-088"), None);
}

#[test]
fn reads_the_branch_its_copy_and_its_feature_from_one_listing() {
    let listing = "  1111 refs/heads/feature/f20-a-diet\n\
                   * 2222 refs/heads/chore/en-123-a-topic\n\
                   \u{20} 3333 refs/remotes/origin/chore/en-123-a-topic\n\
                   \u{20} 4444 refs/remotes/origin/feature/f2-other\n\
                   \u{20} 1111 refs/remotes/origin/feature/f20-a-diet";
    let refs = |number| read_refs(listing, number);

    assert_eq!(
        refs(Some("f20")),
        Some(Refs {
            name: "chore/en-123-a-topic".into(),
            head: "2222".into(),
            remote: Some("3333".into()),
            feature: Some("feature/f20-a-diet".into()),
        })
    );
    assert_eq!(refs(None).and_then(|refs| refs.feature), None);
    // Never pushed; a detached head names no branch.
    assert_eq!(
        read_refs("* 2222 refs/heads/b", None).and_then(|refs| refs.remote),
        None
    );
    assert_eq!(read_refs("  2222 refs/heads/b", None), None);
}

#[test]
fn counts_the_hand_backs_before_this_one() {
    let call = |id: &str| {
        format!(
            r#"{{"type":"assistant","message":{{"content":[{{"type":"tool_use","id":"{id}","name":"SubagentHandback","input":{{"message":"r"}}}}]}}}}"#
        )
    };
    let lines = [call("t1"), call("t2"), "{".to_owned()].join("\n");

    // The current call t2 already written: one earlier; not written yet: two.
    assert_eq!(earlier_handbacks(&lines, "t2"), 1);
    assert_eq!(earlier_handbacks(&lines, "t3"), 2);
    assert_eq!(earlier_handbacks("", "t1"), 0);
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
    assert_eq!(
        decision.get("updatedInput").get("message").text(),
        "Line.\n\nReport."
    );
    assert_eq!(write(decision.get("updatedInput").get("n")), "1");
}

#[test]
fn writes_json_back() {
    let text = r#"{"a":[1,2.5,true,null],"b":"x\"y","c":{}}"#;

    assert_eq!(write(&parse(text).expect("JSON")), text);
    assert_eq!(quote("é"), "\"é\"");
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself, and a set-up inheriting it rewrote its config.
fn clean(program: &str) -> Command {
    let mut command = Command::new(program);

    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command
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
    let remote = root.join("remote.git");
    let worktree = root.join("project/.claude/worktrees/en-123");

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
    fs::create_dir_all(worktree.join("docs/backlog/stories")).expect("stories");
    fs::write(
        worktree.join("docs/backlog/stories/E17-F20-EN-123-a-topic.md"),
        "",
    )
    .expect("story");
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
    let mut child = clean(env!("CARGO_BIN_EXE_check-handback"))
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
fn the_hook_refuses_once_then_lets_through_and_names_story_finish() {
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
    let _ = Command::new(std::env::current_exe().expect("test binary"))
        .args(["--exact", "sets_up_a_project", "--quiet"])
        .env("GIT_DIR", decoy.join(".git"))
        .output()
        .expect("set-up runs");

    assert_eq!(fs::read_to_string(&config).expect("config"), before);
    fs::remove_dir_all(&root).expect("cleaned");
}
