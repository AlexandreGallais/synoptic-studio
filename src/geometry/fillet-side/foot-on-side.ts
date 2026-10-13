import { dot, subtract } from "../../math";
import { cubicVelocity } from "../curve";
import { cubicPoint } from "../placement";

import { crossings } from "./crossings";

import type { FilletSide } from "./fillet-side";
import type { Point } from "../../math";

/**
 * The parameter of the foot of the perpendicular from a point to a side, near the corner: the
 * first place where the point stops lying ahead along the side's tangent, found in a coarse table
 * then by bisection, a curve turning back being allowed (Euclid III.18,
 * `REF-EUCLID-III18`; `REF-POMAX-BEZIER`, "Projecting a point onto a Bézier curve").
 *
 * Formula: the root of (P − B(t)) · B′(t) on [0, end].
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param point - the point projected
 * @param end - the largest parameter looked at
 * @returns the foot's parameter, `end` when the point lies beyond
 * @see DERIV-fillet-beside-curve
 */
export function footOnSide(side: FilletSide, point: Point, end: number): number {
  const [first] = crossings(
    (t) => dot(subtract(point, cubicPoint(side.cubic, t)), cubicVelocity(side.cubic, t)) > 0,
    end,
  );

  return first ?? end;
}
