import { isFreeShape } from "./is-free-shape";
import { shapeKind } from "./shape-kind";

import type { Shape } from "./shape-kind";

/**
 * Whether a shape can be converted to a free shape (F08, F10): a regular polygon of width and
 * height > 0 (one of size 0 has no box to stretch, ADR-0033), or a round shape of any size (the
 * rectangle tool's free shape, valid at a size of 0, ADR-0036); a free shape is one already.
 *
 * @kind domain
 * @param shape - valid shape of the model
 * @returns `true` when the conversion button applies
 * @see docs/adr/0036-round-shape.md
 */
export function isConvertible(shape: Shape): boolean {
  return !isFreeShape(shape) && (shapeKind(shape) === "round" || shape.width * shape.height > 0);
}
