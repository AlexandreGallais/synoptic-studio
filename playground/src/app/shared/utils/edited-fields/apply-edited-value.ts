import { isDrawnText, isPanelSize, panelValueAfterEdit } from "editor";

import { resetShownText } from "../applied-values";
import { DRAWING_KEYS, readStoredDrawing, writeStoredDrawing } from "../kept-shapes";
import { selectElement, writeInputValue } from "../page";
import { MAX_SIZE } from "../shape";

import { readEditedKeptShape } from "./read-edited-kept-shape";
import { writeEditedGeometry } from "./write-edited-geometry";
import { writeEditedText } from "./write-edited-text";

import type { EditedField } from "./edited-field";
import type { KeptShape } from "../kept-shapes";
import type { PanelEdit, TypedPanelValue } from "editor";

/**
 * Writes a shape the panel edits alone into the page, its fields showing its values: a text by
 * `writeEditedText`, a box's width or height applied marking the box as the designer's
 * (`data-box-typed`); any other by `writeEditedGeometry`.
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the shape set, or as it was for a value refused, and its kind
 * @param isBoxTyped - whether a width or height was applied to it
 */
function writeEditedShape(document: Document, kept: KeptShape, isBoxTyped: boolean): void {
  const { shape } = kept;

  if (!isDrawnText(shape)) {
    writeEditedGeometry(document, shape);

    return;
  }

  if (isBoxTyped) {
    selectElement(document, "text-options").dataset["boxTyped"] = "true";
  }

  writeEditedText(document, shape);
}

/**
 * Writes a value applied in a field of a shape refused, which has no value of its own to start
 * from: a relative value from 0, a size taken within 0 and 100 000
 * (`panelValueAfterEdit`), so that the redraw reads it; a text that is no number puts back the
 * text the field showed before (`resetShownText`).
 *
 * @kind procedure
 * @param document - page document
 * @param field - the value it sets and the field's id
 * @param typed - the value, relative or fixed; `undefined` when the text typed is no number
 */
function writeRefusedValue(
  document: Document,
  field: EditedField,
  typed: TypedPanelValue | undefined,
): void {
  const [key, id] = field;

  if (typed === undefined) {
    resetShownText(document, id);

    return;
  }

  writeInputValue(document, id, panelValueAfterEdit(0, { key, maxSize: MAX_SIZE, typed }));
}

/**
 * Sets the shape the panel edits, as last drawn, by a value applied in one of its fields
 * (`readEditedKeptShape`), then keeps it as the one edited and writes it into the panel
 * (`writeEditedShape`), a width or height marking a text's box as the designer's.
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the shape as last drawn, and its kind
 * @param edit - the field, the value applied and the largest size (Q20)
 */
function writeAppliedShape(document: Document, kept: KeptShape, edit: PanelEdit): void {
  const shown = readEditedKeptShape(document, kept, edit);

  writeStoredDrawing(document, DRAWING_KEYS.edited, [shown]);
  writeEditedShape(document, shown, isPanelSize(edit.key));
}

/**
 * Sets the shape the panel edits, alone, by a value of one of its fields, as each of
 * several shapes is set (`readEditedKeptShape`, `shapeAfterPanelEdit`, `textAfterPanelEdit`): typed, a
 * number relative with a sign (`parsePanelValue`), or an arrow's step, relative, from the shape
 * as last drawn, a size taken within 0 and 100 000 (Q20); the shape then kept as the one
 * edited, so that the next key starts from it before the redraw, and written into the panel, its
 * fields showing its values again (`writeEditedShape`), a text that is no number putting them
 * back. A shape refused has no value to start from: a relative value starts from 0
 * (`writeRefusedValue`).
 *
 * @kind procedure
 * @param document - page document
 * @param field - the value it sets, `x`, `y`, `width` or `height`, and the field's id
 * @param typed - the value, relative or fixed; `undefined` when the text typed is no number
 */
export function applyEditedValue(
  document: Document,
  field: EditedField,
  typed: TypedPanelValue | undefined,
): void {
  const [kept] = readStoredDrawing(document, DRAWING_KEYS.edited);

  if (kept === undefined) {
    writeRefusedValue(document, field, typed);

    return;
  }

  if (typed === undefined) {
    // No number: its fields show its values again.
    writeEditedShape(document, kept, false);

    return;
  }

  writeAppliedShape(document, kept, { key: field[0], maxSize: MAX_SIZE, typed });
}
