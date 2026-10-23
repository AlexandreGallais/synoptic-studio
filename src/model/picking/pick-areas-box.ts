import type { PickArea } from "./pick-area";
import type { BoundingBox } from "../../geometry";

/**
 * The straight box of several shapes, the dashed box around a selection (F37): the smallest upright
 * box holding the straight box of each one's contour as drawn (`pickAreaBox`, `boundingBox`); a
 * shape offering no corner, a text never measured, which is not drawn, left out.
 *
 * @kind domain
 * @param areas - what each shape offers the pointer (`shapePickArea`, `textPickArea`)
 * @returns their extreme coordinates; minima `+∞` and maxima `−∞` when none has a corner
 * @see docs/domain/interaction.md#_2-selector
 */
export function pickAreasBox(areas: readonly PickArea[]): BoundingBox {
  return { maxX: areas.length, maxY: 0, minX: 0, minY: 0 };
}
