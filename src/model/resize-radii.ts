/**
 * Radii of a shape whose number of corners changes: each vertex keeps the radius of its number,
 * the removed vertices take theirs away, and the new vertices are sharp (radius 0) (Product
 * Owner, F03).
 *
 * @kind domain
 * @param radii - requested radius of each vertex, in contour order
 * @param count - new number of vertices, an integer ≥ 0
 * @returns `count` radii
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function resizeRadii(radii: readonly number[], count: number): readonly number[] {
  return Array.from({ length: count }, (unused, index) => radii.at(index) ?? 0);
}
