/**
 * Radii after a radius is typed in Points mode: the selected vertices get it, the others keep
 * theirs (F03.AC5).
 *
 * @kind domain
 * @param radii - requested radius of each vertex, in contour order
 * @param selected - numbers of the selected vertices, from 0
 * @param radius - radius typed
 * @returns the new radii, as many as before
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function radiiWithSelection(
  radii: readonly number[],
  selected: readonly number[],
  radius: number,
): readonly number[] {
  return radii.map((kept, index) => (selected.includes(index) ? radius : kept));
}
