import { rectangleCorners } from "./rectangle-corners";

import type { RoundShape } from "./round-shape";
import type { Corner } from "../geometry";

/**
 * Straight corners of a round shape, placed at its position but not turned (`turnedCorners`):
 * the four corners of its box, top-left first, clockwise (Q11), each rounded by half the smaller
 * side, so that it draws a circle when width and height are equal and a capsule otherwise
 * (`DERIV-round-shape` step 1). The radius is derived, fractional for an odd side, never stored.
 *
 * @kind domain
 * @param shape - valid round shape (`isValidRoundShape`)
 * @returns its four corners
 * @see DERIV-round-shape
 */
export function roundShapeCorners(shape: RoundShape): readonly Corner[] {
  const radius = Math.min(shape.width, shape.height) / 2;

  return rectangleCorners({ ...shape, radii: [radius, radius, radius, radius] });
}
