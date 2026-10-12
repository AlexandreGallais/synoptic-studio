# 0006 — Sources of F03: per-vertex corner radius

Spike SP-004, 2026-10-11. Claude Code's own research; every source below was read.

## Clamping with unequal radii (EN-010)

Nothing new to derive: `DERIV-local-radius-clamp` already works per vertex. Each edge compares its length with the sum of the setbacks of its two corners, and each corner takes the smaller factor of its two edges. Two hand-computed rows with unequal radii were added to its check table: a 100 × 20 rectangle with radii 30, 10, 0, 0 gives 20 and 10; a 100 × 30 rectangle with radii 40, 20, 0, 0 gives 30 and 20.

## Model (EN-010, US-011)

- Figma's `cornerRadius` is `number | figma.mixed`, and "can return figma.mixed if different vertices have different values" (`REF-FIGMA-CR`, re-read): the shape's radius is a view of the vertices' radii, not a value of its own.
- Chosen: `radii`, one requested radius per vertex in contour order, replacing `radius` (ADR-0029). The shape's radius is derived: the common value or Mixed. Changing the number of corners keeps the radii by vertex number, new ones at 0 (Product Owner).

## Points mode (US-012)

- Inkscape's Node tool is a tool of its own, entered from the tool bar or with `F2` / `N`; the path is clicked first, then its nodes show as handles (`REF-INKSCAPE-NODE-TOOL`). The Product Owner chose the same: an explicit mode, large handles, the vertices of the selected shape only.
- First and second vertices marked red and green: the Product Owner's design, to read where the contour starts and which way it goes. No tool read does it. They are vertices 1 and 2 of the contour in the model's order (Q11), not the first points written in the path data: a rounded path starts at the end of the first fillet (`DERIV-fillet-arc` step 7), and a zero-length edge is not written (review of SP-004).
- Coinciding handles (a 0 × 0 or 0 × 50 rectangle): the red handle is drawn last, on top.

## Selection (US-013)

- In a list, Shift + click selects the range from the first item clicked to the last, Command + click adds or removes one item (`REF-APPLE-SELECT`; Ctrl on other systems). The W3C listbox pattern gives the keyboard side: Space toggles, Shift + Space selects the range from the most recently selected option (`REF-WAI-APG-LISTBOX`); it states no mouse modifiers.
- On the drawing, no range (a range has no order on a drawing): Shift + click or Ctrl / Cmd + click adds or removes (Product Owner, `docs/domain/interaction.md` §3).
- The start of a range is the project's choice (the sources do not state it): the last row or handle clicked without Shift, alone or with Ctrl / Cmd. Shift + click replaces the selection by the rows from that start to the clicked row; with no start yet, it acts as a plain click.
- Other choices of the agent, left open by the Product Owner, who asked for every rule to be written down rather than built at once: in Points mode the radius field shows the common radius of the selected vertices or Mixed; with no vertex selected it is disabled. The list writes x and y with at most 5 decimals (Q10) and the requested radius, followed by the effective one when it was reduced (Q8). The selection is cleared when the number of corners changes or when Shape mode is chosen. In the playground, switching between rectangle and polygon keeps the radii by vertex number too, as a change of the number of corners does.

## Open

Nothing blocks the stories. The choice of `radii` (ADR-0029) is the agent's, within the Product Owner's refinement, and is shown to them at VAL-004. The keyboard of the list (Space, Shift + Space) is optional in the W3C pattern and left for the editor applications.
