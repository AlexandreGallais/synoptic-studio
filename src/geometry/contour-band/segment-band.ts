import { add, isZeroVector, opposite, quarterTurn, scale, subtract, unit } from "../../math";
import { polygonSegments } from "../painted-area";

import type { Segment } from "../contour";

/**
 * The band about a segment: the points on the lines perpendicular to it, within a half width of
 * it, a rectangle as long as the segment and twice the half width wide, its four sides turning in
 * the positive direction of angles, clockwise on screen, so that the nonzero rule fills the bands
 * of a contour as one (`REF-SVG2-FILL-RULE`).
 *
 * Formula: n = h · unit(d)⊥, d = B − A; corners A − n, B − n, B + n, A + n.
 *
 * @kind geometry
 * @param segment - its start A and its end B
 * @param halfWidth - the half width h of the band, in user units, > 0
 * @returns the rectangle's four sides, from A − n; none for a segment of length 0
 * @see REF-SVG2-PAINT
 */
export function segmentBand(segment: Segment, halfWidth: number): readonly Segment[] {
  const direction = subtract(segment.end, segment.start);

  if (isZeroVector(direction)) {
    return [];
  }

  const across = scale(unit(quarterTurn(direction)), halfWidth);
  const back = opposite(across);

  return polygonSegments([
    add(segment.start, back),
    add(segment.end, back),
    add(segment.end, across),
    add(segment.start, across),
  ]);
}
