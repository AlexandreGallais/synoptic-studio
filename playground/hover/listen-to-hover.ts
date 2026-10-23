import { readPointerOffset, selectElement } from "../page";

import { HOVER_OFFSETS } from "./hover-offsets";
import { showHover } from "./show-hover";

/**
 * Listens to the pointer passing over the drawing (F36): each move keeps its offset on the canvas
 * (`HOVER_OFFSETS`) and outlines the shape under it (`showHover`), read through the view at each
 * redraw (F42); leaving the drawing forgets it and takes the outline away.
 *
 * @kind procedure
 * @param document - page document
 * @see docs/domain/interaction.md#_2-selector
 */
export function listenToHover(document: Document): void {
  const canvas = selectElement(document, "canvas");

  canvas.addEventListener("pointermove", (event) => {
    HOVER_OFFSETS.set(document, readPointerOffset(canvas, event).point);
    showHover(document);
  });

  canvas.addEventListener("pointerleave", () => {
    HOVER_OFFSETS.delete(document);
    showHover(document);
  });
}
