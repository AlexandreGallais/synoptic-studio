import { shapeKind } from "./shape-kind";

import type { FreeShape } from "./free-shape";
import type { Shape } from "./shape-kind";

/**
 * Whether a shape of the model is a free shape, whose points are edited one by one (F08), and
 * neither a regular polygon nor a round shape (ADR-0036).
 *
 * @kind domain
 * @param shape - a shape of the model
 * @returns `true` for a free shape, the rectangle tool's included
 * @see docs/adr/0036-round-shape.md
 */
export function isFreeShape(shape: Shape): shape is FreeShape {
  return shapeKind(shape) === "free";
}
