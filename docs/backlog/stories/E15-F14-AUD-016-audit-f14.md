---
id: AUD-016
epic: E15
feature: F14
title: Audit F14
status: done
points: 2
---

# E15 · F14 · AUD-016 — Audit F14

The feature in depth, the project in breadth, following [audit.md](../../conventions/audit.md) (ADR-0020), with the `/audit` skill.

## Acceptance criteria

- Given the functions of F14, when audited, then every formula of `DERIV-text-box` is re-derived, every expected value recomputed by hand, the balanced wrap checked against a search of every way to break small texts, and each function survives a mutation spot-check.
- Given the sources, when audited, then every `@see` of F14 is re-verified and dated.
- Given the project, when audited, then no duplicate, no copied code, no unnamed calculation (ADR-0049) and no inconsistency between code, domain, ADRs and docs remains unrecorded.

## Tasks

One task = one commit, referenced as `AUD-016.Tn`.

- [x] T1 — The audit by independent reviewers and its findings (2 h)

## Findings

Four independent reviews of F14 only (`feat/val-014-text...feature/f14-text-box`), 2026-10-15: the `auditor` (A to E), the `math-expert` (derivation, expected values, brute force, precision), the `verifier` (tests, 48 mutations) and the `doc-editor` (pages changed). Every formula of `DERIV-text-box` re-derived and every expected value recomputed with `node -e`: all match. No copied code, no source failing what it is cited for.

### Fixed in this audit

| Check | Finding | Evidence | Action |
| --- | --- | --- | --- |
| A3, degenerate | A line height of 0 (a font whose ascender, descender and line gap add up to 0) in a box 0 tall made `shownLineCount` NaN: no line shown, against "at least one line" | new test failed before the fix: `expected NaN to be 3` | fixed: every line wrapped fits when a line takes no height |
| A3, step 2 | A space after a no-break space, in a word broken at a character, was measured for the fit and pushed the no-break space to the next line | new test failed before the fix: "H", "e", "ll", then the no-break space and the space, for "H", "e", "ll" with both, in a box 12 | fixed: the pending no-break space is placed first, the space after it joins unmeasured |
| E1, ADR-0055 | A text in a box turns about its box's middle (`textDrawing`, step 8, `shapes.md`), ADR-0055 decision 3 says its block's; the TSDoc of `createTextElement` said the block's | auditor: `boxCenter(frame)` in `text-drawing.ts` | TSDoc fixed; rule provisional in `shapes.md`, VAL-015 decision 21, the ADR amended once confirmed |
| E1, derivation | The ellipsis pseudo-code exempted leading spaces only, the code also leading no-break spaces; the trimmed band example did not say its line height; the trim sentence of §6.4 was left out | auditor: " Hello" gives " H…" in the code, " …" by the pseudo-code; 15.375 holds only at the automatic line height | `DERIV-text-box` steps 6 and 10 corrected |
| A2, wording | "Hello" broken in a box 5: "each wider than the box" is false for "l" (3.55469 ≤ 5); the result stands, "ll" 7.10938 > 5 | math-expert | derivation, its cross-check and the test comment corrected |
| E1, step 9 | "spacing 0 or more" is not enough for each token to widen a line: a kerning pair across the break must not take back the advance | math-expert | condition added to step 9 |
| E1, note 0018 | It called the balance "exact", which step 9 withdrew (a heuristic for CSS's spread), and kept the old rule "a text cut by the ellipsis" | doc-editor | note corrected |
| E3, docs | `AGENTS.md` state line ("the Text tool next"), `files.md` missing US-034's modules, three tooling pitfalls in `library.md`, stale lines in F14 and US-033 | doc-editor | corrected; the pitfalls moved to `.claude/rules/tooling.md` |
| A5, properties | `withoutTrailingSpaces`, `isMeasureFitting` and `wordBrokenAtCharacters` had examples only | verifier | properties added: prefix and idempotence, monotone in the box, the word given back whole |
| E2, business rule | Only U+00A0 keeps a line from ending after it; U+202F, U+2007 and U+2060 do not | verifier | provisional in `shapes.md`, VAL-015 decision 22 |

### Checked, no action

| Check | Result | Evidence | Action |
| --- | --- | --- | --- |
| A1, A2 | pass | every row of the derivation's cross-check and every test comment recomputed (F14.AC2: 52.46094, 57.78125; balance 3232.02 against 2094.02; bands 3.0625 and 27.0625; cap height at offset 88) | — |
| A1, balanced wrap | pass | math-expert's brute force over every cut into the greedy number of lines: 4 615 paragraphs, 1 319 with negative spacing, 0 mismatches, 222 ties all to the later break | — |
| A4, mutations | pass | 29 mutations by the agent, 26 caught, 3 equivalent (the two pruning bounds of `leastRaggedBreaks`, the balance of one line); the verifier's 48, 42 caught, 6 equivalent or unreachable | logged for NAM-001 |
| B1, sources | pass | the 7 sources F14 added re-read in the library (`REF-CSS-INLINE-3`, `REF-CSS-OVERFLOW-3`, `REF-CSS-OVERFLOW-4`, `REF-CSS-TEXT-4`, `REF-SVG2-FILL-RULE`, `REF-WIKI-KNUTH-PLASS`, `REF-WIKI-PREFIX-SUM`), dated 2026-10-15 | — |
| B2, link check | none | `links.yml` had no run: dispatched on `main` (run 38694822415) | logged |
| C, provenance | pass | TSDoc formulas match the bodies; the dynamic programming, the scan by halves and the ellipsis cut are in the project's idiom | — |
| D1–D4, duplicates | logged | a third table for the share of room before (`blockOffsetInBox` beside `alignmentFactor`); inline bounds and counts in `leastRaggedBreaks`, `shownLineCount`, `textDrawing` | improvement log, NAM-001 |
| A3, precision | logged | ties of the balance in a font of 1000 units per em may break on one ulp (1.82e-12) for 3 lines or more; both cuts are equally even | improvement log |
| E4, backlog | pass | F14's stories `done` but AUD-016 and VAL-015; criteria F14.AC1 to AC6 each tagged in tests | — |
| F2 | pass | no `.only` or `.skip`; each `eslint-disable` of F14 justified (values the page offers, JSON written by the page) | — |
| Docs, style | logged | long blocks in `shapes.md` (Box), Q37, `DERIV-text-box` and note 0018 | improvement log, RET-003 |

### G, next feature

F16 (colors and their pickers): sources on color notations (CSS Color 4 hex and `rgb()`), opacity as an integer percentage (already `REF-SVG2-FILL-OPACITY`), and a picker's fields; the question of one color shared by a text, its background and the shapes.

### H, light evolvability check

- Size: four domain folders opened in F14 at the 15-module signal (already logged, CHK-015).
- Tools: mutations scripted in a copy of the repository worked again.
- Fitness functions: no test sees a pruning bound (logged); a lint rule against a spread in a fold (already logged).
- Patterns: the scan by halves serves the wrap, the break at a character and the balance.
- Agent configuration: tooling pitfalls moved to `tooling.md`.
- Process: four reviewers in parallel found 2 defects after eight reviewed stories, both degenerate cases.
