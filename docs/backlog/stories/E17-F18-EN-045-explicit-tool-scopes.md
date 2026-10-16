---
id: EN-045
epic: E17
feature: F18
title: Every check tool given explicit targets
status: in-progress
points: 2
---

# E17 · F18 · EN-045 — Every check tool given explicit targets

Give each check tool of `check:all` the folders and files it must check, by name, instead of the whole repository (`.`, `**/*`), and limit Vitest to half the machine's cores: faster checks, less CPU, and no tool walking into the copies of the repository that agents and parallel sessions keep in `.claude/worktrees/`. Asked by the Product Owner on 2026-10-16, during RET-003, which placed it in F18 (F18.AC20).

## Acceptance criteria

- Given Prettier, ESLint, secretlint, knip, `tsc` and Vitest, when `check:all` runs, then each reads only targets named in its script or configuration, never `.` nor `**/*`.
- Given `.claude/worktrees/`, `records/`, `coverage/`, `dist/`, `docs/api/`, `docs/.vitepress/{.temp,cache,dist}` and `node_modules`, when a tool runs, then it reads none of their files.
- Given the files each tool read before the change, when they are compared with the files it reads after it, then the only ones missing are in the folders above.
- Given Vitest, when it runs, then it uses at most half the machine's cores (`maxWorkers: "50%"`).
- Given a new tracked folder, when no target names it, then `tests/check-targets.test.ts` fails.

## Measures

Each step alone, one after the other, under the heavy-command lock, on the same machine (seconds):

| Step               | What it reads before                                           | Before |
| ------------------ | -------------------------------------------------------------- | -----: |
| `format:check`     | `.`, minus `.prettierignore` and `.gitignore`                  |   10.1 |
| `lint`             | `.`, minus `IGNORED_FILES` (`.claude/worktrees/` read)         |   48.8 |
| `lint:secrets`     | `**/*`, minus `.secretlintignore` and `.gitignore`             |    0.6 |
| `typecheck`        | `tsconfig.json` `include` (already explicit)                   |    4.9 |
| `test`             | Vitest `include` (already explicit), every core                |   97.3 |
| `deps:audit`       | `package-lock.json`                                            |    1.1 |
| `deps:unused`      | knip `project: **/*.ts`, minus `.gitignore`                    |    1.1 |
| `deps:tools`       | `.github/workflows/`, `.nvmrc`                                 |    1.7 |
| `deps:outdated`    | `package.json`                                                 |    2.5 |
| `docs:build`       | `docs/` (VitePress), `src/index.ts` (TypeDoc)                  |   29.5 |
| `playground:build` | `playground/`                                                  |    0.7 |
| `check:all`        | the steps above in four lanes; `lint` 121.9 and `test` 122.4 s |  122.6 |

In a checkout holding a worktree (the main one), before `.claude/worktrees/` was ignored by Git (RET-003): `lint` 82.8 s and 963 errors, `format:check` and `deps:unused` failed on the worktree's files, `check:all` 167.4 s.

## Tasks

One task = one commit, referenced as `EN-045.Tn`.

- [x] T1 — The steps of `check:all` measured, what each reads written here (1 h)
- [ ] T2 — Each tool given explicit targets, the ignored folders everywhere, Vitest at half the cores, a test that no tracked file drops out (2 h)
- [ ] T3 — Measured again; `docs/tooling/` and the tooling rules updated (1 h)
