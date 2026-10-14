import type { Vector } from "./vector";

/**
 * A vector whose components −0 become 0: equal under IEEE 754, they differ for `Object.is` and in
 * path data ("-0"); adding +0 turns −0 into +0 and leaves every other number as it is.
 *
 * Formula: (vₓ + 0, v_y + 0).
 *
 * @kind math
 * @param vector - any vector
 * @returns the same vector, without −0
 * @see REF-ECMA-262-NUMBER-ADD
 */
export function withoutNegativeZero(vector: Vector): Vector {
  return { x: vector.x + 0, y: vector.y + 0 };
}
