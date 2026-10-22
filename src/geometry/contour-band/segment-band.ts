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
 * @see REF-SVG2-STROKE-SHAPE
 */
export function segmentBand(segment: Segment, halfWidth: number): readonly Segment[] {
  return [segment].slice(0, Math.min(halfWidth, 0));
}
