import { selectPageElement } from "./select-page-element";

/** Where an element sits in the window, and its size. */
export type LaidOutBox = {
  /** Its left edge in the window. */
  readonly left: number;
  /** Its top edge in the window. */
  readonly top: number;
  /** Its width. */
  readonly width: number;
  /** Its height. */
  readonly height: number;
};

/**
 * Sets the box of an element of the page: its place and size, as a browser lays it out and
 * happy-dom does not (it gives every element a box 0 × 0 at the window's corner); set again when
 * the element changes size (US-084).
 *
 * @kind procedure
 * @param selector - CSS selector of the element
 * @param box - its left and top edges in the window, its width and its height
 * @see docs/backlog/stories/E15-F16-US-035-pick-a-color.md
 */
export function setElementBox(selector: string, box: LaidOutBox): void {
  const rectangle = new DOMRect(box.left, box.top, box.width, box.height);

  Object.defineProperty(selectPageElement(selector), "getBoundingClientRect", {
    configurable: true,
    value: () => rectangle,
  });
}
