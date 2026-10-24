import { readEditedKept } from "../edited-fields";
import {
  DRAWING_KEYS,
  readKeptDrawing,
  readSelectedShapes,
  writeStoredDrawing,
} from "../kept-shapes";
import { MAX_SIZE } from "../shape";

import type { PanelValues, TypedPanelValue } from "../../src";

/**
 * Sets the shapes selected by a value of a field of several shapes (F38): typed, a number
 * relative with a sign (`parsePanelValue`), or an arrow's step, relative (US-081), given to each
 * shape (`readEditedKept`), a size taken within 0 and 100 000 as one shape's (Q20, AUD-046), the
 * others kept as they are, in the order drawn; a text that is no number changes none. Every shape
 * is kept below while several are selected (F37).
 *
 * @kind procedure
 * @param document - page document
 * @param key - the field: `x`, `y`, `width` or `height`
 * @param typed - the value, relative or fixed; `undefined` when the text typed is no number
 * @see docs/domain/interaction.md#_2-selector
 */
export function applySeveralValue(
  document: Document,
  key: keyof PanelValues,
  typed: TypedPanelValue | undefined,
): void {
  if (typed === undefined) {
    return;
  }

  const edit = { key, maxSize: MAX_SIZE, typed };
  const selected = readSelectedShapes(document);
  const drawing = readKeptDrawing(document).map((kept, index) =>
    selected.includes(index) ? readEditedKept(document, kept, edit) : kept,
  );

  writeStoredDrawing(document, DRAWING_KEYS.below, drawing);
}
