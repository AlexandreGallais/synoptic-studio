# Interaction

## 1. Tools

| Tool      | French            | Role                                      | Functional reference                  |
| --------- | ----------------- | ----------------------------------------- | ------------------------------------- |
| Selector  | Sélecteur         | select, move, resize objects              | Inkscape, Figma                       |
| Node tool | Édition de nœuds  | select and modify the vertices of a shape | Inkscape (nodes), Figma (vector edit) |
| Rectangle | Rectangle         | create a rectangle                        | —                                     |
| Polygon   | Polygone régulier | create a polygon with n corners           | Inkscape Star/Polygon                 |
| Text      | Texte             | create a text                             | —                                     |
| Port      | Port              | place a connection point (Symbol Editor)  | —                                     |
| Pipe      | Tuyau             | connect two ports (View Editor)           | draw.io                               |

No pen tool and no freehand drawing (see `shapes.md` §3).

## 2. Selector

- Click: select an object; Shift + click or Ctrl + click (Cmd on macOS): add / remove (Product Owner, 2026-10-11). No range on the drawing.
- Selection rectangle (marquee).
- Moving and resizing: **integer** values.
- Properties panel: x, y, width, height typed on the keyboard.

## 3. Node editing

- Entered by an **explicit mode** (the Node tool, a large button; Points mode in the playground, next to Shape mode), never only by a double-click: the target users are not power users (Product Owner, 2026-10-11, F03). Vertex handles are large enough to click easily.
- A regular polygon becomes a free shape by a dedicated button (a voluntary action): it loses its number of corners and its anchor, and does not move. A rectangle needs no conversion: it is a free shape from the start, made by the rectangle tool (ADR-0032). Giving a vertex its own radius (F03) keeps the shape what it is (Product Owner, 2026-10-11).

- Shows every vertex of the **selected shape only**, never those of the other shapes. Its vertex 1 (the first of the contour, Q11) is marked red, vertex 2 green, so that the direction of the contour can be read (Product Owner, 2026-10-11); when handles coincide, the red one is on top (agent's choice, note 0006).
- A **list of the vertices** in contour order (Q11): number, x, y (at most 5 decimals), corner radius requested, and the effective one when it was reduced (Q8). A row is clicked to select its vertex, as its handle is on the drawing; useful when a handle is hard to hit.
- Selection of one or several vertices, on the drawing or in the list: click selects one; Shift + click or Ctrl + click (Cmd on macOS) on the drawing adds / removes; in the list, Shift + click selects a range and Ctrl + click (Cmd) adds / removes, as in any list (Product Owner, 2026-10-11). A range starts at the last row or handle clicked without Shift, and replaces the selection; with no start yet, Shift + click acts as a plain click (agent's choice, note 0006).
- The radius field applies to the selected vertices; it shows their common radius or Mixed, and is disabled when none is selected. The selection is cleared when the number of corners changes or when Shape mode is chosen (agent's choice, note 0006).
- Moving a vertex: typed x and y, integers (F08); dragged in the editor applications (E02). Only the vertices moved land on integers: a polygon converted to a free shape keeps its fractional vertices (Product Owner, 2026-10-11; comes with F08 — until then a free shape stores integer points only, ADR-0032).
- **Adding a vertex**: with two neighbouring vertices selected, a button adds one exactly in the middle of their edge, fractions allowed, no rounding (F08, to come).
- Moving a vertex: typed x and y, integers, negative allowed (F08); each vertex keeps its number wherever it moves, the start of Q11 holds at creation only (Product Owner, 2026-10-11); dragged in the editor applications (E02). Only the vertices moved land on integers: a polygon converted to a free shape keeps its fractional vertices (Product Owner, 2026-10-11).
- **Adding a vertex**: with two neighbouring vertices selected, a button adds one exactly in the middle of their edge, fractions allowed, no rounding (F08).
- Deleting vertices: a button or the Delete key, on a rectangle or a free shape; at least 3 vertices remain; a regular polygon is converted first (F08).
- **Corner radius per vertex**: numeric field, applied to the selected vertices.

## 4. Snapping

Functional reference: Inkscape 1.2+ (`REF-INKSCAPE-SNAP`), Figma, draw.io.

| Target       | Description                                                                                        |
| ------------ | -------------------------------------------------------------------------------------------------- |
| Integer grid | every position is rounded to an integer                                                            |
| Objects      | edges, corners, centers, midpoints of bounding boxes; vertices; ports                              |
| Alignment    | temporary horizontal / vertical guides when a point aligns with a point of another object          |
| Distribution | stop at the position where the gap to a neighbor equals the existing gap between two other objects |

Rules:

- Tolerance expressed in **screen pixels** (zoom-independent).
- Priority: snapping to an object wins over alignment snapping, even a farther one (Inkscape behavior).

## 5. Colors, align and distribute

- Colors: fill and stroke colors picked from a palette or typed.

- Align: left, horizontal center, right, top, vertical middle, bottom.
- Distribute: equal horizontal / vertical spacing (constant gaps).
- Shape builder (Inkscape-like): select shapes, then keep or merge the regions built from their edges and intersections ([shapes.md](./shapes.md) §5).
- Works **whatever the group** of the objects: computed on bounding boxes in world coordinates.

## 6. Shape tree (layers)

- Hierarchy of groups and shapes, stacking order.
- Rename, hide, lock, reorder.
- The tree in the model (groups, children, layout box) comes with E13, before booleans (Q21).
- The layers panel (rename, hide, lock, reorder): priority end of project.

## 7. Navigation and history

- Zoom and pan.
- Undo / redo: command-based history (**Q7**, to be confirmed).
