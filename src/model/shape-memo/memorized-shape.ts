import type { ShapeMemo } from "./shape-memo";

/**
 * The memo of each shape once read, by the shape itself, so that an entry goes with its shape;
 * the entry of `KEYS_GIVEN` holds the next key (ADR-0096, ADR-0097).
 */
const SHAPE_MEMOS = new WeakMap<WeakKey, ShapeMemo>();

/** The entry of `SHAPE_MEMOS` whose key is the next one given, no shape's (ADR-0097). */
const KEYS_GIVEN: WeakKey = {};

/**
 * What is kept of a shape (ADR-0096, ADR-0097): made at its first read, its key the next of the
 * keys given, from 0, its length that of `JSON.stringify(shape)`, then read back from
 * `SHAPE_MEMOS`; two shapes of equal texts, two objects, have two keys, as `historySize` counts
 * them.
 *
 * @kind domain
 * @param shape - the shape, or what is kept of it, an object never changed once made
 * @returns its memo, the same at every call
 * @see ADR-0097
 */
export function memorizedShape(shape: Readonly<WeakKey>): ShapeMemo {
  const known = SHAPE_MEMOS.get(shape);

  if (known !== undefined) {
    return known;
  }

  const key = SHAPE_MEMOS.get(KEYS_GIVEN)?.key ?? 0;
  const memo = { key, length: JSON.stringify(shape).length };

  // lint-disable-next-line functional/immutable-data, functional/no-expression-statements -- the memo of ADR-0096 and ADR-0097, written once for each shape with the next key
  SHAPE_MEMOS.set(KEYS_GIVEN, { key: key + 1, length: 0 }).set(shape, memo);

  return memo;
}
