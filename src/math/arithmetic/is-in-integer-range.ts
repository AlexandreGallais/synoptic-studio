/**
 * Whether a number is one of the integers of an integer interval, both ends included:
 * `first .. last`.
 *
 * Formula: value is an integer and first ≤ value ≤ last.
 *
 * @kind math
 * @param value - the number to check
 * @param first - the interval's first integer
 * @param last - its last integer
 * @returns true when the value is a safe integer from first to last
 * @see REF-WIKI-INTERVAL
 */
export function isInIntegerRange(value: number, first: number, last: number): boolean {
  return Number.isSafeInteger(value) && value >= first && value <= last;
}
