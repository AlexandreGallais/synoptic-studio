---
id: VAL-020
epic: E17
feature: F19
title: Validate F19
status: ready
points: 1
---

# E17 · F19 · VAL-020 — Validate F19

Show the Product Owner F19 on one demo page, with the figures before and after.

## Acceptance criteria

- Given the demo page, when the Product Owner reads it, then each criterion of F19 is shown with its evidence.

## Tasks

One task = one commit, referenced as `VAL-020.Tn`.

- [x] T1 — Each criterion of F19 proven again, what was seen recorded (1 h)
- [ ] T2 — F19's lines in E17's checklist, `?checklist=E17`, each tried (1 h)
- [ ] T3 — The demo page, `docs/guide/f19-checks-on-oxc-and-rust.md` (1 h)
- [ ] T4 — The decisions of the validation recorded (0.5 h)

## Criteria proven again (2026-10-17)

On this Mac (Apple M1 Pro), in the worktree of `feat/val-020-validate-f19`, at `5e4a910d`, the heavy commands under the heavy lock. Each criterion: its tagged tests (`[F19.ACn]` in TypeScript, `f19_acn_…` or a `[F19.ACn]` comment in a Rust tool), then the command a person would run.

| Criterion | Command | Seen | Verdict |
| --- | --- | --- | --- |
| AC1 one lint command, nothing reported | `npx vitest run tests -t "F19.AC1]"`; `npm run lint` | 11 tests passed, the accounting of the 854 rules of `eslint/rules/` among them; oxlint 1.87.0 with `oxlint-tsgolint` 7.0.2004 silent, then `lint: 0 finding(s) in 1967 file(s)`, 1.75 s; no `eslint/` folder, no ESLint package in `package-lock.json`; an `oxlint-disable` directive silencing nothing planted: `error: Unused oxlint-disable directive (no problems were reported).` | holds |
| AC2 the rules that needed types | `npx vitest run tests -t "F19.AC2]"`; `cargo test --release -q --locked -p lint` | 1 test passed (`prefer-destructuring` native, its options); 652 Rust tests passed, those of `naming-convention`, `immutable-data`, `prefer-immutable-types`, `type-declaration-immutability`, `no-mixed-types`, `no-conditional-statements` and `no-non-function-verb-prefix` among them | holds |
| AC3 the project's rules in Rust | the same two; `Cargo.toml` | 1 test passed (the lanes call the Rust linter); the Rust tests of `kind-limits`, `no-return-void`, `no-restricted-syntax`, `require-atomic-updates`, and every scoped name a rule that runs; the `oxc` crates pinned at `=0.153.0` | holds |
| AC4 the Markdown rules | the same; a table row of three cells planted in a page of `docs/`, `npm run lint` | 1 test passed; `markdown/table-column-count Table column count mismatch (Expected: 2, Actual: 3)` at its line | holds |
| AC5 oxfmt | `npx vitest run tests -t "F19.AC5]"`; `cargo test -p format f19_ac5_`; `npm run format:check` | 2 and 1 tests passed; `Markdown: 0 of 526 files not formatted.`, 0.67 s | holds |
| AC6 TypeScript 7 alone, the declarations | `npx vitest run tests -t "F19.AC6]"`; `npx tsc --version`; `npm run build`; a type error planted, `npm run lint` | 6 tests passed (a consumer type-checked against the declarations); `Version 7.0.2`, no `typescript6` in `package-lock.json`; 589 `.d.ts` in `dist/types`, the whole build 1.8 s; `src/planted-probe.ts:1:14: error typescript(TS2322): Type 'string' is not assignable to type 'number'.` | holds |
| AC7 the API pages from Rust | `cargo test -p api-pages f19_ac7_`; `npm run docs:api` | 7 tests passed; `api-pages: 537 pages written to docs/api`, 2.4 s built, 0.27 s after | holds |
| AC8 the docs build on change, 70 to 90 s | `npx vitest run scripts/check-all.test.ts`; `npm run check:all`, twice per case | 6 tests passed; nothing of the docs build's inputs changed: `docs:build, playground:build skipped: nothing changed in their inputs against origin/feature/f19-oxc-and-rust-tools`, 41.3 s and 39.6 s, lanes printed; the checklist (`playground/`) changed: `docs:build` 56.3 s and 49.9 s, 57.0 s and 50.6 s in all (their `test` lane failed on the checklist's draft lines, as it must: T2) | holds, under the 70 to 90 s aimed at |
| AC9 the `rust` lane | `npx vitest run tests -t "F19.AC9]"`; `npm run check:rust` | 26 tests passed; `cargo fmt --check`, Clippy, `advisories ok`, 884 Rust tests passed, coverage 94.05 % of lines and 97.15 % of functions against a floor of 93 %, the Windows check; 65.9 s alone, 33.9 to 46.4 s in `check:all` | holds |
| AC10 portable calls, checked for Windows | `npx vitest run tests -t "F19.AC10]"`; `git grep flock -- tools` | 2 tests passed; no `flock` nor `libc` in the tools; `cargo check --target x86_64-pc-windows-msvc` passed in `check:rust` | holds |
| AC11 the plugins' rules triaged, ESLint out | `npx vitest run tests -t "F19.AC11]"`; the Rust tests above | 2 tests passed; the Rust tests of the JSDoc and Unicorn rules, every rule written run on some file; ADR-0075's counts: 374 in oxlint, 205 in the project's linter, 181 dropped, 94 already off | holds |
| AC12 tables without padding | `cargo test -p format f19_ac12_`; a padded table planted, `npm run format` | 2 tests passed; `\| A    \| B   \|` written `\| A \| B \|`, its separator `\| --- \| --- \|` | holds |

Nothing failed: no fix was needed.
