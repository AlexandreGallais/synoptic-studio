---
id: EN-099
epic: E17
feature: F20
title: The tooling docs written as principles
status: ready
points: 3
---

# E17 · F20 · EN-099 — The tooling docs written as principles

Asked by the Product Owner on 2026-10-17: many rules an agent reads were written close to how a decision was first explained in conversation — its examples, its order, its wording — so an agent has to infer the principle and may doubt where it applies. Each rule is rewritten to state the idea in general terms: the principle, its reason, how to tell a case, what to do on conflict, with an example only where it removes a doubt. The agent is critical of how a decision was explained, gives a wider range of examples when they help, and leaves out what does not matter: shorter docs that serve the reader beat exhaustive ones. Split at F20's refinement into one story per family of pages, each measured in tokens before and after (2026-10-17). A later idea, not for these stories: keep superseded ADRs out of the agent's context.

This story covers `docs/tooling/`.

## Acceptance criteria

- Given each page of `docs/tooling/`, when the pass ends, then every rule reads as a principle with its reason and its limits; dated decision notes stay as the record of when and by whom, never as the rule's body.
- Given an accepted ADR whose wording follows a conversation too closely, when a page applies it, then the ADR is left as written and the page states the principle.
- Given a page or a section that guards nothing an agent needs, when the pass finds it, then it is cut, not rewritten.
- Given the pages before and after, when they are compared, then their length in tokens is written in the story, and no rule was lost: each old rule is traced to its new sentence.

## Tasks

One task = one commit, referenced as `EN-099.Tn`.

- [ ] T1 — Measure the pages in tokens and list their rules (1 h)
- [ ] T2 — Rewrite the pages as principles, each old rule traced (4 h)
- [ ] T3 — Measure again and record the figures (1 h)

## Measures

Words by `wc -w`, characters by `wc -m`, tokens as characters / 2.3, the ratio EN-097 measured on the conventions.

| Page | Words before | Characters before | Tokens before |
| --- | --- | --- | --- |
| `agent-tools.md` | 4,624 | 27,159 | 11,808 |
| `agent.md` | 3,019 | 18,855 | 8,198 |
| `ci.md` | 967 | 6,422 | 2,792 |
| `commands.md` | 1,896 | 12,220 | 5,313 |
| `dependencies.md` | 2,356 | 15,232 | 6,623 |
| `documentation-site.md` | 651 | 4,188 | 1,821 |
| `git-workflow.md` | 1,297 | 8,556 | 3,720 |
| `metrics.md` | 2,909 | 17,096 | 7,433 |
| `README.md` | 156 | 1,190 | 517 |
| `releases.md` | 567 | 3,561 | 1,548 |
| `sources.md` | 809 | 4,932 | 2,144 |
| `versions-and-security.md` | 611 | 4,124 | 1,793 |
| total | 19,862 | 123,535 | 53,711 |

## Rules of the pages

The rules the pages held before the pass, one row each; the reference tables (commands, packages, workflows, hooks, crates, fields) count as one row per table. **Now** names where each one reads after the pass: its page and section, or why it was cut or merged. No rule's meaning changed, except the one the story adds (`GIT_*`, below).

| # | Page | Rule before | Now |
| --- | --- | --- | --- |
| G1 | `git-workflow.md` | `main` holds releases only and takes no push; `develop` integrates; one branch per feature, one per story, named by their patterns | Branches and pull requests, its table, with its reason: what reaches `main` went through a validation and a release |
| G2 | `git-workflow.md` | one commit per task, its id in a `Refs:` footer | Branches and pull requests, "A story" |
| G3 | `git-workflow.md` | a story's pull request goes into its feature branch, rebase merge; `story:finish` in an autonomous run, the Product Owner outside one | Branches and pull requests, "A story" |
| G4 | `git-workflow.md` | a feature joins `develop` from its `VAL` branch by a merge commit, once the Product Owner said the validation is done | Branches and pull requests, "A feature", with its reason |
| G5 | `git-workflow.md` | a release only when asked; `develop` into `main` by merge commit only; release-please's pull request first | Branches and pull requests, "A release", with its reason |
| G6 | `git-workflow.md` | after a release, `develop` fast-forwards to `main` | Branches and pull requests, "After a release"; the four chained commands cut, the guard refusing chains in a worktree |
| G7 | `git-workflow.md` | no long-lived branch but `main`, `develop` and the current feature's | Branches and pull requests, last bullet |
| G8 | `git-workflow.md` | branch names checked by `validate-branch-name` | merged into G7's bullet; the pattern list cut, `.validate-branch-namerc.json` being the source |
| G9 | `git-workflow.md` | `branches:clean` after a feature, a release or a story the Product Owner merged | After a merge |
| G10 | `git-workflow.md` | the logs merge by union; never a sorted table nor front matter; GitHub's merge untried | Logs merged by union, with its test: union fits a file whose lines are independent |
| G11 | `git-workflow.md` | Conventional Commits: types, scopes, 100 characters, breaking changes | Commit messages; the scope list cut and `commitlint.config.ts` named as its one source |
| G12 | `git-workflow.md` | the four footers | Commit messages, its table; the example message cut, the table showing each footer |
| G13 | `git-workflow.md` | the hooks table; installed by `npm install`, a worktree's by `worktree:new`; never bypassed | Hooks, with the reason a worktree needs its own `.husky/_` and what to do when a hook blocks legitimate work |
| C1 | `ci.md` | work branches checked by the local hooks only; nobody waits for a CI | intro, with its reason: a CI per story would wait for checks the pre-push already ran |
| C2 | `ci.md` | the CI runs once per release, merged only when green | intro |
| C3 | `ci.md` | the workflows table | kept; the quality row links the lanes of `check:all` instead of repeating them |
| C4 | `ci.md` | no CI on release-please's pull request; on `main` its jobs skipped by `if:` | merged into C5, as its consequence |
| C5 | `ci.md` | a required workflow never filtered by branch or path | first bullet, as a principle: skip by the job's `if:`, never by the workflow's filter, with its reason |
| C6 | `ci.md` | secrets checked three times | second bullet |
| C7 | `ci.md` | code hygiene is the lint's job | third bullet |
| C8 | `ci.md` | repository settings and rulesets | Repository settings, kept |
| C9 | `ci.md` | Dependabot opens security updates only | Repository settings, last paragraph |
| K1 | `commands.md` | the commands table | kept, each row trimmed of its timings and story ids; the agent's tools gain `context:size` |
| K2 | `commands.md` | `fix` before each commit; `check` not per task; `check:all` at the push | "When to run what", with its reason: `check` repeats what the push runs |
| K3 | `commands.md` | the lanes, the docs lane skipped when nothing it builds changed | The lanes of the full check, with its test: skip only when certain the lane has nothing to check |
| K4 | `commands.md` | every check tool names its targets; a new folder joins them | Explicit targets, with its reason |
| K5 | `commands.md` | one heavy command at a time; its situations; never a lock by hand | One heavy command at a time, with its reason (E15's freeze); two rows of the situations merged (folder or foreign file) |
| K6 | `commands.md` | the merge lock, apart from the heavy one | One heavy command at a time, last paragraph |
| K7 | `commands.md` | the playground imports the public API only | Playground, with its reason |
| K8 | `commands.md` | the declarations fail on an exported constant whose type cannot be read | Build |
| R1 | `releases.md` | the release steps on `develop` | kept, with its reason: `main` never holds a commit `develop` lacks |
| R2 | `releases.md` | docs and tooling wait in `develop` | kept |
| R3 | `releases.md` | merged in the wrong order, a second pull request | kept |
| R4 | `releases.md` | the token expires 2026-12-31 | kept, with why a token rather than `GITHUB_TOKEN` |
| R5 | `releases.md` | below `1.0.0` until the Product Owner calls it stable; the bump table | kept |
| R6 | `releases.md` | the rollback | kept |
| D1 | `documentation-site.md` | the API pages and their checks | Stack; the byte-for-byte comparison with TypeDoc of 2026-10-17 cut to its result, a history note |
| D2 | `documentation-site.md` | the playground built and checked with the site | Playground |
| D3 | `documentation-site.md` | relative links; no double curly braces | Writing pages; "English, one `#` title" cut, said by [writing](../../conventions/writing.md) |
| D4 | `documentation-site.md` | PDF not set up, an ADR when needed | PDF, one sentence; the 2023 plugin's history cut |
| P1 | `dependencies.md` | zero runtime dependency | Policy |
| P2 | `dependencies.md` | pinned exactly | Policy, packages and crates in one bullet, with its reason |
| P3 | `dependencies.md` | one row per package; a test holds it | Policy |
| P4 | `dependencies.md` | adding and removing | Policy, the 7-day release and its notes stated once for every kind |
| P5 | `dependencies.md` | crates, Rust tools and assets follow the same rules | Policy, last bullet; the three restatements under Rust tools, Assets and Crates cut |
| P6 | `dependencies.md` | Inter's licence | Assets, kept |
| P7 | `dependencies.md` | an override goes once its cause is fixed | Overrides, as its principle; the note on a removed override cut, history |
| P8 | `dependencies.md` | every Oxc crate at one version | Crates, with its reason; the per-row versions and release notes cut, `Cargo.toml` holding the versions and the commits the notes read |
| V1 | `versions-and-security.md` | latest version, 7 days old | Latest versions, both reasons stated |
| V2 | `versions-and-security.md` | the upgrade procedure | Upgrading, with its reason: a check passing says nothing of what a release added |
| V3 | `versions-and-security.md` | Actions and Node.js on their latest major | Latest versions, second paragraph |
| V4 | `versions-and-security.md` | no pre-release but VitePress 2.0 alpha | Latest versions, third paragraph |
| V5 | `versions-and-security.md` | any advisory fails, npm and RustSec | Vulnerabilities, first bullet, both merged |
| V6 | `versions-and-security.md` | upgrade, else an override | Vulnerabilities |
| V7 | `versions-and-security.md` | install scripts stay blocked | Vulnerabilities |
| V8 | `versions-and-security.md` | stylelint when SCSS starts | Packages blocked by an advisory; the `markdownlint-cli2` row cut, replaced twice since and blocking nothing |
| S1 | `sources.md` | search the library first | Rules for the agent, 1 |
| S2 | `sources.md` | fetch through `library_get`; `WebFetch` only for an empty page; the hook | Rules for the agent, 2, with its reason; the hook's exceptions live in [agent](../../tooling/agent.md#hooks) |
| S3 | `sources.md` | cite a `REF-*` row | Rules for the agent, 3 |
| S4 | `sources.md` | the database never in a repository | Rules for the agent, 4, with its reason |
| S5 | `sources.md` | freshness | Freshness, kept |
| S6 | `sources.md` | the retrospective reads the uses | Uses; the migration of an older file cut, an implementation detail the code holds |
| A1 | `agent.md` | `docs/` is the source of truth | intro |
| A2 | `agent.md` | the files, skills, agents and hooks tables | kept; the three rule files in one row |
| A3 | `agent.md` | each task kind on its agent type; the fallback; a missed rule into the brief; merges with the main thread | Run's workers, as a principle with its reason and cost; the measured tokens per type cut, kept in EN-065 and research note 0024 |
| A4 | `agent.md` | the progress page | The progress page, kept; its timing measure cut |
| A5 | `agent.md` | expert agents read-only, library first | Expert agents, with its reason; the history of their tools before 2026-10-13 and the before-and-after tokens cut (EN-065) |
| A6 | `agent.md` | one model per type, its reason; precedence | Models, as a principle: the cheapest model that does the task as well |
| A7 | `agent.md` | pushes only for a question or a blocker | Pushes, with its reason |
| A8 | `agent.md` | a second session's four rules; the run picks up `develop` | Parallel sessions, with its reason: the repository is the only shared state |
| A9 | `agent.md` | no cloud agents; a scheduled task only when asked | Cloud agents and scheduled tasks |
| A10 | `agent.md` | the journal before the transcripts | Searching past conversations |
| A11 | `agent.md` | new hooks in Rust; no repeated rule; guards changed by ADR; local settings; changes to `develop` | Rules for this folder, the hook-enforced rule with its reason |
| T1 | `agent-tools.md` | a repeated command becomes a tool | intro, as a principle with its reason; the E14 measures kept as one example, the `check:all` one cut (history, now in commands) |
| T2 | `agent-tools.md` | the tools table and `mutate`'s steps | Tools, kept; `check:all` moved out (commands holds it), the heavy lock reduced to a link |
| T3 | `agent-tools.md` | a tool when its signal shows; the specifications | Specifications for later, as a principle with its reason; the six tables merged into one, the Rust column cut since every new tool is Rust |
| T4 | `agent-tools.md` | every new tool in Rust, reviewed dependencies only | Rust, with ADR-0073's reasons; the E15 measures table cut, the decision being recorded in ADR-0073 |
| T5 | `agent-tools.md` | the crates table | Rust, kept without the story column |
| T6 | `agent-tools.md` | portable, Unix-only behind `cfg(unix)` | Rust, "Portable as a goal" |
| T7 | `agent-tools.md` | the coverage floor, raised never lowered | Rust, "Coverage"; the per-crate figures of 2026-10-17 cut, the failure's table printing them |
| T8 | `agent-tools.md` | a fresh machine; a missing tool fails | Rust, "A fresh machine" |
| T9 | `agent-tools.md` | Rust tests in a repository of their own, `GIT_*` removed | widened by this story to every test: [Tests that run Git](../../tooling/commands.md#tests-that-run-git); Rust, "Tests", links it |
| T10 | `agent-tools.md` | shared files, the guard, its padding exception and limits | Shared files and their rows, with its reason: parallel workers would conflict at each merge |
| T11 | `agent-tools.md` | `rule-docs` at commit; `move-rows` | Shared files and their rows |
| M1 | `metrics.md` | where the data lives, the record of each call | kept; `usage.json` added to the sources it was missing from |
| M2 | `metrics.md` | the report, active time, a moment's story | The report, active time with its reason |
| M3 | `metrics.md` | the status line | Status line; the incidents of 2026-10-14 and 2026-10-15 cut to the behaviour they fixed; ccstatusline's comparison cut to one sentence |
| M4 | `metrics.md` | a run's totals | A run's totals; the measure of 2026-10-17 cut, history |
| M5 | `metrics.md` | the session report with `--all` | Session report, with its reason |
| M6 | `metrics.md` | the retrospective's questions | At each retrospective |
| M7 | `metrics.md` | a long command in the background, watched | Watching a long task, with its reason |
| M8 | `metrics.md` | the limits | Limits, kept |
| — | `commands.md` | (new, 2026-10-17) | Tests that run Git: every test that runs `git`, or a binary that runs `git`, removes every inherited `GIT_*` variable, since the pre-push hook sets `GIT_DIR` |

## For the main thread

For EN-096's `.claude/rules/tooling.md`, which this story does not edit: its `GIT_*` line ("A test running `git` in a temporary repository removes every `GIT_*` variable…") widens to the rule learnt on 2026-10-17, written in [commands](../../tooling/commands.md#tests-that-run-git). Proposed wording: "Every test that runs `git`, or a binary that runs `git`, removes every inherited `GIT_*` variable from the commands it starts: the pre-push hook sets `GIT_DIR`, and a leaked one points the test at the real repository (a `git init --bare` turned it bare). See `docs/tooling/commands.md`, Tests that run Git."
