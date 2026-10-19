---
id: AUD-033
epic: E17
feature: F31
title: Audit F31
status: ready
points: 1
---

# E17 · F31 · AUD-033 — Audit F31

Audit F31 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F31's criteria, when the audit ends, then each is traced to a test titled `[F31.ACn] …` or `f31_acn_…`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — The findings, the interfaces of the fixes and their tests, seen failing (1 h)
- [ ] T2 — The findings fixed: the here-documents and the shell fed, the push's own folder, the Python check, the heavy lock, the write guard's paths and worktree, with the references they cite (2 h)
- [ ] T3 — ADR-0092 for the type-only barrels, the docs, the criteria traced (1 h)

One task = one commit, referenced as `AUD-033.T1`.

## Findings

Reviewed by the `auditor` alone (ADR-0086), each finding reproduced before it is recorded: F1 and F2 by the hook JSON fed to the worktree's own `target/release/guard-bash`, rebuilt by `git-sequences sync`.

| Id | Check | Result | Evidence | Action |
| --- | --- | --- | --- | --- |
| F1 | E3 — the guard finds a here-document in the raw text, quotes and comments ignored, and never reads what a shell is fed: `echo '<<EOF'⏎git push -f⏎EOF`, `echo x # <<EOF⏎…`, `echo '<<"A B"'⏎git push -f⏎A B`, `bash <<EOF⏎git push -f⏎EOF`, `bash <<< 'git push -f'` and `echo 'git push -f' \| bash` pass; so do a body read from the line's first line feed inside quotes, a `<<` in backquotes, in a `$(…)` closed on its line or in `$((…))`, and the substitutions of an unquoted body, which the shell runs (`REF-POSIX-SHELL-QUOTING`, §2.7.4) | reproduced: the binary prints nothing for each of the six forms of the auditor | `tools/hooks/src/guard.rs`, `heredoc_at`; `forbidden.rs`, `unwrap` | the here-documents read by `tools/hooks/src/heredoc.rs`: an unquoted `<<` outside a comment, a backquote and an arithmetic expansion, its word the shell's, its body after the logical line, one opened in a `$(…)` closing in it, else none; the text a shell reads from its input (`bash`, `sh`, `zsh`, `dash`, `ksh` without `-c` nor script, `-s`, `-`, `/dev/stdin`; `source` and `.` of `/dev/stdin`, `REF-POSIX-SH`) judged as commands: the bodies, the here-strings and the words of the other commands of its line; an unquoted body's substitutions judged; every form in `AUDIT_CASES` (T1), refused (T2) |
| F2 | E3 — a push naming no destination is judged by the branch of the folder of the line's first `cd`: `cd /tmp && cd "<main>" && git push`, `git -C "<main>" push`, `GIT_DIR=<main>/.git git push` pass from the main checkout on `develop` and would push `develop` | reproduced: the binary prints nothing for the three, and refuses `cd "<main>" && git push` | `tools/hooks/src/bin/guard-bash.rs`, `shell_refusal`; `forbidden.rs`, `push_reason` | `tools/hooks/src/place.rs`: each part of the line runs in the folders the `cd` before it may have entered, the brief's `cd … \|\| exit 9` the only certain one; the push's `-C`, `--git-dir` and `GIT_DIR` applied (`REF-GIT`); a folder the guard cannot know (a variable, `~`, `-`, a `cd` in a subshell, a condition or a quoted command, `pushd`, `env -C`, a `GIT_DIR` set apart, `GIT_WORK_TREE`) or a branch it cannot read refuses such a push with `UNKNOWN_PUSH`; tests on each form and through the hook on a repository on `develop` (T1) |
| F3 | E3 — the write handler had no `onFailure`, so a missing binary let every `Write` and `Edit` through | done by the main thread in this branch (1592914c) | `.claude/settings.json` | the texts of `docs/tooling/agent.md` and `guard-bash.rs`'s header cover both handlers (T3) |
| F4 | A3 — no test covers the write handler: `the_guard_that_cannot_run_blocks_the_shell_command` expected one handler failing closed, and failed since 1592914c | reproduced: `cargo test -p hooks --test settings` fails on two `guard-bash` handlers | `tools/hooks/tests/settings.rs` | the test reads the `PreToolUse` matchers whose `guard-bash` handler blocks, `Bash` and `Write\|Edit\|NotebookEdit`, and sees a settings text without the writes' `onFailure` (T1) |
| F5 | E3 — the Python check misses `Path('f').open('w')`, `os.open(… O_WRONLY)`, `os.system`, `subprocess`, `exec`, a wrapper (`env`, `timeout`, `bash -c`), bundled options (`-Ic`), `/dev/stdin` with a here-document and `\| tee f`, and refuses `sys.stdout.write(` and `shutil.which(` | reproduced on `worktree_reason` | `tools/hooks/src/guard.rs`, `python_writes_file`, `PYTHON_WRITES` | the program found through `invocations()`, wrappers unwrapped; Python's options read (`REF-PYTHON-CMDLINE`); a path's `.open(` read with its first argument as the mode (`REF-PYTHON-PATHLIB`), `os.open` with flags other than `O_RDONLY`, `os.system`, `os.popen`, `subprocess`, `exec(` refused, `shutil` by its writing functions, `sys.stdout` and `sys.stderr` let write; a `tee` of a file in the line; tests (T1) |
| F6 | E1 — F31's criterion 2 says `replace` without input prints its usage, which the agent's Bash tool, giving no terminal, never reaches | reproduced: `replace` with a closed input and no argument prints `no edit block read` and exits 1 | `docs/backlog/features/E17-F31-…`, criterion 2; EN-138's first criterion | the criterion worded as EN-138's: a terminal, `--help` or an argument print the usage; a closed input ends at once with `no edit block read` (T3); `f31_ac2_ends_at_once_on_a_closed_input_without_an_argument`, broken once by skipping the empty case (T1) |
| F7 | E2 — `export type * from "./x";` in 37 barrels of `src/` (82 lines) and 11 of `playground/` contradicts ADR-0012's decision 1, "containing only `export * from "./x";`", which EN-138 kept | reproduced: `grep -rl '^export type \* from' src` lists 37 files | `docs/adr/0012-folder-barrels.md`; `tools/lint/src/rules/barrel_exports.rs` | ADR-0092 supersedes that decision for a module holding only types; the conventions and ADR-0012's status point to it (T3) |
| F8 | E3 — the write guard compares paths by their letters: a symbolic link into the main checkout, or the project's folder in another case on a case-insensitive file system, goes through | reproduced on macOS, where `realpath` gives the stored case | `tools/hooks/src/written.rs`, `misplaced_write` | `as_seen_from`: the longest existing folder of the path and the project resolved by the file system before comparing; hook tests on a link and on an upper-case folder (T1) |
| F9 | E3 — the worker's worktree is the first `.claude/worktrees/<id>` its transcript's first lines name: a prompt naming another worktree first guards the wrong one | reproduced by reading `write_refusal` and `check-handback`'s `answer` | `tools/hooks/src/bin/guard-bash.rs`; `check-handback.rs` | `prompt_worktree`: the folder of the first `cd "<worktree>" \|\| exit` of the prompt, as written or as its JSON escapes it, else the first worktree named; both hooks read it; tests (T1) |
| F10 | E3 — only `mkdir …/heavy.lock` is refused: `install -d .git/heavy.lock`, `mkdir .git/$LOCK` and `mkdir .git/heavy.*` pass | reproduced on `denied_reason` | `tools/hooks/src/forbidden.rs`, `makes_heavy_lock` | `install -d` and `--directory` read as a `mkdir`; an operand under `.git` holding a variable or a pattern refused; tests (T1) |
| F11 | B — the read-only Git subcommands and the writing options of `is_read_only_git` cite no page of Git's | reproduced: `GIT_WRITING_OPTIONS` cites `REF-GITCLI`, which says nothing of `--output`, `--ext-diff` nor `-O` | `tools/hooks/src/guard.rs` | the git-diff, git-log and git-grep pages read through the library, `REF-GIT-DIFF`, `REF-GIT-LOG`, `REF-GIT-GREP` cited in the code's comment (T3) |
| F12 | D1 — three constants of the worktrees' folder (`guard.rs`, `handback.rs`, `written.rs`) beside the `worktree` crate's, and three readers | reproduced | the three modules | `tools/hooks/src/worktrees.rs`: `worktree::WORKTREES`, one reader of the names (`worktree_id`, `worktree_root`, `prompt_worktree`), the three constants removed (T1) |
| F13 | E2 — `.claude/agents/clerk.md` forbids every chained Git command and `.claude/rules/tooling.md` says a worker is refused "a file outside its prompt's worktree", while the brief lets a chain of Git reads through and the guard lets a file outside the project be written | reproduced by reading | `.claude/agents/clerk.md:22`; `.claude/rules/tooling.md:39` | `clerk.md` on chained Git reads as the brief; the rule says "a file of the main checkout or of another worktree" (T3) |
| F14 | E1 — EN-138's T2 edited `.claude/rules/library.md`, a shared rule, from a story | no action: the change made its own pitfall line true (the autofix keeps `export type *`), which a story may do for the rule it changes | EN-138, T2 | — |
