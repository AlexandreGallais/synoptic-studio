---
id: AUD-020
epic: E17
feature: F18
title: Audit F18
status: done
points: 1
---

# E17 · F18 · AUD-020 — Audit F18

Audit F18 in depth, the project in breadth ([audit.md](../../conventions/audit.md)).

## Acceptance criteria

- Given F18's stories merged, when the audit ends, then each criterion is traced to a test, the tools are mutation-checked, and the findings are fixed or logged.

## Tasks

One task = one commit, referenced as `AUD-020.Tn`.

- [x] T1 — Criteria traced to the Rust tools, scripts, hooks and lint rules; `backlog-index --check` tested on a backlog of its own (1 h)
- [x] T2 — Test gaps of the reviews closed: exceptions in Rust, a synthetic 200 MB buffer, boundaries and selectors (1 h)
- [x] T3 — Skills and tooling pages say the flow of worktrees, `story:finish` and the release (1 h)
- [x] T4 — The agent tools page, the other pages and the done stories cleaned (1 h)
- [x] T5 — The agent limit said as it is, tables moved to F19 by an ADR, accepted limits written (1 h)
- [x] T6 — The findings, the next feature's sources and the evolvability check (30 min)

## Findings

Three independent reviews on 2026-10-17 (`auditor` A, `verifier` V, `doc-editor` D, T, O, S), each finding reproduced before it was recorded. A pull request number marks a fix made in parallel on a `fix/aud-020-…` branch and merged into the feature branch: #285 (the heavy lock), #286 (`story:finish`, `branches:clean`, `move-rows`, the hooks).

### Auditor

| Id | Severity | Finding | Status | Where |
| --- | --- | --- | --- | --- |
| A1 | high | `limit-agents` not in `.claude/settings.json`; `AGENTS.md` said a fourth subagent "refused" | fixed: said active once its line is in place, counted by hand until then; F18.AC2 in force with EN-063, waiting for the Product Owner | AUD-020.T3, EN-063 |
| A2 | high | the criteria trace read only TypeScript tests of three folders: eight of F18's criteria untraced once it is `done` | fixed: scripts, hooks, lint rules and Rust tests read; simulation with F18 `done` lists none (eight before) | AUD-020.T1 |
| A3 | medium | F18.AC4 still asked for tables without padding; ADR-0065 decision 1 neither done nor superseded | fixed: ADR-0068, AC4 reworded, F19's criterion | AUD-020.T5 |
| A4 | medium | heavy lock: two waiters could both take over a dead lock; a reused process id kept it held | fixed: `flock` on the lock file | #285 |
| A5 | medium | `story:finish` committed the whole index of the base checkout | fixed | #286 |
| A6 | medium | `story:finish` not resumable after a failure past the merge | fixed | #286 |
| A7 | low | `move-rows` appends to the newest journal section whatever its date | fixed in the tool; the 2026-10-17 section opened by hand | #286, AUD-020.T4 |
| A8 | low | a foreground subagent stayed counted as running | fixed; its transcript line not yet seen for real (N4) | #286 |
| A9 | low | `branches:clean` could remove a fresh worktree with no commits of its own | fixed | #286 |
| A10 | low | AC10 untagged; the workers test tagged AC3 for AC20's clause | fixed: `[F15.AC4] [F18.AC10]`, `[F18.AC20]` | AUD-020.T1 |
| A11 | low | done stories with unticked tasks | fixed (EN-048's T1 stays unticked: moved to F19) | AUD-020.T4 |
| A12 | low | duplicates: story-id parsing ×4, short duration ×2, the 12-line limit ×2, the tasks twin | logged for F19's crate layout; the 12-line limit tied by a test of `val-prepare` | AUD-020.T5 |
| A13 | low | `merge=union` untried on a GitHub merge | documented: guards local merges until a GitHub merge shows it (`git-workflow.md`, note 0022) | AUD-020.T3, T4 |
| A14 | low | some full Vitest runs escaped `guard-bash` | fixed | #286 |
| A15 | low | the shared-files guard can be bypassed by another branch name or a push to the feature branch | accepted limit, written in `agent-tools.md`: a guard against mistakes, not intent | AUD-020.T5 |

### Verifier

| Id | Severity | Finding | Status | Where |
| --- | --- | --- | --- | --- |
| V1 | high | same as A2 | fixed | AUD-020.T1 |
| V2 | high | `heavy-lock` binary never run across a linked worktree | fixed | #285 |
| V3 | medium | lock tests timed by sleeps | fixed | #285 |
| V4 | medium | `backlog-index --check` never seen to fail | fixed: a backlog of its own, a title changed → exit 1, row 4 and `npm run backlog:index` named, index unchanged | AUD-020.T1 |
| V5 | medium | the register read `.ts` only and needed `--`; `tasks.rs:248` unregistered | fixed: Rust `allow` read, a directive without reason counted, the row added | AUD-020.T2 |
| V6 | medium | the 192 MB case ran on macOS only | fixed: a synthetic part of 200 MB on every machine, 20 s limit | AUD-020.T2 |
| V7 | medium | heavy scripts and `check:rust`'s crate list untested | fixed | #286 |
| V8 | medium | the pre-push golden line tested in a plain repository only | fixed | #286 |
| V9 | medium | same as A3 | fixed | AUD-020.T5 |
| V10 | low | the checklist's flex direction unchecked | fixed: a row, the stage first | AUD-020.T2 |
| V11 | low | US-061's tests read attributes only | fixed | #286 |
| V12 | low | a done commit with a checklist file not refused in a test | fixed | #286 |
| V13 | low | branch deletion and a remote ref of another name untested | fixed | #286 |
| V14 | low | an empty lock file waited forever | fixed | #285 |
| V15 | low | `epic:check` boundaries (2 %, one hour) unchecked | fixed: 1 of 50 accepted, 1 of 49 not; 3 600 000 ms not lost | AUD-020.T2 |
| V16 | low | a spread in a conditional's branch, or held in a variable, not reported | fixed for the branch (it found one fold in `scripts/library-chunks.ts`, rewritten); the variable case a known limit, tested | AUD-020.T2 |
| V17 | low | `val:prepare`'s import lines unchecked; temporary folders left | fixed for `val-prepare` (import lines, removed on drop); `worktree`'s tests leave theirs only on a panic: no action, the test fails | AUD-020.T2 |
| V18 | low | the Inter test passed if both values were missing | fixed: the face looked for first | AUD-020.T2 |

### Documentation

| Id | Severity | Finding | Status | Where |
| --- | --- | --- | --- | --- |
| D1–D3 | medium | the story skill switched branches by hand, opened pull requests by hand, cleaned by hand | fixed: `worktree:new`, `story:finish` in a run, a pull request without the label outside one | AUD-020.T3 |
| D4 | medium | who makes the `done` commit | fixed: the worker stops before it; the main thread makes it, then `story:finish` (run skill, brief) | AUD-020.T3 |
| D5–D7 | medium | the run skill switched the main checkout; release pull request "merges itself" | fixed: feature branch pushed from `origin/develop`, merges in a worktree, ADR-0067's release | AUD-020.T3 |
| D8–D11 | medium | `git-workflow.md`: feature merged by the Product Owner, branches by hand, release after each feature | fixed | AUD-020.T3 |
| D12–D14 | low | `agent.md`: worktree base, clean-up by hand, auditor per story | fixed | AUD-020.T3 |
| D15 | high | same as A1 | fixed | AUD-020.T3 |
| D16 | low | `AGENTS.md`'s state stale, one block of 25 lines | fixed: a table per epic | AUD-020.T3 |
| D17 | low | `mutate` missing from the heavy-lock lists | fixed | AUD-020.T3 |
| D18 | medium | VAL-019 lacked three decisions to confirm | fixed: ADR-0067 decision 2, ADR-0068, EN-062's macOS-only tests | AUD-020.T4 |
| D19 | medium | EN-063 lacked the resumed subagent | fixed: a criterion; EN-063 stays `ready` for the Product Owner | AUD-020.T4 |
| D20 | low | `ci.md` lacked `check:rust`, knip, the playground build | fixed; the runner's Rust tools logged as untried | AUD-020.T3 |
| T1–T5 | low | `agent-tools.md`: contradicting sentence, built tools as specifications, eight paragraphs, old figure, long cell | fixed: one crate table with Does, `move-rows` and `mutate` tables | AUD-020.T4 |
| O1–O3 | low | releases list, "give back", worktrees "in the scratchpad" | fixed | AUD-020.T4 |
| O4–O5 | low | the register's opening sentence; 191 against 192 MB | fixed: a table of directives; the table 191 MB, the file 192 MB | AUD-020.T2 |
| O6–O7 | low | F18's stale intro; AC4 | fixed | AUD-020.T4, T5 |
| O8 | medium | 2026-10-17 rows under 2026-10-16 | fixed by hand: a dated section, inline dates dropped; the tool fixed by A7 | AUD-020.T4 |
| O9–O11 | low | the improvement log: a fact twice, story ids as sources, a false hint | fixed: rows merged, the agent as source, `guard-bash`'s hint | AUD-020.T4, #286 |
| O12–O17 | low | note 0022, SP-020, CHK-019, EN-051, EN-046 not saying what was answered | fixed | AUD-020.T4 |
| S1 | low | same as A11 | fixed | AUD-020.T4 |
| S2 | low | rows never moved from six done stories | fixed: written into the logs, removed from the stories | AUD-020.T4 |

### Found at merge

Found while #285 and #286 merged into the feature branch, each reproduced.

| Id | Severity | Finding | Status | Where |
| --- | --- | --- | --- | --- |
| N1 | medium | `story:finish`'s split files imported each other without `.ts`: Node.js failed at its start, Vitest resolved them and passed | fixed (the imports); `tests/node-run-imports.test.ts` refuses a relative import without `.ts` in `scripts/` and `.claude/hooks/`, mutation-checked; logged | feature branch, AUD-020.T6 |
| N2 | low | `node scripts/story-finish.ts` without arguments rebased on `origin/undefined` | fixed: without its three arguments it prints its usage and stops; logged | AUD-020.T6 |
| N3 | low | the playground's `text-font` change handler is not guarded while the font list is held; unreachable, the list is disabled meanwhile | logged | improvement log |
| N4 | low | A8's foreground subagent: its transcript line's shape not confirmed from a real transcript | logged for F18's VAL | improvement log |

### Mutation spot-checks (A4)

Each new test broken once on purpose, then restored (`git status` clean):

| Mutation | Failing test |
| --- | --- |
| `backlog-index`: `want == have` made `true` | `f18_ac5_check_refuses_a_title_changed_in_its_file_and_names_the_row` |
| the trace reads the old checker (F18 set `done` in the working copy) | `traces every criterion…`: AC2, AC3, AC5, AC7, AC8, AC9, AC15, AC16 untraced |
| the trace ignores Rust test names | `traces every criterion…` (AC2, AC3, AC15) and `reads a Rust test named…` |
| the page's body `flex-direction: column` | `[F18.AC12] … is a column of the page right of the drawing` |
| `epic-check`: `<=` made `<`; `ageMs > STALE_MS` made `>=` | `accepts exactly 2 % of failed calls…` (and the first failed-calls test) |
| the conditional-branch selectors removed | two `[F18.AC18] reports a spread in a conditional…` cases |
| the register's directive needs `--` again | `[F18.AC13] reads a directive's rules and a row's cells` |
| `facePartsToFile` writes each part one byte later | `[F18.AC13] writes a part of 200 MB…` |
| `val-prepare` imports `./checklist-lines` | `f18_ac12_prepares_a_new_epic_checklist_and_its_address` |
| `story-finish.ts` imports `./story-finish-rows` without `.ts` | `scripts/story-finish.ts names the .ts file of each relative import` |
| `val-prepare`'s `LINE_LIMIT` made 13 | `warns_at_the_limit_the_playground_holds`, `warns_above_twelve_criteria…` |

Sources re-verified by the auditor: `REF-INTER-REGULAR-FILE` (archive and file digests, licence), `REF-RELEASE-PLEASE-SOURCE`, `REF-CC-SUBAGENTS`, `REF-CC-ENV-VARS`. No copied code: every crate is std-only.

## G. Sources and questions for F19's spike (SP-021)

- Oxc's formatter on Markdown: does it print tables without padding and keep paragraphs as written (ADR-0068, F19's first criterion)? Read its options and try it on `docs/`.
- `oxc/no-accumulating-spread` (`REF-OXLINT-ACCUMULATING-SPREAD`) against the project's selector: a spread in a conditional's branch and one held in a variable (the selector's known limit).
- A shared crate module, or a Cargo workspace, for what the tools repeat: the story id, a short duration, the JSON reader, the transcript's tasks (A12).
- `flock(2)` on Linux and macOS for the heavy lock (`REF-MACOS-FLOCK`; the Linux manual page to read) if the CI runs heavy commands.
- GitHub's Ubuntu runner: `cargo`, `clippy` and `rustfmt` present for `check:rust` (the image's software list, to read), seen at the first pull request into `main`.
- Whether GitHub's merge applies `merge=union`: tried at the first merge bringing log rows from two sides.

## H. Light evolvability check

1. **Size**: 8 Rust crates beside the TypeScript; `AGENTS.md` and `CLAUDE.md` 163 lines together, under 200.
2. **Tools**: Stryker still blocked; `npm run mutate` covers it; nothing new to adopt before F19.
3. **Fitness functions**: the criteria trace now reads Rust and tooling tests; the register reads Rust `allow`; the 12-line limit is tied across languages.
4. **Patterns**: the duplicates of A12 logged for F19's crate layout.
5. **Agent configuration**: the skills, the brief and the tooling pages now say the worktree flow; the agent limit waits for EN-063 in the settings.
6. **Process**: AUD with three reviewers in parallel and two fixing workers found 77 findings in one pass; the improvement log names the agent as source.
