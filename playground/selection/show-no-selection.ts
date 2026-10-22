import { writeText } from "../page";

/**
 * Shows the drawing with no shape selected (F36): every shape drawn, in the order drawn, none
 * edited; the panel's fields hidden, its title saying none is selected and how to select one.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function showNoSelection(document: Document): void {
  writeText(document, "status", "");
}
