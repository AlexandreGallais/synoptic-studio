import type { BoundingBox } from "./bounding-box";

/**
 * An upright box with each side moved outward by a margin, inward when it is negative: the
 * product of its intervals each widened at both ends (`DERIV-box-overlap` step 10).
 *
 * Formula: [minX − m, maxX + m] × [minY − m, maxY + m].
 *
 * @kind geometry
 * @param box - its extreme coordinates
 * @param margin - how far each side moves outward
 * @returns the box its sides moved, its corners each the margin away on both axes
 * @see REF-WIKI-BOUNDING-BOX
 */
export function widenedBox(box: BoundingBox, margin: number): BoundingBox {
  return {
    maxX: box.maxX + margin,
    maxY: box.maxY + margin,
    minX: box.minX - margin,
    minY: box.minY - margin,
  };
}
