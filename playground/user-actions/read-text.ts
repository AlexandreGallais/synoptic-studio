import { selectPageElement } from "./select-page-element";

/**
 * Text shown in an element.
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @returns its text
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function readText(selector: string): string {
  return selectPageElement(selector).textContent;
}
