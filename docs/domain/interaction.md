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
- A regular polygon becomes a free shape by a dedicated button (a voluntary action): it loses its number of corners and its anchor, and does not move. A polygon of width or height 0 has no box to stretch: the button is disabled (agent's choice). A rectangle needs no conversion: it is a free shape from the start, made by the rectangle tool (ADR-0032). Giving a vertex its own radius (F03) keeps the shape what it is (Product Owner, 2026-10-11).

- Shows every vertex of the **selected shape only**, never those of the other shapes. Its vertex 1 (the first of the contour, Q11) is marked red, vertex 2 green, so that the direction of the contour can be read (Product Owner, 2026-10-11); when handles coincide, the red one is on top (agent's choice, note 0006).
- A **list of the vertices** in contour order (Q11): number, x, y (at most 5 decimals), corner radius requested, and the effective one when it was reduced (Q8). A row is clicked to select its vertex, as its handle is on the drawing; useful when a handle is hard to hit.
- Selection of one or several vertices, on the drawing or in the list: click selects one; Shift + click or Ctrl + click (Cmd on macOS) on the drawing adds / removes; in the list, Shift + click selects a range and Ctrl + click (Cmd) adds / removes, as in any list (Product Owner, 2026-10-11). A range starts at the last row or handle clicked without Shift, and replaces the selection; with no start yet, Shift + click acts as a plain click (agent's choice, note 0006).
- The radius field applies to the selected vertices; it shows their common radius or Mixed, and is disabled when none is selected. The selection is cleared when the number of corners of a polygon is typed or when Shape mode is chosen (agent's choice, note 0006). In the playground the points have a radius field of their own, for the selected vertices, while the shape's radius field stays shown in both modes and rounds every vertex (Product Owner, VAL-005).
- Moving a vertex: typed x and y, integers, negative allowed (F08); each vertex keeps its number wherever it moves, the start of Q11 holds at creation only (Product Owner, 2026-10-11); dragged in the editor applications (E02). Only the vertices moved land on integers: a polygon converted to a free shape keeps its fractional vertices (Product Owner, 2026-10-11, Q27, ADR-0033). In the playground, the x and y of the one vertex selected are typed under the list of points and apply on Enter or when leaving the field, so that a minus sign or an emptied field moves nothing on the way; a typed width or height must be an integer and stretches the free shape, while the fractional size a conversion leaves stays as long as it is not retyped (US-014). A coordinate typed is an integer, the other keeps its value; the free shape keeps its fractions through every edit (Q28, ADR-0034). A vertex typed onto another, or vertices lying on one slanted line, are allowed while the points span some width and height, and may draw nothing visible: they are not refused; two vertices at the same place are flagged in the list of points, asking whether to delete one, never merged nor deleted on their own, since a contour may pass several times at the same place on purpose (Product Owner, VAL-005, Q29). Edges crossing after a move or a deletion, and a cap on typed coordinates, stay open (Q29).
- **Adding a vertex**: with two neighbouring vertices selected (the last and the first are neighbours), a button adds one exactly in the middle of their edge, sharp (radius 0), fractions allowed, no rounding; it comes right after the edge's first vertex, so the vertices after it are numbered one more and vertex 1 stays vertex 1; the edge keeps its place, but the two corners at its ends now have half of it to round; the new vertex is then selected alone, ready to be moved (agent's choice) (F08, US-015).
- Deleting vertices: a button or the Delete key (Backspace too, the key labelled "delete" on a Mac: agent's choice), on a rectangle or a free shape; each vertex goes with its radius, its two neighbours are joined by a straight edge (the last and the first included; Product Owner, VAL-005, Q28), the others keep their place and order, numbered from the first one left; the box shrinks around them (F08, US-016, note 0007). At least 3 vertices remain, and they must span some width and height: otherwise Delete is greyed and a line says why (agent's choice). A regular polygon is converted first. The selection is cleared after a deletion.
- **For later** (Product Owner, VAL-005, 2026-10-11): moving several selected vertices at once, relatively, as by typing `+10` in x or y; and basic arithmetic in every number field (`+`, `−`, `×`, `÷`, parentheses), as in a spreadsheet (« on peut faire des maths dans les inputs, et ça, c'est trop, trop utile »). For the editor applications, not F08.
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
