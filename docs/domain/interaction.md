# Interaction

How the user acts on shapes in the editors: tools, selection, node editing, snapping, alignment, tree and history.

## 1. Tools

| Tool | French | Role | Functional reference |
| --- | --- | --- | --- |
| Selector | Sélecteur | select, move, resize objects | Inkscape, Figma |
| Node tool | Édition de nœuds | select and modify the vertices of a shape | Inkscape (nodes), Figma (vector edit) |
| Rectangle | Rectangle | create a rectangle | — |
| Polygon | Polygone régulier | create a polygon with n corners | Inkscape Star/Polygon |
| Text | Texte | create a text | — |
| Port | Port | place a connection point (Symbol Editor) | — |
| Pipe | Tuyau | connect two ports (View Editor) | draw.io |

No pen tool and no freehand drawing (see `shapes.md` §3).

**Keys editable** (Product Owner, 2026-10-22): every shortcut and modifier key (Shift, Ctrl, Delete, the arrows…) can be changed by the user, simply, in a standard place for settings; the defaults follow Figma's. For E20.

**Adding a shape** (Product Owner, 2026-10-22, F35), as Figma does:

- a tool chosen, a click on the drawing adds a shape 100 × 100 centred on the point clicked, on integers; a text fits its content instead;
- **on integers** (agent's choice, US-063): the top-left of the shape's box, before it turns, is the point clicked less half the box's size, each coordinate rounded to the nearest integer, a tie rounded up (toward +∞), as CSS rounds (`nearestInteger`); a point clicked between two integers once the view's scale is applied, at (250.5, 140.4), places a shape 100 × 100 at (201, 90); a text's measured block, 87.3 × 38.4 clicked at (100, 50), sits at (56, 31) (`centredPosition`);
- **a text without metrics** (agent's choice, US-063, written at AUD-037): a text placed while its font has no metrics, no font being available yet (US-056), is not measured, its block taken as 0 × 0: the top-left of its box is the point clicked, rounded as above, (250.5, 140.6) giving (251, 141), since a size it does not have cannot be centred;
- **a kept text without metrics** (agent's choice, AUD-037): a text kept in the drawing whose font's metrics are no longer read, a family read ahead whose files could not be read again leaving the session (US-061) while Inter is not drawn (US-056), is drawn and exported with the metrics it was last drawn with, so that the shapes already there do not change;
- a press, a drag and a release draw the shape's box instead, the shape following the pointer until the release; a text dragged sits in a box of that size; with Shift held the box stays square, its side the larger of the two distances dragged, so that a round shape is a circle (Product Owner, 2026-10-22; Shift as Figma does, Inkscape uses Ctrl);
- **the box dragged** (agent's choice, US-064, `draggedBox`): the press and the pointer each rounded to the nearest integer, a tie rounded up, as a click is (`nearestInteger`); the box runs from the smaller to the larger of each coordinate, in any direction: (10, 10) to (130, 90) and (130, 90) to (10, 10) both give 120 × 80 at (10, 10), and (10.4, 9.6) to (130.5, 90.49) gives 121 × 80 at (10, 10); square, the side is laid from the press toward the pointer, down or right when the pointer is level with the press: (130, 90) to (10, 10) gives 120 × 120 at (10, −30); Shift pressed or released during the drag changes the shape at once, and a text never stays square; the new shape takes its kind's settings, straight, as a click's does, and the selector is armed again at the release;
- **a release on the press's integer point** is a click (US-063), the shape centred on the point pressed even if the pointer went away and came back (agent's choice, US-064);
- **a width or a height of 0** (agent's choice, US-064): a drag straight across or straight down adds the shape as drawn, its width or its height 0, as the model accepts (Q15): (10, 10) to (10.3, 90) adds a shape 0 × 80 at (10, 10), which draws nothing visible until a side is typed; Shift makes it a square of the distance dragged; a text gets a box of that size;
- the new shape is drawn above the others, the drawing an ordered list of shapes, the last drawn on top, flat until E13.
- **the toolbar** (agent's choice, US-063, as Figma does): a tool's button arms the tool and leaves the shape the panel edits as it is; a click on the drawing then places the tool's shape, the panel edits it, and the **selector** is armed again; with the selector armed, a click on the drawing places nothing (selecting by a click comes with F36);
- a shape refused, a value out of its range, is not kept when another is placed: only the shapes drawn are (agent's choice, US-063);
- in the playground, Reset and each step of the guided test start the drawing again as the page opens: the shapes placed and the settings each kind kept are forgotten, the Text tool's fields written back to the page's, so that the next text is Inter 32 Regular (agent's choice, US-063; the text's fields since AUD-037).
- **For later**: duplicating a shape, the offset of its first move remembered for the next copies, as Figma does, so that copies never pile up (Product Owner, 2026-10-22; not in E02).

**What a new shape starts with** (agent's choice, SP-026, 2026-10-22, the choice left to research by the Product Owner; to confirm at VAL-036; sources and reasons in [note 0027](../research/notes/0027-f35-what-a-new-shape-starts-with.md)):

- its box and place come from the click or the drag, as above, and it starts straight, its angle 0;
- every other setting is the one of the **last shape of its kind** the panel showed, so that a triangle placed again is the same triangle, as Inkscape's shape tools do (`REF-INKSCAPE-TUTORIAL-SHAPES`): a rectangle its corner radii, a polygon its corners, radii, stretched option and anchor, a text every option but its content and box — font, size, weight, color, spacing, decorations, case, line height, trim, background, ellipsis —, as Figma and Inkscape give a new text the last style used (`REF-INKSCAPE-1-2-TOOL-STYLE`, `REF-FIGMA-FORUM-TEXT-DEFAULT`);
- one memory per kind: a rectangle's values never reach a polygon, nor the other way round; before any shape of a kind, the values the page opens with (a radius of 10, 6 corners, the anchor at the center, Inter 32 Regular);
- a shape's settings are kept when the panel leaves it, for a new shape or because it became a free shape (US-063): a polygon set to 3 corners, then converted, gives the next polygon its 3 corners;
- a new text's content is `Text`, selected in the Content field so that typing replaces it: Figma and Inkscape start an empty text typed on the drawing, which the playground cannot do;
- example: a polygon set to 3 corners, a radius of 4 and its anchor at the bottom center, then a rectangle set to a radius of 0; the polygon tool clicked at (250, 140) adds a triangle of radius 4 anchored at the bottom center, its box 100 × 100 at (200, 90), straight; the rectangle tool clicked next adds a rectangle of radius 0;
- why not Figma's fixed defaults (a rectangle 100 × 100, a polygon a triangle, `REF-FIGMA-SHAPE-TOOLS`): Figma gets alike shapes by duplicating, components and styles, none of them in E02; the panel already shows the values, so what is shown is what is placed. A default set by a command (Sketch, Affinity Designer) and a reset to the page's values (Inkscape's button) are left out.

## 2. Selector

- Click: select an object; Shift + click or Ctrl + click (Cmd on macOS): add / remove (Product Owner, 2026-10-11). No range on the drawing.
- **Two layers** (Product Owner, 2026-10-22, F35): the drawing's SVG holds only the shapes and receives the clicks; a transparent SVG above it, which the pointer passes through, draws the outline of the shape under the pointer, the selection and every other help, so that the drawing is never polluted by them.
- Selection rectangle (marquee).
- Moving and resizing: **integer** values.
- **A move lands its anchor on integers** (Product Owner, 2026-10-22): while dragged the shape follows the pointer freely; at the release its anchor is placed on integers, the top-left of its box by default, the anchor chosen once anchors exist; the shape itself is never changed by a move: a side 10.5 long stays 10.5, its points keep their fractions until edited by hand (ADR-0033, ADR-0035). An option of free positions, off the integers, is an open question (Q43).
- Properties panel: x, y, width, height typed on the keyboard.

## 3. Node editing

- Entered by an **explicit mode** (the Node tool, a large button; in the playground, the **Points** button of the toolbar, next to **Shape**), never only by a double-click: the target users are not power users (Product Owner, 2026-10-11, F03). Vertex handles are large enough to click easily.
- A regular polygon becomes a free shape by a dedicated button (a voluntary action): it loses its number of corners and its anchor, and does not move. A polygon of width or height 0 has no box to stretch: the button is disabled (agent's choice). A rectangle needs no conversion: it is a free shape from the start, made by the rectangle tool (ADR-0032). Giving a vertex its own radius (F03) keeps the shape what it is (Product Owner, 2026-10-11).

- Shows every vertex of the **selected shape only**, never those of the other shapes. Its vertex 1 (the first of the contour, Q11) is marked red, vertex 2 green, so that the direction of the contour can be read (Product Owner, 2026-10-11); when handles coincide, the red one is on top (agent's choice, note 0006).
- A **list of the vertices** in contour order (Q11): number, x, y (at most 5 decimals), corner radius requested, and the effective one when it was reduced (Q8). A row is clicked to select its vertex, as its handle is on the drawing; useful when a handle is hard to hit.
- Selection of one or several vertices, on the drawing or in the list: click selects one; Shift + click or Ctrl + click (Cmd on macOS) on the drawing adds / removes; in the list, Shift + click selects a range and Ctrl + click (Cmd) adds / removes, as in any list (Product Owner, 2026-10-11). A range starts at the last row or handle clicked without Shift, and replaces the selection; with no start yet, Shift + click acts as a plain click (agent's choice, note 0006).
- The radius field applies to the selected vertices; it shows their common radius or Mixed, and is disabled when none is selected. The selection is cleared when the number of corners of a polygon is typed or when Shape mode is chosen (agent's choice, note 0006). In the playground the points have a radius field of their own, for the selected vertices, while the shape's radius field stays shown in both modes and rounds every vertex (Product Owner, VAL-005).
- **Moving a vertex** (F08):
  - typed x and y, integers, negative allowed; dragged in the editor applications (E18);
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

| Target | Description |
| --- | --- |
| Integer grid | every position is rounded to an integer: for a turned shape, the x and y of its straight box, its stored position staying fractional (ADR-0035) |
| Objects | edges, corners, centers, midpoints of bounding boxes; vertices; ports |
| Alignment | temporary horizontal / vertical guides when a point aligns with a point of another object |
| Distribution | stop at the position where the gap to a neighbor equals the existing gap between two other objects |

Rules:

- Tolerance expressed in **screen pixels** (zoom-independent).
- Priority: snapping to an object wins over alignment snapping, even a farther one (Inkscape behavior).

## 5. Colors, align and distribute

### Colors

F16; decided at F05's refinement, round 2, 2026-10-14; the rules marked "confirmed at VAL-016" were read from that decision by the agent at SP-017, US-035 and AUD-017, 2026-10-15, confirmed at VAL-016; ADR-0056, `DERIV-color-spaces`.

- **A color**: a hexadecimal value and an opacity, an integer from 0 to 100 %, in two fields always shown beside its swatch, for a quick copy and paste, as Figma's; RGB fields and CSS color names are not offered.
- **Uses**: it serves the text's color today, its background (the agent's reading of F16's fourth criterion, confirmed at VAL-016), and its underline's and strike-through's (US-041), the shapes' fills and strokes with E04.
- **Typed or pasted** (confirmed at VAL-016):
  - with or without `#`, in upper or lower case, white space around it ignored, in 6 digits, in 3 digits each doubled (`f80` gives `#ff8800`), or in 8 or 4 digits whose last pair or digit is the alpha, which sets the opacity to the nearest percent (`#0000ffcc` gives `#0000ff` and 80 %, `#F80C` gives `#ff8800` and 80 %);
  - it is kept as `#` and six lower-case digits, the form of CSS's HTML-compatible serialization (`REF-CSS-COLOR-4`, §16.2.1);
  - the model accepts that form only, so every reader of a color (a field typed or pasted, data loaded) turns it to lower case first, and a color of F05 or F14 written in upper case is turned to lower case when read (agent, SP-017 audit, confirmed at VAL-016);
  - anything else is refused when the field is left or Enter pressed: the field shows again the color it had when it got the focus, or the last one it kept, and a line under it says why (US-035, confirmed at VAL-016);
  - while it is typed, the text is drawn with what the field holds, refused as any other field until it is a color;
  - a color copied is six digits; the opacity is copied from its own field.
- **The underline's and the strike-through's colors** (US-041, the agent's reading, confirmed at VAL-016):
  - each has the same swatch, fields and picker as the text's, with a box "follows the text's color", ticked as the page opens;
  - while ticked, its fields show the text's color and opacity and the stroke is drawn in them;
  - a hexadecimal or an opacity committed (the field left or Enter pressed) and accepted, or picked, unticks it, a value refused or being typed leaving it ticked, the stroke keeping that color when the text's changes;
  - ticked again, it follows the text's color again (`shapes.md`, Text).
- **The picker** (US-035, AUD-017, confirmed at VAL-016; integers since ADR-0058, US-040):
  - it opens on a click on the swatch and closes on another click on it, on Escape or on a click outside;
  - it holds a hue slider from 0 to 359°, holding whole degrees (`#ff8000` at 30°; a hue rounding to 360 is shown 0; Product Owner, 2026-10-15, ADR-0058), and an opacity slider from 0 to 100 %;
  - it holds a plane in HSL or in HSB, a choice between the two, HSB first (the agent's own choice, no source read, confirmed at VAL-016), the plane's three values (hue, saturation, lightness or brightness) as numbers, and the hexadecimal and opacity fields, so the hexadecimal translation is always shown;
  - in the playground the picker opens under its color's fields, which stay in sight, a click on them does not close it, and Escape gives the focus back to the swatch;
  - another model chosen reads the color's values again in it, its own hue, the same in both models, a grey keeping the hue the picker holds (`#7e7f80`, its hue typed 200 in HSB, shows HSL 210°, 1 %, 50 %, its own hue).
- **The plane** shows the colors of the hue chosen: the saturation across, from 0 at the left to 100 % at the right, the lightness or brightness up, from 0 at the bottom to 100 % at the top; a click or a drag picks both (confirmed at VAL-016, `DERIV-color-spaces`, step 10).
- **The values shown and typed** in HSL and HSB are whole numbers, never a decimal (`#ff8000` is HSL 30°, 100 %, 50 % and HSB 30°, 100 %, 100 %):
  - the hexadecimal stays the truth: a color typed or pasted in hexadecimal is kept exactly as typed and never altered afterwards, and the values keep their decimals inside, unshown;
  - the picker's inputs reach fewer colors than the hexadecimal does, which is accepted (Product Owner, 2026-10-15);
  - editing one value rebuilds the color from that typed integer and the exact values of the two others, computed from the hexadecimal, so that editing one never moves the others (Product Owner, 2026-10-15, ADR-0058, superseding ADR-0056's one decimal; US-040);
  - so `#336699` is HSB 210°, 66.66667 %, 60 %, shown 210, 67, 60; its brightness typed 61 gives `#34689c`, where the three shown integers would give `#33679c`;
  - a value typed equal to the one shown changes nothing (agent's reading, confirmed at VAL-016);
  - while the picker is open it holds the values given, and rebuilds each edit against the color its values were last read from (when it opened, a hexadecimal typed, another model chosen);
  - a value equal to the one read stands for its exact value, a value typed stays as typed, so that H 30, S 100, L 50 typed one after the other on `#1e293b` give `#ff8000` (agent's reading, ADR-0059, US-040, confirmed at VAL-016);
  - the picker's look is polished later.
- **From HSL or HSB to the hexadecimal** (US-035, confirmed at VAL-016):
  - a value typed or picked in HSL or HSB becomes the hexadecimal by rounding each of red, green and blue to the nearest integer, a tie rounded up; the hexadecimal and the opacity are what the text keeps;
  - a grey has no hue of its own: it shows 0°, while an open picker keeps the hue its slider shows, so that a grey made lighter keeps it;
  - a number typed beyond an end of a value counts at that end (a hue of 400 is 360, shown 0, while the slider's End key gives 359°; −10 gives 0);
  - a value left empty shows its value again, and a click on the opacity slider gives the nearest percent.
- **The keyboard** (confirmed at VAL-016, `REF-WAI-APG-SLIDER`):

  | Where | Keys | Change |
  | --- | --- | --- |
  | a slider | Right, Up | adds 1 (1° or 1 %) to the whole value shown (30° gives 31°, ADR-0058) |
  | a slider | Left, Down | takes 1 away |
  | a slider | Page Up, Page Down | add and take 10 |
  | a slider | Home, End | go to 0 and to the end (359° for the hue, 100 % for the opacity) |
  | the plane | Left, Right | change the saturation by 1 % |
  | the plane | Up, Down | change the lightness or brightness |
  | the plane | Page Up, Page Down, Home, End | the same as on a slider, on the saturation |

  Each slider and the plane carry their name and value for a screen reader.

### Align and distribute

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
- Undo / redo (**Q7**, settled by the Product Owner on 2026-10-22): one step per finished action — a drag at its release, whatever its path; a value typed when Enter is pressed or the field left; a text when its editing is left; an addition, a deletion. Selecting alone is never a step. The history is bounded by a memory size the user sets, never by a count of steps. How it is built, a history of commands or of states, is chosen at the feature's spike from read sources.
