/**
 * Writes the step shown into the page address (`?step=20`), without reloading nor adding to the
 * history, so that the address can be copied as a link to that step.
 *
 * @kind procedure
 * @param document - page document
 * @param index - index of the step shown, counted from 0
 * @see VAL-003
 */
export function writeStepToAddress(document: Document, index: number): void {
  const address = new URL(document.location.href);

  address.searchParams.set("step", String(index + 1));
  document.defaultView?.history.replaceState(undefined, "", address);
}
