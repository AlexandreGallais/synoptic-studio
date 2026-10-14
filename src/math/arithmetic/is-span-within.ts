/**
 * Tells whether a span of integers lies within a range starting at 0: the integer interval
 * `start .. start + size − 1` inside `0 .. end − 1`. An empty span lies within any range as long as
 * its start is not past the end.
 *
 * Formula: 0 ≤ start and start + size ≤ end.
 *
 * @kind math
 * @param start - the span's first integer
 * @param size - how many integers it holds, ≥ 0
 * @param end - the range's end, excluded
 * @returns true when every integer of the span lies in the range
 * @see REF-WIKI-INTERVAL
 */
export function isSpanWithin(start: number, size: number, end: number): boolean {
  return start >= 0 && start + size <= end;
}
