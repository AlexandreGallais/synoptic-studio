/**
 * The integer nearest a number, a tie rounded up, toward +∞: CSS rounds to the nearest integer "in
 * the direction of +∞ when the fractional portion is exactly 0.5" (REF-CSS-VALUES-4, §5.2), which
 * is `Math.round` (REF-MDN-MATH-ROUND). Unlike `⌊x + 0.5⌋`, it does not round 0.49999999999999994
 * up; the −0 it gives from −0.5 to 0 is turned to 0 by adding +0 (REF-ECMA-262-NUMBER-ADD). A
 * quotient of integers `n / d` is rounded exactly: a half-integer quotient is computed exactly,
 * any other lies at least `1 / (2d)` from a half-integer (`DERIV-color-spaces`, step 2).
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
