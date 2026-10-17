---
id: VAL-011
epic: E14
feature: F12
title: Validate F12
status: done
points: 1
---

# E14 · F12 · VAL-011 — Validate F12

The Product Owner tries each way of typing a handle in the playground, guided by short steps, and says what they see (ADR-0020, ADR-0045).

## Acceptance criteria

- Given the playground, when the Product Owner follows one guided step per method of typing a handle (seven, at the first session), then each method is tried and their choice for Q32 is recorded.
- Given the changes asked at the first session (EN-026, US-027, US-028), when the Product Owner tries the nine options of the second session, then each behaves as the domain states.
- Given the radius/handles switch and the modes kept, when tried, then each behaves as `docs/domain/shapes.md` "Points with handles" states.

## Validation, first session (2026-10-13)

The Product Owner ran the guided steps and decided:

| Remark | Routed to |
| --- | --- |
| keep two methods: x and y on the page, as in Figma, and an angle and a length from the side, which gives the same values at the four corners of a rectangle, so that they can be copied | Q32 settled; EN-026 removes the others |
| no pasted curve data (`C`, `cubic-bezier()`), no tension: not needed | EN-026 |
| keep the three mirror modes | EN-026 |
| a corner beside a curved side should keep its rounding instead of turning sharp | F12.AC7; SP-012, then US-027 |
| copying a group of a point's settings onto another point | E14's UX pass (epic) |
| the status line showed the person's turn while subagents still ran | fixed on `develop` (feedback log) |

The validation resumed once EN-026, SP-012, US-027 and AUD-012 are done; the guided steps are cut to the methods kept.

## Validation, second session (2026-10-14)

The Product Owner ran the nine steps and accepted every option as built:

| Remark | Routed to |
| --- | --- |
| the reduced radius beside a curve as built (47.7 and 46.9), the circle never crossing it | ADR-0047, `shapes.md` |
| a point added beside a large fillet splits the whole curve at its middle (Q34) | Q34 settled |
| a lens, points on one line with bulging sides, stays refused (Q33) | Q33 settled |
| F10 (round shape) next, then F13 (every calculation named), presented together at their end | E14's feature order |

## Tasks

- [x] T1 — The guided test, cut to the options kept: nine steps, one per option, tested end to end (`f12-steps.test.ts`) (1 h)
- [x] T2 — The demo page as a list of the options (30 min)
- [x] T3 — The Product Owner's validation, Q33, Q34 and the reduced radius answered (1 h)
