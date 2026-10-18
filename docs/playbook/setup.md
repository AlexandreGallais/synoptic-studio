# Starter kit

Setting up a new project the way this one ended E01. In order; each step names the files to copy and the decision behind them. Adapt, do not copy blindly: remove what the new project does not need (geometry kinds, for instance), and write an ADR for each difference.

## Steps

| # | Step | Copy from this repository | Why |
| --- | --- | --- | --- |
| 1 | Repository, license, `main` and `develop`, rulesets (merge commits into `main`, no bypass) | `docs/tooling/ci.md` (settings), `docs/tooling/git-workflow.md` | ADR-0038 |
| 2 | TypeScript strict; oxlint as strict as possible, every rule decided (ESLint until EN-085); one formatter (oxfmt, tables without padding); EditorConfig | `tsconfig.json`, `.oxlintrc.json`, `.oxfmtrc.json`, `.editorconfig` | ADR-0009, ADR-0010 |
| 3 | Custom lint rules for the project's structure (one export per file, barrels, kinds, sources) | `tools/lint/` (a Rust linter on Oxc's crates, one module per rule) | ADR-0011, ADR-0012, ADR-0019 |
| 4 | Vitest, 100 % coverage of the library, a floor for the demo app, property tests, fitness functions | `vite.config.ts`, `tests/` | ADR-0022 |
| 5 | Dependencies: one documented row each, latest versions at least 7 days old, audit, unused code | `tests/package.test.ts`, `tools/deps/`, `docs/tooling/dependencies.md` | supply chain, ADR-0038 |
| 6 | Git hooks: the staged files formatted and linted, Conventional Commits with closed scopes and the story's footer, branch names, full check before push | `.githooks/`, `tools/git-hooks/` (EN-108) | `docs/tooling/git-workflow.md` |
| 7 | One CI, on the pull request into `main`; CodeQL; weekly link check | `.github/workflows/` | ADR-0038 |
| 8 | Releases: the project's release tool, a release branch, the test report attached, docs and demo published on release | `.github/workflows/release.yml`, `tools/release/` | `docs/tooling/releases.md` |
| 9 | Documentation site: VitePress, API pages from the TSDoc (a Rust generator on Oxc's parser here, TypeDoc before EN-072), Mermaid diagrams | `docs/.vitepress/` | `docs/tooling/documentation-site.md` |
| 10 | Docs skeleton: domain, ADRs, conventions (writing rules first), references, derivations, research, backlog, process, guide, playbook | `docs/*/README.md`, templates `_*.md`, `docs/conventions/writing.md` | this playbook |
| 11 | Backlog with tests: ids, statuses, parents' tables, feature frame | `tests/backlog.test.ts`, `tests/references.test.ts` | ADR-0017, ADR-0020 |
| 12 | Agent configuration: `AGENTS.md`, `CLAUDE.md`, rules, skills, expert agents, hooks, status line | `AGENTS.md`, `CLAUDE.md`, `.claude/` | ADR-0021 |
| 13 | Metrics and the source library | `tools/metrics/`, `tools/library/`, `.mcp.json` | `docs/tooling/metrics.md`, ADR-0039 |
| 14 | Start-of-project interview, then the first epic | [analysis.md](./analysis.md), `docs/domain/README.md` | principle 6 |

## Rhythm

| When | What |
| --- | --- |
| each feature | refinement questionnaire, spike, stories (a run), audit, validation, release |
| each return of the Product Owner | a review of what was finished since the last merge into `main`, its naming pass, and the method with metrics and one table of every logged item and tool card (ADR-0086) |
| every four to five epics | every rule questioned against how the project went; harden by default |
| any time | ideas to the improvement log, critiques to the feedback log |

## Read first

| Source | Why |
| --- | --- |
| `REF-CC-BEST-PRACTICES` | verification the agent can run; independent reviewers |
| `REF-CC-MEMORY`, `REF-CC-HOOKS` | short instruction files; hooks for what must always happen |
| `REF-DIATAXIS`, `REF-NNG-SCANNING` | pages people scan |
| `REF-SAFE-STORY`, `REF-KANBAN-GUIDE` | story types; flow without sprints |
| `REF-GITFLOW` | `main` for releases, `develop` for integration |

## Starting parallel agents (E15)

Copy before the first run with parallel workers:

| Setting | Why |
| --- | --- |
| `.claude/worktrees/` in `.gitignore`, and out of the IDE's indexing with `coverage/`, `dist/`, `docs/api/` | the IDE indexed each worktree as a new project (256 % of the CPU) |
| a lock for heavy commands in `.git/`, holding its owner's process id | one `check:all` saturates a laptop; a lock left by a force-quit is taken over |
| a `PreToolUse` hook on `Agent` limiting subagents | a sentence was bent once, with a freeze |
| Markdown tables without padding (`tools/format`, ADR-0068); generated indexes; `merge=union` on append-only logs | padded tables rewrite whole and conflict on every parallel merge |
| the worker brief versioned in the run skill, naming the commands that fit a worktree | 549 compound commands refused in worktrees |
| a register of exceptions to the rules, checked | each exception visible and justified |

## Fast tools for the agent (E17)

| Setting | Why |
| --- | --- |
| every tool around the library in Rust, called by its binary (`target/release/<binary>`), rebuilt after each sync and merge | 3 ms a hook against 50 to 150 ms through `cargo run` or `npm run` (ADR-0078) |
| one linter (oxlint, then the project's rules in Rust on Oxc's crates) and one formatter, every rule triaged before it is enabled | `check:all` from 170–217 s to about 73 s; 38 dev packages to 13 |
| a worker type with the project's rules preloaded, without `CLAUDE.md` | a story worker's first request from 38,897 to 10,562 tokens (EN-065) |
| the Git sequences a worker repeats as one call each (`git-sequences sync`, `commit`) | Git calls per worker from a median of 12 to 4 |
| one row or one section read by `page`, every page fetched through the library | `docs/references.md` read whole 121 times in three days before |
| a tool that fails during a run written as a card, all fixed together at the next review (ADR-0086) | each fix during a run cost a worktree, a merge and a rebuild of every running story's binaries |

## Cases met in E01

| Case | What the kit does |
| --- | --- |
| a need understood late (the rectangle was a free shape) | the interview asks early which things are kinds of one thing |
| waiting for CI after every story | local checks; one CI per release |
| the same pages read again at every spike | the source library |
| no idea how long the work took | metrics, session reports |
| the Product Owner's words kept in a public repository | writing rules; decisions in neutral words |
| a long guided test accumulating every feature's steps | one checklist page per epic, a line per option; rerun at the review |
| a computer frozen by parallel agents (E15) | the agent limit and the heavy lock in the tools |
| merge conflicts on indexes and logs (E15) | shared files written at merge; generated indexes; unpadded tables |
| a checklist of 50 lines (E15) | at most 12 lines per feature, each tried before the validation |
| a CI red on the release's pull request only (E17) | the CI's last result read before a release |
