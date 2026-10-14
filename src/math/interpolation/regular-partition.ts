/**
 * The points dividing an interval from 0 into equal parts: a regular partition. Each point is
 * computed on its own, (i · b) / n, the product first, so that no rounding builds up from one to
 * the next.
 *
 * Formula: xᵢ = (i · b) / n, i = 0 … n.
 *
 * @kind math
 * @param end - the interval's end b; it starts at 0
 * @param count - the number of parts n, ≥ 1
 * @returns the n + 1 points, from 0 to b
 * @see REF-OPENSTAX-REGULAR-PARTITION
 */
export function regularPartition(end: number, count: number): readonly number[] {
  return Array.from({ length: count + 1 }, (unused, index) => (index * end) / count);
}
