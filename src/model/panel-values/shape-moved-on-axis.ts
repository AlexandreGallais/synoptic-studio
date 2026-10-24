import type { ShapePlacement } from "../placement";

/**
 * A shape or a text moved along one axis: its position, the top-left of its box before it turns,
 * goes by the shift, its angle and size kept (ADR-0035).
 *
 * @kind domain
 * @template T - a shape a tool makes, or a text
 * @param shape - a shape or a text of the model
 * @param axis - `x` across, `y` down
 * @param shift - how far it moves, in user units
 * @returns it at its new place, nothing else changed
 * @see docs/domain/interaction.md#_2-selector
 */
export function shapeMovedOnAxis<T extends ShapePlacement>(
  shape: T,
  axis: "x" | "y",
  shift: number,
): T {
  return axis === "x" ? { ...shape, x: shape.x + shift } : { ...shape, y: shape.y + shift };
}
