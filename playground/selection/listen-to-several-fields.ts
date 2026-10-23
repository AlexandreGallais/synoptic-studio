import { selectElement } from "../page";

/**
 * Makes the four fields of several shapes set the shapes selected (F38): a value typed is applied
 * on Enter or when the field is left, as one shape's fields, to each shape selected
 * (`shapeAfterPanelEdit`, `textAfterPanelEdit`), a text never measured left as it is; a value
 * refused (`parsePanelValue`), or refused for one shape, changes none; the page then redrawn, the
 * fields showing the values again.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToSeveralFields(document: Document, update: () => void): void {
  selectElement(document, "several-fields").addEventListener("change", update);
}
