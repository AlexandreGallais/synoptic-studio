import { boundingBox } from "../../geometry";

import { pickAreaBox } from "./pick-area-box";

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
 * @see RULE-interaction-2-selector
 */
export function pickAreasBox(areas: readonly PickArea[]): BoundingBox {
  const corners = areas
    .filter(({ corners: points }) => points.length > 0)
    .map((area) => pickAreaBox(area))
    .flatMap((box) => [
      { x: box.minX, y: box.minY },
      { x: box.maxX, y: box.maxY },
    ]);

  return boundingBox(corners);
}
