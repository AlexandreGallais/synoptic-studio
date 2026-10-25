import { memorizedShape } from "./memorized-shape";

/**
 * The length of a shape's JSON text, in 16-bit code units (F40): written once for each shape, then
 * read back (`memorizedShape`, ADR-0096), as `historySize` counts each shape the past and the
 * states to come hold.
 *
 * @kind domain
 * @param shape - the shape measured, or what is kept of it, an object never changed once made
 * @returns the length of `JSON.stringify(shape)`
 * @see RULE-interaction-7-navigation-and-history
 */
export function shapeTextLength(shape: Readonly<WeakKey>): number {
  return memorizedShape(shape).length;
}
