import { ZERO_VECTOR, centralAngle, directionAt, hadamardProduct, polarAngle } from "../../math";

import { chainEnd } from "./chain-end";
import { HALF_SIDE } from "./half-side";

/**
 * Directions of the sides of a stretched polygon's right half, after its half base: the
 * directions `θⱼ = 2πj/n` of the regular polygon scaled vertically by `λ`,
 * `φⱼ = atan2(λ sin θⱼ, cos θⱼ)`, in `(0, π)` since `sin θⱼ > 0`. For an odd `n`, the last side
 * closes the half on the axis, `φₘ = acos(−1/2 − Σⱼ₍ₘ cos φⱼ)`, which is `NaN` when no side can
 * (`DERIV-stretched-polygon` steps 1–2). At `λ = 1` they are the regular directions.
 *
 * @kind geometry
 * @param corners - number of corners `n`, an integer from 3 to 12
 * @param scale - vertical scale `λ > 0`
 * @returns the `⌈n/2⌉ − 1` directions, in radians, from the base upwards
 * @see DERIV-stretched-polygon
 */
export function stretchedSideDirections(corners: number, scale: number): readonly number[] {
  const directions = Array.from({ length: Math.ceil(corners / 2) - 1 }, (empty, index) => {
    const regular = directionAt(centralAngle(index + 1, corners));

    // The regular side's direction stretched vertically by the scale.
    return polarAngle(hadamardProduct(regular, { x: 1, y: scale }));
  });

  if (corners % 2 === 0) {
    return directions;
  }

  const before = directions.slice(0, -1);
  const cosines = chainEnd(ZERO_VECTOR, before).x;

  return [...before, Math.acos(-HALF_SIDE - cosines)];
}
