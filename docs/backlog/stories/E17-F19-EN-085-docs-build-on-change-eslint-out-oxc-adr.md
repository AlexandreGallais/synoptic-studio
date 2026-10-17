---
id: EN-085
epic: E17
feature: F19
title: The docs build on change, ESLint out, the ADR on Oxc
status: ready
points: 3
---

# E17 · F19 · EN-085 — The docs build on change, ESLint out, the ADR on Oxc

Run the docs build only when `docs/`, `src/` or the playground changed (option B), take ESLint and Prettier out of the checks, and record the toolchain in an ADR superseding ADR-0009, ADR-0010 and ADR-0013 (decided at the review of the ADRs, 2026-10-17, EN-094). Sources: [research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), sections "`check:all`, lane by lane", "The target of `check:all`" and "Decisions"; `REF-VITEST-RELATED`, `REF-VOIDZERO`. The packages themselves are removed by F21's clean-up.

## Acceptance criteria

- Given a branch whose changes against its base touch none of `docs/`, `src/` and `playground/`, when `check:all` runs, then it skips the docs and playground builds and says so; a change in any of them runs both (F19.AC8).
- Given a code story on the Product Owner's machine, when `check:all` runs under the heavy lock, then it ends in 70 to 90 s, lane times printed as now, three runs recorded in the story (F19.AC8).
- Given `npm run lint`, `lint-staged` and `check:all`, when the story ends, then none calls ESLint or Prettier; every rule of `eslint/` is carried, rewritten or dropped, and TypeScript 6's last package, used by ESLint only, is out (F19.AC1, F19.AC6, F19.AC11).
- Given the docs, when the story ends, then an ADR on the Oxc toolchain supersedes ADR-0009, ADR-0010 and ADR-0013, as the review of the ADRs decided (2026-10-17, EN-094): the toolchain changes, the rules on dependencies stay (dev packages only, exact versions, each justified, audited), and "the linter first" stays, the linter being oxlint and the project's Rust linter; it keeps ADR-0071 standing, and lists each rule of `eslint/` with how it is carried and each dropped rule with its reason; `.claude/rules/tooling.md` says TypeScript 7 is the only TypeScript, TypeScript 6 and TypeDoc are removed and the API pages come from the Rust generator (F19.AC1).

## Tasks

One task = one commit, referenced as `EN-085.Tn`.

- [ ] T1 — The path filter of the docs build in `check-all.ts`, with its tests (2 h)
- [ ] T2 — ESLint, Prettier and TypeScript 6 out of the scripts, the hooks and `check:all`; three timed runs recorded (2 h)
- [ ] T3 — The ADR, its table of the rules of `eslint/`, `.claude/rules/tooling.md` and the tooling docs (3 h)

## Measures

`check:all` under the heavy lock on the Product Owner's machine, 2026-10-17, after T2 (ESLint out), while another worktree's agent worked beside it:

| Change against the base | Runs (s) | Slowest lanes |
| --- | --- | --- |
| code only: the docs lane skipped | 85.3, 39.4, 38.5 | `test` 38 to 47 s, `check:rust` 37 to 73 s; the first run built the Rust tools again after T2's change to `tools/lint` (`rust:build` 11.8 s, `check:rust` 73.4 s) |
| docs changed: every lane | 76.6, 61.4, 56.9, 61.2, 68.0 | `docs:build` 56 to 76 s, `test` 50 to 70 s, `check:rust` 48 to 61 s; the first two failed in `test` under the docs build's load, their output not kept, and passed in the three runs after; to watch at AUD-021 (the installed fonts' test is the one known flaky) |

A code story ends in about 39 s on a built workspace, under the 70 to 90 s of F19.AC8; a docs story in about 57 to 68 s. ESLint alone took 14.2 s, and 58 s for the three linters in a loaded `check:all` (CHK-020); `lint` now takes 2.2 to 5.2 s.

## Note

Left by SP-021 for this story: `.claude/rules/tooling.md` gets the line that TypeScript 7 is the only TypeScript, that TypeScript 6 and TypeDoc are removed and that the API pages come from a Rust generator ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), "The Product Owner's second round").
