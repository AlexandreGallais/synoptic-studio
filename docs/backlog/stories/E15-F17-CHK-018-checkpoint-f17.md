---
id: CHK-018
epic: E15
feature: F17
title: Checkpoint in the middle of F17
status: done
points: 1
---

# E15 · F17 · CHK-018 — Checkpoint in the middle of F17

Pause between the removals and the fixes of F17: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given the stories merged since the spike, when the checkpoint runs, then `docs/domain/`, derivations, references, conventions, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is, with no balanced wrap, no word broken at a character and no HTML layer left.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for the next stop.

## Tasks

One task = one commit, referenced as `CHK-018.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (1 h)

## Light evolvability check (2026-10-16)

| Question            | Answer                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Size                | 560 modules in `src/` (model 244, geometry 117, io 96, math 95, render 7), barrels counted, against 544 at CHK-017; 405 test files in `src/` and `playground/`; the longest module is still `parse-kern-table.ts` (228 lines), `document-font-embedding.ts` next (210); six folders at the 15-module signal of ADR-0043 (`math/color`, `math/vector`, `model/free-shape`, `model/text-layout`, `model/font`, `geometry/corner-radius`), none past it; the playground holds 240 modules, `user-actions/` 28 |
| Tools               | no trigger since CHK-017: `size-limit` waits for the first npm publication, `api-extractor` for a stable API, Stryker for its issue 6210                                                                                                                                                                                                                                                                                                                                                                   |
| Fitness functions   | the playground's `[F17.AC1]` and `[F17.AC2]` tests refusing `switch`, `foreignObject` and XHTML in the drawing and its export act as one; a test over what `src/render/` itself may write is still to come (CHK-017's row); the golden check still runs only by hand, not in `check:all`                                                                                                                                                                                                                   |
| Patterns            | the removals left no dead code (knip clean); the scan (`scan`) now serves the automatic wrap and a collection's tables (US-046); the memoized text measure (`text-measure/`, US-048) is the first cache of the playground, keyed by its inputs                                                                                                                                                                                                                                                             |
| Agent configuration | AGENTS.md and CLAUDE.md 147 lines together; the state line says F17 is built; one pitfall added to `.claude/rules/library.md`, happy-dom's `<textarea>` keeping its first line break (US-051); the `run` and `review` skills validate on the checklist page (EN-044)                                                                                                                                                                                                                                       |
| Process             | `npm run val:prepare` still prepares guided steps (improvement log, for RET-003); E15's checklist lines are first tried in a browser at VAL-018 (improvement log)                                                                                                                                                                                                                                                                                                                                          |

Consistency — no balanced wrap, no break at a character and no HTML layer left in `src/` or `playground/` but tests that assert their absence; `shapes.md` (Text, Fonts from files, Embedded fonts), Q31, Q37 and Q39, ADR-0060, `DERIV-text-box` (the break at a character in step 4 and step 9 marked as records) and `DERIV-html-layer` (a record) agree with the code; the folder map of `docs/conventions/files.md` names `installed-fonts/`, `redraw/`, `text-measure/`, `checklist/` and `checklists/`. The reference rows that served only the HTML layer now say it was removed, kept as ADR-0053's record (nine rows), those it shared say its uses ended (`REF-CSS-OVERFLOW-3`, `REF-CSS-INLINE-3`, `REF-RESVG-SUPPORT-TABLE`, `REF-HARFBUZZ-SHAPING`), `REF-WIKI-MAXIMUM` says the balanced wrap was removed, and `REF-WIKI-PREFIX-SUM` names its callers today.

Product Owner test cards written in F17, for VAL-018, which tries them as lines of `?checklist=E15`:

| Story                                                                               | Test card                                                                                   |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- |
| [US-042](./E15-F17-US-042-a-long-word-always-whole.md#product-owner-test)           | "running" in a box 50 × 40, whole past the box, then "runn…"; no option for a word too long |
| [US-043](./E15-F17-US-043-no-balanced-wrap.md#product-owner-test)                   | "Pump P-101 is" over "running" in a box 110 × 60; no wrap style                             |
| [US-044](./E15-F17-US-044-no-html-layer.md#product-owner-test)                      | Embed fonts the only option; no `switch` nor `foreignObject` in the export                  |
| [US-045](./E15-F17-US-045-fonts-embedded-at-export.md#product-owner-test)           | the drawing unchanged with Embed fonts on; one `@font-face` for Arial in the export         |
| [US-046](./E15-F17-US-046-installed-fonts-from-collections.md#product-owner-test)   | "Helvetica: 6 faces read."; a family failing with a `TypeError` read, or said in one line   |
| [US-047](./E15-F17-US-047-weight-450-drawn-as-measured.md#product-owner-test)       | IBM Plex Sans at 400, 450 and 500, three weights, the frame hugging each                    |
| [US-048](./E15-F17-US-048-fluid-changes-held-keys.md#product-owner-test)            | the Up arrow held on the size, the opacity and the box's height, no lag                     |
| [US-049](./E15-F17-US-049-background-band-ends-with-its-line.md#product-owner-test) | each band ending with its word, a trailing space or a wrapped line                          |
| [US-050](./E15-F17-US-050-turned-text-frames-as-shapes.md#product-owner-test)       | a rectangle and a text box turned by 30, the same blue and dashed frames                    |
| [US-051](./E15-F17-US-051-validation-checklist-page.md#product-owner-test)          | E15's checklist, no Next; two ticks kept after a reload                                     |
