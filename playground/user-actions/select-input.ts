import { selectPageElement } from "./select-page-element";

/**
 * Input or select of the page.
 *
 * @kind procedure
 * @param id - its id
 * @returns the input or the select
 * @throws {Error} when the page has no element of that id, or one without a value
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function selectInput(id: string): HTMLInputElement | HTMLSelectElement {
  const found = selectPageElement(`#${id}`);

  if (found instanceof HTMLInputElement || found instanceof HTMLSelectElement) {
    return found;
  }

  throw new TypeError(`#${id} is not an input.`);
}
