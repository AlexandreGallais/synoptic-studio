import { shapeSelectionAfterClick } from "../../src";
import { readSelectedShapes } from "../kept-shapes";
import { readPickAreas, readShapeAt } from "../picking";

import { writeShapeSelection } from "./write-shape-selection";

import type { Point } from "../../src";

/**
 * Selects by a click the shape under a point of the drawing (`readShapeAt`, F36): alone, or with
 * Shift added to the shapes selected, or taken out of them (`shapeSelectionAfterClick`, F37); on
 * the empty drawing, none, or with Shift the selection kept. The shapes are then selected by their
 * indices (`writeShapeSelection`): one keeps its place and is loaded into the panel, several are
 * kept with every shape, none edited.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point clicked, in the coordinates of the drawing's SVG
 * @param isShift - whether Shift is held, adding the shape under the point or taking it out
 * @see docs/domain/interaction.md#_2-selector
 */
export function selectShapeAt(document: Document, point: Point, isShift: boolean): void {
  const index = readShapeAt(document, readPickAreas(document), point);

  writeShapeSelection(
    document,
    shapeSelectionAfterClick(readSelectedShapes(document), index, isShift),
  );
}
