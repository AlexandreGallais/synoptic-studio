---
id: SP-015
epic: E15
feature: F05
title: "Research for E15: text measured from font metrics and drawn as positioned lines"
status: ready
points: 2
---

# E15 · F05 · SP-015 — Research for E15: text measured from font metrics and drawn as positioned lines

Read and record the sources of every story of F05, and of what F14, F15 and F16 will need, from the research of the refinement (ADR-0020, research protocol §8).

## Acceptance criteria

- Given the research of the refinement, when the spike ends, then research note 0017 records what was read: SVG text support, `foreignObject`, font files (OpenType tables), measuring, line breaking, scripts, fonts embedding.
- Given the choice of the Product Owner (lines computed by the library, an HTML layer later), when the spike ends, then ADR-0052 records how text is measured and drawn, and the SVG elements it adds (`tspan`).
- Given the width of a text, when it is derived, then `DERIV-text-width` gives `size × Σ advance / unitsPerEm` plus kerning and spacing, with hand-computed cases.
- Given a default font, when it is chosen, then its licence allows bundling its metrics, recorded in `docs/tooling/dependencies.md`.

## Tasks

One task = one commit, referenced as `SP-015.Tn`.

- [ ] T1 — Research note 0017 and the references read (1 h)
- [ ] T2 — ADR-0052 and `DERIV-text-width` with hand-computed cases (2 h)
- [ ] T3 — The default font chosen, its licence recorded; F05's stories updated (1 h)
