/**
 * Element of the page that a test names: missing, the test fails at once instead of going on
 * without it.
 *
 * @kind procedure
 * @param selector - CSS selector
 * @returns the first element matching it
 * @throws {Error} when the page has none
 * @see docs/backlog/stories/E14-F11-EN-021-shared-test-helpers.md
 */
export function selectPageElement(selector: string): HTMLElement {
  const found = document.querySelector<HTMLElement>(selector);

  if (found === null) {
    throw new Error(`The page has no ${selector}.`);
  }

  return found;
}
