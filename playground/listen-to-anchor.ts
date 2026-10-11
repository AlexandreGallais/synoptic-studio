/**
 * Calls back when another anchor is chosen in the 3 × 3 grid, by click or with the arrow keys
 * (a radio group, `REF-WAI-APG-RADIO`).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see docs/backlog/stories/E01-F07-US-010-anchor-picker-playground.md
 */
export function listenToAnchor(document: Document, update: () => void): void {
  for (const button of document.querySelectorAll('input[name="anchor"]')) {
    button.addEventListener("change", update);
  }
}
