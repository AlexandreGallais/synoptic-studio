import type { ShapeFields } from "./shape-fields";

/**
 * The fields a step changed in a drawing (US-077): for each index both drawings hold, the shape
 * replaced by another object, the fields they differ by (`changedFields`), each named once.
 *
 * @kind domain
 * @template T - type of the shapes, or of what is kept of each
 * @param before - the drawing before the step
 * @param after - the drawing after it
 * @param fields - reads the fields of a shape
 * @returns the names of the fields changed, sorted; none when no shape is replaced
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function drawingChangedFields<T>(
  before: readonly T[],
  after: readonly T[],
  fields: ShapeFields<T>,
): readonly string[] {
  return [...before, ...after].map((shape) => Object.keys(fields(shape)).join(","));
}
