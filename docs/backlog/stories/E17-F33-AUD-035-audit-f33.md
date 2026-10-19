---
id: AUD-035
epic: E17
feature: F33
title: Audit F33
status: ready
points: 1
---

# E17 · F33 · AUD-035 — Audit F33

Audit F33 following [audit.md](../../conventions/audit.md) (ADR-0020), the `auditor` alone (ADR-0086).

## Acceptance criteria

- Given F33's criteria, when the audit ends, then each is traced to a test titled `[F33.ACn] …` or `f33_acn_…`, broken once on purpose and seen failing.

## Tasks

- [x] T1 — The findings and the tests of their fixes, seen failing; the mutations of the criteria's tests (1 h)
- [x] T2 — The findings fixed: `guard-bash` through cargo's global options and `r` first, then `git-sequences commit`'s hook notes, `local/anchor-slugs`' ids and exemptions, one `median`, the cargo tools' pins, with the pages that describe them (2 h)
- [x] T3 — ADR-0093 for the editor's TypeScript, the type check's pages, and a binary that runs a command guarded before it is built, in the run skill and the worker brief (1 h)

One task = one commit, referenced as `AUD-035.T1`.

## Findings

Reviewed by the `auditor` alone (ADR-0086), read-only; each finding reproduced before it is recorded, by a test of T1 seen failing on the code as F33 left it, by the release `guard-bash` fed a hook's JSON, or by reading where no test can show it. The mutations by hand, each restored from a copy of its file and the copy compared.

| Id | Check | Result | Evidence | Action |
| --- | --- | --- | --- | --- |
| F1 | E3 — `guard-bash` looks inside `cargo` only when its first word is `run`: `cargo r …`, `cargo +stable run …`, `cargo -q run …`, `cargo --config k=v run …`, `cargo -Zx run …` reach `heavy-lock` or `bench` unjudged, on `develop` too for `heavy-lock` | reproduced: the worktree's release `guard-bash` denies `cargo run -q -p heavy-lock -- gh pr merge 3` and lets `cargo r -q -p heavy-lock -- gh pr merge 3` and `cargo +stable run -p bench -- --runs 1 -- gh pr merge 3` through | `tools/hooks/src/forbidden.rs`, `unwrap`; `f33_ac3_refuses_every_listed_refusal_run_through_cargo_s_global_options_and_its_alias` fails (T1) | fixed first: cargo's global options skipped (`+<toolchain>`, `-q`, `-v`, `--quiet`, `--verbose`, `--locked`, `--offline`, `--frozen`, and `--config`, `-Z`, `-C`, `--color` with their value apart or glued), then `run` or its alias `r` (`REF-CARGO-CLI`); nothing less refused (T2) |
| F2 | process — the main checkout's guard did not judge `bench` while `bench` binaries existed in worktrees: EN-141's T2 built the wrapper before the guard read it | reproduced by reading EN-141's commits; the main thread deleted the built `bench` binaries on 2026-10-19 | `docs/backlog/stories/E17-F33-EN-141-a-bench-binary.md` | the run skill and the worker brief: a binary that runs a command lands with the guard's change in an earlier task, and the main checkout's `guard-bash` is rebuilt from the branch before that binary is built (T3) |
| F3 | E3 — `git-sequences commit` prints a step's output only when it fails, so the `commit-msg` hook's `move-rows --notes` of a `done` commit is never seen | reproduced: a `commit-msg` hook writing a note and passing leaves the tool's standard error empty | `tools/git-sequences/src/main.rs`, `commit`; `f33_ac2_prints_what_the_hooks_write_when_the_commit_succeeds` fails (T1) | the commit step's standard error printed on the tool's when the commit succeeds (T2) |
| F4 | A3 — `local/anchor-slugs`' `top` and line-reference exemptions untested for another page | reproduced, and worse: `[y](./b.md#L3)` is reported, the fragment being lower-cased before `is_line_reference` reads its `L`; the exemption never held | `tools/lint/src/markdown/rules/anchor_slugs.rs`, `problem`; `f33_ac2_accepts_a_link_whose_heading_has_one_anchor` fails once given both links (T1) | the line reference read before the case is dropped; each exemption's mutation seen failing (T2) |
| F5 | D1 — `anchors()` repeats `markdown/no-missing-link-fragments`' ids of a page without its custom `{#id}` nor its HTML ids, so a link to them on another page is reported | reproduced: links to `{#rust-tools}`, `<a id="here">` and `<a name="there">` of another page are each reported | `f33_ac2_accepts_a_custom_or_an_html_id_of_another_page` fails (T1) | one shared function for a page's ids, custom and HTML ids included, both rules reading it (T2) |
| F6 | D1 — three `median`s with three even-count rules: `bench` the mean rounded down, `metrics` the mean, `context-size` the lower middle | reproduced by reading | `tools/bench/src/lib.rs`, `tools/metrics/src/tables.rs`, `tools/hooks/src/context.rs`; `tools/tool-use/tests/median.rs` fails on the placeholder (T1) | one `median` in `tool-use`, which the three crates share: the middle value, or the mean of the two middle ones, a whole number's rounded down; no caller needs the lower middle, so `context-size`'s per-type median of an even count moves up to the mean (T2) |
| F7 | E3 — the cargo tools' check never reports a pinned version under 7 days old, yanked or missing from the index, as `Cargo.lock`'s check does; the row cites no `REF-CARGO-REGISTRY-INDEX` | reproduced: cargo-llvm-cov pinned at 0.9.1 two days after its release, a yanked cargo-deny 0.20.2 and a cargo-llvm-cov 0.9.9 the index lacks give no line | `tools/deps/src/outdated.rs`, `tool_report`; `f33_ac1_a_tool_pinned_too_new_yanked_or_missing_is_listed_as_a_locked_crate_is` fails (T1) | each listed, with the greatest compatible version 7 days old where there is one; the Rust tools' paragraph of `dependencies.md` cites `REF-CARGO-REGISTRY-INDEX` (T2) |
| F8 | C — two pages say the type check runs through oxlint's `--type-check`, which EN-121 moved to `typeCheck` in `.oxlintrc.json` | reproduced by reading | `docs/tooling/dependencies.md` (`typescript`), `docs/conventions/typescript.md` | both name `typeCheck` of `.oxlintrc.json` (T3); the ADRs, accepted, stay as written |
| F9 | C — ADR-0075's sentence on the editors is outdated by EN-121's TypeScript 7 extension and its two settings | reproduced: `.vscode/settings.json` sets `js/ts.experimental.useTsgo` and `js/ts.tsdk.path`, which ADR-0075 says it no longer points at | `docs/adr/0075-the-toolchain-on-oxc.md`, Consequences | ADR-0093 supersedes that sentence, recording the extension and its two settings (`REF-TS7-VSCODE`) (T3) |
| F10 | A3 — `backlog:index -- --check`'s test only runs on an up-to-date index, where a write that dropped `--check` would pass too | reproduced by reading | `tests/backlog.test.ts` | a test on a stale copy of the backlog in `target/`, a non-zero exit naming the row; caught once `--check` is dropped from its call, restored (T1) |
| F11 | A4 — `cargo_tools::install_pins` skipping only long options before the tools survives: no case has a short one | survived, then caught: `skip_while(starts_with("--"))` fails `f33_ac1_reads_the_tools_of_each_cargo_install_line` once given `cargo install -q --locked cargo-deny@0.20.2` | `tools/deps/src/cargo_tools.rs` | the case added (T1); restored |
| C1 | A4 — `cargo_tools.rs`: `env_pins`' key kept with its underscores; `row_pins` read past the Rust tools' section; `disagreements` needing three versions | caught, each | `f33_ac1_reads_a_tool_s_version_from_a_workflow_s_env`, `f33_ac1_reads_the_rows_of_the_rust_tools_table_only`, `f33_ac1_a_tool_behind_is_listed_once_its_release_is_7_days_old` | restored |
| C2 | A4 — `.oxlintrc.json`'s `typeCheck` false; `rust:coverage`'s functions floor at 93; `.vscode/settings.json`'s `js/ts.tsdk.path` to a missing folder | caught, each | `[F33.AC1] sets typeCheck beside typeAware in its options`, `[F33.AC1] reports a planted type error with the repository's configuration alone, no flag`, `[F19.AC9] [F33.AC1] runs the workspace's tests under a floor of lines and functions…`, `[F33.AC1] is the editor's…` | restored |
| C3 | A4 — `slugger.rs`: no underscore before a leading digit; the site's numbering starting at 2 | caught, each | `f33_ac2_writes_the_site_s_anchors`, `f33_ac2_numbers_a_site_anchor_met_again` and three of `anchor_slugs.rs` | restored |
| C4 | A4 — `anchor_slugs.rs`: `joined` not climbing on `..`; two anchors the same taken as differing | caught, each | `reads_a_link_s_target_and_joins_its_path`, `f33_ac2_refuses_a_heading_whose_anchors_differ_naming_its_page` | restored |
| C5 | A4 — `move-rows --notes` writing the rows too | caught | `f33_ac2_names_each_note_with_its_story_and_with_notes_writes_nothing` | restored |
| C6 | A4 — `tests/agent-pages.test.ts`: the finder keeping no script | caught | `[F33.AC2] finds an npm run naming a script package.json lacks, its line named` | restored |
| C7 | A4 — `bench`: the last failing status kept rather than the first | caught | `f33_ac3_exits_with_the_first_failing_status_its_median_of_the_passing_runs` | restored |
| C8 | A5 — properties: F33 adds no `math` nor `geometry` function | no finding | `src/` unchanged | — |
| C9 | A4 — T2's fixes: `cargo_run_arguments` without `r`, without `+<toolchain>`, or without `--config` taking a value; the hooks' notes printed only when empty; the line reference read lower-cased, `#top` not exempt, another page's ids not read; the median's whole mean rounded up, its decimal mean the greater value; a yanked tool pin left unreported | caught, each; the release `guard-bash` rebuilt in the worktree denies `cargo r -q -p heavy-lock -- gh pr merge 3` and `cargo +stable run -p bench -- --runs 1 -- gh pr merge 3`, and lets `cargo --config k=v r -p heavy-lock -- true` through | `f33_ac3_refuses_every_listed_refusal_run_through_cargo_s_global_options_and_its_alias`, `f33_ac2_prints_what_the_hooks_write_when_the_commit_succeeds`, `f33_ac2_accepts_a_link_whose_heading_has_one_anchor`, `f33_ac2_accepts_a_custom_or_an_html_id_of_another_page`, `tools/tool-use/tests/median.rs`, `f33_ac1_a_tool_pinned_too_new_yanked_or_missing_is_listed_as_a_locked_crate_is` | restored |
| C10 | B — the sources F33's fixes cite | `REF-CARGO-CLI` read through the library on 2026-10-19 (`cargo(1)` and Configuration, `[alias]`) and added; `REF-CARGO-REGISTRY-INDEX` and `REF-TS7-VSCODE` read again by `page` | `docs/references.md` | — |
| C11 | `[F33.AC1]` traced | caught | F7, F11, C1, C2 | restored |
| C12 | `[F33.AC2]` traced | caught | F3 to F5, F10, C3 to C6 | restored |
| C13 | `[F33.AC3]` traced | caught | F1, F6, C7 | restored |
| C14 | F — `check:all` green on the head | run by the pre-push hook | the push of T3 | — |

## G. Next feature

F33 closes RET-004's list of checks. What its fixes leave unread, each said where it is written: an alias of `run` set in a Cargo configuration (`[alias]`, `CARGO_ALIAS_<NAME>`, `--config alias.<name>=…`) is not read by `guard-bash`, as a Git alias set earlier is not (`docs/tooling/agent.md`); a heading's custom `{#id}` is accepted on another page as on its own, though GitHub does not read it, the repository holding none on 2026-10-19. No source is needed beyond those read here.

## H. Light evolvability check

- Size: `forbidden.rs` gained `cargo_run_arguments`; `anchor_slugs.rs` lost its own slugging; three `median`s became one.
- Tools: none failed; a `replace` block run in a loop split its old text at an `=` it held, written then as one block each.
- Fitness functions: each finding fixed has a test that failed before its fix, and each fix a mutation caught.
- Patterns: a page's ids read by one function for both link rules; one median; a tool pin judged as a locked crate.
- Agent configuration: the run skill and the worker brief order a wrapper's guard before its binary (F2).
- Process: F1 came from reading every way a program names its subcommand, and F4 from writing the case the finding said untested, which showed the exemption had never held; the next audit of a reader of commands or links does both.

## For the main thread

- F1 holds for the main checkout only once its `guard-bash` is rebuilt from this branch, or from `develop` once F33 reaches it; until then `cargo r -p heavy-lock -- …` goes through there. `.claude/settings.json` is unchanged.
- F2's process sends the worker back after the guard's task and asks the main thread to copy the worktree's `guard-bash` over the main checkout's, then to start a second worker for the rest of the story: one more worker than ADR-0086's one per story, for that kind of story only.
- F6 changes `context-size`'s median per type for an even count: the mean of the two middle ones rounded down, where it took the lower one (7 and 9 give 8, no longer 7).
- F7: `deps:outdated` now also lists a cargo tool pinned under 7 days old, yanked or missing from the index; the pins of 2026-10-19 (cargo-llvm-cov 0.9.1 of 2026-09-06, cargo-deny 0.20.2 of 2026-07-09) give no line.
- `docs/domain/` is not affected: F33 is tooling. Footer `Rule-docs: none affected, a tooling feature, no business rule changed`.
