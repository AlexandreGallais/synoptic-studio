---
id: EN-058
epic: E17
feature: F18
title: A mutation runner as a tool
status: ready
points: 1
---

# E17 · F18 · EN-058 — A mutation runner as a tool

Make the mutation runner used by the audits a tool, since Stryker is still blocked (stryker-js issue 6210 open on 2026-10-16; row 16 of RET-003; I25, I40, I48; F18.AC15).

## Acceptance criteria

- Given `npm run mutate -- <files>`, when it runs, then it applies one mutation per source file in a copy of the repository, runs the related tests, and reports caught and surviving mutations.

## Tasks

One task = one commit, referenced as `EN-058.Tn`.

- [x] T1 — The tool, its test and its line in `docs/tooling/agent-tools.md` (2 h)

## For the main thread

Tried on `src/math/arithmetic/is-span-within.ts` with the default command: 4 caught, 0 survived, in 8 min — `vitest related` reaches most of the suite through the barrels. `-- npx vitest run {test}` runs only the file's own test file.

### Journal

- EN-058: `npm run mutate` (`tools/mutate/`, Rust, std only) tries textual mutations — comparison, operator, constant, condition, return — in a temporary worktree outside the repository, under the heavy lock, and reports caught and surviving ones; the audit skill's A4 step points to it.

### Improvements

| Date       | From  | Item                                                                                                                                                                                                                  | Kind |
| ---------- | ----- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| 2026-10-17 | agent | `npm run replace` silently ignores a second `<<<` block written without its own `@@ file` line, and still prints "1 edits": it should refuse a block it cannot read (EN-058)                                          | tool |
| 2026-10-17 | agent | `npx vitest related` on one `src/` file runs most of the suite (about 1.5 min) because the barrels relate every test to it; `mutate` could choose the file's own test by default once F19 settles the parser (EN-058) | tool |
