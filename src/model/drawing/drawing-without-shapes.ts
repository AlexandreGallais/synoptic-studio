/**
 * The drawing without the shapes selected (F37), removed by Delete: the others keep their order,
 * each unchanged; an index outside the drawing removes nothing.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param drawing - the shapes in the order drawn, the last on top
 * @param removed - the indices of the shapes removed, in the order drawn
 * @returns a new drawing, the shapes left in the order drawn
 * @see docs/domain/interaction.md#_2-selector
 */
export function drawingWithoutShapes<T>(
  drawing: readonly T[],
  removed: readonly number[],
): readonly T[] {
  return removed.length > 0 ? drawing : [...drawing];
}
