import { PICK_TOLERANCE, screenToUser, shapeAtPoint, splitDrawing } from "../../src";
import { readPickAreas } from "../hover";
import {
  DRAWING_KEYS,
  VIEW_SCALE,
  readKeptDrawing,
  readStoredDrawing,
  writeStoredDrawing,
} from "../kept-shapes";
import { saveKindSettings } from "../kind-settings";
import { readShapeKind, selectElement, writeEditedKind } from "../page";

import { loadPickedShape } from "./load-picked-shape";

import type { Point } from "../../src";

/**
 * Selects the shape under a point of the drawing (`shapeAtPoint`, F36): the drawing split around
 * it (`splitDrawing`), so that it keeps its place, the shapes before it below and those after it
 * above; the panel leaves the shape it edited, keeping its settings as the last of its kind
 * (`saveKindSettings`), and loads the one picked (`loadPickedShape`); on the empty drawing, every
 * shape below and none selected.
 *
 * @kind procedure
 * @param document - page document
 * @param point - the point clicked, in the coordinates of the drawing's SVG
 * @param isShift - whether Shift is held, adding the shape under the point or taking it out
 * @see docs/domain/interaction.md#_2-selector
 */
export function selectShapeAt(document: Document, point: Point, isShift: boolean): void {
  // Placeholder until US-068.T2: Shift only written down, a click with it selecting alone.
  selectElement(document, "canvas").dataset["shift"] = String(isShift);

  const index = shapeAtPoint(
    readPickAreas(document),
    point,
    screenToUser(PICK_TOLERANCE, VIEW_SCALE),
  );
  const { above, below, edited } = splitDrawing(readKeptDrawing(document), index);
  const [picked] = edited;

  if (readStoredDrawing(document, DRAWING_KEYS.edited).length > 0) {
    saveKindSettings(document, readShapeKind(document));
  }

  writeStoredDrawing(document, DRAWING_KEYS.below, below);
  writeStoredDrawing(document, DRAWING_KEYS.edited, edited);
  writeStoredDrawing(document, DRAWING_KEYS.above, above);

  if (picked === undefined) {
    writeEditedKind(document, "none");
  } else {
    loadPickedShape(document, picked);
  }
}
