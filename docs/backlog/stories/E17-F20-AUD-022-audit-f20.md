---
id: AUD-022
epic: E17
feature: F20
title: Audit F20
status: done
points: 1
---

# E17 · F20 · AUD-022 — Audit F20

Audit F20 in depth, the project in breadth ([audit.md](../../conventions/audit.md)).

## Acceptance criteria

- Given F20's stories merged, when the audit ends, then each criterion is traced to a test or a measure and the findings are fixed or logged.

## Tasks

One task = one commit, referenced as `AUD-022.Tn`.

- [x] T1 — Audit and fixes (1 h)

## Findings

Two independent reviews on 2026-10-17, read-only (`auditor`, `verifier`); each finding reproduced before it was fixed (by a test failing on the code as it was, or by reading the code where the case could not run), and each new test made to fail once by writing the mutation it guards back by hand. Outcome: **fixed** (its commit), **no action** (with the reason), **deferred** (where it goes).

### Auditor

| Id | Severity | Evidence | Outcome |
| --- | --- | --- | --- |
| A1 | high | `check-handback` took the worktree's folder for the story (`en-087-drop` → `EN-087-DROP`), found no feature, and counted the commits over the whole history (934 listed once) | fixed (1e084d14): the story read from the branch, then the folder; the base is the feature branch, or `origin/develop` for an epic's own story, through the `worktree` crate's `story_of_file` and `base_branch` in place of a copy; no base, no naming check; ten commits listed, the others counted. Two hook tests (a folder named otherwise, no story file) fail on the old code |
| A2 | medium | `git:commit --push` on `develop` pushed it | fixed (42d74285): refused, a feature branch still pushed; `f20_ac7_` tests in `plan.rs` and `binary.rs` |
| A3a | medium | a `library_get` whose fetch throws (a name, a certificate, a time-out) recorded no `failed` use, so the page stayed refused for good | fixed (df927aa0): recorded `failed` by the server's error path; a test with a rejecting `fetch` |
| A3b | medium | the hook lets a page through after a `library_get` that came back short or `failed` | no action: that is the escape the hook offers by design, for a page built by JavaScript or refused by its site (`docs/tooling/sources.md`); the page was asked of the library first |
| A3c | medium | agents without the library's tools (`clerk`, `verifier`, `doc-editor`) were told to call `library_get` | fixed (df927aa0), the guard kept: still refused, told to name the page to their caller; the hook reads the agent's file for its `tools:` |
| A3d | medium | not page reads refused: an installer piped into `sh`, an archive into `tar`, `-o /dev/null` link checks, files saved under a name of their own, registry hosts | fixed (df927aa0): each goes through; a page saved as a page (`-o page.html`, a name without extension) is still a read |
| A3e | medium | a quoted address cut at its `&` | fixed (df927aa0): commands cut where unquoted, into pipelines and their stages |
| A4 | medium | `library-pages` ran through `cargo run` on every Bash call, with the 600-second default timeout, waiting on cargo's build lock during a `check:all` | fixed (af761318), the settings change allowed on 2026-10-17: in `.claude/settings.json`, `WebFetch` keeps its handler; the `Bash` handlers carry `"if": "Bash(*curl*)"` and `"if": "Bash(*wget*)"`, and `library-pages` and `check-handback` a `"timeout": 30`. No guard is weakened: a command the hook refuses runs `curl` or `wget`, so its text names one, `/usr/bin/curl` and `sh -c` too (`REF-CC-PERMISSIONS`); 30 seconds, not the 5 asked, so that a rebuild of the crate (about 1 s) never lets a call through. A test reads the settings |
| A5 | medium | no Rust test tagged with an F20 criterion; criterion 5 dropped; EN-098 counted tokens as characters / 4 | fixed (7bac3daa): criteria 1, 3, 4, 6, 7 tagged `f20_acN_`; criterion 8 a test of the four rewrites' measures; a struck criterion left out of the tracing; EN-098's figures restated at 2.30 characters per token. Criterion 2 is traced by `tests/agent-models.test.ts` and `scripts/session-report.test.ts`, its Sonnet trial compared in CHK-021 |
| A5-AC6 | medium | the phone push of criterion 6 untested | no action: proved by `run:should-stop` (`f20_ac6_` tests) and the run skill's push rule; the push on a phone waits for the Product Owner to turn pushes on in `/config`, not a blocker |
| A6 | low | the `GIT_*` rule read as binding the binaries that run Git, which remove none | fixed (6b452575): reworded in `.claude/rules/tooling.md`, `commands.md`, `agent-tools.md`: every test removes them from each command it starts; a binary run by the agent inherits no hook's `GIT_DIR` |
| A7 | low | duplicates: the tests' `clean` ×8, Git runners, JSON number to integer ×3, usage fields read twice, the story files listed ×3 | the story files fixed (4e173e0d): `worktree::story_files`, called by the three; the others deferred to F21, EN-103 T1's test-support crate and the Rust rewrites of EN-104 and EN-107 |
| A8 | low | `check-handback` checks a `general-purpose` agent; `SubagentHandback` exists only in auto mode | fixed (1e084d14): said in `docs/tooling/agent.md` and EN-088's decisions; auto mode read in `REF-CC-HOOKS` (2026-10-18) |

### Verifier

| Id | Evidence | Outcome |
| --- | --- | --- |
| V1 | no F20 tag on a Rust test | fixed with A5 (7bac3daa) |
| V2 | a cache ignored by `run_tokens` survived | fixed (1a518b38): a cached file counted from its cache, 1 000 + 118 + 131 = 1 249 |
| V3 | a stale tally's tokens never checked | fixed (1a518b38): 999 carried, still 365 |
| V4 | a second session never counted | fixed (1a518b38): `s2.jsonl` listed and counted, in `totals.rs` and the binary |
| V5 | `EMPTY_PAGE` only used, never stated | fixed (df927aa0): asserted 300 |
| V6 | `library_state("0\|")` untested | fixed (df927aa0): stored, 0 characters |
| V7 | a here-string or a shift opened a here-document that never ended, every later line skipped (bug) | fixed (df927aa0): `cat <<< x`, `$((1<<2))`, `$((x << n))` then a `curl` are read |
| V8 | `[::1]`, `-X GET`, `wget -O -` and `--output-document=-` untested | fixed (df927aa0) |
| V9 | `allow_output` checked field by field | fixed (1e084d14): the whole `updatedInput` |
| V10 | the feature lookup of `f20` before `f2` | fixed (1e084d14): replaced by `base_of`, tested with `f2-b` listed before `f20-a`; `feature_number` removed |
| V11 | a Bash call naming `SubagentHandback` | fixed (1e084d14): counts 0 |
| V12 | the set-up test's child never checked | fixed (1e084d14): success and `1 passed` |
| V13 | the task and a later user line of the same length | fixed (1a518b38): 16 against 10 |
| V14 | one row per type not proved | fixed (1a518b38): the row counted once |
| V15 | a window without its reset time | fixed (1a518b38): no snapshot |
| V16 | `run 0 s` flaky across a second | fixed (1a518b38): durations matched by their form |
| V17 | `git:sync` on `main` refused before its fetch, untested | fixed (42d74285): an `origin` that does not exist, the refusal names `main` |
| V18 | tests running Git with the user's home | fixed (1e084d14, 42d74285): an empty home and `GIT_CONFIG_NOSYSTEM=1` |
| V19 | a parked story's question, and a front matter never closed | fixed (1a518b38): each parked story holds its question (`tests/backlog.test.ts`); an unclosed front matter is no parked story |
| V20 | a wrong comment in `tally.rs` | fixed (1a518b38) |
| V21 | `tests/agent-models.test.ts` mutation-checked? | yes: `clerk.md` set to `opus`, `[F20.AC2] clerk.md names its model and why` fails |

### A4. Mutation spot-checks

`npm run mutate` on 2026-10-18, the first mutation of each kind per file. Rust: `handback.rs`, `pages.rs`, `context.rs`, `totals.rs`, `tally.rs`, `stop.rs`, `git-sequences/src/lib.rs`, under `cargo test -p hooks -p run-totals -p git-sequences`: 19 caught, 2 survived. In Rust a returned value made `undefined`, or an array's length changed, fails to compile, so those count as caught without proving a test. The two survivors, each now caught (d3afeacd): `context.rs:38` a missing usage field counted 1 (`counts_a_usage_field_missing_as_zero`), `totals.rs:51` a negative cached offset read as 1 (`keeps_its_cache_for_one_run_only`). TypeScript, `scripts/library-mcp.ts` under `scripts/library-pages.test.ts`: 3 caught, 1 survived, the default search limit 8 made 9: no action, `library_search` predates F20 and F21's EN-106 rewrites the server with its tests.

### Breadth

| Check | Result |
| --- | --- |
| `check:all` | run by the pre-push of the branch |
| weekly link check | the last run (2026-10-15) failed on two links outside F20: `kanban.university` (502) and `adobe.com` (403, a site refusing robots); the library holds the first, read 2026-10-12; left to the next weekly run |
| B, `@see` | F20 adds tooling only, no `@see`; `REF-CC-HOOKS` read again and `REF-CC-PERMISSIONS` added, both 2026-10-18 |
| E, docs | `docs/tooling/agent.md`, `agent-tools.md`, `commands.md`, `sources.md` and `.claude/rules/tooling.md` updated with each fix |

## G. Sources and questions for the next feature (F21)

- Every hook EN-104 ports: an `if` filter and a `timeout` of its own (`REF-CC-HOOKS`, `REF-CC-PERMISSIONS`); whether a hook should start its built binary rather than `cargo run`, measured in SP-023 against the build lock.
- One shell reader for `guard-bash` and `library-pages`: quotes, here-documents, here-strings, arithmetic shifts. A source to read first: the POSIX Shell Command Language (Quoting, Here-Document), a research request for SP-023.
- The library server's port (EN-106): a fetch that fails before any answer stays recorded `failed`; the default search limit gets its test.
- The test-support crate (EN-103 T1): the eight `clean` helpers, each with an empty home and `GIT_CONFIG_NOSYSTEM`; the JSON number and usage readers of `hooks` and `run-totals` merged with the metrics' rewrite (EN-107).

## H. Light evolvability check

1. **Size**: 14 Rust crates; `AGENTS.md` and `CLAUDE.md` 154 lines together.
2. **Tools**: Claude Code's `if` and `timeout` on hooks now used; nothing else to adopt before F21.
3. **Fitness functions**: the settings' filters and timeouts, each parked story's question and the four rewrites' measures are tested.
4. **Patterns**: the duplicates of A7 wait for F21's test-support crate; the story files are listed once.
5. **Agent configuration**: the `GIT_*` rule says whom it binds; `check-handback`'s reach and auto mode are written.
6. **Process**: no finding needed the Product Owner; each was fixed, deferred with its place, or closed with a reason.

## For the main thread

Rule-docs: none affected, tooling and documentation only, no rule of `docs/domain/` changed.

The `.claude/settings.json` change (A4) takes effect in a new session; it adds `if` filters and 30-second timeouts and removes no hook.
