import { selectPageElement } from "./select-page-element";

/**
 * Clicks an element of the page.
 *
 * @kind procedure
 * @param id - id of the element, without `#`
 * @see EN-021
 */
export function clickElement(id: string): void {
  selectPageElement(`#${id}`).click();
}
