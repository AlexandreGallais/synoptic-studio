import { selectElement } from "../page";

import { applySeveralValue } from "./apply-several-value";
import { SEVERAL_FIELDS } from "./several-fields";

/**
 * Makes the four fields of several shapes set the shapes selected (F38): a value typed is applied
 * on Enter or when the field is left, as one shape's fields, to each shape selected
 * (`applySeveralValue`); a value refused, or refused for one shape, changes none; the page is then
 * redrawn, the fields showing the values again.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToSeveralFields(document: Document, update: () => void): void {
  for (const [key, id] of SEVERAL_FIELDS) {
    selectElement(document, id).addEventListener("change", () => {
      applySeveralValue(document, key, id);
      update();
    });
  }
}
