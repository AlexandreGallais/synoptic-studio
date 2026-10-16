/**
 * Whether a measured value lies within a tolerance of its nominal value, either side, the
 * tolerance being the permissible limit of its variation: a resistor of 100 Ω at ±1 Ω is
 * accepted from 99 to 101 Ω. A width of 1 004 measured for 1 000 is within 9; 1 010 is not.
 *
 * Formula: |m − n| ≤ t.
 *
 * @kind math
 * @param measured - the value measured, m
 * @param nominal - the value expected, n
 * @param tolerance - the largest variation accepted, t ≥ 0
 * @returns true when the variation is within the tolerance
 * @see REF-WIKI-ENGINEERING-TOLERANCE
 */
export function isWithinTolerance(measured: number, nominal: number, tolerance: number): boolean {
  return Math.abs(measured - nominal) <= tolerance;
}
