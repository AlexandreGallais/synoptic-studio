import { cyclicItem } from "../../geometry";
import { subtract } from "../../math";
import { positionKeepingPoint } from "../placement";

import { freeShapeCorners } from "./free-shape-corners";

import type { FreeShape } from "./free-shape";
import type { Corner } from "../../geometry";

/** A point the edit did not move: its number before and after the edit. */
export type KeptPoint = {
  /** Its number in the edited shape, from 0. */
  readonly after: number;
  /** Its number before the edit, from 0. */
  readonly before: number;
};

/**
 * Free shape placed so that a point the edit did not move is drawn where it was (ADR-0035,
 * `DERIV-turned-shape` step 5): an edit changes the box of a turned shape, hence its center, and
 * the shape would jump otherwise. At 0° the shape is returned as it is, its box drawn where it is.
 *
 * @kind domain
 * @param shape - free shape after the edit, its box fitted around its straight points
 * @param drawn - corners as drawn before the edit (`turnedCorners`)
 * @param kept - the number of a point not moved, before and after the edit
 * @returns the shape at the position that keeps that point in place
 * @see DERIV-turned-shape
 */
export function freeShapeKeptInPlace(
  shape: FreeShape,
  drawn: readonly Corner[],
  kept: KeptPoint,
): FreeShape {
  if (shape.angle === 0) {
    return shape;
  }

  // Numbers wrap around the contour (Q11); an empty contour falls back on the position.
  const origin = { point: { x: shape.x, y: shape.y }, radius: 0 };
  const { point } = cyclicItem(freeShapeCorners(shape), kept.after, origin);
  const target = cyclicItem(drawn, kept.before, origin).point;

  return {
    ...shape,
    ...positionKeepingPoint(shape, subtract(point, { x: shape.x, y: shape.y }), target),
  };
}
