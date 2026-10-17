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

The rules the pages held before the pass, one row each; the reference tables (commands, packages, workflows, hooks, crates, fields) count as one row per table.

| # | Page | Rule before |
| --- | --- | --- |
| G1 | `git-workflow.md` | `main` holds releases only and takes no push; `develop` integrates; one branch per feature, one per story, named by their patterns |
| G2 | `git-workflow.md` | one commit per task, its id in a `Refs:` footer |
| G3 | `git-workflow.md` | a story's pull request goes into its feature branch, rebase merge; `story:finish` does it in an autonomous run, the Product Owner merges outside one |
| G4 | `git-workflow.md` | a feature joins `develop` from its `VAL` branch by a merge commit, labelled `autonomous` once the Product Owner said the validation is done |
| G5 | `git-workflow.md` | a release only when the Product Owner asks; `develop` into `main` by a merge commit only, since a rebase would break `develop`'s ancestry; release-please's pull request merged into `develop` first |
| G6 | `git-workflow.md` | after a release, `develop` fast-forwards to `main` |
| G7 | `git-workflow.md` | no long-lived branch but `main`, `develop` and the current feature's |
| G8 | `git-workflow.md` | branch names checked by `validate-branch-name` |
| G9 | `git-workflow.md` | `branches:clean` after a feature, a release or a story merged by the Product Owner |
| G10 | `git-workflow.md` | the three logs merge by union; never a sorted table nor a file with front matter; GitHub's merge untried, a conflict there resolved by hand and logged |
| G11 | `git-workflow.md` | Conventional Commits: types, scopes, 100 characters, breaking changes |
| G12 | `git-workflow.md` | footers `Refs:`, `Closes:`, `BREAKING CHANGE:`, `Co-Authored-By:` |
| G13 | `git-workflow.md` | the hooks table; hooks installed by `npm install`, a worktree's by `worktree:new` or `npx husky`; never bypassed |
| C1 | `ci.md` | work branches are checked by the local hooks only; nobody waits for a CI there |
| C2 | `ci.md` | the CI runs once per release, on `develop` into `main`, merged only when green |
| C3 | `ci.md` | the workflows table |
| C4 | `ci.md` | the release pull request runs no CI; on `main` its jobs are skipped by `if:`, reported as success |
| C5 | `ci.md` | a required workflow is never filtered by branch or path: it stays pending |
| C6 | `ci.md` | secrets checked three times |
| C7 | `ci.md` | code hygiene is the lint's job |
| C8 | `ci.md` | the repository settings and rulesets |
| C9 | `ci.md` | Dependabot opens security updates only, into `main`; version updates are the agent's |
| K1 | `commands.md` | the commands table |
| K2 | `commands.md` | `fix` before each commit; `check` not per task; `check:all` at the push |
| K3 | `commands.md` | the lanes of `check:all`, the docs lane skipped when the branch touched nothing it builds |
| K4 | `commands.md` | every check tool names its targets; a new tracked folder joins them, a test fails until it does |
| K5 | `commands.md` | one heavy command at a time, through the heavy lock; its situations; never a lock taken by hand |
| K6 | `commands.md` | the merge lock of `story:finish`, apart from the heavy one |
| K7 | `commands.md` | the playground imports the public API only |
| K8 | `commands.md` | the build's declarations fail on an exported constant whose type cannot be read |
| R1 | `releases.md` | the release steps, the release pull request living on `develop` |
| R2 | `releases.md` | docs and tooling wait in `develop` for the next release |
| R3 | `releases.md` | merged in the wrong order, merge the release pull request then open a second one |
| R4 | `releases.md` | `RELEASE_PLEASE_TOKEN` expires 2026-12-31: renew it before |
| R5 | `releases.md` | below `1.0.0` until the Product Owner calls it stable; the bump table |
| R6 | `releases.md` | the rollback to ADR-0038's flow |
| D1 | `documentation-site.md` | the API pages and their checks, which fail the build |
| D2 | `documentation-site.md` | the playground built and checked with the site |
| D3 | `documentation-site.md` | relative links; never double curly braces |
| D4 | `documentation-site.md` | PDF not set up, an ADR when needed |
| P1 | `dependencies.md` | zero runtime dependency |
| P2 | `dependencies.md` | every development package pinned exactly |
| P3 | `dependencies.md` | one row per package, its reason and the alternatives; a test holds it |
| P4 | `dependencies.md` | adding: alternatives searched, audit, row in the same change, an ADR for a structural choice; removing removes the row |
| P5 | `dependencies.md` | crates, Rust tools and assets follow the same rules; the tools outside npm looked at by hand at each retrospective |
| P6 | `dependencies.md` | Inter's licence: notice and licence beside the data, shipped |
| P7 | `dependencies.md` | an override goes once its cause is fixed, checked at each retrospective |
| P8 | `dependencies.md` | every Oxc crate at one version |
| V1 | `versions-and-security.md` | every package on its latest version at least 7 days old, checked at the push and in the CI |
| V2 | `versions-and-security.md` | upgrading: read what changed, adopt or record new features, adapt to breaking changes, one commit per package with features |
| V3 | `versions-and-security.md` | Actions and Node.js on their latest major, upgraded after reading the notes |
| V4 | `versions-and-security.md` | no pre-release but VitePress 2.0 alpha |
| V5 | `versions-and-security.md` | any advisory fails, npm and RustSec |
| V6 | `versions-and-security.md` | fixing: upgrade the direct dependency, else an override documented |
| V7 | `versions-and-security.md` | install scripts stay blocked |
| V8 | `versions-and-security.md` | stylelint installed when SCSS starts, with an exception list unless `braces` is fixed |
| S1 | `sources.md` | search the library first, read sections |
| S2 | `sources.md` | fetch through `library_get`; `WebFetch` only for a page that comes back empty; the hook holds it |
| S3 | `sources.md` | cite a `REF-*` row once read |
| S4 | `sources.md` | the database stays on the machine, never in a repository |
| S5 | `sources.md` | freshness of a stored page |
| S6 | `sources.md` | the retrospective reads the uses and asks why |
| A1 | `agent.md` | `docs/` is the source of truth; the configuration adds procedures, pitfalls, guards and tools |
| A2 | `agent.md` | the files, skills, agents and hooks tables |
| A3 | `agent.md` | each task kind runs on its agent type; `general-purpose` when none fits; a rule a typed worker misses goes to its brief or `project-rules`; merges stay with the main thread |
| A4 | `agent.md` | the progress page: written by a tool, one address per run, published at start, merges and stop, naming no person nor path |
| A5 | `agent.md` | expert agents are read-only, library first |
| A6 | `agent.md` | one model per agent type, its reason in its file; the precedence of models |
| A7 | `agent.md` | pushes only for a question or a blocker that prevents finishing the scope |
| A8 | `agent.md` | a second session never touches the run's folder, writes its decisions in the repository, tells the run, removes its worktree; the run picks up `develop` before each story |
| A9 | `agent.md` | no cloud agents; a scheduled task only when the Product Owner asks |
| A10 | `agent.md` | the journal before the transcripts |
| A11 | `agent.md` | a new hook in Rust; a hook-enforced rule not repeated; a guard changed by an ADR, never bypassed; local settings never remove a guard; changes go to `develop` |
| T1 | `agent-tools.md` | a repeated hand-written command becomes a tool, tested and named after what it does |
| T2 | `agent-tools.md` | the tools table and `mutate`'s steps |
| T3 | `agent-tools.md` | a tool is written when a retrospective's metrics show its signal; the specifications for later |
| T4 | `agent-tools.md` | every new script, hook or agent tool in Rust, a crate of the workspace with only reviewed dependencies |
| T5 | `agent-tools.md` | the crates table |
| T6 | `agent-tools.md` | portable where the standard library allows, a Unix-only call behind `cfg(unix)` with its reason |
| T7 | `agent-tools.md` | the coverage floor of the workspace, raised never lowered |
| T8 | `agent-tools.md` | a fresh machine installs the Rust tools; without them the checks fail, never pass |
| T9 | `agent-tools.md` | the Rust tests run in a repository of their own, every `GIT_*` variable removed |
| T10 | `agent-tools.md` | shared files written by the main thread only; the guard, its padding exception and its accepted limits |
| T11 | `agent-tools.md` | `rule-docs` runs at commit; `move-rows` and its table |
| M1 | `metrics.md` | where the data lives, the record of each call |
| M2 | `metrics.md` | the report, active time capped at 5 minutes, a moment's story and task |
| M3 | `metrics.md` | the status line, its states and parts |
| M4 | `metrics.md` | a run's totals |
| M5 | `metrics.md` | the session report closes a session with `--all` |
| M6 | `metrics.md` | the retrospective's questions |
| M7 | `metrics.md` | a command over a minute runs in the background, watched, never behind a waiting loop |
| M8 | `metrics.md` | the limits: worktree sessions not read, transcripts kept 3650 days, metrics per clone |
