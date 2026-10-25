/**
 * Whether two lists hold the same items in the same order, each the same value or the same object
 * (F40): a drawing shared with the present unchanged, or a selection unchanged (ADR-0095,
 * decision 3).
 *
 * @kind domain
 * @template T - type of the items
 * @param first - a list
 * @param second - another list
 * @returns true when both have the same length and each item is the other's, by `===`
 * @see RULE-interaction-7-navigation-and-history
 */
export function areSameItems<T>(first: readonly T[], second: readonly T[]): boolean {
  return first.length === second.length && first.every((item, index) => item === second[index]);
}
