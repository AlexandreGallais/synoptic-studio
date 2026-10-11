# 0006 — Sources of F03: per-vertex corner radius

Spike SP-004, 2026-10-11. Claude Code's own research; every source below was read.

## Clamping with unequal radii (EN-010)

Nothing new to derive: `DERIV-local-radius-clamp` already works per vertex. Each edge compares its length with the sum of the setbacks of its two corners, and each corner takes the smaller factor of its two edges. Two hand-computed rows with unequal radii were added to its check table: a 100 × 20 rectangle with radii 30, 10, 0, 0 gives 20 and 10; a 100 × 30 rectangle with radii 40, 20, 0, 0 gives 30 and 20.

## Model (EN-010, US-011)

- Figma's `cornerRadius` is `number | figma.mixed`, and « can return figma.mixed if different vertices have different values » (`REF-FIGMA-CR`, re-read): the shape's radius is a view of the vertices' radii, not a value of its own.
- Chosen: `radii`, one requested radius per vertex in contour order, replacing `radius` (ADR-0029). The shape's radius is derived: the common value or Mixed. Changing the number of corners keeps the radii by vertex number, new ones at 0 (Product Owner).

## Points mode (US-012)

- Inkscape's Node tool is a tool of its own, entered from the tool bar or with `F2` / `N`; the path is clicked first, then its nodes show as handles (`REF-INKSCAPE-NODE-TOOL`). The Product Owner chose the same: an explicit mode, large handles, the vertices of the selected shape only.
- First and second vertices marked red and green: the Product Owner's design, to read where the contour starts and which way it goes. No tool read does it; the first vertex is the `M` point of the path data (`REF-SVG2-PATHS` §9.3.3).

## Selection (US-013)

- In a list, Shift + click selects the range from the first item clicked to the last, Command + click adds or removes one item (`REF-APPLE-SELECT`; Ctrl on other systems). The W3C listbox pattern gives the keyboard side: Space toggles, Shift + Space selects the range from the most recently selected option (`REF-WAI-APG-LISTBOX`); it states no mouse modifiers.
- On the drawing, no range (a range has no order on a drawing): Shift + click or Ctrl / Cmd + click adds or removes (Product Owner, `docs/domain/interaction.md` §3).
- The range starts from the last row clicked without Shift (the anchor of the range), as in the sources above.

## Open

Nothing blocks the stories. The keyboard of the list (Space, Shift + Space) is optional in the W3C pattern and left for the editor applications.
