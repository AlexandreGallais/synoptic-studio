/**
 * Listens to the percentage's menu (SP-030's answer 9): Zoom in, Zoom out, Zoom to 100 %, Fit the
 * drawing and Fit the selection each apply their zoom about the canvas's centre, the pointer being
 * on the menu (`applyZoom`), then close the menu; a percentage typed in its field and applied, by
 * Enter or by leaving the field, zooms to it about the canvas's centre, within the bounds, when it
 * is a number above 0, otherwise the field shows the zoom again (agent's choice); then a redraw
 * asked.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToZoomMenu(document: Document, update: () => void): void {
  document.addEventListener("zoom", update);
}
