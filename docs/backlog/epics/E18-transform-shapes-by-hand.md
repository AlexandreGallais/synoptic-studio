---
id: E18
title: Transform shapes and points by hand
status: dropped
---

# E18 — Transform shapes and points by hand

**Dropped on 2026-10-25** (SP-033, ADR-0100, decision 1, [note 0034](../../research/notes/0034-regroup-the-epics.md) §4): its content went to [E21](./E21-resize-shapes-by-their-box.md), resize by the box, [E22](./E22-turn-shapes-about-a-chosen-center.md), turn about a chosen center, [E23](./E23-drag-points-and-handles.md), dragged points, [E24](./E24-compute-in-number-fields.md), number fields that compute, and [E25](./E25-keep-the-history-fast.md), a history kept fast; a stroke's width kept when stretched went to [E31](./E31-give-shapes-colors-and-strokes.md).

A list of features grouped by theme, with no closing of its own (ADR-0086). Cut from E02 on 2026-10-19 (SP-025): what changes a shape's size, angle or points with the mouse, once E02 lets several shapes be selected and moved; each change lands on integers and is undone by E02's history.

| Field | Content |
| --- | --- |
| For | symbol designers |
| who | need to resize, turn and reshape by hand without losing exact numbers |
| the solution | handles on the shapes' boxes, a rotation mode, dragged points and number fields that compute |
| is a | the editors' transform tools |
| that | every handle dragged gives integer inputs, as typed numbers do |
| unlike | fields typed one by one, the box resized from its top-left only |
| our solution | Inkscape's box handles with a fixed point, Figma's turned box, a rotation center chosen by the designer, points dragged together, `+10` typed in a field |
| Business outcomes | a shape is resized from any fixed point, turned about any center, its points dragged, in the playground |
| Leading indicators | features accepted |
| Non-functional requirements | integer inputs after every drag (ADR-0003); the turned shape's position the only stored fraction (ADR-0035) |
| In scope | the box's eight handles and the 3 × 3 reference point, the rotation mode and its centers, the handles of the turned and the straight boxes, dragged points and handles, arithmetic and relative values in number fields |
| Out of scope | snapping (E19); a turned text handled as a shape (E20); Figma's Bend tool (`shapes.md`, out of scope) |

## Features

Ordered by the agent at SP-025 (2026-10-19). A feature gets its id when it starts, its questions in its own short spike (ADR-0086).

| Feature | Expected stories | Status |
| --- | --- | --- |
| Resize by the box's handles | the eight handles, the opposite side fixed as in Inkscape (`shapes.md`, Rectangle); the reference point of x and y among the box's nine anchors, shown and typed; a polygon and a round shape stretched as their own rules say; Shift held on a corner keeps the shape's proportions (Product Owner, 2026-10-22) (3) | to refine |
| Turn about a chosen center | a rotation mode entered by a button, its center shown, by default the box's center; the center free at an x and y, or on one of the box's nine anchors; one angle only, a new center moving nothing (3) | to refine |
| Turn by handles, centers on points | handles on the turned box's corners, as Figma, and on the straight box's; the center on a point of the shape, following it, or on a point or the center of another shape, placed only; the straight box hidden or dashed (3) | to refine |
| Drag points and handles | in Points mode, a point dragged on integers, several selected points moved together, a curve's handles dragged (E14, out of scope there) (2 to 3) | to refine |
| Number fields that compute | `+`, `−`, `×`, `÷` and parentheses in every number field, as in a spreadsheet; `+10` typed in x or y moves several points or shapes by 10 (`interaction.md` §3, For later) (2) | to refine |
| A history step kept cheap with many shapes | a step, an undo and a redo changing only the counts of the shapes they change, each shape living in one run of consecutive states, so that a step on 1 000 shapes costs what one on a single shape does (9 ms today, about 2 ms aimed); proposed by AUD-043 on 2026-10-23 and placed here, before the drawings grow, rather than added to E02's F41 (1) | to refine |

## Ideas from the Product Owner (2026-10-11, refinement of F04)

- **A rotation mode**, entered by a button, showing the center of rotation (by default the center of the shape's box). The center is either **free**, fixed at an x and y; or one of the **nine anchors** of the shape's box, or a **point of the shape**, both following the shape when it moves or grows; or a **point or the center of another shape**, which only places it, at that x and y, without following that shape.
- **One angle only**: changing the center does not move the shape, only the next turns go about the new center; turning about another center changes the angle and the position, never the shape, so that back to 0 the shape is straight again, maybe elsewhere.
- **Handles on both boxes**: the corners of the turned box, as Figma shows them (the Product Owner's preference), and those of the straight box, dragged to resize the shape; a handle dragged changes width and height in the shape's own sides, and the straight box stays as computed.
- **Reference point of x and y**: the place of the straight box whose x and y are shown and typed, chosen among its anchors, as in Inkscape.
- Hiding the straight box, or showing it dashed, as an option.

## Ideas from the Product Owner (2026-10-11, refinement of F08)

- Dragging points on the drawing, several selected points moving together, on integers.
- Stretching a shape by its width and height from a fixed point chosen in the 3 × 3 grid, as Inkscape; strokes keep their width in pixels.
