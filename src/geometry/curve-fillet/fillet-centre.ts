import { add, scale } from "../../math";
import { sideNormal } from "../fillet-side";
import { cubicPoint } from "../placement";

import type { Point } from "../../math";
import type { FilletSide } from "../fillet-side";

/**
 * The centre of a fillet touching a side at a parameter: on the side's parallel curve
 * (`DERIV-fillet-beside-curve` step 3, `REF-WIKI-PARALLEL-CURVE`).
 *
 * Formula: D = B(t) + r n(t).
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param radius - its distance from the side
 * @param t - where it touches the side
 * @returns the point at that distance along the side's normal
 * @see DERIV-fillet-beside-curve
 */
export function filletCentre(side: FilletSide, radius: number, t: number): Point {
  return add(cubicPoint(side.cubic, t), scale(sideNormal(side, t), radius));
}
