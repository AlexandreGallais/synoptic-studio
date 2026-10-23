import { selectElement } from "../page";

/**
 * Listens to the ends of the panel's changes (F40): a step asked at each value applied or choice
 * made in the panel, `change` fired by Enter, by leaving a field typed in or by a choice
 * (`REF-MDN-CHANGE-EVENT`), at each click on one of its buttons, at each key released on a slider
 * of the color picker and at each release of the pointer on one, or on the plane (SP-029's answer
 * 4); and at the Content field of a new text left with a content typed, its placing and its
 * typing one step (`NEW_TEXTS`, SP-029's answer 5). A value typed back as it was, or a button
 * that changes no shape, takes none, the drawing the same (`historyWithStep`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page from the values typed
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToPanelSteps(document: Document, update: () => void): void {
  selectElement(document, "panel").addEventListener("change", () => {
    update();
  });
}
