---
id: EN-045
epic: E17
feature: F18
title: Every check tool given explicit targets
status: done
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

Each step alone, one after the other, under the heavy-command lock, on the same machine (seconds); the last rows are the slowest steps inside `check:all`, whose lanes run at the same time.

| Step                        | Reads before                                           | Reads after                                                                   | Before | After |
| --------------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------- | -----: | ----: |
| `format:check`              | `.`, minus `.prettierignore` and `.gitignore`          | root files, `.claude/` folders, `.github`, `.husky`, `.vscode`, `docs`, code  |   10.1 |   9.2 |
| `lint`                      | `.`, minus `IGNORED_FILES` (`.claude/worktrees/` read) | root `*.{ts,md}`, `.claude/` folders, `.github`, `docs`, code                 |   48.8 |  46.2 |
| `lint:secrets`              | `**/*`, minus `.secretlintignore` and `.gitignore`     | `git ls-files`                                                                |    0.6 |   0.9 |
| `typecheck`                 | `tsconfig.json` `include` (already explicit)           | unchanged                                                                     |    4.9 |   4.9 |
| `test`                      | Vitest `include` (already explicit), every core        | unchanged, half the cores                                                     |   97.3 | 107.6 |
| `deps:audit`                | `package-lock.json`                                    | unchanged                                                                     |    1.1 |   1.0 |
| `deps:unused`               | knip `project: **/*.ts`, minus `.gitignore`            | knip `project`: root `*.ts`, `.claude/hooks`, `docs/.vitepress`, code folders |    1.1 |   0.9 |
| `deps:tools`                | `.github/workflows/`, `.nvmrc`                         | unchanged                                                                     |    1.7 |   1.3 |
| `deps:outdated`             | `package.json`                                         | unchanged                                                                     |    2.5 |   2.4 |
| `docs:build`                | `docs/` (VitePress), `src/index.ts` (TypeDoc)          | unchanged                                                                     |   29.5 |  29.6 |
| `playground:build`          | `playground/`                                          | unchanged                                                                     |    0.7 |   0.6 |
| `check:all`                 | the steps above in four lanes                          |                                                                               |  122.6 | 112.1 |
| `lint` in `check:all`       |                                                        |                                                                               |  121.9 |  64.2 |
| `test` in `check:all`       |                                                        |                                                                               |  122.4 | 111.9 |
| `docs:build` in `check:all` |                                                        |                                                                               |   71.7 |  43.4 |

Alone, the tests take 10 % longer on half the cores; inside `check:all` they leave the other lanes room, and the whole run is 10 s shorter.

In a checkout holding a worktree (the main one), before `.claude/worktrees/` was ignored by Git (RET-003): `lint` 82.8 s and 963 errors, `format:check` and `deps:unused` failed on the worktree's files, `check:all` 167.4 s.

### Files each tool reads, before and after

Listed with a copy of `src/` and `docs/` in `.claude/worktrees/probe/`, files in `records/` and `coverage/`, on `develop` after RET-003:

| Tool       | Before | After | Left out                                  |
| ---------- | -----: | ----: | ----------------------------------------- |
| ESLint     |  3 373 | 2 004 | 1 369 files of `.claude/worktrees/probe/` |
| Prettier   |  2 086 | 2 086 | none (`.gitignore` already left them out) |
| secretlint |  2 092 | 2 092 | none                                      |
| `tsc`      |  1 541 | 1 541 | none                                      |
| Vitest     |    477 |   477 | none                                      |
| knip       |  1 559 | 1 559 | none                                      |

The new test file is counted in neither column. No file outside the ignored folders left any tool.

## Tasks

One task = one commit, referenced as `EN-045.Tn`.

- [x] T1 — The steps of `check:all` measured, what each reads written here (1 h)
- [x] T2 — Each tool given explicit targets, the ignored folders everywhere, Vitest at half the cores, a test that no tracked file drops out (2 h)
- [x] T3 — Measured again; `docs/tooling/` and the tooling rules updated (1 h)
