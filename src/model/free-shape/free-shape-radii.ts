import type { FreeShape } from "./free-shape";

/**
 * Requested radius of each corner of a free shape, in contour order (ADR-0033).
 *
 * @kind domain
 * @param shape - corners whose radii are read
 * @returns one radius per corner
 * @see RULE-shapes-2-corner-radius
 */
export function freeShapeRadii(shape: FreeShape): readonly number[] {
  return shape.corners.map((corner) => corner.radius);
}
