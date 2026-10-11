import { selectElement } from "./select-element";

import type { TurnedBox } from "../src";

/**
 * Keeps the angle, position and size of the turned shape just drawn, so that a width typed next
 * can keep its turned top-left corner in place (`writeResizedPosition`); a straight shape keeps
 * nothing, its top-left being fixed already (F09).
 *
 * @kind procedure
 * @param document - page document
 * @param box - the shape drawn
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
export function writeDrawnBox(document: Document, box: TurnedBox): void {
  const { angle, height, width, x, y } = box;
  const { dataset } = selectElement(document, "shape");

  if (angle === 0) {
    delete dataset["drawnBox"];

    return;
  }

  dataset["drawnBox"] = JSON.stringify({
    angle,
    height,
    width,
    x,
    y,
  });
}
