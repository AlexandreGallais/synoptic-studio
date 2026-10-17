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

## Note

Left by SP-021 for this story: `.claude/rules/tooling.md` gets the line that TypeScript 7 is the only TypeScript, that TypeScript 6 and TypeDoc are removed and that the API pages come from a Rust generator ([research note 0023](../../research/notes/0023-f19-oxc-and-rust-tools.md), "The Product Owner's second round").
