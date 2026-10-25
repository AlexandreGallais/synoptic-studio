import { selectElement } from "../page";

/**
 * Closes the picker: hidden, its swatch no longer expanded; the text keeps only the hexadecimal
 * and the opacity in its fields, the values the picker held being dropped when it opens again
 * (ADR-0056, decision 3).
 *
 * @kind procedure
 * @param document - page document
 */
export function closeColorPicker(document: Document): void {
  selectElement(document, "color-picker").hidden = true;

  for (const swatch of document.querySelectorAll(".swatch")) {
    swatch.setAttribute("aria-expanded", "false");
  }
}
