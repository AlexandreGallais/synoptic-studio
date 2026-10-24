import { isDrawnText, isPanelSize } from "../../src";
import { DRAWING_KEYS, readStoredDrawing, writeStoredDrawing } from "../kept-shapes";
import { selectElement } from "../page";
import { MAX_SIZE } from "../shape";

import { readEditedKept } from "./read-edited-kept";
import { writeEditedGeometry } from "./write-edited-geometry";
import { writeEditedText } from "./write-edited-text";

import type { PanelEdit, PanelValues, TypedPanelValue } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Writes a shape the panel edits alone into the page, its fields showing its values: a text by
 * `writeEditedText`, a box's width or height applied marking the box as the designer's
 * (`data-box-typed`, US-057); any other by `writeEditedGeometry`.
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the shape set, or as it was for a value refused, and its kind
 * @param isBoxTyped - whether a width or height was applied to it
 * @see docs/domain/interaction.md#_2-selector
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

/** A value applied in a field of the shape the panel edits: its field, the value and the largest size. */
type AppliedEdit = Omit<PanelEdit, "typed"> & {
  /** The value, relative or fixed; `undefined` when the text typed is no integer. */
  readonly typed: TypedPanelValue | undefined;
};

/**
 * Reads the shape the panel edits once a value is applied in one of its fields, as one of several
 * shapes (`readEditedKept`).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - the shape as last drawn, and its kind
 * @param edit - the field, the value applied and the largest size (Q20)
 * @returns the shape set; as it was for a value refused, or no integer
 * @see docs/domain/interaction.md#_2-selector
 */
function readAppliedKept(document: Document, kept: KeptShape, edit: AppliedEdit): KeptShape {
  const { typed } = edit;

  return (
    (typed === undefined ? undefined : readEditedKept(document, kept, { ...edit, typed })) ?? kept
  );
}

/**
 * Sets the shape the panel edits, alone, by a value of one of its fields (US-085), as each of
 * several shapes is set (`readEditedKept`, `shapeAfterPanelEdit`, `textAfterPanelEdit`): typed, an
 * integer relative with a sign (`parsePanelValue`), or an arrow's step, relative, from the shape
 * as last drawn, a size capped at 100 000 (Q20); the shape then kept as the one edited, so that
 * the next key starts from it before the redraw, and written into the panel, its fields showing
 * its values again (`writeEditedShape`), a value refused putting them back. A shape refused has no
 * value to start from: its field keeps the text typed, read as before.
 *
 * @kind procedure
 * @param document - page document
 * @param key - the field: `x`, `y`, `width` or `height`
 * @param typed - the value, relative or fixed; `undefined` when the text typed is no integer
 * @see docs/domain/interaction.md#_2-selector
 */
export function applyEditedValue(
  document: Document,
  key: keyof PanelValues,
  typed: TypedPanelValue | undefined,
): void {
  const [kept] = readStoredDrawing(document, DRAWING_KEYS.edited);

  if (kept === undefined) {
    return;
  }

  const shown = readAppliedKept(document, kept, { key, maxSize: MAX_SIZE, typed });

  writeStoredDrawing(document, DRAWING_KEYS.edited, [shown]);
  writeEditedShape(document, shown, shown !== kept && isPanelSize(key));
}
