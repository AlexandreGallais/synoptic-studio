import { memorizedShape } from "../shape-memo";

/**
 * The key of a shape in the held counts: given once for each shape, the next of the keys
 * given, from 0, then read back (`memorizedShape`), so that the held counts find a shape by an
 * integer, ECMAScript giving an object no hash; two shapes of equal texts, two objects, have two
 * keys, as `historySize` counts them (ADR-0097).
 *
 * @kind domain
 * @param shape - the shape, or what is kept of it, an object never changed once made
 * @returns its key, an integer ≥ 0, the same at every call
 * @see ADR-0097
 */
export function shapeKey(shape: Readonly<WeakKey>): number {
  return memorizedShape(shape).key;
}
