/**
 * The least of some numbers and of a ceiling, folded one number at a time, as the minimum of a set
 * is the minimum of the minima of its parts; a fold rather than `Math.min(...values)`, whose
 * number of arguments engines cap.
 *
 * Formula: min({ceiling} ∪ values).
 *
 * @kind math
 * @param values - the numbers, any count
 * @param ceiling - the greatest result, given back for no number or numbers all over it
 * @returns the least of them
 * @see REF-WIKI-MAXIMUM
 */
export function minimum(values: readonly number[], ceiling: number): number {
  return values.reduce((least, value) => Math.min(least, value), ceiling);
}
