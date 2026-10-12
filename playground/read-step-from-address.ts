/**
 * Reads the step of the guided test asked by the page address (`?step=20`), so that a link opens
 * the playground on that step.
 *
 * @kind procedure
 * @param document - page document
 * @returns index of the step asked, counted from 0; 0 when the address asks none or no integer
 * @see docs/backlog/stories/E01-F07-VAL-003-validate-anchor.md
 */
export function readStepFromAddress(document: Document): number {
  const query = new URLSearchParams(document.location.search);
  const step = Number(query.get("step"));

  return Number.isSafeInteger(step) ? step - 1 : 0;
}
