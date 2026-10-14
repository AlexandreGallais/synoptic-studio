import { EPSILON, distance } from "../../math";
import { cubicPoint } from "../placement";

import type { Point } from "../../math";
import type { FilletSide } from "../fillet-side";

/** Samples of a side checked against a fillet's circle. */
const SAMPLES = 64;

/**
 * Whether a fillet's circle stays clear of a side up to its limit: no sample inside the circle,
 * so that the arc never crosses the curve, before or after where it touches it (ADR-0047: the
 * largest circle still tangent without crossing; `REF-ASTUTE-CORNERS`).
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param centre - the circle's centre
 * @param radius - the circle's radius
 * @returns `true` when every sample is at least the radius away
 * @see ADR-0047
 */
export function isClearOfSide(side: FilletSide, centre: Point, radius: number): boolean {
  return Array.from(
    { length: SAMPLES + 1 },
    (unused, index) => (index * side.limit) / SAMPLES,
    // A share EPSILON of the radius forgives rounding errors only.
  ).every((t) => distance(centre, cubicPoint(side.cubic, t)) >= radius * (1 - EPSILON));
}
