import { boundingBox } from "../bounding-box";

import { mirrorAcrossAxis } from "./mirror-across-axis";
import { stretchedRightHalf } from "./stretched-right-half";

import type { Point } from "../../math";

/**
 * Stretched polygon of sides 1 for a vertical scale `λ`, in the SVG frame (y down) with the
 * top-left of its box at the origin, in drawing order: clockwise on screen from the right end of
 * its base (Q18). The right half and its mirror image about the axis, the apex once for an odd `n`
 * (`DERIV-stretched-polygon` step 6).
 *
 * @kind geometry
 * @param corners - number of corners `n`, an integer from 3 to 12
 * @param scale - vertical scale `λ > 0`
 * @returns the `n` vertices in drawing order
 * @see DERIV-stretched-polygon
 */
export function unitStretchedPolygon(corners: number, scale: number): readonly Point[] {
  const half = stretchedRightHalf(corners, scale);
  const base = half.slice(0, 1);
  const inner = half.slice(1, -1);
  const top = half.slice(-1);
  const ordered = [
    ...base,
    ...base.map((point) => mirrorAcrossAxis(point)),
    ...inner.map((point) => mirrorAcrossAxis(point)),
    ...(corners % 2 === 0 ? top.map((point) => mirrorAcrossAxis(point)) : []),
    ...top,
    ...inner.toReversed(),
  ];
  const box = boundingBox(ordered);

  return ordered.map((point) => ({
    x: point.x - box.minX,
    y: box.maxY - point.y,
  }));
}
