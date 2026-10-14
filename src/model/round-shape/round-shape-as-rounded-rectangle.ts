import { halfSmallerSide } from "../../geometry";
import { rectangleAsFreeShape } from "../rectangle";

import type { RoundShape } from "./round-shape";
import type { FreeShape } from "../free-shape";

/**
 * Free shape a round shape becomes by **Convert to rounded rectangle** (F10.AC5): the rectangle
 * tool's 4 corners of its box, each with half its smaller side rounded up as radius, an integer
 * (ADR-0029) — a circle in a square box, a capsule in a long one (`DERIV-round-shape` step 2).
 *
 * @kind domain
 * @param shape - valid round shape of width and height > 0: one of size 0 is not converted (F08),
 * the caller greys its button, as for a polygon
 * @returns the free shape of its box, rounded
 * @see DERIV-round-shape
 */
export function roundShapeAsRoundedRectangle(shape: RoundShape): FreeShape {
  // An integer radius (ADR-0029): half the smaller side rounded up (DERIV-round-shape step 2).
  const radius = Math.ceil(halfSmallerSide(shape));

  return rectangleAsFreeShape({
    angle: shape.angle,
    height: shape.height,
    radii: [radius, radius, radius, radius],
    width: shape.width,
    x: shape.x,
    y: shape.y,
  });
}
