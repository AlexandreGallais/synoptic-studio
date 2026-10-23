import { PAGE_HISTORIES } from "./page-histories";

/**
 * Empties the page's history (F40), at Reset, at a step of the guided test and when the page is
 * mounted: the next redraw starts it again from the page as it then is, nothing to undo nor to
 * redo (`updateHistory`).
 *
 * @kind procedure
 * @param document - page document, its history then dropped
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function clearPageHistory(document: Document): void {
  PAGE_HISTORIES.has(document);
}
