import { readPointerOffset } from "../page";

import type { PointerOffset } from "../page";

/**
 * Listens to a click or a drag on a slider or the plane: the element takes the focus and the
 * pointer, so that the drag goes on beyond its edges, and each place pressed or dragged over is
 * called back.
 *
 * @kind procedure
 * @param element - the slider or the plane
 * @param pick - called back with the pointer's place on the element
 */
export function listenToDrag(element: HTMLElement, pick: (offset: PointerOffset) => void): void {
  element.addEventListener("pointerdown", (event) => {
    element.focus();
    element.setPointerCapture(event.pointerId);
    pick(readPointerOffset(element, event));
  });

  element.addEventListener("pointermove", (event) => {
    // A pointer moved with no button held is no drag.
    if (event.buttons !== 0) {
      pick(readPointerOffset(element, event));
    }
  });
}
