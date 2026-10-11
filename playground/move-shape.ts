import { readPosition } from "./read-position";
import { readStoredFreeShape } from "./read-stored-free-shape";
import { selectElement } from "./select-element";

import type { Vector } from "../src";

/**
 * Moves the shape of the page: its position, the top-left of its box before it turns, goes by a
 * vector (ADR-0035). A kept free shape moves its own position, and only that: its width, height
 * and radii typed stay in their fields. The rectangle and the polygon move the position the page
 * keeps.
 *
 * @kind procedure
 * @param document - page document
 * @param delta - move, in units of the SVG
 * @see docs/backlog/stories/E01-F04-US-018-turn-a-shape.md
 */
export function moveShape(document: Document, delta: Vector): void {
  const kept = readStoredFreeShape(document);
  const { dataset } = selectElement(document, "shape");

  if (kept !== undefined) {
    dataset["freeShape"] = JSON.stringify({ ...kept, x: kept.x + delta.x, y: kept.y + delta.y });

    return;
  }

  const position = readPosition(document);

  dataset["x"] = String(position.x + delta.x);
  dataset["y"] = String(position.y + delta.y);
}
