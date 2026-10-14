---
id: E10
title: Draw and reuse static drawings
status: draft
---

# E10 — Draw and reuse static drawings

| Field                       | Content                                                                                         |
| --------------------------- | ----------------------------------------------------------------------------------------------- |
| For                         | business users                                                                                  |
| who                         | need to add free-form decoration to views                                                       |
| the solution                | drawing tools and the drawing library                                                           |
| is a                        | static drawings with more freedom than symbols                                                  |
| that                        | drawings reused across views and projects                                                       |
| unlike                      | redrawing the same decorations                                                                  |
| our solution                | a group becomes a reusable drawing                                                              |
| Business outcomes           | a business user draws, groups and saves a drawing, then reuses it in another view and project   |
| Leading indicators          | Q12 settled                                                                                     |
| Non-functional requirements | —                                                                                               |
| In scope                    | drawing tools with a Figma-like Bézier pen (ADR-0018), group to drawing, shared drawing library |
| Out of scope                | parameters and animations                                                                       |
| Closure criteria            | a drawing saved in one project is reused in another                                             |

## Notes for later

- **Point 1 alike in every shape** (Product Owner, 2026-10-14, at REV-002; placed here, the last epic before E11 fixes the file format, so that stored shapes need no renumbering).
  - **The idea.** Every shape starts at its lowest right point and runs clockwise, as a regular polygon already does from the right end of its base (Q18). Today the rectangle starts at its top-left corner (Q11) and the round shape at its top point.
  - **Research (2026-10-14).** SVG 2 starts a circle and an ellipse "at the "3 o'clock" point on the radius" and runs clockwise. It starts a `rect` on its top side, near the top-left corner (`REF-SVG2-SHAPES`). No study of what people prefer was found.
  - **The agent's recommendation.** The rectangle starts at its bottom-right corner. The round shape starts at its right point, as in SVG 2: the right point is the round shape's lowest right point that is on its outline, and the next one clockwise is the bottom.
  - **Before deciding,** check what changes: point numbers in saved shapes, guided steps and test values.

## Candidate features

| Feature          | Status    |
| ---------------- | --------- |
| Drawing tools    | to refine |
| Group to drawing | to refine |
| Drawing library  | to refine |
