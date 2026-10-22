/**
 * Where the page keeps its drawing (F35), on the canvas (`data-*`): the shapes below the one the
 * panel edits, in the order drawn; that one as last drawn, none while it is refused or none is
 * selected; the shapes above it, in the order drawn, once a shape is picked from the middle (F36).
 */
export const DRAWING_KEYS = { above: "above", below: "drawing", edited: "edited" } as const;
