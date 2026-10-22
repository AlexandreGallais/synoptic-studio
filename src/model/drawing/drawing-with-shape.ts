import type { Drawing } from "./drawing";
import type { DrawingShape } from "./drawing-shape";

/**
 * The drawing with one more shape, drawn above the others, which do not change (F35).
 *
 * @kind domain
 * @param drawing - the shapes in the order drawn
 * @param shape - the shape added
 * @returns a new drawing, the shape last
 * @see docs/domain/interaction.md#_1-tools
 */
export function drawingWithShape(drawing: Drawing, shape: DrawingShape): Drawing {
  return [...drawing, shape];
}
