import type { Shape } from "./shape";
import type { RoundShape } from "../round-shape";

/**
 * Whether a shape is a round shape (F10): the only shape with a kind.
 *
 * @kind domain
 * @param shape - a shape of the model
 * @returns `true` for a round shape
 * @see docs/domain/shapes.md#round-shape
 */
export function isRoundShape(shape: Shape): shape is RoundShape {
  return "kind" in shape;
}
