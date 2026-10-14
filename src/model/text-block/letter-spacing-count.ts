/**
 * How many times letter spacing is added in a line: once between each pair of adjacent
 * characters, never at the line's start or end (`DERIV-text-width`, step 3).
 *
 * Formula: max(n − 1, 0).
 *
 * @kind domain
 * @param characterCount - the number of characters of the line, n ≥ 0
 * @returns the number of pairs of adjacent characters
 * @see REF-CSS-TEXT-3
 */
export function letterSpacingCount(characterCount: number): number {
  return Math.max(characterCount - 1, 0);
}
