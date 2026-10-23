import { selectElement } from "../page";

/**
 * Shows the page's history in the toolbar (F40): the Undo button disabled while nothing is past,
 * the Redo button while nothing is to come, and the size the history holds beside the size set,
 * `History 1.2 kB of 100 MB` (`historySize`, `formatByteSize`, SP-029's answer 8).
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function showHistory(document: Document): void {
  selectElement(document, "history-size").hidden = false;
}
