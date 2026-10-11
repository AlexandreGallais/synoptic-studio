import { boundingBox } from "../geometry";

import type { FreeShape } from "./free-shape";
import type { Corner } from "../geometry";

/**
 * Free shape fitted around corners as drawn: those corners, and the width and height of the box
 * around their points, so that it draws them where they are (ADR-0033).
 *
 * @kind domain
 * @param corners - points as drawn and their radii, in contour order
 * @returns the free shape of those corners, at scale 1
 * @see DERIV-free-shape-stretch
 */
export function freeShapeAroundCorners(corners: readonly Corner[]): FreeShape {
  const box = boundingBox(corners.map((corner) => corner.point));

  return { corners, height: box.maxY - box.minY, width: box.maxX - box.minX };
}
