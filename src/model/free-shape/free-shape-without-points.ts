import { turnedCorners } from "../placement";

import { freeShapeAroundCorners } from "./free-shape-around-corners";
import { freeShapeCorners } from "./free-shape-corners";
import { freeShapeKeptInPlace } from "./free-shape-kept-in-place";

import type { FreeShape } from "./free-shape";

/**
 * Free shape without some of its points: each goes with its radius, and its two neighbours become
 * consecutive, joined by one side, curved by their outer handles if they have some, the contour
 * staying closed; the others keep their place —
 * on a turned shape too (ADR-0035) — and their order, numbered from the first one left (F08.AC4, note 0007). Fewer than
 * 3 points left give a free shape that is not valid (see `deletionOutcome`).
 *
 * @kind domain
 * @param shape - free shape, straight or turned
 * @param removed - numbers of the points to remove, from 0, in any order
 * @returns the free shape of the points left, its box placed so that the points left stay where they are drawn
 * @see docs/domain/interaction.md#_3-node-editing
 */
export function freeShapeWithoutPoints(shape: FreeShape, removed: readonly number[]): FreeShape {
  const corners = freeShapeCorners(shape);
  const edited = freeShapeAroundCorners(
    corners.filter((corner, index) => !removed.includes(index)),
    shape.angle,
  );
  // The first point left is number 0 after the deletion.
  const first = corners.findIndex((corner, index) => !removed.includes(index));

  return freeShapeKeptInPlace(edited, turnedCorners(corners, shape), { after: 0, before: first });
}
