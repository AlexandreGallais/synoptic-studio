import { EPSILON, dot, subtract } from "../../math";
import { cubicVelocity } from "../curve";
import { cubicPoint } from "../placement";

import { distanceFromSide } from "./distance-from-side";
import { filletTangency } from "./fillet-tangency";
import { isClearOfSide } from "./is-clear-of-side";

import type { FilletSide } from "./fillet-side";
import type { Point } from "../../math";

/**
 * Whether a point's foot on a side falls before the side's limit: the point is not ahead of the
 * limit along the tangent there (`REF-MATHWORLD-DOT`).
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param point - the fillet's centre
 * @returns `true` when the foot is within the limit
 * @see DERIV-fillet-beside-curve
 */
function isWithinLimit(side: FilletSide, point: Point): boolean {
  const end = cubicPoint(side.cubic, side.limit);

  return dot(subtract(point, end), cubicVelocity(side.cubic, side.limit)) <= 0;
}

/**
 * Whether a fillet of a radius fits a corner: a centre at the radius from both sides, its touch
 * points within each side's limit, its circle clear of both sides before them
 * (`DERIV-fillet-beside-curve` steps 7 and 11, ADR-0047).
 *
 * @kind geometry
 * @param sides - the side before, then the side after, oriented
 * @param radius - the radius tried
 * @returns `true` when the arc can be drawn at that radius
 * @see DERIV-fillet-beside-curve
 */
export function isFilletFitting(sides: readonly [FilletSide, FilletSide], radius: number): boolean {
  const [before, after] = sides;
  const tangency = filletTangency(sides, radius);

  if (tangency === undefined) {
    return false;
  }

  const { centre } = tangency;
  const gap = distanceFromSide(before, centre, before.limit) - radius;

  return (
    isWithinLimit(before, centre) &&
    Math.abs(gap) <= EPSILON * Math.max(1, radius) &&
    isClearOfSide(before, centre, radius) &&
    isClearOfSide(after, centre, radius)
  );
}
