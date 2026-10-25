import { readKeptDrawing } from "../kept-shapes";

import { readShapePickArea } from "./read-shape-pick-area";

import type { PickArea } from "../../src";

/**
 * Reads what each shape of the drawing offers the pointer, in the order drawn
 * (`readKeptDrawing`): the shapes below the one the panel edits, that one, then those above it,
 * one area each (`readShapePickArea`), so that the index `shapeAtPoint` gives is the shape's; a
 * text never measured offers none, as it is not drawn.
 *
 * @kind procedure
 * @param document - page document
 * @returns the areas, the first drawn first
 * @see RULE-interaction-2-selector
 */
export function readPickAreas(document: Document): readonly PickArea[] {
  return readKeptDrawing(document).map(({ shape }) => readShapePickArea(document, shape));
}
