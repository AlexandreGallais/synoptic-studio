---
id: US-044
epic: E15
feature: F17
title: The HTML layer removed
status: done
points: 3
---

# E15 · F17 · US-044 — The HTML layer removed

As a symbol designer, I want each text written once, as positioned lines, so that every reader shows the same text and the file stays simple.

From the Product Owner's validation of E15 (2026-10-15): the HTML layer brings no speed and served only scripts industrial views do not need (ADR-0060). Withdraws F15's criterion 3.

## Acceptance criteria

- Given a text with every option on (a box, the ellipsis, underline and strike-through, a background), when the drawing is written, then it holds no `switch`, no `foreignObject` and no XHTML: one `<g>` with its `<text>` and paths, as with the layer off before.
- Given the code of `src/`, when the checks run, then a test fails on any `switch` or `foreignObject` written, as ADR-0060 allows neither.
- Given the playground, when it opens, then it has no **HTML layer** option nor guided step about it; F15's criterion 3 is withdrawn and the demo page says why.

## Product Owner test

| Step | Do | You should see |
| --- | --- | --- |
| 1 | Open the playground, look at the document's options | **Embed fonts** only, no **HTML layer** |
| 2 | Export SVG a text in a box, open the file as text | no `switch`, no `foreignObject` |

## Tasks

One task = one commit, referenced as `US-044.Tn`.

- [x] T1 — The layer's code removed from the library (EN-041) with its tests; a test refusing `switch` and `foreignObject` (3 h)
- [x] T2 — The playground's option, its guided steps and the demo page updated (2 h)
- [x] T3 — References no longer cited, `DERIV-html-layer` marked as a record, F15's criterion 3 withdrawn (1 h)
