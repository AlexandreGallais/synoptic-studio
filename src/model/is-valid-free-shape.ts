import { areFreeShapePoints } from "./are-free-shape-points";
import { areValidRadii } from "./are-valid-radii";
import { isModelSize } from "./is-model-size";

import type { FreeShape } from "./free-shape";

/**
 * Whether a free shape can enter the model (ADR-0032): width and height integers ≥ 0 (Q15), its
 * stored points allowed (`areFreeShapePoints`), and one radius per point, integers ≥ 0
 * (ADR-0029).
 *
 * @kind domain
 * @param shape - points, width, height and radii to check
 * @returns `true` when every value is allowed
 * @see docs/domain/shapes.md#rectangle
 */
export function isValidFreeShape(shape: FreeShape): boolean {
  return (
    isModelSize(shape.width) &&
    isModelSize(shape.height) &&
    areFreeShapePoints(shape.points) &&
    areValidRadii(shape.radii, shape.points.length)
  );
}
