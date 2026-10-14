import { selectPageElement } from "./select-page-element";

/**
 * Clicks an element of the page.
 *
 * @kind procedure
 * @param id - id of the element, without `#`
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function clickElement(id: string): void {
  selectPageElement(`#${id}`).click();
}
