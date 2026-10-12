import { readStoredFreeShape } from "./read-stored-free-shape";
import { selectElement } from "./select-element";

import type { Point } from "../src";

/**
 * Writes the position of the shape of the page, the top-left of its box before it turns
 * (ADR-0035). A kept free shape gets it in its stored JSON, and only that: its width, height and
 * radii typed stay in their fields. The rectangle and the polygon get it in the page.
 *
 * @kind procedure
 * @param document - page document
 * @param position - the new position, in the coordinates of the SVG
 * @see docs/backlog/stories/E01-F04-US-019-edit-a-turned-shape.md
 */
export function writeShapePosition(document: Document, position: Point): void {
  const kept = readStoredFreeShape(document);
  const { dataset } = selectElement(document, "shape");

  if (kept !== undefined) {
    dataset["freeShape"] = JSON.stringify({ ...kept, ...position });

    return;
  }

  dataset["x"] = String(position.x);
  dataset["y"] = String(position.y);
}
