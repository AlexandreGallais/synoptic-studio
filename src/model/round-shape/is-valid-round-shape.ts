import { isModelSize } from "../layout-box";
import { isShapePlacement } from "../placement";

import type { RoundShape } from "./round-shape";

/**
 * Whether a round shape can enter the model: width and height integers ≥ 0, an angle from 0 to
 * 359 and a finite position (ADR-0035). A size of 0 draws the line it is, allowed (Q15); a
 * negative or fractional size is rejected (ADR-0003).
 *
 * @kind domain
 * @param shape - round shape to check
 * @returns `true` when every value is allowed
 * @see RULE-shapes-round-shape
 */
export function isValidRoundShape(shape: RoundShape): boolean {
  return isModelSize(shape.width) && isModelSize(shape.height) && isShapePlacement(shape);
}
