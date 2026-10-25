import { subtract } from "../../src";

import type { Point, Size } from "../../src";

/** Where a pointer is on an element, from its top-left corner, and the element's size. */
export type PointerOffset = {
  /** The pointer, from the element's top-left corner, y down, in CSS pixels. */
  readonly point: Point;
  /** The element's width and height, in CSS pixels. */
  readonly size: Size;
};

/**
 * Reads where a pointer is on an element of the page, a slider, the color plane or the canvas: its
 * place in the window less the element's top-left corner (`subtract`), and the element's size.
 *
 * @kind procedure
 * @param element - the element under the pointer
 * @param event - the press, the move or the click of the pointer
 * @returns the pointer from the element's corner, and the element's size
 * @see US-035
 */
export function readPointerOffset(element: HTMLElement, event: MouseEvent): PointerOffset {
  const box = element.getBoundingClientRect();

  return {
    point: subtract({ x: event.clientX, y: event.clientY }, { x: box.left, y: box.top }),
    size: { height: box.height, width: box.width },
  };
}
