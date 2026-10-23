import { createOutlineElements } from "../helps-frame";
import { readPickAreas, readShapesTaken } from "../picking";

import { readRectangleDrag } from "./read-rectangle-drag";
import { readSelectionBox } from "./read-selection-box";

/**
 * Creates the outlines of the shapes a selection rectangle would take, while it is dragged (F43,
 * as Figma outlines each object its marquee takes, `REF-FIGMA-SELECT-LAYERS`): those it touches,
 * (`readRectangleDrag`), those it encloses with Alt held at the pointer's last move
 * (`readShapesTaken`), each outlined as
 * under the pointer (`createOutlineElements`), of classes `hover-light` and `hover`, in the order
 * drawn; selected at the release only.
 *
 * @kind procedure
 * @param document - page document
 * @returns their `<path>` elements; nothing without a selection rectangle drawn
 * @see docs/domain/interaction.md#_2-selector
 */
export function createTakenElements(document: Document): readonly SVGPathElement[] {
  const drag = readRectangleDrag(document);

  if (drag === undefined) {
    return [];
  }

  const areas = readPickAreas(document);
  const taken = new Set(readShapesTaken(areas, readSelectionBox(drag), drag.isAlt));

  return areas.flatMap((area, index) =>
    taken.has(index) ? createOutlineElements(document, area, "hover") : [],
  );
}
