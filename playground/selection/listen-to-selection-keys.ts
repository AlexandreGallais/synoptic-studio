import { selectElement } from "../page";

/**
 * Listens to the keys of the selection (F37), outside the panel's fields, which keep every key
 * they are typed in: Ctrl + A, or Cmd + A, selects every shape, in Shape mode; Escape selects
 * none, unless a color picker is open, which Escape closes alone (F16); then redraws.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_1-tools
 */
export function listenToSelectionKeys(document: Document, update: () => void): void {
  // Placeholder until US-068.T3.
  selectElement(document, "canvas").addEventListener("keydown", update);
}
