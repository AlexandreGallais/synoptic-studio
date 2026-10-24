/**
 * Whether a step moved shapes in the drawing's order (US-077): a shape after it the same object as
 * a shape before it at another index, so that comparing the shapes index by index compares two
 * shapes.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param before - the drawing before the step
 * @param after - the drawing after it
 * @returns true when a shape of `after` is one of `before` at another index, by `===`
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function isReordered<T>(before: readonly T[], after: readonly T[]): boolean {
  const shapes = new Set(before);

  return after.some((shape, index) => shape !== before[index] && shapes.has(shape));
}
