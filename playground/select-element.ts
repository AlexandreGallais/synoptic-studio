/** Specification of the story demonstrated by the page. */
const STORY = "docs/backlog/stories/E01-F02-US-007-shape-selector-playground.md";

/**
 * Selects a required element of the page by its id.
 *
 * @kind procedure
 * @param document - page document
 * @param id - `id` attribute, without `#`
 * @returns the HTML element carrying this `id`
 * @throws {Error} when the page has no element with this id
 * @see docs/backlog/stories/E01-F01-US-002-sharp-rectangle-in-playground.md
 */
export function selectElement(document: Document, id: string): HTMLElement {
  const element = document.querySelector<HTMLElement>(`#${id}`);

  if (element === null) {
    throw new Error(`Playground page without #${id} (${STORY}).`);
  }

  return element;
}
