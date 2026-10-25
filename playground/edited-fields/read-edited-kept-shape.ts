import { isDrawnText, shapeAfterPanelEdit, textAfterPanelEdit } from "../../src";
import { writeKeptMetrics } from "../kept-shapes";
import { readTextBlock } from "../picking";

import type { DrawingShape, PanelEdit } from "../../src";
import type { KeptShape } from "../kept-shapes";

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
 * @see RULE-interaction-2-selector
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
 * of several shapes selected (F38) or the one the panel edits alone (US-085), with the kind the
 * panel showed it as, a text keeping its metrics (`writeKeptMetrics`).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - a shape selected and its kind
 * @param edit - the field, the value typed and the largest size (Q20)
 * @returns the shape changed and its kind
 * @see RULE-interaction-2-selector
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
