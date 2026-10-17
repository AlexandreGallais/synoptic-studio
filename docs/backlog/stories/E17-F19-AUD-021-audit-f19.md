---
id: AUD-021
epic: E17
feature: F19
title: Audit F19
status: ready
points: 1
---

# E17 · F19 · AUD-021 — Audit F19

Audit F19 in depth, the project in breadth ([audit.md](../../conventions/audit.md)).

## Acceptance criteria

- Given F19's stories merged, when the audit ends, then each criterion is traced to a test and the findings are fixed or logged.

## Tasks

One task = one commit, referenced as `AUD-021.Tn`.

- [x] T1 — Repair the pages a replacement string broke (E3-1, DE-01, DE-02)
- [x] T2 — Every rule reached by a scope, every scoped name a rule (V1)
- [x] T3 — Unused disable directives reported by oxlint (A-LOST-1)
- [x] T4 — Renames and the Rust toolchain in the docs build's inputs (AC8-1)
- [x] T5 — Criteria traced to tests (TRACE-1, V4, V17)
- [x] T6 — Tests the verifier found missing (V2, V3, V5 to V12, V18)
- [x] T7 — Licences of the ported code and copied cases (C3/C1-1)
- [x] T8 — Small code findings (A-API-1, E3-2, B1-1)
- [x] T9 — Documentation findings (DE-03 to DE-61, E1-1, E1-2)
- [x] T10 — Findings table

## Decision of the Product Owner (2026-10-17)

The licences of the code F19's tools port and of the test cases they copy are cited: `NOTICE` lists each package with its version, licence and copyright lines, `THIRD-PARTY-LICENSES.md` holds each licence text as published at that version, and ADR-0076 allows porting permissive code into the tools with its notice, never into the library, which keeps the research protocol's rule (section 4, amended to say so). A test checks that each package a tool names as the one it ports is listed.

## Findings

Three independent reviews on 2026-10-17 (`auditor`, `verifier`, `doc-editor`), each finding reproduced before it was fixed; each new test was made to fail once by breaking its input or the code it guards. Outcome: **fixed** (the task), **no action** (with the reason), **deferred** (where it goes).

### Auditor

| Id | Severity | Evidence | Outcome |
| --- | --- | --- | --- |
| C3/C1-1 | high | 14 packages ported (Unicorn, JSDoc and its parser, functional, @stylistic, import-x, typescript-eslint, @eslint/markdown, ESLint, eslint-utils, css-tree, Babel's identifier check, are-docs-informative); `eslint-plugin-jsdoc`'s cases copied; `NOTICE` named only Inter | fixed by the Product Owner's decision: `NOTICE`, `THIRD-PARTY-LICENSES.md`, ADR-0076, research protocol section 4, `tests/third-party-licences.test.ts` (T7) |
| AC8-1 | medium | a file moved out of `docs/` listed under its new path only (rename detection); `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` not inputs | fixed: `--no-renames`, the three files, a rename in a temporary repository tested (T4) |
| A-LOST-1 | medium | `reportUnusedDisableDirectives` lost: an `oxlint-disable` silencing nothing passed (probe file) | fixed: `--report-unused-disable-directives-severity=error` in `lint`, `fix`, lint-staged, two tests, note 0023 (T3) |
| TRACE-1 | medium | AC3, AC4, AC11 tagged on target lists only; AC6's declarations checked for exit 0; AC10 one test; AC1's accounting test deleted with `eslint/` | fixed: the 854 rules recorded from Git and accounted against ADR-0075's tables; a consumer type-checked against the declarations; a portability test; Rust tests tagged (T5) |
| E1-1 | low | 20 Markdown rules moved, `no-reference-like-urls` dropped with a reason covering `docs/` only | fixed: the reason covers every page (the weekly link check reads `.claude/` and `.github/`); AC4 corrected in the feature file (T9) |
| E1-2 | low | 203/89 against 196; 171 against 181; AC11's 228/37 | fixed: ADR-0075's counts tested as the final ones, the others named as stages in note 0023 and the feature file (T9) |
| E3-1 | low | `lint.md` cut at `` `' `` and a section copied: a `$'` in a replacement string | fixed: rejoined, copy deleted; every tool passes a function to `replace`; a pitfall in `.claude/rules/tooling.md` (T1) |
| E3-2 | low | ESLint `off` step in `lint.md`, "declarations emitted by `tsc`", the lanes' guard blind to test files | fixed (T8, T9; the guard in T6) |
| A-API-1 | low | `members()` skipped a type's arguments and a function type's parameters | fixed: read, the tree passes, a planted case fails without it (T8) |
| D1-1 | low | `link_folder` twice, two Markdown cell splitters, Git runners in four crates | deferred to F21, which rewrites the tools around one workspace: a shared crate there |
| B1-1 | low | `REF-UNICORN-NON-FUNCTION-VERB-PREFIX` cited the installed package; PR 692's claim had no URL | fixed: the rule's file at `v78.0.0`, PR 692's URL and reviews read (T8) |

### Verifier

| Id | Severity | Evidence | Outcome |
| --- | --- | --- | --- |
| V1 | high | a rule removed from `ALL` or `web::RULES`, or a name from `web::NAMES`: 647 tests passed | fixed: every scoped name finds its rule, every rule runs somewhere, web's names equal its rules (T2) |
| V2 | medium | `no-floating-promises` off: the tests passed | fixed: a floating promise reported with the repository's configuration (T6) |
| V3 | medium | three mutations of the table formatter survived | fixed: three cases (T6) |
| V4 | medium | one planted type error proven twice, the tag on the `tsc` one | fixed: the oxlint test tagged, the `tsc` one dropped (T5) |
| V5 | medium | `fill`, `clear` renamed: passed | fixed: the six methods tested (T6) |
| V6 | low | the `outdated` half of a join untested | fixed (T6) |
| V7 | low | `all_blocks` forced true: passed | fixed (T6) |
| V8 | low | `throw` in a declaration against an expression untested | fixed (T6) |
| V9 | low | `__` check dropped: passed | fixed (T6) |
| V10 | low | offset × 0: passed | fixed: a place test; the directive matched by line is already tested in `lib.rs` (`silences_the_next_line_…`) (T6) |
| V11 | low | the formatter's test wrote its own `.oxfmtrc.json` | fixed: copies the repository's (T6) |
| V12 | low | the lanes' guard read npm scripts only | fixed: the test files' `cargo run --release` read too (T6) |
| V13 | low | the nearest base untested; `startsWith` would pass | fixed: `nearestBase` pure and tested, `package.json.bak` refused (T4) |
| V14 | low | the declarations compared with nothing | fixed otherwise: a consumer type-checks against them, its misuses refused (T5); a golden of `tsc`'s output would compare text, not meaning |
| V15 | low | three test files read every tracked source on their own | deferred: improvements log, for F21's test helpers |
| V16 | low | machine dependencies (cargo-deny's database) | no action: the command names the fetch to run; the CI fetches it |
| V17 | low | AC3, AC4, AC11 without a Rust test tagged | fixed (T5) |
| V18 | low | `unwrap_or_default()` hid a failed fixture write | fixed: `expect` (T6) |

### Doc-editor

| Id | Severity | Outcome |
| --- | --- | --- |
| DE-01, DE-02 | high, medium | fixed (T1) |
| DE-03, DE-04, DE-06 to DE-24, DE-27 to DE-31, DE-33 to DE-44, DE-46 to DE-48, DE-54, DE-60, DE-61 | high to low | fixed (T9; DE-25 in T8, DE-26 in T1); DE-35's one rule lives in `functions.md`, `AGENTS.md` and its stop table follow it; DE-43's reason is the shared files (ADR-0065, decision 4) |
| DE-05 | medium | the page fixed; the test kept as is: oxlint reads an `eslint-disable` directive as its own (tried on a probe file), so the register must list one |
| DE-21 | medium | fixed; F21's criterion 5 kept, an end state still to hold |
| DE-32 | medium | fixed in the worker brief, which now links `CLAUDE.md`'s threshold; `CLAUDE.md` unchanged |
| DE-45 | low | partly fixed (DE-41, DE-42); the other dated instructions keep their date: their reason is not recorded, and none is invented |
| DE-49 to DE-53, DE-57, DE-58 | medium, low | deferred to F20, the context diet: one fact in one place across `AGENTS.md`, `CLAUDE.md` and the skills (`AGENTS.md`'s dangling clause of DE-50 fixed in T9) |
| DE-55 | low | deferred to F21's refinement, which writes its stories table |
| DE-56 | low | no action: the section already opens with its supersession by the feature file |
| DE-59 | low | no action: the comparison tells a reader where the pages differ from TypeDoc's, which still holds |

## G. Sources and questions for the next feature (F20)

- The size of what a session loads (`context:size`, `docs/tooling/agent-tools.md`, Context), measured before and after; the one-fact-one-place findings above (DE-49 to DE-53, DE-57, DE-58) as its first inputs.
- Whether `CLAUDE.md`'s rules can link the skills instead of restating them, so that a rule lives once (DE-52).

## H. Light evolvability check

1. **Size**: 12 Rust crates; `AGENTS.md` and `CLAUDE.md` 165 lines together, under 200.
2. **Tools**: oxlint's unused-directive report now on; nothing new to adopt before F20.
3. **Fitness functions**: rule reach, the accounting of `eslint/`'s rules, the licences of ported code and Unix-only items' twins are tested.
4. **Patterns**: the duplicates of D1-1 wait for F21's shared crate.
5. **Agent configuration**: a replacement string is never user text; a function over its limits has one rule.
6. **Process**: one finding (C3) needed the Product Owner; every other was fixed, deferred with its place, or closed with a reason.

## For the main thread

Rule-docs: none affected, tooling and documentation only, no rule of `docs/domain/` changed.

### Improvements

| 2026-10-17 | AUD-021 | The tools repeat `link_folder`, a Markdown cell splitter (two of them, disagreeing on backslashes) and a Git runner in four crates: one shared crate when F21 rewrites the tools (D1-1) | tooling |
| 2026-10-17 | AUD-021 | `tests/exceptions.test.ts`, `font-files.test.ts` and `typescript-7.test.ts` each read every tracked source on their own; one reader shared through `tests/built-once.ts` (ADR-0070, decision 4) would read them once (V15) | tooling |
| 2026-10-17 | AUD-021 | One fact in several places — the Rust rule eight times, the agent limit four, the two-round refinement twice, run skill steps of 8 to 12 lines — for F20's context diet (DE-49 to DE-53, DE-57, DE-58) | process |
| 2026-10-17 | AUD-021 | A `$'` in a sentence passed to `String.prototype.replace` copied the rest of a page into it: text put into a file is returned by a function, never passed as the replacement (`.claude/rules/tooling.md`) | tooling |
