import { isModelSize } from "./is-model-size";
import { isShapePlacement } from "./is-shape-placement";

import type { RoundShape } from "./round-shape";

/**
 * Whether a round shape can enter the model: width and height integers ≥ 0 (Q15, 0 allowed: a
 * line), an angle from 0 to 359 and a finite position (ADR-0035, ADR-0036).
 *
 * @kind domain
 * @param shape - width, height, angle and position to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#round-shape
 */
export function isValidRoundShape(shape: RoundShape): boolean {
  return isModelSize(shape.width) && isModelSize(shape.height) && isShapePlacement(shape);
}
