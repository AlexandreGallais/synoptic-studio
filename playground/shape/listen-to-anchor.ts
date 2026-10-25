/**
 * Calls back when another anchor is chosen in the 3 × 3 grid, by click or with the arrow keys
 * (a radio group, `REF-WAI-APG-RADIO`), or when the « Stretched » check box is ticked or
 * unticked: both say how the polygon sits in its box (F07, F06).
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the playground
 * @see US-010
 */
export function listenToAnchor(document: Document, update: () => void): void {
  for (const button of document.querySelectorAll('input[name="anchor"]')) {
    button.addEventListener("change", update);
  }

  // The « Stretched » check box places the polygon in its box too (F06).
  document.querySelector("#stretched")?.addEventListener("change", update);
}
