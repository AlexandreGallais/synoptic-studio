import { ALIGNMENTS } from "../geometry";

import type { Anchor } from "../geometry";

/**
 * Whether an anchor can enter the model: `min`, `mid` or `max` on each axis, 9 places in all.
 *
 * @kind domain
 * @param anchor - horizontal and vertical alignment to check
 * @returns `true` when both are allowed
 * @see docs/domain/shapes.md#regular-polygon
 */
export function isAnchor(anchor: Anchor): boolean {
  return ALIGNMENTS.includes(anchor.horizontal) && ALIGNMENTS.includes(anchor.vertical);
}
