import type { Shape } from "./shape";
import type { FreeShape } from "../free-shape";

/**
 * Whether a shape is a free shape, whose points can be typed, added and deleted (F08): not a
 * regular polygon, whose number of corners is a count, nor a round shape, which has none; both
 * are converted first.
 *
 * @kind domain
 * @param shape - a shape of the model
 * @returns `true` for a free shape
 * @see docs/domain/shapes.md#round-shape-f10
 */
export function isFreeShape(shape: Shape): shape is FreeShape {
  return "corners" in shape && typeof shape.corners !== "number";
}
