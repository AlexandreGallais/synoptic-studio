import { DRAWING_KEYS, readStoredDrawing } from "../kept-shapes";
import { readShapeKind } from "../page";

/** The kinds of the panel that edit no shape: none selected, or several. */
const NO_SHAPE_EDITED = new Set(["none", "several"]);

/**
 * Reads the kind of the shape the panel edits when it is refused: a value out of its range,
 * a text not drawn, so that the page keeps no shape edited while the panel still shows one; no
 * step is taken meanwhile, the drawing kept without it being no change the designer made.
 *
 * @kind procedure
 * @param document - page document, just redrawn
 * @returns `rectangle`, `polygon`, `round`, `free` or `text`; undefined for a shape drawn, or
 * none or several selected
 * @see RULE-interaction-7-navigation-and-history
 */
export function readRefusedKind(document: Document): string | undefined {
  const kind = readShapeKind(document);
  const isEdited = readStoredDrawing(document, DRAWING_KEYS.edited).length > 0;

  return [isEdited, NO_SHAPE_EDITED.has(kind)].includes(true) ? undefined : kind;
}
