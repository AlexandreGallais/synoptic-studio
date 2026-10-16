/**
 * The ceiling of a number: "the smallest (closest to -∞) integral Number value that is not less
 * than x", `Math.ceil` (REF-ECMA-262-MATH-CEIL); the −0 it gives from −1 < x < 0 is turned to 0
 * by adding +0 (REF-ECMA-262-NUMBER-ADD).
 *
 * Formula: ⌈x⌉ = min{n ∈ ℤ : n ≥ x}.
 *
 * @kind math
 * @param value - a finite number
 * @returns the smallest integer not less than it, never −0
 * @see REF-ECMA-262-MATH-CEIL
 */
export function ceiling(value: number): number {
  return Math.ceil(value) + 0;
}
