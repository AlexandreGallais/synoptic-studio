import { isModelSize } from "../placement";

/**
 * Whether the requested corner radii of a shape can enter the model: one per vertex, each an
 * integer ≥ 0 (ADR-0029, ADR-0003).
 *
 * A radius larger than the shape is valid: it is clamped (ADR-0007).
 *
 * @kind domain
 * @param radii - requested radius of each vertex, in contour order
 * @param count - number of vertices of the shape
 * @returns `true` when there is one valid radius per vertex
 * @see docs/domain/shapes.md#_2-corner-radius
 */
export function areValidRadii(radii: readonly number[], count: number): boolean {
  return radii.length === count && radii.every((radius) => isModelSize(radius));
}
