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
