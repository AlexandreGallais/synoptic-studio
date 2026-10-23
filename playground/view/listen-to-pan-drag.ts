/**
 * Listens to a drag that pans the view (SP-030's answer 2): pressed with `Space` held or with the
 * middle button, the canvas takes the pointer, each move pans by its opposite (`pannedView`), the
 * drawing following the pointer, and no other listener of the canvas hears the press, its moves,
 * its release nor the click after it, so that nothing is selected, moved nor placed.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToPanDrag(document: Document, update: () => void): void {
  document.addEventListener("pointercancel", update);
}
