import { selectPageElement } from "./select-page-element";

/**
 * Presses a key on an element of the page: its `keydown` event, bubbling as the browser fires it.
 *
 * @kind procedure
 * @param selector - CSS selector of the element, `body` for the page itself
 * @param key - key pressed, `Delete`, `Backspace`
 * @see EN-021
 */
export function pressKey(selector: string, key: string): void {
  selectPageElement(selector).dispatchEvent(new KeyboardEvent("keydown", { bubbles: true, key }));
}
