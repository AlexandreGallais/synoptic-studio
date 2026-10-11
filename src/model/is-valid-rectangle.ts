import { areValidRadii } from "./are-valid-radii";
import { isModelSize } from "./is-model-size";
import { RECTANGLE_CORNER_COUNT } from "./rectangle-corner-count";

import type { Rectangle } from "./rectangle";

/**
 * Whether a rectangle can enter the model: width and height integers ≥ 0, and four corner radii,
 * integers ≥ 0 (ADR-0029).
 *
 * A size of 0 gives a degenerate rectangle, allowed (Q15); a negative or fractional value is
 * rejected (ADR-0003). A radius larger than the rectangle is valid: it is clamped (ADR-0007).
 *
 * @kind domain
 * @param rectangle - width, height and radii to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#rectangle
 */
export function isValidRectangle(rectangle: Rectangle): boolean {
  const { height, radii, width } = rectangle;

  return isModelSize(width) && isModelSize(height) && areValidRadii(radii, RECTANGLE_CORNER_COUNT);
}
