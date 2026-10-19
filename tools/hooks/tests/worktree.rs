//! Tests of the guard read by words in a story's worktree (EN-119): read-only Git chains after the
//! brief's `cd`, quotes and here-documents as the shell reads them, the heavy lock taken by hand, a
//! Python script writing a file, and a worker's write into the main checkout.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Output, Stdio};

use hooks::guard::{
    HAND_LOCK, PYTHON_WRITE, denied_reason, heredoc_bodies, is_read_only_git,
    is_read_only_git_chain, python_writes, shell_reason, worktree_reason,
};
use hooks::json::quote;
use hooks::pages::page_of_command;
use hooks::written::misplaced_write;

/// A story's worktree, as a worker's Bash calls see it.
const WORKTREE: &str = "/work/SVG editor/.claude/worktrees/en-119";

/// The main checkout holding it, where a worker's calls start.
const MAIN: &str = "/work/SVG editor";

/// The start of a refusal of chained Git commands.
const CHAINED: &str = "A worktree refuses a chained Git command";

/// The guard's answer to a command typed after the brief's `cd`, from the main checkout.
fn after_the_brief(rest: &str) -> Option<String> {
    let branch = || "chore/en-119-the-guard-reads-by-words".to_owned();

    shell_reason(
        &format!("cd \"{WORKTREE}\" || exit 9; {rest}"),
        MAIN,
        &branch,
    )
}

fn owned(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| (*word).to_owned()).collect()
}

#[test]
fn f31_ac1_lets_a_chain_of_read_only_git_commands_through_after_the_brief_s_cd() {
    // The reads the transcripts chain most (I93), and the card's `git log`, `git diff --stat`.
    for rest in [
        "git status; git log --oneline -3",
        "git status --short && git diff --stat",
        "git log -1\ngit show --stat HEAD",
        "git status; git diff | head -20; git log --oneline origin/x..HEAD",
        "git branch --show-current; git rev-parse HEAD",
        "git diff --stat HEAD || git status",
        // A `|` inside quotes is no pipe (I93): `grep -E "a|b"`.
        "git grep -n -E \"a|b\" -- src; git status",
        "git log --format='%h|%s' -3; git status",
        "git status 2>&1; git log -2 2>&1 | tail -5",
    ] {
        assert_eq!(after_the_brief(rest), None, "{rest}");
    }
    // A Git command that writes, or any other command, in the chain: refused as before.
    for rest in [
        "git status; git commit -m x",
        "git add a && git status",
        "git log; git push",
        "git status; ls",
        "git branch new; git status",
        "git diff --output=x.patch; git status",
        "git log --ext-diff; git status",
        "git grep -O x; git status",
        "git log > out.txt; git status",
        "git -c core.pager=x log; git status",
        "git -C .. log; git status",
        "git log $(rm x); git status",
        "git status & git log",
        "GIT_DIR=x git log; git status",
    ] {
        assert!(
            after_the_brief(rest).is_some_and(|reason| reason.starts_with(CHAINED)),
            "{rest}: {:?}",
            after_the_brief(rest)
        );
    }
    // In the worktree's own folder with no `cd`, Claude Code's worktree isolation refuses a chain:
    // the guard answers it first, as before.
    assert!(worktree_reason("git status; git log", WORKTREE).is_some());
}

#[test]
fn f31_ac1_tells_a_git_command_that_only_reads() {
    for words in [
        "git status --short",
        "git log --oneline -3 origin/x..HEAD",
        "git diff --stat HEAD",
        "git show HEAD:docs/a.md",
        "git branch --show-current",
        "git branch -a -vv",
        "git branch --list",
        "git rev-parse --abbrev-ref HEAD",
        "git ls-files docs",
    ] {
        assert!(
            is_read_only_git(&owned(&words.split(' ').collect::<Vec<_>>())),
            "{words}"
        );
    }
    for words in [
        "git",
        "git commit -m x",
        "git push",
        "git fetch",
        "git stash",
        "git branch x",
        "git branch -d x",
        "git branch -m x y",
        "git -C .. status",
        "git -c a=b log",
        "git diff --output=x",
        "git log --output x",
        "git diff --ext-diff",
        "git grep --open-files-in-pager=vi x",
        "git grep -Ovi x",
        "gitk log",
        "ls -la",
    ] {
        assert!(
            !is_read_only_git(&owned(&words.split(' ').collect::<Vec<_>>())),
            "{words}"
        );
    }
    assert!(is_read_only_git_chain("git status; git log | head -3"));
    assert!(is_read_only_git_chain("git push | tail -2"));
    assert!(!is_read_only_git_chain("git push; git status"));
    assert!(!is_read_only_git_chain(""));
    assert!(!is_read_only_git_chain("ls; git status"));
}

#[test]
fn f31_ac1_reads_quotes_and_here_documents_as_the_shell_does() {
    for rest in [
        // A quoted message: its `;`, `|` and `&&` are text.
        "git commit -m \"fix(tooling): a; b | c && d\"",
        // A message of several lines (I40).
        "git commit -m \"feat(x): a\n\nA body line; with | a pipe.\n\nRefs: EN-1.T1\"",
        // A URL in it, its `&` and `#` included (I87).
        "git commit -m \"docs: cite https://example.com/a?b=1&c=2#d\"",
        // A here-document holding Git commands: data.
        "target/release/git-sequences commit -F - --push a.md <<'EOF'\nchore(tooling): x\n\ngit push && git log; git status\nEOF",
        // Its word quoted with a hyphen, or a substitution reading it.
        "target/release/git-sequences commit -F - a.md <<'END-OF-MESSAGE'\nchore(tooling): x\n\ngit push; git log\nEND-OF-MESSAGE",
        "git commit -m \"$(cat <<'EOF'\nfix: x\n\ngit push && git log\nEOF\n)\"",
    ] {
        assert_eq!(after_the_brief(rest), None, "{rest}");
    }
    // The library's hook reads a URL in a commit message as no page to fetch (I87).
    assert_eq!(
        page_of_command(
            "git commit -m \"docs: cite https://curl.se/docs, read by curl https://a.b/c\""
        ),
        None
    );
    assert_eq!(
        heredoc_bodies("cat <<'EOF'\na\nb\nEOF\nls <<-X\n\tc\n\tX"),
        ["a\nb", "\tc"]
    );
    // A word with a hyphen or a dot, quoted or not; `<<<` is a here-string, no here-document.
    assert_eq!(heredoc_bodies("cat <<'A-B'\nx\nA-B"), ["x"]);
    assert_eq!(heredoc_bodies("cat <<A-B.c\nx\nA-B.c"), ["x"]);
    assert_eq!(heredoc_bodies("cat <<<A\nx\nA"), [] as [&str; 0]);
    assert_eq!(heredoc_bodies("echo '<<EOF'"), [] as [&str; 0]);
    // After the brief's `cd`, a pipe or a background job is no separator it allows.
    for rest in ["| git log | head", "& git log | head"] {
        let line = format!("cd \"{WORKTREE}\" || exit 9 {rest}");
        let branch = String::new;

        assert!(shell_reason(&line, MAIN, &branch).is_some(), "{rest}");
    }
}

#[test]
fn f31_ac1_refuses_a_heavy_lock_taken_by_hand_naming_the_command_to_run() {
    for command in [
        "mkdir .git/heavy.lock",
        "mkdir -p \"/work/SVG editor/.git/heavy.lock\" && git push",
        "until mkdir ../../.git/heavy.lock; do sleep 5; done",
        "bash -c 'mkdir .git/heavy.lock'",
        "mkdir \"$(git rev-parse --git-common-dir)/heavy.lock\"",
        "mkdir x .git/heavy.lock/",
    ] {
        assert_eq!(denied_reason(command), Some(HAND_LOCK), "{command}");
    }
    assert!(HAND_LOCK.contains("run `git push`, `npm run check:all`"));
    for command in [
        "mkdir heavy",
        "mkdir .git/heavy.lock.d",
        "ls .git/heavy.lock",
        "rmdir .git/heavy.lock",
        "echo mkdir .git/heavy.lock",
        "cat .git/heavy.lock/pid",
    ] {
        assert_eq!(denied_reason(command), None, "{command}");
    }
}

#[test]
fn f31_ac1_refuses_a_python_script_writing_a_file_in_a_worktree() {
    for command in [
        "python3 -c \"open('a.md', 'w').write('x')\"",
        "python3 - <<'EOF'\nfrom pathlib import Path\nPath('a.md').write_text('x')\nEOF",
        "python3 <<'EOF'\nwith open(p, mode=\"a\") as f:\n    f.write(x)\nEOF",
        "python3 -c 'print(1)' > out.txt",
        "python3 -c 'print(1)' >> out.txt",
        "/usr/bin/python3.12 -c \"import os; os.replace('a', 'b')\"",
        "python -c \"import shutil; shutil.copy('a', 'b')\"",
        "python3 <<< \"open('x', 'wb')\"",
    ] {
        assert_eq!(
            worktree_reason(command, WORKTREE).as_deref(),
            Some(PYTHON_WRITE),
            "{command}"
        );
        // Outside a worktree, nothing is refused.
        assert_eq!(worktree_reason(command, MAIN), None, "{command}");
    }
    assert_eq!(
        after_the_brief("python3 -c \"import shutil; shutil.move('a', 'b')\"").as_deref(),
        Some(PYTHON_WRITE)
    );
    assert!(PYTHON_WRITE.contains("`target/release/replace`"));
    for command in [
        "python3 -c \"import json, sys; print(json.load(sys.stdin)['a'])\" < x.json",
        "python3 -c \"print(open('a').read())\"",
        "python3 -c \"print(open('a', 'r', encoding='utf-8').read())\"",
        "python3 -c 'print(1)' 2>&1",
        "python3 -c 'print(1)' > /dev/null",
        "python3 --version",
        // A script file is not read.
        "python3 scratch/sum.py",
        // With `-c`, a here-document is the program's input, not its text.
        "python3 -c 'import sys; print(sys.stdin.read())' <<'EOF'\nf.write(x)\nEOF",
        "git commit -m \"fix: open('a', 'w')\"",
    ] {
        assert_eq!(worktree_reason(command, WORKTREE), None, "{command}");
    }
}

#[test]
fn f31_ac1_tells_a_python_text_that_writes_a_file() {
    for code in [
        "f.write(x)",
        "f.writelines(x)",
        "p.write_text(x)",
        "p.write_bytes(x)",
        "p.touch()",
        "p.rename(q)",
        "os.replace(a, b)",
        "os.remove(a)",
        "p.unlink()",
        "p.mkdir()",
        "os.makedirs(a)",
        "json.dump(x, f)",
        "shutil.rmtree(a)",
        "fileinput.input(a, inplace=True)",
        "open(a, 'w')",
        "open(a, \"a\")",
        "open(a, 'x')",
        "open(a, 'r+')",
        "open(a, mode='wb', encoding='utf-8')",
        "open(join(a, 'b'), 'w')",
    ] {
        assert!(python_writes(code), "{code}");
    }
    for code in [
        "",
        "print(open(a).read())",
        "open(a, 'r')",
        "open(a, 'rb')",
        "open(a, encoding='utf-8')",
        "open(a, newline='')",
        "text.replace('a', 'b')",
        "json.dumps(x)",
        "sys.stdout.flush()",
    ] {
        assert!(!python_writes(code), "{code}");
    }
}

#[test]
fn f31_ac1_refuses_a_worker_s_write_into_the_main_checkout_naming_its_worktree_s_path() {
    let main = misplaced_write("/p/docs/a.md", "/p", "en-1");

    assert!(
        main.as_deref()
            .is_some_and(|reason| reason.contains("`/p/.claude/worktrees/en-1/docs/a.md`")),
        "{main:?}"
    );
    // Another worktree's file: the same file in its own.
    assert!(
        misplaced_write("/p/.claude/worktrees/en-2/docs/a.md", "/p", "en-1")
            .is_some_and(|reason| reason.contains("`/p/.claude/worktrees/en-1/docs/a.md`"))
    );
    // A path that leaves the worktree by `..` is judged where it lands.
    assert!(
        misplaced_write("/p/.claude/worktrees/en-1/../../../docs/a.md", "/p", "en-1")
            .is_some_and(|reason| reason.contains("`/p/.claude/worktrees/en-1/docs/a.md`"))
    );
    for file in [
        "/p/.claude/worktrees/en-1/docs/a.md",
        "/p/.claude/worktrees/en-1",
        "/tmp/scratch/a.md",
        "/p2/a.md",
        "/p/.git/metrics/step",
        // A relative path is not the tools' and is not read against `/`.
        "docs/a.md",
        "p/docs/a.md",
    ] {
        assert_eq!(misplaced_write(file, "/p", "en-1"), None, "{file}");
    }
}

/// A session of one worker: its project with its worktree, and the transcripts Claude Code keeps.
struct Session {
    root: PathBuf,
    project: PathBuf,
    transcript: PathBuf,
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// A session whose worker `a1` was started by a prompt naming the worktree `en-1`.
fn session(name: &str) -> Session {
    let root = std::env::temp_dir().join(format!("guard-write-{name}-{}", std::process::id()));
    let project = root.join("project");
    let transcript = root.join("session.jsonl");
    let agents = root.join("session/subagents");

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(project.join(".claude/worktrees/en-1")).expect("worktree");
    fs::create_dir_all(&agents).expect("agents");
    fs::write(&transcript, "").expect("transcript");
    // `a1`'s worktree is on disk, `a3`'s was removed.
    for (agent, worktree) in [("a1", "en-1"), ("a3", "en-9")] {
        let prompt = format!(
            "Story {worktree}. Worktree: `{}/.claude/worktrees/{worktree}`.",
            project.display()
        );

        fs::write(
            agents.join(format!("agent-{agent}.jsonl")),
            format!(
                "{{\"type\":\"user\",\"message\":{{\"role\":\"user\",\"content\":{}}}}}\n",
                quote(&prompt)
            ),
        )
        .expect("agent transcript");
    }
    Session {
        root,
        project,
        transcript,
    }
}

/// The hook run on a tool call of an agent, its JSON members after `tool_name`.
fn hook(session: &Session, members: &str) -> Output {
    let mut child = test_support::command(env!("CARGO_BIN_EXE_guard-bash"))
        .env("CLAUDE_PROJECT_DIR", &session.project)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("guard-bash runs");
    let input = format!(
        "{{\"cwd\":{},\"transcript_path\":{},{members}}}",
        quote(&session.project.to_string_lossy()),
        quote(&session.transcript.to_string_lossy())
    );

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input written");
    child.wait_with_output().expect("guard-bash ends")
}

/// The members of a `Write` or `Edit` of `file`.
fn write_call(tool: &str, agent: &str, file: &Path) -> String {
    format!(
        "\"tool_name\":\"{tool}\",{agent}\"tool_input\":{{\"file_path\":{},\"content\":\"x\"}}",
        quote(&file.to_string_lossy())
    )
}

#[test]
fn f31_ac1_the_hook_refuses_a_worker_s_write_into_the_main_checkout() {
    let session = session("refused");
    let worker = "\"agent_id\":\"a1\",\"agent_type\":\"story-worker\",";
    let main_file = session.project.join("docs/a.md");
    let worktree_file = session.project.join(".claude/worktrees/en-1/docs/a.md");

    for tool in ["Write", "Edit"] {
        let output = hook(&session, &write_call(tool, worker, &main_file));
        let text = String::from_utf8_lossy(&output.stdout);

        assert!(output.status.success());
        assert!(
            text.contains("\"permissionDecision\":\"deny\""),
            "{tool}: {text}"
        );
        assert!(
            text.contains(&worktree_file.to_string_lossy().into_owned()),
            "{tool}: {text}"
        );
        // Its own worktree's file goes through.
        let own = hook(&session, &write_call(tool, worker, &worktree_file));

        assert_eq!(own.stdout, [] as [u8; 0]);
    }
    // A notebook, by its own key.
    let notebook = format!(
        "\"tool_name\":\"NotebookEdit\",{worker}\"tool_input\":{{\"notebook_path\":{}}}",
        quote(&main_file.to_string_lossy())
    );

    assert!(String::from_utf8_lossy(&hook(&session, &notebook).stdout).contains("deny"));
    // The main thread, an agent that only reads, a worker without transcript, or one whose
    // prompt's worktree is gone.
    for agent in [
        "",
        "\"agent_id\":\"a1\",\"agent_type\":\"Explore\",",
        "\"agent_id\":\"a2\",\"agent_type\":\"story-worker\",",
        "\"agent_id\":\"a3\",\"agent_type\":\"story-worker\",",
    ] {
        let output = hook(&session, &write_call("Write", agent, &main_file));

        assert!(output.status.success());
        assert_eq!(output.stdout, [] as [u8; 0], "{agent}");
    }
}
