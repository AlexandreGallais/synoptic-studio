import { ellipseCorners } from "../../geometry";
import { add } from "../../math";

import type { RoundShape } from "./round-shape";
import type { CurveCorner } from "../../geometry";

/**
 * Corners of a round shape, placed at its position: its top, right, bottom and left points with
 * their handles (`DERIV-round-shape-curve`); its angle is not applied (`turnedCorners`, ADR-0035).
 *
 * @kind domain
 * @param shape - valid round shape (`isValidRoundShape`)
 * @returns its 4 corners, clockwise from the top
 * @see RULE-shapes-round-shape
 */
export function roundShapeCorners(shape: RoundShape): readonly CurveCorner[] {
  return ellipseCorners(shape).map((corner) => ({
    ...corner,
    point: add(corner.point, { x: shape.x, y: shape.y }),
  }));
}
