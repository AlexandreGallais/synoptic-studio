/**
 * The greatest of some numbers and of a floor, folded one number at a time, as the maximum of a
 * set is the maximum of the maxima of its parts; a fold rather than `Math.max(...values)`, whose
 * number of arguments engines cap.
 *
 * Formula: max({floor} ∪ values).
 *
 * @kind math
 * @param values - the numbers, any count
 * @param floor - the least result, given back for no number or numbers all under it
 * @returns the greatest of them
 * @see REF-WIKI-MAXIMUM
 */
export function maximum(values: readonly number[], floor: number): number {
  return values.reduce((greatest, value) => Math.max(greatest, value), floor);
}
