---
id: SP-009
epic: E01
feature: F06
title: Research for F06
status: done
points: 3
---

# E01 · F06 · SP-009 — Research for F06

Find, read and record the sources of every story of F06 before its implementation (ADR-0020).

## Acceptance criteria

- Given each story of F06, when the spike ends, then every formula and behavior it needs has a verified `REF-*` or a `DERIV-*`.
- Given the sources found, when the spike ends, then the stories cite them, and a research note records what was found and what remains open.

## Questions

- The rule of the stretched polygon (research request 0011): a polygon of `n` corners, its base flat at the bottom, symmetric about the vertical axis, touching the four sides of a `w × h` box, its sides as even as possible; which measure of « as even », a closed form or an algorithm for each `n` from 3 to 12, and what happens when equal sides are impossible (a triangle or a square have no freedom left).
- The model: the option stored in the regular polygon, and its effect on the anchor; an ADR if the polygon's model changes.
- Exact or floating values: the stretched points are derived (ADR-0003); test values computable by hand for at least the triangle, the square and the hexagon.

## Tasks

One task = one commit, referenced as `SP-009.Tn`.

- [x] T1 — Research request 0011 answered (by the Product Owner in claude.ai), sources read and recorded, research note; rule C chosen by the Product Owner (1 h)
- [x] T2 — Derivation `DERIV-stretched-polygon`: the λ family, the corner guard, the plain stretch past it (2 h). Open points from the research, each a step to settle:
  - the aspect grows with `λ` for each `n` from 5 to 12, and the sound range is one interval containing `λ = 1`: proved, or tested exhaustively → tested over 6 001 values of `ln λ` for each `n` (step 5), not proved;
  - a read source for `a sin φ − b cos φ = c ⇒ φ = atan(b/a) + asin(c / √(a² + b²))` → not needed: the code bisects for every `n`, and the hand checks substitute the angle;
  - `λ_lo` and `λ_hi` computed at run time or tabulated → computed at each call;
  - ECMA-376 read before any citation of the PowerPoint presets (or the citation dropped). → dropped: the presets are context only, `REF-ECMA-376` stays unverified and is not cited.
- [x] T3 — ADR (the stretched option in the polygon's model), stories updated (1 h)
