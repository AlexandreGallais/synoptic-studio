import { selectPageElement } from "./select-page-element";

import type { Keys } from "./keys";

/**
 * Clicks an element with modifier keys held.
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @param keys - keys held
 */
export function clickWithKeys(selector: string, keys: Keys): void {
  selectPageElement(selector).dispatchEvent(new MouseEvent("click", { ...keys, bubbles: true }));
}
