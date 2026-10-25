import { add, scale, subtract } from "editor";

// lint-disable-next-line local/canonical-import-path -- the rule names the first folder under the common ancestor, playground/src, which has no barrel in the Angular app
import { readView } from "../src/app/shared/utils/view";

import { selectPageElement } from "./select-page-element";

import type { Point } from "editor";

/** The keys held during a pointer event on the drawing. */
export type PointerKeys = {
  /** Whether Alt, Option on a Mac, is held. */
  readonly altKey: boolean;
  /** Whether Shift is held. */
  readonly shiftKey: boolean;
};

/**
 * Reads the buttons held during a pointer event on the canvas: the main one while pressed or moved.
 *
 * @kind procedure
 * @param type - `pointerdown`, `pointermove`, `pointerup`, `pointercancel` or `click`
 * @returns 1 for the main button, 0 for none
 */
function readHeldButtons(type: string): number {
  return type === "pointerdown" || type === "pointermove" ? 1 : 0;
}

/**
 * Creates a pointer event on the canvas as the browser fires it: the main button held while
 * pressed or moved, a `click` a mouse event.
 *
 * @kind procedure
 * @param type - `pointerdown`, `pointermove`, `pointerup`, `pointercancel` or `click`
 * @param client - where it is in the window
 * @param keys - the keys held
 * @returns the event, bubbling
 */
function createDrawingEvent(type: string, client: Point, keys: PointerKeys): MouseEvent {
  const init = {
    ...keys,
    bubbles: true,
    buttons: readHeldButtons(type),
    clientX: client.x,
    clientY: client.y,
    pointerId: 1,
  };

  return type === "click" ? new MouseEvent(type, init) : new PointerEvent(type, init);
}

/**
 * Dispatches a pointer event on the canvas at a point of the drawing, as the browser does
 * (`createDrawingEvent`): the point placed in the window from the canvas's box and the view, its
 * offset from the view's origin times its scale (`readView`, ADR-0098).
 *
 * @kind procedure
 * @param type - `pointerdown`, `pointermove`, `pointerup`, `pointercancel` or `click`
 * @param point - the point of the drawing, possibly fractional
 * @param keys - the keys held
 */
export function dispatchDrawingPointer(type: string, point: Point, keys: PointerKeys): void {
  const canvas = selectPageElement("#canvas");
  const box = canvas.getBoundingClientRect();
  const view = readView(document);
  const client = add({ x: box.left, y: box.top }, scale(subtract(point, view.origin), view.scale));

  canvas.dispatchEvent(createDrawingEvent(type, client, keys));
}
