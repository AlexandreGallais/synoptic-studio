---
id: EN-091
epic: E17
feature: F19
title: Every configuration file reviewed for speed, type errors reported with the lint
status: ready
points: 3
---

# E17 · F19 · EN-091 — Every configuration file reviewed for speed, type errors reported with the lint

Go over every configuration file of the repository — `vite.config.ts` and Vitest's setup, `tsconfig*.json`, `knip.json`, `.ncurc.cjs`, `.lycheeignore`, `.secretlintrc.json`, `.validate-branch-namerc.json`, `release-please-config.json`, `.oxfmtrc.json`, the oxlint configuration, `Cargo.toml`, `.cargo/config.toml`, `rust-toolchain.toml`, `.github/workflows/*` — and make each tool it drives as fast as it can be (ADR-0069). Asked by the Product Owner on 2026-10-17.

Also asked on 2026-10-17, as at the project's start: what TypeScript checks is reported by the linter's pass where it can be, so that a type error never stops the agent half-way through an idea but is listed with the lint findings. Oxlint does not type-check by itself; its type-aware mode runs `tsgolint` on TypeScript 7, which may report the compiler's diagnostics in the same pass — to verify from its documentation and a try here.

## Acceptance criteria

- Given each configuration file, when it is reviewed, then each setting that costs time is listed with its measured effect, and kept with its reason or changed; the list is in the story, with the timings before and after of the tools concerned.
- Given Vitest's setup, when the review ends, then what only some tests need (for instance `@typescript-eslint/rule-tester`, loaded today into every test file, 36 % of the test lane by EN-089's measure) is loaded only by them, and the test lane's time is written before and after.
- Given oxlint's type-aware mode, when its documentation is read and it is tried here, then the story says whether it reports TypeScript's own errors; if it does and matches `tsc --noEmit` on the tree and on a planted error, `tsc` leaves `check:all` and the type errors come with the lint; otherwise `tsc` stays, with the reason and the source.
- Given `tsconfig*.json`, when the review ends, then each file left is used by a tool named in it, and the others are removed.

## Tasks

One task = one commit, referenced as `EN-091.Tn`.

- [ ] T1 — The review: every file, its settings, their cost; timings before (2 h)
- [ ] T2 — Vitest's setup split by need; the changes kept; timings after (2 h)
- [ ] T3 — Type errors with the lint, or `tsc` kept with its reason; `tsconfig` files trimmed (2 h)

## Review

Each configuration file, the settings in it that cost time, their effect measured on this Mac on 2026-10-17 (alone unless said otherwise), and what was done. A file without such a setting is listed with its tool's time.

| File | Setting that costs time | Measured | Decision |
| --- | --- | --- | --- |
| `vite.config.ts` | `setupFiles` at the root: `vitest.setup.ts`, which loads `@typescript-eslint/rule-tester`, run before each of the 492 test files (`REF-VITEST-SETUPFILES`), though only the 10 tests of `eslint/plugin/rules/` use it | setup 14 % of a 39.6 s test lane | changed (T2): a project `rules` of its own holds the setup; the other files never load it |
| `vite.config.ts` | coverage (`v8`) on every `npm run test` | see Timings | kept: the 100 % gate of `src/` and the playground's floor are checks of the definition of done |
| `vite.config.ts` | `maxWorkers: "50%"` | — | kept: worktrees testing at once share the Mac, and the other lanes of `check:all` run beside the tests (EN-045, EN-047) |
| `vite.config.ts` | `isolate: false` for all but the playground; `globalSetup` reading the fonts once | 104.5 s → 37.4 s (EN-090), fonts read once (EN-089) | kept |
| `vitest.setup.ts` | the RuleTester's import | as above | kept, given to the `rules` project only (T2) |
| `tsconfig.json` | `skipLibCheck: true` | without it, `tsc` reports errors inside `node_modules` (`@docsearch/js` imports `react`, `preact`, `search-insights`, absent) | kept: the libraries' own declarations are not ours to check |
| `tsconfig.json` | `include` by folder, `types` listed | `tsc --noEmit` 0.67 to 0.70 s, 3 runs | kept: the program holds only our files and the two type packages |
| `tsconfig.build.json` | a second `tsc` run in `npm run build`, for the declarations only | 0.63 s, 3 runs, 589 files | removed (T4): Oxc writes the same declarations in 0.06 s |
| `knip.json` | explicit `entry` and `project`; no `--cache` | 0.81 to 1.03 s; 0.50 to 0.83 s with `--cache` | kept without cache: about 0.3 s, in a lane that is not the slowest, against a cache that may keep a stale result |
| `.ncurc.cjs` | `cooldown: 7`: the publication time of each package read from the registry | `deps:outdated` 3.1 s in `check:all` | kept: the network is the cost, and the 7-day rule is ADR-0038's |
| `.lycheeignore` | — (the weekly CI only) | no local run | kept |
| `.secretlintrc.json`, `.secretlintignore` | the recommended preset over 2 296 tracked files | 0.91 s alone, 3.4 s in `check:all` | kept |
| `.validate-branch-namerc.json` | one regular expression; the hook starts it through `npx` | 0.26 s through `npx`, 0.04 s direct | changed (T2): the Git hooks call `validate-branch-name`, `lint-staged` and `commitlint` directly, Husky putting `node_modules/.bin` on the `PATH` (`REF-HUSKY-PATH`): about 0.2 s per hook |
| `release-please-config.json` | — (the release workflow only) | no local run | kept |
| `.oxfmtrc.json` | `ignorePatterns` | `format:check` 1.1 s in `check:all` | kept |
| `.oxlintrc.json` | `typeAware: true` | oxlint 0.76 to 0.81 s on the tree | kept; not edited here (EN-075); the type check joins it by the command line (T3) |
| `Cargo.toml` | one workspace, one `target/`, exact Oxc versions | — | kept; two Oxc crates added for the declarations (T4) |
| `.cargo/config.toml` | `warnings = "deny"` | no time | kept |
| `rust-toolchain.toml` | the Windows target for `check:rust`'s check | 0.11 s warm | kept |
| `package.json` `rust:coverage` | `cargo llvm-cov` cleans the workspace's coverage artifacts before each run | 13.5 s; 9.3 s with `--no-clean` | kept: its README warns that old artifacts give "false positives/false negatives" (`REF-CARGO-LLVM-COV`), and the coverage floor is a gate |
| `package.json` `typecheck` in `check:all` | a second type check beside oxlint's | `tsc` 0.67 to 0.70 s | changed (T3): oxlint's `--type-check` reports the same errors in the lint pass |
| `.lintstagedrc.json` | oxlint on the staged files | 0.28 s for one file, with or without `--type-check` | changed (T3): the type errors of the staged files come with their lint |
| `.github/workflows/ci.yml` | `cargo install` of `cargo-llvm-cov` and `cargo-deny` from source on each run | CI only, minutes, no local wait (ADR-0038) | kept; noted for the main thread |
| other workflows | — | CI only | kept |
| `docs/.vitepress/config.ts`, `eslint.config.ts` | the site build (54.4 s in `check:all`), ESLint (49.3 s) | — | not changed: EN-085 builds the site on change and removes ESLint; EN-075 replaces its rules |

## Timings

Before (`1dcfa40b`), 2026-10-17. `npm run test` alone: 39.6 s (tests 52 %, import 15 %, setup 14 %, transform 13 %). `npm run check:all`, its lanes:

| Check | Time |
| --- | --- |
| test | 61.8 s |
| check:rust | 60.3 s |
| docs:build, playground:build | 54.4 s, 0.7 s |
| lint | 49.3 s |
| format:check, lint:secrets, typecheck, deps:audit, deps:unused, deps:tools, deps:outdated | 1.1, 3.4, 1.4, 1.1, 1.4, 1.3, 3.1 s |
| `check:all` | 61.8 s |
