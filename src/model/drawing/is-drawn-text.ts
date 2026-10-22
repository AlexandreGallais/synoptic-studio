import type { DrawingShape } from "./drawing-shape";
import type { Text } from "../text";

/**
 * Whether a shape of a drawing is a text: the only one with a content (F35).
 *
 * @kind domain
 * @param shape - a shape of a drawing
 * @returns `true` for a text
 * @see docs/domain/interaction.md#_1-tools
 */
export function isDrawnText(shape: DrawingShape): shape is Text {
  return "content" in shape;
}
