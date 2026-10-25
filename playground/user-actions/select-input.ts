import { selectPageElement } from "./select-page-element";

/**
 * Input of the page.
 *
 * @kind procedure
 * @param id - its id, without `#`
 * @returns the input element carrying that id
 * @throws {Error} when the page has no element of that id, or one that is not an input
 * @see EN-021
 */
export function selectInput(id: string): HTMLInputElement {
  const found = selectPageElement(`#${id}`);

  if (found instanceof HTMLInputElement) {
    return found;
  }

  throw new TypeError(`#${id} is not an input.`);
}
