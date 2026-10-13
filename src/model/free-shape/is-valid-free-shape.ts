import { isShapePlacement } from "../placement";
import { areValidRadii } from "../radii";

import { areFreeShapePoints } from "./are-free-shape-points";
import { freeShapeRadii } from "./free-shape-radii";

import type { FreeShape } from "./free-shape";

/**
 * Whether a free shape can enter the model (ADR-0032, ADR-0033): width and height finite and
 * ≥ 0 (Q15; fractional as a conversion or an edit of its points leaves them, Q27), the points of its corners allowed
 * (`areFreeShapePoints`), their radii integers ≥ 0 (ADR-0029), an angle from 0 to 359 and a finite
 * position, fractional when an edit leaves it so (ADR-0035).
 *
 * @kind domain
 * @param shape - corners, width and height to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#rectangle
 */
export function isValidFreeShape(shape: FreeShape): boolean {
  const sizes = [shape.width, shape.height];

  return (
    sizes.every((size) => Number.isFinite(size) && size >= 0) &&
    isShapePlacement(shape) &&
    areFreeShapePoints(shape.corners.map((corner) => corner.point)) &&
    areValidRadii(freeShapeRadii(shape), shape.corners.length)
  );
}
