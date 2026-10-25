import { SELECTION_DRAGS } from "../hover";

import type { ShapesMove } from "../hover";

/**
 * Reads the move a press with the Selector on a shape holds (F39), from the press to the release,
 * whether the pointer has gone further than 3 pixels from the press or not: the keys of the selection and the
 * arrows then change nothing, the drag landing the shapes at the release from the drawing it kept
 * at the press.
 *
 * @kind procedure
 * @param document - page document
 * @returns the move; undefined with no press on a shape
 * @see RULE-interaction-2-selector
 */
export function readPressedMove(document: Document): ShapesMove | undefined {
  return SELECTION_DRAGS.get(document)?.move;
}
