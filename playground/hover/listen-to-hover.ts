import { readPointerOffset, selectElement } from "../page";
import { POINTER_OFFSETS } from "../view";

import { showHover } from "./show-hover";

/**
 * Listens to the pointer passing over the drawing: each move keeps its offset on the canvas
 * (`POINTER_OFFSETS`) and outlines the shape under it (`showHover`), read through the view at each
 * redraw; leaving the drawing forgets it and takes the outline away.
 *
 * @kind procedure
 * @param document - page document
 */
export function listenToHover(document: Document): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointermove", (event) => {
    POINTER_OFFSETS.set(document, readPointerOffset(canvas, event).point);
    showHover(document);
  });

  canvas.addEventListener("pointerleave", () => {
    POINTER_OFFSETS.delete(document);
    showHover(document);
  });
}
