import { boundingBox } from "../geometry";

import type { FreeShape } from "./free-shape";
import type { Corner } from "../geometry";

/**
 * Free shape fitted around corners as drawn: those corners, and the box around their points as
 * its position, width and height, so that it draws them where they are (ADR-0033, ADR-0035).
 *
 * @kind domain
 * @param corners - points as drawn and their radii, in contour order
 * @param angle - angle kept, from 0 to 359 (ADR-0035)
 * @returns the free shape of those corners, at scale 1
 * @see DERIV-free-shape-stretch
 */
export function freeShapeAroundCorners(corners: readonly Corner[], angle: number): FreeShape {
  const box = boundingBox(corners.map((corner) => corner.point));

  return {
    angle,
    corners,
    height: box.maxY - box.minY,
    width: box.maxX - box.minX,
    x: box.minX,
    y: box.minY,
  };
}
