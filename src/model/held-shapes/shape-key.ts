/**
 * The key of each shape once given, by the shape itself, so that the held counts find a shape by
 * an integer, ECMAScript giving an object no hash (ADR-0097).
 */
const SHAPE_KEYS = new WeakMap<WeakKey, number>();

/** How many keys were given: the next key (ADR-0097). */
const GIVEN_KEYS = { count: 0 };

/**
 * The key of a shape in the held counts (EN-147): given once for each shape, the next of the keys
 * given (`GIVEN_KEYS`), from 0, then read from `SHAPE_KEYS`; two shapes of equal texts, two
 * objects, have two keys, as `historySize` counts them (ADR-0097).
 *
 * @kind domain
 * @param shape - the shape, or what is kept of it, an object never changed once made
 * @returns its key, an integer ≥ 0, the same at every call
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function shapeKey(shape: Readonly<WeakKey>): number {
  const known = SHAPE_KEYS.get(shape);

  if (known !== undefined) {
    return known;
  }

  const key = GIVEN_KEYS.count;

  // lint-disable-next-line functional/immutable-data, functional/no-expression-statements -- the count of the keys of ADR-0097, one more for each shape given one
  GIVEN_KEYS.count = key + 1;
  // lint-disable-next-line functional/immutable-data, functional/no-expression-statements -- the memo of ADR-0097, written once for each shape
  SHAPE_KEYS.set(shape, key);

  return key;
}
