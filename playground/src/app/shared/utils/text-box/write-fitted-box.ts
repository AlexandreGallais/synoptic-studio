import { selectElement, writeInputValue } from "../page";

import { readFittedBox } from "./read-fitted-box";

import type { Text } from "editor";

/**
 * Writes one side of the box measured, unless the designer typed the box (`#text-options`'
 * `data-box-typed`) and this side still holds a number: a field emptied counts as not typed
 * (provisional, to confirm at VAL-018).
 *
 * @kind procedure
 * @param document - page document
 * @param id - `text-box-width` or `text-box-height`
 * @param side - the side as the text holds it, `NaN` when emptied, and as measured
 * @param side.fitted - the side measured, rounded up
 * @param side.typed - the side the field holds
 */
function writeFittedSide(
  document: Document,
  id: string,
  side: { readonly fitted: number; readonly typed: number },
): void {
  const isTyped = selectElement(document, "text-options").dataset["boxTyped"] === "true";

  if (!isTyped || Number.isNaN(side.typed)) {
    writeInputValue(document, id, side.fitted);
  }
}

/**
 * Writes in the box's width and height the size a text takes from its content, rounded up to
 * integers (`readFittedBox`), when Box is chosen after Fit content: the text keeps its
 * lines and its place, and a turned text its corner, as any change of its frame's size
 * (`choosePlacement`, ADR-0057). The text is measured again at each switch until the designer
 * types the box's width or height (`data-box-typed`, `listenToTextFields`): from then on the box
 * typed comes back, but for a field emptied (`writeFittedSide`).
 *
 * @kind procedure
 * @param document - page document
 * @param text - the text read from the fields, valid or not
 */
export function writeFittedBox(document: Document, text: Text): void {
  const box = text.isInBox ? readFittedBox(document, text) : undefined;

  if (box === undefined) {
    return;
  }

  writeFittedSide(document, "text-box-width", { fitted: box.boxWidth, typed: text.boxWidth });
  writeFittedSide(document, "text-box-height", { fitted: box.boxHeight, typed: text.boxHeight });
}
