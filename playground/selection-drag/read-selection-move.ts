import { pickAreasBox } from "../../src";
import { readPickAreas } from "../hover";
import { readKeptDrawing } from "../kept-shapes";

import type { ShapesMove } from "../hover";

/**
 * Reads the move of the shapes selected (F39): the drawing as it is, the shapes selected before
 * and once moved, the shapes moved, a text never measured, which is not drawn, left as it is, and
 * their anchor, the top-left of their dashed box (`pickAreasBox`), for one shape its straight box,
 * the X and Y the panel shows.
 *
 * @kind procedure
 * @param document - page document
 * @param before - the shapes selected before the move, by their indices, given back to a drag
 * cancelled
 * @param selected - the shapes selected once it moves, by their indices
 * @returns the drawing, the shapes selected and moved, and their anchor
 * @see docs/domain/interaction.md#_2-selector
 */
export function readSelectionMove(
  document: Document,
  before: readonly number[],
  selected: readonly number[],
): ShapesMove {
  const areas = readPickAreas(document);
  // A text never measured offers no corner: it is not drawn, so it neither moves nor anchors.
  const moved = selected.filter((shown) => (areas[shown]?.corners.length ?? 0) > 0);
  const box = pickAreasBox(moved.flatMap((shown) => areas[shown] ?? []));

  return {
    anchor: { x: box.minX, y: box.minY },
    before,
    drawing: readKeptDrawing(document),
    moved,
    selected,
  };
}
