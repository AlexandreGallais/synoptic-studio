# Interaction

How the user acts on shapes in the editors: tools, selection, node editing, snapping, alignment, tree and history.

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

- Entered by an **explicit mode** (the Node tool, a large button; in the playground, the **Points** button of the toolbar, next to **Shape**), never only by a double-click: the target users are not power users (Product Owner, 2026-10-11, F03). Vertex handles are large enough to click easily.
- A regular polygon becomes a free shape by a dedicated button (a voluntary action): it loses its number of corners and its anchor, and does not move. A polygon of width or height 0 has no box to stretch: the button is disabled (agent's choice). A rectangle needs no conversion: it is a free shape from the start, made by the rectangle tool (ADR-0032). Giving a vertex its own radius (F03) keeps the shape what it is (Product Owner, 2026-10-11).

- Shows every vertex of the **selected shape only**, never those of the other shapes. Its vertex 1 (the first of the contour, Q11) is marked red, vertex 2 green, so that the direction of the contour can be read (Product Owner, 2026-10-11); when handles coincide, the red one is on top (agent's choice, note 0006).
- A **list of the vertices** in contour order (Q11): number, x, y (at most 5 decimals), corner radius requested, and the effective one when it was reduced (Q8). A row is clicked to select its vertex, as its handle is on the drawing; useful when a handle is hard to hit.
- Selection of one or several vertices, on the drawing or in the list: click selects one; Shift + click or Ctrl + click (Cmd on macOS) on the drawing adds / removes; in the list, Shift + click selects a range and Ctrl + click (Cmd) adds / removes, as in any list (Product Owner, 2026-10-11). A range starts at the last row or handle clicked without Shift, and replaces the selection; with no start yet, Shift + click acts as a plain click (agent's choice, note 0006).
- The radius field applies to the selected vertices; it shows their common radius or Mixed, and is disabled when none is selected. The selection is cleared when the number of corners of a polygon is typed or when Shape mode is chosen (agent's choice, note 0006). In the playground the points have a radius field of their own, for the selected vertices, while the shape's radius field stays shown in both modes and rounds every vertex (Product Owner, VAL-005).
- **Moving a vertex** (F08):
  - typed x and y, integers, negative allowed; dragged in the editor applications (E02);
  - each vertex keeps its number wherever it moves; the start of Q11 holds at creation only (Product Owner, 2026-10-11);
  - a coordinate typed is an integer, the other keeps its value; the free shape keeps its fractions through every edit (Q27, Q28, ADR-0033, ADR-0034);
  - in the playground, the fields under the list of points apply on Enter or when leaving the field, so that a minus sign or an emptied field moves nothing; a typed width or height must be an integer, and the fractional size a conversion leaves stays until retyped (US-014).
- **Degenerate results** (Product Owner, VAL-005, Q29): a vertex typed onto another, or vertices on one slanted line, are allowed while the points span some width and height; two vertices at the same place are flagged in the list with an offer to delete one, never merged nor deleted on their own; crossing edges are allowed and flagged; typed coordinates are not capped. The flags come with the editor applications.
- **Adding a vertex**: with two neighbouring vertices selected (the last and the first are neighbours), a button adds one exactly in the middle of their edge, sharp (radius 0), fractions allowed, no rounding; it comes right after the edge's first vertex, so the vertices after it are numbered one more and vertex 1 stays vertex 1; the edge keeps its place, but the two corners at its ends now have half of it to round; on a curved side, the curve is split instead (F12, `shapes.md` "Points with handles"); the new vertex is then selected alone, ready to be moved (agent's choice) (F08, US-015).
- Deleting vertices: a button or the Delete key (Backspace too, the key labelled "delete" on a Mac: agent's choice), on a rectangle or a free shape; each vertex goes with its radius, its two neighbours are joined by a straight edge (the last and the first included; Product Owner, VAL-005, Q28) — since F12, by a side curved by their outer handles if they have some (`shapes.md` "Points with handles"), the others keep their place and order, numbered from the first one left; the box shrinks around them (F08, US-016, note 0007). At least 3 vertices remain, and they must span some width and height: otherwise Delete is greyed and a line says why (agent's choice). A regular polygon is converted first. The selection is cleared after a deletion.
- **For later** (Product Owner, VAL-005, 2026-10-11): moving several selected vertices at once, relatively, as by typing `+10` in x or y; and basic arithmetic in every number field (`+`, `−`, `×`, `÷`, parentheses), as in a spreadsheet. For the editor applications, not F08.
- **Corner radius per vertex**: numeric field, applied to the selected vertices.

## 4. Snapping

Functional reference: Inkscape 1.2+ (`REF-INKSCAPE-SNAP`), Figma, draw.io.

| Target       | Description                                                                                                                                     |
| ------------ | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| Integer grid | every position is rounded to an integer: for a turned shape, the x and y of its straight box, its stored position staying fractional (ADR-0035) |
| Objects      | edges, corners, centers, midpoints of bounding boxes; vertices; ports                                                                           |
| Alignment    | temporary horizontal / vertical guides when a point aligns with a point of another object                                                       |
| Distribution | stop at the position where the gap to a neighbor equals the existing gap between two other objects                                              |

Rules:

- Tolerance expressed in **screen pixels** (zoom-independent).
- Priority: snapping to an object wins over alignment snapping, even a farther one (Inkscape behavior).

## 5. Colors, align and distribute

- **Colors** (F16; decided at F05's refinement, round 2, 2026-10-14; the rules marked "provisional" were read from that decision by the agent at SP-017, 2026-10-15, to confirm at VAL-016; ADR-0056, `DERIV-color-spaces`):
  - a color is a hexadecimal value and an opacity, an integer from 0 to 100 %, in two fields always shown beside its swatch, for a quick copy and paste, as Figma's; RGB fields and CSS color names are not offered. It serves the text's color and its background today, the shapes' fills and strokes with E04;
  - **typed or pasted** (provisional): with or without `#`, in upper or lower case, white space around it ignored, in 6 digits, in 3 digits each doubled (`f80` gives `#ff8800`), or in 8 or 4 digits whose last pair or digit is the alpha, which sets the opacity to the nearest percent (`#0000ffcc` gives `#0000ff` and 80 %, `#F80C` gives `#ff8800` and 80 %); it is kept as `#` and six lower-case digits, the form of CSS's HTML-compatible serialization (`REF-CSS-COLOR-4`, §16.2.1); the model accepts that form only, so every reader of a color (a field typed or pasted, data loaded) turns it to lower case first, and a color of F05 or F14 written in upper case is turned to lower case when read (agent, SP-017 audit, provisional); anything else is refused and the field shows the color it had. A color copied is six digits; the opacity is copied from its own field;
  - **the picker** opens on a click on the swatch and closes on another click on it, on Escape or on a click outside (provisional). It holds a hue slider from 0 to 359.9°, holding the hue to one decimal (`#ff8000` at 30.1°; a hue of 360 is shown 0.0), an opacity slider from 0 to 100 %, a plane in HSL or in HSB, a choice between the two, HSB first (the agent's own choice, no source read, provisional), the plane's three values (hue, saturation, lightness or brightness) as numbers, and the hexadecimal and opacity fields, so the hexadecimal translation is always shown;
  - **the plane** shows the colors of the hue chosen: the saturation across, from 0 at the left to 100 % at the right, the lightness or brightness up, from 0 at the bottom to 100 % at the top; a click or a drag picks both (provisional, `DERIV-color-spaces`, step 10);
  - **the values shown** in HSL and HSB have one decimal (`#FF8000` is HSL 30.1°, 100 %, 50 % and HSB 30.1°, 100 %, 100 %), so that typed back they give the same hexadecimal; with integers, nearly 9 colors in 10 would come back changed (`#336699` shown as HSB 210°, 67 %, 60 % would give `#326699`) (provisional, ADR-0056);
  - a value typed or picked in HSL or HSB becomes the hexadecimal by rounding each of red, green and blue to the nearest integer, a tie rounded up; the hexadecimal and the opacity are what the text keeps; a grey has no hue of its own: it shows 0°, while an open picker keeps the hue its slider shows, so that a grey made lighter keeps it (provisional);
  - **the keyboard** (provisional, `REF-WAI-APG-SLIDER`): on a slider, the Right and Up arrows add 1 (1° or 1 %) to the value it holds (30.1° gives 31.1°), the Left and Down arrows take 1 away, Page Up and Page Down 10, Home and End go to 0 and to the end (359.9° for the hue, 100 % for the opacity); on the plane, Left and Right change the saturation by 1 %, Up and Down the lightness or brightness, with the same Page and Home keys on the saturation; each slider and the plane carry their name and value for a screen reader.

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
