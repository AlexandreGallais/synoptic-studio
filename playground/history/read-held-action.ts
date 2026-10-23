import { readPressedMove } from "../selection-keys";
import { SHAPE_DRAWINGS } from "../shape-drawing";

/**
 * Reads the action a press holds on the drawing (F40), from the press to the release: shapes
 * moved by the Selector (`readPressedMove`, F39), or a shape drawn by a tool's drag once it left
 * the press's integer point (F36). Meanwhile no step is taken and the keys of the history change
 * nothing, the release finishing the action.
 *
 * @kind procedure
 * @param document - page document
 * @returns `move` or `drawing`; undefined with no such press
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function readHeldAction(document: Document): "drawing" | "move" | undefined {
  return SHAPE_DRAWINGS.has(document) && readPressedMove(document) !== undefined
    ? "move"
    : undefined;
}
