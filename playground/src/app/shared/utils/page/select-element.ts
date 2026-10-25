/**
 * Selects a required element of the page by its id, read from the document's map of ids
 * (`getElementById`), which a redraw does not empty, where `querySelector` walks the tree again
 * after each change of the document (`REF-HAPPY-DOM-QUERY`).
 *
 * @kind procedure
 * @param document - page document
 * @param id - `id` attribute, without `#`
 * @returns the HTML element carrying this `id`, the first in the page's order
 * @throws {Error} when the page has no HTML element with this id
 */
export function selectElement(document: Document, id: string): HTMLElement {
  // oxlint-disable-next-line unicorn/prefer-query-selector -- the map of ids, not a walk of the tree after each change (REF-HAPPY-DOM-QUERY)
  const element = document.getElementById(id);

  if (!(element instanceof HTMLElement)) {
    throw new Error(`Playground page without #${id}.`);
  }

  return element;
}
