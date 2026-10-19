//! Tests of the shell guard and of the `guard-bash` hook (EN-104), the cases of
//! `.claude/hooks/guard-bash.test.ts` carried over, then the folder a leading `cd` enters, the
//! brief's form and a script missing from `package.json`.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use hooks::guard::{
    Entered, FORCE_PUSH, HOOKS_OFF, MERGE, NO_VERIFY, PROTECTED_PUSH, UNKNOWN_PUSH, denied_reason,
    denied_reason_on, entered, entered_worktree_reason, is_full_vitest_run,
    is_git_read_through_filters, is_read_only_filter, later_folders, missing_script,
    missing_script_reason, npm_script, scripts_of, segments, shell_reason, without_heredocs,
    worktree_reason,
};
use hooks::json::quote;
use hooks::place::Repository;
use hooks::worktrees::worktree_root;

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
        "git config core.hooksPath /dev/null",
        "git -c core.hooksPath=/dev/null commit -m x",
        // Any change of the hooks' folder: set, unset, by the newer `git config set` (EN-113).
        "git config --unset core.hooksPath",
        "git config --local core.hooksPath .husky",
        "git config set core.hooksPath x",
        "git config --get core.hooksPath x; git config core.hooksPath y",
        // A command quoted for a shell is judged too.
        "sh -c \"git status; git push -f\"",
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
        "git -C \"/work/SVG editor\" push -u origin feat/x",
        "git commit -m \"x\" --amend",
        "git log -n 3",
        "sed -i '' 's/a/denies HUSKY=0 and core.hooksPath/' docs/tooling/agent.md",
        "git commit -m \"docs: explain why gh pr merge is denied\"",
        // husky is gone: `HUSKY=0` switches nothing off (EN-113).
        "HUSKY=0 git commit -m \"x\"",
        // Reading the hooks' folder changes nothing.
        "git config --get core.hooksPath",
        "git config core.hooksPath",
        "git config get core.hooksPath",
        "git config --local --get core.hooksPath 2>&1",
        "git config core.hooksPath 2>/dev/null",
        "git config --show-origin --type=path --get-all core.hooksPath",
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
        denied_reason("git config core.hooksPath x")
            .is_some_and(|reason| reason.contains("switched off") && reason.contains(".githooks/"))
    );
    // The messages name the Rust Git hooks, no longer the packages they replaced.
    for reason in [
        denied_reason("git commit -n"),
        denied_reason("git config core.hooksPath x"),
    ] {
        assert!(
            reason
                .is_some_and(|text| !text.contains("lint-staged") && !text.contains("commitlint"))
        );
    }
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
    // A quoted `-C` path holding a space, and the same path read as words.
    assert!(denied_reason("git -C 'a b' --no-pager push +x").is_some());
    assert!(denied_reason("git --git-dir=.git push --force").is_some());
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
    // The word is the shell's (AUD-033): `<<EOFX` closes at a line `EOFX` only, so a body whose
    // line never comes stays whole, as does a here-string.
    assert_eq!(
        without_heredocs("cat <<EOFX\nbody\nEOF\nls"),
        "cat <<EOFX\nbody\nEOF\nls"
    );
    assert_eq!(
        without_heredocs("cat <<E\"O\"F\nbody\nEOF\nls"),
        "cat <<E\"O\"F\nls"
    );
    assert_eq!(without_heredocs("cat <<< \"x\""), "cat <<< \"x\"");
    assert_eq!(without_heredocs("cat <<EOF\nbody"), "cat <<EOF\nbody");
    // No word, or one the shell expands: no document; an empty quoted word closes at an empty line.
    assert_eq!(without_heredocs("cat <<\nx\n\ny"), "cat <<\nx\n\ny");
    assert_eq!(without_heredocs("cat <<$W\nx\n$W"), "cat <<$W\nx\n$W");
    assert_eq!(without_heredocs("cat <<''\nx\n\ny"), "cat <<''\ny");
    // A word escaped or quoted with an escape inside, as the shell removes its quotes.
    assert_eq!(without_heredocs("cat <<\\EOF\nx\nEOF\ny"), "cat <<\\EOF\ny");
    assert_eq!(
        without_heredocs("cat <<\"E\\\"F\"\nx\nE\"F\ny"),
        "cat <<\"E\\\"F\"\ny"
    );
    // A `<<` in an array's index, a parameter or `$[…]` opens nothing; a backquote inside double
    // quotes is skipped.
    for line in [
        "a[1<<2]=x\nb\n2",
        "echo ${x<<y}\nb\ny",
        "echo $[1<<2]\nb\n2",
    ] {
        assert_eq!(without_heredocs(line), line);
    }
    assert_eq!(
        without_heredocs("x \"`a`$(cat <<A\n1\nA\n)\""),
        "x \"`a`$(cat <<A\n)\""
    );
    // An empty body goes with its line; two documents in a row, or on one line, all go.
    assert_eq!(without_heredocs("cat <<A\nA\nB"), "cat <<A\nB");
    assert_eq!(without_heredocs("cat <<A\n\nA\nB"), "cat <<A\nB");
    assert_eq!(without_heredocs("a <<A\n1\nA\nb <<B\n2\nB"), "a <<A\nb <<B");
    assert_eq!(without_heredocs("a <<A <<B\n1\nA\n2\nB\nc"), "a <<A <<B\nc");
    // The body starts after the line's quotes close, and a document opened in a `$(…)` closes in it.
    assert_eq!(
        without_heredocs("a <<A \"q\nr\"\n1\nA\nb"),
        "a <<A \"q\nr\"\nb"
    );
    assert_eq!(
        without_heredocs("x \"$(cat <<'A'\n1\nA\n)\""),
        "x \"$(cat <<'A'\n)\""
    );
}

#[test]
fn splits_a_command_at_its_separators() {
    assert_eq!(
        segments("a && b || c; d | e\n f & g"),
        ["a", "b", "c", "d", "e", "f", "g"]
    );
    // In `$'…'` a backslash escapes the quote (EN-119); in `'…'` it does not.
    assert_eq!(segments(r"echo $'a\'b; c' ; d"), [r"echo $'a\'b; c'", "d"]);
    assert_eq!(segments(r"echo 'a\'; b"), [r"echo 'a\'", "b"]);
    // A job sent to the background is a separator too (EN-119), never a redirection's `&`.
    assert_eq!(segments("a 2>&1 &>x >&2 & b"), ["a 2>&1 &>x >&2", "b"]);
    assert_eq!(segments(""), [""]);
    assert_eq!(segments("a;"), ["a", ""]);
}

#[test]
fn splits_nothing_inside_quotes_nor_at_an_escaped_separator() {
    // A `|` in a pattern, `&&` in a double-quoted text, an escaped `|` (EN-113).
    assert_eq!(
        segments("grep -E 'a|b' x; echo \"c && d\" | e\\|f"),
        ["grep -E 'a|b' x", "echo \"c && d\"", "e\\|f"]
    );
    // A double quote inside single quotes, an escaped quote inside double ones, a line feed
    // inside quotes.
    assert_eq!(
        segments("a '\"' | b \"\\\" ;\" ; c 'x\ny'"),
        ["a '\"'", "b \"\\\" ;\"", "c 'x\ny'"]
    );
    // An unclosed quote runs to the end.
    assert_eq!(segments("echo 'a; b"), ["echo 'a; b"]);
    // A backslash inside single quotes is a character: the quote after it closes them.
    assert_eq!(segments("echo 'a\\' ; b"), ["echo 'a\\'", "b"]);
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
        "git log --format='%h|%s' -3",
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

    assert!(commit.ends_with(" Or in one call: `target/release/git-sequences commit -F <message file> [--push] <paths>` stages, commits and pushes."));
    assert!(sync.ends_with(
        " Or in one call: `target/release/git-sequences sync` fetches and rebases the story on its feature branch."
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
            .contains("pushes; `target/release/git-sequences sync`")
    );
}

#[test]
fn allows_the_rust_command_of_a_git_sequence_in_a_worktree() {
    for command in [
        "cd \"/work/SVG editor/.claude/worktrees/us-061\" || exit 9; npm run git:sync",
        "cd \"/work/SVG editor/.claude/worktrees/us-061\" || exit 9; target/release/git-sequences sync",
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
    // The command of the golden file since `golden:write` went (EN-115), in its two spellings.
    for command in [
        "npx vitest run tests/golden-results.test.ts --mode golden-write",
        "npx vitest run tests/golden-results.test.ts --mode=golden-write",
        "cd x && npx vitest run --mode  golden-write tests/golden-results.test.ts",
    ] {
        assert!(
            worktree_reason(command, WORKTREE)
                .is_some_and(|reason| reason.contains("shared by every worktree")),
            "{command}"
        );
        assert_eq!(worktree_reason(command, MAIN), None, "{command}");
    }
    // Checking the file, another mode, or the mode's name inside a text, writes nothing.
    for command in [
        "npx vitest run tests/golden-results.test.ts --mode golden-check",
        "npm run golden:check",
        "npx vitest run --mode golden-writer",
        "echo golden-write",
    ] {
        assert_eq!(worktree_reason(command, WORKTREE), None, "{command}");
    }
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
    // Not a `cd` of one folder: `cdx`, `cd` alone, two folders, a substitution.
    assert_eq!(entered("cdx /a && ls", MAIN).folder, MAIN);
    assert_eq!(entered("cd", MAIN).folder, MAIN);
    assert_eq!(entered("cd /a /b && ls", MAIN).folder, MAIN);
    assert_eq!(entered("cd \"$(pwd)/x\" && ls", MAIN).folder, MAIN);
    // Its words read as the shell reads them (EN-119): a word glued to the quote is one word, an
    // unclosed quote runs to the end.
    assert_eq!(entered("cd \"/a\"b && ls", MAIN).folder, "/ab");
    assert_eq!(entered("cd \"/a && ls", MAIN).folder, "/a && ls");
    assert_eq!(
        entered("cd /work/SVG\\ editor/.claude/worktrees/us-061 && ls", MAIN).folder,
        WORKTREE
    );
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
    assert_eq!(
        later_folders("ls; echo cd", "/r"),
        [] as [std::string::String; 0]
    );
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
        "`npm run git:sync`: no script `git:sync` in the package.json of branch `chore/en-090-x`. A tool of the agent is called by its binary, `target/release/<binary>`, as docs/tooling/agent-tools.md lists them (EN-115). A branch made before a script gets it by a rebase on its base: run `git fetch origin`, then `git rebase origin/feature/f20-y`, one call each."
    );
}

/// A command without the `GIT_*` variables of the process running the tests: under the pre-push
/// hook, `GIT_DIR` names the repository itself (`.claude/rules/tooling.md`); started through the
/// shared `test_support::command` (EN-103), with a home of its own.
fn clean(program: &str) -> Command {
    let mut command = test_support::command(program);
    let home = std::env::temp_dir().join(format!("guard-bash-home-{}", std::process::id()));

    fs::create_dir_all(&home).expect("home");
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
    assert_eq!(
        hook("npm run -s fix", &root.join("sub")).stdout,
        [] as [u8; 0]
    );
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn lets_through_what_it_cannot_judge() {
    // No package.json above the folder: npm's own message comes.
    let output = hook("npm run anything", Path::new("/"));

    assert!(output.status.success() && output.stdout.is_empty());
    // An input that is not JSON: exit 1, a hook error, which `onFailure: block` makes block.
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
fn a_debug_build_of_a_tool_records_no_use_in_the_checkout_it_runs_in() {
    // `tool_use::run` records nothing in a debug build (EN-102): a test of a binary, run inside a
    // checkout, never writes its `.git/metrics` (verifier, AUD-023).
    let folder = std::env::temp_dir().join(format!("guard-bash-uses-{}", std::process::id()));

    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(folder.join(".git")).expect(".git");
    let mut child = clean(env!("CARGO_BIN_EXE_guard-bash"))
        .current_dir(&folder)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("guard-bash runs");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"{\"tool_input\":{\"command\":\"ls\"}}")
        .expect("written");
    assert!(child.wait_with_output().expect("ends").status.success());
    assert!(!folder.join(".git/metrics/tool-uses.jsonl").exists());
    let _ = fs::remove_dir_all(&folder);
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
            "cd \"{}\" || exit 9; git status; git commit -m x",
            worktree.display()
        ),
        Path::new("/"),
    );

    assert!(
        String::from_utf8_lossy(&output.stdout)
            .contains("A worktree refuses a chained Git command")
    );
}

#[test]
fn f21_ac1_lets_one_git_command_read_through_filters_after_the_brief_s_cd() {
    let judge = |rest: &str| {
        let judged = entered(&format!("cd \"{WORKTREE}\" || exit 9; {rest}"), MAIN);

        entered_worktree_reason(&judged)
    };

    // The recorded cases: `git log | head` passes, `git status; ls` is refused.
    for rest in [
        "git log --oneline | head -5",
        "git log --oneline -4 2>&1 | tail -15",
        "git status --porcelain | grep -v '^??' | wc -l",
        "git diff --stat | sort | uniq -c | cut -c1-80",
        "git show HEAD | sed -n '1,20p'",
        "git log --format='%h %s' | awk '{print $1}'",
        // A `|` inside quotes is no pipe (EN-113).
        "git log --format='%h|%s' | grep -E 'fix|feat'",
        // Since EN-119, a chain of Git reads, each piped or not.
        "git log | head; git status",
        "git log | head && git status",
    ] {
        assert_eq!(judge(rest), None, "{rest}");
    }
    for rest in [
        "git status; ls",
        "git status; head README.md",
        "git log && wc -l x",
        "git status && ls",
        "git status || ls",
        "git push | head; git status",
        "git log | git status",
        "git log | tee out.txt",
        "git log | head > out.txt",
        "git log > out.txt | head",
        "git log | sed -i '' 's/a/b/' x",
        "git log | sed 's/a/b/'",
        "git log | awk '{system(\"rm x\")}'",
        "git log | awk '{print > \"f\"}'",
        "ls | head",
    ] {
        assert!(judge(rest).is_some() || !rest.starts_with("git"), "{rest}");
    }
    // `ls | head` holds no Git command: nothing to refuse.
    assert_eq!(judge("ls | head"), None);
    // Without the brief's `cd … || exit`, a pipe after a Git command is refused as before.
    assert!(worktree_reason("git log --oneline | head -5", WORKTREE).is_some());
    let plain = entered(&format!("cd \"{WORKTREE}\" && git log | head"), MAIN);

    assert!(entered_worktree_reason(&plain).is_some());
    // In the worktree's own folder, with no `cd` at all: refused as before.
    let unguarded = Entered {
        folder: WORKTREE.to_owned(),
        command: "git log | head".to_owned(),
        guarded: false,
    };

    assert!(entered_worktree_reason(&unguarded).is_some());
}

#[test]
fn tells_a_read_only_filter() {
    for filter in [
        "head -5",
        "tail",
        "grep -c x",
        "wc -l",
        "cut -d: -f1",
        "sort -u",
        "uniq",
        "sed -n 1p",
        "awk 'NR==1'",
        "head 2>&1",
        "sort -k2 -t: -u",
        "uniq -c",
        "uniq -f 1 in.txt",
        "sed -n '/^a/,/b$/p'",
        "sed -nE -e '1,5p' -e '$p'",
        "grep -c x < in.txt",
    ] {
        assert!(is_read_only_filter(filter), "{filter}");
    }
    for filter in [
        "tee x",
        "xargs rm",
        "sed 1d",
        "sed -n -i 1p x",
        "awk '{getline}'",
        "head > x",
        "",
        // Writers and runners found by AUD-023's review.
        "sort -o out.txt",
        "sort -uo out.txt",
        "sort --output=out.txt",
        "sort --out out.txt",
        "sort --compress-program=sh",
        "uniq - out.txt",
        "uniq in.txt out.txt",
        "sed -n 'w out.txt'",
        "sed -n 's/a/b/w out.txt'",
        "sed -n '1e date'",
        "sed -n --in-place 1p x",
        "sed -n --in 1p x",
        "awk '{print | \"sh\"}'",
        "awk -f prog.awk",
        "head $(git status)",
        "grep `git push`",
        "head &> x",
    ] {
        assert!(!is_read_only_filter(filter), "{filter}");
    }
    assert!(!is_git_read_through_filters("git log"));
    assert!(!is_git_read_through_filters(""));
}

/// The cases of AUD-023's review, each with the reason expected, `None` for a command that goes
/// through: forms that passed the text matchers, and commands they refused for a text.
const AUDIT_CASES: [(&str, Option<&str>); 109] = [
    // A force push, its option bundled, quoted, after a global option, an assignment, a wrapper,
    // in a subshell, a substitution, a `-c` text, an `eval`, or across a line continuation.
    ("git push -uf origin x", Some(FORCE_PUSH)),
    ("git push -fu origin x", Some(FORCE_PUSH)),
    ("git push \"-f\"", Some(FORCE_PUSH)),
    ("git push \"--force\" origin x", Some(FORCE_PUSH)),
    ("git push --forc origin x", Some(FORCE_PUSH)),
    ("git push origin '+x'", Some(FORCE_PUSH)),
    ("git push --mirror origin", Some(FORCE_PUSH)),
    ("git -P push -f", Some(FORCE_PUSH)),
    ("git --git-dir .git push -f", Some(FORCE_PUSH)),
    ("/usr/bin/git push -f", Some(FORCE_PUSH)),
    ("GIT_DIR=. git push -f", Some(FORCE_PUSH)),
    ("env git push -f", Some(FORCE_PUSH)),
    ("env -u HOME A=1 git push -f", Some(FORCE_PUSH)),
    ("env -S 'git push -f'", Some(FORCE_PUSH)),
    ("command git push -f", Some(FORCE_PUSH)),
    ("sudo -u me git push -f", Some(FORCE_PUSH)),
    ("nohup nice -n 5 timeout 10 git push -f", Some(FORCE_PUSH)),
    ("(git push -f)", Some(FORCE_PUSH)),
    ("{ git push -f; }", Some(FORCE_PUSH)),
    ("if git push -f; then :; fi", Some(FORCE_PUSH)),
    ("x=$(git push -f)", Some(FORCE_PUSH)),
    ("echo \"$(git push -f)\"", Some(FORCE_PUSH)),
    ("echo `git push -f`", Some(FORCE_PUSH)),
    ("cat <(git push -f)", Some(FORCE_PUSH)),
    ("bash -c \"git push -f\"", Some(FORCE_PUSH)),
    ("bash -o pipefail -lc 'git push -f'", Some(FORCE_PUSH)),
    ("eval git push -f", Some(FORCE_PUSH)),
    ("eval 'git push -f'", Some(FORCE_PUSH)),
    ("git push \\\n -f", Some(FORCE_PUSH)),
    ("git push $'-\\x66'", Some(FORCE_PUSH)),
    ("echo x | xargs git push -f", Some(FORCE_PUSH)),
    (r"find . -exec git push -f \;", Some(FORCE_PUSH)),
    ("target/release/heavy-lock git push -f", Some(FORCE_PUSH)),
    (
        "cargo run -q -p heavy-lock -- git push -f",
        Some(FORCE_PUSH),
    ),
    (
        "cargo run -q -p heavy-lock -- --lock merge git push -f",
        Some(FORCE_PUSH),
    ),
    ("git -c alias.p=push p -f", Some(FORCE_PUSH)),
    ("git -c 'alias.p=!git push -f' p", Some(FORCE_PUSH)),
    // The hooks skipped: quoted, abbreviated, by a merge.
    ("git commit \"--no-verify\" -m x", Some(NO_VERIFY)),
    ("git commit \"-n\" -m x", Some(NO_VERIFY)),
    ("git commit --no-veri -m x", Some(NO_VERIFY)),
    ("git commit -anm x", Some(NO_VERIFY)),
    (
        "target/release/heavy-lock git commit -n -m x",
        Some(NO_VERIFY),
    ),
    ("git merge --no-verify x", Some(NO_VERIFY)),
    ("git push --no-verify origin x", Some(NO_VERIFY)),
    // The hooks' folder set: in any case, glued, quoted, by the environment.
    ("git config core.hookspath /tmp", Some(HOOKS_OFF)),
    ("git -c core.hookspath=x commit -m x", Some(HOOKS_OFF)),
    ("git -ccore.hooksPath=x status", Some(HOOKS_OFF)),
    ("git --config-env=core.hooksPath=X status", Some(HOOKS_OFF)),
    ("git config \"core.hooksPath\" x", Some(HOOKS_OFF)),
    (
        "git config -f .git/config core.hooksPath x",
        Some(HOOKS_OFF),
    ),
    ("git config --remove-section core", Some(HOOKS_OFF)),
    ("git config --edit", Some(HOOKS_OFF)),
    (
        "GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=core.hooksPath GIT_CONFIG_VALUE_0=x git commit -m x",
        Some(HOOKS_OFF),
    ),
    (
        "export GIT_CONFIG_PARAMETERS=\"'core.hooksPath'='x'\"",
        Some(HOOKS_OFF),
    ),
    // A merge after an assignment or a wrapper, or through the API.
    ("GH_X=1 gh pr merge 3", Some(MERGE)),
    ("env gh pr merge 3", Some(MERGE)),
    ("command gh pr merge 3", Some(MERGE)),
    ("gh pr -R o/r merge 3", Some(MERGE)),
    ("target/release/heavy-lock gh pr merge 3", Some(MERGE)),
    ("gh api -X PUT repos/o/r/pulls/3/merge", Some(MERGE)),
    // A push to `main` or `develop`, named, deleted, by pattern, or all branches.
    ("git push origin HEAD:develop", Some(PROTECTED_PUSH)),
    ("git push origin :develop", Some(PROTECTED_PUSH)),
    ("git push origin main", Some(PROTECTED_PUSH)),
    ("git push origin x:refs/heads/main", Some(PROTECTED_PUSH)),
    ("git push --delete origin develop", Some(PROTECTED_PUSH)),
    (
        "git push origin 'refs/heads/*:refs/heads/*'",
        Some(PROTECTED_PUSH),
    ),
    ("git push origin :", Some(PROTECTED_PUSH)),
    ("git push --all origin", Some(PROTECTED_PUSH)),
    // A here-document is opened by an unquoted `<<` outside a comment, its body read from the end
    // of its logical line; a shell reading its input runs what it is fed (AUD-033).
    ("echo '<<EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("echo x # <<EOF\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("echo '<<\"A B\"'\ngit push -f\nA B", Some(FORCE_PUSH)),
    ("echo \"<<EOF\"\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("echo \\<<EOF\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("echo $((1<<2))\ngit push -f\n2", Some(FORCE_PUSH)),
    ("echo `cat <<EOF`\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("echo `cat <<EOF `\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("echo $(cat <<EOF)\ngit push -f\nEOF", Some(FORCE_PUSH)),
    (
        "echo $(cat <<EOF) $(true\ngit push -f\nEOF\n)",
        Some(FORCE_PUSH),
    ),
    (
        "cat <<EOF; bash -c \"\ngit push -f\n\"\nEOF",
        Some(FORCE_PUSH),
    ),
    ("cat <<EOF\n$(git push -f)\nEOF", Some(FORCE_PUSH)),
    ("cat <<EOF\n`git push -f`\nEOF", Some(FORCE_PUSH)),
    ("bash <<EOF\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("bash <<'EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("sh -s <<'EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("bash -s x y <<'EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("zsh - <<'EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("env bash <<'EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    (
        "source /dev/stdin <<'EOF'\ngit push -f\nEOF",
        Some(FORCE_PUSH),
    ),
    (". /dev/stdin <<'EOF'\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("cat <<'EOF' | bash\ngit push -f\nEOF", Some(FORCE_PUSH)),
    ("bash <<< 'git push -f'", Some(FORCE_PUSH)),
    ("echo 'git push -f' | bash", Some(FORCE_PUSH)),
    ("printf '%s\\n' 'git push -f' | sh", Some(FORCE_PUSH)),
    // What the text matchers refused, and goes through.
    ("git commit -m \"fix -n flag\"", None),
    ("git commit -m n", None),
    ("git config core.hooksPath > /dev/null", None),
    ("grep -E \"x|gh pr merge\" notes.md", None),
    ("node -e \"console.log('git push -f')\"", None),
    ("echo 'git push --force'", None),
    ("git push --force-with-lease origin docs/aud-023-x", None),
    ("git push -u origin feature/f21-agent-tools-in-rust", None),
    ("git push origin docs/aud-023-x:docs/aud-023-x", None),
    // A here-document's body is data unless a shell reads it: quoted, in a substitution inside
    // quotes, two on a line, its substitutions harmless (AUD-033).
    (
        "git commit -m \"$(cat <<'EOF'\nfix: x\n\ngit push -f is refused\nEOF\n)\"",
        None,
    ),
    ("cat <<A <<'B'\ngit push -f\nA\ngit push -f\nB", None),
    ("cat <<EOF\ngit push -f is $(date)\nEOF", None),
    ("cat <<'EOF'\n$(git push -f) is not run\nEOF", None),
    ("cat <<EOF\ngit push -f\nEOF\n# <<X", None),
    ("bash script.sh <<'EOF'\ngit push -f\nEOF", None),
    ("echo 'git push -f' | grep push", None),
];

#[test]
fn judges_each_case_of_the_audit_word_by_word() {
    for (command, expected) in AUDIT_CASES {
        assert_eq!(denied_reason(command), expected, "{command}");
    }
}

#[test]
fn f31_ac1_judges_each_case_of_the_audit_alike_after_the_brief_s_cd_into_a_worktree() {
    // The guard's worktree forms read by words (EN-119) change nothing of what it forbids: each
    // case gives the same answer typed after `cd "<worktree>" || exit 9` from the main checkout.
    let branch = |_: &Repository| Some("chore/en-119-the-guard-reads-by-words".to_owned());

    for (command, expected) in AUDIT_CASES {
        let line = format!("cd \"{WORKTREE}\" || exit 9; {command}");

        assert_eq!(
            shell_reason(&line, MAIN, &branch),
            expected.map(str::to_owned),
            "{command}"
        );
    }
}

#[test]
fn refuses_a_push_naming_no_destination_on_a_protected_branch() {
    let on = |branch: &'static str| move |_: &Repository| Some(branch.to_owned());

    for command in [
        "git push",
        "git push origin",
        "git push -u origin HEAD",
        "git push origin @",
    ] {
        assert_eq!(
            denied_reason_on(command, MAIN, &on("develop")),
            Some(PROTECTED_PUSH),
            "{command}"
        );
        assert_eq!(
            denied_reason_on(command, MAIN, &on("main")),
            Some(PROTECTED_PUSH),
            "{command}"
        );
        assert_eq!(
            denied_reason_on(command, MAIN, &on("docs/aud-023-x")),
            None,
            "{command}"
        );
    }
    // A destination named: the current branch is not asked.
    let never = |_: &Repository| -> Option<String> { panic!("the branch is not asked") };

    assert_eq!(denied_reason_on("git push origin x:y", MAIN, &never), None);
    assert_eq!(denied_reason_on("git status", MAIN, &never), None);
}

/// The main checkout's Git folder.
const MAIN_GIT: &str = "/work/SVG editor/.git";

/// The branch of each repository as the hook would read it: the main checkout on `develop`, by
/// its folder or its Git folder; a folder named `/unreadable` gives none; any other a story's.
fn main_on_develop(repository: &Repository) -> Option<String> {
    let main = match repository.git_dir.as_deref() {
        Some(git_dir) => git_dir == MAIN_GIT,
        None => repository.folder == MAIN,
    };

    (repository.folder != "/unreadable")
        .then(|| if main { "develop" } else { "docs/aud-033-x" }.to_owned())
}

#[test]
fn f31_ac1_reads_the_branch_of_a_push_in_the_folder_it_runs_in() {
    // A push naming no destination judged by the branch of its own repository (AUD-033): the last
    // `cd` before it, its `-C`, its `--git-dir`, its `GIT_DIR`, each read from the session's
    // folder, `/tmp`, on a story's branch.
    for command in [
        format!("cd /tmp && cd \"{MAIN}\" && git push"),
        format!("cd \"{WORKTREE}\" || exit 9; cd \"{MAIN}\"; git push"),
        format!("cd \"{WORKTREE}\" || exit 9\ncd .. && cd ../.. && git push -u origin HEAD"),
        format!("git -C \"{MAIN}\" push"),
        "git -C / -C \"work/SVG editor\" push origin @".to_owned(),
        format!("GIT_DIR=\"{MAIN_GIT}\" git push"),
        format!("git --git-dir \"{MAIN_GIT}\" push"),
        format!("git --git-dir=\"{MAIN_GIT}\" push -u origin HEAD"),
        format!("env GIT_DIR=\"{MAIN_GIT}\" git push"),
        format!("cd \"{MAIN}\" && git push; cd \"{WORKTREE}\" || exit 9"),
        format!("git -c 'alias.p=!cd \"{MAIN}\" && git push' p"),
    ] {
        let from_tmp = |cwd: &str| denied_reason_on(&command, cwd, &main_on_develop);

        assert_eq!(from_tmp("/tmp"), Some(PROTECTED_PUSH), "{command}");
    }
    // From the main checkout: a push before the brief's `cd` is judged there, and a `cd` that may
    // fail leaves the push where it was too.
    for command in [
        format!("git push; cd \"{WORKTREE}\" || exit 9"),
        format!("cd \"{WORKTREE}\"; git push"),
        format!("cd \"{WORKTREE}\" && git push"),
        format!("true || cd \"{WORKTREE}\" || exit 9; git push"),
        format!("(\ncd \"{WORKTREE}\" || exit 9\n)\ngit push"),
        format!("{{\ncd \"{WORKTREE}\" || exit 9\n}} &\ngit push"),
        format!("if false\nthen\ncd \"{WORKTREE}\" || exit 9\nfi\ngit push"),
    ] {
        assert_eq!(
            denied_reason_on(&command, MAIN, &main_on_develop),
            Some(PROTECTED_PUSH),
            "{command}"
        );
    }
    // Where the guard cannot know the folder or the branch: refused, the reason saying how.
    for command in [
        "cd \"$HOME\" && git push".to_owned(),
        "cd ~ && git push".to_owned(),
        "cd - && git push".to_owned(),
        format!("(cd \"{MAIN}\"; git push)"),
        format!("bash -c 'cd \"{MAIN}\" && git push'"),
        format!("env -C \"{MAIN}\" git push"),
        format!("pushd \"{MAIN}\" && git push"),
        format!("export GIT_DIR=\"{MAIN_GIT}\"; git push"),
        format!("GIT_DIR=\"{MAIN_GIT}\"; git push"),
        format!("GIT_WORK_TREE=\"{MAIN}\" git push"),
        "git -C \"$X\" push".to_owned(),
        "git -C /unreadable push".to_owned(),
        format!("if cd \"{MAIN}\"; then git push; fi"),
        "git -c 'alias.p=!cd ~ && git push' p".to_owned(),
    ] {
        assert_eq!(
            denied_reason_on(&command, "/tmp", &main_on_develop),
            Some(UNKNOWN_PUSH),
            "{command}"
        );
    }
    assert!(UNKNOWN_PUSH.contains("`cd \"<folder>\" || exit 9`"));
    // The brief's form, and a folder on a story's branch, go through.
    for (command, cwd) in [
        (
            format!("cd \"{WORKTREE}\" || exit 9\ngit push -u origin HEAD"),
            MAIN,
        ),
        (format!("cd \"{WORKTREE}\" || exit 9; git push"), MAIN),
        ("git push".to_owned(), WORKTREE),
        (format!("git -C \"{WORKTREE}\" push"), MAIN),
        (
            format!("cd \"{MAIN}\" || exit 9; git push origin x:y"),
            "/tmp",
        ),
        ("cd \"$HOME\" && git push origin HEAD:x".to_owned(), "/tmp"),
    ] {
        assert_eq!(
            denied_reason_on(&command, cwd, &main_on_develop),
            None,
            "{command}"
        );
    }
}

/// A repository on `develop`, made for a hook's test.
fn on_develop(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("guard-bash-{name}-{}", std::process::id()));

    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("folder");
    git(&root, &["init", "-q", "-b", "develop"]);
    root
}

#[test]
fn f31_ac1_the_hook_reads_the_branch_of_a_push_in_its_own_repository() {
    let root = on_develop("develop");
    let story = old_branch("story");
    let main = root.display();

    for command in [
        format!("cd \"{}\" && cd \"{main}\" && git push", story.display()),
        format!("git -C \"{main}\" push"),
        format!("GIT_DIR=\"{main}/.git\" git push"),
    ] {
        let output = hook(&command, &story);

        assert!(
            String::from_utf8_lossy(&output.stdout).contains(PROTECTED_PUSH),
            "{command}"
        );
    }
    // From the story's repository alone, the push goes through.
    assert_eq!(hook("git push", &story).stdout, [] as [u8; 0]);
    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_dir_all(&story);
}
