---
id: SP-016
epic: E15
feature: F14
title: "Research for F14: a box of text wrapped, cut with an ellipsis and aligned"
status: done
points: 2
---

# E15 · F14 · SP-016 — Research for F14: a box of text wrapped, cut with an ellipsis and aligned

Find, read and record the sources of every story of F14 before its implementation, then write its stories (ADR-0020, research protocol §8). F05's refinement, rounds 1 and 2, settled the business rules of the box; what they leave open is decided here, provisional, to confirm at VAL-015.

## Acceptance criteria

- Given each story of F14, when the spike ends, then every formula, algorithm and behavior it needs has a verified `REF-*`, a `DERIV-*` or an open research request.
- Given the sources found, when the spike ends, then the stories' criteria and tasks cite them, and research note 0018 records what was found and what remains open.
- Given F14's second criterion, when the spike ends, then it carries two numeric examples computed by the derivation: one line cut with "…", and "…" on the last of a maximum number of lines.

## Questions

| Question | Story |
| --- | --- |
| Where may a line wrap, when only spaces count (UAX #14), and what becomes of the spaces at a wrap? | EN-033 |
| What does a word too long for the box do when it breaks at a character, or moves whole (CSS `overflow-wrap`)? | EN-033 |
| How many lines fit in a box of a given height, with a maximum number of lines (CSS `line-clamp`, `max-lines`)? | EN-034 |
| Where does the ellipsis go, what is removed to fit it, and how wide is it (CSS `text-overflow`, U+2026's advance) | EN-034 |
| Where is each baseline for a chosen line height (CSS `line-height`, half-leading)? | EN-035 |
| Where does the cap height come from, and how does a trim from it to the baseline place the lines (`text-box`)? | EN-035 |
| How are the lines placed vertically in the box (top, middle, bottom)? | EN-035 |
| Which algorithm balances the lines, and what does it keep (CSS `text-wrap-style: balance`)? | EN-036 |
| Where is the background drawn behind each line (a highlighter as a `<path>`)? | US-034 |
| What must every shape and text of E01, E14 and F05 keep? | every story |

## Tasks

One task = one commit, referenced as `SP-016.Tn`.

- [x] T1 — Inventory the formulas and behaviors of the stories (1 h)
- [x] T2 — Find, read and record the sources; write `DERIV-text-box` (3 h)
- [x] T3 — Write research note 0018, the domain's provisional rules and F14's stories (2 h)
