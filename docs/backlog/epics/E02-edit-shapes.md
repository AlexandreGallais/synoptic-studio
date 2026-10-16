---
id: E02
title: Edit shapes with precision
status: draft
---

# E02 — Edit shapes with precision

| Field                       | Content                                                                                                            |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| For                         | symbol designers                                                                                                   |
| who                         | need fast, exact placement and styling                                                                             |
| the solution                | selection, transform, color, alignment and history tools                                                           |
| is a                        | the editors' interaction layer                                                                                     |
| that                        | integer placement with Inkscape-like snapping                                                                      |
| unlike                      | approximate mouse placement                                                                                        |
| our solution                | every move lands on an integer; guides, snapping, alignment and constant spacing help                              |
| Business outcomes           | shapes are selected, moved, resized, colored, aligned, evenly spaced, undone and redone; the canvas zooms and pans |
| Leading indicators          | interaction features accepted                                                                                      |
| Non-functional requirements | snapping tolerance in screen pixels (`REF-INKSCAPE-SNAP`)                                                          |
| In scope                    | selector, marquee, snapping, fill and stroke colors, align, distribute, undo / redo (Q7), zoom and pan             |
| Out of scope                | —                                                                                                                  |
| Closure criteria            | the scenarios of `interaction.md` are demonstrated                                                                 |

## For the agent's work during E02

- **Lighter context** (Product Owner, 2026-10-14, at RET-002): measure what a session loads before its first word — `AGENTS.md`, `CLAUDE.md`, the rules, the memory index, the skills' descriptions — and make it smaller, so that each session starts with fewer tokens and the conversation memory fills up more slowly. A tool `context:size` gives the figures (`docs/tooling/agent-tools.md`); the retrospective of E02 compares them with E15's. Today: about 29 000 characters for those files, 27 000 more for the skills.

## Candidate features

| Feature                    | Status    |
| -------------------------- | --------- |
| Selection and transform    | to refine |
| Colors                     | to refine |
| Snapping                   | to refine |
| Align and constant spacing | to refine |
| History and navigation     | to refine |

## Ideas from the Product Owner (2026-10-11, refinement of F08)

- Dragging points on the drawing, several selected points moving together, on integers.
- Stretching a shape by its width and height from a fixed point chosen in the 3 × 3 grid, as Inkscape; strokes keep their width in pixels.
- When a polygon is converted to a free shape, a message saying what happens, with a **do not show again** box.

## Ideas from the Product Owner (2026-10-15, validation of E15)

For a later pass on the editor's ease of use; none is planned yet:

- **A search field over the installed fonts**: typing filters the font list.
- **A reset button on the stroke colors**: the underline's and the strike-through's colors get a button that makes them follow the text's color again, instead of the box "follows the text's color".
- **Fewer options**: simplify the Text tool's options, the stroke colors perhaps removed (kept for now, VAL-016).

## Ideas from the Product Owner (2026-10-16, first pass of VAL-018)

Not scheduled; for a later pass on text:

- **Trim trailing empty lines at run time**: line breaks with nothing after them take no space.
- **One line in a box**: in box mode, an option to show everything on one line, ignoring line breaks, when the box is wide enough.

## Ideas from the Product Owner (2026-10-16, second pass of VAL-018)

Not scheduled; for the later UI/UX epic:

- **One line in a box** (above): the Product Owner accepts it for later; it adds a model field, so it is not done now.
- **A curated set of fonts**: the software ships fonts suited to synoptic views; installing a local font is a choice, through an "Install" button listing the local fonts to pick from, beside importing a font file. Not every font is accepted (Comic Sans, Braille and emoji fonts are of no use in a synoptic view).
- **Turned text**: handle a turned text like rectangles, polygons and round shapes.

## Ideas from the Product Owner (2026-10-16, decisions after US-056)

Not scheduled; for the later UI/UX epic (US-059):

- **Fonts shipped**: free fonts that can be exported, installed with the software.
- **A font chosen from the computer**: the designer selects an installed font to import, and the software says whether it can be exported.
- **A font that cannot be embedded**: exported once the designer forces it after a confirmation, or refused outright — to decide then (ADR-0062 refuses it for now).
- **A font list that says more**: which fonts are usable, and which can be exported.
- **Fonts installed automatically**.

## Ideas from the Product Owner (2026-10-16, review of E15)

To do at the later UI/UX epic (REV-003):

- **Remove the text direction**: right to left is of no use, the Latin script being the strict target, as for fonts. It stays until then; that epic drops the options that make no sense, the aim being the fewest options doing the most.

## Ideas from the Product Owner (2026-10-11, refinement of F04)

- **A rotation mode**, entered by a button, showing the center of rotation (by default the center of the shape's box). The center is either **free**, fixed at an x and y; or one of the **nine anchors** of the shape's box, or a **point of the shape**, both following the shape when it moves or grows; or a **point or the center of another shape**, which only places it, at that x and y, without following that shape.
- **One angle only**: changing the center does not move the shape, only the next turns go about the new center; turning about another center changes the angle and the position, never the shape, so that back to 0 the shape is straight again, maybe elsewhere.
- **Handles on both boxes**: the corners of the turned box, as Figma shows them (the Product Owner's preference), and those of the straight box, dragged to resize the shape; a handle dragged changes width and height in the shape's own sides, and the straight box stays as computed.
- **Reference point of x and y**: the place of the straight box whose x and y are shown and typed, chosen among its anchors, as in Inkscape.
- Hiding the straight box, or showing it dashed, as an option.
