import { cyclicItem } from "../contour";

import type { Point } from "../../math";
import type { CornerPoints } from "../contour";

/**
 * Every corner of a cyclic contour with its two neighbours: the vertex before the first is the
 * last, the vertex after the last is the first (Q11). Built once per contour, then shared by the
 * turning angles and the fillets.
 *
 * @kind geometry
 * @param contour - vertices in drawing order
 * @returns one triple per vertex, in drawing order
 * @see DERIV-turning-angle
 */
export function cornerTriples(contour: readonly Point[]): readonly CornerPoints[] {
  return contour.map((vertex, index) => ({
    next: cyclicItem(contour, index + 1, vertex),
    previous: cyclicItem(contour, index - 1, vertex),
    vertex,
  }));
}
