# Starter kit

Setting up a new project the way this one ended E01. In order; each step names the files to copy and the decision behind them. Adapt, do not copy blindly: remove what the new project does not need (geometry kinds, for instance), and write an ADR for each difference.

## Steps

| #   | Step                                                                                                                                 | Copy from this repository                                                                | Why                                  |
| --- | ------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------- | ------------------------------------ |
| 1   | Repository, license, `main` and `develop`, rulesets (merge commits into `main`, no bypass)                                           | `docs/tooling/ci.md` (settings), `docs/tooling/git-workflow.md`                          | ADR-0038                             |
| 2   | TypeScript strict; ESLint as strict as possible, every rule decided; Prettier; EditorConfig                                          | `tsconfig.json`, `eslint/`, `.prettierrc.json`, `.editorconfig`                          | ADR-0009, ADR-0010                   |
| 3   | Custom lint rules for the project's structure (one export per file, barrels, kinds, sources)                                         | `eslint/plugin/`                                                                         | ADR-0011, ADR-0012, ADR-0019         |
| 4   | Vitest, 100 % coverage of the library, a floor for the demo app, property tests, fitness functions                                   | `vite.config.ts`, `vitest.setup.ts`, `tests/`                                            | ADR-0022                             |
| 5   | Dependencies: one documented row each, latest versions at least 7 days old, audit, knip                                              | `tests/package.test.ts`, `.ncurc.cjs`, `knip.json`, `docs/tooling/dependencies.md`       | supply chain, ADR-0038               |
| 6   | Git hooks: lint-staged, commitlint with closed scopes and backlog footers, branch names, full check before push                      | `.husky/`, `.lintstagedrc.json`, `commitlint.config.ts`, `.validate-branch-namerc.json`  | `docs/tooling/git-workflow.md`       |
| 7   | One CI, on the pull request into `main`; CodeQL; weekly link check                                                                   | `.github/workflows/`                                                                     | ADR-0038                             |
| 8   | Releases: release-please, test report attached, docs and demo published on release                                                   | `.github/workflows/release.yml`, `release-please-config.json`                            | `docs/tooling/releases.md`           |
| 9   | Documentation site: VitePress, TypeDoc, Mermaid diagrams                                                                             | `docs/.vitepress/`                                                                       | `docs/tooling/documentation-site.md` |
| 10  | Docs skeleton: domain, ADRs, conventions (writing rules first), references, derivations, research, backlog, process, guide, playbook | `docs/*/README.md`, templates `_*.md`, `docs/conventions/writing.md`                     | this playbook                        |
| 11  | Backlog with tests: ids, statuses, parents' tables, feature frame                                                                    | `tests/backlog.test.ts`, `tests/references.test.ts`                                      | ADR-0017, ADR-0020                   |
| 12  | Agent configuration: `AGENTS.md`, `CLAUDE.md`, rules, skills, expert agents, hooks, status line                                      | `AGENTS.md`, `CLAUDE.md`, `.claude/`                                                     | ADR-0021                             |
| 13  | Metrics and the source library                                                                                                       | `scripts/metrics-*.ts`, `scripts/session-report.ts`, `scripts/library-*.ts`, `.mcp.json` | `docs/tooling/metrics.md`, ADR-0039  |
| 14  | Start-of-project interview, then the first epic                                                                                      | [analysis.md](./analysis.md), `docs/domain/README.md`                                    | principle 6                          |

## Rhythm

| When                     | What                                                                                               |
| ------------------------ | -------------------------------------------------------------------------------------------------- |
| each feature             | refinement questionnaire, spike, stories (a run), audit, validation, release                       |
| each epic                | review with the full guided recheck; retrospective with metrics and one table of every logged item |
| every four to five epics | every rule questioned against how the project went; harden by default                              |
| any time                 | ideas to the improvement log, critiques to the feedback log                                        |

## Read first

| Source                               | Why                                                        |
| ------------------------------------ | ---------------------------------------------------------- |
| `REF-CC-BEST-PRACTICES`              | verification the agent can run; independent reviewers      |
| `REF-CC-MEMORY`, `REF-CC-HOOKS`      | short instruction files; hooks for what must always happen |
| `REF-DIATAXIS`, `REF-NNG-SCANNING`   | pages people scan                                          |
| `REF-SAFE-STORY`, `REF-KANBAN-GUIDE` | story types; flow without sprints                          |
| `REF-GITFLOW`                        | `main` for releases, `develop` for integration             |

## Cases met in E01

| Case                                                    | What the kit does                                               |
| ------------------------------------------------------- | --------------------------------------------------------------- |
| a need understood late (the rectangle was a free shape) | the interview asks early which things are kinds of one thing    |
| waiting for CI after every story                        | local checks; one CI per release                                |
| the same pages read again at every spike                | the source library                                              |
| no idea how long the work took                          | metrics, session reports                                        |
| the Product Owner's words kept in a public repository   | writing rules; decisions in neutral words                       |
| a long guided test accumulating every feature's steps   | a short guided test per feature; the full recheck at the review |
