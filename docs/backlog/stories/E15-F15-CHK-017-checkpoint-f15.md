---
id: CHK-017
epic: E15
feature: F15
title: Checkpoint in the middle of F15
status: done
points: 1
---

# E15 · F15 · CHK-017 — Checkpoint in the middle of F15

Pause between the font stories and the HTML layer of F15: bring documentation and methods up to date as the project grows, before going on (ADR-0028, `/run` skill).

## Acceptance criteria

- Given EN-039 and EN-040 merged, when the checkpoint runs, then `docs/domain/shapes.md` (Fonts from files, Embedded fonts), ADR-0053, `DERIV-font-embedding`, the references, `AGENTS.md`, `CLAUDE.md` and `.claude/rules/` describe the code as it is.
- Given the light evolvability check, when it runs, then each of its six questions has a one-line answer and anything worth acting on is in the improvement log.
- Given the user stories merged so far, when the checkpoint ends, then their Product Owner test cards are listed for VAL-017.

## Tasks

One task = one commit, referenced as `CHK-017.Tn`.

- [x] T1 — Consistency of code, docs and agent configuration; fixes (1 h)
- [x] T2 — Light evolvability check; improvement log; test cards listed (30 min)

## Light evolvability check (2026-10-15)

| Question | Answer |
| --- | --- |
| Size | 544 modules in `src/` (model 240, geometry 117, math 91, io 88, render 7), barrels counted, against 481 at CHK-016; 389 test files; the longest module is still `parse-kern-table.ts` (228 lines), `document-font-embedding.ts` next (210, nine local functions); `model/text/` reached 15 modules at EN-040, the signal of ADR-0043 |
| Tools | no trigger since CHK-016: `size-limit` waits for the first npm publication, `api-extractor` for a stable API, Stryker for its issue 6210 |
| Fitness functions | ADR-0053's list of allowed SVG elements, each in its place (`style` only in `defs`, `switch` only around a text), is still only written: a test over the elements `src/render/` creates could check it before EN-041 adds two of them (improvement log) |
| Patterns | a family's `version` is now empty for every family read from files, each face's file holding its own: the field serves only the shipped Inter; `isSameFace` (font embedding) and `facesWithFace` (font import) compare a face's weight and style in two places, a candidate for NAM-001 |
| Agent configuration | AGENTS.md and CLAUDE.md 147 lines together; the state line now names F15; the rules still hold for the font code (no new pitfall met by EN-039 and EN-040) |
| Process | the EN-040 audit added three provisional rules (family whole, generic name, not named) late in the story: they reached `shapes.md` but not Q39 until this checkpoint |

Consistency — `shapes.md` (Embedded fonts) lists the five reasons a face is left out and no longer reads as if a face of the default font were embedded on its own; Q39 names EN-040's rules; `files.md` places the generic family check in `model/text/`; AGENTS.md's state line names F15 and F16 as audited. `src/io/font-name/`, `font-face/`, `font-tables/`, `font-embedding/`, `src/model/font-import/`, `font-embedding/`, `document/`, `text-font/`, `text/` and `src/render/create-embedded-fonts-elements.ts` agree with `shapes.md` (Fonts from files, Embedded fonts), ADR-0053 (decision 2, narrowed by the domain's provisional rules, not contradicted), `DERIV-font-embedding` (steps 3 to 7 and its checks), the folder map of `files.md` and the references their `@see` cite, each a verified row.

Product Owner test cards written so far in F15: none, its user story (US-036) comes after the HTML layer (EN-041). F15's provisional rules are collected in VAL-017 ("Decisions to confirm"), 18 rows, each with its example; rows 11 to 18 come from EN-039 and EN-040.
