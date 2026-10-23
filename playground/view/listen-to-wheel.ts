import { pannedView, wheelPixels } from "../../src";
import { readPointerOffset, selectElement } from "../page";

import { PAGE_VIEWS } from "./page-views";
import { readView } from "./read-view";
import { WHEEL_ZOOMS } from "./wheel-zooms";

import type { Vector } from "../../src";

/**
 * Reads the pan of a wheel alone (SP-030's answer 2): its deltas; with `Shift`, across, the
 * vertical delta most wheels send and the horizontal one macOS sends instead added (agent's
 * choice), as Penpot turns a vertical delta across except on macOS (`REF-PENPOT-VIEWPORT-ACTIONS`).
 *
 * @kind procedure
 * @param delta - the wheel's deltas, in pixels
 * @param isShift - whether `Shift` is held
 * @returns the pan, in pixels
 * @see REF-PENPOT-VIEWPORT-ACTIONS
 */
function readWheelPan(delta: Vector, isShift: boolean): Vector {
  return isShift ? { x: delta.x + delta.y, y: 0 } : delta;
}

/**
 * Adds a wheel's vertical delta to the zoom asked for the next frame, about the pointer's offset
 * on the canvas, the last one kept (`WHEEL_ZOOMS`), so that the deltas of a frame zoom once by their
 * sum.
 *
 * @kind procedure
 * @param document - page document
 * @param event - the wheel's turn on the canvas
 * @param delta - its vertical delta, in pixels
 * @see REF-PENPOT-VIEWPORT-ACTIONS
 */
function addWheelZoom(document: Document, event: WheelEvent, delta: number): void {
  const { point } = readPointerOffset(selectElement(document, "canvas"), event);
  const asked = WHEEL_ZOOMS.get(document)?.delta ?? 0;

  WHEEL_ZOOMS.set(document, { delta: asked + delta, offset: point });
}

/**
 * Writes the view a wheel's turn asks: with `Ctrl` or `Cmd` held, or a pinch, its vertical delta
 * added to the zoom asked for the next frame (`addWheelZoom`); alone, a pan by its deltas
 * (`pannedView`).
 *
 * @kind procedure
 * @param document - page document
 * @param event - the wheel's turn on the canvas
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
function writeWheelView(document: Document, event: WheelEvent): void {
  const delta = {
    x: wheelPixels(event.deltaX, event.deltaMode),
    y: wheelPixels(event.deltaY, event.deltaMode),
  };

  if (event.ctrlKey || event.metaKey) {
    addWheelZoom(document, event, delta.y);
  } else {
    PAGE_VIEWS.set(document, pannedView(readView(document), readWheelPan(delta, event.shiftKey)));
  }
}

/**
 * Listens to the wheel on the canvas, its default prevented, so that the page neither scrolls nor
 * zooms (`REF-MDN-WHEEL-EVENT`), the listener not passive, its deltas in pixels (`wheelPixels`):
 * with `Ctrl` or `Cmd` held, and a pinch, which the browser sends as a wheel with `Ctrl`, the
 * deltas gathered for the next frame's zoom about the pointer (`WHEEL_ZOOMS`); alone, a pan by them,
 * with `Shift` across (`pannedView`, SP-030's answer 2); then a redraw asked.
 *
 * @kind procedure
 * @param document - page document
 * @param update - redraws the page
 * @see docs/domain/interaction.md#_7-navigation-and-history
 */
export function listenToWheel(document: Document, update: () => void): void {
  selectElement(document, "canvas").addEventListener(
    "wheel",
    (event) => {
      event.preventDefault();
      writeWheelView(document, event);
      update();
    },
    { passive: false },
  );
}
