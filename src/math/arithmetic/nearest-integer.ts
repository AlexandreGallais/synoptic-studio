/**
 * The integer nearest a number, a tie rounded up, toward +∞: CSS's rule for a byte, "rounded to
 * the closest integer (rounding up if two values are equally close)" (REF-CSS-COLOR-4, §16.1.1),
 * which is `Math.round` (REF-MDN-MATH-ROUND). Unlike `⌊x + 0.5⌋`, it does not round
 * 0.49999999999999994 up; the −0 it gives from −0.5 to 0 is turned to 0 by adding +0
 * (REF-ECMA-262-NUMBER-ADD). `roundedQuotient` is exact for integers only (`DERIV-color-spaces`,
 * step 8).
 *
 * Formula: round(x) = the integer nearest x, x + ½ when x is halfway.
 *
 * @kind math
 * @param value - a finite number
 * @returns the nearest integer, never −0
 * @see REF-MDN-MATH-ROUND
 */
export function nearestInteger(value: number): number {
  return Math.round(value) + 0;
}
