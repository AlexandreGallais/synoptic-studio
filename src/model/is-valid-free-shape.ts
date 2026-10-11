import { areFreeShapePoints } from "./are-free-shape-points";
import { areValidRadii } from "./are-valid-radii";
import { freeShapeRadii } from "./free-shape-radii";

import type { FreeShape } from "./free-shape";

/**
 * Whether a free shape can enter the model (ADR-0032, ADR-0033): width and height finite and
 * ≥ 0 (Q15; fractional only as a conversion leaves them, Q27), the points of its corners allowed
 * (`areFreeShapePoints`), and their radii integers ≥ 0 (ADR-0029).
 *
 * @kind domain
 * @param shape - points, width, height and radii to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#rectangle
 */
export function isValidFreeShape(shape: FreeShape): boolean {
  const sizes = [shape.width, shape.height];

  return (
    sizes.every((size) => Number.isFinite(size) && size >= 0) &&
    areFreeShapePoints(shape.corners.map((corner) => corner.point)) &&
    areValidRadii(freeShapeRadii(shape), shape.corners.length)
  );
}
