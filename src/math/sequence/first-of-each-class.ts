/**
 * The index of each key's first occurrence in a list of keys.
 *
 * @kind math
 * @param keys - the keys, in order
 * @returns each distinct key's first index
 * @see REF-WIKI-EQUIVALENCE-CLASS
 */
function firstIndexes(keys: readonly string[]): ReadonlyMap<string, number> {
  // From the last key to the first: the first index of each key is set last.
  return new Map(keys.map((key, index): [string, number] => [key, index]).toReversed());
}

/**
 * The first item of each equivalence class of a list, in order, two items being equivalent when
 * a key function gives them the same key (the kernel of that function): its representative. With
 * the case-folded name as key, "arial" after Arial is left out. Each key looked up among the
 * first index of each: n items cost n lookups, not the n (n − 1) / 2 comparisons of every item
 * with each before it.
 *
 * Formula: keep xᵢ when no j < i has f(xⱼ) = f(xᵢ).
 *
 * @kind math
 * @template T - the items
 * @param items - the list, in order
 * @param key - the function whose equal values make two items equivalent
 * @returns the first item of each class, in the list's order
 * @see REF-WIKI-EQUIVALENCE-CLASS
 */
export function firstOfEachClass<T>(items: readonly T[], key: (item: T) => string): readonly T[] {
  const first = firstIndexes(items.map((item) => key(item)));

  return items.filter((item, index) => first.get(key(item)) === index);
}
