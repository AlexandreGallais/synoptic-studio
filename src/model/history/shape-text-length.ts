/**
 * The length of each shape's JSON text once measured, by the shape itself, a shape never being
 * changed once made (ADR-0014): the history's size reads it again at each step without writing
 * the text again (ADR-0096).
 */
const TEXT_LENGTHS = new WeakMap<WeakKey, number>();

/**
 * The length of a shape's JSON text, in 16-bit code units (F40): written once for each shape, then
 * read from `TEXT_LENGTHS` (ADR-0096), as `historySize` counts each shape the past and the states
 * to come hold.
 *
 * @kind domain
 * @param shape - the shape measured, or what is kept of it, an object never changed once made
 * @returns the length of `JSON.stringify(shape)`
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function shapeTextLength(shape: Readonly<WeakKey>): number {
  const known = TEXT_LENGTHS.get(shape);

  if (known !== undefined) {
    return known;
  }

  const { length } = JSON.stringify(shape);

  // lint-disable-next-line functional/immutable-data, functional/no-expression-statements -- the memo of ADR-0096, written once for each shape
  TEXT_LENGTHS.set(shape, length);

  return length;
}
