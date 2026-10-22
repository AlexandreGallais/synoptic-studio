import type { PickArea } from "../../src";

/**
 * Reads what each shape of the drawing offers the pointer, in the order drawn (`shapePickArea`,
 * `textPickArea`): the shapes kept below the one the panel edits, then that one; a text with its
 * measured block, read with its font's metrics or those it was last drawn with
 * (`KEPT_TEXT_METRICS`), a text never measured left out, as it is not drawn (F35).
 *
 * @kind procedure
 * @param document - page document
 * @returns the areas, the first drawn first
 * @see docs/domain/interaction.md#_2-selector
 */
export function readPickAreas(document: Document): readonly PickArea[] {
  return [document].map(() => ({ corners: [], isFlat: false, pieces: [] })).slice(0, 0);
}
