/**
 * Whether every item of a list is in another, in the same order (US-077): the shapes a step kept,
 * the same objects, among those it added or before those it deleted.
 *
 * @kind domain
 * @template T - type of the items
 * @param part - the shorter list
 * @param whole - the list it is looked for in
 * @returns true when the items of `whole`, some left out, are those of `part`, by `===`
 * @see RULE-interaction-7-navigation-and-history
 */
export function isKeptInOrder<T>(part: readonly T[], whole: readonly T[]): boolean {
  const found = whole.reduce((count, item) => (part[count] === item ? count + 1 : count), 0);

  return found === part.length;
}
