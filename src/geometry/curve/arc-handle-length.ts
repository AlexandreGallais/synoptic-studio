import { CUBIC_DEGREE } from "../placement";

/** The 4 of the handle length of an arc, `4/3 · tan(θ/4)` (`REF-POMAX-BEZIER`). */
const FOUR = 4;

/**
 * Length of each handle of a cubic drawing a circular arc of radius 1: the handles leave the arc's
 * ends along its tangents, and the cubic passes through the arc's middle
 * (`DERIV-round-shape-curve` step 1).
 *
 * Formula: k(θ) = 4/3 · tan(θ/4).
 *
 * @kind geometry
 * @param angle - angle of the arc, in radians, from 0 to π
 * @returns the handle length, as a share of the radius
 * @see REF-POMAX-BEZIER
 */
export function arcHandleLength(angle: number): number {
  return (FOUR / CUBIC_DEGREE) * Math.tan(angle / FOUR);
}
