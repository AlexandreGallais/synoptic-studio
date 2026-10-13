---
id: VAL-011
epic: E14
feature: F12
title: Validate F12
status: in-progress
points: 1
---

# E14 · F12 · VAL-011 — Validate F12

The Product Owner tries each way of typing a handle in the playground, guided by short steps, and says what they see (ADR-0020, ADR-0045).

## Acceptance criteria

- Given the playground, when the Product Owner follows one guided step per method of typing a handle (seven), then each method is tried and their choice for Q32 is recorded.
- Given the radius/handles switch and the four modes, when tried, then each behaves as `docs/domain/shapes.md` "Points with handles" states.

## Validation, first session (2026-10-13)

The Product Owner ran the guided steps and decided:

| Remark                                                                                                                                                                                  | Routed to                              |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------- |
| keep two methods: x and y on the page, as in Figma, and an angle and a length from the side, which gives the same values at the four corners of a rectangle, so that they can be copied | Q32 settled; EN-026 removes the others |
| no pasted curve data (`C`, `cubic-bezier()`), no tension: not needed                                                                                                                    | EN-026                                 |
| keep the three mirror modes                                                                                                                                                             | EN-026                                 |
| a corner beside a curved side should keep its rounding instead of turning sharp                                                                                                         | F12.AC7; SP-012, then US-027           |
| copying a group of a point's settings onto another point                                                                                                                                | E14's UX pass (epic)                   |
| the status line showed the person's turn while subagents still ran                                                                                                                      | fixed on `develop` (feedback log)      |

The validation resumes once EN-026, SP-012, US-027 and AUD-012 are done; the guided steps are cut to the methods kept.
