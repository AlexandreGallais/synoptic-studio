import { readPressedMove } from "../selection-keys";
import { SHAPE_DRAWINGS } from "../shape-drawing";

/**
 * Reads the action a press holds on the drawing, from the press to the release: shapes
 * moved by the Selector (`readPressedMove`), or a shape drawn by a tool's drag once it went
 * further than 3 pixels from the press. Meanwhile no step is taken and the keys of the history change
 * nothing, the release finishing the action.
 *
 * @kind procedure
 * @param document - page document
 * @returns `move` or `drawing`; undefined with no such press
 * @see RULE-interaction-7-navigation-and-history
 */
export function readHeldAction(document: Document): "drawing" | "move" | undefined {
  if (readPressedMove(document) !== undefined) {
    return "move";
  }

  return SHAPE_DRAWINGS.get(document)?.isDrawn === true ? "drawing" : undefined;
}
