/**
 * A number limited to a range: moved to the nearest bound when it is outside. When the bounds
 * cross, the upper one wins.
 *
 * Formula: min(max(x, low), high).
 *
 * @kind math
 * @param value - the number
 * @param low - the lower bound
 * @param high - the upper bound
 * @returns the number in the range
 * @see REF-WIKI-CLAMP
 */
export function clamp(value: number, low: number, high: number): number {
  return Math.min(Math.max(value, low), high);
}
