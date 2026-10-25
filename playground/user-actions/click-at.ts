import { selectPageElement } from "./select-page-element";

import type { PointerPress } from "./press-pointer";

/**
 * Clicks an element of the page at a point of the window: its `click` event, bubbling, with the
 * point's `clientX` and `clientY`, as the browser fires it.
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @param point - the pointer's x and y in the window, possibly fractional
 * @see US-063
 */
export function clickAt(selector: string, point: Pick<PointerPress, "clientX" | "clientY">): void {
  selectPageElement(selector).dispatchEvent(new MouseEvent("click", { ...point, bubbles: true }));
}
