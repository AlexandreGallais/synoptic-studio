import { isDrawnText, shapeAfterPanelEdit, textAfterPanelEdit } from "editor";

import { writeKeptMetrics } from "../kept-shapes";
import { readTextBlock } from "../picking";

import type { KeptShape } from "../kept-shapes";
import type { DrawingShape, PanelEdit } from "editor";

/**
 * Reads a shape of the drawing once a value is typed in its X, Y, Width or Height: a shape a tool
 * makes by `shapeAfterPanelEdit`, a text by `textAfterPanelEdit` with its block as it is drawn
 * (`readTextBlock`), a text never measured, which is not drawn, as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - a shape selected
 * @param edit - the field, the value typed and the largest size (Q20)
 * @returns the shape changed
 */
function readEditedShape(document: Document, shape: DrawingShape, edit: PanelEdit): DrawingShape {
  if (!isDrawnText(shape)) {
    return shapeAfterPanelEdit(shape, edit);
  }

  const block = readTextBlock(document, shape);

  return block === undefined ? shape : textAfterPanelEdit(shape, block, edit);
}

/**
 * Reads a shape kept once a value is typed in its X, Y, Width or Height (`readEditedShape`), one
 * of several shapes selected or the one the panel edits alone, with the kind the
 * panel showed it as, a text keeping its metrics (`writeKeptMetrics`).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - a shape selected and its kind
 * @param edit - the field, the value typed and the largest size (Q20)
 * @returns the shape changed and its kind
 */
export function readEditedKeptShape(
  document: Document,
  kept: KeptShape,
  edit: PanelEdit,
): KeptShape {
  const shape = readEditedShape(document, kept.shape, edit);

  writeKeptMetrics(kept.shape, shape);

  return { kind: kept.kind, shape };
}
