/**
 * Edge between two neighbouring points selected: its number is the one of its first point in
 * contour order, the edge closing the contour (from the last point to the first) being the last.
 *
 * @kind domain
 * @param selected - numbers of the points selected, from 0
 * @param count - number of points of the shape
 * @returns the number of the edge, `undefined` unless exactly two neighbouring points are selected
 * @see docs/domain/interaction.md#_3-node-editing
 */
export function edgeOfSelection(selected: readonly number[], count: number): number | undefined {
  // The edge starts at the point whose next one, in contour order, is the other point selected.
  return selected.length === 2
    ? selected.find((point) => selected.includes((point + 1) % count))
    : undefined;
}
