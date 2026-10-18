---
id: AUD-023
epic: E17
feature: F21
title: Audit F21
status: ready
points: 2
---

# E17 · F21 · AUD-023 — Audit F21

Audit F21 in depth and the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given F21's criteria, when the audit ends, then each is traced to a Rust test titled `f21_acN_…` or a TypeScript test titled `[F21.ACn] …`, and each test is broken once on purpose and seen failing.
- Given each port, when audited, then its output was compared with the TypeScript tool's on recorded cases before the tool was removed, and the comparison is in its story.
- Given each crate F21 added, when audited, then its row in `docs/tooling/dependencies.md` holds its exact version, its age at the time it was added, its licence and its release notes; `cargo deny` finds no advisory; every package whose code or cases were ported is in `NOTICE` and `THIRD-PARTY-LICENSES.md` (ADR-0076).
- Given each tooling story, when audited, then it states its times before and after (ADR-0069), and the record of each tool's use (EN-102) shows no tool slower than its story stated.
- Given the project, when audited, then no duplicate, no copied code and no inconsistency between the tools, the docs, the ADRs and the agent's rules remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-023.Tn`.

- [x] T1 — A. Independent review by the `auditor` subagent; criteria traced, mutation spot-checks (3 h)
- [x] T2 — B to E. Sources, crates and licences, duplicates, consistency (2 h)
- [x] T3 — F, G. Quality gates and times; findings table (1 h)

## Findings

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| Auditor 1, `.claude/settings.json` allows `Bash(target/release/*)` | reproduced: `target/release/heavy-lock git push -f` ran without a prompt, `heavy-lock` running any command after it | `tools/hooks/tests/settings.rs` fails on the wildcard | fixed (`ebe8bf02`): each safe binary allowed by name, `git-sequences`, `dependencies` and `release` by subcommand; `heavy-lock`, `branches-clean`, `git-hooks`, `release prepare` ask |
| Auditor 2, verifier 4 and 5, `guard-bash` matches text | reproduced: 68 of the 77 cases of the table judged wrong by the release binary (bundled `-uf`, quoted `"-f"`, wrappers, subshells, substitutions, `bash -c`, `eval`, `--no-veri`, `core.hookspath`, no push to `main` or `develop` refused; `-m "fix -n flag"` refused); 0 of 77 after | `judges_each_case_of_the_audit_word_by_word`, `refuses_a_push_naming_no_destination_on_a_protected_branch`, `tells_a_read_only_filter`, `tools/hooks/tests/shell.rs`; three mutations by hand caught (`-f` letter, `\"` in double quotes, `uniq`'s count) | fixed (`dd942ec9`): a shell word reader (`REF-POSIX-SHELL-QUOTING`), wrappers looked through, Git's options read as Git reads them (`REF-GITCLI`), push destinations checked (`REF-GIT-PUSH`), the read-only filters tightened. Verifier 5 not reproduced: `segments` already read `'a\'` right, its test added |
| Auditor 3, a hook's binary missing | reproduced from `REF-CC-HOOKS`: a hook that cannot start is a non-blocking error, its call carried out | `the_guard_that_cannot_run_blocks_the_shell_command`, failing with `onFailure` set otherwise | fixed for `guard-bash` (`b6e4ea71`): `onFailure: block`, native since Claude Code 2.1.295 (2.1.302 here), in place of a launcher, which would be a shell script (ADR-0073); a person runs `npm run rust:build` once. No action for `limit-agents`, `library-pages`, `check-handback`: `docs/tooling/agent.md` decides that their own failure never blocks work, and `limit-agents` is backed by Claude Code's own cap |
| Auditor 4, verifier 1 to 3, criteria tags | reproduced: no `f21_ac2_` test; EN-103's criteria numbers under F21's prefix; release-please's test tagged F21.AC5; no test of F21.AC5 | `rg f21_ac` before and after | fixed (`231d76ee`): re-tagged `f21_ac2_`, `[F21.AC6]`; `[F21.AC5] keeps no package that a Rust tool replaced…` in `tests/package.test.ts`, seen failing with `vitest` added to its pattern |
| Auditor 5, `civil_from_days` twice | reproduced: `shared-files/src/date.rs` and `tool-use/src/lib.rs` | `rg civil_from_days` | fixed (`76af949d`): `shared_files::date::date_of_days` formats `tool_use::civil_from_days`; `REF-HINNANT-DATES` read through the library. `days_from_civil` copied in `tool-use/tests/record.rs`: no action, a test oracle in a crate `hooks` depends on |
| Auditor 5, `library_file` twice | reproduced: an empty home named a file in `hooks`, none in `library`; only the library read `USERPROFILE` | `the_library_file_is_the_one_the_library_scripts_open`, failing before | fixed (`76af949d`): one function in `hooks::pages`, re-exported by `library::store`, with `FILE_VARIABLE`; the library's copy of the test removed |
| Auditor 5, three Git helpers in `hooks` | reproduced | `src/git.rs`, `bin/check-handback.rs`, `bin/guard-bash.rs` | fixed (`76af949d`): `hooks::git::git`, its error output dropped (a hook exiting 0 shows Claude none) |
| Auditor 5, two JSON parsers | reproduced: `hooks::json` beside `serde_json` | `hooks::json` is used by every hook, the metrics and `run-totals` | no action here: not cheap; a row for the improvement log |
| Auditor 6, read-only filters | see auditor 2 |  | fixed (`dd942ec9`) |
| Auditor 7, `curl` reads `~/.curlrc` | reproduced: neither call passed `--disable` first (`REF-CURL`); the library's had no time limit | `curl_is_asked_for_http_only_its_facts_after_a_mark`, `curl_reads_no_curlrc` | fixed (`c6c7d299`): `--disable` first, `--max-time 60` for the library |
| Verifier 6, `.git/config` check | reproduced: `[url] insteadOf`, `[include] path`, `[includeIf]`, `[extensions]` and a key on its header line let through | `f21_ac9_guards_the_urls_rewritten_the_files_included_and_a_key_on_its_header_line`; two mutations caught (`includeIf`'s case, the header line's rest) | fixed (`6e093809`) |
| Verifier 7 and 8, the 7-day boundaries | reproduced: `>=` and `<` survived | `f21_ac4_a_crate_is_old_enough_at_exactly_7_days`, `f21_ac4_an_action_s_next_major_is_listed_at_exactly_7_days`; `>=`→`>`, `<`→`<=` caught (three mutations) | fixed (`fe128ae9`) |
| Verifier 9, `tool-use`'s debug guard | reproduced: only the exit code was tested | `a_debug_build_of_a_tool_records_no_use_in_the_checkout_it_runs_in`; `cfg!(debug_assertions)`→`false` caught | fixed (`fe128ae9`) |
| Verifier 10, the binaries' floor | reproduced: `>= 18` against 42, a comment fooled `contains` | `reads_the_call_in_the_code_never_in_a_comment`; comment stripping removed caught | fixed (`fe128ae9`): `assert_eq!(…, 42)`, the call read in code, its line breaks ignored |
| Verifier 11, `run-totals`' poll read from source | reproduced | `f21_ac7_records_every_subcommand_but_the_status_line_s_poll`; `status`→`start` caught | fixed (`fe128ae9`): `run_totals::is_poll` |
| Verifier 12, `stale-binaries`' newest source | reproduced: `max_by_key`→`min_by_key` survived | `names_the_newest_source_and_reads_only_the_listed_ones`, the mutation caught | fixed (`fe128ae9`) |
| Verifier 13 and 14, `release` | a mixed bump and a group repeated untested | `bumps_by_the_lines_a_feature_then_a_patch`, `f21_ac6_refuses_each_rule_broken_at_its_line`; `<=`→`<` caught | fixed (`fe128ae9`) |
| Verifier 15, `secret-scan/src/js.rs` | reproduced: no unit test | `tools/secret-scan/tests/js.rs`; two mutations caught (LS in the repeated unit, `DOT`) | fixed (`fe128ae9`) |
| Verifier 16, tests reading the repository | each made to fail once on purpose: `runs_every_check_of_check_the_playground_after_the_site` and `f19_ac9_writes_no_release_binary…` (`package.json` broken), `the_settings_run_the_hook_on_every_web_fetch…` (`*wget*` narrowed), both `settings.rs` tests, `f21_ac7_every_binary…` | restored each time, `diff` clean | done; `record.rs`' source-text test replaced (verifier 11), its others write a temporary `.git` |
| Verifier 17, no test starting Git by itself | none found | `f21_ac9_no_rust_test_starts_git_but_through_test_support`, failing on a planted file | added (`fe128ae9`) |
| Verifier 18, `session-state`'s one day | reproduced | `a_call_is_stale_only_past_one_day`; `>`→`>=` caught | fixed (`fe128ae9`): `hooks::session::is_stale` |
| Verifier 19, `git-hooks/tests/branch.rs` keeps the home | reproduced | its Git now has `HOME` and `GIT_CONFIG_NOSYSTEM` of its own | fixed (`fe128ae9`) |
| E3, `docs/tooling/agent.md` | said `guard-bash` refuses `git -C`, which no code does | `rg "git -C" tools/hooks` | fixed (`dd942ec9`): the claim removed, the row rewritten |
| E3, `CLAUDE.md` and `.claude/rules/tooling.md` | the guard's list lacks pushes to `main` and `develop`; the rule says the guard reads a command inside `-m "…"`, no longer true |  | for the main thread, below: agent configuration |
| A4, `npm run mutate` on `guard.rs`, `forbidden.rs`, `shell.rs`, `check_all.rs` (`-- cargo test -q -p hooks -p repo-tools`) | 14 caught, 1 survived; the four `return … → undefined` only fail to compile in Rust, so 10 of 11 that run are caught | `forbidden.rs:25` `DEPTH` 8 → 9 survived | no action: the depth only stops a runaway recursion of wrappers; nine nested (`eval eval …`) is no command an agent writes, and a test would pin the limit, not a behaviour |
| AC3, crate rows of `docs/tooling/dependencies.md` | the five crates F21 added (`rmcp`, `tokio`, `rusqlite`, `serde_json`, `regex`) held their exact version, publication date, licence and release notes, not their age when added; `oxc_syntax` (EN-109) held none of them | `git log -S` on `Cargo.toml`: all added on 2026-10-18 | fixed: each row states its age that day (13, 15, 71, 8, 95 days); `oxc_syntax` 0.153.0 of 2026-10-05, 13 days, MIT, its notes read (`REF-OXC-CRATES-0153`) |
| AC3, `cargo deny`, `NOTICE`, `THIRD-PARTY-LICENSES.md` | no advisory (`rust:advisories` in the pre-push `check:all`, green); every port named in a module (`lint-staged`, `@commitlint/*`, `conventional-commits-parser`, `conventional-changelog-conventionalcommits`, `es-toolkit`, `semver`, `secretlint`, `@secretlint/*`, `structured-source`) has its entry; `npm-check-updates`, `knip` and release-please were written anew, their modules say so | `tests/third-party-licences.test.ts` green | none needed |
| AC4, each tool's recorded uses (`target/release/metrics`, Tools) against its story's after | not slower: `guard-bash` 0 ms (2.9 stated), `track-tool-call` 0 (3.0), `log-tool-call` 2 (3.0), `page` 0 (3.2), `replace` 0 (7), `backlog-status` 172 (172), the library's server 1, 6, 6 ms (0.7, 6.6, 6.6) and command line 4 to 10 ms (5.7 to 11.3), `git-hooks commit-msg` 12 (23.7), `branch` 4 (12.8), `install` 21 (21.4), `activity` 135 (125 to 138), `check:all` 70 s (74.6 s, EN-113). A record counts the tool's work, not its process start, so it reads lower than a timer around the process | `.git/metrics/tool-uses.jsonl` of 2026-10-18 | none |
| AC4, the ones above their figure | `dependencies outdated` 1.7 s against 1.48 s, the network taking 1.27 s of it; `dependencies unused` 111 to 209 ms after EN-109's last change against 120; `session-state` 56 and 108 ms, its two uses during EN-104 itself, against 25.4; `worktree-new` 44.2 s for this story's worktree against 15.7 s seeded, during a `check:all` of the main checkout, its only use since the seeding (`6d851a23`) | the same file | no action: within the load and the network, or no use since the final version; `worktree-new` and `session-state` to read again at RET-004 |
| G, for NAM-002 | 25 `Command::new("git")` calls in 19 files left across the binaries (`format`, `deps`, `heavy-lock`, `metrics`, `run-totals`, `records-save`, `branches-clean`, `story-finish`, `secret-scan`, `worktree-new`, `refuse-shared-files`, `check-rule-docs`…), each crate its own; `hooks::json` beside `serde_json`; the text matchers of `guard.rs` kept for the worktree's forms (`after_spaces`, `starts_with_word`…), which the word reader of `shell.rs` could serve | `rg 'Command::new\("git"\)' tools/*/src` | for NAM-002's tooling pass |
| G, for E02 | nothing of the library: F21 changed no file of `src/`; what E02's stories meet is the guard's new refusals (a push to `main` or `develop`, the current branch's too) and the release binaries allowed by name |  | nothing to prepare |
| H, size | `tools/hooks` grew by `forbidden.rs` (675 lines) and `shell.rs` (352); `guard.rs` 818 | `wc -l` | nothing yet |
| H, tools | the "considered for later" rows' triggers (first npm publication, a stable API, stryker-js 6210) have not happened | `docs/tooling/dependencies.md` | nothing yet |
| H, fitness functions | the settings' permissions and the guard's fail-closed handler are now tests (`settings.rs`); no Rust test starting Git by itself is one (`test-support`) |  | done in this audit |
| H, patterns | `hooks::json` beside `serde_json` |  | improvement log, below |
| H, agent configuration | `AGENTS.md` and `CLAUDE.md` 154 lines together; two lines out of date |  | for the main thread, below |
| H, process | the audit's two read-only reviews found 26 findings, two of them bugs in guards: the reviews earn their cost |  | nothing yet |

## For the main thread

Two lines of the agent's configuration, left to the main thread: a subagent does not change `CLAUDE.md` nor `.claude/rules/` on another agent's request, which is no consent of the Product Owner's:

- `CLAUDE.md`, the `guard-bash` sentence: add "and pushes to `main` or `develop`" after "force pushes without lease (`+branch` included)".
- `.claude/rules/tooling.md`, the `guard-bash` bullet, to read: "The `guard-bash` hook skips the bodies of here-documents, which are data, and reads each command word by word as the shell does: a guarded command inside a quoted text goes through (`-m "… -n …"`), unless a shell runs that text (`bash -c`, `eval`, `$(…)`), whose commands are judged too (AUD-023)."

F21.AC9, in `docs/backlog/features/E17-F21-agent-tools-in-rust.md`, to read (the check now guards more sections, `6e093809`):

```markdown
9. `check:all` fails, naming the lines, when a run changed a line of `.git/config` a test could break (`[core]`, `[user]`, `[commit]`, `[url]`, `[include]`, `[includeIf]`, `[extensions]`, the remotes' URLs; EN-113, AUD-023), and every Rust test that runs `git` starts it without the inherited `GIT_*` variables (EN-103).
```

### Improvements

| 2026-10-18 | AUD-023 | `tools/hooks` parses JSON with its own `hooks::json` while the library, the dependency checks and the release tool use `serde_json`, already reviewed (research note 0025 said the hooks would move to it): one parser for the workspace, in the next tooling pass | tool |
