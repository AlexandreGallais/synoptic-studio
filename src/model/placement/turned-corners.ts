import { turnAbout, turnedHandles } from "../../geometry";
import { turnDirection } from "../../math";

import type { CurveCorner } from "../../geometry";
import type { FreeShape } from "../free-shape";

/** Angle and box of a shape: what turning it needs (ADR-0035). */
export type TurnedBox = Pick<FreeShape, "angle" | "height" | "width" | "x" | "y">;

/**
 * Corners of a shape as drawn: its straight corners turned by its angle about the center of its
 * box, clockwise on screen (`DERIV-turned-shape` step 4). Each corner keeps its radius and its
 * number, and its handles turn with it (`DERIV-cubic-sides` step 5); at 0° the corners are returned as
 * they are, bit for bit.
 *
 * @kind domain
 * @param corners - straight corners, placed in the SVG (`freeShapeCorners`, `regularPolygonCorners`)
 * @param box - the shape's angle, position, width and height
 * @returns the corners as drawn, in the same order
 * @see DERIV-turned-shape
 */
export function turnedCorners(
  corners: readonly CurveCorner[],
  box: TurnedBox,
): readonly CurveCorner[] {
  if (box.angle === 0) {
    return corners;
  }

  const center = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
  const direction = turnDirection(box.angle);

  return corners.map((corner) => ({
    ...corner,
    handles: turnedHandles(corner.handles, direction),
    point: turnAbout(corner.point, center, direction),
  }));
}
