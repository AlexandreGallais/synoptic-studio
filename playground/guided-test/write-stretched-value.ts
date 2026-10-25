/**
 * Ticks or unticks the « Stretched » check box of the polygon, without firing any event (F06).
 *
 * @kind procedure
 * @param document - page document
 * @param isStretched - whether the box is ticked
 * @see US-021
 */
export function writeStretchedValue(document: Document, isStretched: boolean): void {
  const box = document.querySelector<HTMLInputElement>("#stretched");

  if (box !== null) {
    box.checked = isStretched;
  }
}
