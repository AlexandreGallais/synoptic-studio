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

- Click: select an object; Shift + click: add / remove.
- Selection rectangle (marquee).
- Moving and resizing: **integer** values.
- Properties panel: x, y, width, height typed on the keyboard.

## 3. Node editing

- Entered by an **explicit mode** (the Node tool, a large button), never only by a double-click: the target users are not power users (Product Owner, 2026-10-11, F03). Vertex handles are large enough to click easily.
- Editing the vertices of a rectangle or a regular polygon turns it into a free contour, by a voluntary action: it loses its number of corners and its anchor (F08).

- Shows every vertex of the selected shape.
- Selection of one or several vertices.
- Moving a vertex: integer.
- **Adding a vertex** between two existing vertices (on an edge).
- Deleting a vertex.
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
