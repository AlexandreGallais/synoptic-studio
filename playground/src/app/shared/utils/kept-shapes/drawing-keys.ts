/**
 * Where the page keeps its drawing, on the canvas (`data-*`): the shapes below the one the
 * panel edits, in the order drawn; that one as last drawn, none while it is refused or none is
 * selected; the shapes above it, in the order drawn, once a shape is picked from the middle;
 * the indices of the shapes selected while several are, every shape then kept below.
 */
export const DRAWING_KEYS = {
  above: "above",
  below: "drawing",
  edited: "edited",
  selected: "selected",
} as const;
