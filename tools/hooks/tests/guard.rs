//! Tests of the shell guard and of the `guard-bash` hook (EN-104), the cases of
//! `.claude/hooks/guard-bash.test.ts` carried over, then the folder a leading `cd` enters, the
//! brief's form and a script missing from `package.json`.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use hooks::guard::{
    denied_reason, entered, is_full_vitest_run, later_folders, lexical_join, missing_script,
    missing_script_reason, npm_script, scripts_of, segments, without_heredocs, worktree_reason,
    worktree_root,
};
use hooks::json::quote;

/// A story's worktree, as a worker's Bash calls see it.
const WORKTREE: &str = "/work/SVG editor/.claude/worktrees/us-061";

/// The main checkout holding it.
const MAIN: &str = "/work/SVG editor";

#[test]
fn denies_what_skips_a_check_or_loses_work() {
    for command in [
        "gh pr merge 21 --rebase",
        "cd repo && gh pr merge 21",
        "git status; gh pr merge 21",
        "git status\ngh pr merge 21",
        "false || gh pr merge 21",
        "git commit --no-verify -m \"x\"",
        "git push --no-verify origin main",
        "git push --force origin feat/x",
        "git push -f",
        "git push origin +feat/x",
        "git -C \"/work/SVG editor\" push --force",
        "git -c user.name=x commit --no-verify",
        "git commit -nm \"x\"",
        "git commit -n -m \"x\"",
        "HUSKY=0 git commit -m \"x\"",
        "export HUSKY=0",
        "git config core.hooksPath /dev/null",
        "git -c core.hooksPath=/dev/null commit -m x",
        "CI=1 HUSKY=0 git push",
        "cat > a.txt <<'EOF'\ntext\nEOF\ngh pr merge 21",
        "npx vitest run",
        "npx vitest run --coverage",
        "cd repo && npx vitest run --reporter=dot",
        "npx vitest",
        // Every test run all the same (AUD-020): the current folder, a project, the binary itself.
        "npx vitest run .",
        "npx vitest run ./ --coverage",
        "npx vitest run --project unit",
        "npx vitest --reporter dot run",
        "node_modules/.bin/vitest run",
        "./node_modules/.bin/vitest run --project x",
    ] {
        assert!(denied_reason(command).is_some(), "{command}");
    }
}

#[test]
fn allows_the_commands_that_only_look_alike() {
    for command in [
        "gh pr view 21",
        "gh pr create --fill",
        "git push --force-with-lease origin feat/x",
        "git push origin main",
        "git -C \"/work/SVG editor\" push -u origin feat/x",
        "git commit -m \"x\" --amend",
        "git log -n 3",
        "sed -i '' 's/a/denies HUSKY=0 and core.hooksPath/' docs/tooling/agent.md",
        "git commit -m \"docs: explain why gh pr merge is denied\"",
        "npm run check:all",
        "npx vitest run scripts/check-all.test.ts",
        "npx vitest run src/geometry --coverage",
        "npx vitest run --project unit scripts/story-finish.test.ts",
        "node_modules/.bin/vitest run .claude/hooks/guard-bash.test.ts",
        "npx vitest-preview",
        "npm run test",
        "cat > note.md <<'EOF'\nThe agent never runs gh pr merge.\nEOF",
        "git commit -F - <<\"MSG\"\ndocs: x\n\ngit push --force is denied\nMSG\ngit log -1",
    ] {
        assert_eq!(denied_reason(command), None, "{command}");
    }
}

#[test]
fn names_each_reason() {
    assert!(denied_reason("gh pr merge 1").is_some_and(|reason| reason.contains("Product Owner")));
    assert!(
        denied_reason("git push -f").is_some_and(|reason| reason.contains("--force-with-lease"))
    );
    assert!(
        denied_reason("HUSKY=0 git push").is_some_and(|reason| reason.contains("switched off"))
    );
    assert!(denied_reason("git commit -n").is_some_and(|reason| reason.contains("bypassed")));
    assert!(denied_reason("npx vitest").is_some_and(|reason| reason.contains("heavy lock")));
}

#[test]
fn matches_as_the_regular_expressions_did_on_their_edges() {
    // `-n` among other short options, and not a value: `-m n` is no `-n`.
    assert!(denied_reason("git commit -am x -vn").is_some());
    assert_eq!(denied_reason("git commit -m n"), None);
    // `--no-verify` as a word only, and `-f` as a word only.
    assert_eq!(denied_reason("git push --no-verifyx"), None);
    assert_eq!(denied_reason("git push -fu origin x"), None);
    // A quoted `-C` path holding a space, and the same path read as words.
    assert!(denied_reason("git -C 'a b' --no-pager push +x").is_some());
    assert!(denied_reason("git --git-dir=.git push --force").is_some());
    // `HUSKY=0` after settings and `env`, never inside another word nor a value.
    assert!(denied_reason("env A=1 B= HUSKY=0 npm test").is_some());
    assert_eq!(denied_reason("echo HUSKY=0"), None);
    assert_eq!(denied_reason("HUSKY=01 git push"), None);
    // `core.hooksPath` after `config` anywhere later, or after `-c`; not as `core.hooksPathX`.
    assert!(denied_reason("git --no-pager config --local core.hooksPath x").is_some());
    assert_eq!(denied_reason("git config core.hooksPathX x"), None);
    assert_eq!(denied_reason("gitx config core.hooksPath x"), None);
    // `gh pr merge` as words: `gh pr mergeable` is not it.
    assert_eq!(denied_reason("gh pr mergeable"), None);
    // A here-document whose word is followed by more on its line, and an unclosed one.
    assert_eq!(denied_reason("cat <<EOF > a\ngh pr merge\nEOF"), None);
    assert!(denied_reason("cat <<EOF\ngh pr merge").is_some());
}

#[test]
fn removes_the_bodies_of_here_documents_only() {
    // The body and its last line go; the opening line and what follows stay.
    assert_eq!(
        without_heredocs("cat <<'EOF' > a\nbody\nEOF\nls"),
        "cat <<'EOF' > a\nls"
    );
    assert_eq!(without_heredocs("cat <<-X\n\tx\n\tX"), "cat <<-X");
    // `\w+` gives back characters: `<<EOFX` closes at a line `EOF`.
    assert_eq!(
        without_heredocs("cat <<EOFX\nbody\nEOF\nls"),
        "cat <<EOFX\nls"
    );
    // A here-string, and a body never closed, stay whole.
    assert_eq!(without_heredocs("cat <<< \"x\""), "cat <<< \"x\"");
    assert_eq!(without_heredocs("cat <<EOF\nbody"), "cat <<EOF\nbody");
    // An empty body stays, as the expression left it: it needs a line feed before its last word,
    // then one after the opening line. Two documents in a row both go.
    assert_eq!(without_heredocs("cat <<A\nA\nB"), "cat <<A\nA\nB");
    assert_eq!(without_heredocs("cat <<A\n\nA\nB"), "cat <<A\nB");
    assert_eq!(without_heredocs("a <<A\n1\nA\nb <<B\n2\nB"), "a <<A\nb <<B");
}

#[test]
fn splits_a_command_at_its_separators() {
    assert_eq!(
        segments("a && b || c; d | e\n f & g"),
        ["a", "b", "c", "d", "e", "f & g"]
    );
    assert_eq!(segments(""), [""]);
    assert_eq!(segments("a;"), ["a", ""]);
}

#[test]
fn tells_a_full_vitest_run_from_a_named_one() {
    assert!(is_full_vitest_run("npx  vitest"));
    assert!(is_full_vitest_run("npx vitest run -t name"));
    assert!(!is_full_vitest_run("npx vitest run -t name a.test.ts"));
    assert!(!is_full_vitest_run("npxvitest"));
    assert!(!is_full_vitest_run("npx vitestx"));
    assert!(!is_full_vitest_run("vitest run"));
}

#[test]
fn f18_ac8_denies_a_chained_git_command_in_a_worktree_with_the_form_that_works() {
    for command in [
        "cd \"/work/SVG editor/.claude/worktrees/us-061\" && git status",
        "git add -A && git commit -m x",
        "git fetch origin; git rebase origin/develop",
        "git log --oneline | head -5",
        "npm run fix && git add scripts",
    ] {
        let reason = worktree_reason(command, WORKTREE).unwrap_or_default();

        // A subagent's calls start in the main checkout: the hint enters the worktree first.
        assert!(
            reason.starts_with(
                "A worktree refuses a chained Git command: run `cd \"/work/SVG editor/.claude/worktrees/us-061\" || exit 9` then `git "
            ),
            "{command}: {reason}"
        );
        assert!(!reason.contains('\n'));
        // From a folder inside the worktree, the hint still names the worktree's root.
        assert_eq!(
            worktree_reason(command, &format!("{WORKTREE}/scripts")).as_deref(),
            Some(reason.as_str())
        );
    }
}

#[test]
fn allows_a_lone_git_command_in_a_worktree() {
    for command in [
        "git status",
        "git commit -F - <<'EOF'\nfix: x\n\ngit push && git log\nEOF",
        "npm run fix && npx tsc --noEmit",
        "ls docs; cat AGENTS.md",
    ] {
        assert_eq!(worktree_reason(command, WORKTREE), None, "{command}");
        assert_eq!(
            worktree_reason(command, &format!("{WORKTREE}/scripts")),
            None
        );
    }
}

#[test]
fn names_the_rust_command_of_a_frequent_git_sequence() {
    let commit = worktree_reason("git add -A && git commit -m x", WORKTREE).unwrap_or_default();
    let sync = worktree_reason("git fetch origin; git rebase origin/develop", WORKTREE)
        .unwrap_or_default();

    assert!(commit.ends_with(" Or in one call: `npm run git:commit -- -F <message file> [--push] <paths>` stages, commits and pushes."));
    assert!(sync.ends_with(
        " Or in one call: `npm run git:sync` fetches and rebases the story on its feature branch."
    ));
    assert!(
        !worktree_reason("git log --oneline | head -5", WORKTREE)
            .unwrap_or_default()
            .contains("one call")
    );
    // Both sequences named, in their order.
    assert!(
        worktree_reason("git pull; git push", WORKTREE)
            .unwrap_or_default()
            .contains("pushes; `npm run git:sync`")
    );
}

#[test]
fn allows_the_rust_command_of_a_git_sequence_in_a_worktree() {
    for command in [
        "cd \"/work/SVG editor/.claude/worktrees/us-061\" || exit 9; npm run git:sync",
        "cd \"/work/SVG editor/.claude/worktrees/us-061\" || exit 9\nnpm run git:commit -- -F - --push a.md <<'EOF'\nchore(tooling): x\n\nRefs: EN-100.T1\nEOF",
    ] {
        assert_eq!(denied_reason(command), None);
        assert_eq!(worktree_reason(command, WORKTREE), None);
    }
}

#[test]
fn allows_a_chained_git_command_outside_a_worktree() {
    assert_eq!(worktree_reason("git add -A && git commit -m x", MAIN), None);
    assert_eq!(
        worktree_reason("git status && git log", "/work/.claude/worktrees-old"),
        None
    );
}

#[test]
fn f18_ac8_refuses_golden_write_in_a_worktree_and_says_why() {
    assert!(
        worktree_reason("npm run golden:write", WORKTREE)
            .is_some_and(|reason| reason.contains("shared by every worktree"))
    );
    assert_eq!(worktree_reason("npm run golden:write", MAIN), None);
    assert_eq!(worktree_reason("npm run golden:check", WORKTREE), None);
}

#[test]
fn finds_the_root_of_the_last_worktree_of_a_path() {
    assert_eq!(worktree_root(&format!("{WORKTREE}/a/b")), Some(WORKTREE));
    assert_eq!(
        worktree_root("/a/.claude/worktrees/x/.claude/worktrees/y"),
        Some("/a/.claude/worktrees/x/.claude/worktrees/y")
    );
    // A last one without its name falls back to the one before.
    assert_eq!(
        worktree_root("/a/.claude/worktrees/x/.claude/worktrees/"),
        Some("/a/.claude/worktrees/x")
    );
    assert_eq!(worktree_root("/a/.claude/worktrees/"), None);
    assert_eq!(worktree_root(MAIN), None);
}

#[test]
fn f21_ac1_judges_the_command_in_the_folder_a_leading_cd_enters() {
    // From the main checkout, the command enters the worktree: judged there.
    let quoted = entered(&format!("cd \"{WORKTREE}\" && git status && git log"), MAIN);

    assert_eq!(quoted.folder, WORKTREE);
    assert!(worktree_reason(&quoted.command, &quoted.folder).is_some());
    // Unquoted, single-quoted, relative, with `..`: resolved against the session's folder.
    assert_eq!(
        entered("cd .claude/worktrees/us-061 && ls", MAIN).folder,
        WORKTREE
    );
    assert_eq!(
        entered(&format!("  cd '{WORKTREE}'; ls"), MAIN).folder,
        WORKTREE
    );
    assert_eq!(
        entered("cd ../scripts && ls", &format!("{WORKTREE}/x")).folder,
        format!("{WORKTREE}/scripts")
    );
    assert_eq!(entered("cd .. && ls", "/a/b").folder, "/a");
    // From a worktree, a command entering the main checkout is judged there.
    let out = entered(
        &format!("cd \"{MAIN}\"; git worktree add x && git status"),
        WORKTREE,
    );

    assert_eq!(worktree_reason(&out.command, &out.folder), None);
    // A folder named by a variable or `~` cannot be read: the session's folder stays.
    assert_eq!(
        entered("cd \"$WT\" && git a && git b", WORKTREE).folder,
        WORKTREE
    );
    assert_eq!(entered("cd ~/x && ls", MAIN).folder, MAIN);
    // Not a `cd` of one folder: `cdx`, `cd` alone, an unclosed quote, a word glued to the quote.
    assert_eq!(entered("cdx /a && ls", MAIN).folder, MAIN);
    assert_eq!(entered("cd", MAIN).folder, MAIN);
    assert_eq!(entered("cd \"/a && ls", MAIN).folder, MAIN);
    assert_eq!(entered("cd \"/a\"b && ls", MAIN).folder, MAIN);
}

#[test]
fn f21_ac1_lets_the_brief_s_form_through_and_refuses_two_git_commands_after_it() {
    // The brief's form: `cd "<worktree>" || exit 9`, then one Git command (ADR-0072).
    for command in [
        format!("cd \"{WORKTREE}\" || exit 9; git status"),
        format!("cd \"{WORKTREE}\" || exit 9\ngit log --oneline -3"),
        format!("cd \"{WORKTREE}\" || exit 9 && git push -u origin HEAD"),
        format!("cd \"{WORKTREE}\" || exit; git status"),
        format!("cd \"{WORKTREE}\" || exit 9"),
    ] {
        let judged = entered(&command, MAIN);

        assert_eq!(
            worktree_reason(&judged.command, &judged.folder),
            None,
            "{command}"
        );
    }
    // Two Git commands chained after it: the same hint as in the worktree itself.
    let chained = format!("cd \"{WORKTREE}\" || exit 9; git add -A && git commit -m x");
    let judged = entered(&chained, MAIN);

    assert_eq!(
        worktree_reason(&judged.command, &judged.folder),
        worktree_reason("git add -A && git commit -m x", WORKTREE)
    );
    // `&&` after the `cd` is no guard: the `cd` stays a segment chained with the Git command.
    let plain = entered(&format!("cd \"{WORKTREE}\" && git status"), MAIN);

    assert!(worktree_reason(&plain.command, &plain.folder).is_some());
    // `|| exit 9` followed by more on its line is not the brief's form either.
    let odd = entered(&format!("cd \"{WORKTREE}\" || exit 9 x; git status"), MAIN);

    assert!(worktree_reason(&odd.command, &odd.folder).is_some());
}

#[test]
fn finds_a_worktree_entered_later_in_the_command() {
    let command = format!("cd /tmp && ls; cd \"{WORKTREE}\" && git fetch; git rebase x");

    assert_eq!(later_folders(&command, "/tmp"), [WORKTREE]);
    assert_eq!(
        // The first `cd` is the leading one, whose folder `entered` gives.
        later_folders("cd a; cd b; cd \"$X\"; cd ../c", "/r/a"),
        ["/r/a/b", "/r/a/c"]
    );
    assert_eq!(later_folders("ls; echo cd", "/r"), [] as [std::string::String; 0]);
}

#[test]
fn reads_the_script_an_npm_command_runs() {
    assert_eq!(npm_script("npm run -s git:sync"), Some("git:sync"));
    assert_eq!(npm_script("npm run --silent page -- REF-X"), Some("page"));
    assert_eq!(npm_script("npm -s run fix"), Some("fix"));
    assert_eq!(npm_script("npm  run-script  fix"), Some("fix"));
    // npm's own `env`, the list of scripts, another npm command, another program.
    assert_eq!(npm_script("npm run env"), None);
    assert_eq!(npm_script("npm run"), None);
    assert_eq!(npm_script("npm run -s"), None);
    assert_eq!(npm_script("npm test"), None);
    assert_eq!(npm_script("npx run x"), None);
    // A name holding quotes or a variable is code or text around `npm run`.
    assert_eq!(npm_script("npm run rust:build\","), None);
    assert_eq!(npm_script("npm run $s"), None);
}

#[test]
fn f21_ac1_finds_a_script_missing_from_the_package_json() {
    let scripts =
        scripts_of(r#"{"name":"x","scripts":{"fix":"a","git:sync":"b"}}"#).unwrap_or_default();

    assert_eq!(scripts, ["fix", "git:sync"]);
    assert_eq!(missing_script("npm run -s git:sync", &scripts), None);
    assert_eq!(
        missing_script("npm run fix && npm run -s git:commit -- -F m", &scripts).as_deref(),
        Some("git:commit")
    );
    assert_eq!(missing_script("ls; npm test", &scripts), None);
    // No scripts at all, and a package.json that is not JSON.
    assert_eq!(scripts_of(r#"{"name":"x"}"#), Some(Vec::new()));
    assert_eq!(scripts_of("{"), None);
    assert_eq!(
        missing_script_reason("git:sync", "chore/en-090-x", "origin/feature/f20-y"),
        "`npm run git:sync`: no script `git:sync` in the package.json of branch `chore/en-090-x`. A branch made before the script gets it by a rebase on its base: run `git fetch origin`, then `git rebase origin/feature/f20-y`, one call each."
    );
}

#[test]
fn joins_a_folder_without_reading_the_disk() {
    assert_eq!(lexical_join("/a/b", "c/./d"), "/a/b/c/d");
    assert_eq!(lexical_join("/a/b", "../../.."), "/");
    assert_eq!(lexical_join("/a", "/x/y"), "/x/y");
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself (`.claude/rules/tooling.md`). A local helper until
/// EN-103's shared test-support crate is merged.
fn clean(program: &str) -> Command {
    let mut command = Command::new(program);
    let home = std::env::temp_dir().join(format!("guard-bash-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("GIT_") {
            command.env_remove(name);
        }
    }
    command.env("HOME", home).env("GIT_CONFIG_NOSYSTEM", "1");
    command
}

/// Runs Git in a folder, panicking on a failure.
fn git(folder: &Path, arguments: &[&str]) {
    let status = clean("git")
        .args(arguments)
        .current_dir(folder)
        .status()
        .expect("git runs");

    assert!(status.success(), "git {arguments:?}");
}

/// The hook run on a command from a folder.
fn hook(command: &str, cwd: &Path) -> Output {
    let mut child = clean(env!("CARGO_BIN_EXE_guard-bash"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("guard-bash runs");
    let input = format!(
        "{{\"tool_name\":\"Bash\",\"cwd\":{},\"tool_input\":{{\"command\":{}}}}}",
        quote(&cwd.to_string_lossy()),
        quote(command)
    );

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("input written");
    child.wait_with_output().expect("guard-bash ends")
}

/// A repository on branch `chore/en-090-old`, made before the script `git:sync`.
fn old_branch(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("guard-bash-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("folder");
    fs::write(root.join("package.json"), r#"{"scripts":{"fix":"x"}}"#).expect("package.json");
    git(&root, &["init", "-q", "-b", "chore/en-090-old"]);
    root
}

#[test]
fn f21_ac1_refuses_a_script_missing_from_the_branch_naming_it_and_its_rebase() {
    let root = old_branch("missing");
    let output = hook(
        &format!("cd \"{}\" || exit 9; npm run -s git:sync", root.display()),
        Path::new("/"),
    );
    let text = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
    assert!(text.starts_with("{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\""), "{text}");
    assert!(text.contains("no script `git:sync`"), "{text}");
    assert!(text.contains("branch `chore/en-090-old`"), "{text}");
    assert!(
        text.contains("`git fetch origin`, then `git rebase origin/<base>`, one call each"),
        "{text}"
    );
    // A script the branch has, and a folder below the package's, go through.
    assert_eq!(hook("npm run fix", &root).stdout, [] as [u8; 0]);
    fs::create_dir_all(root.join("sub")).expect("sub");
    assert_eq!(hook("npm run -s fix", &root.join("sub")).stdout, [] as [u8; 0]);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn lets_through_what_it_cannot_judge() {
    // No package.json above the folder: npm's own message comes.
    let output = hook("npm run anything", Path::new("/"));

    assert!(output.status.success() && output.stdout.is_empty());
    // An input that is not JSON: exit 1, a hook error that lets the call through.
    let mut child = clean(env!("CARGO_BIN_EXE_guard-bash"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("guard-bash runs");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"not json")
        .expect("written");
    let output = child.wait_with_output().expect("ends");

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stdout, [] as [u8; 0]);
}

#[test]
fn prints_the_denial_as_the_typescript_hook_did() {
    let output = hook("gh pr merge 1", Path::new("/"));

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "{\"hookSpecificOutput\":{\"hookEventName\":\"PreToolUse\",\"permissionDecision\":\"deny\",\"permissionDecisionReason\":\"Story pull requests are merged by the Product Owner, never by the agent (AGENTS.md, Never).\"}}"
    );
    // A command enters a worktree from the main checkout: judged there.
    let worktree = std::env::temp_dir().join(format!(
        "guard-bash-wt-{}/.claude/worktrees/en-1",
        std::process::id()
    ));
    let output = hook(
        &format!(
            "cd \"{}\" || exit 9; git status; git log",
            worktree.display()
        ),
        Path::new("/"),
    );

    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("A worktree refuses a chained Git command")
    );
}
