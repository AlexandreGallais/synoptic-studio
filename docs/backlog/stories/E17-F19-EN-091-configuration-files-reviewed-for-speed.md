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
- [ ] T4 — The library's declarations written by Oxc's isolated declarations, `tsconfig.build.json` removed (asked by the Product Owner on 2026-10-17 during the story) (2 h)

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

`npm run test` alone, Vitest's own `Duration`:

| Run | Duration | Shares |
| --- | --- | --- |
| before (`1dcfa40b`) | 39.6 s | tests 52 %, import 15 %, setup 14 %, transform 13 % |
| after T2, the RuleTester in the `rules` project only, 3 runs | 29.7 s, 33.6 s, 32.4 s | tests 62 to 68 %, import 18 to 19 %, setup 1 % |
| after T2, without coverage (for the review) | 25.4 s | tests 55 %, import 21 %, transform 13 % |

The setup's share falls from 14 % to 1 %: 6 to 10 s saved. Coverage costs about 7 s of the lane. `tests/vitest-workers.test.ts` now checks that each tracked test file runs in exactly one project and that only `rules` has a setup; widening `shared` back to `eslint/**/*.test.ts` makes it fail, naming the 10 rule tests.

## Type errors with the lint

**Answer: yes.** Oxc has no type checker of its own: its README lists Oxlint, Oxfmt, the parser, transformer, minifier and resolver (`REF-OXC-README`). But oxlint's type-aware mode runs `tsgolint` on typescript-go, TypeScript 7's compiler (`REF-TSGOLINT`), and its `--type-check` flag adds "TypeScript compiler diagnostics" to the lint findings (`REF-OXLINT-CLI`); Oxc's guide says it "can replace a separate `tsc --noEmit` step in CI" (`REF-OXLINT-TYPE-AWARE`). The command line calls it "experimental".

Tried here on 2026-10-17, oxlint 1.87.0 with `oxlint-tsgolint` 7.0.2004 against `tsc` 7.0.2:

| Case | `tsc --noEmit` | `oxlint --type-check` |
| --- | --- | --- |
| the tree | no error | no error, no finding |
| five errors planted in one module: a missing module (TS2307), a string given to a number (TS2322), an index read without `noUncheckedIndexedAccess`'s check (TS2322), `undefined` given to an optional property under `exactOptionalPropertyTypes` (TS2375), a possibly undefined value (TS18048) | the five, at their line and column | the same five, same codes, same places, beside the lint findings they cause (`restrict-plus-operands`, `no-unsafe-argument`) |
| an error in `playground/` (TS2322) | reported | reported, same place |
| two modules declaring one global with two types (TS2403, across files) | reported in the second file | reported, same place |
| `tsconfig.json` given an unknown option and a removed one (`baseUrl`) | TS5023 and TS5102 | the unknown option only; the run stops at the first invalid option, as `tsc` stops before checking the code |

So `tsc` leaves `check` and `check:all`: `npm run lint`, `lint:fix`, `npm run fix` and the pre-commit hook run oxlint with `--type-check`, and a type error comes listed with the lint findings instead of stopping a separate step. The type check costs oxlint about 0.3 s on the tree (0.76 to 0.81 s without it, 1.08 to 1.17 s with it, 3 runs each) against 0.67 to 0.70 s for `tsc` alone. `npm run typecheck` stays as a command of its own, and TypeScript 7 stays a dev package: `tsgolint` brings its own typescript-go, but the editors and `tests/typescript-7.test.ts` use `tsc`. The `tsconfig.json` option that oxlint does not report, `baseUrl`, is one TypeScript 7 removed and this project never had. `tests/type-check-with-lint.test.ts` proves a planted TS2322 reported by oxlint at its place, and that every oxlint pass carries `--type-check`; dropping the flag from its call makes it fail.

`tsconfig.json` stays: `tsc`, oxlint's type-aware mode, Vite and Vitest read it. `tsconfig.build.json` leaves with T4.

## Declarations by Oxc

Asked by the Product Owner on 2026-10-17 during the story: whether Oxc's isolated declarations can write the library's declarations instead of `tsc`, on one condition — the extra typing stays on the exported API (the return types of exported functions, exported constants whose type cannot be read from a plain value), never on local constants or every variable. The Product Owner favours it: an export with its type written makes the published API an explicit interface, which also shows a breaking change in the diff of the export itself.

The trial, `isolatedDeclarations` turned on for `src/` (`tsc -p tsconfig.build.json --isolatedDeclarations`), then Oxc's `oxc_isolated_declarations` 0.153.0 on the same files:

| Kind of error | In `src/` | Where |
| --- | --- | --- |
| exported function without its return type (TS9007) | 0 | the rule already asks for it (`explicit-module-boundary-types`) |
| exported constant whose type is not read from its value (TS9010) | 2 | `src/math/angle/degrees-per-half-turn.ts` (`DEGREES_PER_TURN / 2`), `src/geometry/round-shape/quarter-circle-handle.ts` (a call) |
| other (locals, variables, default exports) | 0 | — |

Two annotations, both on exports: the condition holds, so the declarations are Oxc's (ADR-0071). Outside `src/`, which publishes no declarations, the rule would ask 87 annotations (64 in `eslint/`): it is not turned on there.

| Declarations of the 589 modules | Time, 3 runs | Same as `tsc` |
| --- | --- | --- |
| `tsc -p tsconfig.build.json` | 0.63 s | — |
| `tools/declarations` (Oxc) | 0.06 to 0.07 s | 103 files byte for byte; all 589 once whitespace is ignored, but for a type parameter list wrapped over lines and `.5` written for `0.5`: the same in meaning |

`npm run build` runs `cargo run --release -q -p declarations -- src dist/types`; `tsconfig.build.json` is removed. `tools/declarations` refuses a module that breaks the rule, naming its file, line and TypeScript's code, and writes nothing; its six tests run it on libraries of their own (a mutation keeping the output on an error, and one keeping the test files, each fail them). `tests/declarations.test.ts` runs it on `src/`, so that an export missing its type fails the test lane, not only the CI's build; removing the annotation of `DEGREES_PER_HALF_TURN` makes it fail with TS9010 at line 4.

## Timings after

`npm run check:all` alone under the heavy lock, the four tasks applied, 2026-10-17:

| Check | Before | After |
| --- | --- | --- |
| test | 61.8 s | 51.5 s |
| check:rust | 60.3 s | 44.1 s |
| docs:build, playground:build | 54.4 s, 0.7 s | 48.2 s, 0.7 s |
| lint (type errors included after) | 49.3 s | 45.5 s |
| typecheck | 1.4 s | — (in the lint) |
| format:check, lint:secrets, deps:audit, deps:unused, deps:tools, deps:outdated | 1.1, 3.4, 1.1, 1.4, 1.3, 3.1 s | 1.0, 2.1, 1.0, 2.3, 1.4, 3.6 s |
| `check:all` | 61.8 s | 51.5 s |

The test lane is still the slowest, now with the site's build and ESLint close behind (EN-085 removes the second and builds the first on change only). `check:rust` fell with less load beside it; its own steps alone are unchanged (`rust:coverage` 13.5 s of it).

## For the main thread

- The feature file (shared, left alone): F19.AC6's row names TypeScript 7 for "the type check and the declarations"; the declarations are now Oxc's (ADR-0071), and the test `[F19.AC6] is the tsc of the type check` was retitled so.
- ADR-0071 was numbered on `1dcfa40b`; if another story of the run takes 0071 first, renumber at merge (the ADR, its row in `docs/adr/README.md`, ADR-0010's status line, `docs/conventions/typescript.md`, `docs/tooling/commands.md`, this story).
- `.oxlintrc.json` (EN-075's): `--type-check` is passed on the command line; `options.typeCheck: true` in the root configuration would carry it into editors and any other oxlint call (`REF-OXLINT-TYPE-AWARE`), once EN-075 is merged.
- `.github/workflows/ci.yml` compiles `cargo-llvm-cov` and `cargo-deny` from source on each run: a prebuilt install would save minutes of CI, a new action to review (`deps:tools`); no local wait, so left.
- The `done` commit: `Rule-docs: none affected, a tooling story that changes no rule of docs/domain/`.

### Journal

- `#run` EN-091: every configuration file reviewed for speed; the RuleTester's setup given to the rule tests only (test lane 39.6 s to about 30 s), type errors reported by oxlint's `--type-check` with the lint and `tsc` out of `check:all`, the declarations written by Oxc's isolated declarations (0.63 s to 0.06 s, two exports annotated), `check:all` 61.8 s to 51.5 s (ADR-0071).
