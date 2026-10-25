import { selectElement } from "./select-element";

/**
 * Writes a text into the element of the given id.
 *
 * @kind procedure
 * @param document - page document
 * @param id - id of the target element
 * @param text - text to display
 */
export function writeText(document: Document, id: string, text: string): void {
  selectElement(document, id).textContent = text;
}
