import { rectangleAsFreeShape } from "./rectangle-as-free-shape";

import type { FreeShape } from "./free-shape";
import type { RoundShape } from "./round-shape";

/**
 * Free shape a round shape becomes by the conversion button (ADR-0036): the free shape the
 * rectangle tool makes for its box — the unit square stretched to its width and height, valid at
 * a size of 0 — with its angle and position, each of its 4 corners rounded by half the smaller
 * side rounded up to an integer, so that it draws the same circle or capsule at the same place
 * (`DERIV-round-shape` step 2).
 *
 * @kind domain
 * @param shape - valid round shape (`isValidRoundShape`)
 * @returns the free shape drawing that round shape
 * @see DERIV-round-shape
 */
export function roundShapeAsFreeShape(shape: RoundShape): FreeShape {
  const radius = Math.ceil(Math.min(shape.width, shape.height) / 2);
  const { angle, height, width, x, y } = shape;

  return rectangleAsFreeShape({
    angle,
    height,
    radii: [radius, radius, radius, radius],
    width,
    x,
    y,
  });
}
