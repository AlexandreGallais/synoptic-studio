import { isShapeAngle } from "./is-shape-angle";

import type { FreeShape } from "../free-shape";

/** Angle and position of a shape (ADR-0035). */
export type ShapePlacement = Pick<FreeShape, "angle" | "x" | "y">;

/**
 * Whether a shape's angle and position can enter the model: an angle from 0 to 359, and a finite
 * position, fractional when an edit of a turned shape leaves it so (ADR-0035).
 *
 * @kind domain
 * @param placement - the angle and the top-left of the box before turning
 * @returns `true` when both are allowed
 * @see RULE-shapes-rotation-of-shapes
 */
export function isShapePlacement(placement: ShapePlacement): boolean {
  return (
    isShapeAngle(placement.angle) &&
    [placement.x, placement.y].every((value) => Number.isFinite(value))
  );
}
