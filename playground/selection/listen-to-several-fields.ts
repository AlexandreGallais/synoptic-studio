import { listenToNumberField } from "../number-fields";

import { applySeveralValue } from "./apply-several-value";
import { SEVERAL_FIELDS } from "./several-fields";

/**
 * Makes the four fields of several shapes set the shapes selected (F38), each a text field acting
 * as a number field (`listenToNumberField`, US-085): a value typed is applied on Enter or when the
 * field is left, as one shape's fields, to each shape selected (`applySeveralValue`,
 * `parsePanelValue`); Up and Down step each shape's own value by 1, or 10 with Shift (US-081), in
 * X or Y landing each shape's own straight box on integers as one shape's (F46, AUD-048); a value
 * refused, or refused for one shape, changes none; the page is then redrawn, the fields showing
 * the values again.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see RULE-interaction-2-selector
 */
export function listenToSeveralFields(document: Document, update: () => void): void {
  for (const [key, id] of SEVERAL_FIELDS) {
    listenToNumberField(document, id, (typed, isStep) => {
      applySeveralValue(document, key, { isStep, typed });
      update();
    });
  }
}
