import { shapeMovedOnAxis } from "../panel-values";

import type { Point } from "../../math";
import type { ShapePlacement } from "../placement";

/**
 * A shape or a text moved by a shift: its position, the top-left of its box before it turns,
 * goes by it on each axis (`shapeMovedOnAxis`), its angle, size, points and every setting kept, so
 * that a move never changes the shape itself (ADR-0033, ADR-0035).
 *
 * @kind domain
 * @template T - a shape a tool makes, or a text
 * @param shape - a shape or a text of the model
 * @param shift - how far it moves across and down, in user units
 * @returns it at its new place, nothing else changed
 * @see RULE-interaction-2-selector
 */
export function shapeMovedBy<T extends ShapePlacement>(shape: T, shift: Point): T {
  return shapeMovedOnAxis(shapeMovedOnAxis(shape, "x", shift.x), "y", shift.y);
}
