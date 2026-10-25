import { freeShapeWithoutPoints } from "./free-shape-without-points";
import { isValidFreeShape } from "./is-valid-free-shape";
import { MIN_FREE_SHAPE_POINTS } from "./min-free-shape-points";

import type { FreeShape } from "./free-shape";

/** Whether points may be deleted, and why not: too few points left, or none spanning a box. */
export type DeletionOutcome = "allowed" | "flat" | "too-few";

/**
 * Whether some points of a free shape may be deleted: the points left must be at least 3 and
 * span some width and height, as any free shape (F08.AC4, ADR-0033).
 *
 * @kind domain
 * @param shape - free shape, as drawn at its size
 * @param removed - numbers of the points to delete, from 0; one outside the shape deletes nothing
 * @returns `allowed`, `too-few` (fewer than 3 points left) or `flat` (no width or no height left)
 * @see RULE-interaction-3-node-editing
 */
export function deletionOutcome(shape: FreeShape, removed: readonly number[]): DeletionOutcome {
  const left = freeShapeWithoutPoints(shape, removed);

  if (left.corners.length < MIN_FREE_SHAPE_POINTS) {
    return "too-few";
  }

  return isValidFreeShape(left) ? "allowed" : "flat";
}
