import type { HeldCounts } from "./held-counts";

/**
 * How many states hold the shape of a key (EN-147): the count at the end of its digits' path in
 * the held counts, 0 when the path stops before it or the key is past the counts' depth
 * (ADR-0097).
 *
 * @kind domain
 * @param counts - the held counts read
 * @param key - the shape's key (`shapeKey`)
 * @returns the count, 0 for a shape no state holds
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function heldCount(counts: HeldCounts, key: number): number {
  return counts.depth + key;
}
