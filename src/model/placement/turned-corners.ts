import { turnAbout } from "../../geometry";
import { turnDirection } from "../../math";

import type { Corner } from "../../geometry";
import type { FreeShape } from "../free-shape";

/** Angle and box of a shape: what turning it needs (ADR-0035). */
export type TurnedBox = Pick<FreeShape, "angle" | "height" | "width" | "x" | "y">;

/**
 * Corners of a shape as drawn: its straight corners turned by its angle about the center of its
 * box, clockwise on screen (`DERIV-turned-shape` step 4). Each corner keeps its radius and its
 * number; at 0° the corners are returned as they are, bit for bit.
 *
 * @kind domain
 * @template C - a corner, with its handles for a free shape (kept as they are until EN-025 turns them)
 * @param corners - straight corners, placed in the SVG (`freeShapeCorners`, `regularPolygonCorners`)
 * @param box - the shape's angle, position, width and height
 * @returns the corners as drawn, in the same order
 * @see DERIV-turned-shape
 */
export function turnedCorners<C extends Corner>(
  corners: readonly C[],
  box: TurnedBox,
): readonly C[] {
  if (box.angle === 0) {
    return corners;
  }

  const center = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
  const direction = turnDirection(box.angle);

  return corners.map((corner) => ({
    ...corner,
    point: turnAbout(corner.point, center, direction),
  }));
}
