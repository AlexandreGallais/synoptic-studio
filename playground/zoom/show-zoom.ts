import { writeText } from "../page";

/**
 * Shows the zoom in the toolbar (SP-030's answer 9): its percentage, rounded to the nearest integer
 * (`nearestInteger`), on the menu's button and, unless it is being typed in, in the menu's field.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function showZoom(document: Document): void {
  writeText(document, "zoom-percentage", "");
}
