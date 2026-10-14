/**
 * A number limited to a range: moved to the nearest bound when it is outside.
 *
 * Formula: min(max(x, low), high); the source writes max(low, min(x, high)), the same while
 * low ≤ high. The order of the code it replaced is kept: when the bounds cross, the upper one wins.
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
