import { norm, scale, withoutNegativeZero } from "../../math";

import type { HandleMode } from "./handle-mode";
import type { Vector } from "../../math";

/**
 * The other handle of a point once one handle is typed, by Figma's mirroring settings: `none`
 * keeps it; `angle-and-length` is the typed handle reversed; `angle` reverses the typed handle's
 * direction, keeping the other's length — kept as it was when the typed handle has no direction.
 *
 * Formula: none → o; angle and length → −t; angle → −t · |o| / |t|, or o when |t| = 0.
 *
 * @kind geometry
 * @param mode - how the two handles follow each other
 * @param typed - handle typed, a vector from the point
 * @param other - the other handle before the change
 * @returns the other handle after the change
 * @see REF-FIGMA-HANDLE-MIRRORING
 */
export function mirroredHandle(mode: HandleMode, typed: Vector, other: Vector): Vector {
  const length = norm(typed);

  if (mode === "none" || (mode === "angle" && length === 0)) {
    return other;
  }

  // Never −0, so that a path never writes "-0".
  return withoutNegativeZero(scale(typed, mode === "angle" ? -norm(other) / length : -1));
}
