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
 * Reads where a pointer is on a slider or the plane: its place in the window less the element's
 * top-left corner (`subtract`), and the element's size.
 *
 * @kind procedure
 * @param element - the slider or the plane
 * @param event - the press or the move of the pointer
 * @returns the pointer from the element's corner, and the element's size
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function readPointerOffset(element: HTMLElement, event: PointerEvent): PointerOffset {
  const box = element.getBoundingClientRect();

  return {
    point: subtract({ x: event.clientX, y: event.clientY }, { x: box.left, y: box.top }),
    size: { height: box.height, width: box.width },
  };
}
