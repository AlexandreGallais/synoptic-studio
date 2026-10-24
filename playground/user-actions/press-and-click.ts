import { clickAt } from "./click-at";
import { pressPointer } from "./press-pointer";

import type { PointerPress } from "./press-pointer";

/**
 * Clicks an element of the page at a point of the window as a mouse does: the main button pressed
 * there, released there, then the `click` event (F36), so that what the press keeps is read.
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @param point - the pointer's x and y in the window, possibly fractional
 * @see docs/domain/interaction.md#_2-selector
 */
export function pressAndClick(
  selector: string,
  point: Pick<PointerPress, "clientX" | "clientY">,
): void {
  pressPointer(selector, "pointerdown", { ...point, buttons: 1 });
  pressPointer(selector, "pointerup", { ...point, buttons: 0 });
  clickAt(selector, point);
}
