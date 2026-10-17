---
id: EN-071
epic: E17
feature: F19
title: TypeScript 7 for the type check and the declarations
status: ready
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

- [ ] T1 — `typescript` 7 installed, the alias for ESLint and TypeDoc, its row in `docs/tooling/dependencies.md` (2 h)
- [ ] T2 — The type check and the declarations compared with TypeScript 6 on the tree and on a planted error; the exit code read as 1 (2 h)
