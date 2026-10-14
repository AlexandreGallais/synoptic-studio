import { quarterTurn, scale } from "../../math";

import { sideTangent } from "./side-tangent";

import type { FilletSide } from "./fillet-side";
import type { Vector } from "../../math";

/**
 * The unit normal of a side at a parameter, towards its fillet (`DERIV-fillet-beside-curve`
 * steps 2 and 3, `REF-POMAX-BEZIER`, "Tangents and normals").
 *
 * Formula: n(t) = sign · T(t)⊥.
 *
 * @kind geometry
 * @param side - the side, oriented
 * @param t - the parameter, from 0 to 1
 * @returns the normal at t
 * @see DERIV-fillet-beside-curve
 */
export function sideNormal(side: FilletSide, t: number): Vector {
  return scale(quarterTurn(sideTangent(side.cubic, t)), side.sign);
}
