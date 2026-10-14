import { rectangleAsFreeShape } from "../rectangle";

import type { RoundShape } from "./round-shape";
import type { FreeShape } from "../free-shape";

/**
 * Free shape a round shape becomes by **Convert to rounded rectangle** (F10.AC5): the rectangle
 * tool's 4 corners of its box, each with half its smaller side rounded up as radius, an integer
 * (ADR-0029) — a circle in a square box, a capsule in a long one (`DERIV-round-shape` step 2).
 *
 * @kind domain
 * @param shape - valid round shape of width and height > 0
 * @returns the free shape of its box, rounded
 * @see DERIV-round-shape
 */
export function roundShapeAsRoundedRectangle(shape: RoundShape): FreeShape {
  const radius = Math.ceil(Math.min(shape.width, shape.height) / 2);

  return rectangleAsFreeShape({
    angle: shape.angle,
    height: shape.height,
    radii: [radius, radius, radius, radius],
    width: shape.width,
    x: shape.x,
    y: shape.y,
  });
}
