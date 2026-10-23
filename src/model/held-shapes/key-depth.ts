/**
 * The levels the held counts need to hold a key (EN-147): its digits in base 32, 1 for 0 to 31, 2
 * for 32 to 1 023 (ADR-0097).
 *
 * @kind domain
 * @param key - a shape's key (`shapeKey`), an integer ≥ 0
 * @returns the count of its digits in base 32, 1 or more
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function keyDepth(key: number): number {
  return key;
}
