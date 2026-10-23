/**
 * A key's digit in base 32 at a level of the held counts (EN-147), the level 0 the lowest: 33 is 1
 * at the level 0 and 1 at the level 1, 1 × 32 + 1 (ADR-0097).
 *
 * @kind domain
 * @param key - a shape's key (`shapeKey`), an integer ≥ 0
 * @param level - the level read, 0 for the last, the counts' own
 * @returns the digit, from 0 to 31
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function keyDigit(key: number, level: number): number {
  return key + level;
}
