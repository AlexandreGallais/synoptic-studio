/**
 * Where the page keeps its drawing (F35), on the canvas (`data-*`): the shapes placed below the
 * one the panel edits, in the order drawn, and that one as last drawn, none while it is refused.
 */
export const DRAWING_KEYS = { below: "drawing", edited: "edited" } as const;
