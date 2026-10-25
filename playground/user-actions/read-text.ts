import { selectPageElement } from "./select-page-element";

/**
 * Text shown in an element.
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @returns its text
 * @see EN-021
 */
export function readText(selector: string): string {
  return selectPageElement(selector).textContent;
}
