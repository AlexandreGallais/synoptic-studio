/**
 * The integers of an integer interval, both ends included: `first .. last`.
 *
 * Formula: first, first + 1, …, last; none when last < first.
 *
 * @kind math
 * @param first - the interval's first integer
 * @param last - its last integer
 * @returns its integers, ascending
 * @see REF-WIKI-INTERVAL
 */
export function integerRange(first: number, last: number): readonly number[] {
  return Array.from({ length: Math.max(last - first + 1, 0) }, (unused, index) => first + index);
}
