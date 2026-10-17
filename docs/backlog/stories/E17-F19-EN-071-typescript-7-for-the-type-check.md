---
id: EN-071
epic: E17
feature: F19
title: TypeScript 7 for the type check and the declarations
status: done
points: 2
---

# E17 · F19 · EN-071 — TypeScript 7 for the type check and the declarations

Make TypeScript 7, the native compiler, the `tsc` of the type check and of the declarations of `build`, as the Product Owner decided on 2026-10-17. Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "TypeScript 7, the native compiler" and "TypeScript 7 alone with Vite, Vitest and VitePress"; `REF-TS7-ANNOUNCEMENT`, `REF-TS-NATIVE-PORT`. TypeScript 7 has no API until 7.1, and typescript-eslint and TypeDoc read TypeScript 6's API: TypeScript 6 stays only under its alias `@typescript/typescript6`, for them, until EN-072 replaces TypeDoc and EN-085 takes ESLint out; its last package leaves there.

## Acceptance criteria

- Given `typescript` 7.0.2 or later (at least 7 days old) in `package.json`, when `npm run typecheck` runs on the tree, then it reports no error, and on a copy with one planted type error it reports the error TypeScript 6 reported, at the same place (F19.AC6).
- Given `npm run build`, when it runs, then `tsc -p tsconfig.build.json` writes the same 587 declaration files as TypeScript 6, byte for byte (F19.AC6).
- Given a script or a test that reads `tsc`'s exit code, when the type check fails, then it accepts 1, the code TypeScript 7 returns (F19.AC6).
- Given ESLint and TypeDoc, when they run in this story's `check:all`, then they read TypeScript 6 through `@typescript/typescript6` only, and no other tool imports it; the packages of TypeScript 6 left are listed with the story that removes each (F19.AC6).

## Tasks

One task = one commit, referenced as `EN-071.Tn`.

- [x] T1 — `typescript` 7 installed, the alias for ESLint and TypeDoc, its row in `docs/tooling/dependencies.md` (2 h)
- [x] T2 — The type check and the declarations compared with TypeScript 6 on the tree and on a planted error; the exit code read as 1 (2 h)

## Decision

On 2026-10-17 the Product Owner set the end state: no ESLint and no TypeScript 6 left, and as little TypeScript 7 tooling as possible beyond the type check, Rust tools wherever possible. TypeScript 7 does the type check and the declarations now. TypeScript 6 stays only as a transition, as the API that typescript-eslint and TypeDoc load, so that no lint rule is lost meanwhile: it leaves with ESLint at EN-085, and TypeDoc leaves at EN-072.

The layout is the one `REF-TS7-ANNOUNCEMENT` recommends for tools that import `typescript` through a peer dependency: `typescript` is `npm:@typescript/typescript6@6.0.2`, which re-exports TypeScript 6.0.3's API, and `@typescript/native` is `npm:typescript@7.0.2`, whose binary is the `tsc` of `npx tsc`. A per-package npm override giving TypeScript 6 to typescript-eslint alone was tried and refused by npm (`ERESOLVE`: a peer dependency resolves to the root's `typescript`). At EN-085, `typescript` becomes `7.0.2` again and `@typescript/native` leaves.

## Results

| Check, 2026-10-17 | TypeScript 6.0.3 | TypeScript 7.0.2 |
| --- | --- | --- |
| `tsc --noEmit`, 3 runs | 4.23 to 4.51 s | 0.88 to 1.06 s |
| errors on the tree | 0 | 0 |
| one type error planted in a copy | TS2322 at `average.ts(14,9)`, exit 2 | TS2322 at `average.ts(14,9)`, exit 1 |
| declarations of `tsconfig.build.json` | 589 files | 589 files, byte for byte the same |
| ESLint and TypeDoc (`docs:api`) on the alias | — | both run, loading TypeScript 6.0.3 |

The tree now has 589 declaration files, two more than when the story was written. `scripts/check-all.ts` already counts any code other than 0 as a failure, so the exit code 1 needs no change there; `tests/typescript-7.test.ts` proves the planted error, its place and the code 1.

## For the main thread

- `.claude/rules/tooling.md` still says TypeScript is pinned to 6.0 and that changing it needs an ADR, and ADR-0009 and `docs/tooling/dependencies.md`'s former row named that pin. This story follows the Product Owner's decision of 2026-10-17 recorded in research note 0023; whether it needs its own ADR superseding ADR-0009's TypeScript pin, or waits for EN-085's ADR on Oxc, is for the main thread to decide, and the rule's line to be updated with it.
- `.vscode/settings.json` points `typescript.tsdk` to `node_modules/typescript/lib`, now the TypeScript 6 alias without `tsserver.js`: VS Code falls back to its own TypeScript. Left as it is; to settle with the editor setup.
- Rule-docs: none affected, a tooling change with no business rule.
