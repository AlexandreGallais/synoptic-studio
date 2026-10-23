/**
 * Listens to what changes the view of the drawing (ADR-0098, SP-030's answers 2 and 6): the wheel
 * and the pinch (`listenToWheel`), `Space` (`listenToSpace`), a drag that pans
 * (`listenToPanDrag`), wired before the drawing's listeners so that a pan stops them; Reset and the
 * guided test's steps, which bring back the opening view.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToView(document: Document, update: () => void): void {
  document.addEventListener("viewchange", update);
}
