import { isDrawnText, parsePanelValue, shapeAfterPanelEdit, textAfterPanelEdit } from "../../src";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readSelectedShapes,
  writeKeptMetrics,
  writeStoredDrawing,
} from "../kept-shapes";
import { readTextBlock } from "../picking";
import { MAX_SIZE } from "../shape";
import { readFieldValue } from "../text";

import type { DrawingShape, PanelEdit, PanelValues } from "../../src";
import type { KeptShape } from "../kept-shapes";

/**
 * Reads a shape of the drawing once a value is typed for several shapes: a shape a tool makes by
 * `shapeAfterPanelEdit`, a text by `textAfterPanelEdit` with its block as it is drawn
 * (`readTextBlock`), a text never measured, which is not drawn, as it is.
 *
 * @kind procedure
 * @param document - page document
 * @param shape - a shape selected
 * @param edit - the field, the value typed and the largest size (Q20)
 * @returns the shape changed; `undefined` when it refuses the value
 * @see docs/domain/interaction.md#_2-selector
 */
function readEditedShape(
  document: Document,
  shape: DrawingShape,
  edit: PanelEdit,
): DrawingShape | undefined {
  if (!isDrawnText(shape)) {
    return shapeAfterPanelEdit(shape, edit);
  }

  const block = readTextBlock(document, shape);

  return block === undefined ? shape : textAfterPanelEdit(shape, block, edit);
}

/**
 * Reads a shape kept once a value is typed for several shapes (`readEditedShape`), with the kind
 * the panel showed it as, a text keeping its metrics (`writeKeptMetrics`).
 *
 * @kind procedure
 * @param document - page document
 * @param kept - a shape selected and its kind
 * @param edit - the field, the value typed and the largest size (Q20)
 * @returns the shape changed and its kind; `undefined` when it refuses the value
 * @see docs/domain/interaction.md#_2-selector
 */
function readEditedKept(
  document: Document,
  kept: KeptShape,
  edit: PanelEdit,
): KeptShape | undefined {
  const shape = readEditedShape(document, kept.shape, edit);

  if (shape === undefined) {
    return undefined;
  }

  writeKeptMetrics(kept.shape, shape);

  return { kind: kept.kind, shape };
}

/**
 * Reads the drawing once a value is typed for several shapes: each shape selected changed
 * (`readEditedKept`), the others as they are, in the order drawn.
 *
 * @kind procedure
 * @param document - page document
 * @param edit - the field, the value typed and the largest size (Q20)
 * @returns the drawing; `undefined` when a shape selected refuses the value
 * @see docs/domain/interaction.md#_2-selector
 */
function readEditedDrawing(document: Document, edit: PanelEdit): readonly KeptShape[] | undefined {
  const selected = readSelectedShapes(document);
  const drawing = readKeptDrawing(document).map((kept, index) =>
    selected.includes(index) ? readEditedKept(document, kept, edit) : kept,
  );
  const edited = drawing.filter((kept): kept is KeptShape => kept !== undefined);

  return edited.length === drawing.length ? edited : undefined;
}

/**
 * Sets the shapes selected by the value typed in a field of several shapes (F38): read as an
 * integer, relative with a sign (`parsePanelValue`), and given to each shape
 * (`readEditedDrawing`), a size capped at 100 000 as one shape's (Q20); a value refused, or
 * refused by one shape, changes none. Every shape is kept below while several are selected (F37).
 *
 * @kind procedure
 * @param document - page document
 * @param key - the field typed in: `x`, `y`, `width` or `height`
 * @param id - its id
 * @see docs/domain/interaction.md#_2-selector
 */
export function applySeveralValue(document: Document, key: keyof PanelValues, id: string): void {
  const typed = parsePanelValue(readFieldValue(document, id));
  const drawing =
    typed === undefined
      ? undefined
      : readEditedDrawing(document, { key, maxSize: MAX_SIZE, typed });

  if (drawing !== undefined) {
    writeStoredDrawing(document, DRAWING_KEYS.below, drawing);
  }
}
