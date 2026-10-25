import { drawingWithoutShapes } from "../../src";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readSelectedShapes,
  writeStoredDrawing,
} from "../kept-shapes";

import { writeShapeSelection } from "./write-shape-selection";

/**
 * Removes the shapes selected, the others kept in their place, in the order drawn
 * (`drawingWithoutShapes`), and selects none: the panel leaves the shape it edited, keeping its
 * settings as the last of its kind, or dropping it when refused, as a click on the empty drawing
 * does (`writeShapeSelection`); with none selected, nothing changes.
 *
 * @kind procedure
 * @param document - page document
 * @see RULE-interaction-2-selector
 */
export function removeSelectedShapes(document: Document): void {
  const selected = readSelectedShapes(document);

  writeShapeSelection(document, []);

  writeStoredDrawing(
    document,
    DRAWING_KEYS.below,
    drawingWithoutShapes(readKeptDrawing(document), selected),
  );
}
