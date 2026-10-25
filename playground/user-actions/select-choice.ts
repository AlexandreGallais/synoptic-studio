import { selectPageElement } from "./select-page-element";

/**
 * Select (drop-down list) of the page.
 *
 * @kind procedure
 * @param id - its id, without `#`
 * @returns the select element carrying that id
 * @throws {Error} when the page has no element of that id, or one that is not a select
 * @see EN-021
 */
export function selectChoice(id: string): HTMLSelectElement {
  const found = selectPageElement(`#${id}`);

  if (found instanceof HTMLSelectElement) {
    return found;
  }

  throw new TypeError(`#${id} is not a select.`);
}
