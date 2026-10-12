/**
 * Ticks or unticks the « Stretched » check box of the polygon, without firing any event (F06).
 *
 * @kind procedure
 * @param document - page document
 * @param isStretched - whether the box is ticked
 * @see docs/backlog/stories/E01-F06-US-021-stretched-option.md
 */
export function writeStretchedValue(document: Document, isStretched: boolean): void {
  const box = document.querySelector<HTMLInputElement>("#stretched");

  if (box !== null) {
    box.checked = isStretched;
  }
}
